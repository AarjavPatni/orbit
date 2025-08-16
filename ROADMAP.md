# Orbit Development Roadmap

Detailed task breakdown for building a distributed, fault-tolerant key-value store with CLI-demonstrable fault tolerance.

## Milestone 1: Basic CLI Store (1-2 weeks)
**Demo Goal**: Redis-like CLI commands work locally

- [ ] **CLI Parser Implementation**
  - [ ] Support GET, SET, DEL, KEYS commands from command line
  - [ ] Handle command arguments and validation
  - [ ] Display helpful error messages for invalid commands
  - **Acceptance**: `./orbit set foo bar` and `./orbit get foo` work

- [ ] **In-Memory Store**
  - [ ] Store key-value pairs in memory during session
  - [ ] Handle basic string keys and values
  - [ ] Return appropriate responses for each operation
  - **Acceptance**: Data persists during single session, all operations work

- [ ] **Basic Error Handling**
  - [ ] Graceful handling of missing keys
  - [ ] Clear error messages for malformed commands
  - [ ] Proper exit codes for different scenarios
  - **Acceptance**: Program doesn't crash on invalid input

## Milestone 2: Persistent Single Node (1-2 weeks)
**Demo Goal**: Data survives restarts

- [ ] **Append-Only Log**
  - [ ] Write operations to a log file
  - [ ] Use human-readable format for debugging
  - [ ] Ensure atomic writes
  - **Acceptance**: Log file grows with each SET/DEL operation

- [ ] **Log Replay on Startup**
  - [ ] Read log file when program starts
  - [ ] Rebuild in-memory state from log
  - [ ] Handle corrupted entries gracefully
  - **Acceptance**: Restart preserves all data from previous session

- [ ] **Crash Recovery**
  - [ ] Verify data integrity after unexpected shutdown
  - [ ] Handle partial writes correctly
  - [ ] Test with kill -9 scenarios
  - **Acceptance**: No data loss even with kill -9

- [ ] **Log Compaction**
  - [ ] Prevent log file from growing indefinitely
  - [ ] Implement periodic cleanup
  - [ ] Maintain data consistency during compaction
  - **Acceptance**: Log size doesn't grow indefinitely with repeated operations

## Milestone 3: Network-Aware Single Node (1-2 weeks)
**Demo Goal**: Remote access to store

- [ ] **TCP Server**
  - [ ] Listen on configurable port
  - [ ] Accept incoming connections
  - [ ] Handle connection errors gracefully
  - **Acceptance**: `nc localhost 8080` connects successfully

- [ ] **Wire Protocol**
  - [ ] Define text-based communication protocol
  - [ ] Parse commands from network clients
  - [ ] Send responses back to clients
  - **Acceptance**: Can send "SET foo bar" via netcat and get response

- [ ] **Concurrent Connections**
  - [ ] Handle multiple clients simultaneously
  - [ ] Prevent blocking between connections
  - [ ] Clean up resources on disconnect
  - **Acceptance**: 10+ simultaneous connections work without blocking

- [ ] **Client Library**
  - [ ] Create simple client for easier testing
  - [ ] Handle connection and communication
  - [ ] Provide clean API for operations
  - **Acceptance**: Rust client can connect and perform all operations

## Milestone 4: Leader-Follower Replication (2-3 weeks)
**Demo Goal**: Basic fault tolerance visible

- [ ] **Node Discovery**
  - [ ] Nodes can find and connect to each other
  - [ ] Configure peer lists
  - [ ] Detect when peers are unavailable
  - **Acceptance**: Nodes successfully establish connections

- [ ] **Log Replication**
  - [ ] Leader forwards operations to followers
  - [ ] Followers apply operations in correct order
  - [ ] Handle network failures between nodes
  - **Acceptance**: Operations on leader appear on followers

- [ ] **Leader Election**
  - [ ] Ensure only one leader at a time
  - [ ] Automatic failover when leader fails
  - [ ] Clear distinction between leader and follower roles
  - **Acceptance**: Kill leader, follower becomes new leader

- [ ] **Write Safety**
  - [ ] Only leader accepts write operations
  - [ ] Followers reject writes with clear errors
  - [ ] Prevent conflicting writes during leadership changes
  - **Acceptance**: Only one node can accept writes at any time

## Milestone 5: Multi-Node Consensus (Raft) (2-3 weeks)
**Demo Goal**: Partition tolerance demonstration

- [ ] **Raft Elections**
  - [ ] Implement robust leader election
  - [ ] Use term numbers to prevent conflicts
  - [ ] Handle split votes correctly
  - **Acceptance**: Reliable leader election even with network delays

- [ ] **Raft Log Replication**
  - [ ] Ensure log consistency across all nodes
  - [ ] Handle log conflicts properly
  - [ ] Implement proper commitment rules
  - **Acceptance**: All nodes have identical committed logs

- [ ] **Cluster Membership**
  - [ ] Support 3+ node clusters
  - [ ] Handle node failures gracefully
  - [ ] Maintain consensus with majority online
  - **Acceptance**: 3+ node cluster maintains consistency

- [ ] **Partition Testing**
  - [ ] Simulate network partitions
  - [ ] Verify consistency is maintained
  - [ ] Test partition healing scenarios
  - **Acceptance**: Network partition demo shows proper behavior

## Milestone 6: Transaction-Level Consensus (2-3 weeks)
**Demo Goal**: Compare consensus approaches

- [ ] **Per-Transaction Consensus**
  - [ ] Implement consensus for individual operations
  - [ ] Maintain safety properties
  - [ ] Measure performance characteristics
  - **Acceptance**: Each operation gets individual consensus

- [ ] **Performance Comparison**
  - [ ] Compare both consensus approaches
  - [ ] Measure latency and throughput
  - [ ] Document trade-offs
  - **Acceptance**: Clear performance comparison data

- [ ] **Runtime Configuration**
  - [ ] Switch between consensus approaches
  - [ ] Maintain data consistency during switches
  - [ ] Provide clean configuration interface
  - **Acceptance**: Can toggle between approaches without data loss

## Milestone 7: Advanced Fault Scenarios (2-3 weeks)
**Demo Goal**: Comprehensive fault tolerance

- [ ] **Dynamic Membership**
  - [ ] Add nodes to running cluster
  - [ ] Remove nodes safely
  - [ ] Handle rapid membership changes
  - **Acceptance**: Add/remove nodes without service interruption

- [ ] **Complex Partition Recovery**
  - [ ] Handle minority partition scenarios
  - [ ] Resolve split-vote situations
  - [ ] Detect and respond to network healing
  - **Acceptance**: Recovery from complex partition scenarios

- [ ] **Automated Fault Testing**
  - [ ] Inject faults automatically
  - [ ] Verify consistency under faults
  - [ ] Measure performance degradation
  - **Acceptance**: Automated fault injection with consistency verification

## Milestone 8: Production Features (1-2 weeks)
**Demo Goal**: Ready for real use

- [ ] **Observability**
  - [ ] Add metrics endpoint
  - [ ] Implement structured logging
  - [ ] Provide key performance indicators
  - **Acceptance**: Monitoring dashboard shows system health

- [ ] **Configuration Management**
  - [ ] Support configuration files
  - [ ] Allow environment variable overrides
  - [ ] Validate configuration on startup
  - **Acceptance**: Flexible configuration without code changes

- [ ] **Performance Optimization**
  - [ ] Profile and optimize hot paths
  - [ ] Reduce memory usage
  - [ ] Minimize operation latency
  - **Acceptance**: Measurable performance improvements

## Progress Tracking

- **Current Milestone**: 1 (Basic CLI Store)
- **Next Demo**: CLI commands working locally
- **Completed Milestones**: 0 (Project Setup)

## Demo Schedule

Each milestone should conclude with a working demonstration:
- **Milestone 1**: Show CLI commands working locally
- **Milestone 2**: Restart application, data still there
- **Milestone 3**: Connect from multiple terminals
- **Milestone 4**: Kill leader, show failover
- **Milestone 5**: Network partition demonstration
- **Milestone 6**: Performance comparison demo
- **Milestone 7**: Complex fault tolerance scenarios
- **Milestone 8**: Production deployment demo