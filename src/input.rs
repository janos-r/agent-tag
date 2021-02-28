use structopt::StructOpt;

/// A tag game simulator with stupid agents
#[derive(StructOpt, Debug)]
#[structopt(name = "agent-tag")]
pub struct Input {
    /// Number of agents
    #[structopt(short, long, default_value = "40")]
    pub agents: usize,

    /// Number of ms between tics
    #[structopt(short, long, default_value = "1000")]
    pub time: u64,

    /// Size of field
    #[structopt(short, long, default_value = "25")]
    pub size: usize,
}
