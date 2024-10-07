use std::str::FromStr;

pub enum License {
    Apache20,
}

impl FromStr for License {
    type Err = ();

    fn from_str(license: &str) -> Result<Self, Self::Err> {
        match license {
            "Apache-2.0" => Ok(License::Apache20),
            _ => Err(()),
        }
    }
}

impl License {
    pub fn to_str(&self) -> &str {
        match self {
            License::Apache20 => "Apache-2.0",
        }
    }
}
