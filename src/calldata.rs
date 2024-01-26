use bytes::Bytes;
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
    // TODO: Change to handle U256
    pub fn read_word(&self, offset: usize) -> u128 {
        let mut bytes = [0u8; 16];
        for i in offset..offset + 16 {
            bytes[i] = self.read_byte(i) as u8
        }
        u128::from_be_bytes(bytes)
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
        // Extended hex_data to 32 hex digits (16 bytes)
        let hex_data = "0123456789abcdef0123456789abcdef";
        let data = match hex::decode(&hex_data) {
            Ok(bytes) => Bytes::from(bytes),
            Err(e) => panic!("Failed to decode hex: {}", e),
        };
        let calldata = Calldata::new(data);

        let offset = 0;

        // Calculate the expected u128 value from the 16 bytes starting at the offset
        let expected_word = u128::from_be_bytes(
            hex::decode(&hex_data[offset * 2..offset * 2 + 32])
                .unwrap()
                .try_into()
                .unwrap(),
        );

        let word = calldata.read_word(offset);

        assert_eq!(
            word, expected_word,
            "read_word did not return the expected value at offset {}",
            offset
        );
    }
}
