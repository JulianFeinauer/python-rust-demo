use std::io::Write;

#[allow(dead_code)]
enum Endianess {
    LittleEndian,
    BigEndian
}

pub struct WriteBuffer {
    position: u64,
    endianness: Endianess,
    // There can be a situation where only some bits are written, then we need a BitWriter
    unfinished_byte: Option<u8>,
    writer: dyn Write,
}

pub struct BitWriter {
    position: u8,
    value: u8,
}

impl BitWriter {

    // Writes the given value as the given number of bits to the Bitwriter
    // If it "overflows" the "full" byte is returned
    fn write<T: Write>(&mut self, value: u8, bits: u8, writer: &mut T) {
        // Write until the byte is full or bits are over
        let mut bit_index: u8 = 0;
        loop {
            if self.position == 8 {
                // Flush and then go to 0 again
                self.flush(writer);
            }
            if bit_index == bits {
                break;
            }
            let mask = ((value >> bit_index) & (0x01)) << self.position;
            self.value = self.value | mask;

            bit_index += 1;
            self.position += 1;
        }
    }

    fn flush<T: Write>(&mut self, writer: &mut T) {
        writer.write(&[self.value]);
        self.position = 0;
        self.value = 0;
    }

}

#[macro_export]
macro_rules! write_int {
    ($func:ident, $type:ty) => {
        pub fn $func(&mut self, x: $type) -> std::io::Result<usize> {
        let bytes = match self.endianness {
            Endianess::LittleEndian => {
                x.to_le_bytes()
            }
            Endianess::BigEndian => {
                x.to_be_bytes()
            }
        };
        self.write(&bytes)
    }
    };
}

impl WriteBuffer {

    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let bytes_written = self.writer.write(&bytes)?;
        self.position = self.position + bytes_written as u64;
        Ok(bytes_written)
    }

    pub fn write_u8(&mut self, x: u8) -> std::io::Result<usize> {
        self.write(&[x])
    }

    write_int!(write_u16, u16);
    write_int!(write_u32, u32);
    write_int!(write_u64, u64);
    write_int!(write_u128, u128);

    write_int!(write_i8, i8);
    write_int!(write_i16, i16);
    write_int!(write_i32, i32);
    write_int!(write_i64, i64);
    write_int!(write_i128, i128);

    write_int!(write_f32, f32);
    write_int!(write_f64, f64);
}

#[cfg(test)]
mod test {
    use std::io::Write;
    use crate::buffer::{BitWriter};

    #[test]
    fn test_it() {
        let mut target: u8 = 0x1;

        let value: u8 = 0x03;
        let mut position: u8 = 1;
        let num_bits = 2;

        for bit_index in 0..num_bits {
            let mask = ((value >> bit_index) & (0x01)) << position;
            target = target | mask;
            position += 1;
        }

        assert_eq!(target, 0x07);
    }

    #[test]
    fn test_write() {
        let mut writer = BitWriter {
            position: 0,
            value: 0
        };

        let buffer: Vec<u8> = vec![];

        let mut noop_writer: Box<dyn Write> = Box::new(buffer);
        writer.write(0x01, 1, &mut noop_writer);
        assert_eq!(writer.value, 0x01);
        assert_eq!(writer.position, 1);

        writer.write(0x01, 1, &mut noop_writer);
        assert_eq!(writer.value, 0x03);
        assert_eq!(writer.position, 2);

        writer.write(0x01, 1, &mut noop_writer);
        assert_eq!(writer.value, 0x07);
        assert_eq!(writer.position, 3);

        writer.write(0x03, 2, &mut noop_writer);
        assert_eq!(writer.value, 31);
        assert_eq!(writer.position, 5);

        // Now overflow
        writer.write(0x00, 3, &mut noop_writer);
        assert_eq!(writer.value, 0);
        assert_eq!(writer.position, 0);
    }

    #[test]
    fn test_write_byte() {
        let mut writer = BitWriter {
            position: 0,
            value: 0
        };

        let mut bytes: Vec<u8> = vec![];

        // Now overflow
        writer.write(0xFF, 8, &mut bytes);
        assert_eq!(writer.value, 0);
        assert_eq!(writer.position, 0);
        assert_eq!(*bytes.get(0).unwrap(), 0xFF);
    }
}

