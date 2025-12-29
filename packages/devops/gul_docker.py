"""
GUL Docker Integration
Helpers for Docker containerization.

Status: ✅ Implemented
Priority: Critical
Phase: 1
"""

from typing import Dict, List, Optional
from dataclasses import dataclass, field

__version__ = "0.1.0"
__all__ = ['DockerfileBuilder', 'ComposeService', 'ComposeConfig']

@dataclass
class DockerfileBuilder:
    """
    Builder for generating Dockerfiles
    
    Example:
        dockerfile = DockerfileBuilder(
            base_image="python:3.12-slim",
            working_dir="/app"
        )
        dockerfile.add_run("apt-get update && apt-get install -y curl")
        dockerfile.add_copy("requirements.txt", ".")
        dockerfile.add_run("pip install -r requirements.txt")
        
        print(dockerfile.build())
    """
    
    base_image: str
    working_dir: str = "/app"
    user: Optional[str] = None
    expose_ports: List[int] = field(default_factory=list)
    env_vars: Dict[str, str] = field(default_factory=dict)
    volumes: List[str] = field(default_factory=list)
    
    def __post_init__(self):
        self.instructions = []
    
    def add_run(self, command: str) -> 'DockerfileBuilder':
        """Add RUN instruction"""
        self.instructions.append(f"RUN {command}")
        return self
    
    def add_copy(self, src: str, dest: str) -> 'DockerfileBuilder':
        """Add COPY instruction"""
        self.instructions.append(f"COPY {src} {dest}")
        return self
    
    def add_add(self, src: str, dest: str) -> 'DockerfileBuilder':
        """Add ADD instruction"""
        self.instructions.append(f"ADD {src} {dest}")
        return self
    
    def add_env(self, key: str, value: str) -> 'DockerfileBuilder':
        """Add environment variable"""
        self.env_vars[key] = value
        return self
    
    def add_expose(self, port: int) -> 'DockerfileBuilder':
        """Expose a port"""
        self.expose_ports.append(port)
        return self
    
    def add_volume(self, path: str) -> 'DockerfileBuilder':
        """Add VOLUME"""
        self.volumes.append(path)
        return self
    
    def build(self) -> str:
        """Generate Dockerfile content"""
        lines = [f"FROM {self.base_image}", ""]
        
        # Environment variables
        if self.env_vars:
            for key, value in self.env_vars.items():
                lines.append(f"ENV {key}={value}")
            lines.append("")
        
        # Working directory
        lines.append(f"WORKDIR {self.working_dir}")
        lines.append("")
        
        # Custom instructions
        lines.extend(self.instructions)
        
        if self.instructions:
            lines.append("")
        
        # Expose ports
        for port in self.expose_ports:
            lines.append(f"EXPOSE {port}")
        
        if self.expose_ports:
            lines.append("")
        
        # Volumes
        for volume in self.volumes:
            lines.append(f"VOLUME {volume}")
        
        if self.volumes:
            lines.append("")
        
        # User
        if self.user:
            lines.append(f"USER {self.user}")
            lines.append("")
        
        return "\n".join(lines)

@dataclass
class ComposeService:
    """Docker Compose service configuration"""
    
    image: Optional[str] = None
    build: Optional[str] = None
    ports: List[str] = field(default_factory=list)
    environment: Dict[str, str] = field(default_factory=dict)
    volumes: List[str] = field(default_factory=list)
    depends_on: List[str] = field(default_factory=list)
    restart: str = "unless-stopped"
    command: Optional[str] = None
    healthcheck: Optional[Dict] = None
    networks: List[str] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """Convert to dictionary for YAML"""
        config = {}
        
        if self.image:
            config['image'] = self.image
        
        if self.build:
            config['build'] = self.build
        
        if self.ports:
            config['ports'] = self.ports
        
        if self.environment:
            config['environment'] = self.environment
        
        if self.volumes:
            config['volumes'] = self.volumes
        
        if self.depends_on:
            config['depends_on'] = self.depends_on
        
        if self.restart:
            config['restart'] = self.restart
        
        if self.command:
            config['command'] = self.command
        
        if self.healthcheck:
            config['healthcheck'] = self.healthcheck
        
        if self.networks:
            config['networks'] = self.networks
        
        return config

class ComposeConfig:
    """
    Docker Compose configuration builder
    
    Example:
        compose = ComposeConfig(version="3.8")
        
        compose.add_service("app", ComposeService(
            build=".",
            ports=["8080:8080"],
            environment={"ENV": "production"}
        ))
        
        compose.add_service("db", ComposeService(
            image="postgres:16",
            environment={"POSTGRES_PASSWORD": "secret"}
        ))
        
        yaml_content = compose.to_yaml()
    """
    
    def __init__(self, version: str = "3.8"):
        self.version = version
        self.services: Dict[str, ComposeService] = {}
        self.networks: Dict[str, Dict] = {}
        self.volumes: Dict[str, Dict] = {}
    
    def add_service(self, name: str, service: ComposeService) -> 'ComposeConfig':
        """Add a service"""
        self.services[name] = service
        return self
    
    def add_network(self, name: str, driver: str = "bridge") -> 'ComposeConfig':
        """Add a network"""
        self.networks[name] = {"driver": driver}
        return self
    
    def add_volume(self, name: str) -> 'ComposeConfig':
        """Add a named volume"""
        self.volumes[name] = {}
        return self
    
    def to_dict(self) -> Dict:
        """Convert to dictionary"""
        config = {"version": self.version}
        
        if self.services:
            config['services'] = {
                name: service.to_dict()
                for name, service in self.services.items()
            }
        
        if self.networks:
            config['networks'] = self.networks
        
        if self.volumes:
            config['volumes'] = self.volumes
        
        return config
    
    def to_yaml(self) -> str:
        """Generate YAML content"""
        import yaml
        return yaml.dump(self.to_dict(), default_flow_style=False, sort_keys=False)

# Template generators
class Templates:
    """Pre-built Docker templates"""
    
    @staticmethod
    def python_app(port: int = 8080) -> DockerfileBuilder:
        """Python application Dockerfile"""
        return (DockerfileBuilder(base_image="python:3.12-slim")
            .add_run("apt-get update && apt-get install -y --no-install-recommends curl && rm -rf /var/lib/apt/lists/*")
            .add_copy("requirements.txt", ".")
            .add_run("pip install --no-cache-dir -r requirements.txt")
            .add_copy(".", ".")
            .add_env("PYTHONUNBUFFERED", "1")
            .add_expose(port)
            .add_run("useradd -m -u 1000 app")
            .user = "app")
    
    @staticmethod
    def rust_app(port: int = 8080) -> str:
        """Rust multi-stage build Dockerfile"""
        return """FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \\
    ca-certificates \\
    libssl3 \\
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/app /usr/local/bin/

RUN useradd -m -u 1000 app
USER app

EXPOSE """ + str(port) + """

CMD ["app"]
"""
    
    @staticmethod
    def full_stack() -> ComposeConfig:
        """Full stack application with database and cache"""
        compose = ComposeConfig()
        
        compose.add_service("app", ComposeService(
            build=".",
            ports=["8080:8080"],
            environment={
                "DATABASE_URL": "postgresql://user:pass@db:5432/appdb",
                "REDIS_URL": "redis://redis:6379"
            },
            depends_on=["db", "redis"]
        ))
        
        compose.add_service("db", ComposeService(
            image="postgres:16-alpine",
            environment={
                "POSTGRES_USER": "user",
                "POSTGRES_PASSWORD": "pass",
                "POSTGRES_DB": "appdb"
            },
            volumes=["postgres-data:/var/lib/postgresql/data"]
        ))
        
        compose.add_service("redis", ComposeService(
            image="redis:7-alpine",
            volumes=["redis-data:/data"]
        ))
        
        compose.add_volume("postgres-data")
        compose.add_volume("redis-data")
        
        return compose

# Helper functions
def create_multistage_dockerfile(
    builder_image: str,
    runtime_image: str,
    build_commands: List[str],
    binary_path: str,
    target_path: str = "/usr/local/bin/app"
) -> str:
    """
    Create a multi-stage Dockerfile
    
    Args:
        builder_image: Image for build stage
        runtime_image: Image for runtime stage
        build_commands: Commands to run in build stage
        binary_path: Path to built binary
        target_path: Where to copy binary in runtime
    
    Returns:
        Dockerfile content
    """
    lines = [
        f"FROM {builder_image} as builder",
        "",
        "WORKDIR /build",
        "COPY . .",
        ""
    ]
    
    lines.extend(build_commands)
    
    lines.extend([
        "",
        f"FROM {runtime_image}",
        "",
        f"COPY --from=builder {binary_path} {target_path}",
        "",
        "RUN useradd -m -u 1000 app",
        "USER app",
        "",
        f"CMD [\"{target_path}\"]"
    ])
    
    return "\n".join(lines)

def create_healthcheck(
    test: str,
    interval: str = "30s",
    timeout: str = "3s",
    retries: int = 3
) -> Dict:
    """
    Create healthcheck configuration
    
    Args:
        test: Health check command
        interval: Check interval
        timeout: Check timeout
        retries: Number of retries
    
    Returns:
        Healthcheck dictionary
    """
    return {
        "test": test if isinstance(test, list) else ["CMD-SHELL", test],
        "interval": interval,
        "timeout": timeout,
        "retries": retries
    }
