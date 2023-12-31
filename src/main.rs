use leetcode_cli::cli::cli_main;
use tokio::runtime::Builder;

fn main() {
    env_logger::init();

    if let Err(err) = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build tokio runtime failed")
        .block_on(cli_main())
    {
        eprintln!("{}", err);
    }
}
