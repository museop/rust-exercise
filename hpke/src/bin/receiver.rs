use hpke::{
    aead::ChaCha20Poly1305, kdf::HkdfSha384, kem::X25519HkdfSha256, Deserializable,
    Kem as KemTrait, OpModeR, Serializable,
};
use rand::{rngs::StdRng, SeedableRng};
use std::io;

// Setup KEM, KDF, and AEAD
type Kem = X25519HkdfSha256;
type Kdf = HkdfSha384;
type Aead = ChaCha20Poly1305;

fn generate_keypair() -> (<Kem as KemTrait>::PrivateKey, <Kem as KemTrait>::PublicKey) {
    let (sk_r, pk_r) = <Kem as KemTrait>::gen_keypair(&mut StdRng::from_os_rng());
    (sk_r, pk_r)
}

fn main() {
    // Receiver: generate a key pair
    let (sk_r, pk_r) = generate_keypair();

    // Print the public key for the sender to use
    println!(
        "Receiver's Public Key (hex): {}",
        hex::encode(pk_r.to_bytes())
    );
    println!("--------------------------------------------------");
    println!("Waiting for sender's output...");

    // Read the encapsulated key from stdin
    println!("Enter Encapsulated Key (hex):");
    let mut enc_hex = String::new();
    io::stdin().read_line(&mut enc_hex).unwrap();
    let enc_bytes = hex::decode(enc_hex.trim()).expect("Invalid hex for encapsulated key");
    let enc =
        <Kem as KemTrait>::EncappedKey::from_bytes(&enc_bytes).expect("Invalid encapsulated key");

    // Read the ciphertext from stdin
    println!("Enter Ciphertext (hex):");
    let mut ct_hex = String::new();
    io::stdin().read_line(&mut ct_hex).unwrap();
    let ct = hex::decode(ct_hex.trim()).expect("Invalid hex for ciphertext");

    // Receiver: setup HPKE context for opening
    let mut receiver_context = hpke::setup_receiver::<Aead, Kdf, Kem>(
        &OpModeR::Base,
        &sk_r,
        &enc,
        b"some info", // application-specific info
    )
    .unwrap();

    // Receiver: open the ciphertext
    let aad = b"additional data";
    let opened_pt = receiver_context.open(&ct, aad).unwrap();

    println!(
        "Successfully decrypted: {}",
        String::from_utf8_lossy(&opened_pt)
    );
}
