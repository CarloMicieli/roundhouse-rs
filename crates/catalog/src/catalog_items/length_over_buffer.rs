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
}

#[cfg(test)]
mod tests {
    use super::*;

    mod lengths_over_buffer {
        use super::*;
    }
}
