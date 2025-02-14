# PicoLimbo

> [!WARNING]
> This software is highly experimental. Use at your own risks and report any bugs by submitting an issue on GitHub.

An attempt at writing a lightweight Minecraft server from scratch in Rust. Currently, supports 1.19 and above.

## Introduction

This project is a lightweight Minecraft server written in Rust designed to serve as an AFK or waiting server. Its
primary focus is on efficiency, implementing only the essential packets required for client login and maintaining
connection (keep alive) without unnecessary overhead.

The server does not aim to replicate every feature or packet supported by Minecraft servers. However, it aims to support
all Minecraft versions from 1.7.2 up to the most recent ones.

## Getting Started

### Pterodactyl

To use in the Pterodactyl panel, you can import the [egg file](./pterodactyl/egg-pico-limbo.json) into your panel.

### Using Docker

```shell
docker run --rm -p "25565:25565" ghcr.io/quozul/picolimbo:master
```

### Using Docker Compose

A sample [docker-compose.yml file](./docker-compose.yml) is available in the repository.

## Similar Projects

- [Limbo](https://github.com/LOOHP/Limbo) - Supports only one version of the game at a time
- [NanoLimbo](https://github.com/Nan1t/NanoLimbo)
- [TyphoonLimbo](https://github.com/TyphoonMC/TyphoonLimbo) - No longer active

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvement, please submit an issue or
pull request on GitHub. Make sure to follow the existing code style and include relevant tests.

1. Fork the repository.
2. Create a new branch `git checkout -b <branch-name>`.
3. Make changes and commit `git commit -m 'Add some feature'`.
4. Push to your fork `git push origin <branch-name>`.
5. Submit a pull request.
