# Raiders (WIP)

Raiders is a procedurally generated stealth/RTS VR game written in Rust, using the Amethyst game engine. This project has three goals:

	- To help me gain skills with developing for gaming and virtual reality platforms, as these are domains I have no prior experience with
	- To contribute to the Amethyst ecosystem by developing an OpenXR plugin
	- To put my new HTC Vive to use (other than just for playing games and trying out cool VR workstations)
	
This project is currently in its ***very*** early stages. Currently, I am working towards putting together a minimum viable product (MVC), so I can start developing an OpenXR plugin for the engine. 

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
