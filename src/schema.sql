create table if not exists tracks(
    id blob primary key unique,
    name text not null,
    artists text not null,
    album text not null,
    album_cover text,
    release_date text,
    duration integer not null,
    explicit bool not null,
    danceability real not null,
    acousticness real not null,
    energy real not null,
    instrumentalness real not null,
    key integer not null,
    mode integer not null,
    liveness real not null,
    loudness real not null,
    speechiness real not null,
    tempo real not null,
    valence real not null,
    time_signature integer not null
);