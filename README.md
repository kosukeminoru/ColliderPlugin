# ColliderPlugin
For the Scene to apply a collider to: 
Add an AddCollider component to the entity. Boolean is toggle for perfect mesh or approximation, handle is Handle<Scene> 

.insert(AddCollider::new(true, handle))

let mut handle: Handle<Scene> =
        asset_server.load("path-to-my.glb#Scene0");
        
