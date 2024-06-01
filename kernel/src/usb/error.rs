use core::error;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};

pub enum PciError {
    Full
}


impl Debug for PciError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Display for PciError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.description())
    }
}

impl error::Error for PciError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl PciError {
    fn description(&self) -> &'static str {
        match self { PciError::Full => "PCI Device is Full" }
    }
}
