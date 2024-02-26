use std::time::Duration;
use async_std::task;
use clap::Parser;
use actix_web::{web, App, HttpServer};
use actix_web_opentelemetry::{PrometheusMetricsHandler, RequestMetrics, RequestTracing};
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::{metrics::SdkMeterProvider, Resource};
use lazy_static::lazy_static;
use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref TENABLE_VULNERABILITY_INFO: IntCounterVec = IntCounterVec::new(
        Opts::new("tenable_vulnerability_info", "Tenable vulnerability info"),
        &["severity"],
    ).unwrap();
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct AppArguments {
    #[arg(long)]
    tio_access_key: String,
    #[arg(long)]
    tio_secret_key: String,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
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

    // Run actix task to fetch data every 300 seconds
    let arguments = AppArguments::parse();
    task::spawn(async move {
        loop {
            fetch_data(arguments.clone(), ).await;
            task::sleep(Duration::from_secs(300)).await;
        }
    });

    // Run actix server, metrics are now available at http://localhost:8080/metrics
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

    // Configuration should be like: --header 'X-ApiKeys: accessKey=;secretKey='
    let mut configuration = openapi::apis::configuration::Configuration::default();
    configuration.api_key = Some(openapi::apis::configuration::ApiKey {
        prefix: None,
        key: format!("accessKey={};secretKey={}", arguments.tio_access_key, arguments.tio_secret_key),
    });

    let exports_vulns_request_export_request = openapi::models::ExportsVulnsRequestExportRequest { num_assets: 100, include_unlicensed: None, filters: None };
    let request_vulns_export = openapi::apis::exports_api::exports_vulns_request_export(&configuration, exports_vulns_request_export_request).await;
    let export_uuid = request_vulns_export.unwrap().export_uuid.unwrap();
    let vulns_export_status = openapi::apis::exports_api::exports_vulns_export_status(&configuration, &export_uuid).await;
    let export_status = vulns_export_status.unwrap().status.unwrap();
    let mut export_status_string = export_status.status.unwrap();
    let mut total_chunks = export_status.total_chunks.unwrap();
    while export_status_string != "FINISHED" {
        task::sleep(Duration::from_secs(1)).await;
        let vulns_export_status = openapi::apis::exports_api::exports_vulns_export_status(&configuration, &export_uuid).await;
        let export_status = vulns_export_status.unwrap().status.unwrap();
        total_chunks = export_status.total_chunks.unwrap();
        export_status_string = export_status.status.unwrap();
    }

    for chunk_id in 0..total_chunks {
        let chunk = openapi::apis::exports_api::exports_vulns_download_chunk(&configuration, &export_uuid, chunk_id).await;
        let chunk = chunk.unwrap();
        chunk.severity.unwrap();
        //metrics.tenable_vulnerability_severity.get_or_create(&()).inc();
        TENABLE_VULNERABILITY_INFO.with_label_values(&["Critical"]).inc();
    }
}