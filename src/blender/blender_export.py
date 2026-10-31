import addon_utils, bpy, sys

out = sys.argv[sys.argv.index('--') + 1]

def has_skein_data(item):
    # Blender 5+ keeps addon-defined properties in separate system storage.
    props = item.bl_system_properties_get() if hasattr(item, 'bl_system_properties_get') else item
    return bool(props and props.get('skein_two'))

tagged = [item.name
          for data in (bpy.data.objects, bpy.data.meshes, bpy.data.materials,
                       bpy.data.scenes, bpy.data.cameras, bpy.data.lights)
          for item in data if has_skein_data(item)]
if tagged and not hasattr(bpy.types.Scene, 'skein_extension_properties'):
    module = next((m.__name__ for m in addon_utils.modules()
                   if m.__name__.rsplit('.', 1)[-1] == 'bevy_skein'), None)
    if module is None:
        raise RuntimeError(
            f'{bpy.data.filepath} has skein components (on {tagged}) but the skein '
            'addon is not installed in this Blender; the components would be dropped')
    # default_set: the glTF exporter only runs hooks of add-ons listed as enabled.
    addon_utils.enable(module, default_set=True)
    # Its load_post handler already missed the file load; rebuild the registry.
    bpy.ops.wm.reload_skein_registry()
bpy.ops.export_scene.gltf(filepath=out, export_format='GLB', export_apply=True)
