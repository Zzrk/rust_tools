use crate::cli::RunCommand;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose, Engine as _};
use clap::Args;
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};
use std::error::Error;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

/// 此处采用飞书开放平台的 AES-256-CBC 加密方式
/// https://open.feishu.cn/document/server-docs/event-subscription-guide/event-subscription-configure-/encrypt-key-encryption-configuration-case
/// 1. 使用 SHA256 对 Encrypt Key 进行哈希得到密钥 key。
/// 2. 使用 PKCS7Padding 方式将事件内容进行填充。
/// 3. 生成 16 字节的随机数作为初始向量 iv。
/// 4. 使用 iv 和 key 对事件内容加密得到 encrypted_event。
/// 5. 应用收到的密文为 base64(iv+encrypted_event)。
#[derive(Args)]
pub struct AesArgs {
    /// Encrypt or decrypt message
    message: String,
    /// Encrypt message, default
    #[arg(short, long)]
    encrypt: bool,
    /// Decrypt message, should not be used with encrypt
    #[arg(short, long)]
    decrypt: bool,
    /// Key to encrypt or decrypt
    #[arg(short, long)]
    key: String,
}

impl AesArgs {
    /// 生成密钥
    fn generate_key(&self) -> [u8; 32] {
        // 通过 sha256 生成 32 字节的密钥
        let mut hasher = Sha256::new();
        hasher.update(self.key.as_bytes());
        let key = hasher.finalize().into();
        key
    }

    /// 生成随机 iv
    fn generate_iv(&self) -> [u8; 16] {
        // iv 为 16 字节
        let mut bytes = [0u8; 16];
        let mut rng = OsRng;
        rng.fill_bytes(&mut bytes);
        bytes
    }

    fn encrypt(&self) -> String {
        // 生成密钥
        let key = self.generate_key();
        // 生成随机 iv
        let iv: [u8; 16] = self.generate_iv();

        // 每 16 字节进行填充
        let buf_len = self.message.len() + 16 - self.message.len() % 16;
        let mut buf = vec![0u8; buf_len];

        // 指定密钥、iv、PKCS7Padding 的填充方式、明文, 进行加密
        Aes256CbcEnc::new(key.as_slice().into(), &iv.into())
            .encrypt_padded_b2b_mut::<Pkcs7>(self.message.as_bytes(), &mut buf)
            .unwrap();

        // iv + ct
        let mut ct = iv.to_vec();
        ct.extend_from_slice(&buf);

        // base64 编码
        general_purpose::STANDARD.encode(ct)
    }

    fn decrypt(&self) -> String {
        // 生成密钥
        let key = self.generate_key();
        // base64 解码
        let bytes = general_purpose::STANDARD
            .decode(self.message.as_bytes())
            .unwrap();

        // 前 16 字节为 iv, 后面为密文
        let iv = &bytes[0..16];
        let ciphertext = &bytes[16..];

        // 每 16 字节进行填充
        let buf_len = ciphertext.len() + 16 - ciphertext.len() % 16;
        let mut buf = vec![0u8; buf_len];

        // 指定密钥、iv、PKCS7Padding 的填充方式、密文, 进行解密
        let ct = Aes256CbcDec::new(key.as_slice().into(), iv.into())
            .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
            .unwrap();

        // &[u8] -> String
        String::from_utf8_lossy(&ct).to_string()
    }
}

impl RunCommand for AesArgs {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.encrypt && self.decrypt {
            println!("encrypt and decrypt can't be used together");
        } else if self.decrypt {
            println!("{}", self.decrypt());
        } else {
            println!("{}", self.encrypt());
        }
        Ok(())
    }
}
