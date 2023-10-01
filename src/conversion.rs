//! Conversions from DB struct to API struct

use transportation_api::models::RouteListInner;
use transportation_api::GetRouteResponse::SuccessfulOperation;
use transportation_api::{models, GetRouteResponse};

use crate::model::model_db::{Route, Station};

/// Create `GetRouteResponse` object for getRoute response
pub fn convert_to_route_result(route: Route, stations: Vec<Station>) -> GetRouteResponse {
    SuccessfulOperation(models::Route {
        id: Some(route.name),
        list: stations
            .iter()
            .map(|s| RouteListInner {
                name: Some(s.name.clone()),
            })
            .collect(),
    })
}
