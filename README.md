# RUST

Homepage: https://www.rust-lang.org/
Rust Book: https://doc.rust-lang.org/book/ 

Rusts goals:
- Performance
- Reliability
- Productivity

Rust's aspirational goals:
- Zero-Cost Abstractions: Move behaviour to compile time rather than runtime.
- Fearless Concurrency

Information on Rusts compiler, called rustc, is at https://doc.rust-lang.org/rustc
Cargo is Rust's build system and package manager. Refer https://doc.rust-lang.org/cargo/



Rust statically links everything but glibc (and maybe libgcc) by default. If you want to get a 100% statically linked
binary, you can use MUSL instead of glibc:
- add 'x86_64-unknown-linux-musl' as a target by doing: rustup target add x86_64-unknown-linux-musl
- then build to this target : cargo build --target=x86_64-unknown-linux-musl
- more information at https://doc.rust-lang.org/rustc/platform-support.html
