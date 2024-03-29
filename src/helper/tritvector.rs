use crate::helper::tritvectorbuffer::TritVectorBuffer;
use crate::helper::tritconverter::*; 

#[derive(Debug, Clone)]
pub struct TritVector {
    pub name: String,
    offset: usize,
    size: usize,
    value_trits: usize,
    vector: TritVectorBuffer
}

impl From<&TritVector> for TritVector {
    fn from(copy: &TritVector) -> Self
    {
        TritVector {
            name: copy.name.clone(),
            offset: copy.offset,
            size: copy.offset,
            value_trits: copy.value_trits,
            vector: copy.vector.clone()
        }
    }
}

impl From<usize> for TritVector {
    fn from(size: usize) -> Self
    {
        TritVector {
            name: String::new(),
            offset: 0,
            size,
            value_trits: size,
            vector: TritVectorBuffer::from(size)
        }
    }
}

impl From<Vec<i8>> for TritVector {
    fn from(trits: Vec<i8>) -> Self
    {
        let mut trit_vector = TritVector::from(trits.len());

        for i in 0..trit_vector.size {
            trit_vector.vector.buffer[i] = "-01".chars().nth((trits[i] + 1) as usize).unwrap();
        }
        trit_vector
    }
}

impl From<String> for TritVector {
    fn from(trits: String) -> Self {
        let mut trit_vector = TritVector::from(trits.len());
        for i in 0..trit_vector.size {
            trit_vector.vector.buffer[i] = trits.chars().nth(i).unwrap();
        };
        trit_vector
    }
}

impl TritVector {

    pub fn zeros() -> TritVectorBuffer {
        TritVectorBuffer::from(0)
    }

    pub fn nulls() -> TritVectorBuffer {
        TritVectorBuffer::from(0)
    }

    pub fn signle_trits() -> TritVectorBuffer
    {
        TritVectorBuffer::from(2)
    }

    pub fn new(size: usize, trit: char) -> Self {
        let mut trit_vector = TritVector::from(size);
        match trit {
            '@' => trit_vector.vector = TritVector::nulls(),
            '0' => { 
                        trit_vector.vector = TritVector::zeros();
                        trit_vector.value_trits = size;
                    }
            '-' | '1' => {
                        if size == 1 {
                            trit_vector.vector = TritVector::signle_trits();
                            trit_vector.offset = if trit == '1' {1} else {0};
                            trit_vector.value_trits = 1;
                        }
                    }
            _ => { 
                panic!("Undefined initialization trit");
                }
            }

        trit_vector.vector.grow(size);

        while trit_vector.vector.used < trit_vector.vector.buffer.len() as usize {
            trit_vector.vector.buffer[trit_vector.vector.used] = trit;
            trit_vector.vector.used += 1;
        }
        trit_vector
    }

    pub fn combine(lhs: TritVector, rhs: TritVector) -> Self {
    
        let new_size = lhs.size + rhs.size;
        let mut trit_vector = TritVector::from(new_size);

        trit_vector.value_trits = lhs.value_trits + rhs.value_trits;

        for i in 0..lhs.size {
            trit_vector.vector.buffer[i] = lhs.vector.buffer[i];
        }

       for i in lhs.size..trit_vector.size {
            trit_vector.vector.buffer[i] = rhs.vector.buffer[i];
        }

        trit_vector
    }

    pub fn concat(lhs: Option<TritVector>, rhs: Option<TritVector>) -> Self {
        // Check if any values are None if yes return the other one and unwrap both.

        match lhs {
            Some(_) => {},
            None => {return rhs.unwrap()}
        }

        match rhs {
            Some(_) => {},
            None => {return lhs.unwrap()}
        }

        let mut lhsu = lhs.unwrap();
        let mut rhsu = rhs.unwrap();

        // Can we directly concatenete in lhs vector?
        if (&lhsu.offset + &lhsu.size != lhsu.vector.used) || (&lhsu.vector == &TritVector::nulls()) || (&lhsu.vector == &TritVector::zeros()) {
            // Combine two null vectors?
            if lhsu.is_null() && rhsu.is_null() {
                return TritVector::new(lhsu.size() + rhsu.size(), '@')
            }

            if &lhsu.vector == &TritVector::zeros() && &rhsu.vector == &TritVector::zeros() {
                return TritVector::new(lhsu.size() + rhsu.size(), '0')
            }            
            
            return TritVector::combine(lhsu, rhsu)
        }

        // grow vector if necessary

        let new_length = &lhsu.vector.used + &rhsu.size();
        lhsu.vector.grow(new_length);
    
        // concatenate into lhs vector
        let copy_to = lhsu.vector.used;
        lhsu.copy(&rhsu, &copy_to);
        lhsu.vector.used += rhsu.size();
        
        // point to the new combined vector
        let mut result: TritVector = TritVector::from(lhsu);
        result.size += rhsu.size();
        result.value_trits += rhsu.value_trits;

        return result
    }

    pub fn is_null(&self) -> bool {
        self.value_trits == 0
    }

    pub fn is_value(&self) -> bool {
        self.value_trits == self.size
    }

    pub fn is_zero(&self) -> bool {
        if self.vector == TritVector::zeros() {
            return true
        }

        if !self.is_value() {
            return false
        }

        for i in 0..self.size {
            if self.trit(i) != '0' {
                return false;
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        let mut trit_string = self.name.clone();

        trit_string.push('(');
        for i in 0..self.vector.buffer.len() {
            trit_string.push(self.vector.buffer[i]);
        }        
        
        trit_string.push(')');

        trit_string

    }

    pub fn trits(&self) -> String {
        let mut trit_string = String::new();

        for i in self.offset..self.offset + self.size() {
            trit_string.push(self.vector.buffer[i]);
        }
        trit_string
    }

    pub fn trit(&self, index: usize) -> char {
        if index >= self.size {
            eprintln!("Index out of range");
        }
        self.vector.buffer[index]
    }

    pub fn tryte(trits: Vec<char>) -> char {

        let mut value: usize = 13;

        match trits[0] {
            '-' => value -= 1,
            '1' => value += 1,
            _ => {}
        }
        match trits[1] {
            '-' => value -= 3,
            '1' => value += 3,
            _ => {}
        }
        match trits[2] {
            '-' => value -= 9,
            '1' => value += 9,
            _ => {}
        }

        TRYTES.chars().nth(value).unwrap()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn copy(&mut self, src: &TritVector, to: &usize)
    {
        for i in 0..src.size() {
            self.vector.buffer[to + i] = src.trit(i);
        }
    }

    pub fn from_trytes(trytes: String) -> TritVector {
        let mut result = TritVector::from(trytes.len() * 3);

        let mut offset: usize = 0;
        for i in 0..trytes.len() {
            let index = TRYTES.chars().position(|c| c == trytes.chars().nth(i).unwrap()).unwrap();
            let trits: String = String::from(TRYTE_TRITS[index]);
            for j in 0..3 {
                result.vector.buffer[offset + j] = trits.chars().nth(j).unwrap();
            }
            offset += 3;
        }
        result
    }

    pub fn slice(&self, start: usize, length: usize) -> TritVector
    {
        if start + length > self.size() {
            eprintln!("Slice out of range ({}): {}:{}", self.size(), start, length);
        }

        let mut result: TritVector = TritVector::from(self);

        if start == 0 && length == self.size()
        {
            // slice the entire vector ie clone
            return result;
        }

        result.offset += start;
        result.size = length;
        if self.is_value() {
            result.value_trits = length;
            return result
        }
        if self.is_null() {
            return result
        }

        // have to count non-null trits
        for i in 0..result.size() {
            if result.trit(i) != '@' {
                result.value_trits += 1;
            }
        }
        result
    }


    pub fn slice_padded(&self, start: usize, length: usize) -> TritVector {
        // slices trit vector as if it was padded with infinite zeroes

        if start + length <= self.size() {
            // fits within range, normal slice
            return self.slice(start, length)
        }

        if start >= self.size() {
            // completely outside range, just zeroes
            return TritVector::new(length, '0')
        }

        let remain: usize = self.size() - start;
        let padded_zeros: TritVector = TritVector::new(length - remain, '0');
        TritVector::concat(Some(self.slice(start, length)), Some(padded_zeros))
    }

    // pub fn to_decimal(&self) -> String {
    //     String::from(TritConverter::to_decimal(self.trits()).as_str())
    // }

    pub fn to_trytes(&self) -> String {
        let mut buffer: Vec<char> = vec!['0'; (self.size() + 2) / 3];
        let mut start: usize = self.offset;
        let mut trits: Vec<char> = vec!['0'; 3];
        let trytes: usize = self.size() / 3;
        for i in 0..trytes {
            for j in 0..3 {
                trits[j] = self.vector.buffer[start + j];
            }
            buffer[i] = TritVector::tryte(trits.clone());
            start += 3;
        }

        if buffer.len() > trytes {
            // do remaining 1 or 2 trits
            trits[1] = '0';
            trits[2] = '0';
            let end: usize  = trytes * 3;
            for i in end..self.size() {
                trits[i - end] = self.vector.buffer[self.offset + i];
            }
            buffer[trytes] = TritVector::tryte(trits);
        }
        buffer.into_iter().collect()
    }
}

impl PartialEq for TritVector {
    fn eq(&self, other: &Self) -> bool {

        if self.size != other.size {
            return false
        }

        if self.vector != other.vector {
            return false
        }

        true
    }
}