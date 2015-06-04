# Hello rust!

This is not a hello world as you might expect. Well, it does print "hello world" but the process is not as straightforward as:
```rust
fn main() {
    println!("Hello World!");
}
```

## Hello world is evolving

It all started as an experiment with rust: how easily can I implement a basic genetic algorithm? The answer is: pretty easily.

What will you find in the code?

* Genes
* A lot of Individuals
* Population: a bunch of individuals which may grew old enough to reproduce themselves
* A breed method and a crossover method
* A levenshtein distance algorithm to compute the fitness of our individuals

The goal here is to make a population evolve itself until one of the individual has chromosome "Hello World".
For each generation the fittest member is printed so you can see the evolution.

## Process explanation

### Initialization

At first, a population of random individuals is created. They all have a chromosome of the same length than the target ("Hello World" here).

### Selection

Only fittest elements are allowed to make it to the next generation. Individuals are sorted by fitness descending and the last 10% is replaced by new (random) individuals.

Fitness is actually the levenshtein distance between the target and the chromosome.

### Reproduction

There may be crossover between two individuals. Without crossover, children are clones of their parents.

### Mutation

Every now and then a gene can mutate. There is nothing preventing multiple mutation on the same individual: chances of mutation are per gene.

### Again

Once we have gone through this process, we repeat it until an individual's chromosome equals the target.
