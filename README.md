# Filler Game - Rust Solution

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)

</div>

A Rust implementation of an AI player for the Filler game, containerized with Docker for easy testing and development.


## 📖 About

Filler is a strategic board game where two players (or AI bots) compete to occupy the maximum territory on a grid. This project contains a Rust-based AI solution that competes against pre-built robot opponents.

### Game Demo

Here's an example of a game between Bender and Terminator:

![Filler Game - Bender vs Terminator](Screenshot%20from%202025-10-15%2018-20-39.png)

## 🚀 Quick Start

### Prerequisites

- Docker installed on your system
- Basic knowledge of command line operations

### Initial Setup

Before building the Docker image, you need to download the required game files:

1. Download the `docker_image` folder as a zip file from: https://assets.01-edu.org/filler/filler.zip
2. Extract the zip file - it contains:
   - `Dockerfile` - Container configuration
   - `game_engine` - The game executable
   - `maps/` - Game maps (map00, map01, map02)
   - `robots/` - Pre-built opponent AIs (bender, h2_d2, terminator, wall_e)
3. Ensure all files are properly extracted in your project directory

### Building the Docker Image

Build the Docker image with the following command:

```bash
docker build -t filler .
```

### Running the Container

Launch an interactive terminal session inside the container:

```bash
docker run -it filler
```

The `solution` directory is automatically mounted and compiled inside the container.

## 🎮 Playing the Game

### Running a Game

Inside the container, use the game engine to start a match:

```bash
# Basic syntax
./game_engine -f <map_file> -p1 <player1> -p2 <player2>
```

### Example Commands

**Watch two robots compete:**
```bash
./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
```

**Test your solution against a robot:**
```bash
./game_engine -f maps/map01 -p1 robots/bender -p2 solution/target/debug/solution
```

**Available Maps:**
- `maps/map00`
- `maps/map01`
- `maps/map02`

**Available Opponent Robots:**
- `robots/bender` - Standard difficulty
- `robots/terminator` - Advanced difficulty ⚠️

## 📁 Project Structure

```
.
├── Dockerfile              # Container configuration
├── README.md              # This file
├── maps/                  # Game maps
│   ├── map00
│   ├── map01
│   └── map02
├── solution/              # Your Rust AI implementation
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       ├── main.rs        # Main game logic
│       └── utils.rs       # Helper functions
└── robots/                # Pre-built opponent AIs
```

## 🛠️ Development

### Building Your Solution

Your solution is located in the `solution/` directory. The Docker container automatically builds it using:

```bash
cargo build --release
```

For development and debugging, you can use:

```bash
cargo build  # Debug build (faster compilation)
```

### Testing Your Solution

1. Make changes to the Rust code in `solution/src/`
2. Rebuild the Docker image
3. Run the container and test against different opponents and maps

## 📝 Notes

- **Terminator Challenge**: The `terminator` robot is exceptionally strong and represents an advanced challenge. Beating it is optional but demonstrates a highly optimized solution.
- The solution uses Rust's standard I/O to communicate with the game engine
- Debug output is written to `output.txt` for troubleshooting

## 🎯 Strategy Tips

- Analyze the board state carefully before each move
- Consider both offensive and defensive positioning
- Different maps may require different strategies
- Start by consistently beating `bender` before challenging `terminator`

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

<div align="center">

**⭐ Star this repository if you found it helpful! ⭐**

Made with ❤️ from 🇸🇳

</div>