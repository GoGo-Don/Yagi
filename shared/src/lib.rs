use serde::{Deserialize, Serialize};
use tracing::{debug, info, trace, warn};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Breed {
    Beetal,
    Jamunapari,
    Barbari,
    Sirohi,
    Osmanabadi,
    BlackBengal,
    Kutchi,
    Kaghani,
    Chegu,
    Jakhrana,
    Other(String),
}

impl Breed {
    /// Converts a database string to `Breed` enum, treating unknown values as `Other`.
    pub fn from_str(s: &str) -> Self {
        trace!("Parsing Breed from '{}'", s);
        match s {
            "Beetal" => Breed::Beetal,
            "Jamunapari" => Breed::Jamunapari,
            "Barbari" => Breed::Barbari,
            "Sirohi" => Breed::Sirohi,
            "Osmanabadi" => Breed::Osmanabadi,
            "BlackBengal" => Breed::BlackBengal,
            "Kutchi" => Breed::Kutchi,
            "Kaghani" => Breed::Kaghani,
            "Chegu" => Breed::Chegu,
            "Jakhrana" => Breed::Jakhrana,
            other => {
                debug!("Unknown Breed '{}', mapping to Other", other);
                Breed::Other(other.to_string())
            }
        }
    }

    /// Converts a `Breed` enum to a database string.
    pub fn to_str(breed: &Self) -> &str {
        match breed {
            Breed::Beetal => "Beetal",
            Breed::Jamunapari => "Jamunapari",
            Breed::Barbari => "Barbari",
            Breed::Sirohi => "Sirohi",
            Breed::Osmanabadi => "Osmanabadi",
            Breed::BlackBengal => "BlackBengal",
            Breed::Kutchi => "Kutchi",
            Breed::Kaghani => "Kaghani",
            Breed::Chegu => "Chegu",
            Breed::Jakhrana => "Jakhrana",
            Breed::Other(name) => name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    /// Converts a database string to `Gender` enum with detailed error reporting.
    pub fn from_str(s: &str) -> Result<Gender, String> {
        trace!("Parsing Gender from '{}'", s);
        match s {
            "Male" => Ok(Gender::Male),
            "Female" => Ok(Gender::Female),
            other => {
                debug!("Failed to parse Gender enum from '{}'", other);
                Err(format!("{}", other))
            }
        }
    }

    /// Converts a `Gender` enum to a database string.
    pub fn to_str(gender: &Gender) -> &str {
        match gender {
            Gender::Male => "Male",
            Gender::Female => "Female",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum Vaccine {
    Rabies,
    Cdt,
    Clostridium,
    FootAndMouth,
    Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum Disease {
    FootRot,
    Mastitis,
    Parasites,
    Pneumonia,
    Other(String),
}

// VaccineRf and DiseaseRef currently look the same.
// However, we can add more functionality like booster date for vaccine
// and symptoms for disease.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VaccineRef {
    pub id: Option<i64>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiseaseRef {
    pub id: Option<i64>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GoatParams {
    pub name: String,
    pub breed: Breed,
    pub gender: Gender,
    pub offspring: i32,
    pub cost: f64,
    pub weight: f64,
    pub current_price: f64,
    pub diet: String,
    pub last_bred: Option<String>,
    pub health_status: String,
    pub vaccinations: Vec<VaccineRef>,
    pub diseases: Vec<DiseaseRef>,
}
