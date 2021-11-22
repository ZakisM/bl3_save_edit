use std::collections::HashSet;
use std::slice::Iter;

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use bl3_save_edit_core::resources::ResourcePartInfo;

use crate::views::item_editor::parts::available_parts::{
    AvailableCategorizedPart, AvailableResourcePart,
};
use crate::views::item_editor::parts::current_parts::{
    CurrentCategorizedPart, CurrentItemEditorPart,
};

pub mod available_parts;
pub mod current_parts;

pub trait SearchablePart {
    fn name(&self) -> &str;

    fn info(&self) -> &ResourcePartInfo;
}

pub trait SearchableCategorizedParts {
    type PartType: SearchablePart + Sync;

    fn category(&self) -> &str;

    fn parts(&self) -> Iter<Self::PartType>;
}

pub fn filter_parts<'a, T>(
    search_query: &str,
    parts: &'a [T],
) -> Vec<&'a <T as SearchableCategorizedParts>::PartType>
where
    T: SearchableCategorizedParts + Sync,
{
    let search_query = search_query.trim().to_lowercase();

    let search_query_words = search_query
        .split_ascii_whitespace()
        .collect::<HashSet<_>>();

    parts
        .into_par_iter()
        .map(|cat_p| {
            if cat_p.category().to_lowercase().contains(&search_query) {
                cat_p.parts().collect::<Vec<_>>()
            } else {
                cat_p
                    .parts()
                    .filter(|p| {
                        search_query_words.par_iter().all(|s| {
                            p.name().to_lowercase().contains(s)
                                || p.info()
                                    .positives
                                    .par_iter()
                                    .any(|pos| pos.to_lowercase().contains(s))
                                || p.info()
                                    .negatives
                                    .par_iter()
                                    .any(|neg| neg.to_lowercase().contains(s))
                                || p.info()
                                    .effects
                                    .par_iter()
                                    .any(|eff| eff.to_lowercase().contains(s))
                        })
                    })
                    .collect::<Vec<_>>()
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}

impl SearchablePart for AvailableResourcePart {
    fn name(&self) -> &str {
        &self.part.name
    }

    fn info(&self) -> &ResourcePartInfo {
        &self.part.info
    }
}

impl SearchableCategorizedParts for AvailableCategorizedPart {
    type PartType = AvailableResourcePart;

    fn category(&self) -> &str {
        &self.category
    }

    fn parts(&self) -> Iter<Self::PartType> {
        self.parts.iter()
    }
}

impl SearchableCategorizedParts for &AvailableCategorizedPart {
    type PartType = AvailableResourcePart;

    fn category(&self) -> &str {
        &self.category
    }

    fn parts(&self) -> Iter<Self::PartType> {
        self.parts.iter()
    }
}

impl SearchablePart for CurrentItemEditorPart {
    fn name(&self) -> &str {
        &self.part.part.ident
    }

    fn info(&self) -> &ResourcePartInfo {
        &self.part.info
    }
}

impl SearchableCategorizedParts for CurrentCategorizedPart {
    type PartType = CurrentItemEditorPart;

    fn category(&self) -> &str {
        &self.category
    }

    fn parts(&self) -> Iter<Self::PartType> {
        self.parts.iter()
    }
}
