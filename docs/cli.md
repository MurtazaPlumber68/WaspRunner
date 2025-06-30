
# WasmRunner CLI Reference

## Overview

The `wasmrunner` CLI provides a Docker-like interface for managing WASM containers and applications.

## Global Options

```bash
wasmrunner [GLOBAL OPTIONS] COMMAND [COMMAND OPTIONS]
```

### Global Options
- `--config PATH`: Specify configuration file path
- `--verbose, -v`: Enable verbose logging
- `--help, -h`: Show help information
- `--version, -V`: Show version information

## Commands

### `run` - Run a WASM Application

Execute a WASM container with specified options.

```bash
wasmrunner run [OPTIONS] IMAGE[:TAG] [-- ARGS...]
```

#### Options
- `--memory LIMIT`: Memory limit in MB (default: 128)
- `--cpu PERCENT`: CPU limit as percentage (default: 100) 
- `--env, -e KEY=VALUE`: Set environment variables
- `--network MODE`: Network mode: none, host (default: none)
- `--detach, -d`: Run in detached mode
- `--name NAME`: Assign container name
- `--timeout SECONDS`: Execution timeout (default: 300)

#### Examples
```bash
# Run hello-world app
wasmrunner run hello-world

# Run with custom memory limit
wasmrunner run --memory 256 my-app:latest

# Run with environment variables
wasmrunner run -e ENV=production -e DEBUG=false web-service

# Run detached with custom name
wasmrunner run -d --name my-service web-app:v1.0

# Run with arguments
wasmrunner run calculator -- add 5 10
```

### `build` - Build a WASM Application

Build a WASM container from source code.

```bash
wasmrunner build [OPTIONS] PATH
```

#### Options
- `--tag, -t NAME:TAG`: Tag for the built image
- `--file, -f DOCKERFILE`: Dockerfile path (default: Dockerfile.wasm)
- `--target TARGET`: Build target architecture
- `--no-cache`: Disable build cache

#### Examples
```bash
# Build from current directory
wasmrunner build -t my-app:latest .

# Build with custom Dockerfile
wasmrunner build -t web-service:v1.0 -f Dockerfile.web .

# Build without cache
wasmrunner build --no-cache -t my-app:dev .
```

### `push` - Push Image to Registry

Upload an image to a remote registry.

```bash
wasmrunner push IMAGE[:TAG]
```

#### Examples
```bash
# Push to default registry
wasmrunner push my-app:latest

# Push with full registry URL
wasmrunner push registry.example.com/my-app:v1.0
```

### `pull` - Pull Image from Registry

Download an image from a remote registry.

```bash
wasmrunner pull IMAGE[:TAG]
```

#### Examples
```bash
# Pull latest version
wasmrunner pull hello-world

# Pull specific version
wasmrunner pull web-service:v2.1
```

### `list` - List Containers

Show running and stopped containers.

```bash
wasmrunner list [OPTIONS]
```

#### Options
- `--all, -a`: Show all containers (including stopped)
- `--quiet, -q`: Only show container IDs
- `--format FORMAT`: Custom output format

#### Examples
```bash
# List running containers
wasmrunner list

# List all containers
wasmrunner list --all

# Show only IDs
wasmrunner list -q
```

### `logs` - Show Container Logs

Display logs from a container.

```bash
wasmrunner logs [OPTIONS] CONTAINER
```

#### Options
- `--follow, -f`: Follow log output
- `--tail LINES`: Number of lines to show (default: 100)
- `--since TIME`: Show logs since timestamp
- `--timestamps`: Show timestamps

#### Examples
```bash
# Show recent logs
wasmrunner logs my-container

# Follow logs in real-time
wasmrunner logs -f web-service

# Show last 50 lines
wasmrunner logs --tail 50 my-app
```

### `stop` - Stop Container

Stop a running container.

```bash
wasmrunner stop CONTAINER [CONTAINER...]
```

#### Examples
```bash
# Stop single container
wasmrunner stop my-container

# Stop multiple containers
wasmrunner stop web-app db-service
```

### `remove` - Remove Container

Remove one or more containers.

```bash
wasmrunner remove [OPTIONS] CONTAINER [CONTAINER...]
```

#### Options
- `--force, -f`: Force removal of running containers
- `--volumes, -v`: Remove associated volumes

#### Examples
```bash
# Remove stopped container
wasmrunner remove my-container

# Force remove running container
wasmrunner remove -f web-service
```

### `search` - Search Registry

Search for images in the registry.

```bash
wasmrunner search TERM
```

#### Examples
```bash
# Search for web applications
wasmrunner search web

# Search for specific service
wasmrunner search database
```

### `install` - Install Plugin

Install a WasmRunner plugin.

```bash
wasmrunner install PLUGIN_NAME
```

#### Examples
```bash
# Install monitoring plugin
wasmrunner install monitor

# Install from specific source
wasmrunner install github.com/user/wasmrunner-plugin
```

## Configuration

### Configuration File

WasmRunner looks for configuration in these locations:
1. Path specified by `--config` flag
2. `~/.wasmrunner/config.toml`
3. Environment variables with `WASMRUNNER_` prefix

### Example Configuration

```toml
[runtime]
default_runtime = "wasmtime"
memory_limit_mb = 128
cpu_limit_percent = 100
timeout_seconds = 300

[security]
enable_seccomp = true
enable_memory_guard = true
allow_network = false
allow_filesystem = true

[registry]
default_registry = "registry.wasmrunner.dev"
cache_dir = "~/.wasmrunner/cache"

[plugins]
plugin_dir = "~/.wasmrunner/plugins"
auto_discovery = true
```

### Environment Variables

- `WASMRUNNER_RUNTIME_DEFAULT_RUNTIME`: Default WASM runtime
- `WASMRUNNER_SECURITY_ENABLE_SECCOMP`: Enable seccomp filtering
- `WASMRUNNER_REGISTRY_DEFAULT_REGISTRY`: Default registry URL

## Exit Codes

- `0`: Success
- `1`: General error
- `2`: Invalid arguments
- `125`: Container failed to run
- `126`: Container command not executable
- `127`: Container command not found

## Tips and Best Practices

### Performance
- Use `--memory` to set appropriate memory limits
- Consider `--cpu` limits for shared environments
- Use `--detach` for long-running services

### Security
- Always review images before running
- Use minimal network permissions
- Set appropriate resource limits
- Monitor container logs regularly

### Development
- Use consistent tagging for versions
- Build with `--no-cache` when dependencies change
- Test with different resource limits
- Use descriptive container names
