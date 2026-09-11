pub fn measure_throughput(operations: u64, elapsed_secs: f64) -> f64 {
    operations as f64 / elapsed_secs
}
