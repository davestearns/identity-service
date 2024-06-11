use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug)]
pub enum ID {
    Account,
}

impl ID {
    pub fn create(&self) -> String {
        format!(
            "{}_{}",
            self.to_string().to_ascii_lowercase(),
            Uuid::new_v4()
                .to_string()
                .replace('-', "")
                .to_ascii_lowercase()
        )
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
