// Mush8 - sarc.rs
// Constantin Clerc (c22dev) - 0.1
use anyhow::{Result, bail, ensure};

pub fn yaz0(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 16 || &data[..4] != b"Yaz0" {
        return Ok(data.to_vec());
    }
    let size = u32::from_be_bytes(data[4..8].try_into()?) as usize;
    let mut out = Vec::with_capacity(size);
    let mut i = 16;
    while out.len() < size {
        let Some(&code) = data.get(i) else { bail!("truncated Yaz0") };
        i += 1;
        for bit in (0..8).rev() {
            if out.len() >= size {
                break;
            }
            if code >> bit & 1 == 1 {
                out.push(*data.get(i).ok_or_else(|| anyhow::anyhow!("truncated Yaz0"))?);
                i += 1;
            } else {
                ensure!(i + 1 < data.len(), "truncated Yaz0");
                let (b1, b2) = (data[i] as usize, data[i + 1] as usize);
                i += 2;
                let dist = ((b1 & 0xF) << 8 | b2) + 1;
                let n = if b1 >> 4 == 0 {
                    let n = data[i] as usize + 0x12;
                    i += 1;
                    n
                } else {
                    (b1 >> 4) + 2
                };
                ensure!(dist <= out.len(), "bad Yaz0 back-reference");
                let start = out.len() - dist;
                for k in 0..n {
                    let b = out[start + k];
                    out.push(b);
                }
            }
        }
    }
    out.truncate(size);
    Ok(out)
}

pub struct Entry<'a> {
    pub name: String,
    pub data: &'a [u8],
}

pub fn sarc(data: &[u8]) -> Result<Vec<Entry<'_>>> {
    ensure!(data.len() >= 0x20 && &data[..4] == b"SARC", "not a SARC");
    let le = u16::from_le_bytes([data[6], data[7]]) == 0xFEFF;
    let u16_at = |o: usize| -> u16 { if le { u16::from_le_bytes([data[o], data[o + 1]]) } else { u16::from_be_bytes([data[o], data[o + 1]]) } };
    let u32_at = |o: usize| -> u32 {
        let b: [u8; 4] = data[o..o + 4].try_into().unwrap();
        if le { u32::from_le_bytes(b) } else { u32::from_be_bytes(b) }
    };
    let header = u16_at(4) as usize;
    let data_off = u32_at(0xC) as usize;
    let sfat = header;
    ensure!(&data[sfat..sfat + 4] == b"SFAT", "missing SFAT");
    let count = u16_at(sfat + 6) as usize;
    let nodes = sfat + 0xC;
    let sfnt = nodes + count * 0x10;
    ensure!(&data[sfnt..sfnt + 4] == b"SFNT", "missing SFNT");
    let names = sfnt + 8;
    let mut out = Vec::with_capacity(count);
    for k in 0..count {
        let n = nodes + k * 0x10;
        let attr = u32_at(n + 4);
        let (start, end) = (u32_at(n + 8) as usize, u32_at(n + 0xC) as usize);
        let name = if attr >> 24 != 0 {
            let o = names + (attr & 0xFFFF) as usize * 4;
            let e = data[o..].iter().position(|&c| c == 0).unwrap_or(0);
            String::from_utf8_lossy(&data[o..o + e]).into_owned()
        } else {
            format!("{:08x}", u32_at(n))
        };
        ensure!(data_off + end <= data.len() && start <= end, "SARC entry out of range");
        out.push(Entry { name, data: &data[data_off + start..data_off + end] });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaz0_literals_and_backrefs() {
        let mut d = b"Yaz0".to_vec();
        d.extend_from_slice(&9u32.to_be_bytes());
        d.extend_from_slice(&[0; 8]);
        d.push(0b1110_0000);
        d.extend_from_slice(b"abc");
        d.extend_from_slice(&[0x40, 0x02]);
        assert_eq!(yaz0(&d).unwrap(), b"abcabcabc");
    }
}
