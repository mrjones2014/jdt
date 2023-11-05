pub fn checksum_string(data: &[u8]) -> String {
    let hash = ring::digest::digest(&ring::digest::SHA256, data);
    data_encoding::HEXLOWER.encode(hash.as_ref())
}
