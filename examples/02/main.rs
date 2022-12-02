use itertools::Itertools;
use std::fs::read_to_string;

// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors

fn get_winning_move(their_move: &str) -> &str {
    match their_move{
        "A" => return "Y",
        "B" => return "Z",
        "C" => return "X",
        _=>panic!("Invalid move")
    }
}
fn get_losing_move(their_move: &str) -> &str {
    match their_move{
        "A" => return "Z",
        "B" => return "X",
        "C" => return "Y",
        _=>panic!("Invalid move")
    }
}

fn get_move_score(move_char: &str) -> u32 {
    match move_char{
        "A"|"X"=> 1,
        "B"|"Y"=> 2,
        "C"|"Z"=> 3,
        _=>panic!("Invalid move")
    }
}

static WIN_SCORE_VALUE: u32 = 6;
static DRAW_SCORE_VALUE: u32 = 3;

fn part_one(_contents: &String) -> u32 {
    let score = _contents
        .lines()
        .flat_map(|s| s.split_whitespace().tuples())
        .fold(0, |score: u32, (their_move, my_move)| {
            let mut round_scoure = get_move_score(my_move);
            
            if get_move_score(their_move) == get_move_score(my_move) {
                round_scoure+=DRAW_SCORE_VALUE;
            } else if get_winning_move(their_move) == my_move {
                round_scoure+=WIN_SCORE_VALUE;
            }

            score+round_scoure
        });
    score
}

fn part_two(_contents: &String) -> u32 {
    let score = _contents
        .lines()
        .flat_map(|s| s.split_whitespace().tuples())
        .fold(0, |score: u32, (their_move, my_goal)| {
            let mut round_score = 0;

            match my_goal {
                // lose
                "X"=>{
                    round_score+=get_move_score(get_losing_move(their_move));
                },
                // draw
                "Y"=>{
                    round_score+=DRAW_SCORE_VALUE+get_move_score(their_move);
                },
                //win
                "Z"=>{
                    round_score+=WIN_SCORE_VALUE+get_move_score(get_winning_move(their_move));
                }
                _=>panic!("Invalid Move")
            };

            score+round_score
        });
    score
}

fn main() {
    let contents =
        read_to_string("examples/02/main.in").expect("Something went wrong reading the file");

    println!("What would your total score be if everything goes exactly according to your strategy guide?\n- {}", part_one(&contents));

    println!("what would your total score be if everything goes exactly according to your strategy guide?\n- {}", part_two(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/02/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(15, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(12, part_two(&get_contents()));
    }
}