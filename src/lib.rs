use std::sync::{Arc, RwLock};

use rand::prelude::*;

pub mod zort;

pub struct Gene<T> {
    innovation: usize,
    enabled: bool,
    phene: T,
}

pub struct Genome<T> {
    platter: Arc<RwLock<Platter>>,
    genes: Vec<Gene<T>>,
}

pub struct Platter {
    innovation: usize,
    rng: ThreadRng,
}

type Bias<T> = fn(Gene<T>, Gene<T>) -> Gene<T>;

impl<T> Genome<T> {
    pub fn new(platter: Arc<RwLock<Platter>>) -> Self { Self { platter, genes: vec![] } }

    pub fn splice(
        platter: Arc<RwLock<Platter>>,
        left: &Self,
        right: &Self,
        bias: Bias<T>,
        mutation: f64
    ) -> Self {
        let mut genome = Genome::new(platter);
        let zort = zort::Zort::new(
            left.genes.iter(),
            right.genes.iter(),
            Genome::ranker,
            1
        );
        for g in zort { genome.push(g, mutation); }
        genome
    }

    pub fn bias(left: Gene<T>, _: Gene<T>) -> Gene<T> {
        left
    }

    fn ranker(value: &&Gene<T>) -> usize {
        value.innovation
    }

    pub fn push(&mut self, gene: &Gene<T>, mutation: f64) {
        let y: f64 = self.platter.write().unwrap().rng.gen();
    }

    pub fn collect(self) -> Vec<T> {
        let mut output: Vec<T> = vec![];
        for gene in self.genes {
            if gene.enabled {
                output.push(gene.phene);
            }
        }
        output
    }
}

impl Platter {
    pub fn new() -> Self { Self { innovation: 0, rng: rand::thread_rng() } }

    pub fn rand(mut self) -> f64 {
        self.rng.gen()
    }
}
