
# Quackers Frontend

Bevy code for the amazing online multiplayer game: _[Quackers](https://quackers-game.itch.io/quackers-beta)_

<br/>

## Quickstart

1) Install wasm built target:
```
rustup target add wasm32-unknown-unknown
```

2) Set backend endpoint

```
export BACKEND_WS_ENDPOINT=ws://0.0.0.0:8000/ws
```

_Note: You'll also need to have the [Quackers backend project](https://github.com/JimLynchCodes/Quackers-Backend) running locally in order for the game to work properly._ 


3) Run frontend wasm project locally in a browser: 
```
trunk serve
```

<br/>

## Deploying

make sure to set these secrets in Github actions:

1) `BUTLER_CREDENTIALS` - just straight up copy and paste the itch.io key for app you want to deploy

2) `BACKEND_WS_ENDPOINT` - endpoint for the backend server. eg: `wss://your-subdomain.your-domain.com/ws`

Backend ws endpoint defaults to `ws://0.0.0.0:8000/ws`

but you can override it and for example get the live backend locally:
```
export BACKEND_WS_ENDPOINT=ws://0.0.0.0:8000/ws
```

you can also unset local env vars like this:
```
unset BACKEND_WS_ENDPOINT
```

You can also view and locally run the [Quackers-Backend code](https://github.com/JimLynchCodes/Quackers-Backend). 

Github secrets are copied over to env vars in the app by `release.yaml`.

# Bevy New 2D

This template is a great way to get started on a new 2D [Bevy](https://bevyengine.org/) game!
Start with a [basic project structure](#write-your-game) and [CI / CD](#release-your-game) that can deploy to [itch.io](https://itch.io).
You can [try this template in your web browser!](https://the-bevy-flock.itch.io/bevy-new-2d)

[@ChristopherBiscardi](https://github.com/ChristopherBiscardi) made a video on how to use this template from start to finish:

[<img src="./docs/img/thumbnail.png" width=40% height=40% alt="A video tutorial for bevy_new_2d, formerly known as bevy_quickstart"/>](https://www.youtube.com/watch?v=ESBRyXClaYc)

## Prerequisites

We assume that you know how to use Bevy already and have seen the [official Quick Start Guide](https://bevyengine.org/learn/quick-start/introduction/).

If you're new to Bevy, the patterns used in this template may look a bit weird at first glance.
See our [Design Document](./docs/design.md) for more information on how we structured the code and why.

## Create a new game

Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) and run the following command:

```sh
cargo generate thebevyflock/bevy_new_2d
```

Then [create a GitHub repository](https://github.com/new) and push your local repository to it.

## Write your game

The best way to get started is to play around with what you find in [`src/demo/`](./src/demo).

This template comes with a basic project structure that you may find useful:

| Path                                               | Description                                                        |
| -------------------------------------------------- | ------------------------------------------------------------------ |
| [`src/lib.rs`](./src/lib.rs)                       | App setup                                                          |
| [`src/asset_tracking.rs`](./src/asset_tracking.rs) | A high-level way to load collections of asset handles as resources |
| [`src/audio/`](./src/audio)                        | Marker components for sound effects and music                      |
| [`src/demo/`](./src/demo)                          | Example game mechanics & content (replace with your own code)      |
| [`src/dev_tools.rs`](./src/dev_tools.rs)           | Dev tools for dev builds (press \` aka backtick to toggle)         |
| [`src/screens/`](./src/screens)                    | Splash screen, title screen, gameplay screen, etc.                 |
| [`src/theme/`](./src/theme)                        | Reusable UI widgets & theming                                      |

Feel free to move things around however you want, though.

> [!Tip]
> Be sure to check out the [3rd-party tools](./docs/tooling.md) we recommend!

## Run your game

Running your game locally is very simple:

- Use `cargo run` to run a native dev build.
- Use [`trunk serve`](https://trunkrs.dev/) to run a web dev build.

If you're using [VS Code](https://code.visualstudio.com/), this template comes with a [`.vscode/tasks.json`](./.vscode/tasks.json) file.

<details>
  <summary>Run release builds</summary>

- Use `cargo run --profile release-native --no-default-features` to run a native release build.
- Use `trunk serve --release --no-default-features` to run a web release build.

</details>

<details>
  <summary>Linux dependencies</summary>

If you are using Linux, make sure you take a look at Bevy's [Linux dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md).
Note that this template enables Wayland support, which requires additional dependencies as detailed in the link above.
Wayland is activated by using the `bevy/wayland` feature in the [`Cargo.toml`](./Cargo.toml).

</details>

<details>
    <summary>(Optional) Improve your compile times</summary>

[`.cargo/config_fast_builds.toml`](./.cargo/config_fast_builds.toml) contains documentation on how to set up your environment to improve compile times.
After you've fiddled with it, rename it to `.cargo/config.toml` to enable it.

</details>

## Release your game

This template uses [GitHub workflows](https://docs.github.com/en/actions/using-workflows) to run tests and build releases.
See [Workflows](./docs/workflows.md) for more information.

## Known Issues

There are some known issues in Bevy that require some arcane workarounds.
To keep this template simple, we have opted not to include those workarounds.
You can read about them in the [Known Issues](./docs/known-issues.md) document.

## License

The source code in this repository is licensed under any of the following at your option:

- [CC0-1.0 License](./LICENSE-CC0-1.0.txt)
- [MIT License](./LICENSE-MIT.txt)
- [Apache License, Version 2.0](./LICENSE-Apache-2.0.txt)

The CC0 license explicitly does not waive patent rights, but we confirm that we hold no patent rights to anything presented in this repository.

## Credits

The [assets](./assets) in this repository are all 3rd-party. See the [credits screen](./src/screens/credits.rs) for more information.
