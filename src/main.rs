use std::env;

use cache::Cache;
use color_eyre::{eyre::eyre, Result};
use color_spinner::ColorSpinner;
use colored::{Color, Colorize};
use config::LoadResult;
use paths::get_cachepath;
use rspotify::{prelude::Id, Credentials};
use spotify::Spotify;
use track::Track;

mod cache;
mod color_spinner;
mod config;
mod paths;
mod spotify;
mod track;

fn main() -> Result<()> {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    color_eyre::install()?;

    let config = match config::load()? {
        LoadResult::Opened(c) => c,
        LoadResult::Created(path) => {
            eprintln!(
                "{} {path}\n{}",
                "A new configuration file has been created in".bright_green(),
                "Adjust it and run spotk again.".bright_magenta()
            );
            return Ok(());
        }
    };

    let mut args = env::args().skip(1);

    let uri = args
        .next()
        .ok_or_else(|| eyre!("No Spotify URI/URL specified"))?;
    let track_id = spotify::parse_track_id(&uri)?;
    let raw_track_id = base62::decode(track_id.id())
        .map_err(|e| eyre!("Failed to decode the track ID: {e}"))?
        .to_be_bytes();

    let mut cache = Cache::open(get_cachepath()?.to_str().unwrap())?;

    if let Some(track) = cache.get(raw_track_id)? {
        print_track(&track)
    } else {
        eprintln!(
            "{}",
            "Track not in cache, fetching from the API...".bright_green()
        );
        let spotify = Spotify::login(Credentials::new(&config.api.id, &config.api.secret))?;
        let track = spotify.fetch_track(track_id, raw_track_id)?;
        print_track(&track);
        cache.set(track)?;
    }

    Ok(())
}

fn print_track(track: &Track) {
    let allowed_colors = &[
        Color::BrightBlue,
        Color::BrightCyan,
        Color::BrightGreen,
        Color::BrightMagenta,
        Color::BrightRed,
        Color::BrightYellow,
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

    eprintln!("{}: {}", "name".color(spinner.next()), track.name);
    eprintln!("{}: {}", "artists".color(spinner.next()), track.artists);
    eprintln!("{}: {}", "album".color(spinner.next()), track.album);
    eprintln!(
        "{}: {}",
        "album cover".color(spinner.next()),
        track.album_cover.as_deref().unwrap_or("none")
    );
    eprintln!(
        "{}: {}",
        "release date".color(spinner.next()),
        track.release_date.as_deref().unwrap_or("no idea")
    );
    eprintln!("{}: {dur_min}:{dur_sec}", "duration".color(spinner.next()));
    eprintln!("{}: {}", "explicit".color(spinner.next()), track.explicit);
    eprintln!(
        "{}: {}",
        "danceability".color(spinner.next()),
        track.danceability
    );
    eprintln!(
        "{}: {}",
        "acousticness".color(spinner.next()),
        track.acousticness
    );
    eprintln!("{}: {}", "energy".color(spinner.next()), track.energy);
    eprintln!(
        "{}: {}",
        "instrumentalness".color(spinner.next()),
        track.instrumentalness
    );
    eprintln!("{}: {}", "liveness".color(spinner.next()), track.liveness);
    eprintln!("{}: {}", "loudness".color(spinner.next()), track.loudness);
    eprintln!(
        "{}: {}",
        "speechiness".color(spinner.next()),
        track.speechiness
    );
    eprintln!("{}: {}", "tempo".color(spinner.next()), track.tempo);
    eprintln!("{}: {}", "valence".color(spinner.next()), track.valence);
    eprintln!(
        "{}: {}/4",
        "time signature".color(spinner.next()),
        track.time_signature
    );

    eprintln!("{}: {key}", "key".color(spinner.next()));
    eprintln!("{}: {mode}", "mode".color(spinner.next()));
}
