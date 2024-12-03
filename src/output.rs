use std::{
    io,
    sync::{Arc, RwLock},
};

use crossbeam::thread;
use crossbeam_channel::bounded;
use log::LevelFilter;
use pyo3::PyResult;
use xvc_logging::XvcOutputSender;
use xvc_rust::{
    cli::{XvcCLI, XvcSubCommand},
    config::XvcVerbosity,
    core::{
        aliases, check_ignore, git_checkout_ref, handle_git_automation, root, Error as XvcCoreError,
    },
    error, file, init,
    logging::{debug, setup_logging, uwr, XvcOutputLine},
    pipeline, storage, Error as XvcError, XvcRootOpt,
};

use crate::XvcPyError;
use crate::{Result, XvcPyRootOpt};

const CHANNEL_BOUND: usize = 10000;

pub struct PyCommandOutput {
    pub output: String,
}

/// Runs the supplied xvc command.
pub fn run(xvc_root_opt: &XvcPyRootOpt, args: &[&str]) -> PyResult<PyCommandOutput> {
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
pub fn dispatch_with_root(
    xvc_root_opt: &Arc<RwLock<XvcRootOpt>>,
    cli_opts: XvcCLI,
) -> PyResult<PyCommandOutput> {
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

        if let Some(from_ref) = cli_opts.from_ref {
            let xvc_root_opt = xvc_root_opt.read().expect("Lock xvc_root").to_owned();
            if let Some(ref xvc_root) = xvc_root_opt {
                uwr!(
                    git_checkout_ref(&output_snd, xvc_root, from_ref),
                    output_snd
                );
            }
        }

        let command_thread = s
            .spawn(move |_| -> PyResult<()> {
                match cli_opts.command {
                    XvcSubCommand::Init(opts) => {
                        let to_branch = cli_opts.to_branch.as_deref();
                        let xvc_cmd = cli_opts.command_string.as_ref();
                        handle_init(&output_snd, xvc_root_opt, opts, to_branch, xvc_cmd)?;
                    }

                    XvcSubCommand::Aliases(opts) => {
                        handle_aliases(&output_snd, opts)?;
                    }

                    // following commands can only be run inside a repository
                    XvcSubCommand::Root(opts) => handle_root(&output_snd, xvc_root_opt, opts)?,

                    XvcSubCommand::CheckIgnore(opts) => {
                        handle_check_ignore(&output_snd, xvc_root_opt, opts)?
                    }

                    XvcSubCommand::File(opts) => handle_file(&output_snd, xvc_root_opt, opts)?,

                    XvcSubCommand::Pipeline(opts) => {
                        handle_pipeline(&output_snd, xvc_root_opt, opts)?
                    }

                    XvcSubCommand::Storage(opts) => {
                        handle_storage(&output_snd, xvc_root_opt, opts)?
                    }
                };

                if !cli_opts.skip_git {
                    let xvc_root_opt = xvc_root_opt.read().expect("lock xvc_root").to_owned();
                    if let Some(xvc_root) = xvc_root_opt {
                        xvc_root.record();
                        handle_git_automation(
                            &output_snd,
                            &xvc_root,
                            cli_opts.to_branch.as_deref(),
                            &cli_opts.command_string,
                            // FIXME: Handle this error more gracefully
                        )
                        .unwrap();
                    }
                }

                Ok(())
            })
            .join();

        match command_thread.unwrap() {
            Ok(_) => {
                debug!(output_snd_clone, "Command completed successfully.");
            }
            Err(e) => {
                error!(output_snd_clone, "{}", e);
            }
        };

        output_snd_clone.send(None).unwrap();
        let output = output_thread.join().unwrap();

        Ok(PyCommandOutput { output })
    })
    .unwrap();

    command_output
}

fn handle_storage(
    output_snd: &XvcOutputSender,
    xvc_root_opt: &XvcPyRootOpt,
    opts: storage::StorageCLI,
) -> Result<()> {
    let stdin = io::stdin();
    let input = stdin.lock();
    {
        let xvc_root_opt = xvc_root_opt.read().expect("lock xvc_root").to_owned();
        storage::cmd_storage(
            input,
            output_snd,
            xvc_root_opt
                .as_ref()
                .ok_or(XvcPyError(XvcError::RequiresXvcRepository))?,
            opts,
        )
        .map_err(|e| XvcPyError(e.into()))?;
    }
    Ok(())
}

fn handle_check_ignore(
    output_snd: &XvcOutputSender,
    xvc_root_opt: &XvcPyRootOpt,
    opts: check_ignore::CheckIgnoreCLI,
) -> Result<()> {
    let stdin = io::stdin();
    let input = stdin.lock();
    {
        let xvc_root_opt = xvc_root_opt.read().expect("lock xvc_root").to_owned();
        check_ignore::cmd_check_ignore(
            input,
            output_snd,
            xvc_root_opt
                .as_ref()
                .ok_or(XvcPyError(XvcError::RequiresXvcRepository))?,
            opts,
        )
        .map_err(|e| XvcPyError(e.into()))?;
    }
    Ok(())
}

fn handle_pipeline(
    output_snd: &XvcOutputSender,
    xvc_root_opt: &XvcPyRootOpt,
    opts: pipeline::PipelineCLI,
) -> Result<()> {
    let stdin = io::stdin();
    let input = stdin.lock();
    {
        let xvc_root_opt = xvc_root_opt.read().expect("lock xvc_root").to_owned();
        pipeline::cmd_pipeline(
            input,
            output_snd,
            xvc_root_opt
                .as_ref()
                .ok_or(XvcPyError(XvcError::RequiresXvcRepository))?,
            opts,
        )
        .map_err(|e| XvcPyError(e.into()))?;
    }
    Ok(())
}

fn handle_file(
    output_snd: &XvcOutputSender,
    xvc_root_opt: &XvcPyRootOpt,
    opts: file::XvcFileCLI,
) -> Result<()> {
    let xvc_root_opt = xvc_root_opt.read().expect("lock xvc_root").to_owned();
    file::run(output_snd, xvc_root_opt.as_ref(), opts).map_err(|e| XvcPyError(e.into()))?;
    Ok(())
}

fn handle_root(
    output_snd: &XvcOutputSender,
    xvc_root_opt: &XvcPyRootOpt,
    opts: root::RootCLI,
) -> Result<()> {
    let xvc_root_opt = xvc_root_opt.read().expect("lock xvc_root").to_owned();
    root::run(
        output_snd,
        xvc_root_opt
            .as_ref()
            .ok_or_else(|| XvcPyError(XvcError::RequiresXvcRepository))?,
        opts,
    )
    .map_err(|e| XvcPyError(e.into()))?;
    Ok(())
}

fn handle_aliases(output_snd: &XvcOutputSender, opts: aliases::AliasesCLI) -> Result<()> {
    aliases::run(output_snd, opts).map_err(|e| XvcPyError(e.into()))?;
    Ok(())
}

fn handle_init(
    output_snd: &XvcOutputSender,
    xvc_root_opt: &XvcPyRootOpt,
    opts: init::InitCLI,
    to_branch: Option<&str>,
    xvc_cmd: &str,
) -> Result<()> {
    let use_git = !opts.no_git;

    {
        let mut xvc_root_opt = xvc_root_opt.write().expect("lock xvc_root");
        let xvc_root = init::run(xvc_root_opt.as_ref(), opts).map_err(XvcPyError)?;
        *xvc_root_opt = Some(xvc_root);
    }

    if use_git {
        let xvc_root_opt = xvc_root_opt.read().expect("lock xvc_root").to_owned();
        if let Some(ref xvc_root) = xvc_root_opt {
            handle_git_automation(output_snd, xvc_root, to_branch, xvc_cmd)
                .map_err(|e: XvcCoreError| XvcPyError(e.into()))?;
        }
    }

    Ok(())
}
