#[derive(Debug)]
pub struct BasiliskkErr {
    pub desc: String,
}

impl std::fmt::Display for BasiliskkErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl From<std::io::Error> for BasiliskkErr {
    fn from(value: std::io::Error) -> Self {
        Self {
            desc: value.to_string(),
        }
    }
}

impl std::error::Error for BasiliskkErr {}
