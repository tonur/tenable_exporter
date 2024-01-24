from prometheus_client import start_http_server, Gauge, Counter
from tenable.io import TenableIO
import logging
import os
import time
import datetime
import signal

signal.signal(signal.SIGINT, signal.SIG_DFL)

# Create Prometheus metrics
asset_vulnerability_metric = Counter('tenable_asset_vulnerabilities', 'Tenable Asset Vulnerabilities',
                                     ['asset_uuid', 'severity'])
asset_compliance_metric = Gauge('tenable_asset_compliance', 'Tenable Asset Compliance',
                                ['asset_uuid', 'policy_name'])
vulnerability_age_metric = Gauge('tenable_vulnerability_age', 'Tenable Vulnerability Age',
                                 ['vulnerability_id'])
remediation_time_metric = Gauge('tenable_remediation_time', 'Tenable Remediation Time',
                                ['vulnerability_id'])
asset_inventory_metric = Gauge('tenable_asset_inventory', 'Tenable Asset Inventory',
                               ['asset_type'])
vulnerability_severity_metric = Counter('tenable_vulnerability_severity', 'Tenable Vulnerability Severity',
                                        ['severity'])

def get_environment_variable(variable):
    return os.environ.get(variable).strip()

def fetch_tenable_data(tio_access_key, tio_secret_key):
    try:
        tio = TenableIO(tio_access_key, tio_secret_key)

        for vuln in tio.exports.vulns():
            asset = vuln.get('asset', {})
            plugin = vuln.get('plugin', {})

            asset_vulnerability_metric.labels(
                asset_uuid=asset.get('uuid', ''),
                severity=vuln.get('severity', '')
            ).inc()

            last_found_str = vuln.get('last_found', '')
            last_found_dt = datetime.datetime.strptime(last_found_str, '%Y-%m-%dT%H:%M:%S.%fZ')
            last_found_unix = int(last_found_dt.timestamp())

            vulnerability_age_metric.labels(
                vulnerability_id=vuln.get('plugin_id', '')
            ).set(time.time() - last_found_unix)

            remediation_time_metric.labels(
                vulnerability_id=vuln.get('plugin_id', '')
            ).set(vuln.get('remediation', {}).get('days_to_remediate', 0))

            asset_inventory_metric.labels(
                asset_type=asset.get('type', '')
            ).inc()

            vulnerability_severity_metric.labels(
                severity=vuln.get('severity', '')
            ).inc()

    except Exception as e:
        logging.exception(f"Error fetching Tenable data: {e}")

def update_metrics():
    tio_access_key = get_environment_variable('TIO_ACCESS_KEY')
    tio_secret_key = get_environment_variable('TIO_SECRET_KEY')

    if not tio_access_key or not tio_secret_key:
        logging.error('Environment variables TIO_ACCESS_KEY and TIO_SECRET_KEY must be set')
        raise ValueError('Missing TIO_ACCESS_KEY or TIO_SECRET_KEY')

    fetch_tenable_data(tio_access_key, tio_secret_key)

update_interval_seconds = 300
start_http_server(8080)

while True:
    update_metrics()
    time.sleep(update_interval_seconds)
