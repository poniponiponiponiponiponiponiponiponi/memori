use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, multicall = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Change type of the variables we scan for
    #[clap(visible_alias("t"))]
    Type(TypeArgs),

    /// PID of the process to scan the memory of
    #[clap(visible_alias("proc"))]
    Process(ProcessArgs),

    /// Expression by which to filter addresses
    #[clap(visible_alias("f"))]
    Filter(FilterArgs),

    /// Print addresses
    #[clap(visible_alias("p"))]
    Print,

    /// Add address to selected
    #[clap(visible_alias("s"))]
    Select(SelectArgs),

    /// Remove address from selected
    #[clap(visible_alias("u"), visible_alias("uns"))]
    Unselect(UnselectArgs),

    /// Set selected address to value
    Set(SetArgs),

    /// Freeze selected address so the value doesn't change
    Freeze(FreezeArgs),

    /// Exit the program
    #[clap(visible_alias("quit"))]
    Exit,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ValType {
    I32,
    U32,
    I16,
    U16,
}

#[derive(Debug, Args)]
pub struct TypeArgs {
    pub val_type: ValType,
}

#[derive(Debug, Args)]
pub struct ProcessArgs {
    pub pid: u32,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum FilterOperator {
    /// <
    #[value(alias("<"), hide = false)]
    Less,
    /// <=
    #[value(alias("<="), hide = false)]
    LessEqual,
    /// >
    #[value(alias(">"), hide = false)]
    Greater,
    /// >=
    #[value(alias(">="), hide = false)]
    GreaterEqual,
    /// ==
    #[value(alias("=="), hide = false)]
    Equal,
    /// !=
    #[value(alias("!="), hide = false)]
    NotEqual,
    Changed,
    NotChanged,
    Unknown,
}

#[derive(Debug, Args)]
pub struct FilterArgs {
    pub operator: FilterOperator,
    pub operand: Option<String>,
}

#[derive(Debug, Args)]
pub struct SelectArgs {
    pub to_select: usize,
}

#[derive(Debug, Args)]
pub struct UnselectArgs {
    pub to_unselect: usize,
}

#[derive(Debug, Args)]
pub struct SetArgs {
    pub selected: usize,
    pub value: String,
}

#[derive(Debug, Args)]
pub struct FreezeArgs {
    pub selected: usize,
}

impl Cli {
    pub fn exec(&self) {
        println!("{:?}", self);
    }
}
