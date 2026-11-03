// Mush8 - crypto.rs
// Constantin Clerc (c22dev) - 0.1
use aes::cipher::{BlockCipherDecrypt, BlockCipherEncrypt, KeyInit};
use aes::{Aes128, Block};

pub type Key128 = [u8; 16];

pub fn aes(key: &Key128) -> Aes128 {
    Aes128::new(&(*key).into())
}

pub fn ecb_decrypt(key: &Key128, data: &[u8; 16]) -> Key128 {
    let mut b = Block::from(*data);
    aes(key).decrypt_block(&mut b);
    b.into()
}

pub fn xts_decrypt(data: &mut [u8], key: &[u8; 32], sector_size: usize, first_sector: u64) {
    let k1 = aes(key[..16].try_into().unwrap());
    let k2 = aes(key[16..].try_into().unwrap());
    for (i, sector) in data.chunks_mut(sector_size).enumerate() {
        let mut t = Block::default();
        t[8..].copy_from_slice(&(first_sector + i as u64).to_be_bytes());
        k2.encrypt_block(&mut t);
        for chunk in sector.chunks_mut(16) {
            let mut b = Block::try_from(&*chunk).expect("sector not a multiple of 16");
            xor16(&mut b, &t);
            k1.decrypt_block(&mut b);
            xor16(&mut b, &t);
            chunk.copy_from_slice(&b);
            let carry = t[15] >> 7;
            for j in (1..16).rev() {
                t[j] = (t[j] << 1) | (t[j - 1] >> 7);
            }
            t[0] <<= 1;
            if carry != 0 {
                t[0] ^= 0x87;
            }
        }
    }
}

fn xor16(a: &mut [u8], b: &[u8]) {
    for (x, y) in a.iter_mut().zip(b) {
        *x ^= y;
    }
}

pub fn ctr_apply(cipher: &Aes128, iv: u128, data: &mut [u8]) {
    const BATCH: usize = 256;
    let mut ks = [Block::default(); BATCH];
    let mut ctr = iv;
    for chunk in data.chunks_mut(BATCH * 16) {
        let n = chunk.len().div_ceil(16);
        for b in ks.iter_mut().take(n) {
            b.copy_from_slice(&ctr.to_be_bytes());
            ctr = ctr.wrapping_add(1);
        }
        cipher.encrypt_blocks(&mut ks[..n]);
        for (d, k) in chunk.chunks_mut(16).zip(&ks[..n]) {
            if let Ok(d16) = <&mut [u8; 16]>::try_from(&mut *d) {
                let k16: [u8; 16] = k.as_slice().try_into().unwrap();
                *d16 = (u128::from_ne_bytes(*d16) ^ u128::from_ne_bytes(k16)).to_ne_bytes();
            } else {
                d.iter_mut().zip(k.iter()).for_each(|(d, k)| *d ^= k);
            }
        }
    }
}
