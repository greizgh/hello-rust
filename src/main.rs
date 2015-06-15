use std::fmt;
extern crate rand;
use rand::Rng;

mod levenshtein;


/// Individual, member of a Population
#[derive(Clone,Eq,PartialEq,PartialOrd)]
struct Individual {
    /// Sequence of genes
    chromosome: String,
    /// Fitness denotes how 'fit' an individual is with regard to the goal
    fitness: i32,
}

impl std::cmp::Ord for Individual {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness.cmp(&other.fitness)
    }
}

impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.chromosome, self.fitness)
    }
}

impl Individual {
    /// Return a new Individual with given chromosome
    fn new<S: Into<String>>(s: S) -> Individual {
        Individual {chromosome: s.into(), fitness: 0}
    }

    /// Return a new Individual with a random chromosome of specified length
    fn new_random(chromosome_size: usize) -> Individual {
        let chromosome: String = (0..chromosome_size).map(|_| (0x20u8 + (rand::random::<f32>() * 90.) as u8) as char).collect();
        Individual::new(chromosome)
    }

    /// Mutate the individual
    fn mutate(&mut self, probability: f32) {
        let chromosome = self.chromosome.clone();
        self.chromosome.clear();

        for gene in chromosome.chars() {
            if rand::thread_rng().gen_range(0.0, 100.0) <= probability * 100. {
                self.chromosome.push((0x20u8 + (rand::random::<f32>() * 90.) as u8) as char);
            } else {
                self.chromosome.push(gene);
            }
        }
    }
}

/// Group of Individuals
struct Population<'a> {
    /// Mutation rate is the probability for a gene to be randomized
    mutation_rate: f32,
    crossover_rate: f32,
    generation: i32,
    /// Our actual members
    individuals: Vec<Individual>,
    /// closure to evaluate individual fitness
    fitness: &'a (Fn(&Individual) -> i32),
}

impl<'a> Population<'a> {
    /// Create a new population
    fn new(size: usize, chromosome_size: usize, crossover: f32, mutation: f32, fitness: &'a Fn(&Individual) -> i32 ) -> Population<'a> {
        let mut members: Vec<Individual> = Vec::with_capacity(size);
        for _ in 0..size {
            members.push(Individual::new_random(chromosome_size));
        }
        Population {mutation_rate: mutation, crossover_rate: crossover, generation: 0, individuals: members, fitness: fitness}
    }

    /// Evaluate fitness for every individual in the population
    fn compute_fitness(&mut self) {
        for individual in &mut self.individuals {
            individual.fitness = (self.fitness)(&individual);
        }
        self.individuals.sort();
    }

    /// Get the fittest individual of the population
    fn fittest(&self) -> &Individual {
        &self.individuals[0]
    }
    
    /// Mix two individual
    fn crossover(&self, parent_a: &Individual, parent_b: &Individual) -> (Individual, Individual) {
        let mut chromosome_a = String::with_capacity(parent_a.chromosome.len());
        let mut chromosome_b = String::with_capacity(parent_b.chromosome.len());

        for (gene_a, gene_b) in parent_a.chromosome.chars().zip(parent_b.chromosome.chars()) {
            if rand::thread_rng().gen_range(0., 100.) <= self.crossover_rate * 100. {
                chromosome_a.push(gene_b);
                chromosome_b.push(gene_a);
            } else {
                chromosome_a.push(gene_a);
                chromosome_b.push(gene_b);
            }
        }

        (Individual::new(chromosome_a), Individual::new(chromosome_b))
    }

    /// Let our population breed and create another generation
    fn breed(&mut self) {
        let pop_size = self.individuals.len();
        self.individuals.truncate((pop_size as f32 * 0.8) as usize);
        let parents = self.individuals.clone();
        let chromosome_size = parents[0].chromosome.len();

        self.individuals.clear();

        for chunk in parents.chunks(2) {
            if chunk.len() == 2 {
                let (child_a, child_b) = self.crossover(&chunk[0], &chunk[1]);
                self.individuals.push(child_a);
                self.individuals.push(child_b);
            }
        }

        for individual in &mut self.individuals {
            individual.mutate(self.mutation_rate);
        }

        while self.individuals.len() < pop_size {
            self.individuals.push(Individual::new_random(chromosome_size));
        }

        self.compute_fitness();

        self.generation += 1;
    }
}

fn main() {
    let target = "Hello World!";

    let fitness = |individual: &Individual| -> i32 {
        levenshtein::levenshtein(target, &individual.chromosome)
    };

    let mut pop = Population::new(100, target.len(), 0.5, 0.01, &fitness);

    pop.compute_fitness();

    while &pop.fittest().chromosome != target {
        println!("Generation {}, size: {}\tFittest: {}", pop.generation, pop.individuals.len(), pop.fittest());

        pop.breed();
    }
    println!("Generation {}, size: {}\tFittest: {}", pop.generation, pop.individuals.len(), pop.fittest());
}
