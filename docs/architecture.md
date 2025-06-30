
# WasmRunner Architecture

## Overview

WasmRunner is designed as a modular, secure, and performant containerless application runner for WebAssembly. The architecture follows a layered approach with clear separation of concerns.

## High-Level Components

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI Interface                           │
│  Commands: run, build, push, pull, list, logs, etc.       │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                  Core Engine                               │
│  • Container Management    • Image Registry                │
│  • Configuration           • Plugin System                 │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                Runtime Layer                               │
│  • WASM Runtime Abstraction (Wasmtime, Wasmer)           │
│  • Module Loading & Validation                            │
│  • Execution Management                                   │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                Sandbox Layer                               │
│  • Memory Isolation     • Seccomp Filtering              │
│  • Process Isolation    • Resource Limits                │
│  • Network Controls     • Filesystem Controls            │
└─────────────────────────────────────────────────────────────┘
```

## Core Modules

### 1. CLI Interface (`wasmrunner-cli`)
- **Purpose**: User-facing command-line interface
- **Key Features**:
  - Docker-like command syntax
  - Rich argument parsing with clap
  - Progress indicators and user feedback
  - Configuration management

### 2. Core Engine (`wasmrunner-core`)
- **Purpose**: Central types and business logic
- **Components**:
  - Container lifecycle management
  - Image and manifest handling
  - Registry operations
  - Configuration management

### 3. Runtime Layer (`wasmrunner-runtime`)
- **Purpose**: WASM execution abstraction
- **Features**:
  - Multiple runtime support (Wasmtime, Wasmer)
  - Module loading and validation
  - WASI integration
  - Host function binding

### 4. Sandbox Layer (`wasmrunner-sandbox`)
- **Purpose**: Security and isolation
- **Capabilities**:
  - Memory limits and guards
  - CPU quotas
  - Seccomp-BPF filtering
  - Network isolation
  - Filesystem restrictions

### 5. Plugin System (`wasmrunner-plugins`)
- **Purpose**: Extensibility and customization
- **Features**:
  - Dynamic plugin loading
  - Plugin discovery and management
  - Host-plugin communication
  - Plugin metadata and versioning

## Security Model

### Threat Model
- **Untrusted Code**: WASM modules may contain malicious code
- **Resource Exhaustion**: Prevent DoS through resource limits
- **Privilege Escalation**: Sandbox prevents system compromise
- **Data Exfiltration**: Network and filesystem controls

### Security Layers
1. **WASM Sandboxing**: Memory-safe execution environment
2. **OS-Level Isolation**: Seccomp, namespaces, cgroups
3. **Resource Limits**: Memory, CPU, and I/O quotas
4. **Capability System**: Minimal required permissions

## Plugin Architecture

### Plugin Discovery
- Scan `~/.wasmrunner/plugins/` directory
- Load plugin manifests (`plugin.toml`)
- Validate signatures and metadata
- Register plugin capabilities

### Plugin Communication
- Host functions exposed to plugins
- Event-driven plugin lifecycle
- Structured message passing
- Capability-based security

## Registry and Storage

### Local Registry
- SQLite database for metadata
- Filesystem blob storage
- Content-addressable storage
- Image layering and deduplication

### Remote Registry
- REST API for push/pull operations
- Authentication and authorization
- Image signing and verification
- Distributed caching support

## Performance Considerations

### Startup Time
- Pre-compiled WASM modules
- Runtime optimization flags
- Lazy loading of plugins
- Connection pooling

### Memory Usage
- Shared runtime instances
- Memory-mapped module storage
- Garbage collection tuning
- Resource pool management

### I/O Performance
- Async I/O throughout
- Batch operations
- Compression for network transfers
- Local caching strategies
