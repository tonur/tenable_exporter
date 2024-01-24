from prometheus_client import start_http_server, Counter
from tenable.io import TenableIO
import logging
import os
import time
import signal

signal.signal(signal.SIGINT, signal.SIG_DFL)

# Create Prometheus metrics
asset_vulnerability_metric = Counter('tenable_asset_vulnerabilities', 'Tenable Asset Vulnerabilities',
                                     ['asset_uuid', 'severity'])

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
