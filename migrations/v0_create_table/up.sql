-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE IF NOT EXISTS station (
                                       id INTEGER PRIMARY KEY NOT NULL,
                                       name TEXT NOT NULL
) ;


CREATE TABLE IF NOT EXISTS route(
                                    id INTEGER  PRIMARY KEY NOT NULL,
                                    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS route_stations(
                                            id INTEGER  PRIMARY KEY NOT NULL,
                                            route INTEGER NOT NULL,
                                            station INTEGER NOT NULL,
                                            FOREIGN KEY(station) REFERENCES station(id),
                                            FOREIGN KEY(route) REFERENCES route(id)
    );

CREATE TABLE IF NOT EXISTS distance(
                                       id INTEGER  PRIMARY KEY NOT NULL,
                                       a INTEGER NOT NULL,
                                       b INTEGER NOT NULL,
                                       dist INTEGER NOT NULL,
                                       FOREIGN KEY(a) REFERENCES station(id),
    FOREIGN KEY(b) REFERENCES station(id)
    );

CREATE TABLE IF NOT EXISTS auto(
                                   license TEXT PRIMARY KEY NOT NULL,
                                   car_type TEXT NOT NULL,
                                   location TEXT NOT NULL,
                                   state TEXT NOT NULL,
                                   route INTEGER,
                                   departure_time INTEGER NOT NULL,
                                   delay INTEGER NOT NULL,
                                   FOREIGN KEY(route) REFERENCES route(id)
    );