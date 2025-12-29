# GUL Production Deployment Guide

**Version**: 0.13.0 | **Updated**: 2025-12-28

Complete guide for deploying GUL applications to production.

---

## Overview

This guide covers deploying GUL applications as:

- Containerized microservices (Docker/Kubernetes)
- Serverless functions
- Traditional VMs
- SaaS platforms

---

## Prerequisites

- GUL 0.13.0+
- Docker 20.10+
- Kubernetes 1.24+ (optional)
- Cloud provider account (AWS/GCP/Azure)

---

## 1. Containerization with Docker

### Basic Dockerfile

```dockerfile
# Multi-stage build for optimal size
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build the GUL application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/gul /usr/local/bin/

# Create non-root user
RUN useradd -m -u 1000 gul
USER gul

WORKDIR /home/gul

# Health check
HEALTHCHECK --interval=30s --timeout=3s \
    CMD ["/usr/local/bin/gul", "health"] || exit 1

ENTRYPOINT ["gul"]
CMD ["serve"]
```

### Build and Run

```bash
# Build image
docker build -t gul-app:0.13.0 .

# Run container
docker run -d \
    --name gul-app \
    -p 8080:8080 \
    -e GUL_ENV=production \
    -e GUL_LOG_LEVEL=info \
    gul-app:0.13.0

# View logs
docker logs -f gul-app

# Stop container
docker stop gul-app
```

### Docker Compose

```yaml
version: "3.8"

services:
  gul-app:
    image: gul-app:0.13.0
    build: .
    ports:
      - "8080:8080"
    environment:
      - GUL_ENV=production
      - GUL_LOG_LEVEL=info
      - DATABASE_URL=postgresql://user:pass@db:5432/guldb
      - REDIS_URL=redis://redis:6379
    depends_on:
      - db
      - redis
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "gul", "health"]
      interval: 30s
      timeout: 3s
      retries: 3

  db:
    image: postgres:16-alpine
    environment:
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=pass
      - POSTGRES_DB=guldb
    volumes:
      - postgres-data:/var/lib/postgresql/data
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    volumes:
      - redis-data:/data
    restart: unless-stopped

volumes:
  postgres-data:
  redis-data:
```

---

## 2. Kubernetes Deployment

### Namespace

```yaml
# namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: gul-production
```

### ConfigMap

```yaml
# configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: gul-config
  namespace: gul-production
data:
  GUL_ENV: "production"
  GUL_LOG_LEVEL: "info"
  GUL_PORT: "8080"
```

### Secrets

```yaml
# secret.yaml
apiVersion: v1
kind: Secret
metadata:
  name: gul-secrets
  namespace: gul-production
type: Opaque
stringData:
  DATABASE_URL: "postgresql://user:pass@postgres:5432/guldb"
  REDIS_URL: "redis://redis:6379"
  JWT_SECRET: "your-secret-key"
```

### Deployment

```yaml
# deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: gul-app
  namespace: gul-production
  labels:
    app: gul
    version: v0.13.0
spec:
  replicas: 3
  selector:
    matchLabels:
      app: gul
  template:
    metadata:
      labels:
        app: gul
        version: v0.13.0
    spec:
      containers:
        - name: gul
          image: gul-app:0.13.0
          imagePullPolicy: IfNotPresent
          ports:
            - containerPort: 8080
              name: http
          envFrom:
            - configMapRef:
                name: gul-config
            - secretRef:
                name: gul-secrets
          resources:
            requests:
              memory: "256Mi"
              cpu: "250m"
            limits:
              memory: "512Mi"
              cpu: "500m"
          livenessProbe:
            httpGet:
              path: /health
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 10
            timeoutSeconds: 3
          readinessProbe:
            httpGet:
              path: /ready
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 5
            timeoutSeconds: 3
          startupProbe:
            httpGet:
              path: /health
              port: 8080
            failureThreshold: 30
            periodSeconds: 10
```

### Service

```yaml
# service.yaml
apiVersion: v1
kind: Service
metadata:
  name: gul-service
  namespace: gul-production
spec:
  selector:
    app: gul
  type: ClusterIP
  ports:
    - port: 80
      targetPort: 8080
      protocol: TCP
      name: http
```

### Ingress

```yaml
# ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: gul-ingress
  namespace: gul-production
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/rate-limit: "100"
spec:
  ingressClassName: nginx
  tls:
    - hosts:
        - api.example.com
      secretName: gul-tls
  rules:
    - host: api.example.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: gul-service
                port:
                  number: 80
```

### HorizontalPodAutoscaler

```yaml
# hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: gul-hpa
  namespace: gul-production
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: gul-app
  minReplicas: 3
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
    - type: Resource
      resource:
        name: memory
        target:
          type: Utilization
          averageUtilization: 80
```

### Deploy to Kubernetes

```bash
# Apply all manifests
kubectl apply -f namespace.yaml
kubectl apply -f configmap.yaml
kubectl apply -f secret.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
kubectl apply -f ingress.yaml
kubectl apply -f hpa.yaml

# Check status
kubectl get pods -n gul-production
kubectl get svc -n gul-production
kubectl get ing -n gul-production

# View logs
kubectl logs -f deployment/gul-app -n gul-production

# Scale manually
kubectl scale deployment gul-app --replicas=5 -n gul-production
```

---

## 3. Helm Chart Deployment

### Chart Structure

```text
gul-chart/
├── Chart.yaml
├── values.yaml
└── templates/
    ├── deployment.yaml
    ├── service.yaml
    ├── ingress.yaml
    ├── configmap.yaml
    ├── secret.yaml
    └── hpa.yaml
```

### Chart.yaml

```yaml
apiVersion: v2
name: gul
description: GUL Programming Language Platform
type: application
version: 0.13.0
appVersion: "0.13.0"
maintainers:
  - name: GUL Team
    email: team@gul-lang.org
```

### values.yaml

```yaml
replicaCount: 3

image:
  repository: gul-app
  pullPolicy: IfNotPresent
  tag: "0.13.0"

service:
  type: ClusterIP
  port: 80
  targetPort: 8080

ingress:
  enabled: true
  className: nginx
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
  hosts:
    - host: api.example.com
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: gul-tls
      hosts:
        - api.example.com

resources:
  limits:
    cpu: 500m
    memory: 512Mi
  requests:
    cpu: 250m
    memory: 256Mi

autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 10
  targetCPUUtilizationPercentage: 70
  targetMemoryUtilizationPercentage: 80

env:
  GUL_ENV: production
  GUL_LOG_LEVEL: info

secrets:
  DATABASE_URL: ""
  REDIS_URL: ""
  JWT_SECRET: ""
```

### Install with Helm

```bash
# Install chart
helm install gul ./gul-chart \
    --namespace gul-production \
    --create-namespace \
    --set secrets.DATABASE_URL="postgresql://..." \
    --set secrets.REDIS_URL="redis://..." \
    --set secrets.JWT_SECRET="secret"

# Upgrade
helm upgrade gul ./gul-chart \
    --namespace gul-production

# Rollback
helm rollback gul 1 --namespace gul-production

# Uninstall
helm uninstall gul --namespace gul-production
```

---

## 4. Monitoring & Observability

### Prometheus Metrics

```yaml
# servicemonitor.yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: gul-metrics
  namespace: gul-production
spec:
  selector:
    matchLabels:
      app: gul
  endpoints:
    - port: http
      path: /metrics
      interval: 30s
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "GUL Application Metrics",
    "panels": [
      {
        "title": "Request Rate",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])"
          }
        ]
      },
      {
        "title": "Response Time",
        "targets": [
          {
            "expr": "histogram_quantile(0.99, rate(http_duration_seconds_bucket[5m]))"
          }
        ]
      },
      {
        "title": "Error Rate",
        "targets": [
          {
            "expr": "rate(http_errors_total[5m])"
          }
        ]
      }
    ]
  }
}
```

---

## 5. CI/CD Pipeline

### GitHub Actions

```yaml
name: Deploy to Production

on:
  push:
    tags:
      - "v*"

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build Docker image
        run: docker build -t gul-app:${{ github.ref_name }} .

      - name: Push to registry
        run: |
          echo "${{ secrets.DOCKER_PASSWORD }}" | docker login -u "${{ secrets.DOCKER_USERNAME }}" --password-stdin
          docker push gul-app:${{ github.ref_name }}

      - name: Deploy to Kubernetes
        uses: azure/k8s-deploy@v4
        with:
          manifests: |
            k8s/deployment.yaml
            k8s/service.yaml
          images: |
            gul-app:${{ github.ref_name }}
          namespace: gul-production
```

---

## 6. Production Checklist

### Pre-Deployment

- [ ] Environment variables configured
- [ ] Secrets stored securely
- [ ] Database migrations tested
- [ ] Load testing completed
- [ ] Security audit passed
- [ ] Backup strategy defined
- [ ] Monitoring configured
- [ ] Alerting rules set up
- [ ] Documentation updated
- [ ] Team trained on new features

### Post-Deployment

- [ ] Health checks passing
- [ ] Metrics being collected
- [ ] Logs aggregated
- [ ] Alerts configured
- [ ] Backup verified
- [ ] Performance acceptable
- [ ] Security headers enabled
- [ ] SSL/TLS configured
- [ ] Rate limiting active
- [ ] Auto-scaling tested

---

## 7. Troubleshooting

### Common Issues

**Container won't start**:

```bash
# Check logs
docker logs gul-app
kubectl logs deployment/gul-app -n gul-production

# Check events
kubectl describe pod <pod-name> -n gul-production
```

**High memory usage**:

```bash
# Check resource usage
kubectl top pods -n gul-production

# Adjust limits
kubectl patch deployment gul-app -n gul-production \
    -p '{"spec":{"template":{"spec":{"containers":[{"name":"gul","resources":{"limits":{"memory":"1Gi"}}}]}}}}'
```

**Database connection issues**:

```bash
# Test connection
kubectl run -it --rm debug --image=postgres:16 --restart=Never -- \
    psql postgresql://user:pass@postgres:5432/guldb
```

---

## 8. Best Practices

1. **Use Health Checks**: Implement `/health` and `/ready` endpoints
2. **Resource Limits**: Always set CPU and memory limits
3. **Horizontal Scaling**: Use HPA for automatic scaling
4. **Monitoring**: Collect metrics and logs
5. **Security**: Use secrets, non-root users, read-only filesystems
6. **Graceful Shutdown**: Handle SIGTERM properly
7. **Rolling Updates**: Use deployment strategies
8. **Backup**: Regular database and configuration backups
9. **Testing**: Test in staging environment first
10. **Documentation**: Keep runbooks up to date

---

## Next Steps

1. Review [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) for package implementation
2. Set up monitoring with Prometheus/Grafana
3. Configure alerting rules
4. Plan disaster recovery procedures
5. Establish on-call rotation

---

**Status**: Production Deployment Guide  
**For**: GUL 0.13.0+  
**Updated**: 2025-12-28
