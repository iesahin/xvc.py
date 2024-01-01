use std::{io, path::Path};

use crossbeam::thread;
use crossbeam_channel::bounded;
use log::LevelFilter;
use pyo3::PyResult;
use xvc_rust::{
    cli::{XvcCLI, XvcSubCommand},
    config::{XvcConfigInitParams, XvcVerbosity},
    core::{aliases, check_ignore, default_project_config, root, types::xvcroot::load_xvc_root},
    error, file, git_checkout_ref, handle_git_automation, init,
    logging::{debug, setup_logging, uwr, XvcOutputLine},
    pipeline, storage, AbsolutePath, Error, Result,
};

use crate::XvcPyError;

const CHANNEL_BOUND: usize = 10000;

/// Runs the supplied xvc command.
pub fn run(args: &[&str]) -> PyResult<String> {
    let cli_options = XvcCLI::from_str_slice(args).map_err(XvcPyError)?;
    dispatch(cli_options)
}

/// Dispatch commands to respective functions in the API
///
/// It sets output verbosity with [XvcCLI::verbosity].
/// Determines configuration sources by filling [XvcConfigInitParams].
/// Tries to create an XvcRoot to determine whether we're inside one.
/// Creates two threads: One for running the API function, one for getting strings from output
/// channel.
///
/// A corresponding function to reuse the same [XvcRoot] object is [test_dispatch].
/// It doesn't recreate the whole configuration and this prevents errors regarding multiple
/// initializations.
pub fn dispatch(cli_opts: XvcCLI) -> PyResult<String> {
    let verbosity = if cli_opts.quiet {
        XvcVerbosity::Quiet
    } else {
        match cli_opts.verbosity {
            0 => XvcVerbosity::Default,
            1 => XvcVerbosity::Warn,
            2 => XvcVerbosity::Info,
            3 => XvcVerbosity::Debug,
            _ => XvcVerbosity::Trace,
        }
    };

    let term_log_level = match verbosity {
        XvcVerbosity::Quiet => LevelFilter::Off,
        XvcVerbosity::Default => LevelFilter::Error,
        XvcVerbosity::Warn => LevelFilter::Warn,
        XvcVerbosity::Info => LevelFilter::Info,
        XvcVerbosity::Debug => LevelFilter::Debug,
        XvcVerbosity::Trace => LevelFilter::Trace,
    };

    setup_logging(
        Some(term_log_level),
        if cli_opts.debug {
            Some(LevelFilter::Trace)
        } else {
            None
        },
    );

    let xvc_config_params = XvcConfigInitParams {
        current_dir: AbsolutePath::from(&cli_opts.workdir),
        include_system_config: !cli_opts.no_system_config,
        include_user_config: !cli_opts.no_user_config,
        project_config_path: None,
        local_config_path: None,
        include_environment_config: !cli_opts.no_env_config,
        command_line_config: Some(cli_opts.consolidate_config_options()),
        default_configuration: default_project_config(true),
    };

    let xvc_root_opt = match load_xvc_root(Path::new(&cli_opts.workdir), xvc_config_params) {
        Ok(r) => Some(r),
        Err(e) => {
            e.debug();
            None
        }
    };

    let output_str = thread::scope(move |s| {
        let (output_snd, output_rec) = bounded::<Option<XvcOutputLine>>(CHANNEL_BOUND);

        let output_snd_clone = output_snd.clone();
        let _output_rec_clone = output_rec.clone();

        let output_thread = s.spawn(move |_| {
            let mut output_str = String::new();
            while let Ok(Some(output_line)) = output_rec.recv() {
                // output_str.push_str(&output_line);
                match term_log_level {
                    LevelFilter::Off => match output_line {
                        XvcOutputLine::Output(_) => {}
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Warn(_) => {}
                        XvcOutputLine::Error(_) => {}
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Error => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Warn(_) => {}
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Warn => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Warn(m) => eprintln!("[WARN] {}", m),
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Info => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Info(m) => eprintln!("[INFO] {}", m),
                        XvcOutputLine::Warn(m) => eprintln!("[WARN] {}", m),
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Debug => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Info(m) => eprintln!("[INFO] {}", m),
                        XvcOutputLine::Warn(m) => eprintln!("[WARN] {}", m),
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(m) => eprintln!("[DEBUG] {}", m),
                    },
                    LevelFilter::Trace => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Info(m) => eprintln!("[INFO] {}", m),
                        XvcOutputLine::Warn(m) => eprintln!("[WARN] {}", m),
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Debug(m) => eprintln!("[DEBUG] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                    },
                }
            }
            output_str
        });

        if let Some(ref xvc_root) = xvc_root_opt {
            if let Some(from_ref) = cli_opts.from_ref {
                uwr!(
                    git_checkout_ref(&output_snd, xvc_root, from_ref),
                    output_snd
                );
            }
        }

        let command_thread = s.spawn(move |_| -> PyResult<()> {
            match cli_opts.command {
                XvcSubCommand::Init(opts) => {
                    let use_git = !opts.no_git;
                    let xvc_root = init::run(xvc_root_opt.as_ref(), opts).map_err(XvcPyError)?;
                    if use_git {
                        handle_git_automation(
                            &output_snd,
                            xvc_root,
                            cli_opts.to_branch.as_deref(),
                            &cli_opts.command_string,
                        )
                        .map_err(XvcPyError)?;
                    }
                    Result::Ok(())
                }

                XvcSubCommand::Aliases(opts) => {
                    Ok(aliases::run(&output_snd, opts).map_err(|e| XvcPyError(e.into()))?)
                }

                // following commands can only be run inside a repository
                XvcSubCommand::Root(opts) => Ok(root::run(
                    &output_snd,
                    xvc_root_opt
                        .as_ref()
                        .ok_or_else(|| XvcPyError(Error::RequiresXvcRepository))?,
                    opts,
                )
                .map_err(|e| XvcPyError(e.into()))?),

                XvcSubCommand::File(opts) => {
                    Ok(file::run(&output_snd, xvc_root_opt.as_ref(), opts)
                        .map_err(|e| XvcPyError(e.into()))?)
                }

                XvcSubCommand::Pipeline(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();
                    Ok(pipeline::cmd_pipeline(
                        input,
                        &output_snd,
                        xvc_root_opt
                            .as_ref()
                            .ok_or(XvcPyError(Error::RequiresXvcRepository))?,
                        opts,
                    )
                    .map_err(|e| XvcPyError(e.into()))?)
                }

                XvcSubCommand::CheckIgnore(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();

                    Ok(check_ignore::cmd_check_ignore(
                        input,
                        &output_snd,
                        xvc_root_opt
                            .as_ref()
                            .ok_or(XvcPyError(Error::RequiresXvcRepository))?,
                        opts,
                    )
                    .map_err(|e| XvcPyError(e.into()))?)
                }

                XvcSubCommand::Storage(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();
                    Ok(storage::cmd_storage(
                        input,
                        &output_snd,
                        xvc_root_opt
                            .as_ref()
                            .ok_or(XvcPyError(Error::RequiresXvcRepository))?,
                        opts,
                    )
                    .map_err(|e| XvcPyError(e.into()))?)
                }
            }
            .map_err(XvcPyError)?;

            match xvc_root_opt {
                Some(xvc_root) => {
                    if cli_opts.skip_git {
                        debug!(output_snd, "Skipping Git operations");
                    } else {
                        handle_git_automation(
                            &output_snd,
                            xvc_root,
                            cli_opts.to_branch.as_deref(),
                            &cli_opts.command_string,
                        )
                        .map_err(XvcPyError)?;
                    }
                }
                None => {
                    debug!(
                        output_snd,
                        "Xvc is outside of a project, no need to handle Git operations."
                    );
                }
            }
            Ok(())
        });

        match command_thread.join().unwrap() {
            Ok(_) => debug!(output_snd_clone, "Command completed successfully."),
            Err(e) => error!(output_snd_clone, "{}", e),
        }

        output_snd_clone.send(None).unwrap();
        output_thread.join().unwrap()
    })
    .unwrap();

    Ok(output_str)
}
