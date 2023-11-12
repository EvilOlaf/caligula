use run_mode::RunMode;

mod byteseries;
mod children;
mod compression;
mod device;
mod escalated_daemon;
mod escalation;
mod hash;
mod ipc_common;
mod logging;
mod native;
mod run_mode;
mod ui;
mod writer_process;

fn main() {
    match RunMode::detect() {
        RunMode::Main => ui::main::main(),
        RunMode::EscalatedDaemon => escalated_daemon::main().unwrap(),
        _ => todo!(),
    }
}
