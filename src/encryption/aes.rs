use crypto::buffer::{ReadBuffer, WriteBuffer};
use crypto::symmetriccipher;

const KEY_LEN: usize = 32;
const IV_LEN: usize = 16;

pub fn make_derived_key(password: &str) -> [u8; 32] {
    // 256-bit derived key
    //  hashlib.pbkdf2_hmac('sha1', password, salt, 1000, dklen=32)
    //let mut dk = [0u8; 32];
    let mut derived_key = [0; KEY_LEN];
    openssl::pkcs5::pbkdf2_hmac(
        password.as_bytes(),
        "randomsalt".as_bytes(),
        1000,
        openssl::hash::MessageDigest::sha1(),
        &mut derived_key,
    )
    .unwrap();
    //let mut result = String::from("");
    //result.push_str(&derived_key.to_base64(base64::STANDARD)[..]);
    derived_key
}

pub fn make_iv(password: &str) -> [u8; 16] {
    // 256-bit iv
    //  hashlib.pbkdf2_hmac('sha1', password, salt, 1000, dklen=16)
    //let mut iv = [0u8; 16];
    let mut iv = [0; IV_LEN];
    openssl::pkcs5::pbkdf2_hmac(
        password.as_bytes(),
        "randomsalt".as_bytes(),
        1000,
        openssl::hash::MessageDigest::sha1(),
        &mut iv,
    )
    .unwrap();
    //let mut result = String::from("");
    //result.push_str(&derived_key.to_base64(base64::STANDARD)[..]);
    iv
}

// encrypt AES-256-CBC
pub fn encrypt_aes_256_cbc(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    //setup
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);
    let mut encryptor = crypto::aes::cbc_encryptor(
        crypto::aes::KeySize::KeySize256,
        key,
        iv,
        crypto::blockmodes::PkcsPadding,
    );
    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .cloned(),
        );

        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

// decrypt AES-256-CBC
pub fn decrypt_aes_256_cbc(
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = crypto::aes::cbc_decryptor(
        crypto::aes::KeySize::KeySize256,
        key,
        iv,
        crypto::blockmodes::PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .cloned(),
        );
        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}
