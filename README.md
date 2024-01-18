# Tenable-Exporter
Simple Tenable exporter to get Open-Telemetry metrics of Tenable Scans and Vulnerabilities.

## Usage
Run the Docker image with the following variables:
``` yaml
TIO_ACCESS_KEY: 
TIO_SECRET_KEY: 
SECURITYCENTER_NETWORK_ADDRESS: 
SC_USERNAME: 
SC_PASSWORD: 
```
``` sh
docker run -p 8080:<your_port> -e TIO_ACCESS_KEY=your_access_key -e TIO_SECRET_KEY=your_secret_key -e SECURITYCENTER_NETWORK_ADDRESS=your_sc_address -e SC_USERNAME=your_sc_username -e SC_PASSWORD=your_sc_password tonur/tenable-exporter
```

You need to specify either (TIO_ACCESS_KEY, TIO_SECRET_KEY, SECURITYCENTER_NETWORK_ADDRESS), (SC_USERNAME, SC_PASSWORD) or all of them to have this image do anything. In the case of only SC Environment Variables supplied, only the Scanning metrics is updated and in the case of only TIO + Network Address supplied, only the Vulnerabilities is updated.