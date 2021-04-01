use bevy::prelude::*;
use crate::position::Position;

pub const SQUARE_WIDTH: i32 = 50;

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Shape {
    pub entities: Vec<Entity>
}

impl Shape {}

pub struct ShapeBuilder {
    pub positions: Vec<Position>
}

impl ShapeBuilder {
    /// * * *
    /// * * *
    /// * * *
    pub fn new_board(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32, nb_cols: i32, nb_rows: i32) {
        let mut board = Self { positions: vec![] };
        for i in 0..nb_rows {
            board.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), nb_cols));
        };
        board.build(commands, material);
    }

    /// *
    /// *
    /// *
    pub fn new_rectangle_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut rectangle = Self { positions: vec![] };
        for i in 0..3 {
            rectangle.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), 1));
        };
        rectangle.build(commands, material);
    }

    /// *
    /// *
    /// * *
    pub fn new_l_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut piece = Self { positions: vec![] };
        for i in 0..3 {
            piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), 1));
        };
        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x + SQUARE_WIDTH, start_y, 1));
        piece.build(commands, material);
    }

    /// * *
    ///   * *
    pub fn new_z_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut piece = Self { positions: vec![] };

        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + SQUARE_WIDTH, 2));
        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x + SQUARE_WIDTH, start_y, 2));

        piece.build(commands, material);
    }

    /// *
    /// * *
    pub fn new_corner_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut piece = Self { positions: vec![] };

        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y, 2));
        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + SQUARE_WIDTH, 1));

        piece.build(commands, material);
    }

    /// *
    pub fn new_dot_square_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut piece = Self { positions: vec![] };

        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y, 1));
        piece.build(commands, material);
    }

    fn build(&mut self, commands: &mut Commands, materials: Handle<ColorMaterial>) {
        let entities: Vec<Entity> = self.positions.iter_mut().map(|position| {
            commands.spawn(
                SpriteBundle {
                    material: materials.clone(),
                    sprite: Sprite::new(Vec2::new(SQUARE_WIDTH as f32, SQUARE_WIDTH as f32)),
                    transform: Transform::from_translation(position.to_vec()),
                    ..Default::default()
                }
            )
                .with(*position)
                .current_entity()
                .unwrap()
        }).collect();

        let shape = Shape {
            entities
        };
        commands.with(shape);
        // shape
    }

    fn new_horizontal_rectangle(start_x: i32, start_y: i32, length: i32) -> Vec<Position> {
        let mut squares = vec![];
        for i in 0..length {
            squares.push(Position {
                x: start_x + (i * SQUARE_WIDTH),
                y: start_y,
            });
        }
        squares
    }
}

#[cfg(test)]
mod tests {
    use bevy::asset::HandleId;

    use super::*;

    /// LE HELPER
    // use std::sync::Once;
    // use rstest::rstest;
    // static INIT: Once = Once::new();

    // pub fn setup() -> () {
    //     INIT.call_once(|| {
    //         // initialization code here
    //         // jsp ce qu'on init ici
    //     });
    // }

    // #[rstest]
    // fn should_success(setup: ()) {
    //     // bon là par contre j'ai fait un c/c de SO du coup je vois pas
    // }

    // struct TestHelper {
    //     world: World,
    //     resources: Resources,
    //     commands: Commands,
    //     materials: Handle<ColorMaterial>,
    // }
    //
    // impl TestHelper {
    //     fn setup() -> TestHelper {
    //         let mut world = World::default();
    //         let mut resources = Resources::default();
    //         let mut commands = Commands::default();
    //         commands.set_entity_reserver(world.get_entity_reserver());
    //         let mut materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());
    //         TestHelper {
    //             world,
    //             resources,
    //             commands,
    //             materials,
    //         }
    //     }
    // }

    #[test]
    fn test_build_board() {
        // Given
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut commands = Commands::default();
        commands.set_entity_reserver(world.get_entity_reserver());
        let mut materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // *
        ShapeBuilder::new_board(&mut commands, materials, 0, 0, 1, 2);
        commands.apply(&mut world, &mut resources);

        // Then
        let results = world.query::<&Position>().collect::<Vec<_>>();
        assert_eq!(results, vec![&Position { x: 0, y: 0 }, &Position { x: 0, y: SQUARE_WIDTH }]);
    }

    #[test]
    fn test_build_rectangle_piece() {
        // Given
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut commands = Commands::default();
        commands.set_entity_reserver(world.get_entity_reserver());
        let mut materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // *
        // *
        ShapeBuilder::new_rectangle_piece(&mut commands, materials, 0, 0);
        commands.apply(&mut world, &mut resources);

        // Then
        let results = world.query::<&Position>().collect::<Vec<_>>();
        assert_eq!(results, vec![
            &Position { x: 0, y: 0 },
            &Position { x: 0, y: SQUARE_WIDTH },
            &Position { x: 0, y: 2 * SQUARE_WIDTH }
        ]);
    }

    #[test]
    fn test_build_l_piece() {
        // Given
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut commands = Commands::default();
        commands.set_entity_reserver(world.get_entity_reserver());
        let mut materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // *
        // * *
        ShapeBuilder::new_l_piece(&mut commands, materials, 0, 0);
        commands.apply(&mut world, &mut resources);

        // Then
        let results = world.query::<&Position>().collect::<Vec<_>>();
        assert_eq!(results, vec![
            &Position { x: 0, y: 0 },
            &Position { x: 0, y: SQUARE_WIDTH },
            &Position { x: 0, y: 2 * SQUARE_WIDTH },
            &Position { x: SQUARE_WIDTH, y: 0 }
        ]);
    }

    #[test]
    fn test_build_z_piece() {
        // Given
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut commands = Commands::default();
        commands.set_entity_reserver(world.get_entity_reserver());
        let mut materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // * *
        //   * *
        ShapeBuilder::new_z_piece(&mut commands, materials, 0, 0);
        commands.apply(&mut world, &mut resources);

        // Then
        let results = world.query::<&Position>().collect::<Vec<_>>();
        assert_eq!(results, vec![
            &Position { x: 0, y: SQUARE_WIDTH },
            &Position { x: SQUARE_WIDTH, y: SQUARE_WIDTH },
            &Position { x: SQUARE_WIDTH, y: 0 },
            &Position { x: 2 * SQUARE_WIDTH, y: 0 }
        ]);
    }

    #[test]
    fn test_build_corner_piece() {
        // Given
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut commands = Commands::default();
        commands.set_entity_reserver(world.get_entity_reserver());
        let mut materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // * *
        ShapeBuilder::new_corner_piece(&mut commands, materials, 0, 0);
        commands.apply(&mut world, &mut resources);

        // Then
        let results = world.query::<&Position>().collect::<Vec<_>>();
        assert_eq!(results, vec![
            &Position { x: 0, y: 0 },
            &Position { x: SQUARE_WIDTH, y: 0 },
            &Position { x: 0, y: SQUARE_WIDTH }
        ]);
    }

    #[test]
    fn test_build_dot_square_piece() {
        // Given
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut commands = Commands::default();
        commands.set_entity_reserver(world.get_entity_reserver());
        let mut materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        ShapeBuilder::new_dot_square_piece(&mut commands, materials, 0, 0);
        commands.apply(&mut world, &mut resources);

        // Then
        let results = world.query::<&Position>().collect::<Vec<_>>();
        assert_eq!(results, vec![&Position { x: 0, y: 0 }]);
    }
}
