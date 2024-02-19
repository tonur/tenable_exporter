use std::sync::Mutex;
use async_std::task;
use clap::Parser;
use openapi::apis::configuration;
use openapi::apis::configuration::ApiKey;
use tokio::time::interval;
use tokio::time::Duration;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;

use openapi;

macro_rules! get_field_name {
    ($struct_name:ident . $field:ident) => {
        stringify!($field)
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = web::Data::new(AppArguments::parse());

    let metrics = web::Data::new(Metrics {
        tenable_asset_compliance: Family::new_with_constructor(|| Counter::default()),
        tenable_vulnerability_age: Family::new_with_constructor(|| Gauge::default()),
        tenable_remediation_time: Family::new_with_constructor(|| Gauge::default()),
        tenable_asset_inventory: Family::new_with_constructor(|| Gauge::default()),
        tenable_vulnerability_severity: Family::new_with_constructor(|| Counter::default()),
    });

    let mut state = AppState {
        registry: Registry::default(),
    };

    state.registry.register(
        get_field_name!(metrics.tenable_asset_compliance),
        "Tenable Asset Compliance",
        metrics.tenable_asset_compliance.clone(),
    );
    state.registry.register(
        get_field_name!(metrics.tenable_vulnerability_age),
        "Tenable Vulnerability Age",
        metrics.tenable_vulnerability_age.clone(),
    );
    state.registry.register(
        get_field_name!(metrics.tenable_remediation_time),
        "Tenable Remediation Time",
        metrics.tenable_remediation_time.clone(),
    );
    state.registry.register(
        get_field_name!(metrics.tenable_asset_inventory),
        "Tenable Asset Inventory",
        metrics.tenable_asset_inventory.clone(),
    );
    state.registry.register(
        get_field_name!(metrics.tenable_vulnerability_severity),
        "Tenable Vulnerability Severity",
        metrics.tenable_vulnerability_severity.clone(),
    );

    let state = web::Data::new(Mutex::new(state));

    actix_rt::spawn(async move {
        let mut interval = interval(Duration::from_secs(300));

        loop {
            interval.tick().await;
            fetch_data(metrics.clone(), args.clone()).await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::resource("/metrics").route(web::get().to(metrics_handler)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub async fn metrics_handler(state: web::Data<Mutex<AppState>>) -> Result<HttpResponse> {
    let state = state.lock().unwrap();
    let mut body = String::new();
    encode(&mut body, &state.registry).unwrap();
    Ok(HttpResponse::Ok()
        .content_type("application/openmetrics-text; version=1.0.0; charset=utf-8")
        .body(body))
}

pub async fn fetch_data(metrics: web::Data<Metrics>, arguments: web::Data<AppArguments>) {
    // metrics.requests.get_or_create(&()).inc();
    // metrics.elapsed.get_or_create(&()).observe(10f64);
    // "okay".to_string()

    // Configuration should be like: --header 'X-ApiKeys: accessKey=;secretKey='
    let mut configuration = configuration::Configuration::default();
    configuration.api_key = Some(ApiKey {
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
        
        metrics.tenable_vulnerability_severity.get_or_create(&()).inc();
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AppArguments {
    #[arg(long)]
    tio_access_key: String,
    #[arg(long)]
    tio_secret_key: String,
}

pub struct Metrics {
    tenable_asset_compliance: Family<(), Counter>,
    tenable_vulnerability_age: Family<(), Gauge>,
    tenable_remediation_time: Family<(), Gauge>,
    tenable_asset_inventory: Family<(), Gauge>,
    tenable_vulnerability_severity: Family<(), Counter>,
}

pub struct AppState {
    pub registry: Registry,
}