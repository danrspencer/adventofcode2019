// https://adventofcode.com/2019/day/2

#[test]
fn it_processes_a_basic_program() {
    let program = vec![1, 5, 6, 3, 99, 30, 40, 50];
    let expected_result = vec![1, 5, 6, 70, 99, 30, 40, 50];

    assert_eq!(intcode(program), Some(expected_result));
}

#[test]
fn it_processes_a_program_with_multiple_steps() {
    let program = vec![1, 9, 10, 3, 1, 3, 11, 0, 99, 30, 40, 50];
    let expected_result = vec![120, 9, 10, 70, 1, 3, 11, 0, 99, 30, 40, 50];

    assert_eq!(intcode(program), Some(expected_result));
}

#[test]
fn it_can_process_multiply_command() {
    let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let expected_result = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

    assert_eq!(intcode(program), Some(expected_result));
}

pub fn intcode(mut program: Vec<usize>) -> Option<Vec<usize>> {
    const ADD: usize = 1;
    const MULTIPLY: usize = 2;
    const END: usize = 99;

    let mut pointer = 0;

    loop {
        let operation = program.get(pointer);

        let values = tuple_option(program.get(pointer + 1), program.get(pointer + 2))
            .and_then(|(a, b)| tuple_option(program.get(*a), program.get(*b) ));

        let output = program.get(pointer + 3).and_then(|op| program.get_mut(*op));

        let result = match operation {
            Some(&ADD) => values.and_then(|(a, b)| Some(a + b)),
            Some(&MULTIPLY) => values.and_then(|(a, b)| Some(a * b)),
            Some(&END) => break Some(program),
            _ => break None,
        };

        if let Some((r, o)) = tuple_option(result, output) {
            *o = r
        } else {
            break None
        }

        pointer += 4;
    }
}

fn tuple_option<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
    a.and_then(|aa| b.map(|bb| (aa, bb)))
}


//pub fn find_inputs(result: usize) -> (usize, usize) {
//    let mut a = result / 2;
//    let mut b = result / 2;
//
//    let switch = false;
//
//    loop {
//        let output = intcode(input(a, b));
//
//        if output[0] == result {
//            break (a, b);
//        }
//
//        if a < 99 {
//            a += 1
//        } else if b < 99 {
//            a = 0;
//            b += 1
//        } else {
//            panic!("I give up!")
//        }
//    }
//}

pub fn input(a: usize, b: usize) -> Vec<usize> {
    vec![
        1, a, b, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 6, 23, 2, 23, 13, 27,
        1, 27, 5, 31, 2, 31, 10, 35, 1, 9, 35, 39, 1, 39, 9, 43, 2, 9, 43, 47, 1, 5, 47, 51, 2, 13,
        51, 55, 1, 55, 9, 59, 2, 6, 59, 63, 1, 63, 5, 67, 1, 10, 67, 71, 1, 71, 10, 75, 2, 75, 13,
        79, 2, 79, 13, 83, 1, 5, 83, 87, 1, 87, 6, 91, 2, 91, 13, 95, 1, 5, 95, 99, 1, 99, 2, 103,
        1, 103, 6, 0, 99, 2, 14, 0, 0,
    ]
}
