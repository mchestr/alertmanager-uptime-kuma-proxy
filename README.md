# alertmanager-uptime-kuma-proxy
Proxy for AlertManager heartbeat and Uptime Kuma.

This is a small binary that essentially converts a `HTTP POST` into an `HTTP GET` since uptime-kuma push requests only support `HTTP GET`...

### Usage

```
export UPTIME_KUMA_URL="https://some.example.com/api/push/sometoken"
docker run -p 3000:3000 alertmanager-uptime-kuma-proxy
curl -X POST http://localhost:3000
```
