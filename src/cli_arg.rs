use clap::{
    Arg,
    Command,
};

// This is not the recommended way to set clap args but it works and its too late to change it now
pub fn create_match() -> clap::Command {
    let matches = Command::new("sothis")
        .version("0.5.0")
        .author("makemake <vukasin@gostovic.me>")
        .about("Tool for replaying historical transactions. Designed to be used with anvil or hardhat.")
        .arg(Arg::new("source_rpc")
            .long("source_rpc")
            .short('s')
            .num_args(1..)
            .required(true)
            .help("HTTP JSON-RPC of the node we're querying data from"))
        .arg(Arg::new("replay_rpc")
            .long("replay_rpc")
            .short('r')
            .num_args(1..)
            .help("HTTP JSON-RPC of the node we're replaying data to"))
        .arg(Arg::new("mode")
            .long("mode")
            .short('m')
            .num_args(1..)
            .default_value("historic")
            .help("Choose between live, historic, track, fast_track, or call_track"))
        .arg(Arg::new("terminal_block")
            .long("terminal_block")
            .short('b')
            .num_args(1..)
            .required_if_eq("mode", "historic")
            .help("Block we're replaying until"))
        .arg(Arg::new("exit_on_tx_fail")
            .long("exit_on_tx_fail")
            .num_args(0..)
            .help("Exit the program if a transaction fails"))
        .arg(Arg::new("block_listen_time")
            .long("block_listen_time")
            .short('t')
            .num_args(1..)
            .default_value("500")
            .help("Time in ms to check for new blocks."))
        .arg(Arg::new("entropy_threshold")
            .long("entropy_threshold")
            .num_args(1..)
            .default_value("0.07")
            .help("Set the percentage of failed transactions to trigger a warning"))
        .arg(Arg::new("replay_delay")
            .long("replay_delay")
            .short('d')
            .num_args(1..)
            .default_value("0")
            .help("Default delay for block replay in ms"))
        .arg(Arg::new("send_as_unsigned")
            .long("send_as_unsigned")
            .num_args(0..)
            .help("Exit the program if a transaction fails"))
        .arg(Arg::new("no_setup")
            .long("no_setup")
            .num_args(0..)
            .help("Start replaying immediately."))
        .arg(Arg::new("decimal")
            .long("decimal")
            .num_args(0..)
            .help("Start replaying immediately."))
        .arg(Arg::new("contract_address")
            .long("contract_address")
            .short('c')
            .num_args(1..)
            .required_if_eq("mode", "track")
            .required_if_eq("mode", "fast_track")
            .help("Address of the contract we're tracking storage."))
        .arg(Arg::new("storage_slot")
            .long("storage_slot")
            .short('l')
            .num_args(1..)
            .required_if_eq("mode", "track")
            .required_if_eq("mode", "fast_track")
            .help("Storage slot for the variable we're tracking"))
        .arg(Arg::new("calldata")
            .long("calldata")
            .short('a')
            .num_args(1..)
            .required_if_eq("mode", "call_track")
            .help("Storage slot for the variable we're tracking"))
        .arg(Arg::new("origin_block")
            .long("origin_block")
            .short('o')
            .num_args(1..)
            .required_if_eq("mode", "fast_track")
            .help("First block sothis will look at."))
        .arg(Arg::new("query_interval")
            .long("query_interval")
            .short('q')
            .num_args(1..)
            .help("First block sothis will look at."))
        .arg(Arg::new("path")
            .long("path")
            .short('p')
            .num_args(1..)
            .default_value(".")
            .help("Path to file we're writing to"))
        .arg(Arg::new("filename")
            .long("filename")
            .short('f')
            .num_args(1..)
            .default_value("")
            .help("Name of the file."));

    return matches;
}
