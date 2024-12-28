use memori::process::Process;
use memori::repl::Repl;

fn main() {
    let a = Process::try_new(1464);
    let mut repl = Repl::new();
    repl.repl();
}
