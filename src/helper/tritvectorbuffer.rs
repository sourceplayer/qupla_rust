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

    pub fn grow(mut self, needed: usize) -> TritVectorBuffer {
        let length =  &self.buffer.len();
        if length < &needed {
            let mut new_size = length * 3;
            while &new_size < &needed
            {
                new_size *= 3;
            }

            let mut new_buffer: Vec<char>;
            new_buffer = self.buffer.clone();
            self.buffer = new_buffer;
        }
        self
    }

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