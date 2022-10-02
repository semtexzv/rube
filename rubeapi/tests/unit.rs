use rubeapi::api::systemd;

#[test]
fn test_unit() {
    let data = &[
        10, 3, 104, 101, 121, 18, 7, 102, 105, 108, 101, 58, 47, 47, 26, 16, 103, 114, 97, 112,
        104, 105, 99, 97, 108, 46, 116, 97, 114, 103, 101, 116, 64, 0, 162, 1, 109, 10, 43, 68,
        111, 110, 39, 116, 32, 114, 117, 110, 32, 116, 104, 105, 115, 32, 119, 105, 116, 104, 111,
        117, 116, 32, 99, 111, 110, 115, 117, 108, 116, 105, 110, 103, 32, 109, 101, 32, 102, 105,
        114, 116, 115, 116, 16, 2, 50, 14, 47, 98, 108, 97, 98, 108, 97, 98, 108, 97, 47, 112, 105,
        100, 66, 19, 10, 17, 101, 99, 104, 111, 32, 92, 34, 72, 101, 121, 32, 89, 111, 117, 33, 92,
        34, 74, 9, 8, 60, 40, 30, 48, 144, 28, 56, 60, 80, 2, 90, 8, 8, 0, 8, 1, 8, 2, 16, 66, 120,
        2,
    ][..];

    let decoded: systemd::service::Unit = protokit::binformat::from_slice(data).unwrap();
    let manual = systemd::service::Unit::default()
        .with_description("hey".to_string())
        .with_doc_url("file://".to_string())
        .with_wants("graphical.target".to_string())
        .with_enabled(false)
        .with_details_service(
            systemd::service::Service::default()
                .with_title("Don't run this without consulting me firtst".to_string())
                .with_type(systemd::service::ServiceType::FORKING)
                .with_pid_file("/blablabla/pid".to_string())
                .with_exec(
                    systemd::service::ServiceExec::default()
                        .with_start("echo \\\"Hey You!\\\"".to_string()),
                )
                .with_time(
                    systemd::service::ServiceTime::default()
                        .with_restart(60)
                        .with_timeout(30)
                        .with_limit(3600)
                        .with_watchdog(60),
                )
                .with_restart(systemd::service::ServiceRestart::ON_FAILURE)
                .with_status(
                    systemd::service::ServiceStatus::default()
                        .with_success(0)
                        .with_success(1)
                        .with_success(2)
                        .with_prevent_restart(66),
                )
                .with_oom_policy(systemd::service::ServiceOOMPolicy::KILL),
        );
    assert_eq!(decoded, manual, "{:#?}\n{:#?}", decoded, manual);
    // panic!("{:?}", x);
}
