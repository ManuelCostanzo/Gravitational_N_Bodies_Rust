# Gravitational N bodies simulation using Rust.

This repo contains an optimized Rust implementation of the N-Body simulation for x86 architectures.

## Usage

RUSTFLAGS='-C **target-cpu=native** -Ctarget-feature=+avx'  cargo +nightly build --release --bin **opt**

```
target-cpu: it depends on the architecture.
  
opt1: naive version
opt2: multi-threading version
opt3: fold version
opt4-3: math optimization version
opt5: Jemalloc version
opt6: block processing version
```

## References

Please, cite accordingly:

Costanzo, M., Rucci E., Naiouf, M., De Giusti, A. (2021) Performance vs Programming Effort between Rust and C on Multicore Architectures: Case Study in N-Body. In: Actas de la XLVII Conferencia Latinoamericana de Informática  (CLEI 2021). In press. [PDF](https://arxiv.org/abs/2107.11912)
