
StructOpt 可以非常方便的把命令行参数解析到指定的结构体中。
只需要声明一个结构体，并派生 `StructOpt` trait 即可自动解析命令行参数到结构体。并且结构体的每个字段都可以通过 `structopt` 属性来申明参数的类型，长短名称，帮助信息等。

`#[structopt(...)]` 属性可以放在 结构体，结构体字段，枚举，枚举变体等之上。举例如下：

```rust

use structopt::StructOpt;
use std::path::PathBuf;

/// top-level
#[derive(StructOpt, Debug)]
#[structopt( name="empty_git", author, about="An example of StructOpt usage.")]
struct GitCommand {
    /// field-level
    #[structopt(short, long, help="Man message")]
    man: bool,

    #[structopt(long, parse(from_os_str), help="the execute path")]
    exec_path: PathBuf,

    #[structopt(long)]
    git_dir: Option<String>,

    #[structopt(subcommand)]
    sub_command: Option<GitSubCommand>
}

#[derive(StructOpt, Debug)]
enum GitSubCommand {
    /// top-level
    #[structopt(about="Add file contents to the index")]
    add {
        /// field-level
        #[structopt(name="FILE", parse(from_os_str))]
        files: Vec<PathBuf>,
        #[structopt(short, long, help="Interactively choose hunks of patch between the index and the work tree and add them to the index. ")]
        patch: bool,
        #[structopt(short, long, help="Open the diff vs. the index in an editor and let the user edit it.")]
        edit: bool
    },
    #[structopt(about="Download objects and refs from another repository")]
    fetch {
        #[structopt(long, help="Fetch all remotes")]
        all: bool,
        #[structopt(short, long, help="Append ref names and object names of fetched refs to the existing of .git/FETCH_HEAD")]
        append: bool,
        #[structopt(short, long, help="Fetch all tags from the remote")]
        tags: bool
    }
}

fn main() {
    let args = GitCommand::from_args();
    println!("Hello, world!: {:?}", args);
}
```
结果如下：
```
$ empty_git.exe -h
empty_git 0.1.0
zmant <zmant724@aliyun.com>
An example of StructOpt usage.

USAGE:
    empty_git.exe [FLAGS] [OPTIONS] --exec-path <exec-path> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -m, --man        Man message
    -V, --version    Prints version information

OPTIONS:
        --exec-path <exec-path>    the execute path
        --git-dir <git-dir>

SUBCOMMANDS:
    add      Add file contents to the index
    fetch    Download objects and refs from another repository
    help     Prints this message or the help of the given subcommand(s)
```

但是用起来似乎还有一些小问题，比如：

* 无法指定某字段中的值一定需要给定，只能根据其他字段的条件来申明他一定要出现。
* 无法在 FLAGS 段指定出 bool 类型外的类型参数，