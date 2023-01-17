fn main() {
    let lines = include_str!("day1.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let mut groups = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .collect::<Vec<_>>();
   
    groups.sort();
    groups.reverse();
    let result = groups.iter().take(3).sum::<u64>();
    println!("groups = {result:?}");
}
