#[derive(PartialEq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "A" | "X" => Some(Shape::Rock),
            "B" | "Y" => Some(Shape::Paper),
            "C" | "Z" => Some(Shape::Scissors),
            _ => None,
        }
    }

    fn points_against(&self, other: &Self) -> usize {
        if self == other {
            return 3;
        }
        match (self, other) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Scissors, Shape::Paper)
            | (Shape::Paper, Shape::Rock) => 6,
            _ => 0,
        }
    }

    fn shape_against(&self, winning_indicator: &str) -> Option<Self> {
        Some(match (winning_indicator, self) {
            ("Z", Shape::Rock) => Shape::Paper,
            ("Z", Shape::Paper) => Shape::Scissors,
            ("Z", Shape::Scissors) => Shape::Rock,
            ("X", Shape::Rock) => Shape::Scissors,
            ("X", Shape::Paper) => Shape::Rock,
            ("X", Shape::Scissors) => Shape::Paper,
            ("Y", &shape) => shape,
            _ => return None,
        })
    }

    fn points(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

struct Game(Shape, Shape);

impl Game {
    fn from_str_guess(input: &str) -> Option<Game> {
        let Some((opponent, we)) = input.split_once(" ") else { return None };
        let (Some(opponent), Some(we)) = (Shape::from_str(opponent), Shape::from_str(we)) else { return None };
        Some(Game(opponent, we))
    }

    fn from_str(input: &str) -> Option<Game> {
        let Some((opponent, we)) = input.split_once(" ") else { return None };
        let Some(opponent) = Shape::from_str(opponent) else { return None };
        let Some(we) = opponent.shape_against(we) else { return None };

        Some(Game(opponent, we))
    }

    fn points(&self) -> usize {
        self.1.points_against(&self.0) + self.1.points()
    }
}

fn main() {
    let input = include_str!("input.txt");

    let expected_points: usize = input
        .lines()
        .map(|line| {
            Game::from_str_guess(line)
                .expect("should be parseable")
                .points()
        })
        .sum();

    println!("The expected points are {}", expected_points);

    let expected_points: usize = input
        .lines()
        .map(|line| Game::from_str(line).expect("should be parseable").points())
        .sum();

    println!(
        "The expected points with the correct calculation are {}",
        expected_points
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_test_game_guess() {
        assert_eq!(Game::from_str_guess("A Y").expect("parsable").points(), 8);
        assert_eq!(Game::from_str_guess("B X").expect("parsable").points(), 1);
        assert_eq!(Game::from_str_guess("C Z").expect("parsable").points(), 6);
    }
    #[test]
    fn should_calculate_test_game_real() {
        assert_eq!(Game::from_str("A Y").expect("parsable").points(), 4);
        assert_eq!(Game::from_str("B X").expect("parsable").points(), 1);
        assert_eq!(Game::from_str("C Z").expect("parsable").points(), 7);
    }
}
