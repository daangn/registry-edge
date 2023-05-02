# Contribution Guide

## Run local server

Run `yarn wrangler dev`

## Testing Registry API

You can test registry API via [Podman](https://podman.io/) (recommended over docker for ease of testing locally)

e.g. Push image

```bash
podman pull library/ubuntu
podman tag ubuntu 192.168.0.1:8787/library/ubuntu

# Pass `--tls-verify=false` option to test local server
# Note that the loopback host may not works
podman push --tls-verify=false 192.168.0.1:8787/library/ubuntu
```
