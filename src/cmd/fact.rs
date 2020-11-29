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
pub struct Fact {
    pub key: String,
    pub val: Option<String>,
    pub kind: Option<FactKind>,
    pub units: Option<String>, //Eventually store in enum datatype
    pub created: DateTime<Utc>,
}

impl Fact {
    fn with_kind(self, kind: FactKind) -> Self {
        Self {
            kind: Some(kind), ..self
        }
    }

    pub fn prompt_units() {}

    pub fn check_units(self, mut key: String, args: &mut Arguments)
    -> Result<Self, pico_args::Error> {
        if let Some(units) = args.subcommand()? { // units with a space btwen
            let _units = WrittenUOM::new_keyval(
                key.to_string(), units.to_string()
            );
            key.extend(units.chars()); //for now, just combine
            Ok(Self { units: Some(key), ..self })
        } else { //units just like 1 hr
            Ok(Self { units: Some(key.to_string()), ..self })
        }
        //TODO properly handle single nospace: 1hr
    }
}

impl SubCommand for Fact {

    fn cmd_string() -> Vec<&'static str> {
        vec!["Fact", "f", "-f", "--Fact"]
    }

    fn new(key: String, val: Option<String>) -> Self {
        Self {
            key, val,
            kind: Some(FactKind::default()),
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
                let fact = if let Some(quantity) = args.subcommand()? {
                    let units = Fact::new(
                        key.clone(),
                        Some(val.clone())
                    ).check_units(quantity.clone(), args)?;
                    units
                } else {
                    Fact::new(key.clone(), Some(val))
                };
                Self::printclr(format!("Fact {} = {:?}, units: {:?} @ {}",
                    fact.key,
                    fact.val.clone().unwrap(),
                    fact.units,
                    fact.created.to_rfc2822()),
                false, false, false);
                fact.insert()?;
                Ok(fact)
            }
            (Some(key), None) => {
                if Self::cmd_string().contains(&key.as_str()) {
                    let nval = Self::new(key, None).prompt_value()?;
                    return Self::with_args(Some(nval), args);
                }
                Self::printclr(format!("Fact {}", key), false,false,false);
                 let val = Self::new(key.clone(), None).prompt_value()?;
                 let fact = Fact::new(key.clone(), Some(val));
                 fact.insert()?;
                 Ok(fact)
            }
            _ => Err(pico_args::Error::NonUtf8Argument)
        }
    }

}

impl Default for Fact {
    fn default() -> Self {
        let key = Self::prompt_key().unwrap();
        let val = Self::new(key.clone(), None).prompt_value().unwrap();
        Self::new(key, Some(val))
    }
}

impl ToString for Fact {
    fn to_string(&self) -> String {
        "item".to_string()
    }
}

#[derive(Debug, Clone)]
pub enum FactKind {
    String,
    Int,
    Float,
    Bool,
    Date,
}

impl Default for FactKind {
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
