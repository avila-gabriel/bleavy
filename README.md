## 🚧 Requirements

To use this project as a base for your own game, you'll need the following tools installed:

- [**Gleam**](https://gleam.run/) — compiles core game logic
- [**Task**](https://taskfile.dev/) — runs build pipelines and watches for changes
- [**Bun**](https://bun.sh/) — handles JavaScript bundling and asset building

### 🛠️ First-time setup

Before running the development watcher, make sure to compile the Gleam source once manually:

```sh
cd systems
gleam build
cd ..
```

Then start the automated build pipeline:

```sh
task --watch all
```

This setup ensures that the watcher can detect future changes properly.

You can now begin extending the game logic using Gleam and building your game on top of this template.
