//Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
//
//For example:
//
//For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
//For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
//For a mass of 1969, the fuel required is 654.
//For a mass of 100756, the fuel required is 33583.

#[test]
fn it_calculates_fuel_for_modules() {
    assert_eq!(calc_modules_fuel(vec![12]), 2);
    assert_eq!(calc_modules_fuel(vec![14]), 2);
    assert_eq!(calc_modules_fuel(vec![1969]), 654);
    assert_eq!(calc_modules_fuel(vec![100756]), 33583);

    assert_eq!(
        calc_modules_fuel(vec![12, 14, 1969, 100756]),
        2 + 2 + 654 + 33583
    );

    // Mass under 6 is lifted by fairies
    assert_eq!(calc_modules_fuel(vec![3]), 0);
}

// For each module mass, calculate its fuel and add it to the total. Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel requirement is zero or negative. For example:
//
// A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
// At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
// The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.

#[test]
fn it_calculates_fuel_for_fuel() {
    assert_eq!(calc_total_fuel(vec![14]), 2);
    assert_eq!(calc_total_fuel(vec![1969]), 966);
    assert_eq!(calc_total_fuel(vec![100756]), 50346);

    assert_eq!(calc_total_fuel(vec![14, 1969, 100756]), 2 + 966 + 50346)
}

pub fn calc_modules_fuel(module_masses: Vec<u32>) -> u32 {
    module_masses
        .iter()
        .fold(0, |sum, size| sum + calc_fuel(size))
}

pub fn calc_total_fuel(module_masses: Vec<u32>) -> u32 {
    module_masses.iter().fold(0, |sum, size| {
        let module_fuel = calc_fuel(size);
        sum + module_fuel + calc_fuel_fuel(&module_fuel)
    })
}

fn calc_fuel(mass: &u32) -> u32 {
    let some_mass = mass / 3;

    if some_mass > 2 {
        some_mass - 2
    } else {
        0
    }
}

fn calc_fuel_fuel(fuel_mass: &u32) -> u32 {
    let fuel_fuel = calc_fuel(fuel_mass);

    if fuel_fuel > 0 {
        fuel_fuel + calc_fuel_fuel(&fuel_fuel)
    } else {
        0
    }
}

pub fn input() -> Vec<u32> {
    vec![
        118505, 138608, 125418, 146131, 136211, 98802, 95533, 65338, 50826, 118071, 54328, 108176,
        60708, 99108, 64556, 103726, 87778, 108165, 78648, 68391, 79748, 94421, 137497, 138264,
        125872, 127901, 116850, 93683, 68526, 134908, 102609, 69769, 106181, 89986, 127197, 74669,
        109912, 51328, 125824, 105763, 70648, 66476, 117452, 50181, 70915, 92821, 94381, 130405,
        128627, 65090, 125594, 148506, 95193, 143595, 71834, 147624, 142247, 112838, 149474,
        139785, 134850, 92177, 110591, 102115, 141224, 117174, 68695, 65212, 54709, 51982, 53791,
        68079, 120439, 76513, 132952, 132959, 142650, 135186, 59593, 83982, 56889, 141751, 87634,
        148232, 50803, 63222, 97836, 103121, 106561, 88348, 69735, 75400, 74045, 56715, 101561,
        124401, 106296, 144550, 134903, 113838,
    ]
}
