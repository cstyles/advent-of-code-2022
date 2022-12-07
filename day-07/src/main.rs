use std::collections::HashMap;

#[derive(Debug)]
enum Command<'input> {
    CdRoot,              // cd '/'
    CdUp,                // cd ".."
    CdDown(&'input str), // cd a
    Ls,                  // ls
}

impl<'input> From<&'input str> for Command<'input> {
    fn from(string: &'input str) -> Self {
        match string.split_once(' ') {
            None => Command::Ls,
            Some(("cd", "/")) => Command::CdRoot,
            Some(("cd", "..")) => Command::CdUp,
            Some(("cd", name)) => Command::CdDown(name),
            _ => unreachable!("not a command: {}", string),
        }
    }
}

#[derive(Debug)]
enum Line<'input> {
    Cmd(Command<'input>),
    File(usize, &'input str),
    Dir(&'input str),
}

impl<'input> From<&'input str> for Line<'input> {
    fn from(string: &'input str) -> Self {
        let (first, rest) = string.split_once(' ').unwrap();

        match first {
            "$" => Line::Cmd(Command::from(rest)),
            "dir" => Line::Dir(rest),
            size => Line::File(size.parse().unwrap(), first),
        }
    }
}

#[derive(Debug, Default)]
struct Directory<'input> {
    files: HashMap<&'input str, usize>,
    directories: HashMap<&'input str, Self>,
}

impl<'input> Directory<'input> {
    fn add_file(&mut self, name: &'input str, size: usize) {
        self.files.insert(name, size);
    }

    fn add_directory(&mut self, name: &'input str) {
        self.directories.insert(name, Directory::default());
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
    let total_space_already_used = calculate_directory_sizes("/".into(), &root, &mut sizes);

    let part1: usize = sizes
        .values()
        .copied()
        .filter(|&size| size <= 100_000)
        .sum();

    println!("part1 = {part1}");

    let total_disk_space = 70_000_000;
    let space_necessary_for_update = 30_000_000;

    let free_space = total_disk_space - total_space_already_used;
    let need_to_delete = space_necessary_for_update - free_space;

    let part2 = sizes
        .values()
        .copied()
        .filter(|&size| size >= need_to_delete)
        .min()
        .unwrap();

    println!("part2 = {part2}");
}

fn build<'input, T: Iterator<Item = Line<'input>>>(
    directory: &mut Directory<'input>,
    mut lines: T,
) -> T {
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
