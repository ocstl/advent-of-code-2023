use std::ops::Range;

pub trait RangeExtension {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;

    fn intersection(&self, other: &Self) -> Self;
}

impl<U: Sized + PartialOrd + Ord + Copy> RangeExtension for Range<U> {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(&other.start) && self.contains(&other.end)
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(&other.start) || other.contains(&self.start)
    }

    fn intersection(&self, other: &Self) -> Self {
        Self {
            start: self.start.max(other.start),
            end: self.end.min(other.end),
        }
    }
}
