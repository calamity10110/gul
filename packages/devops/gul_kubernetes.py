"""
GUL Kubernetes Integration
Kubernetes manifest generation and deployment helpers.

Status: ✅ Implemented
Priority: Critical
Phase: 1
"""

from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field

__version__ = "0.1.0"
__all__ = ['Deployment', 'Service', 'Ingress', 'ConfigMap', 'Secret', 'HPA', 'KubernetesManifest']

@dataclass
class KubernetesManifest:
    """Base class for Kubernetes manifests"""
    
    api_version: str
    kind: str
    metadata: Dict[str, Any]
    spec: Dict[str, Any]
    
    def to_dict(self) -> Dict:
        """Convert to dictionary"""
        return {
            'apiVersion': self.api_version,
            'kind': self.kind,
            'metadata': self.metadata,
            'spec': self.spec
        }
    
    def to_yaml(self) -> str:
        """Convert to YAML"""
        import yaml
        return yaml.dump(self.to_dict(), default_flow_style=False)

class Deployment:
    """Kubernetes Deployment"""
    
    def __init__(
        self,
        name: str,
        namespace: str = "default",
        replicas: int = 3,
        image: str = "",
        port: int = 8080
    ):
        self.name = name
        self.namespace = namespace
        self.replicas = replicas
        self.image = image
        self.port = port
        self.env_vars: Dict[str, str] = {}
        self.labels: Dict[str, str] = {'app': name}
        self.resources: Dict[str, Dict] = {}
        self.probes: Dict[str, Dict] = {}
    
    def add_env(self, key: str, value: str) -> 'Deployment':
        """Add environment variable"""
        self.env_vars[key] = value
        return self
    
    def add_label(self, key: str, value: str) -> 'Deployment':
        """Add label"""
        self.labels[key] = value
        return self
    
    def set_resources(
        self,
        cpu_request: str = "100m",
        memory_request: str = "128Mi",
        cpu_limit: str = "500m",
        memory_limit: str = "512Mi"
    ) -> 'Deployment':
        """Set resource requests and limits"""
        self.resources = {
            'requests': {'cpu': cpu_request, 'memory': memory_request},
            'limits': {'cpu': cpu_limit, 'memory': memory_limit}
        }
        return self
    
    def add_liveness_probe(
        self,
        path: str = "/health",
        port: Optional[int] = None,
        initial_delay: int = 30,
        period: int = 10
    ) -> 'Deployment':
        """Add liveness probe"""
        self.probes['livenessProbe'] = {
            'httpGet': {
                'path': path,
                'port': port or self.port
            },
            'initialDelaySeconds': initial_delay,
            'periodSeconds': period
        }
        return self
    
    def add_readiness_probe(
        self,
        path: str = "/ready",
        port: Optional[int] = None,
        initial_delay: int = 5,
        period: int = 5
    ) -> 'Deployment':
        """Add readiness probe"""
        self.probes['readinessProbe'] = {
            'httpGet': {
                'path': path,
                'port': port or self.port
            },
            'initialDelaySeconds': initial_delay,
            'periodSeconds': period
        }
        return self
    
    def to_manifest(self) -> KubernetesManifest:
        """Generate Kubernetes manifest"""
        container = {
            'name': self.name,
            'image': self.image,
            'ports': [{'containerPort': self.port}]
        }
        
        if self.env_vars:
            container['env'] = [
                {'name': k, 'value': v}
                for k, v in self.env_vars.items()
            ]
        
        if self.resources:
            container['resources'] = self.resources
        
        container.update(self.probes)
        
        return KubernetesManifest(
            api_version='apps/v1',
            kind='Deployment',
            metadata={
                'name': self.name,
                'namespace': self.namespace,
                'labels': self.labels
            },
            spec={
                'replicas': self.replicas,
                'selector': {'matchLabels': self.labels},
                'template': {
                    'metadata': {'labels': self.labels},
                    'spec': {'containers': [container]}
                }
            }
        )

class Service:
    """Kubernetes Service"""
    
    def __init__(
        self,
        name: str,
        namespace: str = "default",
        service_type: str = "ClusterIP",
        port: int = 80,
        target_port: int = 8080
    ):
        self.name = name
        self.namespace = namespace
        self.service_type = service_type
        self.port = port
        self.target_port = target_port
        self.labels = {'app': name}
    
    def to_manifest(self) -> KubernetesManifest:
        """Generate Kubernetes manifest"""
        return KubernetesManifest(
            api_version='v1',
            kind='Service',
            metadata={
                'name': self.name,
                'namespace': self.namespace
            },
            spec={
                'selector': self.labels,
                'type': self.service_type,
                'ports': [{
                    'port': self.port,
                    'targetPort': self.target_port,
                    'protocol': 'TCP'
                }]
            }
        )

class Ingress:
    """Kubernetes Ingress"""
    
    def __init__(
        self,
        name: str,
        namespace: str = "default",
        host: str = "example.com",
        service_name: str = "",
        service_port: int = 80
    ):
        self.name = name
        self.namespace = namespace
        self.host = host
        self.service_name = service_name
        self.service_port = service_port
        self.tls_enabled = False
        self.annotations: Dict[str, str] = {}
    
    def enable_tls(self, secret_name: str) -> 'Ingress':
        """Enable TLS"""
        self.tls_enabled = True
        self.tls_secret = secret_name
        return self
    
    def add_annotation(self, key: str, value: str) -> 'Ingress':
        """Add annotation"""
        self.annotations[key] = value
        return self
    
    def to_manifest(self) -> KubernetesManifest:
        """Generate Kubernetes manifest"""
        spec = {
            'ingressClassName': 'nginx',
            'rules': [{
                'host': self.host,
                'http': {
                    'paths': [{
                        'path': '/',
                        'pathType': 'Prefix',
                        'backend': {
                            'service': {
                                'name': self.service_name,
                                'port': {'number': self.service_port}
                            }
                        }
                    }]
                }
            }]
        }
        
        if self.tls_enabled:
            spec['tls'] = [{
                'hosts': [self.host],
                'secretName': self.tls_secret
            }]
        
        return KubernetesManifest(
            api_version='networking.k8s.io/v1',
            kind='Ingress',
            metadata={
                'name': self.name,
                'namespace': self.namespace,
                'annotations': self.annotations
            },
            spec=spec
        )

class HPA:
    """Horizontal Pod Autoscaler"""
    
    def __init__(
        self,
        name: str,
        namespace: str = "default",
        target_deployment: str = "",
        min_replicas: int = 2,
        max_replicas: int = 10,
        cpu_percent: int = 70
    ):
        self.name = name
        self.namespace = namespace
        self.target_deployment = target_deployment
        self.min_replicas = min_replicas
        self.max_replicas = max_replicas
        self.cpu_percent = cpu_percent
    
    def to_manifest(self) -> KubernetesManifest:
        """Generate Kubernetes manifest"""
        return KubernetesManifest(
            api_version='autoscaling/v2',
            kind='HorizontalPodAutoscaler',
            metadata={
                'name': self.name,
                'namespace': self.namespace
            },
            spec={
                'scaleTargetRef': {
                    'apiVersion': 'apps/v1',
                    'kind': 'Deployment',
                    'name': self.target_deployment
                },
                'minReplicas': self.min_replicas,
                'maxReplicas': self.max_replicas,
                'metrics': [{
                    'type': 'Resource',
                    'resource': {
                        'name': 'cpu',
                        'target': {
                            'type': 'Utilization',
                            'averageUtilization': self.cpu_percent
                        }
                    }
                }]
            }
        )

class ConfigMap:
    """Kubernetes ConfigMap"""
    
    def __init__(self, name: str, namespace: str = "default"):
        self.name = name
        self.namespace = namespace
        self.data: Dict[str, str] = {}
    
    def add_data(self, key: str, value: str) -> 'ConfigMap':
        """Add data"""
        self.data[key] = value
        return self
    
    def to_manifest(self) -> KubernetesManifest:
        """Generate Kubernetes manifest"""
        return KubernetesManifest(
            api_version='v1',
            kind='ConfigMap',
            metadata={
                'name': self.name,
                'namespace': self.namespace
            },
            spec={}
        )._replace_spec_with_data(self.data)
    
def _replace_spec_with_data(manifest: KubernetesManifest, data: Dict) -> Dict:
    """Replace spec with data for ConfigMap/Secret"""
    d = manifest.to_dict()
    del d['spec']
    d['data'] = data
    return d

class Secret:
    """Kubernetes Secret"""
    
    def __init__(self, name: str, namespace: str = "default"):
        self.name = name
        self.namespace = namespace
        self.data: Dict[str, str] = {}
    
    def add_data(self, key: str, value: str) -> 'Secret':
        """Add data (base64 will be applied)"""
        import base64
        self.data[key] = base64.b64encode(value.encode()).decode()
        return self
    
    def to_dict(self) -> Dict:
        """Generate manifest dict"""
        return {
            'apiVersion': 'v1',
            'kind': 'Secret',
            'metadata': {
                'name': self.name,
                'namespace': self.namespace
            },
            'type': 'Opaque',
            'data': self.data
        }

# Helper to generate full stack
def generate_app_manifests(
    name: str,
    image: str,
    host: str,
    namespace: str = "default"
) -> List[Dict]:
    """Generate complete app manifests"""
    
    # Deployment
    deployment = (Deployment(name, namespace, image=image)
        .set_resources()
        .add_liveness_probe()
        .add_readiness_probe())
    
    # Service
    service = Service(name, namespace)
    
    # Ingress
    ingress = Ingress(name, namespace, host, name)
    
    # HPA
    hpa = HPA(name, namespace, name)
    
    return [
        deployment.to_manifest().to_dict(),
        service.to_manifest().to_dict(),
        ingress.to_manifest().to_dict(),
        hpa.to_manifest().to_dict()
    ]
