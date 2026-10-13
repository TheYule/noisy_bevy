use std::path::Path;

use bevy::{
    app::{App, Plugin},
    asset::{Assets, Handle, uuid_handle},
    shader::{Shader, ShaderImport},
};

/// Adds noise library as a wesl import
///
/// General functionality can be imported through:
///
/// ```wesl
/// import noisy_bevy;
/// ```
pub struct NoisyShaderPlugin;

impl Plugin for NoisyShaderPlugin {
    fn build(&self, app: &mut App) {
        // workaround: inlined load_internal_asset to change the import path
        let mut shader = Shader::from_wesl(
            include_str!("../assets/noisy_bevy.wesl"),
            Path::new(file!())
                .parent()
                .unwrap()
                .join("../assets/noisy_bevy.wesl")
                .to_string_lossy(),
        );
        shader.import_path = ShaderImport::Custom("noisy_bevy".into());

        let mut assets = app.world_mut().resource_mut::<Assets<_>>();
        assets.insert(NOISY_SHADER_HANDLE.id(), shader).unwrap();
    }
}

const NOISY_SHADER_HANDLE: Handle<Shader> = uuid_handle!("9e85d206-7851-41d9-a04f-c4879ddd7143");
