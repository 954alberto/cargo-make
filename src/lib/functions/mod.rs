//! # functions
//!
//! Custom operations which can be invoked in makefiles.
//!

#[cfg(test)]
#[path = "mod_test.rs"]
mod mod_test;

mod decode_func;
mod getat_func;
mod remove_empty_func;
mod split_func;
mod trim_func;

use crate::error::CargoMakeError;
use crate::types::{Step, Task};

fn run_function(
    function_name: &str,
    function_args: &Vec<String>,
) -> Result<Vec<String>, CargoMakeError> {
    debug!(
        "Running function: {} arguments: {:#?}",
        &function_name, &function_args
    );

    match function_name {
        "split" => Ok(split_func::invoke(function_args)),
        "remove-empty" => Ok(remove_empty_func::invoke(function_args)),
        "trim" => trim_func::invoke(function_args),
        "getat" => Ok(getat_func::invoke(function_args)),
        "decode" => Ok(decode_func::invoke(function_args)),
        _ => {
            error!("Unknown function: {}", &function_name);
            Err(CargoMakeError::NotFound(format!(
                "Unknown function: {}",
                &function_name
            )))
        }
    }
}

fn get_function_name(function_string: &str) -> Option<String> {
    match function_string.find('(') {
        Some(index) => Some(function_string[0..index].to_string()),
        None => None,
    }
}

fn get_function_argument(value: &str) -> String {
    let str_value = if value.len() == 1 {
        value
    } else {
        value.trim()
    };

    str_value.to_string()
}

fn get_function_arguments(function_string: &str) -> Option<Vec<String>> {
    if function_string.starts_with("(") && function_string.ends_with(")") {
        let args_string = function_string[1..(function_string.len() - 1)].to_string();

        let arguments = if args_string.len() > 0 {
            args_string
                .split(",")
                .map(|str_value| get_function_argument(&str_value))
                .collect()
        } else {
            vec![]
        };

        Some(arguments)
    } else {
        None
    }
}

fn evaluate_and_run(value: &str) -> Result<Vec<String>, CargoMakeError> {
    let value_string = value.to_string();

    if value_string.starts_with("@@") {
        let mut function_string = value[2..].to_string();

        let func_name_option = get_function_name(&function_string);
        match func_name_option {
            Some(function_name) => {
                function_string = function_string[function_name.len()..].to_string();
                let func_args_option = get_function_arguments(&function_string);

                match func_args_option {
                    Some(function_args) => run_function(&function_name, &function_args),
                    None => Ok(vec![value_string]),
                }
            }
            None => Ok(vec![value_string]),
        }
    } else {
        Ok(vec![value_string])
    }
}

fn modify_arguments(task: &mut Task) -> Result<(), CargoMakeError> {
    task.args = match task.args {
        Some(ref args) => {
            let mut new_args = vec![];

            for index in 0..args.len() {
                let result_args = evaluate_and_run(&args[index])?;

                for result_index in 0..result_args.len() {
                    new_args.push(result_args[result_index].clone());
                }
            }

            Some(new_args)
        }
        None => None,
    };
    Ok(())
}

pub(crate) fn run(step: &Step) -> Result<Step, CargoMakeError> {
    //clone data before modify
    let mut config = step.config.clone();

    //update args by running any needed function
    modify_arguments(&mut config)?;

    Ok(Step {
        name: step.name.clone(),
        config,
    })
}
