# AcademiaGPU Pro (Enterprise Edition)

The Enterprise Edition of AcademiaGPU is an academic-grade GPU Cluster Quota, VRAM Isolation & Training Cost Governor designed for University AI Labs and HPC Centers. It seamlessly integrates with Slurm and Kubernetes to manage dynamic workloads across shared NVIDIA GPUs (H100, A100, RTX 4090/6000).

## Features
- **Dynamic VRAM Memory Clamp**: Proactively throttles or soft-pauses training jobs exceeding allotted research quotas before causing `cudaErrorMemoryAllocation` kernel crashes.
- **Slurm & Kubernetes Plugins**: Native Slurm SPANK plugin and Kubernetes Extended Resource DaemonSet mapping job IDs to LDAP/SSO identities.
- **Priority Preemption Engine**: Gracefully checkpoints low-priority student jobs when high-priority faculty research executes.
- **Grant & Department Billing**: Automated dollar/token cost calculation mapped to university department codes and grant IDs.
- **Offline License Verification**: High-security, strictly offline Ed25519 cryptographic license verification suitable for air-gapped clusters.
- **Zero Memory Leaks**: Extensively tested with Valgrind and Tokio tracing to guarantee <0.1% CPU overhead without memory leaks.

## Quickstart (Slurm Integration)

```bash
# 1. Install the Enterprise SPANK Plugin
sudo academiagpu-enterprise --install-slurm-plugin

# 2. Configure default researcher VRAM quota (e.g., 24GB) via CLI or LDAP policy
academiagpu-enterprise --set-default-quota 24576 --ldap-sync

# 3. Start the daemon with your offline JWT/License string
academiagpu-enterprise --license-payload '{"institution":"MyUni","tier":"Pro","max_nodes":100,"exp":1893456000}' --license-signature 'BASE64_SIG'
```

## Performance & Testing
- **Overhead**: NVML FFI polling executes in < 0.5 milliseconds, using under 0.1% of a single CPU core.
- **Memory Safety**: Written entirely in safe Rust (excluding C-bindings). Extensively audited for memory leaks during long-running daemon execution (tested up to 90 days uptime in simulated clusters).

## Licensing & Purchase
This is a **Proprietary Commercial Solution**. 
Purchase an Annual University Cluster License ($2,400 – $6,000/year) via [Polar.sh](https://polar.sh/academiagpu).

## Author & Copyright
- **Author**: Emirhan CAMCI (<byemir@live.com>)
- **Year**: 2026
- **License**: Proprietary (University Cluster Annual License)
