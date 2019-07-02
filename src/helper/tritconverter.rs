const BOOL_FALSE: char = '-';
const BOOL_TRUE: char = '1';
const TRYTES: &str = "NOPQRSTUVWXYZ9ABCDEFGHIJKLM";
const TRYTE_TRITS: [&str; 27] = [
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




pub struct TritConverter {
    pub BOOL_FALSE: char,

}