use std::fmt;

use crate::endpoints::movie::MovieShort;
use crate::endpoints::person::PersonShort;
use crate::endpoints::search::MultiResult;
use crate::endpoints::tv::TvShort;
use crate::{Language, Page};

/// the trending time window
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeWindow {
    Day,
    Week,
}

impl fmt::Display for TimeWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Day => "day",
            Self::Week => "week",
        })
    }
}

endpoint! {
    /// the trending movies
    trending_movies(window: TimeWindow): GET "/trending/movie/{}" => Page<MovieShort> {
        params { language: Language }
    }
}

endpoint! {
    /// the trending series
    trending_tv(window: TimeWindow): GET "/trending/tv/{}" => Page<TvShort> {
        params { language: Language }
    }
}

endpoint! {
    /// the trending people
    trending_people(window: TimeWindow): GET "/trending/person/{}" => Page<PersonShort> {
        params { language: Language }
    }
}

endpoint! {
    /// everything trending, movies, series and people mixed
    trending_all(window: TimeWindow): GET "/trending/all/{}" => Page<MultiResult> {
        params { language: Language }
    }
}
