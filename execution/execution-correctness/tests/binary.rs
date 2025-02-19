// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_config::{
    config::{ExecutionCorrectnessService, PersistableConfig, RemoteExecutionService},
    utils,
};
use execution_correctness::ExecutionCorrectnessManager;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const BINARY: &str = env!("CARGO_BIN_EXE_execution-correctness");
#[test]
fn test_rest() {
    let (mut config, _handle, db_rw) = executor_test_helpers::start_storage_service();
    let server_port = utils::get_available_port();
    let server_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), server_port);
    config.execution.service =
        ExecutionCorrectnessService::Process(RemoteExecutionService { server_address });

    let config_path = aptos_temppath::TempPath::new();
    config_path.create_as_file().unwrap();
    config.save_config(config_path.path()).unwrap();

    let mut command = std::process::Command::new(BINARY);
    command
        .arg(config_path.path())
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());
    let mut child = command.spawn().unwrap();

    // Run a command as a client to verify the service is running
    let res = ExecutionCorrectnessManager::new(&config, db_rw)
        .client()
        .reset();

    // Ensure the safety-rules subprocess is killed whether the test passes or fails.
    // Not doing this would result in a zombie process.
    child
        .kill()
        .expect("could not kill execution-correctness process");
    child
        .wait()
        .expect("could not wait on execution-correctness process");
    res.unwrap();
}
