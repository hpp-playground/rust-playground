use itertools::Itertools;

fn main() {
    let s = "this is a🤔 テスト🥺🥺 string wooo🥺🌝".to_string();
    let subs: Vec<String> = s.chars()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect();

    println!("{:?}", subs);
    println!("{:?}", s.chars().chunks(3).into_iter().next().unwrap().collect::<String>());
}