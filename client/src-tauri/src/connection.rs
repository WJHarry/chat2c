use std::fmt;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum ConnStatus {
    DISCONNECTED,
    INVITED,
    AGREED,
    ESTABLISHED
}

impl fmt::Display for ConnStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Connection {
    pub target_uid: String,
    pub target_addr: String,
    pub status: ConnStatus
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.target_uid, self.target_addr, self.status)
    }
}