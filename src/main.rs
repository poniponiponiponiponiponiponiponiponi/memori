use memori::repl::Repl;
use memori::process::Process;

fn main() {
    let a = Process::try_new(1464);
    let mut repl = Repl::new();
    repl.repl();
}
