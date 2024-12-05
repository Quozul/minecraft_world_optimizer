# Minecraft World Optimizer

> [!WARNING]
> This software is highly experimental. Use at your own risks and report any bugs by submitting an issue on GitHub.

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

- **Server Worlds:** Backed-up world of my survival server with over 17.9 million generated chunks.
- **New World:** New amplified world pre-generated using Chunky with a square radius of 1024.

| World         | Before Optimization                      | After Optimization                     | Size Reduction |
|---------------|------------------------------------------|----------------------------------------|----------------|
| Server Worlds | 134,079,252 Kilobytes <br/> 29,861 Files | 27,220,248 Kilobytes <br/> 9,734 Files | **79.7%**      |
| New World     | 235,204 Kilobytes <br/> 36 Files         | 2,440 Kilobytes <br/> 4 Files          | **98.9%**      |

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
# View up-to-date usage of the command:
❯ minecraft_world_optimizer --help
❯ minecraft_world_optimizer <MODE> <WORLD_PATHS>...
```

Replace mode with one of the following:

- `check`: the program will only check for chunks and region files that can be deleted without actually deleting any
  data. This mode is around two times faster than the write mode as it does not perform any file system operations.
- `write`: the program will delete unused chunks and region files.

Replace <WORLD_PATHS> with the path to your Minecraft world folder containing region files.
It will detect the 3 vanilla dimensions and optimise them. Note that this has not been tested on modded worlds with
multiple dimensions.

Example:

```shell
❯ minecraft_world_optimizer check ~/.minecraft/saves/MyWorld
```

It can also be used to optimize server worlds as dimensions are split in multiple worlds:

```shell
❯ minecraft_world_optimizer check /path/to/server/world*
# Or if your shell does not support wildcard:
❯ minecraft_world_optimizer check /path/to/server/world /path/to/server/world_nether /path/to/server/world_the_end
```

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvement, please submit an issue or
pull request on GitHub. Make sure to follow the existing code style and include relevant tests.

1. Fork the repository.
2. Create a new branch `git checkout -b <branch-name>`.
3. Make changes and commit `git commit -m 'Add some feature'`.
4. Push to your fork `git push origin <branch-name>`.
5. Submit a pull request.
