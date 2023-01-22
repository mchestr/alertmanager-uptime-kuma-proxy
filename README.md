# alertmanager-uptime-kuma-proxy
Proxy for AlertManager heartbeat and Uptime Kuma.

This is a small binary that essentially converts a `HTTP POST` into an `HTTP GET` since uptime-kuma push requests only support `HTTP GET`...

Currently in use on my [k8s cluster](https://github.com/mchestr/home-cluster/blob/main/kubernetes/apps/monitoring/kube-prometheus-stack/app/helm-release.yaml#L121-L130)

### Usage

```
export UPTIME_KUMA_URL="https://some.example.com"
docker run -p 3000:3000 alertmanager-uptime-kuma-proxy
curl -X POST http://localhost:3000/api/push/sometoken
```
