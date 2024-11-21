# sink

This module is a bad implementation of synchronisation primitives as developed for `rosti`.  
(sink? sync? hahaha)

So far, this only has one (probably flawed) spinlock-based mutex.

## Known Issues  
- `SpinMutex` can potentially deadlock
