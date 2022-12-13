fn main() {
    let input = include_str!("input.txt");
    let mut calories = input.split("\n\n").into_iter().map(sum_calories).collect::<Vec<_>>();
    let maximum_calories = calories.iter().max().expect("there should be at least one line");

    println!("The maximum calories an elve is carrying is {}", maximum_calories);
    calories.sort();
    calories.reverse();
    let top_three_calories: usize = calories[0..3].iter().sum();
    println!("The calories the top three elves are carrying is {}", top_three_calories);

}

fn sum_calories(input: &str) -> usize {
    input.lines().map(|line| line.parse::<usize>().expect("should contain a parsable line")).sum()
}