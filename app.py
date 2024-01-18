import os
import logging
from flask import Flask, Response
from prometheus_client import Gauge, generate_latest
from tenable.io import TenableIO
from tenable.sc import TenableSC

app = Flask(__name__)

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)

# Define Prometheus metrics
enable_scan_metric = os.environ.get('ENABLE_SCAN_METRIC', 'true').lower() == 'true'
enable_vuln_metric = os.environ.get('ENABLE_VULN_METRIC', 'true').lower() == 'true'

scan_metric = Gauge('tenable_scans', 'Tenable Scans', ['id', 'uuid', 'name', 'status'])
vuln_metric = Gauge('tenable_vulnerabilities', 'Tenable Vulnerabilities', ['ip', 'pluginID', 'pluginName'])

def check_environment_variables():
    tio_variables = ['TIO_ACCESS_KEY', 'TIO_SECRET_KEY']
    sc_variables = ['SECURITYCENTER_NETWORK_ADDRESS', 'SC_USERNAME', 'SC_PASSWORD']

    missing_tio_variables = [variable for variable in tio_variables if os.environ.get(variable) is None]
    missing_sc_variables = [variable for variable in sc_variables if os.environ.get(variable) is None]

    if missing_tio_variables:
        logging.warning(f"Missing environment variables for Tenable.io: {', '.join(missing_tio_variables)}")

    if missing_sc_variables:
        logging.warning(f"Missing environment variables for Tenable.sc: {', '.join(missing_sc_variables)}")

    if missing_tio_variables and missing_sc_variables:
        logging.error("No environment variables for Tenable.io and Tenable.sc, nothing to scrape")
        return False
    return True

def enable_metrics():
    if enable_scan_metric:
        logging.info("Tenable Scan Status metric is enabled.")
    if enable_vuln_metric:
        logging.info("Tenable Vulnerability metric is enabled.")

def scans(tio_access_key, tio_secret_key):
    # Tenable.io
    tio = TenableIO(tio_access_key, tio_secret_key)
    for scan in tio.scans.list():
        scan_metric.labels(id=scan['id'], uuid=scan['uuid'], name=scan['name'], status=scan['status']).set(1)

def vulns(sc_network_address, sc_username, sc_password):
    # Tenable.sc
    sc = TenableSC(sc_network_address)
    sc.login(sc_username, sc_password)
    for vuln in sc.analysis.vulns():
        vuln_metric.labels(ip=vuln['ip'], pluginID=vuln['pluginID'], pluginName=vuln['pluginName']).set(1)

@app.route('/metrics')
def metrics():
    if not check_environment_variables():
        return Response("ERROR: Missing environment variables", status=500)

    enable_metrics()

    if enable_scan_metric:
        tio_access_key = os.environ['TIO_ACCESS_KEY']
        tio_secret_key = os.environ['TIO_SECRET_KEY']
        scans(tio_access_key, tio_secret_key)

    if enable_vuln_metric:
        sc_network_address = os.environ['SECURITYCENTER_NETWORK_ADDRESS']
        sc_username = os.environ['SC_USERNAME']
        sc_password = os.environ['SC_PASSWORD']
        vulns(sc_network_address, sc_username, sc_password)

    return Response(generate_latest(), mimetype='text/plain')

@app.route('/healthz')
def healthz():
    if not check_environment_variables():
        return Response("ERROR: Missing environment variables", status=500)
    return Response("OK", status=200)
