// @generated automatically by Diesel CLI.

diesel::table! {
    auto (license) {
        license -> Text,
        car_type -> Text,
        location -> Text,
        state -> Text,
        route -> Nullable<Integer>,
        departure_time -> Integer,
        delay -> Integer,
    }
}

diesel::table! {
    distance (id) {
        id -> Integer,
        a -> Integer,
        b -> Integer,
        dist -> Integer,
    }
}

diesel::table! {
    route (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    route_stations (id) {
        id -> Integer,
        route -> Integer,
        station -> Integer,
    }
}

diesel::table! {
    station (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(auto -> route (route));
diesel::joinable!(route_stations -> route (route));
diesel::joinable!(route_stations -> station (station));

diesel::allow_tables_to_appear_in_same_query!(
    auto,
    distance,
    route,
    route_stations,
    station,
);
