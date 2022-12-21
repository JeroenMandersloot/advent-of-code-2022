fn mix(input: &Vec<i64>, decryption_key: i64, num_rounds: usize) -> i64 {
    let input = input.iter().map(|v| *v * decryption_key).collect::<Vec<_>>();
    let length = input.len() as usize;
    let mut indices = (0..input.len()).collect::<Vec<_>>();
    for _ in 0..num_rounds {
        for (idx, value) in input.iter().enumerate() {
            let current_idx = indices.iter().position(|v| *v == idx).unwrap();
            indices.remove(current_idx);
            let modulo = (length as i64) - 1;
            let new_idx = ((current_idx as i64 + value.rem_euclid(modulo)) % modulo) as usize;
            indices.insert(new_idx as usize, idx);
        }
    }

    let original_base_idx = input.iter().position(|v| *v == 0).unwrap();
    let base_idx = indices.iter().position(|v| *v == original_base_idx).unwrap();
    [1000, 2000, 3000].into_iter().map(|c| input[indices[(base_idx + c) % length]]).sum()
}

fn main() {
    let input = aoc::io::get_input(20).lines().map(|c| c.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
    println!("{}", mix(&input, 1, 1));
    println!("{}", mix(&input, 811589153, 10));
}