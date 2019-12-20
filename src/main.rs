mod day1;
mod day2;

fn main() {
    println!(
        "Day 1 - Module Fuel Required: {}",
        day1::calc_modules_fuel(day1::input())
    );
    println!(
        "Day 1 - Total Fuel Required: {}",
        day1::calc_total_fuel(day1::input())
    );
    println!("----------------------------------------");
    //    println!(
    //        "Day 2 - Program Result: {}",
    //        day2::intcode(day2::input(1, 12))[0]
    //    );
    //    println!(
    //        "Day 2 - Inputs for 19690720: {:?}",
    //        day2::find_inputs(19690720)
    //    );
}
