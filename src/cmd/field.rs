// TODO implement without pico-args
use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;
use chrono::{DateTime, Utc};
use parse_duration::parse;
use uom::{ Kind,
    si::{self, length, time, Unit, Units},
    str::ParseQuantityError,
};
use crate::util::Either;

#[derive(Debug,Clone)]
pub struct Field {
    pub key: String,
    pub val: String,
    pub kind: Option<FieldKind>,
    pub units: Option<UnitKind>,
    created: DateTime<Utc>,
}

impl Field {
    fn with_kind(self, kind: FieldKind) -> Self {
        Self {
            kind: Some(kind), ..self
        }
    }

    pub fn prompt_units() {}

    pub fn check_units(self, key: String, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        if let Some(units) = args.subcommand()? {
            Self::printclr(format!("Field {:?} = {:?}, units: {:?}", self.key, self.val,  units), false, false, false);
            let _units = WrittenUOM::new_keyval(
                key.to_string(), units.to_string()
            );
        } else {
            Self::printclr(format!("Field {:?} = {:?}, units: {:?}", self.key, self.val,  key), false, false, false);
        }
        Ok(self)
    }
}

impl SubCommand for Field {

    fn cmd_string() -> Vec<&'static str> {
        vec!["field", "f", "-f", "--field"]
    }

    fn new(key: String, val: Option<String>) -> Self {
        Self {
            key,
            val: val.unwrap_or_default(),
            kind: Some(FieldKind::default()),
            created: Utc::now(),
            units: None,
        }
    }

    fn insert(&self) -> Result<(), pico_args::Error> {
        Ok(())
    }

    fn color() -> Color { Color::BrightBlue }

    fn with_args(mut key: Option<String>, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        if let Some(inner) = key.clone() {
            if Self::cmd_string().contains(&inner.as_str()) {
                key = args.subcommand()?;
            }
        }
        match (key, args.subcommand()?) {
            (Some(key), Some(val)) => {
                if Self::cmd_string().contains(&key.as_str()) {
                    return Self::with_args(Some(val.clone()), args);
                }
                if let Some(quantity) = args.subcommand()? {
                    let units = Field::new(
                        key.clone(),
                        Some(val.clone())
                    ).check_units(quantity, args)?;
                    let _units = units.units;
                }
                let field= Field::new(key.clone(), Some(val));
                field.insert()?;
                Ok(field)
            }
            (Some(key), None) => {
                if Self::cmd_string().contains(&key.as_str()) {
                    let nval = Self::new(key, None).prompt_value()?;
                    return Self::with_args(Some(nval), args);
                }
                 let val = Self::new(key.clone(), None).prompt_value()?;
                 let field = Field::new(key.clone(), Some(val));
                 field.insert()?;
                 Ok(field)
            }
            _ => Err(pico_args::Error::NonUtf8Argument)
        }
    }

}

impl Default for Field {
    fn default() -> Self {
        let key = Self::prompt_key().unwrap();
        let val = Self::new(key.clone(), None).prompt_value().unwrap();
        Self::new(key, Some(val))
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        "item".to_string()
    }
}

#[derive(Debug, Clone)]
pub enum FieldKind {
    String,
    Int,
    Float,
    Bool,
    Date,
}

impl Default for FieldKind {
    fn default() -> Self {
        Self::String
    }
}

#[derive(Debug, Clone)]
pub struct WrittenUOM {
    input: Option<String>,
    quantity: Option<String>,
    units: Option<String>,
}

impl WrittenUOM {

    pub fn new(input: String) -> Self {
        Self { input: Some(input), quantity: None, units: None }
    }

    pub fn new_keyval(key: String, val: String) -> Self {
        Self { input: None, quantity: Some(key), units: Some(val) }
    }

    pub fn split_digits_letters(self) -> Self {
        let (mut quant, mut unit)  =
            (String::new(), String::new());
        for ch in self.input.clone().unwrap().chars().into_iter() {
            if ch.is_digit(ch as u32) {
                quant.push(ch);
            } else if ch.is_alphabetic() {
                &unit.push(ch);
            } else {
                continue
            }
        }
        Self { quantity: Some(quant.clone()), units: Some(unit.clone()), ..self }
    }

    pub fn convert_to_uom(self) -> Option<Either<impl si::Unit, impl time::Unit>> {
        match self.units.unwrap().as_str() {
            "km" | "kilometers" => Some(Either::A(length::kilometer)),
            "day" | "d" => Some(Either::B(time::day)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnitKind {
    Distance(i32, String),
    Duration(i32, String),
    Instant(i32, String),
    Custom(i32, String, String),
    CustomVec(Vec<String>),
}

impl From<String> for UnitKind {
    fn from(input: String) -> Self {
        UnitKind::CustomVec(Vec::new())
    }
}

// TODO implement TO uom for Units
