from prometheus_client import start_http_server, Gauge
from tenable.io import TenableIO
import logging
import os
import time
import signal

def signal_handler(sig, frame):
    logging.info("Received termination signal. Exiting gracefully.")
    exit(0)

signal.signal(signal.SIGINT, signal_handler)

# Create Prometheus metrics
vulnerability_metric = Gauge('tenable_vulnerability_info', 'Tenable Vulnerability Information',
                             ['asset_uuid', 'plugin_id', 'plugin_name', 'severity', 'state', 'source'])

def get_environment_variable(variable):
    return os.environ.get(variable).strip()

def fetch_tenable_data(tio_access_key, tio_secret_key):
    try:
        tio = TenableIO(tio_access_key, tio_secret_key)

        for vuln in tio.exports.vulns():
            asset = vuln.get('asset', {})
            plugin = vuln.get('plugin', {})

            vulnerability_metric.labels(
                asset_uuid=asset.get('uuid', ''),
                plugin_id=plugin.get('id', ''),
                plugin_name=plugin.get('name', ''),
                severity=vuln.get('severity', ''),
                state=vuln.get('state', ''),
                source=vuln.get('source', '')
            ).set(1)

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