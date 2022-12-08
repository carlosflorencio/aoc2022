use std::collections::HashMap;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> HashMap<String, usize> {
    let mut cwd: Vec<String> = vec!["/".into()];
    let mut dirs: HashMap<String, usize> = HashMap::new();

    input.lines().skip(1).for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "$" => match parts[1] {
                "cd" => match parts[2] {
                    ".." => {
                        cwd.pop();
                    }
                    _ => {
                        cwd.push(parts[2].into());
                    }
                },
                _ => {}
            },
            "dir" => {}
            _ => {
                let size = parts[0].parse::<usize>().unwrap();
                let key = cwd.join("");
                *dirs.entry(key.clone()).or_insert(0) += size;

                let mut full_path: String = "".into();
                for segment in cwd.iter() {
                    full_path.push_str(segment);

                    if full_path == key {
                        break;
                    }

                    *dirs.entry(full_path.clone()).or_insert(0) += size;
                }
            }
        }
    });

    dirs
}

#[aoc(day7, part1)]
pub fn part1(input: &HashMap<String, usize>) -> usize {
    input.values().filter(|&e| *e < 100_000 as usize).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &HashMap<String, usize>) -> usize {
    let used = input.get("/").unwrap();
    let free: usize = 70_000_000 - used;
    let min = 30_000_000 - free;

    *input.values().filter(|&s| *s > min).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"##;

    #[test]
    fn test_example_input() {
        assert_eq!(95437, part1(&parse_input(INPUT)));
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(24933642, part2(&parse_input(INPUT)));
    }
}
