# Summary
A Rust implementation of a persistent key/value store server and client with synchronous networking

Features - pluggable storage engines (sled crate or `kvs`, the custom implementation for learning purposes)

Main branch - a single-threaded implementation

Multi-threaded branch - a multi-threaded implementation with pluggable thread pool implementations (rayon crate or `SharedQueueThreadPool`, the custom implementation for learning purposes)

# Next Steps

Async
