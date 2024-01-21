import os
from prometheus_client import start_http_server, Gauge
import schedule
import time
from tenable.io import TenableIO
from tenable.sc import TenableSC

tio_variables = ['TIO_ACCESS_KEY', 'TIO_SECRET_KEY']
sc_variables = ['SECURITYCENTER_NETWORK_ADDRESS', 'SC_USERNAME', 'SC_PASSWORD']

missing_tio_variables = [variable for variable in tio_variables if os.environ.get(variable) is None]
missing_sc_variables = [variable for variable in sc_variables if os.environ.get(variable) is None]

# Create Prometheus metrics
if not missing_tio_variables:
    tenable_cloud_metrics = Gauge('tenable_cloud_metrics', 'Tenable Cloud Scans', ['id', 'uuid', 'name', 'status'])
if not missing_sc_variables:
    security_center_metrics = Gauge('security_center_metrics', 'Tenable Security Center Vulnerabilities', ['ip', 'pluginID', 'pluginName'])


def get_environment_variable(variable):
    return os.environ.get(variable).strip()

# Function to update metrics with new data
def update_metrics():
    if not missing_tio_variables:
        tio_access_key = get_environment_variable('TIO_ACCESS_KEY')
        tio_secret_key = get_environment_variable('TIO_SECRET_KEY')
        report_tenable_cloud_metrics(tio_access_key, tio_secret_key)

    if not missing_sc_variables:
        sc_network_address = get_environment_variable('SECURITYCENTER_NETWORK_ADDRESS')
        sc_username = get_environment_variable('SC_USERNAME')
        sc_password = get_environment_variable('SC_PASSWORD')
        report_security_center_metrics(sc_network_address, sc_username, sc_password)

def report_tenable_cloud_metrics(tio_access_key, tio_secret_key):
    # Tenable.io
    tio = TenableIO(tio_access_key, tio_secret_key)
    for vuln in tio.exports.vulns():
        print("Vuln was {}".format(vuln))
        tenable_cloud_metrics.labels(vuln)

def report_security_center_metrics(sc_network_address, sc_username, sc_password):
    # Tenable.sc
    sc = TenableSC(sc_network_address)
    sc.login(sc_username, sc_password)
    for vuln in sc.analysis.vulns():
        security_center_metrics.info({
            'ip':vuln['ip'],
            'pluginID': vuln['pluginID'],
            'pluginName': vuln['pluginName']
        })

# Schedule the update_metrics function to run every 5 minutes
schedule.every(1).seconds.do(update_metrics)

# Start the Prometheus HTTP server
if __name__ == '__main__':
    # Start Prometheus HTTP server on port 8000
    start_http_server(8080)

    while True:
        # Run pending scheduled tasks
        schedule.run_pending()

        # Sleep for 1 second to avoid excessive CPU usage
        time.sleep(1)
