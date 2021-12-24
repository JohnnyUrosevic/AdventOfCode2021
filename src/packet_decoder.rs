use crate::get_input::get_input;

enum PacketType {
  Sum = 0b000,
  Product = 0b001,
  Minimum = 0b010,
  Maximum = 0b011,
  Literal = 0b100,
  GreaterThan = 0b101,
  LessThan = 0b110,
  EqualTo = 0b111,
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

  fn get_packet(&mut self) -> u64 {
    self.version_sum += self.read(3);

    let packet_type = self.read(3);

    if packet_type == PacketType::Literal as u64 {
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
          return value;
        }
      }
    }
    else {
      let length_type = self.read(1);

      let mut values = vec![];
      if length_type == 0 {
        let bit_length = self.read(15) as usize;

        let original_index = self.bit_index;

        while self.bit_index - original_index < bit_length {
          values.push(self.get_packet());
        }
      }
      else {
        let packet_length = self.read(11) as usize;

        for _ in 0..packet_length {
          values.push(self.get_packet());
        }
      }

      match packet_type {
        packet_type if packet_type == PacketType::Sum as u64 => 
          values.iter().sum(),
        packet_type if packet_type == PacketType::Product as u64 => 
          values.iter().product(),
        packet_type if packet_type == PacketType::Minimum as u64 => 
          *values.iter().min().expect("Must be at least one sub packet"),
        packet_type if packet_type == PacketType::Maximum as u64 => 
          *values.iter().max().expect("Must be at least one sub packet"),
        packet_type if packet_type == PacketType::GreaterThan as u64 => 
          (values[0] > values[1]) as u64,
        packet_type if packet_type == PacketType::LessThan as u64 => 
          (values[0] < values[1]) as u64,
        packet_type if packet_type == PacketType::EqualTo as u64 => 
          (values[0] == values[1]) as u64,
        _ => panic!("Literals should not get this far"),
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
  let result = bs.get_packet();
  
  (bs.version_sum, result)
}
