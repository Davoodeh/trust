//! A OnceCell like boolean which once set to one value and not to the other again.

use std::ops::{Deref, Not};

/// Boolean which can only be set to true.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct RiseOnlyBool(bool);

impl RiseOnlyBool {
    /// Create a new instance (if given true, there is no way to toggle back).
    #[allow(dead_code)]
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    /// Try to set the memory and if already true, raise false.
    pub fn rise_if(&mut self, value: bool) -> bool {
        if !self.0 && value {
            self.0 = value;
        }
        self.0
    }

    /// Get the value of this boolean.
    pub fn value(&self) -> bool {
        self.0
    }
}

impl Not for RiseOnlyBool {
    type Output = bool;

    fn not(self) -> Self::Output {
        !self.value()
    }
}

impl Deref for RiseOnlyBool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rise_new() {
        assert_eq!(RiseOnlyBool::new(true).deref(), &true);
        assert_eq!(RiseOnlyBool::new(false).deref(), &false);
    }

    #[test]
    fn test_rise_if_return() {
        assert_eq!(RiseOnlyBool::new(true).rise_if(false), true);
        assert_eq!(RiseOnlyBool::new(false).rise_if(false), false);
        assert_eq!(RiseOnlyBool::new(true).rise_if(true), true);
        assert_eq!(RiseOnlyBool::new(false).rise_if(true), true);
        let mut v = RiseOnlyBool::new(false);
        v.rise_if(true);
        assert_eq!(*v, true);
    }
}
