use std::env;

use diesel::{
    pg::PgConnection, upsert::excluded, Connection, ExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper, TextExpressionMethods, sql_types::{SingleValue, SqlType}, Expression, expression::{ValidGrouping, MixedAggregates},
};
use diesel_full_text_search::{
    configuration::{TsConfiguration, TsConfigurationByName},
    to_tsquery_with_search_config, to_tsvector_with_search_config, TextOrNullableText,
};
#[allow(unused)]
use diesel_full_text_search::{to_tsquery, to_tsvector, TsVectorExtensions};
use diesel_logger::LoggingConnection;
use dotenvy::dotenv;
use tokio::sync::mpsc::Receiver;

use crate::models::{ComponentInfo, ComponentInfoWithTime};

pub fn establish_connection() -> LoggingConnection<PgConnection> {
    dotenv().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    LoggingConnection::new(PgConnection::establish(&database_url).unwrap())
}

pub async fn insert_components(mut receiver: Receiver<Vec<ComponentInfo>>) {
    use crate::schema::components::dsl::*;
    let mut connection = establish_connection();
    // let mut n = 0;
    loop {
        match receiver.recv().await {
            Some(component_vec) => {
                let n = diesel::insert_into(crate::schema::components::table)
                    .values(&component_vec)
                    .on_conflict(lcsc_part)
                    .do_update()
                    .set((
                        // Duplicating the whole scheme again. Sigh
                        first_category.eq(excluded(first_category)),
                        second_category.eq(excluded(second_category)),
                        mfr_part.eq(excluded(mfr_part)),
                        solder_joint.eq(excluded(solder_joint)),
                        manufacturer.eq(excluded(manufacturer)),
                        library_type.eq(excluded(library_type)),
                        description.eq(excluded(description)),
                        datasheet.eq(excluded(datasheet)),
                        price.eq(excluded(price)),
                        stock.eq(excluded(stock)),
                        package.eq(excluded(package)),
                        api_last_key.eq(excluded(api_last_key)),
                        // created.eq(excluded(created)), Explicitly don't update "created"
                        // updated.eq(excluded(updated)),
                    ))
                    .execute(&mut connection)
                    .unwrap();
                dbg!(n);
            }
            None => break,
        }
    }
}

pub fn last_api_key() -> Option<String> {
    use crate::schema::components::dsl::*;
    let mut connection = establish_connection();
    let _ = dbg!(components.count().get_result::<i64>(&mut connection));
    components
        .order(created.desc())
        .select(api_last_key)
        .first(&mut connection)
        .ok()
        .flatten()
}

pub fn get_components(search_term: Option<&str>) -> Vec<ComponentInfoWithTime> 
{
    use crate::schema::components::dsl::*;
    let mut connection = establish_connection();
    if let Some(search) = search_term{
        let field = to_tsvector_with_search_config(TsConfigurationByName("english"), description);
        let search = to_tsquery_with_search_config(TsConfigurationByName("english"), search);
        components
            .filter(field.matches(search))
            .select((ComponentInfo::as_select(), created, updated))
            .load(&mut connection)
            .unwrap()
    } else {
        components
            .select((ComponentInfo::as_select(), created, updated))
            .load(&mut connection)
            .unwrap()
    }
}
