pub fn sum (num_one: usize, num_two: usize) -> usize {
    num_one + num_two
}

pub fn rest (num_one: usize, num_two: usize) -> usize {
    num_one - num_two
}

pub fn mult (num_one: usize, num_two: usize) -> usize {
    num_one * num_two
}

pub fn div (num_one: usize, num_two: usize) -> (usize, bool) {
    if num_two == 0 && num_one == 0 {
        return (0, false);
    }
    (num_one / num_two, true)
}