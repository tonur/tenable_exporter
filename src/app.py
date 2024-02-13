from prometheus_client import start_http_server, Gauge
from tenable.io import TenableIO
import argparse
import logging
import time

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Tenable exporter for Prometheus. Provide your Access Key and Secret Key to scrape to expose Prometheus metrics.")
    parser.add_argument("--tenable.access_key", required=True, help="Tenable Access Key")
    parser.add_argument("--tenable.secret_key", required=True, help="Tenable Secret Key")
    parser.add_argument("--tenable.interval", "-i", default=600, type=int, help="Polling interval for requesting data from Tenable API in seconds")
    parser.add_argument("--tenable.timeout", default=30, type=int, help="Timeout for requests against Tenable API")
    parser.add_argument("--web.listen_address", default=":9521", help="Address on which to expose metrics")
    # Add other flags as needed
    args = parser.parse_args()

    # Set up logging
    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')


# Create Prometheus metrics
vulnerability_metric = Gauge('tenable_vulnerability_info', 'Tenable Vulnerability Information',
                             ['asset_uuid', 'plugin_id', 'plugin_name', 'severity', 'state', 'source'])

def fetch_tenable_data(tio_access_key, tio_secret_key):
    try:
        tio = TenableIO(tio_access_key, tio_secret_key)

        vulnerabilities = tio.exports.vulns()
        for vulnerability in vulnerabilities:
            asset = vulnerability.get('asset', {})
            plugin = vulnerability.get('plugin', {})

            vulnerability_metric.labels(
                asset_uuid=asset.get('uuid', ''),
                plugin_id=plugin.get('id', ''),
                plugin_name=plugin.get('name', ''),
                severity=vulnerability.get('severity', ''),
                state=vulnerability.get('state', ''),
                source=vulnerability.get('source', '')
            ).set(1)

    except Exception as e:
        logging.exception(f"Error fetching Tenable data: {e}")

def aggregation_key(vulnerability):
    asset = vulnerability.get('asset', {})
    plugin = vulnerability.get('plugin', {})
    return f"{asset.get('uuid', '')}_{plugin.get('id', '')}_{plugin.get('name', '')}_{vulnerability.get('severity', '')}_{vulnerability.get('state', '')}_{vulnerability.get('source', '')}"

def aggregate_vulnerabilities(vulnerabilities):
    aggregate_results = {}
    logging.debug(f"Input vulnerabilities to aggregate was: {vulnerabilities}")
    for vulnerability in vulnerabilities:
        asset = vulnerability.get('asset', {})
        plugin = vulnerability.get('plugin', {})
        key = aggregation_key(vulnerability)
        if key not in aggregate_results:
            aggregate_results[key] = {
                'uuid': asset.get('uuid', ''),
                'id': plugin.get('id', ''),
                'name': plugin.get('name', ''),
                'severity': vulnerability.get('severity', ''),
                'state': vulnerability.get('state', ''),
                'source': vulnerability.get('source', ''),
            }
        aggregate_results[key]['count'] += 1
        logging.debug(f"Added aggregation for vulnerability{vulnerability} with key {key}")

    output = list(aggregate_results.values())
    logging.debug(f"Output of aggregation was: {output}")
    return output

def update_metrics():
    fetch_tenable_data(args.tenable.access_key, args.tenable.secret_key)

update_interval_seconds = 300
start_http_server(args.tenable.listen_address)

while True:
    update_metrics()
    time.sleep(update_interval_seconds)