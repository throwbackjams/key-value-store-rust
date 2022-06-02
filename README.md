# Summary
A Rust implementation of a persistent key/value store server and client with synchronous networking

_Features_- pluggable storage engines (sled crate or `kvs`, the custom implementation for learning purposes)

_Main branch_ - a single-threaded implementation

_Multi-threaded branch_ - a multi-threaded implementation with pluggable thread pool implementations (rayon crate or `SharedQueueThreadPool`, the custom implementation for learning purposes)

# Next Steps

Async
