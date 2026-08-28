use chrono::{Duration, Utc};

/// SM-2 spaced repetition algorithm result.
pub struct Sm2Result {
    pub ease_factor: f64,
    pub interval: i64,
    pub repetitions: i64,
    /// ISO-formatted date string for the next review.
    pub next_review: String,
}

/// User rating for a flashcard review.
#[derive(Debug, Clone, Copy)]
pub enum Rating {
    /// Complete blackout — reset progress.
    Again,
    /// Recalled with significant difficulty.
    Hard,
    /// Recalled with some hesitation.
    Good,
    /// Perfect recall.
    Easy,
}

impl Rating {
    /// Map the rating to an SM-2 quality score (0–5).
    fn quality(self) -> i64 {
        match self {
            Rating::Again => 0,
            Rating::Hard => 3,
            Rating::Good => 4,
            Rating::Easy => 5,
        }
    }
}

impl std::str::FromStr for Rating {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "again" => Ok(Rating::Again),
            "hard" => Ok(Rating::Hard),
            "good" => Ok(Rating::Good),
            "easy" => Ok(Rating::Easy),
            _ => Err("invalid rating; expected again|hard|good|easy"),
        }
    }
}

/// Apply the SM-2 algorithm and return updated scheduling fields.
///
/// Reference: <https://www.supermemo.com/en/archives1990-2015/english/ol/sm2>
pub fn calculate(
    current_ef: f64,
    current_interval: i64,
    current_repetitions: i64,
    rating: Rating,
) -> Sm2Result {
    let q = rating.quality();

    let (new_ef, new_interval, new_reps) = if q < 3 {
        // Failed recall — reset
        let ef = (current_ef - 0.20).max(1.3);
        (ef, 1i64, 0i64)
    } else {
        // Successful recall
        let ef = {
            let delta = 0.1 - (5 - q) as f64 * (0.08 + (5 - q) as f64 * 0.02);
            (current_ef + delta).max(1.3)
        };

        let interval = match current_repetitions {
            0 => 1,
            1 => 6,
            _ => {
                let raw = current_interval as f64 * ef;
                raw.round() as i64
            }
        };

        (ef, interval, current_repetitions + 1)
    };

    let next_date = Utc::now().date_naive() + Duration::days(new_interval);

    Sm2Result {
        ease_factor: new_ef,
        interval: new_interval,
        repetitions: new_reps,
        next_review: next_date.format("%Y-%m-%d").to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_card_good_rating() {
        let r = calculate(2.5, 0, 0, Rating::Good);
        assert_eq!(r.interval, 1);
        assert_eq!(r.repetitions, 1);
        assert!(r.ease_factor > 2.4);
    }

    #[test]
    fn interval_grows_with_good_ratings() {
        let r1 = calculate(2.5, 0, 0, Rating::Good);
        let r2 = calculate(r1.ease_factor, r1.interval, r1.repetitions, Rating::Good);
        let r3 = calculate(r2.ease_factor, r2.interval, r2.repetitions, Rating::Good);
        assert_eq!(r1.interval, 1);
        assert_eq!(r2.interval, 6);
        assert!(r3.interval >= 14, "expected >=14, got {}", r3.interval);
    }

    #[test]
    fn again_resets_progress() {
        let r = calculate(2.5, 14, 3, Rating::Again);
        assert_eq!(r.interval, 1);
        assert_eq!(r.repetitions, 0);
        assert!(r.ease_factor <= 2.5);
    }

    #[test]
    fn ease_factor_never_below_1_3() {
        let r = calculate(1.3, 1, 0, Rating::Again);
        assert!(r.ease_factor >= 1.3);
    }

    #[test]
    fn easy_boosts_ease_factor() {
        let r = calculate(2.5, 6, 2, Rating::Easy);
        assert!(r.ease_factor > 2.5);
        assert!(r.interval > 6);
    }
}
