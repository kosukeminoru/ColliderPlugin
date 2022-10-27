use bevy::prelude::*;
use bevy::render::render_asset::RenderAsset;
use bevy_rapier3d::prelude::ComputedColliderShape::ConvexDecomposition;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Default)]
pub struct ColliderBuilderPlugin;

impl Plugin for ColliderBuilderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system(mesh_collider_create);
        app.add_system(mesh_collider_transform);
        app.insert_resource(Run((false, false, false)));
    }
}

#[derive(Component)]
pub struct BetterParent(Entity);

#[derive(Component)]
pub struct AddCollider((bool, Handle<Scene>));
impl AddCollider {
    pub fn new(bool: bool, handle: Handle<Scene>) -> AddCollider {
        AddCollider((bool, handle))
    }
}

pub struct Run((bool, bool, bool));

pub fn mesh_collider_transform(
    mut commands: Commands,
    q_child: Query<(Entity, &BetterParent, &mut Transform, &GlobalTransform)>,
    mut ass_world: ResMut<Assets<Scene>>,
    asset_server: Res<AssetServer>,
    mut run: ResMut<Run>,
    parent_mesh_transform: Query<(Entity, &Handle<Mesh>, &GlobalTransform)>,
    objects: Query<&AddCollider>,
) {
    // If mesh has been rendered and one loop has passed, and this function has not been called
    if run.0 .0 & !run.0 .1 & run.0 .2 {
        for tagged_entities in objects.iter() {
            let collider_info = tagged_entities.0.clone();
            match ass_world.get_mut(&collider_info.1) {
                Some(world) => {
                    let mut query_one = world
                        .world
                        .query::<(Entity, &Handle<Mesh>, &GlobalTransform)>();

                    for (e, bp, mut t, mut gz) in q_child.iter() {
                        let prnt = query_one.get(&mut world.world, bp.0);
                        for (a, b, c) in parent_mesh_transform.iter() {
                            if b == prnt.unwrap().1 {
                                commands
                                    .entity(e)
                                    .insert_bundle(TransformBundle::from(Transform::from(*c)));

                                break;
                            }
                        }
                    }
                    run.0 .1 = true;
                }
                None => (),
            }
        }
    } else if run.0 .0 & !run.0 .1 & !run.0 .2 {
        run.0 .2 = true;
    }
}

pub fn mesh_collider_create(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ass_world: ResMut<Assets<Scene>>,
    mut ss: Res<Assets<bevy::gltf::Gltf>>,
    mut run: ResMut<Run>,
    objects: Query<&AddCollider>,
) {
    for tagged_entities in objects.iter() {
        let collider_info = tagged_entities.0.clone();
        if !run.0 .0 {
            match ass_world.get_mut(&collider_info.1) {
                Some(world) => {
                    let mut query_one = world.world.query::<(Entity, &Handle<Mesh>)>();
                    for (entity, mesh) in query_one.iter_mut(&mut world.world) {
                        let parent = commands.entity(entity).id();
                        let mut collider = Collider::default();
                        if collider_info.0 {
                            let collider = Collider::from_bevy_mesh(
                                &meshes.get(&mesh).unwrap().extract_asset(),
                                &ComputedColliderShape::TriMesh,
                            )
                            .unwrap();
                        } else {
                            let collider = Collider::from_bevy_mesh(
                                &meshes.get(&mesh).unwrap().extract_asset(),
                                &ConvexDecomposition(VHACDParameters::default()),
                            )
                            .unwrap();
                        }
                        let child_mesh = commands
                            .spawn()
                            .insert(
                                Collider::from_bevy_mesh(
                                    &meshes.get(&mesh).unwrap().extract_asset(),
                                    &ComputedColliderShape::TriMesh,
                                )
                                .unwrap(),
                            )
                            .insert(Friction::new(1000.0))
                            .insert(BetterParent(parent))
                            .insert_bundle(TransformBundle::default())
                            .id();
                    }
                    run.0 .0 = true;
                }

                None => (run.0 .0 = false),
            }
        }
    }
}
