use crate::interval::{Interval, IntervalClass, IntervalRoot};
use std::collections::BTreeSet;

pub type IntervalSet = BTreeSet<Interval>;
pub type IntervalClassSet = BTreeSet<IntervalClass>;
pub type IntervalRootSet = BTreeSet<IntervalRoot>;
