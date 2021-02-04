extern crate miracl_core;

use miracl_core::rand::{RAND_impl, RAND};

use functional_encryption_schemes::dmcfe_ip::*;

fn main() {
    let num_clients: usize = 10;
    let bound = BigNum::new_int(10000);
    let mut clients: Vec<Dmcfe> = Vec::with_capacity(num_clients);
    let mut pub_keys: Vec<G1> = Vec::with_capacity(num_clients);
    let mut ciphers: Vec<G1> = Vec::with_capacity(num_clients);
    let mut fe_key: Vec<G2Vector> = Vec::with_capacity(num_clients);
    let mut temp: G1;
    let mut raw: [u8; 100] = [0; 100];

    let mut rng = RAND_impl::new();
    rng.clean();
    for i in 0..100 {
        raw[i] = i as u8
    }
    rng.seed(100, &raw);

    for i in 0..num_clients {
        clients.push(Dmcfe::new(&mut rng, i));
    }

    for i in 0..num_clients {
        temp = clients[i].client_pub_key.clone();
        pub_keys.push(temp);
    }

    for i in 0..num_clients {
        clients[i].set_share(&pub_keys);
    }

    let label = "dmcfe-label";
    let mut x: Vec<BigNum> = Vec::with_capacity(num_clients);
    let y = vec![BigNum::new_int(1); num_clients];

    for i in 0..num_clients {
        x.push(BigNum::new_int(i as isize));
    }

    for i in 0..num_clients {
        ciphers.push(clients[i].encrypt(&x[i], label));
        fe_key.push(clients[i].derive_fe_key_share(&y));
    }
    use std::time::Instant;
    let now = Instant::now();
    let xy = Dmcfe::decrypt(&ciphers, &y, &fe_key, label, &bound);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("xy {:?}", xy);
}
