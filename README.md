# LocalStack MCP Server for Zed

A [Zed](https://zed.dev) extension that adds the [LocalStack MCP server](https://github.com/localstack/localstack-mcp-server) to the Agent Panel. The agent can then start and stop LocalStack, deploy CDK, Terraform, and SAM projects, run AWS commands against the local cloud, read logs, inject chaos, and manage Cloud Pods.

## Install

1. Open Zed and go to **Zed > Extensions** (`cmd-shift-x` on macOS, `ctrl-shift-x` on Linux and Windows).
2. Search for **LocalStack MCP Server** and click **Install**.
3. Open the Agent Panel, click the settings icon, find **LocalStack MCP Server** and click **Configure**.
4. Paste your [LocalStack Auth Token](https://app.localstack.cloud/workspace/auth-tokens) into `localstack_auth_token`.

Docker must be installed and running. The server creates and manages the LocalStack container through the Docker API. You do not need Node.js or the LocalStack CLI on your PATH: Zed downloads the npm package and runs it with its bundled Node.js.

## Settings

The extension reads its settings from `context_servers.mcp-server-localstack.settings` in your Zed `settings.json`:

```json
{
  "context_servers": {
    "mcp-server-localstack": {
      "settings": {
        "localstack_auth_token": "<YOUR_LOCALSTACK_AUTH_TOKEN>",
        "env": {
          "LOCALSTACK_IMAGE_NAME": "localstack/localstack-pro:latest",
          "DEBUG": "1"
        }
      }
    }
  }
}
```

| Key | Required | Description |
| --- | --- | --- |
| `localstack_auth_token` | yes | Passed to the server as `LOCALSTACK_AUTH_TOKEN`. Every LocalStack MCP tool needs it. |
| `env` | no | Extra environment variables for the MCP server and the LocalStack container. See the [server configuration reference](https://github.com/localstack/localstack-mcp-server#localstack-configuration). |

The `env` map cannot override `LOCALSTACK_AUTH_TOKEN`; the token setting always wins.

## Server version

The extension tracks the latest published `@localstack/localstack-mcp-server` on npm. On each start it compares the installed version with the latest one and updates when they differ. If npm cannot be reached and a copy is already installed, that copy is used.

To pin a version or run the server another way (for example the Docker image), use Zed's built-in `command` override for the context server:

```json
{
  "context_servers": {
    "mcp-server-localstack": {
      "command": {
        "path": "npx",
        "args": ["-y", "@localstack/localstack-mcp-server@0.6.0"],
        "env": { "LOCALSTACK_AUTH_TOKEN": "<YOUR_LOCALSTACK_AUTH_TOKEN>" }
      }
    }
  }
}
```

## Develop

Requirements: Rust via [rustup](https://rustup.rs) (Zed installs the `wasm32-wasip2` target on demand) and Zed.

```sh
cargo test
cargo clippy --target wasm32-wasip2 -- -D warnings
cargo build --target wasm32-wasip2 --release
```

To try the extension in Zed, open the Extensions page, click **Install Dev Extension**, and select this directory. Run `zed --foreground` from a terminal to see extension logs.

Layout:

- `extension.toml`: Zed manifest and the `mcp-server-localstack` context server entry.
- `src/lib.rs`: extension entry point, wires settings and the npm package into a `Command`.
- `src/settings.rs`: settings schema, validation, and the environment passed to the server.
- `src/server_package.rs`: installs or updates the npm package in Zed's work directory.
- `configuration/`: instructions and default settings shown in the Agent Panel.

## Release

The first release is a manual pull request against [`zed-industries/extensions`](https://github.com/zed-industries/extensions):

1. Fork `zed-industries/extensions` (a personal fork lets Zed staff push fixes to the PR).
2. Add this repository as a submodule and register it:

   ```sh
   git submodule add https://github.com/localstack/zed-localstack-extension.git extensions/mcp-server-localstack
   ```

   ```toml
   [mcp-server-localstack]
   submodule = "extensions/mcp-server-localstack"
   version = "0.1.0"
   ```

3. Run `pnpm sort-extensions`, commit, and open the PR. One extension per PR; reply to review feedback within three weeks.

For later versions, bump `version` in `extension.toml` and `Cargo.toml`, tag the commit `vX.Y.Z`, and the release workflow opens the update PR from the `localstack/zed-extensions` fork (needs a `COMMITTER_TOKEN` secret). Without the workflow, run `git submodule update --remote extensions/mcp-server-localstack` in the fork and bump the version in `extensions.toml` by hand.

## License

[Apache License 2.0](./LICENSE)
