FROM gcr.io/distroless/python3-debian12

COPY . /opt/tenable-exporter

ENTRYPOINT ["/opt/tenable-exporter/app.py"]