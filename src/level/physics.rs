use crate::prelude::*;

#[derive(
  SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
pub enum PhysicsSystems {
  PopulateGrid,
  Update,
}

pub fn plugin(app: &mut App) {
  app.configure_sets(
    PostUpdate,
    (PhysicsSystems::PopulateGrid, PhysicsSystems::Update)
      .chain()
      .before(TransformSystems::Propagate),
  );

  app
    .init_resource::<SpatialGrid>()
    .add_systems(
      PostUpdate,
      (clear_grid, populate_grid).chain().in_set(PhysicsSystems::PopulateGrid),
    )
    .add_systems(Update, debug_spatial_grid.run_if(in_debug(D::L1)));
}

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Layer(u32);

impl Layer {
  pub const NONE: u32 = 0;
  pub const PLAYER: u32 = 1 << 0;
  pub const ENEMY: u32 = 1 << 1;
  pub const PROJECTILE: u32 = 1 << 2;
  pub const PICKUP: u32 = 1 << 3;
  pub const ALL: u32 = u32::MAX;
}

#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq, Debug)]
pub struct CollisionLayers {
  pub groups: u32,
  pub masks: u32,
}

impl CollisionLayers {
  pub const fn new(groups: u32, masks: u32) -> Self {
    Self { groups, masks }
  }

  #[inline(always)]
  pub const fn interacts_with(&self, other: &Self) -> bool {
    (self.groups & other.masks) != 0 && (self.masks & other.groups) != 0
  }
}

impl Default for CollisionLayers {
  fn default() -> Self {
    CollisionLayers::new(Layer::ALL, Layer::ALL)
  }
}

#[derive(Component, Reflect, Debug)]
#[require(CollisionLayers)]
pub struct Collider(pub f32);

#[derive(Event)]
pub struct CollisionEvent(Entity, Entity);

const CELL_SIZE: f32 = 2.0;

pub struct GridEntry {
  pub entity: Entity,
  pub radius: f32,
  pub pos: Vec2,
}

#[derive(Resource, Default)]
pub struct SpatialGrid {
  pub cells: HashMap<(i32, i32), Vec<GridEntry>>,
}

impl SpatialGrid {
  pub fn pos_to_cell(pos: Vec2) -> (i32, i32) {
    ((pos.x / CELL_SIZE).floor() as i32, (pos.y / CELL_SIZE).floor() as i32)
  }

  pub fn query_radius(
    &self,
    pos: Vec2,
    radius: f32,
  ) -> impl Iterator<Item = &GridEntry> + '_ {
    let (min_x, min_y) = Self::pos_to_cell(pos - Vec2::splat(radius));
    let (max_x, max_y) = Self::pos_to_cell(pos + Vec2::splat(radius));

    (min_x..=max_x)
      .flat_map(move |x| (min_y..=max_y).map(move |y| (x, y)))
      .filter_map(move |key| self.cells.get(&key))
      .flatten()
  }

  pub fn query_nearest(
    &self,
    pos: Vec2,
    radius: f32,
  ) -> impl Iterator<Item = &GridEntry> + '_ {
    let (cx, cy) = Self::pos_to_cell(pos);
    let (min_x, min_y, max_x, max_y) = (cx - 1, cy - 1, cx + 1, cy + 1);

    (min_x..=max_x)
      .flat_map(move |x| (min_y..=max_y).map(move |y| (x, y)))
      .filter_map(move |key| self.cells.get(&key))
      .flatten()
  }
}

fn clear_grid(mut grid: ResMut<SpatialGrid>) {
  for cell in grid.cells.values_mut() {
    cell.clear();
  }
}

fn populate_grid(
  mut grid: ResMut<SpatialGrid>,
  query: Query<(Entity, &Transform2D, &Collider)>,
) {
  for (entity, &Transform2D { translation: pos, .. }, &Collider(radius)) in
    query.iter()
  {
    grid
      .cells
      .entry(SpatialGrid::pos_to_cell(pos))
      .or_insert_with(Vec::new)
      .push(GridEntry { entity, radius, pos });
  }
}

pub fn debug_spatial_grid(grid: Res<SpatialGrid>, mut gizmos: Gizmos) {
  for (&(cx, cy), entities) in &grid.cells {
    let count = entities.len();
    if count == 0 {
      continue;
    }

    let center = Vec2::new(
      cx as f32 * CELL_SIZE + CELL_SIZE * 0.5,
      cy as f32 * CELL_SIZE + CELL_SIZE * 0.5,
    );

    let intensity = (count as f32 / 5.0).clamp(0.2, 2.0);
    let color = Color::LinearRgba(LinearRgba::new(0.0, intensity, 0.0, 1.0)); // Неоновый зеленый

    gizmos.rect_2d(center, Vec2::splat(CELL_SIZE - 0.1), color);
  }
}
