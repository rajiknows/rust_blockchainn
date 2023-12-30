//use rsa::{RsaPrivateKey, RsaPublicKey}
    use rsa::{RsaPrivateKey, RsaPublicKey};
    pub use secp256k1::{ecdsa::Signature, key::PublicKey, Message, Secp256k1};
    use sha2::{
        digest::{crypto_common::KeyInit, Update},
        Digest, Sha256,
    };
    pub use uuid::Uuid;

    pub struct KeyPair {
        public_key: String,
        private_key: String,
    }
    pub struct ChainUtil;

    impl ChainUtil {
        pub fn gen_key_pair() -> KeyPair {
            let bits = 256;
            let mut range = rand::thread_rng();
            let private_key =
                RsaPrivateKey::new(&mut range, bits).expect("unable to generate the private key");
            let public_key = RsaPublicKey::from(&private_key);

            KeyPair {
                public_key: format!("{:?}", public_key),
                private_key: format!("{:?}", private_key),
            }
        }

        pub fn gen_hash(data: String) -> String {
            let mut hasher = Sha256::new();
            Update::update(&mut hasher, data.as_bytes());
            let hash_bytes = hasher.finalize();
            format!("{:?}", hash_bytes)
        }

        pub fn gen_id() -> String {
            Uuid::new_v4().to_string()
        }

        pub fn verify_sign(public_key: KeyPair, signature: &str, expected_data_hash: &str) -> bool {
            let publickey = PublicKey::from_slice(
                &hex::decode(&public_key.public_key).expect("couldnt decode public_key"),
            )
            .expect("invalid public key");
            let signature =
                Signature::from_der(&hex::decode(signature).expect("couldnt decode signature"))
                    .expect("invalid signature");
            let expected_data_hash = Message::from_slice(
                &hex::decode(expected_data_hash).expect("unable to  parse the expected data"),
            )
            .expect("invalid data");

            let secp = Secp256k1::new();

            secp.verify_ecdsa(&expected_data_hash, &signature, &publickey)
                .is_ok()
        }
    }

