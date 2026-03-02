use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_systems(Startup, init_enemy_assets);
}

#[derive(Resource)]
pub struct EnemyAssets {
  pub swarm: (Handle<Mesh>, Handle<ColorMaterial>),
  pub pulsar: (Handle<Mesh>, Handle<ColorMaterial>),
  pub singularity: (Handle<Mesh>, Handle<ColorMaterial>),
  pub constellation_node: (Handle<Mesh>, Handle<ColorMaterial>),
}

fn init_enemy_assets(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let swarm_mesh = meshes.add(RegularPolygon::new(1.0, 3));
  let swarm_mat =
    materials.add(Color::LinearRgba(LinearRgba::new(0.0, 2.0, 8.0, 1.0)));

  let pulsar_mesh = meshes.add(RegularPolygon::new(1.0, 8));
  let pulsar_mat =
    materials.add(Color::LinearRgba(LinearRgba::new(8.0, 2.0, 0.0, 1.0)));

  let singularity_mesh = meshes.add(Circle::new(1.0));
  let singularity_mat = materials.add(Color::BLACK);

  let const_mesh = meshes.add(Circle::new(1.0));
  let const_mat =
    materials.add(Color::LinearRgba(LinearRgba::new(6.0, 0.0, 8.0, 1.0)));

  commands.insert_resource(EnemyAssets {
    swarm: (swarm_mesh, swarm_mat),
    pulsar: (pulsar_mesh, pulsar_mat),
    singularity: (singularity_mesh, singularity_mat),
    constellation_node: (const_mesh, const_mat),
  });
}
