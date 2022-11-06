/// The length over buffer for the model.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct LengthOverBuffer(u32);

impl LengthOverBuffer {
    /// Creates a new value, the provided value must be positive.
    pub fn new(value: u32) -> Self {
        if value <= 0 {
            panic!("Length over buffer cannot be 0 or negative");
        }
        LengthOverBuffer(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod lengths_over_buffer {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_length_over_buffer() {
            let len = LengthOverBuffer::new(303);
            assert_eq!(303, len.value());
        }
    }
}
