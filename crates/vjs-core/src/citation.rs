use crate::types::*;
use crate::error::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Citation {
    pub year: i32,
    pub series: CitationSeries,
    pub n: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum CitationSeries {
    Cc(String),
    Pc,
    Sc,
    Reg,
    Act,
}

pub struct CitationRegistry {
    pub citations: HashMap<Citation, AuthorityId>,
}

impl Default for CitationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CitationRegistry {
    pub fn new() -> Self {
        Self {
            citations: HashMap::new(),
        }
    }

    pub fn next_citation(&self, series: CitationSeries, year: i32) -> Citation {
        let highest = self
            .citations
            .keys()
            .filter(|c| c.year == year && c.series == series)
            .map(|c| c.n)
            .max()
            .unwrap_or(0);

        Citation {
            year,
            series,
            n: highest + 1,
        }
    }

    pub fn register(&mut self, citation: Citation, authority: AuthorityId) -> Result<(), KernelError> {
        if self.citations.contains_key(&citation) {
            return Err(KernelError::CitationCollision(format!(
                "{:?}",
                citation
            )));
        }
        self.citations.insert(citation, authority);
        Ok(())
    }
}
