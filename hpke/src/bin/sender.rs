use clap::Parser;
use hpke::{
    aead::ChaCha20Poly1305, kdf::HkdfSha384, kem::X25519HkdfSha256, Deserializable,
    Kem as KemTrait, OpModeS, Serializable,
};
use rand::{rngs::StdRng, SeedableRng};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Receiver's public key in hex format
    #[arg(short, long)]
    public_key_hex: String,

    /// Plaintext message to encrypt
    #[arg(short, long)]
    message: String,
}

// Setup KEM, KDF, and AEAD
type Kem = X25519HkdfSha256;
type Aead = ChaCha20Poly1305;
type Kdf = HkdfSha384;

fn decode_public_key(public_key_hex: String) -> <Kem as KemTrait>::PublicKey {
    let pk_r_bytes = hex::decode(public_key_hex).expect("Invalid hex for public key");
    let pk_r =
        <Kem as KemTrait>::PublicKey::from_bytes(&pk_r_bytes).expect("Invalid public key bytes");
    return pk_r;
}

fn main() {
    let cli = Cli::parse();
    let public_key_hex = cli.public_key_hex;
    let message = cli.message;

    // Decode the receiver's public key
    let pk_r = decode_public_key(public_key_hex);

    // Sender: setup HPKE
    let mut csprng = StdRng::from_os_rng();
    let (enc, mut sender_context) = hpke::setup_sender::<Aead, Kdf, Kem, _>(
        &OpModeS::Base,
        &pk_r,
        b"some info", // application-specific info
        &mut csprng,
    )
    .unwrap();

    let pt = message.as_bytes();
    let aad = b"additional data";
    let ct = sender_context.seal(pt, aad).unwrap();

    println!("Encapsulated Key (hex): {}", hex::encode(enc.to_bytes()));
    println!("Ciphertext (hex): {}", hex::encode(&ct));
}
