# rust-tsp, "Rusty Genes"
- A Rust implementation of a genetic program to solve the travelling salesperson problem.
- The program reads a list of cities from a file, runs a genetic algorithm to find an optimal path, and prints the best path found.
- The genetic algorithm uses a combination of single point, uniform order, and partially mapped crossover methods for breeding, and a simple swap mutation method.
- The program uses command line arguments to control the number of iterations, population size, crossover rate, mutation rate, and survival rate.
- See [this repo](https://github.com/LTeder/rust-symboreg) for my other attempt at writing a genetic program/Rust programming.

## Next Steps
- Add unit tests to ensure the correctness of the genetic algorithm.
- Add additional mutation methods.
- Implement a GUI visualization of the program as it runs using a library such as `iced`. It should display the current best path and update in real time as the program runs.