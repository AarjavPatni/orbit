<idea>
    A distributed, fault-tolerant key-value store in Rust that can be driven entirely from a CLI,
    much like Redis but with built-in consensus for consistency across nodes. The project will 
    evolve in demoable milestones: starting with a single-node in-memory store, then layering on 
    persistence, consensus (both per-transaction and replicated log approaches), and multiple 
    transport backends (goroutines, Unix sockets, TCP). The end goal is a system where you can 
    visibly demonstrate fault tolerance from the command line—partitioning nodes, killing leaders, 
    and still retrieving consistent data—while also experimenting with different designs for 
    canonical state, all within a modular Rust workspace.
</idea>
