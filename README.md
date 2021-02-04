# functional-encryption-schemes
Implementation of Few Selected Functional Encryption Schemes



- **Scheme 1** [Decentralized Multi-Client Functional Encryption for Inner Product](https://eprint.iacr.org/2017/989.pdf) by *Chotard, Dufour Sans, Gay, Phan and Pointcheval*
    - Implemented [here](src/dmcfe_ip.rs) - this is reimplementation in Rust of C-implementation available in this awesome library [CiFEr](https://github.com/fentec-project/CiFEr). 
    - I have used BLS-12381 curve for pairing instead of BN-256 as in CiFEr.


### Build and Run Instruction
```sh
cargo build
cargo run
```
## Security Warnings

As of now, this project serves mainly proof-of-concepts, benchmarking and evaluation purpose and not for production use. Also implementation have not been fully-reviewed.

## License

All code in this workspace is licensed under

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
