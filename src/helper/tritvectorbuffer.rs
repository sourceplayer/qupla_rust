use std::fmt::*;

const INITIAL_SIZE: usize = 27;

pub struct TritVectorBuffer {
    pub buffer: Vec<char>,
    pub used: usize
}


impl From<usize> for TritVectorBuffer {
    fn from(size: usize) -> Self {
        let mut new_size: usize = INITIAL_SIZE;
        while &new_size < &size {
            new_size *= 3;
        }
    
        TritVectorBuffer {
            buffer: vec!['0'; new_size],
            used: size
        }
    }
}

impl TritVectorBuffer {

    pub fn grow(&mut self, needed: usize) {
        let length =  &self.buffer.len();
        if length < &needed {
            let mut new_size = length * 3;
            while new_size < needed
            {
                new_size *= 3;
            }

            for i in 0..new_size - length
            {
                self.buffer.push('0');
            }
        }
    }

    // pub fn copy(vec1: Vec<char>, vec2: Vec<char>) -> Vec<char>
    // {
    //     for item in vec1 {

    //     }

    // }

    pub fn clone(&self) -> TritVectorBuffer
    {
        TritVectorBuffer {
            buffer: self.buffer.clone(),
            used: self.used
        }
    }

}

impl PartialEq for TritVectorBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }

}


impl Display for TritVectorBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TritVectorBuffer: size {} used: {} content: {:?}",
        &self.buffer.len(),
        &self.used,
        &self.buffer)
    }
}

impl Debug for TritVectorBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TritVectorBuffer: size {} used: {} content: {:?}",
        &self.buffer.len(),
        &self.used,
        &self.buffer)
    }
}