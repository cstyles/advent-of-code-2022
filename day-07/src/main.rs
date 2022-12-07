use std::collections::HashMap;

#[derive(Debug)]
enum Command {
    CdRoot,         // cd '/'
    CdUp,           // cd ".."
    CdDown(String), // cd a
    Ls,             // ls
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string.split_once(' ') {
            None => Command::Ls,
            Some(("cd", "/")) => Command::CdRoot,
            Some(("cd", "..")) => Command::CdUp,
            Some(("cd", name)) => Command::CdDown(name.to_string()),
            _ => unreachable!("not a command: {}", string),
        }
    }
}

#[derive(Debug)]
enum Line {
    Cmd(Command),
    File(usize, String),
    Dir(String),
}

impl From<&str> for Line {
    fn from(string: &str) -> Self {
        let (first, rest) = string.split_once(' ').unwrap();

        match first {
            "$" => Line::Cmd(Command::from(rest)),
            "dir" => Line::Dir(rest.to_string()),
            size => {
                let size = size.parse().unwrap();
                let name = rest.to_string();
                Line::File(size, name)
            }
        }
    }
}

#[derive(Debug, Default)]
struct Directory {
    files: HashMap<String, usize>,
    directories: HashMap<String, Self>,
}

impl Directory {
    fn add_file(&mut self, name: impl ToString, size: usize) {
        self.files.insert(name.to_string(), size);
    }

    fn add_directory(&mut self, name: impl ToString) {
        self.directories
            .insert(name.to_string(), Directory::default());
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let mut root = Directory::default();
    let lines = input.lines().map(Line::from).skip(1);
    let leftover_lines = build(&mut root, lines);
    assert_eq!(0, leftover_lines.count()); // assert that we parsed every line

    let mut sizes = HashMap::default();
    let _total_size = calculate_directory_sizes("/".into(), &root, &mut sizes);

    let part1: usize = sizes
        .values()
        .copied()
        .filter(|&size| size <= 100_000)
        .sum();

    println!("part1 = {part1}");
}

fn build<T: Iterator<Item = Line>>(directory: &mut Directory, mut lines: T) -> T {
    while let Some(line) = lines.next() {
        match line {
            Line::Cmd(Command::Ls) => {} // no-op
            Line::Cmd(Command::CdUp) => return lines,
            Line::Cmd(Command::CdRoot) => unreachable!("let's pretend this doesn't exist"),
            Line::Cmd(Command::CdDown(name)) => {
                let descend_into = directory.directories.get_mut(&name).unwrap();
                lines = build(descend_into, lines);
            }
            Line::File(size, name) => directory.add_file(name, size),
            Line::Dir(name) => directory.add_directory(name),
        }
    }

    lines
}

fn calculate_directory_sizes(
    name: String,
    directory: &Directory,
    sizes: &mut HashMap<String, usize>,
) -> usize {
    let size_of_files: usize = directory.files.values().sum();
    let size_of_directories: usize = directory
        .directories
        .iter()
        .map(|(dir_name, d)| calculate_directory_sizes(format!("{name}/{dir_name}"), d, sizes))
        .sum();

    let total = size_of_files + size_of_directories;
    sizes.insert(name, total);
    total
}
