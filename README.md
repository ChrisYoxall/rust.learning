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

Use rustfmt to format Rust code.


## OO or Functional?

Rust does have support for Object-Oriented Programming, but it is not the primary paradigm. It also has support for
Functional Programming, but again, it is not the primary paradigm. Rust is a multi-paradigm language. See:
- https://doc.rust-lang.org/book/ch13-00-functional-features.html
- https://doc.rust-lang.org/book/ch17-00-oop.html








Rust statically links everything but glibc (and maybe libgcc) by default. If you want to get a 100% statically linked
binary, you can use MUSL instead of glibc:
- add 'x86_64-unknown-linux-musl' as a target by doing: rustup target add x86_64-unknown-linux-musl
- then build to this target : cargo build --target=x86_64-unknown-linux-musl
- more information at https://doc.rust-lang.org/rustc/platform-support.html






## Checkout Out

- Rust Ray Tracer Challenge video https://www.youtube.com/watch?v=xGEDQXBMdV4&list=PLy68GuC77sUTyOUvDhVboQoOlHoa4XrSO&index=2 and repo https://github.com/jakobwesthoff/the_ray_tracer_challenge_in_rust
- Another Ray Tracer Challenge https://github.com/ahamez/ray-tracer
- And another Ray Tracer Challenge https://github.com/lerouxrgd/raytracer








