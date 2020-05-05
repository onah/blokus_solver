extern crate rand;

mod board;
mod pieces;
mod rule;

use board::Location;
use rule::Rule;
// use rand::Rng;

pub struct Solver {}

impl Solver {
    pub fn run() {
        let mut totalcount = 0;
        let mut selected: Vec<usize> = Vec::new();

        loop {
            totalcount += 1;
            println!("Total Count: {}", totalcount);

            let flag = solver1(&mut selected);
            //let flag = solver2(&mut selected);
            if flag {
                break;
            }
        }
    }
}

fn print_board(rule: &Rule) {
    for y in 0..8 {
        for x in 0..10 {
            let loc = Location { x, y };
            let value = rule.get_cell(&loc).unwrap();
            print!("{}", value);
        }
        println!();
    }
}

/*
fn is_one_size_of_empty(rule: &Rule) -> bool {
    for y in 0..8 {
        for x in 0..10 {
            let loc = Location { x, y };
            let value = rule.get_cell(&loc);

            if value == Some(0) {
                let locs: [Location; 4] = [
                    Location { x: x - 1, y },
                    Location { x: x + 1, y },
                    Location { x, y: y + 1 },
                    Location { x, y: y - 1 },
                ];

                let mut flag = false;
                for loc1 in locs.iter() {
                    if !is_out_of_board(&loc1) {
                        let value1 = rule.get_cell(&loc1);
                        if value1 == Some(0) {
                            flag = true;
                        }
                    }
                }

                if !flag {
                    return true;
                }
            }
        }
    }
    false
}
*/

fn is_side_of_empty(rule: &Rule, location: &Location) -> Vec<Location> {
    let mut result: Vec<Location> = Vec::new();
    let value = rule.get_cell(location);
    if value == Some(0) {
        let locs: [Location; 4] = [
            Location {
                x: location.x - 1,
                y: location.y,
            },
            Location {
                x: location.x + 1,
                y: location.y,
            },
            Location {
                x: location.x,
                y: location.y + 1,
            },
            Location {
                x: location.x,
                y: location.y - 1,
            },
        ];
        for loc1 in locs.iter() {
            if !is_out_of_board(&loc1) {
                let value1 = rule.get_cell(&loc1);
                if value1 == Some(0) {
                    result.push(loc1.clone());
                }
            }
        }
    }
    result
}

/*
fn is_under_four_size_of_empty(rule: &Rule) -> bool {
    for y in 0..8 {
        for x in 0..10 {
            let first_loc = Location::new(x, y);
            let value = rule.get_cell(&first_loc);
            if value == Some(0) {
                let mut flag = false;

                let second_locs = is_side_of_empty(rule, &first_loc);
                for second_loc in second_locs.iter() {
                    let third_locs = is_side_of_empty(rule, second_loc);
                    for third_loc in third_locs.iter() {
                        if third_loc != &first_loc {
                            let fourth_locs = is_side_of_empty(rule, third_loc);
                            for fourth_loc in fourth_locs.iter() {
                                if fourth_loc != &first_loc && fourth_loc != second_loc {
                                    flag = true;
                                }
                            }
                        }
                    }
                }
                if !flag {
                    return true;
                }
            }
        }
    }
    false
}
*/

fn is_empty_multi_four(rule: &Rule) -> bool {
    for x in 0..10 {
        for y in 0..8 {
            let loc = Location::new(x, y);
            let size = get_empty_size_of_side_cell(rule, &loc);

            if size % 4 != 0 {
                return false;
            }
        }
    }
    true
}

fn get_empty_size_of_side_cell(rule: &Rule, location: &Location) -> usize {
    let mut empty_locs: Vec<Location> = Vec::new();
    recursive_get_empty_size_of_side_cell(rule, location, &mut empty_locs);

    empty_locs.len()
}

fn recursive_get_empty_size_of_side_cell(
    rule: &Rule,
    location: &Location,
    empty_locs: &mut Vec<Location>,
) {
    let value = rule.get_cell(&location);
    if value == Some(0) {
        let side_emtpy_locs = is_side_of_empty(rule, location);
        for side_emtpy_loc in side_emtpy_locs.iter() {
            let mut same_flag = false;
            for empty_loc in empty_locs.iter() {
                if empty_loc == side_emtpy_loc {
                    same_flag = true;
                }
            }
            if !same_flag {
                empty_locs.push(side_emtpy_loc.clone());
                recursive_get_empty_size_of_side_cell(rule, side_emtpy_loc, empty_locs);
            }
        }
    }
}

fn is_out_of_board(location: &Location) -> bool {
    if location.x < 0 {
        return true;
    }
    if location.y < 0 {
        return true;
    }
    if location.x >= 10 {
        return true;
    }
    if location.y >= 8 {
        return true;
    }

    false
}
fn solver1(selected: &mut Vec<usize>) -> bool {
    let mut rule = Rule::new();
    let mut count = 0;
    loop {
        //println!("Now Turn: {}", rule.get_turn());
        //println!("Count: {}", count);

        //let enable = rule.get_enable_to_put_piece();
        let enable = rule.get_enable_to_put_piece2();
        if enable.is_empty() {
            let size = selected.len();
            selected[size - 1] += 1;

            println!("-----------");
            print_board(&rule);
            return false;
        }
        /*
                if is_one_size_of_empty(&rule) {
                    let size = selected.len();
                    selected[size - 1] += 1;

                    println!("-----------");
                    print_board(&rule);
                    return false;
                }
        */
        /*
        if is_under_four_size_of_empty(&rule) {
            let size = selected.len();
            selected[size - 1] += 1;

            println!("-----------");
            print_board(&rule);
            return false;
        }
        */

        if !is_empty_multi_four(&rule) {
            let size = selected.len();
            selected[size - 1] += 1;

            println!("-----------");
            print_board(&rule);
            return false;
        }

        let index;
        if selected.len() > count {
            index = selected[count];
            if enable.len() == index {
                selected.remove(count);
                if count == 0 {
                    println!("NO ANSER!!");
                    return true;
                }
                selected[count - 1] += 1;
                return false;
            }
        } else {
            index = 0;
            selected.push(index);
        }

        let input = enable[index].clone();
        let result = rule.put_piece2(input);
        if result {
            count += 1;
            // println!("{}", count);
            if count == 20 {
                println!("--- CLEAR !!! --------");
                print_board(&rule);
                return true;
            }
        } else {
            println!("BUG");
        }
    }
}

/*
fn solver2(selected: &mut Vec<usize>) -> bool {
    let mut rule = Rule::new();
    let mut count = 0;

    loop {
        let enable = rule.get_enable_to_put_piece();
        if enable.is_empty() {
            let size = selected.len();
            selected.remove(size-1);

            println!("-----------");
            print_board(&rule);
            return false;
        }
        if is_one_size_of_empty(&rule) {
            let size = selected.len();
            selected.remove(size-1);
            println!("-----------");
            print_board(&rule);
            return false;
        }

        let index;
        let rand_num = rand::thread_rng().gen_range(0,10);

        if rand_num > 2 && selected.len() > count {
            selected.truncate(count+1);
            index = selected[count];
            if enable.len() == index {
                selected.remove(count);
                if count == 0 {
                    println!("NO ANSER!!");
                    return true;
                }

                //selected[count - 1] = rand::...
                return false;
            }
        } else {
            index = rand::thread_rng().gen_range(0, enable.len());
            selected.push(index);
        }

        let input = enable[index].clone();
        let result = rule.put_piece(input);
        if result {
            count += 1;
            // println!("{}", count);
            if count == 20 {
                println!("--- CLEAR !!! --------");
                print_board(&rule);
                return true;
            }
        } else {
            println!("BUG");
        }
    }
}
*/
