use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::BTreeMap;

pub const AUTH_TOKEN_VARIABLE: &str = "LOCALSTACK_AUTH_TOKEN";
const TOKEN_PLACEHOLDER: &str = "YOUR_LOCALSTACK_AUTH_TOKEN";

/// Settings a user writes under `context_servers.mcp-server-localstack.settings`.
#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
pub struct LocalStackSettings {
    /// LocalStack Auth Token. Required by every LocalStack MCP tool.
    pub localstack_auth_token: String,
    /// Extra environment variables for the MCP server and the LocalStack container.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

impl LocalStackSettings {
    pub fn from_json(value: zed_extension_api::serde_json::Value) -> Result<Self, String> {
        let settings: Self = zed_extension_api::serde_json::from_value(value)
            .map_err(|error| format!("invalid LocalStack settings: {error}"))?;
        settings.validate()?;
        Ok(settings)
    }

    fn validate(&self) -> Result<(), String> {
        let token = self.localstack_auth_token.trim();
        if token.is_empty() || token == TOKEN_PLACEHOLDER {
            return Err(
                "set `localstack_auth_token` in the LocalStack MCP server settings \
                 (get a token at https://app.localstack.cloud/workspace/auth-tokens)"
                    .to_string(),
            );
        }
        Ok(())
    }

    /// Environment for the server process. The token always wins over `env`.
    pub fn environment(&self) -> Vec<(String, String)> {
        let mut environment: Vec<(String, String)> = self
            .env
            .iter()
            .filter(|(name, _)| name.as_str() != AUTH_TOKEN_VARIABLE)
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect();
        environment.push((
            AUTH_TOKEN_VARIABLE.to_string(),
            self.localstack_auth_token.trim().to_string(),
        ));
        environment
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::serde_json::json;

    #[test]
    fn parses_token_and_env() {
        let settings = LocalStackSettings::from_json(json!({
            "localstack_auth_token": "ls-token",
            "env": { "DEBUG": "1" }
        }))
        .unwrap();

        assert_eq!(settings.localstack_auth_token, "ls-token");
        assert_eq!(settings.env.get("DEBUG").unwrap(), "1");
    }

    #[test]
    fn env_defaults_to_empty() {
        let settings =
            LocalStackSettings::from_json(json!({ "localstack_auth_token": "ls-token" })).unwrap();

        assert!(settings.env.is_empty());
    }

    #[test]
    fn rejects_missing_token() {
        let error = LocalStackSettings::from_json(json!({})).unwrap_err();

        assert!(error.contains("invalid LocalStack settings"));
    }

    #[test]
    fn rejects_placeholder_token() {
        let error = LocalStackSettings::from_json(json!({
            "localstack_auth_token": "YOUR_LOCALSTACK_AUTH_TOKEN"
        }))
        .unwrap_err();

        assert!(error.contains("set `localstack_auth_token`"));
    }

    #[test]
    fn rejects_blank_token() {
        let error =
            LocalStackSettings::from_json(json!({ "localstack_auth_token": "   " })).unwrap_err();

        assert!(error.contains("set `localstack_auth_token`"));
    }

    #[test]
    fn environment_contains_env_and_token() {
        let settings = LocalStackSettings::from_json(json!({
            "localstack_auth_token": " ls-token ",
            "env": { "DEBUG": "1", "GATEWAY_LISTEN": ":4566" }
        }))
        .unwrap();

        assert_eq!(
            settings.environment(),
            vec![
                ("DEBUG".to_string(), "1".to_string()),
                ("GATEWAY_LISTEN".to_string(), ":4566".to_string()),
                (AUTH_TOKEN_VARIABLE.to_string(), "ls-token".to_string()),
            ]
        );
    }

    #[test]
    fn env_cannot_override_token() {
        let settings = LocalStackSettings::from_json(json!({
            "localstack_auth_token": "real",
            "env": { "LOCALSTACK_AUTH_TOKEN": "fake" }
        }))
        .unwrap();

        let tokens: Vec<_> = settings
            .environment()
            .into_iter()
            .filter(|(name, _)| name == AUTH_TOKEN_VARIABLE)
            .collect();
        assert_eq!(
            tokens,
            vec![(AUTH_TOKEN_VARIABLE.to_string(), "real".to_string())]
        );
    }
}
