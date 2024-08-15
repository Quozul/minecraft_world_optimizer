# Minecraft World Optimizer

**Minecraft World Optimizer** is a command-line tool designed to reduce the size of your Minecraft world by parsing and
optimizing region files. It's particularly useful for large worlds with many generated but unexplored chunks.

## How it works

The optimizer employs the following steps to analyze and optimize your world:

1. **Parallel Processing:** Utilizes all available threads on your computer to load and parse region files
   simultaneously,
   improving processing speed.
2. **Invalid File Removal:** Deletes any invalid region files, such as those with insufficient size to hold chunk data.
3. **Chunk Parsing & Removal:**
    * Ignores and deletes invalid chunks within valid region files.
    * Removes not fully generated or unexplored chunks to reduce world size.

The optimizer operates in-place, replacing original region files with optimized ones. **Always make a backup before
running!**

## Supported Compression Algorithms

Minecraft supports several compression algorithms for region files.
The Minecraft World Optimizer currently supports deflate, notably Zlib and GZip, as deflate is the default for most
clients and servers.
If your world uses a different compression method, you may experience **data loss**.

## Expected Results

In testing on a backed-up server world:

- Original world size : 105 681 020 bytes and 24 381 files
- After running the program : 16 145 728 bytes and 6 961 files
  Resulting in an **84.7%** world size reduction.

## Prerequisites

- Rust toolchain (stable)
- A backed-up Minecraft world

## Installation

### With Cargo

```shell
cargo install --git https://github.com/Quozul/minecraft_world_optimizer.git
```

### From Source

1. Clone the repository:
   ```shell
   git clone https://github.com/Quozul/minecraft_world_optimizer.git
   cd minecraft_world_optimizer
   ```
2. Install the optimizer globally:
   ```shell
   cargo install --path .
   ```

## Usage

```shell
minecraft_world_optimizer <WORLD_PATH>
```

Replace <WORLD_PATH> with the path to your Minecraft world folder containing region files.

Example:

```shell
minecraft_world_optimizer ~/.minecraft/saves/MyWorld/region
```

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvement, please submit an issue or
pull request on GitHub. Make sure to follow the existing code style and include relevant tests.

1. Fork the repository.
2. Create a new branch (git checkout -b <branch-name>).
3. Make changes and commit (git commit -am 'Add some feature').
4. Push to your fork (git push origin <branch-name>).
5. Submit a pull request.
