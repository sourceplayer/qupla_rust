use super::tritvector::TritVector;
use num_bigint::{BigInt, Sign};

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

// const POWER_DIGITS: [isize;]


pub struct TritConverter {



}

impl TritConverter {
    pub fn from_decimal(decimal: String) -> String {
        if decimal.len() == 1 && (decimal.chars().nth(0).unwrap() as u8) < ('2' as u8)
        {
            return decimal;
        }

        // take abs (name)
        let negative: bool = decimal.starts_with("-");
        let value: String = if negative { String::from(&decimal[1..])} else { String::from(decimal) };

        // Convert to unbalanced trinary
        let mut buffer: Vec<char> = vec!['0'; value.len() * 3];
        let mut quotient: Vec<char> = value.chars().collect();
        for i in 0.. quotient.len() {
            quotient[i] = (quotient[i] as u8 - '0' as u8) as char;
        }

        let mut q_length = quotient.len();

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
                quotient[q_length] = (digit / 3) as u8 as char;
                q_length += 1;
                digit %= 3;
            }
            buffer[b_length] = digit as u8 as char;
            b_length += 1;
        }

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
                    carry = 1;                },
                3 => {
                    buffer[i] = '0';
                    carry = 1;
                },
                _ => {}
            }
        }

        if carry != 0 {
            buffer[b_length] = if negative {'-'} else {'1'};
            b_length += 1;
        }
        let s: String = buffer.into_iter().collect();
        String::from(&s[..b_length])
    }

    pub fn from_float(value: String, man_size: usize, exp_size: usize) -> String {
        let dot: Option<usize> = value.chars().position(|c| c == '.');
        println!("{:?}", dot);
        if dot == None {
            // Handle integer constant
            if value.eq("0") {
                // special case: both mantissa and exponent zero
                return TritConverter::zeros(man_size + exp_size)
            }

            // get minimum trit vector that represents integrer
            let trits: String = TritConverter::from_decimal(value.clone());
            
            // make sure it fits in the mantissa
            if trits.len() > man_size {
                eprintln!("Mantissa {} exceeds {} trits", &value, man_size);
            }

            // Shift all significant trits to normalize
            let mut mantissa: String = TritConverter::zeros(man_size - trits.len());
            mantissa.push_str(&trits);
            println!("Mantissa: {} Trits.len() {} expSize {}", &mantissa, trits.len(), exp_size);
            return TritConverter::make_float(mantissa, trits.len() as isize, exp_size)
        }

        // handle float constant

        // use BigInteger arithmetic to convert the value
        // <integer> * 10^-<decimals> * 3^0
        // into the following without losing too much precision
        // <ternary> * 10^0 * 3^<exponent>
        // we do that by calculating a minimum necessary ternary exponent,
        // then multiply by that <exponent> and divide by 10^<decimals>
        // after that it becomes a matter of normalizing and rounding the
        // ternary representation of the result

        let dot: usize = dot.unwrap();
        let decimals: usize = value.len() - dot - 1;
        let integer: String = String::from(format!("{}{}", &value[0..dot], &value[dot + 1..]));
        let int_value: BigInt = BigInt::from(integer.parse::<BigInt>().unwrap());
        let ten_power: BigInt = BigInt::from(format!("1{}", TritConverter::zeros(decimals)).parse::<BigInt>().unwrap());
        let exponent: isize = -((man_size + 20 + 3 * decimals) as isize);
        let trinary: BigInt = int_value * (TritConverter::get_power(-exponent as usize) / ten_power);
        let trits: String = TritConverter::from_decimal(trinary.make_string());
        
        // take <man_size> most significant trits
        let mantissa: String = String::from(&trits[trits.len() - man_size as usize..]);
        println!("Mantissa: {}", &mantissa );

        TritConverter::make_float(mantissa, exponent + trits.len() as isize, exp_size)
    }

    pub fn get_power(nr: usize) -> BigInt
    {
        let mut powers: Vec<BigInt> = vec![];
        let mut power_digits: Vec<usize> = vec![];

        powers.push(BigInt::from(1));
        power_digits.push(1);
            
        let mut big: BigInt = BigInt::from(1);
        
        for i in powers.len()..(nr + 1) as usize {
            big = big * 3;
            powers.push(big.clone());
            power_digits.push(big.make_string().len());
            // println!("Power {} - {} - digits {}", i, &powers[powers.len() - 1], &power_digits[power_digits.len() - 1]);
        }
        powers[nr].clone()
    }

    pub fn make_float(mantissa: String, exponent: isize, exp_size: usize) -> String
    {
        let trits = Self::from_long(exponent);
        println!("Exponent trits {}", &trits );

        // Make sure exponent fits
        if trits.len() > exp_size {
            eprintln!("Exponent {} exceeds {} trits", exponent, exp_size);
        }

        let mut result: String = mantissa;
        result.push_str(&trits);
        result.push_str(&TritConverter::zeros(exp_size - trits.len()));
        result
    }

    pub fn from_long(decimal: isize) -> String
    {
        //TODO replace this inefficient lazy-ass code :-P
        TritConverter::from_decimal(String::from(format!("{}", decimal)))
    }

    pub fn zeros(size: usize) -> String
    {
        TritVector::new(size, '0').trits()
    }

    pub fn to_decimal(trits: String) -> BigInt {
        let mut result = BigInt::from(0);
        for i in 0..trits.len() {
            let c: char = trits.chars().nth(i).unwrap();
            if c != '0' {
                let power: BigInt = TritConverter::get_power(i);
                result = if c == '-' { result - power } else { result + power };
            }
        }
        result
    }

    pub fn to_float(trits: String, man_size: usize, exp_size: usize) -> String {

        // Find first significant trit
        let mut significant: usize = 0;
        while significant < man_size && trits.chars().nth(significant).unwrap() == '0' {
            significant += 1;
        }

        // Special case: All zero trits in mantissa
        if significant == man_size {
            return String::from("0.0")
        }

        // Shift the significant trits of the mantissa to the left to get
        // its integer representation (we will need to correct the exponent)
        let mantissa: String = String::from(&trits[significant..man_size]);
        let integer: BigInt = TritConverter::to_decimal(mantissa.clone());

        // Get exponent and correct with mantissa shift factor
        let exponent: isize = TritConverter::to_int(String::from(&trits[man_size..])) as isize - mantissa.len() as isize;
        if exponent == 0 {
            // Simple case: 3^0 equals 1, just return integer
            return String::from(format!("{}.0", integer))
        }

        if exponent > 0 {
            return String::from(format!("{}.0", integer * TritConverter::get_power(exponent as usize)))
        }

        TritConverter::to_float_with_fraction(integer, exponent, man_size)
    }

    fn to_float_with_fraction(integer: BigInt, exponent: isize, man_size: usize) -> String
    {
        if integer.sign() == Sign::Minus
        {
            return String::from(format!("-{}", TritConverter::to_float_with_fraction(integer * -1, exponent, man_size)))
        }

        let digits: usize = TritConverter::get_power(man_size).make_string().len();
        let lhs_length: usize = integer.make_string().len();
        let power: BigInt = TritConverter::get_power(-exponent as usize);
        let rhs_length: usize = power.make_string().len();
        let extra: usize = if lhs_length < rhs_length { rhs_length - lhs_length } else { 0 };
        let mul: BigInt = integer * BigInt::from(format!("1{}", TritConverter::zeros(digits + extra)).parse::<BigInt>().unwrap());
        let div: BigInt = mul / power;
        let div_result: String = div.make_string();
        let decimal: isize = div_result.len() as isize - digits as isize - extra as isize;
        let mut last = div_result.len() - 1;
        
        while last > 0 && last as isize > decimal && div_result.chars().nth(last - 1).unwrap() == '0' {
            last -= 1;
        } 

        if decimal < 0 {
            return String::from(format!("0.{}{}", TritConverter::zeros(-decimal as usize), &div_result[0..last]))
        }

        let fraction: String = String::from(if last as isize == decimal { "0" } else { &div_result[decimal as usize..last] });
        if decimal == 0 {
            return String::from(format!("0.{}", fraction))
        }

        return String::from(format!("{}.{}", &div_result[0..decimal as usize], fraction))
    }



    fn to_int(trits: String) -> isize {
        let mut result: isize = 0;
        let mut power: isize = 1;
        for i in 0..trits.len() {
            let trit: char = trits.chars().nth(i).unwrap();
            if trit != '0' {
                result += if trit == '-' { -power } else { power }
            }
            power *= 3;
        }
        result
    }

    pub fn to_long(trits: String) -> i64 {
        let mut result: i64 = 0;
        let mut power: i64 = 1;

        for i in 0..trits.len() {
            let trit = trits.chars().nth(i).unwrap();
            if (trit != '0') {
                result += if trit == '-' { -power } else { power };
            }
            power *= 3;
        }
        return result;
    }

    pub fn trits_to_trytes(trits: Vec<i8>) -> String {
        
        let size = trits.len() / 3;
        let mut buffer: Vec<char> = vec!['0'; size];
        let mut offset = 0;
        for i in 0..size {
            let index: i8 = trits[offset] + trits[offset + 1] * 3 + trits[offset + 2] * 9;
            buffer[i] = TRYTES.chars().nth((index + 13) as usize).unwrap();
            offset += 3;
        }
        buffer.make_string()
    } 

    pub fn trits_to_vector(trits: Vec<i8>) -> TritVector {
        TritVector::from(trits)
    }

    pub fn trytes_to_trits(trytes: String) -> Vec<i8> {
        let mut result: Vec<i8> = vec![0; trytes.len() * 3];
        let mut offset: usize = 0;
        for i in 0..trytes.len() {
            let index: usize = TRYTES.chars().position(|c| c == trytes.chars().nth(i).unwrap()).unwrap();
            let trits: String = String::from(TRYTE_TRITS[index]);
            for j in 0..3 {
                match trits.chars().nth(j).unwrap() {
                    '1' => { result[offset + j] = 1 },
                    '-' => { result[offset + j] = -1 },
                    _ => {}
                }
            }
            offset += 3;
        }
        result
    }

    pub fn trytes_to_vector(trytes: String) -> TritVector {
        TritVector::from_trytes(trytes)
    }

    pub fn vector_to_trits(vector: TritVector) -> Vec<i8> {
        let mut result: Vec<i8> = vec![0; vector.size()];
        for i in 0..result.len() {
            match vector.trit(i) {
                '1' => { result[i] = 1 },
                '-' => { result[i] = -1 },
                _ => {}
            }
        }
        result
    }

    pub fn vector_to_trytes(vector: TritVector) -> String {
        vector.to_trytes()
    }
}

trait Stringify {
    fn make_string(&self) -> String;
}

impl Stringify for BigInt {
    fn make_string(&self) -> String
    {
        String::from(format!("{}", &self))
    }
}

impl Stringify for Vec<char> {
    fn make_string(&self) -> String {
        self.into_iter().collect()
    }
}


