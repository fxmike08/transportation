use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use log::{debug, info};
use swagger::XSpanIdString;
use transportation_api::GetRouteResponse;

use crate::conversion::convert_to_route_result;
use crate::errors::ServiceError;
use crate::model::model_db::{Route, RouteStation, Station};
use crate::schema::route::dsl::route as RouteTable;
use crate::schema::route_stations::dsl::route_stations as RouteStationTable;
use crate::schema::station::dsl::station as StationTable;
use crate::schema::*;

#[derive(Copy, Clone)]
pub struct Db<'a> {
    conn: &'a Pool<ConnectionManager<SqliteConnection>>,
}

impl<'a> Db<'a> {
    pub fn new(database_pool: &'a Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Db {
            conn: database_pool,
        }
    }

    pub(crate) fn get_route(
        &self,
        id: String,
        x_span_id: &XSpanIdString,
    ) -> Result<GetRouteResponse, ServiceError> {
        let mut connection = self
            .conn
            .get()
            .expect("Unable to obtain a DB connection from pool");

        RouteTable
            .limit(1)
            .filter(route::name.eq(id.as_str()))
            .load::<Route>(&mut connection)
            .and_then(|r|
                if r.clone().capacity() == 1 {
                    let res = r[0].clone();
                    Ok(res)
                } else{
                    info!("get_route(\"{}\") - X-Span-ID: {:?} : Unable to find specified route by name: {} ", id, x_span_id, id);
                    Err(Error::NotFound)
                }
            )
            .and_then(|r| {
                RouteStationTable
                            .inner_join(StationTable)
                            .filter(route_stations::route.eq(r.id))
                            .load::<(RouteStation,Station)>(&mut connection)
                            .and_then(|s| {
                                let stations = s.into_iter().map(|(_,b)| b).collect();
                                Ok(convert_to_route_result(r.clone(), stations))
                            })
            })
            .map_err(|e| convert_to_error(e, x_span_id))
    }
}

/// Convert database error to ServiceError
fn convert_to_error(e: Error, x_span_id: &XSpanIdString) -> ServiceError {
    debug!(
        "convert_to_error(\"{:?}\") - X-Span-ID: {:?} : Converting to Service Error ",
        e, x_span_id
    );
    match e {
        Error::NotFound => ServiceError::NotFound,
        _ => ServiceError::InternalError,
    }
}
