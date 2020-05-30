
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
