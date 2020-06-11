#[derive(Debug)]

pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let score = self
            .scores
            .iter()
            .fold(0, |acc, x| if acc > *x { acc } else { *x });
        match score {
            0 => Option::None,
            n => Option::Some(n),
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top3 = self.scores.clone();
        // sort and reverse to get top 3!
        top3.sort();
        top3.reverse();
        // return three highest, use copied to get u32 instead of &u32
        top3.iter().take(3).copied().collect::<Vec<_>>()
    }
}
