use bytes::Bytes;
use primitive_types::U256;
#[derive(Debug)]
pub struct Calldata {
    pub data: Bytes,
}

impl Calldata {
    pub fn new(data: Bytes) -> Self {
        Self { data }
    }

    pub fn read_byte(&self, offset: usize) -> u8 {
        if offset < self.data.len() {
            self.data[offset]
        } else {
            0
        }
    }

    pub fn read_word(&self, offset: usize) -> U256 {
        let mut bytes = [0u8; 32];
        for i in offset..offset + 32 {
            bytes[i] = self.read_byte(i)
        }
        U256::from_big_endian(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_read_byte() {
        // Convert a hexadecimal string to bytes
        let hex_data = "01020304"; // Without the "0x" prefix
        let data = match hex::decode(hex_data) {
            Ok(bytes) => Bytes::from(bytes),
            Err(e) => panic!("Failed to decode hex: {}", e),
        };
        let calldata = Calldata::new(data);

        assert_eq!(calldata.read_byte(0), 0x01);
        assert_eq!(calldata.read_byte(1), 0x02);
        assert_eq!(calldata.read_byte(2), 0x03);

        // Offset is outside the data range
        assert_eq!(calldata.read_byte(10), 0);
    }
    #[test]
    fn test_read_word() {
        let hex_data = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let data = match hex::decode(hex_data) {
            Ok(bytes) => Bytes::from(bytes),
            Err(e) => panic!("Failed to decode hex: {}", e),
        };
        let calldata = Calldata::new(data);

        let expected_word_start =
            U256::from_big_endian(&hex::decode(hex_data[..64].to_string()).unwrap());
        assert_eq!(calldata.read_word(0), expected_word_start);
    }
}
