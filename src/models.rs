use diesel::{prelude::{Queryable, Insertable}, Selectable, query_builder::AsChangeset, data_types::PgTimestamp};
use serde::Deserialize;


#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::components)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct ComponentInfo {
    pub lcsc_part: String,
    pub first_category: Option<String>,
    pub second_category: Option<String>,
    pub mfr_part: Option<String>,
    pub solder_joint: Option<String>,
    pub manufacturer: Option<String>,
    pub library_type: Option<String>,
    pub description: Option<String>,
    pub datasheet: Option<String>,
    pub price: Option<String>,
    pub stock: Option<i32>,
    pub package: Option<String>,
    pub api_last_key: Option<String>,
}


#[derive(Debug, PartialEq, Eq, Queryable)]
#[diesel(table_name = crate::schema::components)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComponentInfoWithTime {
    pub component_info: ComponentInfo,
    pub created: PgTimestamp,
    pub updated: PgTimestamp,
}


