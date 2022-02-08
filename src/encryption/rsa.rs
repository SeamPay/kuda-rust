use openssl::rsa::{Padding, Rsa};

pub fn rsa_encrypt(password: &str, public_key: &str) -> String {
    let rsa = Rsa::public_key_from_pem(public_key.as_bytes()).unwrap();
    let mut buf = vec![0; rsa.size() as usize];
    let _ = rsa
        .public_encrypt(password.as_bytes(), &mut buf, Padding::PKCS1)
        .unwrap();

    base64::encode(buf)
}

pub fn rsa_decrypt(encrypted_data: String, private_key: &str) -> String {
    let rsa = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa
        .private_decrypt(
            &base64::decode(encrypted_data.as_bytes()).unwrap(),
            &mut buf,
            Padding::PKCS1,
        )
        .unwrap();

    String::from_utf8(buf).unwrap()
}
