use super::*;
use crate::test;
use crate::types::{ScriptValue, Task};

#[test]
fn execute_duckscript() {
    let valid = execute(
        &vec!["echo test".to_string()],
        &vec![],
        Some(&test::create_empty_flow_info()),
        Some(Rc::new(RefCell::new(FlowState::new()))),
        true,
    )
    .unwrap();
    assert!(valid);
}

#[test]
fn execute_duckscript_error_no_validate() {
    let valid = execute(
        &vec!["badcommand".to_string()],
        &vec![],
        Some(&test::create_empty_flow_info()),
        Some(Rc::new(RefCell::new(FlowState::new()))),
        false,
    )
    .unwrap();
    assert!(!valid);
}

#[test]
#[should_panic]
fn execute_duckscript_error_with_validate() {
    execute(
        &vec!["badcommand".to_string()],
        &vec![],
        Some(&test::create_empty_flow_info()),
        Some(Rc::new(RefCell::new(FlowState::new()))),
        true,
    )
    .unwrap();
}

#[test]
fn execute_duckscript_cli_arguments() {
    let valid = execute(
        &vec!["assert ${1}".to_string()],
        &vec!["true".to_string()],
        Some(&test::create_empty_flow_info()),
        Some(Rc::new(RefCell::new(FlowState::new()))),
        true,
    )
    .unwrap();
    assert!(valid);
}

#[test]
#[should_panic]
fn execute_duckscript_cli_arguments2() {
    execute(
        &vec!["assert ${1}".to_string()],
        &vec!["false".to_string()],
        Some(&test::create_empty_flow_info()),
        Some(Rc::new(RefCell::new(FlowState::new()))),
        true,
    )
    .unwrap();
}

#[test]
#[should_panic]
fn execute_duckscript_crash() {
    execute(
        &vec!["function test".to_string()],
        &vec![],
        Some(&test::create_empty_flow_info()),
        Some(Rc::new(RefCell::new(FlowState::new()))),
        true,
    )
    .unwrap();
}

#[test]
#[should_panic]
fn execute_duckscript_crash2() {
    execute(
        &vec!["assert false".to_string()],
        &vec![],
        Some(&test::create_empty_flow_info()),
        Some(Rc::new(RefCell::new(FlowState::new()))),
        true,
    )
    .unwrap();
}

#[test]
fn cm_run_task_valid() {
    envmnt::set("CM_RUN_TASK_VALID_TEST", "0");

    let mut flow_info = test::create_empty_flow_info();

    let mut task = Task::new();
    task.script_runner = Some("@duckscript".to_string());
    task.script = Some(ScriptValue::Text(vec![r#"
    value = get_env CM_RUN_TASK_VALID_TEST
    value = calc ${value} + 1
    set_env CM_RUN_TASK_VALID_TEST ${value}
    "#
    .to_string()]));
    flow_info.config.tasks.insert("increment".to_string(), task);

    assert!(envmnt::is_equal("CM_RUN_TASK_VALID_TEST", "0"));

    execute(
        &vec![r#"
    cm_run_task increment
    cm_run_task increment
    cm_run_task increment
    "#
        .to_string()],
        &vec![],
        Some(&flow_info),
        Some(Rc::new(RefCell::new(FlowState::new()))),
        true,
    )
    .unwrap();

    assert!(envmnt::is_equal("CM_RUN_TASK_VALID_TEST", "3"));
}

#[test]
#[should_panic]
fn cm_run_task_error() {
    let flow_info = test::create_empty_flow_info();

    execute(
        &vec![r#"
    cm_run_task increment
    cm_run_task increment
    cm_run_task increment
    "#
        .to_string()],
        &vec![],
        Some(&flow_info),
        Some(Rc::new(RefCell::new(FlowState::new()))),
        true,
    )
    .unwrap();
}
