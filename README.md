# magic-ring-buffer

[magic-ring-buffer] is a Rust crate providing a magic ring buffer (also known as a virtual ring buffer, VRB, or mirrored buffer) which is lock-free for multiple producers and a single consumer.

A magic ring buffer allows 'wrap-around' of a ring buffer without the need to use two separate read or two separate writes; it exploits the fact that virtual memory does not need to be implemented using contiguous physical memory.

The current design only works on Linux-like systems, as it relies on mapping files in `/dev/shm`.

It should be possible to make implementations that work on Mac OS X, the BSDs and Windows.


## Licensing

The license for this project is MIT.

[magic-ring-buffer]: https://github.com/lemonrock/magic-ring-buffer "magic-ring-buffer GitHub page"
