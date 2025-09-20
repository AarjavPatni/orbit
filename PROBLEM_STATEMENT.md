### **Problem Statement: Resilient, Encrypted, Distributed File Storage**

Organizations and individuals face growing risks to their data from:

1. **Hardware failure** (disk crashes, device loss, outages).
2. **Malicious attacks** (ransomware, insider threats, unauthorized access).
3. **Centralized points of failure** (single servers or cloud providers being compromised).

Traditional storage systems often rely on centralized infrastructure, which creates bottlenecks and vulnerabilities. Even cloud storage, while resilient, demands trust in a third party to safeguard sensitive information.

We need a **distributed file storage system** that:

* **Shards** files into smaller pieces so no single node holds a full file.
* **Encrypts** each shard so storage providers (or attackers) can’t read user data.
* **Replicates** shards across multiple nodes to tolerate failures and ensure durability.
* **Reconstructs** files on demand, given the correct key, even if some nodes are offline.
* **Scales** across devices or servers without relying on a single central authority.

This system should:

* Work initially across a small number of machines (e.g., your own laptop, home server, cloud VM).
* Be usable for personal backups, small-team collaboration, or sensitive corporate data storage.
* Provide a foundation to extend into more advanced features, such as consensus-driven metadata, erasure coding for efficiency, and decentralized trust models.

**Goal:** Design and prototype a system that makes data storage **fault-tolerant, private, and resilient to compromise** by combining distributed systems techniques (replication, sharding, failure recovery) with cryptography.

