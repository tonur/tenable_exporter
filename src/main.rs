use std::time::Duration;
use async_std::task;
use clap::Parser;
use actix_web::{web, App, HttpServer};
use actix_web_opentelemetry::{PrometheusMetricsHandler, RequestMetrics, RequestTracing};
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::{metrics::SdkMeterProvider, Resource};
use lazy_static::lazy_static;
// use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts, Registry};
use prometheus::{IntCounterVec, Opts, Registry};
use serde::{Deserialize, Serialize};
lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref TENABLE_VULNERABILITY_INFO: IntCounterVec = IntCounterVec::new(
        Opts::new("tenable_vulnerability_info", "Tenable vulnerability info"),
        &["severity"],
    ).unwrap();
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
pub struct AppArguments {
    #[arg(long)]
    tio_access_key: String,
    #[arg(long)]
    tio_secret_key: String,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_arguments_file = "AppArguments.json";

    let arguments: AppArguments = if std::path::Path::new(app_arguments_file).exists() {
        let file_content = std::fs::read_to_string(app_arguments_file)
            .expect("Failed to read the file");
        serde_json::from_str(&file_content)
            .expect("JSON was not well-formatted")
    } else {
        AppArguments::parse()
    };
    
    REGISTRY.register(Box::new(TENABLE_VULNERABILITY_INFO.clone())).unwrap();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(REGISTRY.clone())
        .build()?;

    // set up your meter provider with your exporter(s)
    let provider = SdkMeterProvider::builder()
        .with_reader(exporter)
        .with_resource(Resource::new([KeyValue::new("service.name", "tenable_exporter")]))
        .build();
    global::set_meter_provider(provider);

    task::spawn(async move {
        loop {
            fetch_data(arguments.clone()).await;
            task::sleep(Duration::from_secs(300)).await;
        }
    });

    let _ = HttpServer::new(move || {
        App::new()
            .wrap(RequestTracing::new())
            .wrap(RequestMetrics::default())
            .route("/metrics", web::get().to(PrometheusMetricsHandler::new(REGISTRY.clone())))
        })
        .bind("localhost:8080")?
        .run()
        .await;

    Ok(())
}

pub async fn fetch_data(arguments: AppArguments) {
    let mut configuration = openapi::apis::configuration::Configuration::default();
    configuration.api_key = Some(openapi::apis::configuration::ApiKey {
        prefix: None,
        // Configuration should be like: --header 'X-ApiKeys: accessKey=;secretKey='
        key: format!("accessKey={};secretKey={}", arguments.tio_access_key, arguments.tio_secret_key),
    });

    let exports_vulns_request_export_request = openapi::models::ExportsVulnsRequestExportRequest { num_assets: 100, include_unlicensed: None, filters: None };
    let request_vulns_export = openapi::apis::exports_api::exports_vulns_request_export(&configuration, exports_vulns_request_export_request).await;
    let export_uuid = request_vulns_export.unwrap().export_uuid.unwrap();
    let mut export_status_string;
    let mut total_chunks;
    while {
        let vulns_export_status = openapi::apis::exports_api::exports_vulns_export_status(&configuration, &export_uuid).await;
        let export_status = vulns_export_status.unwrap().status.unwrap();
        export_status_string = export_status.status.unwrap();
        total_chunks = export_status.total_chunks.unwrap();
        export_status_string != "FINISHED"
    } {
        task::sleep(Duration::from_secs(1)).await;
    }

    for chunk_id in 0..total_chunks {
        let chunk = openapi::apis::exports_api::exports_vulns_download_chunk(&configuration, &export_uuid, chunk_id).await;
        let chunk = chunk.unwrap();
        let severity = chunk.severity.clone().unwrap();
        TENABLE_VULNERABILITY_INFO.with_label_values(&[severity.as_str()]).inc();
        // Write chunk to file
        let _ = std::fs::write(format!("chunk_{}.json", chunk_id), serde_json::to_string(&chunk).unwrap());
    }
}