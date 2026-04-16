
<p align="center">
  <img src="./repo/file.svg" alt="Daisy Logo" width="400"/>
</p>


# Daisy

**Daisy** is a lightweight, high-performance domain routing and application serving engine built in Rust. It is designed to act as a flexible layer between incoming HTTP requests and multiple hosted applications, enabling efficient domain-based routing with minimal overhead.

Daisy focuses on simplicity, performance, and control — giving developers the ability to define how domains map to applications without relying on heavy, monolithic web servers or opaque infrastructure.

---

## Overview

Modern applications often require serving multiple domains, subdomains, or projects from a single machine or deployment unit. Daisy provides a structured and efficient way to handle this by routing requests based on domain context and directing them to the appropriate application or resource.

Instead of tightly coupling infrastructure with application logic, Daisy introduces a clean separation layer that handles:

- Domain-based request routing  
- Application mapping and dispatching  
- Lightweight request handling with minimal latency  

This makes it particularly useful for developers building custom hosting environments, multi-tenant platforms, or experimental infrastructure systems.

---

## What Daisy Is

Daisy is:

- A **domain routing engine** that maps incoming requests to specific applications or services  
- A **lightweight HTTP service layer** designed for performance and clarity  
- A **developer-controlled infrastructure component**, not a black-box platform  
- A **Rust-native system**, leveraging safety, concurrency, and speed  

---

## What Daisy Is Not

Daisy is not:

- A full-featured web framework  
- A traditional reverse proxy with extensive configuration complexity  
- A managed hosting platform  

Instead, it sits in a focused niche: **precise, programmable domain routing with minimal abstraction overhead**.

---

## Core Philosophy

Daisy is built around a few key principles:

### 1. Simplicity Over Abstraction
Clear, predictable behavior is prioritized over hidden magic or excessive configuration layers.

### 2. Performance by Default
Written in Rust, Daisy is designed to handle concurrent requests efficiently while maintaining low latency.

### 3. Explicit Control
Developers define exactly how domains are resolved and routed, making behavior transparent and debuggable.

### 4. Modularity
Daisy is intended to integrate into larger systems, acting as a composable building block rather than a complete solution.

---

## Use Cases

Daisy is well-suited for:

- **Multi-domain hosting** on a single server  
- **Developer tooling** for local or experimental infrastructure  
- **Custom platform backends** requiring domain-based routing logic  
- **Lightweight alternatives** to traditional reverse proxies  
- **Educational projects** exploring how web routing and hosting layers work  

---

## Design Perspective

At its core, Daisy operates by inspecting incoming HTTP requests, extracting the host context, and resolving it against a predefined configuration. Based on this resolution, requests are routed to the appropriate application layer or resource handler.

This approach allows developers to:

- Decouple domain management from application logic  
- Maintain a clean mapping between domains and services  
- Extend or customize routing behavior as needed  

---

## Vision

Daisy aims to evolve into a foundational tool for developers who want deeper control over how applications are served across domains — without inheriting the complexity of traditional infrastructure stacks.

It is built for those who prefer **understanding and shaping their systems**, rather than configuring around them.
