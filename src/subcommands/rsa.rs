use clap::Args;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{FromPrimitive, One, Zero};

/// RSA 加解密，仅用于学习
/// 参考资料：阮一峰的 RSA 算法原理
/// https://www.ruanyifeng.com/blog/2013/06/rsa_algorithm_part_one.html
/// https://www.ruanyifeng.com/blog/2013/07/rsa_algorithm_part_two.html
#[derive(Args)]
pub struct RsaArgs {
    // /// Encrypt or decrypt message
    // message: String,
    // /// Encrypt message, default
    // #[arg(short, long)]
    // encrypt: bool,
    // /// Decrypt message, should not be used with encrypt
    // #[arg(short, long)]
    // decrypt: bool,
}

impl RsaArgs {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 选择两个不相等的质数 p 和 q
        let p: BigUint = BigUint::from_u32(61).unwrap();
        let q: BigUint = BigUint::from_u32(53).unwrap();
        // 计算 n = p * q
        // 计算 φ(n) = φ(p * q) = (p - 1) * (q - 1)
        // 选择一个整数 e，条件是 1 < e < φ(n)，且 e 和 φ(n) 互质
        let e: BigUint = BigUint::from_u32(17).unwrap();
        // 计算 e 对于 φ(n) 的模反元素 d，即使得 e * d ≡ 1 (mod φ(n))
        // 获得公钥 (e, n) 和私钥 (d, n)
        let keys = generate_key_pair(&p, &q, &e);
        let public_key = keys.0;
        let private_key = keys.1;
        println!("PublicKey: {:?}", public_key);
        println!("PrivateKey: {:?}", private_key);

        let message: BigUint = BigUint::from_u32(123).unwrap();
        let cipher = encrypt(&public_key, &message);
        let decrypted = decrypt(&private_key, &cipher);
        println!("Message: {}", message);
        println!("Cipher: {}", cipher);
        println!("Decrypted: {}", decrypted);
        Ok(())
    }
}

/// 计算 a 对于 n 的模反元素 x，即使得 a * x ≡ 1 (mod n)
/// 中途可以使用 BigInt，但是最后返回 BigUint
/// TODO: 使用 devv 生成
fn mod_inverse(a: &BigUint, n: &BigUint) -> BigUint {
    // 检查 n 是否为 1
    if n.is_one() {
        return BigUint::one();
    }

    // 初始化变量
    let (mut a, mut m, mut x, mut inv) = (
        a.to_bigint().unwrap(),
        n.to_bigint().unwrap(),
        BigInt::zero(),
        BigInt::one(),
    );

    // 使用扩展欧几里得算法计算模反元素
    while &a > &BigInt::one() {
        let div = &a / &m;
        let rem = &a % &m;
        let temp = &inv - &div * &x;
        inv = x;
        x = temp;
        a = m;
        m = rem;
    }

    // 处理负数情况
    if &inv < &BigInt::zero() {
        inv += n.to_bigint().unwrap();
    }

    inv.to_biguint().unwrap()
}

/// 生成公钥和私钥
fn generate_key_pair(
    p: &BigUint,
    q: &BigUint,
    e: &BigUint,
) -> ((BigUint, BigUint), (BigUint, BigUint)) {
    // 计算 n = p * q
    let n = p * q;
    // 计算 φ(n) = φ(p * q) = (p - 1) * (q - 1)
    let phi = (p - BigUint::one()) * (q - BigUint::one());
    // 计算 e 对于 φ(n) 的模反元素 d，即使得 e * d ≡ 1 (mod φ(n))
    let d = mod_inverse(e, &phi);
    // 获得公钥 (e, n) 和私钥 (d, n)
    ((e.clone(), n.clone()), (d, n))
}

/// 加密
fn encrypt(public_key: &(BigUint, BigUint), message: &BigUint) -> BigUint {
    let (e, n) = public_key;
    // message ^ e mod n
    message.modpow(e, n)
}

/// 解密
fn decrypt(private_key: &(BigUint, BigUint), cipher: &BigUint) -> BigUint {
    let (d, n) = private_key;
    // cipher ^ d mod n
    cipher.modpow(d, n)
}

#[cfg(test)]
mod tests {
    /// 使用 rsa 库进行加解密
    #[test]
    fn test_rsa() {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = rsa::RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = rsa::RsaPublicKey::from(&priv_key);

        // Encrypt
        let data = b"hello world";
        let enc_data = pub_key
            .encrypt(&mut rng, rsa::Pkcs1v15Encrypt, &data[..])
            .expect("failed to encrypt");
        assert_ne!(&data[..], &enc_data[..]);

        // Decrypt
        let dec_data = priv_key
            .decrypt(rsa::Pkcs1v15Encrypt, &enc_data)
            .expect("failed to decrypt");
        assert_eq!(&data[..], &dec_data[..]);
    }
}
