The LocalStack MCP server lets the agent start LocalStack, deploy infrastructure, inspect state and logs, and run AWS commands against your local cloud.

Before you start:

1. Install and start Docker. The server manages the LocalStack container through the Docker API.
2. Get a LocalStack Auth Token from the [LocalStack web app](https://app.localstack.cloud/workspace/auth-tokens) and paste it into `localstack_auth_token` below.

Zed downloads the `@localstack/localstack-mcp-server` npm package and runs it with its bundled Node.js. You do not need Node.js or the LocalStack CLI on your PATH.

Optional: add entries to `env` to configure LocalStack, for example `LOCALSTACK_IMAGE_NAME`, `GATEWAY_LISTEN`, `DEBUG`, or the `AWS_*` source credentials for the AWS Replicator tool. See the [server README](https://github.com/localstack/localstack-mcp-server#localstack-configuration) for the full list.
