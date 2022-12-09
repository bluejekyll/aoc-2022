//! --- Day 7: No Space Left On Device ---
//! You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?
//!
//! The device the Elves gave you has problems with more than just its communication system. You try to run a system update:
//!
//! $ system-update --please --pretty-please-with-sugar-on-top
//! Error: No space left on device
//! Perhaps you can delete some files to make space for the update?
//!
//! You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:
//!
//! $ cd /
//! $ ls
//! dir a
//! 14848514 b.txt
//! 8504156 c.dat
//! dir d
//! $ cd a
//! $ ls
//! dir e
//! 29116 f
//! 2557 g
//! 62596 h.lst
//! $ cd e
//! $ ls
//! 584 i
//! $ cd ..
//! $ cd ..
//! $ cd d
//! $ ls
//! 4060174 j
//! 8033020 d.log
//! 5626152 d.ext
//! 7214296 k
//! The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.
//!
//! Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:
//!
//! cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
//! cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
//! cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
//! cd / switches the current directory to the outermost directory, /.
//! ls means list. It prints out all of the files and directories immediately contained by the current directory:
//! 123 abc means that the current directory contains a file named abc with size 123.
//! dir xyz means that the current directory contains a directory named xyz.
//! Given the commands and output in the example above, you can determine that the filesystem looks visually like this:
//!
//! - / (dir)
//!   - a (dir)
//!     - e (dir)
//!       - i (file, size=584)
//!     - f (file, size=29116)
//!     - g (file, size=2557)
//!     - h.lst (file, size=62596)
//!   - b.txt (file, size=14848514)
//!   - c.dat (file, size=8504156)
//!   - d (dir)
//!     - j (file, size=4060174)
//!     - d.log (file, size=8033020)
//!     - d.ext (file, size=5626152)
//!     - k (file, size=7214296)
//! Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.
//!
//! Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)
//!
//! The total sizes of the directories above can be found as follows:
//!
//! The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
//! The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
//! Directory d has total size 24933642.
//! As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
//! To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)
//!
//! Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?
//!
//! Your puzzle answer was 1453349.
//!
//! --- Part Two ---
//! Now, you're ready to choose a directory to delete.
//!
//! The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.
//!
//! In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.
//!
//! To achieve this, you have the following options:
//!
//! Delete directory e, which would increase unused space by 584.
//! Delete directory a, which would increase unused space by 94853.
//! Delete directory d, which would increase unused space by 24933642.
//! Delete directory /, which would increase unused space by 48381165.
//! Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.
//!
//! Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?
//!
//! Your puzzle answer was 2948823.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, space0, space1},
    },
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

const TOTAL_DISK_SIZE: usize = 70000000;
const SPARE_DISK_NEED: usize = 30000000;

#[derive(PartialEq, Eq, Debug)]
enum Command<'a> {
    List,
    ChangeDirectory(Directory<'a>),
}

#[derive(PartialEq, Eq, Debug)]
enum Directory<'a> {
    Root,
    Parent,
    Path(&'a str),
}

#[derive(PartialEq, Eq, Debug)]
struct FileRef<'a> {
    name: &'a str,
    extension: Option<&'a str>,
    size: u32,
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct OwnedFile {
    name: String,
    extension: Option<String>,
    size: u32,
}

impl From<FileRef<'_>> for OwnedFile {
    fn from(file: FileRef<'_>) -> Self {
        OwnedFile {
            name: file.name.to_owned(),
            extension: file.extension.map(String::from),
            size: file.size,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Default)]
struct Dir {
    files: HashSet<OwnedFile>,
    dirs: HashSet<PathBuf>,
}

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

// parse: $ cd|ls {dir}
fn parse_command(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = character::complete::char('$')(input)?;
    let (input, _) = space1(input)?;

    let (input, command) = alpha1(input)?;
    let (input, _) = space0(input)?;
    //    let (input, arg) = opt(alpha0)(input)?;
    let (input, arg) = opt(map(alt((tag(".."), tag("/"), alpha1)), |s| match s {
        ".." => Directory::Parent,
        "/" => Directory::Root,
        _ => Directory::Path(s),
    }))(input)?;

    let command = match command {
        "ls" => Command::List,
        "cd" => Command::ChangeDirectory(arg.expect("no arg for cd")),
        _ => panic!("bad command: {command}"),
    };

    Ok((input, command))
}

// parse file size and name: {u32} {name}
fn parse_file_size(input: &str) -> IResult<&str, FileRef<'_>> {
    let (input, size) = character::complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, (name, _, extension)) =
        tuple((alpha1, opt(character::complete::char('.')), opt(alpha1)))(input)?;

    Ok((
        input,
        FileRef {
            name,
            extension,
            size,
        },
    ))
}

// parse dir listing: dir {name}
fn parse_dir(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("dir")(input)?;
    let (input, _) = space1(input)?;
    alpha1(input)
}

// calculate directory sizes
fn find_size(directories: &HashMap<PathBuf, Dir>, path: &Path) -> usize {
    if let Some(dir) = directories.get(path) {
        let files_sizes: usize = dir.files.iter().map(|f| f.size as usize).sum();
        let directory_sizes: usize = dir
            .dirs
            .iter()
            .map(|path| find_size(directories, path))
            .sum();

        files_sizes + directory_sizes
    } else {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;
    let file = BufReader::new(File::open(filename)?);
    let mut directories = HashMap::<PathBuf, Dir>::new();
    let mut current_dir = PathBuf::new();

    // build up the directories, etc...
    for line in file.lines() {
        let line = &line?;

        if let Ok((_, command)) = parse_command(line) {
            match command {
                Command::ChangeDirectory(path) => match path {
                    Directory::Root => current_dir.clear(),
                    Directory::Parent => {
                        current_dir.pop();
                    }
                    Directory::Path(path) => current_dir.push(path),
                },
                Command::List => {
                    // just a noop, we're going to assume any dirs or follows all follow a directory listing command...
                }
            }
        } else if let Ok((_, dir)) = parse_dir(line) {
            let full_dir_path = current_dir.join(dir);
            directories
                .entry(current_dir.clone())
                .or_insert_with(Default::default)
                .dirs
                .insert(full_dir_path);
        } else if let Ok((_, file)) = parse_file_size(line) {
            directories
                .entry(current_dir.clone())
                .or_insert_with(Default::default)
                .files
                .insert(file.into());
        }
    }

    // unmut reference
    let directories = directories;

    // calculate part 1.
    let total: usize = directories
        .keys()
        .map(|path| find_size(&directories, path))
        .filter(|size| *size <= 100000)
        .sum();

    println!("part1 total: {total}");

    // part two
    let root_dir_size = find_size(&directories, Path::new(""));
    let spare_disk = TOTAL_DISK_SIZE - root_dir_size;

    let mut dir_sizes: Vec<usize> = directories
        .keys()
        .map(|path| find_size(&directories, path))
        .collect();

    dir_sizes.sort_unstable();
    let dir_size_to_delete = *dir_sizes
        .iter()
        .find(|size| *size + spare_disk >= SPARE_DISK_NEED)
        .expect("no directories found to free enough space");

    println!("part2 dir to remove: {dir_size_to_delete}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("$ cd ..").unwrap().1,
            Command::ChangeDirectory(Directory::Parent),
        );
        assert_eq!(
            parse_command("$ cd /").unwrap().1,
            Command::ChangeDirectory(Directory::Root),
        );
        assert_eq!(
            parse_command("$ cd abc").unwrap().1,
            Command::ChangeDirectory(Directory::Path("abc")),
        );
    }

    #[test]
    fn test_parse_file() {
        assert_eq!(
            parse_file_size("12345 file.txt").unwrap().1,
            FileRef {
                name: "file",
                extension: Some("txt"),
                size: 12345
            }
        );

        assert_eq!(
            parse_file_size("12345 file").unwrap().1,
            FileRef {
                name: "file",
                extension: None,
                size: 12345
            }
        );
    }

    #[test]
    fn test_parse_dir() {
        assert_eq!(parse_dir("dir ddddd").unwrap().1, "ddddd");
    }
}
