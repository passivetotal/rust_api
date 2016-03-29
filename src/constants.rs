pub enum Source {
    RISKIQ,
    PINGLY,
    DNSRES,
    KASPERSKY,
}

impl Source {
    pub fn string(&self) -> String {
        match *self {
            Source::RISKIQ => "riskiq".to_string(),
            Source::PINGLY => "pingly".to_string(),
            Source::DNSRES => "dnsres".to_string(),
            Source::KASPERSKY => "kaspersky".to_string(),
        }
    }
}
