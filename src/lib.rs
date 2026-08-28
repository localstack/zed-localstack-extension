mod server_package;
mod settings;

use settings::LocalStackSettings;
use zed_extension_api::{
    self as zed, serde_json, settings::ContextServerSettings, Command, ContextServerConfiguration,
    ContextServerId, Project, Result,
};

const SERVER_ID: &str = "mcp-server-localstack";

struct LocalStackExtension;

impl zed::Extension for LocalStackExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = load_settings(project)?;
        let entry_point = server_package::ensure_installed()?;
        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![entry_point.to_string_lossy().into_owned()],
            env: settings.environment(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let settings_schema = serde_json::to_string(&schemars::schema_for!(LocalStackSettings))
            .map_err(|error| error.to_string())?;
        Ok(Some(ContextServerConfiguration {
            installation_instructions: include_str!(
                "../configuration/installation_instructions.md"
            )
            .to_string(),
            default_settings: include_str!("../configuration/default_settings.jsonc").to_string(),
            settings_schema,
        }))
    }
}

fn load_settings(project: &Project) -> Result<LocalStackSettings> {
    let settings = ContextServerSettings::for_project(SERVER_ID, project)?;
    let value = settings.settings.ok_or_else(|| {
        format!("missing settings for `{SERVER_ID}`: add `localstack_auth_token` under `context_servers.{SERVER_ID}.settings`")
    })?;
    LocalStackSettings::from_json(value)
}

zed::register_extension!(LocalStackExtension);
