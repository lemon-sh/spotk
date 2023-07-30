use std::env;

use cache::Cache;
use color_eyre::{eyre::eyre, Result};
use color_spinner::ColorSpinner;
use config::LoadResult;
use owo_colors::{AnsiColors, OwoColorize, Stream::Stdout};
use paths::get_cachepath;
use rspotify::{model::TrackId, prelude::Id, Credentials};
use spotify::Spotify;
use track::Track;

mod cache;
mod color_spinner;
mod config;
mod paths;
mod spotify;
mod track;

fn main() -> Result<()> {
    let config = match config::load()? {
        LoadResult::Opened(c) => c,
        LoadResult::Created(path) => {
            eprintln!(
                "{} {path}\n{}",
                "A new configuration file has been created in"
                    .if_supports_color(Stdout, OwoColorize::bright_green),
                "Adjust it and run spotk again."
                    .if_supports_color(Stdout, OwoColorize::bright_magenta)
            );
            return Ok(());
        }
    };

    let mut args: Vec<String> = env::args().skip(1).collect();

    let mut flags: Vec<char> = Vec::new();
    args.retain(|a| {
        let mut chars = a.chars();
        if a.len() != 2 || chars.next() != Some('-') {
            true
        } else {
            flags.push(chars.next().unwrap());
            false
        }
    });

    let uri = args
        .pop()
        .ok_or_else(|| eyre!("No Spotify URI specified"))?;
    let track_id = TrackId::from_uri(&uri)?;
    let raw_track_id = base62::decode(track_id.id())
        .map_err(|e| eyre!("Failed to decode the track ID: {e}"))?
        .to_be_bytes();

    let mut cache = Cache::open(get_cachepath()?.to_str().unwrap())?;

    if let Some(track) = cache.get(raw_track_id)? {
        print_track(&track)
    } else {
        eprintln!("{}", "Track not in cache, fetching from the API...".if_supports_color(Stdout, OwoColorize::bright_green));
        let spotify = Spotify::login(Credentials::new(&config.api.id, &config.api.secret))?;
        let track = spotify.fetch_track(track_id, raw_track_id)?;
        print_track(&track);
        cache.set(track)?;
    }

    Ok(())
}

fn print_track(track: &Track) {
    let allowed_colors = &[
        AnsiColors::BrightBlue,
        AnsiColors::BrightCyan,
        AnsiColors::BrightGreen,
        AnsiColors::BrightMagenta,
        AnsiColors::BrightRed,
        AnsiColors::BrightYellow,
    ];
    let spinner = ColorSpinner::new(allowed_colors);

    let mut dur_sec = track.duration;
    let dur_min = dur_sec / 60;
    dur_sec -= dur_min * 60;

    let key = match track.key {
        0 => "C",
        1 => "C#/Db",
        2 => "D",
        3 => "D#/Eb",
        4 => "E",
        5 => "F",
        6 => "F#/Gb",
        7 => "G",
        8 => "G#/Ab",
        9 => "A",
        10 => "A#/Bb",
        11 => "B",
        _ => "no idea",
    };

    let mode = match track.mode {
        1 => "Major",
        2 => "minor",
        _ => "no idea",
    };

    eprintln!(
        "{}: {}",
        "name".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.name
    );
    eprintln!(
        "{}: {}",
        "artists".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.artists
    );
    eprintln!(
        "{}: {}",
        "album".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.album
    );
    eprintln!(
        "{}: {}",
        "album cover".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.album_cover.as_deref().unwrap_or("none")
    );
    eprintln!(
        "{}: {}",
        "release date".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.release_date.as_deref().unwrap_or("no idea")
    );
    eprintln!(
        "{}: {dur_min}:{dur_sec}",
        "duration".if_supports_color(Stdout, |v| v.color(spinner.next()))
    );
    eprintln!(
        "{}: {}",
        "explicit".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.explicit
    );
    eprintln!(
        "{}: {}",
        "danceability".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.danceability
    );
    eprintln!(
        "{}: {}",
        "acousticness".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.acousticness
    );
    eprintln!(
        "{}: {}",
        "energy".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.energy
    );
    eprintln!(
        "{}: {}",
        "instrumentalness".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.instrumentalness
    );
    eprintln!(
        "{}: {}",
        "liveness".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.liveness
    );
    eprintln!(
        "{}: {}",
        "loudness".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.loudness
    );
    eprintln!(
        "{}: {}",
        "speechiness".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.speechiness
    );
    eprintln!(
        "{}: {}",
        "tempo".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.tempo
    );
    eprintln!(
        "{}: {}",
        "valence".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.valence
    );
    eprintln!(
        "{}: {}/4",
        "time signature".if_supports_color(Stdout, |v| v.color(spinner.next())),
        track.time_signature
    );

    eprintln!(
        "{}: {key}",
        "key".if_supports_color(Stdout, |v| v.color(spinner.next()))
    );
    eprintln!(
        "{}: {mode}",
        "mode".if_supports_color(Stdout, |v| v.color(spinner.next()))
    );
}
