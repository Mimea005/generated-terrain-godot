tool
extends EditorPlugin

func _enter_tree():
    # Get the base icon from the current editor interface/theme
    var gui = get_editor_interface().get_base_control()
    var terrain_icon = gui.get_icon("MeshInstance", "EditorIcons")

    # Alternatively, preload to a custom icon at the following
    # var node_icon preload("res://icon_ferris.png")

    add_custom_type(
        "CameraController",
        "Spatial",
        preload("res://addons/rust_nodes/CameraController.gdns"),
        gui.get_icon("Camera", "EditorIcons")
    )

    # Add any additional custom nodes here here.
    add_custom_type(
        "FlatTerrain",
        "MeshInstance",
        preload("res://addons/rust_nodes/FlatTerrain.gdns"),
        terrain_icon
    )


func _exit_tree():
    remove_custom_type("FlatTerrain")
    # Add a remove for each registered custom type to clean up

