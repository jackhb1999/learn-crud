use sea_orm::{ActiveValue, DeriveActiveEnum, EnumIter, IntoActiveValue};
use serde::{Deserialize, Serialize};
use sea_orm::prelude::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize,EnumIter,DeriveActiveEnum)]
#[serde(rename_all = "lowercase")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)",rename_all = "lowercase")]
pub enum Gender{
    // #[sea_orm(string_value="male")]
    Male,
    // #[sea_orm(string_value="female")]
    Female,
}

impl IntoActiveValue<Gender> for Gender{
    fn into_active_value(self) -> ActiveValue<Gender> {
        ActiveValue::Set(self)
    }
}