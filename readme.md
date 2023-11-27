# game_of_life 

A simple implementation of Conway's Game of Life in Rust using the macroquad game development framework.

## Overview

Conway's Game of Life is a cellular automaton devised by the mathematician John Conway. The game consists of a grid of cells, each of which can be in one of two states: alive or dead. The state of the cells evolves based on simple rules, creating interesting patterns and behaviors.

## Features

- **Customizable Board:** Specify the width and height of the game board, allowing for different grid sizes.
  
- **Random Initialization:** Initialize the board with a random configuration or use predefined patterns like the glider gun.

- **Interactive Controls:** 
  - **F1:** Randomize Board - Resets the board with a random configuration.
  - **F2:** Toggle Auto-Play - Starts or stops the automatic progression of generations.
  - **F3:** Glider Gun - Initializes the board with the famous glider gun pattern.
  - **F4:** Manual Step - Advances the simulation by one generation when auto-play is disabled.


- **Visualization:** Visualize the evolving generations of cells with a simple graphical representation using macroquad.

## Example
<img src="example.gif"></img>

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/)

### Building and Running

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/rust-game-of-life.git
    cd rust-game-of-life
    ```

2. Build and run the project:

    ```bash
    cargo run
    ```

## Acknowledgments

- [macroquad](https://github.com/not-fl3/macroquad) - Simple and easy-to-use game development framework for Rust.

- [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) - Learn more about the fascinating world of cellular automata.
