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
    workout_id UUID REFERENCES workouts(id) ON DELETE CASCADE,
    average NUMERIC NOT NULL,
    time_in_zone NUMERIC[] NOT NULL
    average_effective NUMERIC NOT NULL,
    time_in_zone_effective NUMERIC[] NOT NULL
);

CREATE TABLE power_data (
    workout_id UUID REFERENCES workouts(id) ON DELETE CASCADE,
    average NUMERIC NOT NULL,
    weighted_average NUMERIC NOT NULL,
    normalized NUMERIC NOT NULL,
    time_in_zone NUMERIC[] NOT NULL,
    time_in_zone_effective NUMERIC[] NOT NULL
);

CREATE TABLE metrics (
    metric_id UUID ,
    user_id UUID REFERENCES workouts(id) ON DELETE CASCADE,
    provider VARCHAR NOT NULL,
    time TIMESTAMP NOT NULL,
    weight NUMERIC NOT NULL,
    sleep_duration NUMERIC NOT NULL,
    resting_heart_rate NUMERIC NOT NULL,
    hrv NUMERIC NOT NULL
);


