use std::io;

use crossbeam::thread;
use crossbeam_channel::bounded;
use log::LevelFilter;
use pyo3::PyResult;
use xvc_rust::{
    cli::{XvcCLI, XvcSubCommand}, config::XvcVerbosity, core::{aliases, check_ignore, root}, error, file, git_checkout_ref, handle_git_automation, init, logging::{debug, setup_logging, uwr, XvcOutputLine}, pipeline, storage, watch, Error, Result, XvcRootOpt
};

use crate::XvcPyError;

const CHANNEL_BOUND: usize = 10000;

pub struct PyCommandOutput {
    pub output: String,
    pub xvc_root_opt: XvcRootOpt
}

/// Runs the supplied xvc command.
pub fn run(xvc_root_opt: XvcRootOpt, args: &[&str]) -> PyResult<PyCommandOutput> {
    let cli_opts = XvcCLI::from_str_slice(args).map_err(XvcPyError)?;
    dispatch_with_root(xvc_root_opt, cli_opts)
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
///
/// The xvc_root_opt is passed within a cell to make it updatable in xvc init command. Otherwise
/// the return value should be the same with sent value. 
pub fn dispatch_with_root(xvc_root_opt: XvcRootOpt, cli_opts: XvcCLI) -> PyResult<PyCommandOutput> {
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

    let command_output = thread::scope(move |s| {
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
                        XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Error => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Warn(_) => {}
                        XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                        XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Warn => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Warn(m) => output_str.push_str(&format!("[WARN] {}", m)),
                        XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                        XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Info => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Info(m) => output_str.push_str(&format!("[INFO] {}", m)),
                        XvcOutputLine::Warn(m) => output_str.push_str(&format!("[WARN] {}", m)),
                        XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                        XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Debug => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Info(m) => output_str.push_str(&format!("[INFO] {}", m)),
                        XvcOutputLine::Warn(m) => output_str.push_str(&format!("[WARN] {}", m)),
                        XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                        XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                        XvcOutputLine::Debug(m) => output_str.push_str(&format!("[DEBUG] {}", m)),
                        XvcOutputLine::Tick(_) => todo!(),
                    },
                    LevelFilter::Trace => match output_line {
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Info(m) => output_str.push_str(&format!("[INFO] {}", m)),
                        XvcOutputLine::Warn(m) => output_str.push_str(&format!("[WARN] {}", m)),
                        XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                        XvcOutputLine::Debug(m) => output_str.push_str(&format!("[DEBUG] {}", m)),
                        XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
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

        let command_thread = s.spawn(move |_| -> PyResult<XvcRootOpt> {
            let res_xvc_root_opt: Result<XvcRootOpt> = match cli_opts.command {
                XvcSubCommand::Init(opts) => {
                    let use_git = !opts.no_git;
                    let xvc_root = init::run(xvc_root_opt.as_ref(), opts).map_err(XvcPyError)?;
                    // FIXME: Do we need a separate xvc_root.record() here to record the EC state?
                    if use_git {
                        handle_git_automation(
                            &output_snd,
                            &xvc_root,
                            cli_opts.to_branch.as_deref(),
                            &cli_opts.command_string,
                        )
                        .map_err(XvcPyError)?;
                    }
                    Ok(Some(xvc_root))
                }

                XvcSubCommand::Aliases(opts) => {
                    aliases::run(&output_snd, opts).map_err(|e| XvcPyError(e.into()))?;
                    Ok(xvc_root_opt)

                }

                // following commands can only be run inside a repository
                XvcSubCommand::Root(opts) => { 
                    root::run(
                    &output_snd,
                    xvc_root_opt
                        .as_ref()
                        .ok_or_else(|| XvcPyError(Error::RequiresXvcRepository))?,
                    opts,
                )
                .map_err(|e| XvcPyError(e.into()))?;

                    Ok(xvc_root_opt) }

                XvcSubCommand::File(opts) => {
                    file::run(&output_snd, xvc_root_opt.as_ref(), opts)
                        .map_err(|e| XvcPyError(e.into()))?;
                    Ok(xvc_root_opt)
                }

                XvcSubCommand::Pipeline(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();
                    pipeline::cmd_pipeline(
                        input,
                        &output_snd,
                        xvc_root_opt
                            .as_ref()
                            .ok_or(XvcPyError(Error::RequiresXvcRepository))?,
                        opts,
                    )
                    .map_err(|e| XvcPyError(e.into()))?;
                    Ok(xvc_root_opt)
                }

                XvcSubCommand::CheckIgnore(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();

                    check_ignore::cmd_check_ignore(
                        input,
                        &output_snd,
                        xvc_root_opt
                            .as_ref()
                            .ok_or(XvcPyError(Error::RequiresXvcRepository))?,
                        opts,
                    )
                    .map_err(|e| XvcPyError(e.into()))?;

                    Ok(xvc_root_opt)
                }

                XvcSubCommand::Storage(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();
                    storage::cmd_storage(
                        input,
                        &output_snd,
                        xvc_root_opt
                            .as_ref()
                            .ok_or(XvcPyError(Error::RequiresXvcRepository))?,
                        opts,
                    )
                    .map_err(|e| XvcPyError(e.into()))?;

                    Ok(xvc_root_opt)
                }
            };

            let xvc_root_opt = match res_xvc_root_opt {
                Ok(xvc_root_opt) => xvc_root_opt, 
                Err(e) => { error!(&output_snd, "{}", e); None
                },
            };

            if let Some(ref xvc_root) = xvc_root_opt { 
                if !cli_opts.skip_git {
                    xvc_root.record();
                    handle_git_automation(
                        &output_snd,
                        xvc_root,
                        cli_opts.to_branch.as_deref(),
                        &cli_opts.command_string,
                        // FIXME: Handle this error more gracefully
                    ).unwrap();
            }
            }


            assert!(xvc_root_opt.is_some());
            Ok(xvc_root_opt)
        }).join();

        let xvc_root_opt= match command_thread.unwrap() {
            Ok(xvc_root_opt) => { debug!(output_snd_clone, "Command completed successfully."); 
            xvc_root_opt }
            Err(e) => { error!(output_snd_clone, "{}", e); None } 
        };

        output_snd_clone.send(None).unwrap();
        let output = output_thread.join().unwrap();

        let res = PyCommandOutput { output, xvc_root_opt };
        Ok(res)

    })
    .unwrap();

    command_output
}
