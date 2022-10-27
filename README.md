# ColliderPlugin
Applying a Collider:
Add an AddCollider component to the entity. Boolean is toggle for perfect mesh or approximation, handle is Handle<Scene> 
Plugin should detect when a new AddCollider component is created.

.insert(AddCollider::new(true, handle))

let mut handle: Handle<Scene> =
        asset_server.load("path-to-my.glb#Scene0");
    
