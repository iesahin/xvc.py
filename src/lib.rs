pub mod file;
pub mod output;
pub mod pipeline;
pub mod storage;

use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

use file::XvcFile;
use output::dispatch_with_root;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use xvc_rust::core::types::xvcroot::load_xvc_root;
use xvc_rust::error::Error as XvcError;
use xvc_rust::{cli, watch, AbsolutePath, XvcLoadParams, XvcRootOpt};

pub use pipeline::XvcPipeline;
pub use storage::XvcStorage;

use git_version::git_version;
const GIT_VERSION: &str = git_version!(cargo_prefix = "", fallback = "unknown");

#[pyfunction]
pub fn version() -> PyResult<String> {
    Ok(GIT_VERSION.to_owned())
}

struct XvcPyError(XvcError);
type Result<T> = std::result::Result<T, XvcPyError>;
type XvcPyRootOpt = Arc<RwLock<XvcRootOpt>>;

/// Call Xvc with the command line arguments
#[pyfunction]
pub fn run_xvc(cmd: String) -> PyResult<String> {
    let args: Vec<&str> = cmd.split(' ').collect();
    let cli_opts = match cli::XvcCLI::from_str_slice(&args) {
        Ok(opts) => opts,
        Err(e) => {
            return Ok(e.to_string());
        }
    };

    let mut xvc_load_params = XvcLoadParams::new(
        AbsolutePath::from(cli_opts.workdir.as_deref().unwrap_or(Path::new("."))),
        None,
    );
    xvc_load_params.include_system_config = !cli_opts.no_system_config;
    xvc_load_params.include_user_config = !cli_opts.no_user_config;
    xvc_load_params.include_environment_config = !cli_opts.no_env_config;
    xvc_load_params.command_line_config = Some(cli_opts.consolidate_config_options());

    let xvc_root_opt = match load_xvc_root(xvc_load_params) {
        Ok(r) => Some(r),
        Err(e) => {
            e.debug();
            None
        }
    };

    watch!(cli_opts);
    let py_output = dispatch_with_root(&Arc::new(RwLock::new(xvc_root_opt)), cli_opts)?;

    Ok(py_output.output)
}

impl From<XvcPyError> for PyErr {
    fn from(error: XvcPyError) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error.0.to_string())
    }
}

#[pymodule]
fn xvc(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<Xvc>()?;
    m.add_function(wrap_pyfunction!(run_xvc, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Xvc {
    xvc_load_params: XvcLoadParams,
    verbosity: Option<u8>,
    quiet: Option<bool>,
    debug: Option<bool>,
    workdir: Option<String>,
    skip_git: Option<bool>,
    from_ref: Option<String>,
    to_branch: Option<String>,
    xvc_root_opt: Arc<RwLock<XvcRootOpt>>,
}

impl Xvc {
    fn run(&self, args: Vec<String>) -> PyResult<String> {
        let cli_opts = match cli::XvcCLI::from_str_slice(
            &args.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
        ) {
            Ok(opts) => opts,
            Err(e) => {
                let out = e.to_string();
                return Ok(out);
            }
        };

        watch!(cli_opts);

        let out = dispatch_with_root(&self.xvc_root_opt, cli_opts)?;

        Ok(out.output)
    }
}

#[pymethods]
impl Xvc {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = 
            (verbosity=None, 
             quiet=None, 
             debug=None, 
             workdir=None, 
             no_system_config=None, 
             no_user_config=None,
             no_env_config=None, 
             skip_git=None, 
             from_ref=None, 
             to_branch=None))]
    fn new(
        verbosity: Option<u8>,
        quiet: Option<bool>,
        debug: Option<bool>,
        workdir: Option<String>,
        no_system_config: Option<bool>,
        no_user_config: Option<bool>,
        no_env_config: Option<bool>,
        skip_git: Option<bool>,
        from_ref: Option<String>,
        to_branch: Option<String>,
    ) -> PyResult<Self> {
        let mut xvc_load_params = XvcLoadParams::new(
            AbsolutePath::from(workdir.clone().unwrap_or_else(|| ".".to_owned())),
            None,
        );
        xvc_load_params.include_system_config = !no_system_config.unwrap_or_default();
        xvc_load_params.include_user_config = !no_user_config.unwrap_or_default();
        xvc_load_params.include_environment_config = !no_env_config.unwrap_or_default();

        watch!(xvc_load_params);

        let xvc_root_opt = match load_xvc_root(xvc_load_params.clone()) {
            Ok(r) => Arc::new(RwLock::new(Some(r))),
            Err(e) => {
                e.debug();
                Arc::new(RwLock::new(None))
            }
        };
        watch!(&xvc_root_opt);

        Ok(Self {
            xvc_load_params,
            verbosity,
            quiet,
            debug,
            workdir,
            skip_git,
            from_ref,
            to_branch,
            xvc_root_opt,
        })
    }

    fn cli(&self) -> PyResult<Vec<String>> {
        let mut cli_opts = vec!["xvc".to_string()];
        if let Some(verbosity) = self.verbosity {
            cli_opts.push(format!("-{}", "v".repeat(verbosity as usize)));
        }
        if Some(true) == self.quiet {
            cli_opts.push("--quiet".to_string());
        }

        if Some(true) == self.debug {
            cli_opts.push("--debug".to_string());
        }

        if let Some(workdir) = &self.workdir {
            cli_opts.push("-C".to_string());
            cli_opts.push(workdir.to_string());
        }

        if !self.xvc_load_params.include_system_config {
            cli_opts.push("--no-system-config".to_string());
        }

        if !self.xvc_load_params.include_user_config {
            cli_opts.push("--no-user-config".to_string());
        }

        // TODO: We don't consider project and local config options for now.
        //if !self.xvc_load_params.include_project_config {
        //    cli_opts.push("--no-project-config".to_string());
        //}

        if !self.xvc_load_params.include_environment_config {
            cli_opts.push("--no-env-config".to_string());
        }

        if Some(true) == self.skip_git {
            cli_opts.push("--skip-git".to_string());
        }

        if let Some(from_ref) = &self.from_ref {
            cli_opts.push("--from-ref".to_string());
            cli_opts.push(from_ref.to_string());
        }

        if let Some(to_branch) = &self.to_branch {
            cli_opts.push("--to-branch".to_string());
            cli_opts.push(to_branch.to_string());
        }
        Ok(cli_opts)
    }

    fn file(&self) -> PyResult<XvcFile> {
        watch!(self);
        XvcFile::new(self)
    }

    fn storage(&self) -> PyResult<XvcStorage> {
        XvcStorage::init(self)
    }

    #[pyo3(signature = (pipeline_name=None))]
    fn pipeline(&self, pipeline_name: Option<String>) -> PyResult<XvcPipeline> {
        XvcPipeline::init(self, pipeline_name)
    }

    #[pyo3(signature = (**opts))]
    fn root(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("root".to_string());
        update_cli!(opts, &mut cli_opts, flag ["absolute"] => "--absolute");
        watch!(cli_opts);
        self.run(cli_opts)
    }

    #[pyo3(signature = (*targets, **opts))]
    fn check_ignore(
        &self,
        targets: &Bound<PyTuple>,
        opts: Option<&Bound<PyDict>>,
    ) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("check-ignore".to_string());

        update_cli!(opts, &mut cli_opts, flag ["help"] => "--help");
        update_cli!(opts, &mut cli_opts, ["ignore_filename", "ignore-filename"] => "--ignore-filename");

        update_targets(targets, &mut cli_opts)?;

        self.run(cli_opts)
    }

    /// Initialize an Xvc project
    #[pyo3(signature = (**opts))]
    fn init(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("init".to_string());

        update_cli!(opts, &mut cli_opts, flag ["help"] => "--help");
        update_cli!(opts, &mut cli_opts, ["path"] => "--path");
        update_cli!(opts, &mut cli_opts, flag ["no-git", "no_git"] => "--no-git");
        update_cli!(opts, &mut cli_opts, flag ["force"] => "--force");

        watch!(self.xvc_root_opt.read().unwrap());
        self.run(cli_opts)
    }

    /// Show help
    fn help(&self) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("help".to_string());
        self.run(cli_opts)
    }
}

fn get_bool(dict: Option<&Bound<PyDict>>, keys: &[&str]) -> PyResult<Option<bool>> {
    if let Some(dict) = dict {
        for key in keys {
            if let Some(value) = dict.get_item(key)? {
                return Ok(Some(value.extract::<bool>()?));
            }
        }
    }
    Ok(None)
}

fn get_string(dict: Option<&Bound<PyDict>>, keys: &[&str]) -> PyResult<Option<String>> {
    if let Some(dict) = dict {
        for key in keys {
            if let Some(value) = dict.get_item(key)? {
                return Ok(Some(value.extract::<String>()?));
            }
        }
    }
    Ok(None)
}

pub fn update_cli_flag(
    dict: Option<&Bound<PyDict>>,
    cli: &mut Vec<String>,
    keys: &[&str],
    cli_flag: &str,
) -> PyResult<()> {
    if let Some(value) = get_bool(dict, keys)? {
        if value {
            cli.push(cli_flag.to_string());
        }
    }
    Ok(())
}

pub fn update_cli_opt(
    dict: Option<&Bound<PyDict>>,
    cli: &mut Vec<String>,
    keys: &[&str],
    cli_opt: &str,
) -> PyResult<()> {
    if let Some(value) = get_string(dict, keys)? {
        cli.push(cli_opt.to_string());
        cli.push(value.to_string());
    }
    Ok(())
}

pub fn update_cli_tuple(
    dict: Option<&Bound<PyDict>>,
    cli: &mut Vec<String>,
    keys: (&str, &str),
    cli_opt: &str,
) -> PyResult<()> {
    let (first_key, second_key) = keys;
    if let Some(first_value) = get_string(dict, &[first_key])? {
        if let Some(second_value) = get_string(dict, &[second_key])? {
            cli.push(cli_opt.to_string());
            cli.push(first_value.to_string());
            cli.push(second_value.to_string());
        }
    }
    Ok(())
}

#[macro_export]
macro_rules! update_cli {
    ($opts:expr, $cli_opts:expr, flag [ $($keys:expr),+ ] => $flag:expr) => {
        $crate::update_cli_flag($opts, $cli_opts, &[$($keys),+], $flag)?;
    };
    ($opts:expr, $cli_opts:expr, [ $($keys:expr),+ ] => $flag:expr) => {
        $crate::update_cli_opt($opts, $cli_opts, &[$($keys),+], $flag)?;
    };
    ($opts:expr, $cli_opts:expr, tuple ( $key1:expr, $key2:expr ) => $flag:expr) => {
        $crate::update_cli_tuple($opts, $cli_opts, ($key1, $key2), $flag)?;
    };
}

pub fn update_targets(tuple: &Bound<PyTuple>, cli: &mut Vec<String>) -> PyResult<()> {
    for target in tuple.iter() {
        cli.push(target.extract::<String>()?);
    }
    Ok(())
}
