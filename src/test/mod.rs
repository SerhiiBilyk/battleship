use super::*;
use field::{status_u8, Direction, Draw, Point, ShipDirection, Status};

use std::time::{Duration, SystemTime};
use utils::{generate_all_empty_points, random_number};
#[cfg(test)]
mod test {

    use super::{
        generate_all_empty_points, random_number, status_u8, Direction, Draw, GameField, Point,
        ShipDirection, Status,
    };
    use std::time::{Duration, SystemTime};
    const ALL_SHIPS: u8 = 20;


    fn point_sum(field: GameField, status: Status) -> u8 {
        field.field.iter().flatten().fold(0, |acc, elem| {
            if *elem == status {
                acc + status_u8(*elem)
            } else {
                acc
            }
        })
    }

    #[test]
    fn create_field() {
        let GameField { field, .. } = super::GameField::new();
        assert_eq!(field, [[Status::Empty; 12]; 12]);
    }
    #[test]
    fn get_ships() {
        let GameField { ships, .. } = super::GameField::new();
        let arr = [1, 2, 3, 4];
        let length = arr.len();
        for (i, &elem) in arr.iter().enumerate() {
            let index = length - (1 + i);
            let to_be = &arr[index];
            assert_eq!(ships.get(&elem).unwrap(), to_be);
        }
    }
    #[test]
    fn reduce_ships() {
        let mut field = super::GameField::new();
        for index in 1..5 {
            field.reduce_ships(&index);
        }
        assert_eq!(*field.ships.get(&1).unwrap(), 3);
        assert_eq!(*field.ships.get(&2).unwrap(), 2);
        assert_eq!(*field.ships.get(&3).unwrap(), 1);
        assert_eq!(*field.ships.get(&4).unwrap(), 0);
    }
    #[test]
    fn check_permission_positive() {
        let mut field = super::GameField::new();
        let permission = field.check_permission(4);
        assert_eq!(permission, true);
    }
    #[test]
    fn check_permission_negative() {
        let mut field = super::GameField::new();
        field.reduce_ships(&4);
        let permission = field.check_permission(4);
        assert_eq!(permission, false);
    }
    #[test]
    fn test_generate_all_empty_points() {
        {
            let mut field = super::GameField::new();
            let size = 4;
            field.create_ship(
                size,
                &ShipDirection::Vertical,
                Some(Point { row: 5, column: 6 }),
                random_number,
            );
            let empty_points = generate_all_empty_points(field.field);
            println!("EMpty points {}", empty_points.len());
            field.show();
            assert_eq!(empty_points.len(), 82);
        }
        {
            let mut field = super::GameField::new();
            let size = 1;
            field.create_ship(
                size,
                &ShipDirection::Vertical,
                Some(Point { row: 5, column: 6 }),
                random_number,
            );
            let empty_points = generate_all_empty_points(field.field);
            println!("EMpty points {}", empty_points.len());
            field.show();
            assert_eq!(empty_points.len(), 91);
        }
        {
            let mut field = super::GameField::new();
            field.create_ship(
                4,
                &ShipDirection::Vertical,
                Some(Point { row: 5, column: 6 }),
                random_number,
            );
            field.create_ship(
                3,
                &ShipDirection::Vertical,
                Some(Point { row: 2, column: 8 }),
                random_number,
            );
            let empty_points = generate_all_empty_points(field.field);
            println!("EMpty points {}", empty_points.len());
            field.show();
            assert_eq!(empty_points.len(), 69);
        }
        {
            let mut field = super::GameField::new();
            field.create_ship(
                4,
                &ShipDirection::Vertical,
                Some(Point { row: 2, column: 1 }),
                random_number,
            );
            field.create_ship(
                3,
                &ShipDirection::Vertical,
                Some(Point { row: 2, column: 3 }),
                random_number,
            );
            field.create_ship(
                3,
                &ShipDirection::Vertical,
                Some(Point { row: 2, column: 5 }),
                random_number,
            );
            let empty_points = generate_all_empty_points(field.field);
            println!("EMpty points {}", empty_points.len());
            field.show();
            assert_eq!(empty_points.len(), 68);
        }

    }
    #[test]
    fn generate_random_field() {
        let start = SystemTime::now();
        (0..2000).collect::<Vec<u16>>().iter().for_each(|_| {
            let mut field = super::GameField::new();
            field.generate_random_field(random_number);
            let sum = point_sum(field, Status::Ship);
            assert_eq!(sum, ALL_SHIPS);
        });

        let end = SystemTime::now();
        let test_time = end
            .duration_since(start)
            .expect("SystemTime::duration_since failed");
        let two_seconds = Duration::new(3, 0);
        println!("Average {:?}", test_time);
      //  assert_eq!(test_time < two_seconds, true);
    }
    #[test]
    fn random_point() {
        let mut field = super::GameField::new();
        for size in 1..5 {
            for _ in 0..100 {
                let Point { row, column } =
                    field.generate_random_point(&ShipDirection::Vertical, size, random_number);

                let expect_row = row >= 1 && row <= (12 - size) - 1;
                let expect_column = column >= 1 && column <= 10;

                assert_eq!(expect_row, true);
                assert_eq!(expect_column, true);
            }
        }
    }
    #[test]
    fn scan_for() {
        let mut field = super::GameField::new();
        let size = 4;
        let point = Point { row: 5, column: 6 };
        field.create_ship(size, &ShipDirection::Vertical, Some(point), random_number);
        {
            let path = vec![(Direction::Down, 1)];
            assert_eq!(
                field.scan_for(&path, point, vec![Status::Empty, Status::Bound]),
                false
            );
        }
        {
            let path = vec![(Direction::Down, 1)];
            let point = Point { row: 5, column: 8 };
            assert_eq!(field.scan_for(&path, point, vec![Status::Empty]), true);
        }
        {
            let path = vec![(Direction::Down, 1)];
            let point = Point { row: 8, column: 6 };
            assert_eq!(field.scan_for(&path, point, vec![Status::Bound]), true);
        }
        {
            let path = vec![(Direction::Down, 1)];
            let point = Point { row: 9, column: 6 };
            assert_eq!(field.scan_for(&path, point, vec![Status::Empty]), true);
        }
    }
    #[test]
    fn draw_cell() {
        let mut field = super::GameField::new();
        let points: Vec<(Point, Status)> = vec![
            (Point { row: 5, column: 5 }, Status::Ship),
            (Point { row: 8, column: 9 }, Status::Bound),
            (Point { row: 2, column: 4 }, Status::Empty),
            (Point { row: 4, column: 1 }, Status::Kill),
        ];
        points.iter().for_each(|(point, status)| {
            field.draw_cell(*point, *status);
            let cell = field.get_cell_value(*point);
            assert_eq!(cell, *status);
        })

    }
    #[test]
    fn get_cell_value() {
        let mut field = super::GameField::new();
        let point = Point { row: 1, column: 1 };
        field.draw_cell(point, Status::Bound);
        let cell_value = field.get_cell_value(point);
        assert_eq!(cell_value, Status::Bound);
    }
    #[test]
    fn draw_by_path() {
        let mut field = super::GameField::new();
        let path = vec![
            (Direction::Right, 1),
            (Direction::Right, 1),
            (Direction::Down, 1),
        ];
        let point = Point { row: 7, column: 6 };

        field.draw_by_path(Draw {
            start_point: point,
            path,
            draw_status: Status::Bound,
            allowed_status: vec![Status::Empty, Status::Bound],
        });
        let sum = point_sum(field, Status::Bound);
        assert_eq!(sum / 2, 3)
    }
    #[test]
    fn draw_ship_core() {
        let mut field = super::GameField::new();
        let size = 4;
        let result = field.draw_ship_core(
            &ShipDirection::Horizontal,           
            size,
             Point { row: 3, column: 5 },
        );
        let sum = point_sum(field, Status::Ship);
        assert_eq!(sum, size);
        assert_eq!(result, Some(()));
    }

    #[test]
    fn draw_ship_bounds() {
        let mut field = super::GameField::new();
        let size = 4;
        let bound_quantity = 14;
        field.draw_ship_bounds(
            &ShipDirection::Horizontal,
            size,
            Point { row: 5, column: 5 },
        );
        let sum = point_sum(field, Status::Bound);
        assert_eq!(sum / 2, bound_quantity)
    }
    #[test]
    fn draw_ship() {
        let mut field = super::GameField::new();
        let size = 4;
        let ship = field.draw_ship(
            size,
            &ShipDirection::Horizontal,
            Point { row: 5, column: 6 },
        );
        println!("HELLO");
        assert_eq!(100, 100);
    }
    #[test]
    fn generate_ship_bounds() {
        let field = super::GameField::new();
        let size = 3;
        let bounds_path_horizontal = field.generate_ship_bounds(&ShipDirection::Horizontal, size);
        let bounds_path_vertical = field.generate_ship_bounds(&ShipDirection::Vertical, size);
        assert_eq!(bounds_path_horizontal.len(), 5);
        assert_eq!(bounds_path_vertical.len(), 5)
    }
}