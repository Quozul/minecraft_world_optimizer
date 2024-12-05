use std::fmt::Display;

#[derive(Default, Clone)]
pub struct OptimizeResult {
    pub total_chunks: usize,
    pub deleted_chunks: usize,
    pub deleted_regions: usize,
}

impl Display for OptimizeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Optimization Result:\n\
                   Total Chunks: {}\n\
                   Deleted Chunks: {}\n\
                   Deleted Regions: {}",
            self.total_chunks, self.deleted_chunks, self.deleted_regions
        )
    }
}

pub fn reduce_optimize_results(results: &mut [OptimizeResult]) -> OptimizeResult {
    results
        .iter_mut()
        .reduce(|acc, cur| {
            acc.deleted_regions += cur.deleted_regions;
            acc.total_chunks += cur.total_chunks;
            acc.deleted_chunks += cur.deleted_chunks;
            acc
        })
        .cloned()
        .unwrap_or_default()
}
