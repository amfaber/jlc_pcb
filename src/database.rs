use std::{env, time::Instant};

use diesel::{
    pg::PgConnection, upsert::excluded, Connection, ExpressionMethods,
    QueryDsl, RunQueryDsl, SelectableHelper,
};
#[allow(unused)]
use diesel_full_text_search::{to_tsquery, to_tsvector, TsVectorExtensions};
use dotenvy::dotenv;
use tokio::sync::mpsc::Receiver;

use crate::models::{ComponentInfo, ComponentInfoWithTime};

pub fn establish_connection() -> PgConnection {
    dotenv().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
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

pub fn get_components() -> Vec<ComponentInfoWithTime> {
    use crate::schema::components::dsl::*;
    let mut connection = establish_connection();
    let now = Instant::now();
    // let search = to_tsquery("trans:*");
    let out = components
        // .filter(to_tsvector(description).matches(search))
        // .filter(lcsc_part.eq("C15749"))
        .select((ComponentInfo::as_select(), created, updated))
        .load(&mut connection)
        .unwrap();
    dbg!(now.elapsed());
    out
}
