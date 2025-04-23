use std::collections::HashMap;

use termint::enums::Color;

use crate::{
    bench::bench_struct::Bench,
    solver::domain::{
        bit_domain::BitDomain, hash_domain::HashDomain, vec_domain::VecDomain,
        DomainTrait,
    },
};

use super::{bench_stat::BenchStat, charter::Charter};

/// Implements methods for running the benchmarks used in the documentation.
#[derive(Debug, Clone)]
pub struct DocBench;

impl DocBench {
    /// Runs all benchmarks used in documentation.
    pub fn run() {
        Self::bench_domains();
    }

    /// Runs domains benchmarks - memory usage & time complexity.
    pub fn bench_domains() {
        let mut speed_charter = Charter::empty("Domain benchmark");
        let mut size_charter = Charter::empty("Domain size").y_label("Bytes");

        let repeats = 10000;
        let domains = get_domains();

        for size in [10, 20, 30, 40, 50, 60] {
            println!("Domain with {size} numbers, {repeats} repeats");
            let mut stats = HashMap::new();
            let mut memory = HashMap::new();

            for (domain, get_domain) in domains.iter() {
                let (test_domain, bytes) = get_domain(size);
                memory.insert(format!("{domain} size"), bytes);
                Self::bench_domain(&mut stats, domain, test_domain, size);
            }

            println!("Benchmark:");
            let mut stats: Vec<_> = stats.iter().collect();
            stats.sort_by_key(|(_, stat)| stat.total_time);
            for (i, (domain, stat)) in stats.iter().enumerate() {
                print!("{}{}. ", Color::Gray.to_fg(), i + 1);

                let secs = stat.avg_time().as_secs_f64();
                speed_charter.push(domain.to_string(), size as i32, secs);
                Self::print_stat(domain, stat);
            }

            println!("Memory:");
            let mut memory: Vec<_> = memory.iter().collect();
            memory.sort_by_key(|(_, size)| *size);
            for (i, (domain, bytes)) in memory.iter().enumerate() {
                print!("{}{}. ", Color::Gray.to_fg(), i + 1);

                size_charter.push(
                    domain.to_string(),
                    size as i32,
                    **bytes as f64,
                );
                Self::print_memory(domain, bytes);
            }
        }
        _ = speed_charter.plot("domains_benchmark.png");
        _ = size_charter.plot_lin("domain_size.png");
    }

    fn bench_domain(
        stats: &mut HashMap<String, BenchStat>,
        domain: &String,
        test_domain: Box<dyn DomainTrait>,
        size: usize,
    ) {
        let repeats = 10;
        let stat =
            Bench::run(|| _ = test_domain.clone().remove(size), repeats);
        stats.insert(format!("{} remove", domain), stat);

        let stat =
            Bench::run(|| _ = test_domain.clone().remove_greater(1), repeats);
        stats.insert(format!("{} remove greater", domain), stat);

        let stat =
            Bench::run(|| _ = test_domain.clone().remove_lower(size), repeats);
        stats.insert(format!("{} remove lower", domain), stat);
    }

    fn print_stat(title: &str, stat: &BenchStat) {
        println!(
            "{}{title}:\n\
            {}└>\x1b[0m Time: [{}{:?} {}{:?} {}{:?}\x1b[0m]",
            Color::Green.to_fg(),
            Color::Gray.to_fg(),
            Color::Gray.to_fg(),
            stat.min_time,
            Color::White.to_fg(),
            stat.avg_time(),
            Color::Gray.to_fg(),
            stat.max_time
        );
    }

    fn print_memory(title: &str, bytes: &usize) {
        println!(
            "{}{title}:\n\
            {}└>\x1b[0m Memory: {}{:?} B{} => {} b\x1b[0m",
            Color::Green.to_fg(),
            Color::Gray.to_fg(),
            Color::White.to_fg(),
            bytes,
            Color::Gray.to_fg(),
            bytes * 8,
        );
    }
}

fn get_domains(
) -> Vec<(String, Box<dyn Fn(usize) -> (Box<dyn DomainTrait>, usize)>)> {
    vec![
        ("HashSet".to_string(), Box::new(get_hash_domain)),
        ("Vector".to_string(), Box::new(get_vec_domain)),
        ("Bitmap".to_string(), Box::new(get_bit_domain)),
    ]
}

fn get_hash_domain(size: usize) -> (Box<dyn DomainTrait>, usize) {
    let domain = HashDomain::default(size);
    let bucket_size = domain.0.capacity() * size_of::<usize>();
    (Box::new(domain), size_of::<HashDomain>() + bucket_size)
}

fn get_vec_domain(size: usize) -> (Box<dyn DomainTrait>, usize) {
    let domain = VecDomain::default(size);
    let bucket_size = domain.0.capacity() * size_of::<usize>();
    (Box::new(domain), size_of::<VecDomain>() + bucket_size)
}

fn get_bit_domain(size: usize) -> (Box<dyn DomainTrait>, usize) {
    (Box::new(BitDomain::default(size)), size_of::<BitDomain>())
}
