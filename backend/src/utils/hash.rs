use sha2::{Sha256, Digest};

pub fn sha256str(string : &str) -> String {
    let mut hash = Sha256::new();
    hash.update(string);
    format!("{:x}", hash.finalize())
}
