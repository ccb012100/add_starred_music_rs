use chrono::{DateTime, Datelike, Utc};
use std::{
    fmt,
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug)]
pub(crate) struct AlbumString(pub(crate) String);

#[derive(Debug)]
pub(crate) struct DateAdded(DateTime<Utc>);

#[derive(Debug)]
pub(crate) struct ReleaseYear(pub(crate) i32);

#[derive(Debug)]
pub(crate) struct TrackCount(pub(crate) u16);

#[derive(Debug)]
pub(crate) struct Album {
    name: AlbumString,
    artist: AlbumString,
    tracks: TrackCount,
    release_year: ReleaseYear,
    date_added: DateAdded,
}

impl fmt::Display for DateAdded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M:%S"))
    }
}

impl fmt::Display for AlbumString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TrackCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ReleaseYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"Album
{{
    name:       {}
    artist:     {}
    tracks:     {}
    released:   {}
    added:      {}
}}"#,
            self.name, self.artist, self.tracks, self.release_year, self.date_added
        )
    }
}

impl FromStr for TrackCount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<u16>() {
            Ok(tc) => {
                if tc > 750 || tc == 0 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Track Count must be between 1 and 750",
                    ));
                }

                Ok(TrackCount(tc))
            }
            Err(err) => Err(Error::new(ErrorKind::InvalidData, err)),
        }
    }
}

impl FromStr for AlbumString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "AlbumString cannot be empty",
            ));
        }

        let trimmed = s.trim().to_string();

        if trimmed.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "AlbumString cannot be white space only",
            ));
        }

        Ok(AlbumString(trimmed))
    }
}

impl FromStr for ReleaseYear {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<i32>() {
            Ok(ry) => {
                // 1928 is arbitrary, but should encompass almost anything I'll add
                let min_year: i32 = 1928;
                let current_year: i32 = Utc::now().year();

                if !(min_year..=current_year).contains(&ry) {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Release year must be between {} and {}",
                            min_year, current_year
                        ),
                    ));
                }

                Ok(ReleaseYear(ry))
            }
            Err(err) => Err(Error::new(ErrorKind::InvalidData, err)),
        }
    }
}

impl DateAdded {
    pub(crate) fn new() -> Self {
        DateAdded(Utc::now())
    }
}

impl Album {
    pub(crate) fn new(
        name: AlbumString,
        artist: AlbumString,
        tracks: TrackCount,
        release_year: ReleaseYear,
    ) -> Self {
        Self {
            name,
            artist,
            tracks,
            release_year,
            date_added: DateAdded::new(),
        }
    }

    pub(crate) fn to_tsv_entry(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.artist, self.name, self.tracks, self.release_year, self.date_added
        )
    }
}
