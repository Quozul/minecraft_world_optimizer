# Minecraft World Optimizer

**Minecraft World Optimizer** is a command-line tool designed to reduce the size of your Minecraft world by parsing and
optimizing region files. It's particularly useful for large worlds that have many generated but unexplored chunks.

## How it works

The optimizer employs the following steps to analyze and optimize your world:

1. **Parallel Processing:** Leverage all available threads on your computer to load and parse region files
   simultaneously,
   improving processing speed.
2. **Invalid File Removal:** Deletes any invalid region files, such as those with insufficient size to hold chunk data.
3. **Chunk Parsing & Removal:**
    * Ignores and deletes invalid chunks within valid region files.
    * Removes not fully generated or unexplored chunks to reduce world size.
4. **Improved Compression:** Chunks are saved back using the best available compression level for further space
   optimization.

The optimizer operates in-place, replacing original region files with optimized ones. **Always make a backup before
running!**

## Supported Compression Algorithms

Minecraft supports several compression algorithms for chunk data inside the region files.
The Minecraft World Optimizer currently only supports the deflate compression algorithm, Zlib and GZip, as deflate is
the default for most clients and servers.
If your world uses a different compression method, you may experience **data loss**.

The Minecraft World Optimizer has only been successfully tested on 1.20.6 and 1.21 worlds.

## Expected Results

The Minecraft World Optimizer as successfully been tested on the following worlds:

- **Server World:** Backed-up world of my survival server with over 13.9 million generated chunks.
- **New World:** New amplified world pre-generated using Chunky with a square radius of 1024.

| World        | Before Optimization                      | After Optimization                     | Size Reduction |
|--------------|------------------------------------------|----------------------------------------|----------------|
| Server World | 105,681,020 Kilobytes <br/> 24,381 Files | 16,145,728 Kilobytes <br/> 6,961 Files | **84.7%**      |
| New World    | 235,204 Kilobytes <br/> 36 Files         | 2,440 Kilobytes <br/> 4 Files          | **98.9%**      |

## Getting Started

### Prerequisites

- Rust toolchain (stable)
- A backed-up Minecraft world

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
2. Create a new branch `git checkout -b <branch-name>`.
3. Make changes and commit `git commit -m 'Add some feature'`.
4. Push to your fork `git push origin <branch-name>`.
5. Submit a pull request.
