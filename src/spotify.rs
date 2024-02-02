use color_eyre::Result;
use rspotify::{
    model::{Modality, TrackId},
    prelude::BaseClient,
    ClientCredsSpotify, Credentials,
};

use crate::track::Track;

pub struct Spotify {
    spotify: ClientCredsSpotify,
}

impl Spotify {
    pub fn login(creds: Credentials) -> Result<Self> {
        let spotify = ClientCredsSpotify::new(creds);
        spotify.request_token()?;
        Ok(Self { spotify })
    }

    pub fn fetch_track(&self, id: TrackId, raw_id: [u8; 16]) -> Result<Track> {
        let fulltrack = self.spotify.track(id.clone(), None)?;
        let features = self.spotify.track_features(id)?;

        let name = fulltrack.name;
        let album = fulltrack.album.name;
        let release_date = fulltrack.album.release_date;
        let duration = i32::try_from(fulltrack.duration.num_seconds())?;
        let explicit = fulltrack.explicit;

        let mode = match features.mode {
            Modality::Major => 1,
            Modality::Minor => 2,
            Modality::NoResult => 0,
        };

        let mut cover_images = fulltrack.album.images;
        cover_images.sort_unstable_by_key(|i| i.height);
        let album_cover = cover_images.pop().map(|i| i.url);

        let mut artists = String::new();
        let last_artist = fulltrack.artists.len() - 1;
        for (i, artist) in fulltrack.artists.into_iter().enumerate() {
            artists.push_str(&artist.name);
            if i < last_artist {
                artists.push_str(", ");
            }
        }

        Ok(Track {
            id: raw_id,
            name,
            artists,
            album,
            album_cover,
            release_date,
            duration,
            explicit,
            danceability: features.danceability,
            acousticness: features.acousticness,
            energy: features.energy,
            instrumentalness: features.instrumentalness,
            key: features.key,
            mode,
            liveness: features.liveness,
            loudness: features.loudness,
            speechiness: features.speechiness,
            tempo: features.tempo,
            valence: features.valence,
            time_signature: features.time_signature,
        })
    }
}
