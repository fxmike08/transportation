//! Main library entry point for transportation_api implementation.

use async_trait::async_trait;
use log::info;
use swagger::ApiError;
use swagger::{Has, XSpanIdString};
use transportation_api::models;
use transportation_api::{
    Api, GetAutomotiveResponse, GetPointDistancesResponse, GetRouteResponse,
    GetStationEventsResponse, UpdateAutomotiveResponse, UpdatePointDistancesResponse,
    UpdateRouteResponse,
};

use crate::db::Db;
use crate::errors::ServiceError;

#[derive(Copy, Clone)]
pub struct Server<'a> {
    db: &'a Db<'a>,
}

impl<'a> Server<'a> {
    pub fn new(db: &'static mut Db<'a>) -> Self {
        Server { db }
    }
}

macro_rules! ok_or_return {
    ($obj:expr, $fn_name:ident) => {
        match $obj {
            Err(s) => Ok($fn_name(s)),
            Ok(e) => Ok(e),
        }
    };
}

#[async_trait]
impl<'a, C> Api<C> for Server<'a>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// Update an existing automotive
    async fn update_automotive(
        &self,
        license: String,
        automotive: models::Automotive,
        context: &C,
    ) -> Result<UpdateAutomotiveResponse, ApiError> {
        let context = context.clone();
        info!(
            "update_automotive(\"{}\", {:?}) - X-Span-ID: {:?}",
            license,
            automotive,
            context.get().0.clone()
        );
        todo!("Unimplemented");
        // Err(ApiError("Generic failure".into()))
    }

    /// Update or add distances between two points/stations
    async fn update_point_distances(
        &self,
        a: String,
        b: String,
        distances: models::Distances,
        context: &C,
    ) -> Result<UpdatePointDistancesResponse, ApiError> {
        let context = context.clone();
        info!(
            "update_point_distances(\"{}\", \"{}\", {:?}) - X-Span-ID: {:?}",
            a,
            b,
            distances,
            context.get().0.clone()
        );
        todo!("Unimplemented");
        // Err(ApiError("Generic failure".into()))
    }

    /// Update an existing route
    async fn update_route(
        &self,
        id: String,
        route: models::Route,
        context: &C,
    ) -> Result<UpdateRouteResponse, ApiError> {
        let context = context.clone();
        info!(
            "update_route(\"{}\", {:?}) - X-Span-ID: {:?}",
            id,
            route,
            context.get().0.clone()
        );
        todo!("Unimplemented");
        // Err(ApiError("Generic failure".into()))
    }

    /// Get info about automotive
    async fn get_automotive(
        &self,
        license: String,
        context: &C,
    ) -> Result<GetAutomotiveResponse, ApiError> {
        let context = context.clone();
        info!(
            "get_automotive(\"{}\") - X-Span-ID: {:?}",
            license,
            context.get().0.clone()
        );
        todo!("Unimplemented");
        // Err(ApiError("Generic failure".into()))
    }

    /// Get distance between two points/stations
    async fn get_point_distances(
        &self,
        a: String,
        b: String,
        context: &C,
    ) -> Result<GetPointDistancesResponse, ApiError> {
        let context = context.clone();
        info!(
            "get_point_distances(\"{}\", \"{}\") - X-Span-ID: {:?}",
            a,
            b,
            context.get().0.clone()
        );
        todo!("Unimplemented");
        // Err(ApiError("Generic failure".into()))
    }

    /// Get an existing route by Id
    async fn get_route(&self, id: String, context: &C) -> Result<GetRouteResponse, ApiError> {
        let context = context.clone();
        info!(
            "get_route(\"{}\") - X-Span-ID: {:?}",
            id.clone(),
            context.get().0.clone()
        );
        let result = self.db.get_route(id.clone(), context.get());
        info!(
            "get_route(\"{}\") - X-Span-ID: {:?} \n Response: {:?}",
            id,
            context.get().0.clone(),
            serde_json::to_string(&result)
        );
        ok_or_return!(result, convert_get_route_error)
    }

    /// Get station upcoming events
    async fn get_station_events(
        &self,
        station: String,
        context: &C,
    ) -> Result<GetStationEventsResponse, ApiError> {
        let context = context.clone();
        info!(
            "get_station_events(\"{}\") - X-Span-ID: {:?}",
            station,
            context.get().0.clone()
        );
        todo!("Unimplemented");
        // Err(ApiError("Generic failure".into()))
    }
}

fn convert_get_route_error(e: ServiceError) -> GetRouteResponse {
    match e {
        ServiceError::NotFound => GetRouteResponse::RouteNotFound(e.into()),
        _ => GetRouteResponse::ServiceProblem(e.into()),
    }
}
