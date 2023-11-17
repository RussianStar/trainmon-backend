-- Add migration script here
-- Up
CREATE TABLE workouts (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    duration INTERVAL NOT NULL,
    sport TEXT NOT NULL,
    distance NUMERIC NOT NULL,
    tss NUMERIC NOT NULL
);

CREATE TABLE heart_rate_data (
    workout_id UUID REFERENCES workouts(id),
    average NUMERIC NOT NULL,
    time_in_zone INTERVAL NOT NULL,
    average_effective NUMERIC NOT NULL,
    time_in_zone_effective INTERVAL NOT NULL
);

CREATE TABLE power_data (
    workout_id UUID REFERENCES workouts(id),
    average NUMERIC NOT NULL,
    weighted_average NUMERIC NOT NULL,
    normalized NUMERIC NOT NULL,
    time_in_zone INTERVAL NOT NULL,
    time_in_zone_effective INTERVAL NOT NULL
);

-- Down
DROP TABLE power_data;
DROP TABLE heart_rate_data;
DROP TABLE workouts;