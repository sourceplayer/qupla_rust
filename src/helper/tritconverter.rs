pub const BOOL_FALSE: char = '-';
pub const BOOL_TRUE: char = '1';
pub const TRYTES: &str = "NOPQRSTUVWXYZ9ABCDEFGHIJKLM";
pub const TRYTE_TRITS: [&str; 27] = [
      "---",
      "0--",
      "1--",
      "-0-",
      "00-",
      "10-",
      "-1-",
      "01-",
      "11-",
      "--0",
      "0-0",
      "1-0",
      "-00",
      "000",
      "100",
      "-10",
      "010",
      "110",
      "--1",
      "0-1",
      "1-1",
      "-01",
      "001",
      "101",
      "-11",
      "011",
      "111"
];

// const POWER_DIGITS: [usize;]


pub struct TritConverter {



}

impl TritConverter {
    pub fn from_decimal(decimal: String) -> String {
        if decimal.len() == 1 && decimal.chars().nth(0).unwrap() < '2'
        {
            return decimal;
        }

        // take abs (name)
        let negative: bool = decimal.starts_with("-");
        let value: String = if negative { String::from(&decimal[1..])} else { String::from(decimal) };

        // Convert to unbalanced trinary
        let mut buffer: Vec<char> = vec![];
        let mut quotient: Vec<char> = value.chars().collect();
        for i in 0.. quotient.len() {
            quotient[i] = (quotient[i] as u8 - '0' as u8) as char;
        }

        println!("{:?}", &quotient);

        let mut q_length = quotient.len();
        println!("{:?}", &quotient.len());

        let mut b_length = 0;
        while q_length != 1 || quotient[0] as u8 != 0 {
            let v_length = q_length;
            q_length = 0;
            let mut digit: u32 = quotient[0] as u32;
            if digit >= 3 || v_length == 1 {
                quotient[q_length] = (digit / 3) as u8 as char;
                q_length += 1;
                digit %= 3;
            }

            for index in 1..v_length {
                digit = digit * 10 + (quotient[index] as u32);
                quotient[q_length] = digit as u8 as char;
                q_length += 1;
                digit %= 3;
            }
            buffer.push(digit as u8 as char);
            b_length += 1;
        }

        println!("While loop finished");

        let mut carry = 0;
        for i in 0..b_length {
            match (buffer[i] as u8) + carry {
                0 => {
                    buffer[i] = '0';
                    carry = 0;
                    },
                1 => {
                    buffer[i] = if negative {'-'} else {'1'};
                    carry = 0;
                },
                2 => {
                    buffer[i] = if negative {'1'} else {'-'};
                },
                3 => {
                    buffer[i] = '0';
                    carry = 1;
                },
                _ => {}
            }
        }
        println!("For loop finished");

        if carry != 0 {
            buffer[b_length] = if negative {'-'} else {'1'};
            b_length += 1;
        }
        let s: String = buffer.into_iter().collect();
        let ss: &str = &s[..];
        String::from(format!("{}{}{}", ss, 0, b_length))
    }    
}

