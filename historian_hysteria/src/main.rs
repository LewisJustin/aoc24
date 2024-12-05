// https://adventofcode.com/2024/day/1

fn _part1() {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let input = include_str!("../input.txt");
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        let num1 = numbers.next().unwrap().parse::<i32>().unwrap();
        let num2 = numbers.next().unwrap().parse::<i32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }

    list1.sort();
    list2.sort();

    let mut distances = Vec::new();
    for i in 0..list1.len() {
        distances.push((list1[i] - list2[i]).abs());
    }

    let sum = distances.iter().fold(0, |acc, x| acc + x);

    println!("{}", sum);
}

fn part2() {
    let mut list1 = Vec::new();
    let mut map = std::collections::HashMap::new();

    let input = include_str!("../input.txt");
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        let num1 = numbers.next().unwrap().parse::<i32>().unwrap();
        let num2 = numbers.next().unwrap().parse::<i32>().unwrap();
        list1.push(num1);

        if map.contains_key(&num2) {
            map.insert(num2, map.get(&num2).unwrap() + 1);
        } else {
            map.insert(num2, 1);
        }
    }

    let mut distances = Vec::new();

    for i in 0..list1.len() {
        assert!(list1[i] > 0);
        if map.contains_key(&list1[i]) {
            distances.push(list1[i] * map.get(&list1[i]).unwrap());
        }
    }
    let sum = distances.iter().fold(0, |acc, x| acc + x);

    println!("{}", sum);
}

fn main() {
    part2()
}
