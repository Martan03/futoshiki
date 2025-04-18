use std::collections::HashMap;

use termint::enums::Color;

use crate::{
    bench::bench_struct::Bench,
    solver::domain::{
        bit_domain::BitDomain, hash_domain::HashDomain, DomainTrait,
    },
};

use super::{bench_stat::BenchStat, charter::Charter};

#[derive(Debug, Clone)]
pub struct DocBench;

impl DocBench {
    pub fn run() {
        Self::bench_domains();
    }

    pub fn bench_domains() {
        let mut charter = Charter::empty("Domain benchmark");
        let repeats = 10000;
        let domains = get_domains();

        for size in [10, 20, 30, 40, 50, 60] {
            println!("Domain with {size} numbers, {repeats} repeats");
            let mut stats = HashMap::new();

            for (domain, get_domain) in domains.iter() {
                let test_domain = get_domain(size);
                Self::bench_domain(&mut stats, domain, test_domain, size);
            }

            let mut stats: Vec<_> = stats.iter().collect();
            stats.sort_by_key(|(_, stat)| stat.total_time);
            for (i, (domain, stat)) in stats.iter().enumerate() {
                print!("{}{}. ", Color::Gray.to_fg(), i + 1);

                let secs = stat.avg_time().as_secs_f64();
                charter.push(domain.to_string(), size as i32, secs);
                Self::print_stat(domain, stat);
            }
        }

        _ = charter.plot("domains_benchmark.png");
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

    fn print_stat(title: &String, stat: &BenchStat) {
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
}

fn get_domains() -> Vec<(String, Box<dyn Fn(usize) -> Box<dyn DomainTrait>>)> {
    vec![
        ("HashSet".to_string(), Box::new(get_hash_domain)),
        ("Bitmap".to_string(), Box::new(get_bit_domain)),
    ]
}

fn get_hash_domain(size: usize) -> Box<dyn DomainTrait> {
    Box::new(HashDomain::default(size))
}

fn get_bit_domain(size: usize) -> Box<dyn DomainTrait> {
    Box::new(BitDomain::default(size))
}
