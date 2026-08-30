# Corporate Rust Micro-Package Workspace

An enterprise-grade, monorepo-structured Rust workspace orchestrating highly decoupled modules, domain libraries, and
microservices under unified dependency tracking.

---

## 🏗️ Architecture & Topology

This workspace operates under a **shared-nothing runtime architecture**. Internal libraries (`Domain Core`) are strictly
decoupled from operational targets (`Consumers`), ensuring isolated compile units and independent scalability.

---

## 🛡️ Workspace Governance

### 1. Dependency Isolation

All third-party crates must be declared inside the root `[workspace.dependencies]` to prevent version drift across
runtime boundaries. Sub-packages must consume them strictly via `{ workspace = true }`.

### 2. Toolchain Enforcement

New components added to the `Application Layer` should default to the `2024` compiler edition to leverage optimized
coroutines. Domain libraries maintaining backward compatibility may target `2021`.