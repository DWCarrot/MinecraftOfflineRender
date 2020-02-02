
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::fmt;

const ACC_GROUP_LENS: [usize; 5] = [8, 12, 16, 20, 32];

#[derive(Clone, PartialEq, Eq)]
pub struct UuidStorage(pub [u8; 16]);

impl UuidStorage {

    pub fn parse(s: &str) -> Option<Self> {
        let bs = s.bytes();
        let mut data = [0u8; 16];
        match bs.len() {
            32 => {
                let mut acc = 0;
                for (i, c) in bs.enumerate() {
                    let c = match c {
                        b'0'..=b'9' => c - b'0',
                        b'a'..=b'f' => c - b'a' + 10,
                        b'A'..=b'F' => c - b'A' + 10,
                        _ => { return None; }
                    };
                    if i & 0x1 == 0x1 {
                        acc |= c;
                        data[i >> 1] = acc;
                    } else {
                        acc = c << 4;
                    }
                }
            },
            36 => {
                let mut acc = 0;
                let mut i = 0;
                let mut isp = 0;
                for (j, c) in bs.enumerate() {
                    let c = match c {
                        b'0'..=b'9' => { i += 1; c - b'0' },
                        b'a'..=b'f' => { i += 1; c - b'a' + 10 },
                        b'A'..=b'F' => { i += 1; c - b'A' + 10 },
                        b'-' => {
                            if j == ACC_GROUP_LENS[isp] {
                                isp += 1;
                                continue;
                            } else {
                                return None; 
                            }
                        },
                        _ => { return None; }
                    };
                    if i & 0x1 == 0x1 {
                        acc |= c;
                        data[i >> 1] = acc;
                    } else {
                        acc = c << 4;
                    }
                }
            },
            _ => { return None; }
        };
        Some(UuidStorage(data))
    }

    pub fn to_string(&self, lower_case: bool, hyphenated: bool) -> String {
        let mut buf = String::with_capacity(36);
        let numbase = b'0';
        let alpbase = if lower_case { b'a' - 10 } else { b'A' - 10 };
        let mut i = 0;
        let mut j = 0;
        for b in &self.0 {
            let h = ((*b) >> 4) & 0x0F;
            if h < 10 {
                buf.push( (h + numbase) as char );
            } else {
                buf.push( (h + alpbase) as char );
            }
            i += 1;
            let l = ((*b) >> 0) & 0x0F;
            if l < 10 {
                buf.push( (l + numbase) as char );
            } else {
                buf.push( (l + alpbase) as char );
            }
            i += 1;
            if hyphenated && i == ACC_GROUP_LENS[j] {
                j += 1;
                buf.push('-');
            }
        }
        buf
    }
}

impl fmt::Display for UuidStorage {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.to_string(false, false).as_str())
    }
}

impl From<&[u8; 16]> for UuidStorage {

    fn from(p: &[u8; 16]) -> Self {
       UuidStorage(unsafe { std::mem::transmute_copy(p) })
    }
}