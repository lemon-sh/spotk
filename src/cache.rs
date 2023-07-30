use rusqlite::{params, Connection, OptionalExtension};

use crate::track::Track;

pub struct Cache(Connection);

impl Cache {
    pub fn open(path: &str) -> rusqlite::Result<Self> {
        let db = Connection::open(path)?;
        db.execute_batch(include_str!("schema.sql"))?;
        Ok(Self(db))
    }

    pub fn get(&mut self, id: [u8; 16]) -> rusqlite::Result<Option<Track>> {
        let mut stmt = self.0.prepare("select * from tracks where id = ?")?;
        stmt.query_row([id], |t| {
            Ok(Track {
                id: t.get(0)?,
                name: t.get(1)?,
                artists: t.get(2)?,
                album: t.get(3)?,
                album_cover: t.get(4)?,
                release_date: t.get(5)?,
                duration: t.get(6)?,
                explicit: t.get(7)?,
                danceability: t.get(8)?,
                acousticness: t.get(9)?,
                energy: t.get(10)?,
                instrumentalness: t.get(11)?,
                key: t.get(12)?,
                mode: t.get(13)?,
                liveness: t.get(14)?,
                loudness: t.get(15)?,
                speechiness: t.get(16)?,
                tempo: t.get(17)?,
                valence: t.get(18)?,
                time_signature: t.get(19)?,
            })
        })
        .optional()
    }

    pub fn set(&mut self, t: Track) -> rusqlite::Result<()> {
        let mut stmt = self.0.prepare("insert into tracks(id,name,artists,album,album_cover,release_date,duration,explicit,danceability,acousticness,energy,instrumentalness,key,mode,liveness,loudness,speechiness,tempo,valence,time_signature values (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)")?;
        stmt.execute(params![
            t.id,
            t.name,
            t.artists,
            t.album,
            t.album_cover,
            t.release_date,
            t.duration,
            t.explicit,
            t.danceability,
            t.acousticness,
            t.energy,
            t.instrumentalness,
            t.key,
            t.mode,
            t.liveness,
            t.loudness,
            t.speechiness,
            t.tempo,
            t.valence,
            t.time_signature
        ])?;
        Ok(())
    }
}
