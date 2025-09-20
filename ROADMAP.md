# Orbit Development Roadmap

Detailed milestone breakdown for building a distributed, encrypted file storage system with P2P networking and rich TUI interface.

## Milestone 1: Basic File Operations
**Demo Goal**: Upload, chunk, encrypt, and reconstruct files locally

- [ ] **File Chunking**
  - [ ] Implement adaptive chunk sizing (64KB/1MB/4MB based on file size)
  - [ ] Handle edge cases (empty files, files smaller than chunk size)
  - [ ] Efficient memory usage during chunking process
  - **Acceptance**: `orbit upload test.txt` creates appropriately-sized chunks

- [ ] **Per-Shard Encryption**
  - [ ] Generate unique encryption keys for each shard
  - [ ] Derive shard keys from master passphrase
  - [ ] Secure key handling and storage
  - **Acceptance**: Encrypted shards cannot be read without proper keys

- [ ] **Local File Reconstruction**
  - [ ] Combine chunks back into original file
  - [ ] Verify file integrity after reconstruction
  - [ ] Handle missing or corrupted shards gracefully
  - **Acceptance**: `orbit download test.txt` recreates original file perfectly

## Milestone 2: Multi-Node Storage
**Demo Goal**: Files stored and retrieved across multiple processes

- [ ] **Basic Network Communication**
  - [ ] TCP server for node-to-node communication
  - [ ] Simple protocol for shard transfer
  - [ ] Connection management and error handling
  - **Acceptance**: Nodes can send shards to each other via TCP

- [ ] **Manual Node Configuration**
  - [ ] Static configuration file with node addresses
  - [ ] Load balancing across available nodes
  - [ ] Handle node unavailability gracefully
  - **Acceptance**: `orbit upload --nodes node1,node2,node3 file.txt` distributes shards

- [ ] **Multi-Node Retrieval**
  - [ ] Query multiple nodes for file shards
  - [ ] Parallel shard downloading
  - [ ] Reconstruct files from distributed shards
  - **Acceptance**: Download file successfully from any participating node

- [ ] **Basic Replication**
  - [ ] Store each shard on multiple nodes (2x replication)
  - [ ] Handle node failures during upload
  - [ ] Redundancy verification and reporting
  - **Acceptance**: File remains accessible when one storage node goes offline

## Milestone 3: P2P Discovery
**Demo Goal**: Nodes automatically find each other on local network

- [ ] **mDNS Service Discovery**
  - [ ] Advertise storage service on local network
  - [ ] Discover other nodes automatically
  - [ ] Handle service registration and deregistration
  - **Acceptance**: Nodes discover each other without manual configuration

- [ ] **Peer Management**
  - [ ] Maintain list of active peers
  - [ ] Handle peer connection and disconnection
  - [ ] Periodic health checking of discovered peers
  - **Acceptance**: `orbit status` shows all discovered nodes on network

- [ ] **Automatic Distribution**
  - [ ] Upload files to discovered nodes without manual specification
  - [ ] Intelligent peer selection for shard placement
  - [ ] Fallback handling when preferred nodes unavailable
  - **Acceptance**: `orbit upload file.txt` works without specifying target nodes

- [ ] **Local Network Testing**
  - [ ] Multiple nodes running on same machine with different ports
  - [ ] Verify discovery works across different terminals
  - [ ] Test network partition and recovery scenarios
  - **Acceptance**: Start 3 nodes in different terminals, they all discover each other

## Milestone 4: Basic TUI
**Demo Goal**: Beautiful progress bars and real-time status

- [ ] **Upload Progress Visualization**
  - [ ] Show file chunking progress with progress bars
  - [ ] Display encryption status for each shard
  - [ ] Real-time upload progress to different nodes
  - **Acceptance**: Upload shows beautiful step-by-step progress

- [ ] **Download Progress Visualization**
  - [ ] Display peer discovery and shard location
  - [ ] Show parallel download progress from multiple nodes
  - [ ] Real-time decryption and reconstruction status
  - **Acceptance**: Download shows detailed progress and node sources

- [ ] **Basic Status Dashboard**
  - [ ] List discovered nodes with connection status
  - [ ] Show stored files and their replication status
  - [ ] Display basic network health information
  - **Acceptance**: `orbit status` shows clean, informative dashboard

- [ ] **Async UI Updates**
  - [ ] Non-blocking progress updates during operations
  - [ ] Real-time status refreshing
  - [ ] Responsive interface during long-running operations
  - **Acceptance**: UI remains responsive during file operations

## Milestone 5: Gossip Protocol
**Demo Goal**: Nodes share information about stored files and network state

- [ ] **Gossip Message Types**
  - [ ] Node health and status announcements
  - [ ] File shard location advertisements
  - [ ] Network topology updates
  - **Acceptance**: Nodes exchange metadata about stored files

- [ ] **Gossip Propagation**
  - [ ] Periodic gossip rounds with random peers
  - [ ] Efficient message spreading across network
  - [ ] Handle duplicate and stale information
  - **Acceptance**: Information spreads to all nodes within reasonable time

- [ ] **Distributed File Metadata**
  - [ ] Nodes know which files exist in the network
  - [ ] Track shard locations across multiple nodes
  - [ ] Handle metadata inconsistencies gracefully
  - **Acceptance**: Any node can locate and retrieve any stored file

- [ ] **Network State Convergence**
  - [ ] Eventual consistency of network view
  - [ ] Handle node joins and departures
  - [ ] Detect and resolve conflicting information
  - **Acceptance**: Network reaches consistent state after changes

## Milestone 6: Fault Tolerance
**Demo Goal**: Manual node recovery and file resilience

- [ ] **Node Failure Detection**
  - [ ] Detect when nodes become unresponsive
  - [ ] Update network topology when nodes disappear
  - [ ] Handle graceful and ungraceful shutdowns
  - **Acceptance**: Network adapts when nodes are killed or disconnected

- [ ] **File Availability During Failures**
  - [ ] Retrieve files from remaining replicas when nodes fail
  - [ ] Reroute requests to available nodes automatically
  - [ ] Report degraded file status when replication reduced
  - **Acceptance**: Files remain downloadable even after killing storage nodes

- [ ] **Manual Node Recovery**
  - [ ] Nodes rejoin network after restart
  - [ ] Synchronize with current network state via gossip
  - [ ] Restore replication levels after recovery
  - **Acceptance**: Restarted nodes seamlessly rejoin and restore full replication

- [ ] **Fault Tolerance Demonstration**
  - [ ] Kill nodes during file operations and show continued functionality
  - [ ] Demonstrate file retrieval with partial node availability
  - [ ] Show network healing and replication restoration
  - **Acceptance**: Compelling demo of system resilience to node failures

## Milestone 7: Rich TUI Dashboard
**Demo Goal**: Comprehensive real-time network and file visualization

- [ ] **Advanced Node Status Display**
  - [ ] Live node health monitoring with ping times
  - [ ] Storage capacity and usage visualization
  - [ ] Network topology graph or tree view
  - **Acceptance**: Detailed node status with real-time updates

- [ ] **File Health Monitoring**
  - [ ] Replication status for each stored file
  - [ ] Shard distribution across nodes
  - [ ] File integrity and availability indicators
  - **Acceptance**: Clear visualization of file health and distribution

- [ ] **Real-time Network Activity**
  - [ ] Live gossip message flow visualization
  - [ ] Upload/download activity tracking
  - [ ] Network bandwidth and performance metrics
  - **Acceptance**: Watch network activity in real-time during operations

- [ ] **Interactive Dashboard Features**
  - [ ] Click/select nodes for detailed information
  - [ ] Filter and search stored files
  - [ ] Export network status and logs
  - **Acceptance**: Rich, interactive interface for network management

## Milestone 8: Polish & Performance
**Demo Goal**: Production readiness and optimization

- [ ] **Error Handling & Recovery**
  - [ ] Comprehensive error handling for all failure modes
  - [ ] Graceful degradation when nodes are unreachable
  - [ ] User-friendly error messages and recovery suggestions
  - **Acceptance**: System handles edge cases gracefully with clear feedback

- [ ] **Performance Optimization**
  - [ ] Profile and optimize file transfer speeds
  - [ ] Efficient memory usage during large file operations
  - [ ] Minimize network overhead and latency
  - **Acceptance**: Measurable performance improvements and efficient resource usage

- [ ] **Configuration & Usability**
  - [ ] Configuration files for node settings and network parameters
  - [ ] Command-line help and usage documentation
  - [ ] Logging and debugging capabilities
  - **Acceptance**: Easy to configure and troubleshoot

- [ ] **Final Integration Testing**
  - [ ] End-to-end testing with multiple files and nodes
  - [ ] Stress testing with large files and many operations
  - [ ] Long-running stability testing
  - **Acceptance**: Robust system ready for real-world usage

## Progress Tracking

- **Current Milestone**: 1 (Basic File Operations)
- **Next Demo**: Upload, chunk, encrypt, and reconstruct files locally
- **Completed Milestones**: None (starting fresh with file storage focus)

## Demo Schedule

Each milestone should conclude with a working demonstration:
- **Milestone 1**: Upload file, show chunking/encryption, download and verify
- **Milestone 2**: Store file across multiple processes, retrieve from any node
- **Milestone 3**: Start nodes, watch them discover each other automatically
- **Milestone 4**: Beautiful TUI showing upload/download progress in real-time
- **Milestone 5**: Nodes gossiping metadata, files discoverable network-wide
- **Milestone 6**: Kill storage nodes, files still accessible via remaining nodes
- **Milestone 7**: Rich dashboard showing live network activity and file status
- **Milestone 8**: Comprehensive system stress test and performance showcase