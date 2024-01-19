import os
import logging
from flask import Flask, Response
from prometheus_client import Gauge, generate_latest
from tenable.io import TenableIO
from tenable.sc import TenableSC

app = Flask(__name__)

tio_variables = ['TIO_ACCESS_KEY', 'TIO_SECRET_KEY']
sc_variables = ['SECURITYCENTER_NETWORK_ADDRESS', 'SC_USERNAME', 'SC_PASSWORD']

missing_tio_variables = [variable for variable in tio_variables if os.environ.get(variable) is None]
missing_sc_variables = [variable for variable in sc_variables if os.environ.get(variable) is None]

def enable_metrics():
    if missing_tio_variables:
        logging.info("Tenable Cloud metrics is enabled.")
    if missing_sc_variables:
        logging.info("Tenable Security Center metric is enabled.")

tenable_cloud_metrics = Gauge('tenable_cloud_metrics', 'Tenable Cloud Scans', ['id', 'uuid', 'name', 'status'])
security_center_metrics = Gauge('security_center_metrics', 'Tenable Security Center Vulnerabilities', ['ip', 'pluginID', 'pluginName'])

def verify_environment_variables():
    if missing_tio_variables:
        logging.warning(f"Missing environment variables for Tenable.io: {', '.join(missing_tio_variables)}")

    if missing_sc_variables:
        logging.warning(f"Missing environment variables for Tenable.sc: {', '.join(missing_sc_variables)}")

    if missing_tio_variables and missing_sc_variables:
        logging.error("No environment variables for Tenable.io and Tenable.sc, nothing to scrape")
        return False
    return True

def tenable_cloud_metrics(tio_access_key, tio_secret_key):
    # Tenable.io
    tio = TenableIO(tio_access_key, tio_secret_key)
    for scan in tio.scans.list():
        tenable_cloud_metrics.labels(id=scan['id'], uuid=scan['uuid'], name=scan['name'], status=scan['status']).set(1)

def security_center_metrics(sc_network_address, sc_username, sc_password):
    # Tenable.sc
    sc = TenableSC(sc_network_address)
    sc.login(sc_username, sc_password)
    for vuln in sc.analysis.vulns():
        security_center_metrics.labels(ip=vuln['ip'], pluginID=vuln['pluginID'], pluginName=vuln['pluginName']).set(1)

@app.route('/metrics')
def metrics():
    if not verify_environment_variables():
        return Response("ERROR: Missing environment variables", status=500)

    enable_metrics()

    if missing_tio_variables:
        tio_access_key = os.environ['TIO_ACCESS_KEY']
        tio_secret_key = os.environ['TIO_SECRET_KEY']
        tenable_cloud_metrics(tio_access_key, tio_secret_key)

    if missing_sc_variables:
        sc_network_address = os.environ['SECURITYCENTER_NETWORK_ADDRESS']
        sc_username = os.environ['SC_USERNAME']
        sc_password = os.environ['SC_PASSWORD']
        security_center_metrics(sc_network_address, sc_username, sc_password)
    return Response(generate_latest(), mimetype='text/plain')

@app.route('/healthz')
def healthz():
    if not verify_environment_variables():
        return Response("ERROR: Missing environment variables", status=500)
    return Response("OK", status=200)


if __name__ == '__main__':
    from waitress import serve
    serve(app, port=8080)