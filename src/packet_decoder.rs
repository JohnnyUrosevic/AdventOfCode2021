use crate::get_input::get_input;

enum PacketType {
  Literal = 0b100,
}

struct BitStream {
  bits: Vec<u64>,
  bit_index: usize,
  version_sum: u64,
}

impl BitStream {
  fn new(bits: Vec<u64>) -> Self {
    BitStream{bits, bit_index: 0, version_sum: 0}
  }

  fn read(&mut self, count: usize) -> u64 {
    let mut result = 0;
    for _ in 0..count {
      result = (result << 1) + self.bits[self.bit_index];
      self.bit_index += 1;
    }

    result
  }

  fn get_packet(&mut self) {
    self.version_sum += self.read(3);
    println!("version sum {}", self.version_sum);

    let packet_type = self.read(3);

    if packet_type == PacketType::Literal as u64 {
      println!("literal");
      let mut last_chunk = false;
      let mut value = 0;
      loop {
        value = value << 4;

        let header = self.read(1);
        if header == 0 {
          last_chunk = true;
        }

        value += self.read(4);

        if last_chunk {
          break;
        }
      }
    }
    else {
      println!("operator");
      let length_type = self.read(1);

      println!("length type: {}", length_type);

      if length_type == 0 {
        let bit_length = self.read(15) as usize;

        println!("bit length: {}", bit_length);
        
        let original_index = self.bit_index;

        while self.bit_index - original_index < bit_length {
          self.get_packet();
        }
      }
      else {
        let packet_length = self.read(11) as usize;

        println!("packet length: {}", packet_length);

        for _ in 0..packet_length {
          self.get_packet();
        }
      }
    }
  }

}

pub fn packet_decoder() -> (u64, u64) {
  let input = get_input(16).expect("Could not get input");

  let bits: Vec<u64> = input.iter()
    .next()
    .expect("Bad Input")
    .chars()
    .map(|c| {
      let mut nibble = c.to_digit(16).expect("Bad Input") as u64;
      let mut bits = [0; 4];
      for i in 0..4 {
        bits[i] = nibble & 1;
        nibble = nibble >> 1;
      }

      bits.into_iter().rev().collect::<Vec<u64>>()
    })
    .flatten()
    .collect();

  
  let mut bs = BitStream::new(bits);
  bs.get_packet();
  
  (bs.version_sum, 0)
}
