use crate::schema::auto as AutoTable;
use crate::schema::route as RouteTable;
use crate::schema::route_stations as RouteStationTable;
use crate::schema::station as StationTable;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = AutoTable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Auto {
    pub license: String,
    pub car_type: String,
    pub location: String,
    pub state: String,
    #[diesel(sql_type = Nullable<Route>)]
    pub route: Option<i32>,
    pub departure_time: i32,
    pub delay: i32,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = RouteTable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Route {
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = RouteStationTable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RouteStation {
    pub id: i32,
    pub route: i32,
    pub station: i32,
}
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = StationTable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Station {
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Insertable)]
#[diesel(table_name = StationTable)]
pub struct NewStation {
    pub name: String,
}
// #[derive(Queryable, Selectable, Insertable)]
// #[diesel(table_name = Distance)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
// pub struct Distance{
//     pub id: i32,
//     pub a: Option<Station>,
//     pub b: Option<Station>,
//     pub distance: i32
// }
//
