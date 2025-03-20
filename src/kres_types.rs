use std::{fmt::{format, Debug}, str::FromStr};

use sqlx::{encode::IsNull, types::BigDecimal,Database, Encode, Type, Postgres};

pub struct HundredthTemperatureCelcius {
    pub value: BigDecimal
}

impl ToString for HundredthTemperatureCelcius {
    fn to_string(&self) -> String {
        format(format_args!("{:.2}°C", self.value))
    }
}
impl FromStr for HundredthTemperatureCelcius {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            return Err("Temperature string is too short".to_string());
        }
        if s.ends_with("°C") == false {
            return Err("Temperature string does not end with °C".to_string());
        }
        let index = s.find("°C");
        let index = match index {
            Some(i) => i,
            None => return Err("Temperature string does not contain °C".to_string())
        };

        let val_part = &s[..index];
        let value = BigDecimal::from_str(val_part).map_err(|err| format!("{} is not a valid decimal value: {}", val_part, err))?;
        Ok(HundredthTemperatureCelcius { value })
    }
}

impl Debug for HundredthTemperatureCelcius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{:.2}°C", self.value)
        self.to_string().fmt(f)
    }
}

impl From<BigDecimal> for HundredthTemperatureCelcius {
    fn from(value: BigDecimal) -> Self {
        HundredthTemperatureCelcius { value }
    }
}

// The implementation of the Encode and Type trait for the HundredthTemperatureCelcius
// enabled use to bind the Option<HundredthTemperatureCelcius> to the nullable numeric
// in postgres database.
impl<'r> Encode<'r, Postgres> for HundredthTemperatureCelcius {
    fn encode_by_ref(
            &self,
            buf: &mut <Postgres as Database>::ArgumentBuffer<'r>,
        ) -> Result<IsNull, sqlx::error::BoxDynError> {
            self.value.clone().encode(buf)
        }
}

impl Type<Postgres> for HundredthTemperatureCelcius {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <BigDecimal as Type<Postgres>>::type_info()
    }
}


// The below code doesn't work because of the orphan rule.
// impl From<Option<BigDecimal>> for Option<HundredthTemperatureCelcius> {
//     fn from(value: Option<BigDecimal>) -> Self {
//         match value {
//             Some(val) => Some(HundredthTemperatureCelcius::from(val)),
//             None => None,
//         }
//     }
// }

// So we need to wrap the type in a Newtype pattern to implement the conversion which is used
// in the sqlx::query_as! macro.
#[derive(Debug)]
pub struct Optional<T>(pub Option<T>);

//
// Conversion A from Option<BigDecimal> to Optional<HundredthTemperatureCelcius>
//
// impl From<Option<BigDecimal>> for Optional<HundredthTemperatureCelcius> {
//     fn from(opt: Option<BigDecimal>) -> Self {
//         match opt {
//             Some(val) => Optional(Some(HundredthTemperatureCelcius::from(val))),
//             None => Optional(None),
//         }
//     }
// }

// A more generic implementation for above conversion A
impl <A,B> From<Option<A>> for Optional<B> where B: From<A> {
    fn from(opt: Option<A>) -> Self {
        match opt {
            Some(val) => Optional(Some(B::from(val))),
            None => Optional(None),
        }
    }
}
