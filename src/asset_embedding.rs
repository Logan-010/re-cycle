use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::prelude::*;
use std::path::Path;

macro_rules! embed_assets {
    ($app:expr, $( $asset:expr ),*) => {
        {
            let assets = $app.world_mut().resource_mut::<EmbeddedAssetRegistry>();

            $(
                let temp = "../embedded-assets/";
                let asset_name = Path::new(&$asset[temp.len()..]);
                let asset_bytes = include_bytes!($asset);

                assets.insert_asset($asset.into(), asset_name, asset_bytes);
            )*
        }
    };
}

pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        embed_assets!(app, "../embedded-assets/splash.png");
    }
}
