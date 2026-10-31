use bevy::prelude::*;
use bevy::reflect::CreateTypeData;

#[derive(Clone)]
pub struct ReflectConfig;

impl<T: Resource + Reflect> CreateTypeData<T> for ReflectConfig {
    fn create_type_data((): ()) -> Self {
        Self
    }
}
