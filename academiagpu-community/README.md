# AcademiaGPU (Community Edition)

AcademiaGPU is a low-overhead GPU VRAM quota enforcer and hardware telemetry monitor for academic and university AI labs. 

Built in Rust, it leverages direct NVML C-bindings to poll per-PID GPU memory usage, Tensor Core utilization, and thermal throttling in sub-millisecond intervals.

## Features
- **Low-Overhead Telemetry**: Written in Rust, taking <0.1% CPU overhead.
- **VRAM Monitor**: Proactively detects Out-Of-Memory (OOM) thrashing and logs VRAM usage per job.
- **Open Source**: Licensed under AGPLv3 for community use (up to 8 GPUs).

## Quickstart (3-Line Setup)

```bash
# 1. Download & Install the binary
curl -sSL https://academiagpu.io/install.sh | bash

# 2. Start the daemon with an 8GB VRAM limit per process
academiagpu --vram-limit-mb 8192

# 3. View real-time usage (or integrate logs into your monitoring stack)
journalctl -u academiagpu -f
```

## Community vs. Enterprise Edition

| Feature | Community (AGPLv3) | Enterprise (Proprietary) |
| --- | --- | --- |
| GPU Limit | Up to 8 GPUs | Unlimited (Cluster-scale) |
| VRAM Monitoring | Logging & Warnings | Dynamic Throttling & Job Preemption |
| Orchestration | Manual | Slurm SPANK & K8s Device Plugin |
| Licensing | Free (AGPLv3) | Offline Ed25519 Cryptographic License |
| Billing & Reports | Basic Stats | Auto Grant/Department Billing (PDF/CSV) |

For University HPC Centers needing strict VRAM isolation, LDAP integration, and billing, consider the **Enterprise Edition**. Purchase a cluster license via [Polar.sh](https://polar.sh/academiagpu).

## Author & Copyright
- **Author**: Emirhan CAMCI (<byemir@live.com>)
- **Year**: 2026
- **License**: AGPL-3.0
