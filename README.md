# Tenable-Exporter
Simple Tenable exporter to get Open-Telemetry metrics of Tenable Vulnerabilities.

## Usage
Run the Docker image with the following variables:
``` yaml
TIO_ACCESS_KEY: 
TIO_SECRET_KEY: 
```
``` sh
docker run -p 8080:<your_port> -e TIO_ACCESS_KEY=your_access_key -e TIO_SECRET_KEY=your_secret_key tonur/tenable-exporter
```