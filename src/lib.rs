pub mod file;
pub mod output;
pub mod pipeline;
pub mod storage;

use file::XvcFile;
use output::dispatch;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use xvc_rust::cli;
use xvc_rust::error::Error as XvcError;

pub use pipeline::XvcPipeline;
pub use storage::XvcStorage;

use git_version::git_version;
const GIT_VERSION: &str = git_version!(cargo_prefix = "", fallback = "unknown");


#[pyfunction]
pub fn version() -> PyResult<String> {
    Ok(GIT_VERSION.to_owned())
}

/// Call Xvc with the command line arguments
#[pyfunction]
pub fn run_xvc(cmd: String) -> PyResult<String> {
    let args: Vec<&str> = cmd.split(' ').collect();
    let opts = match cli::XvcCLI::from_str_slice(&args) {
        Ok(opts) => opts,
        Err(e) => {
            return Ok(e.to_string());
        }
    };

    println!("{:?}", opts);
    dispatch(opts)
}

pub fn run(args: Vec<&str>) -> PyResult<String> {

    let opts = match cli::XvcCLI::from_str_slice(&args) {
        Ok(opts) => opts,
        Err(e) => {
            return Ok(e.to_string());
        }
    };

    println!("{:?}", opts);
    dispatch(opts)
}

struct XvcPyError(XvcError);

impl From<XvcPyError> for PyErr {
    fn from(error: XvcPyError) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error.0.to_string())
    }
}

#[pymodule]
fn xvc(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Xvc>()?;
    m.add_function(wrap_pyfunction!(run_xvc, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

#[pyclass]
#[derive(Clone)]
pub struct Xvc {
    verbosity: Option<u8>,
    quiet: Option<bool>,
    debug: Option<bool>,
    workdir: Option<String>,
    no_system_config: Option<bool>,
    no_user_config: Option<bool>,
    no_project_config: Option<bool>,
    no_local_config: Option<bool>,
    no_env_config: Option<bool>,
    skip_git: Option<bool>,
    from_ref: Option<String>,
    to_branch: Option<String>,
}

#[pymethods]
impl Xvc {
    #[allow(clippy::too_many_arguments)]
    #[new]
    fn new(
        verbosity: Option<u8>,
        quiet: Option<bool>,
        debug: Option<bool>,
        workdir: Option<String>,
        no_system_config: Option<bool>,
        no_user_config: Option<bool>,
        no_project_config: Option<bool>,
        no_local_config: Option<bool>,
        no_env_config: Option<bool>,
        skip_git: Option<bool>,
        from_ref: Option<String>,
        to_branch: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            verbosity,
            quiet,
            debug,
            workdir,
            no_system_config,
            no_user_config,
            no_project_config,
            no_local_config,
            no_env_config,
            skip_git,
            from_ref,
            to_branch,
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

        if Some(true) == self.no_system_config {
            cli_opts.push("--no-system-config".to_string());
        }

        if Some(true) == self.no_user_config {
            cli_opts.push("--no-user-config".to_string());
        }

        if Some(true) == self.no_project_config {
            cli_opts.push("--no-project-config".to_string());
        }

        if Some(true) == self.no_local_config {
            cli_opts.push("--no-local-config".to_string());
        }

        if Some(true) == self.no_env_config {
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
        XvcFile::new(self)
    }

    fn storage(&self) -> PyResult<XvcStorage> {
        XvcStorage::init(self)
    }
    fn pipeline(&self, pipeline_name: Option<String>) -> PyResult<XvcPipeline> {
        XvcPipeline::init(self, pipeline_name)
    }
    fn root(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("root".to_string());
        update_cli_flag(opts, &mut cli_opts, &["absolute"], "--absolute")?;
        println!("{:?}", cli_opts);
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn check_ignore(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("check-ignore".to_string());

        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;
        update_cli_flag(opts, &mut cli_opts, &["details"], "--details")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["ignore_filename"],
            "--ignore-filename",
        )?;
        update_cli_flag(opts, &mut cli_opts, &["non_matching"], "--non-matching")?;

        update_targets(targets, &mut cli_opts)?;

        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    /// Initialize an Xvc project
    fn init(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("init".to_string());

        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;
        update_cli_opt(opts, &mut cli_opts, &["path"], "--path")?;
        update_cli_flag(opts, &mut cli_opts, &["no-git"], "--no-git")?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;

        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    /// Show help
    fn help(&self) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("help".to_string());
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }
}

fn get_bool(dict: Option<&PyDict>, keys: &[&str]) -> PyResult<Option<bool>> {
    if let Some(dict) = dict {
        for key in keys {
            if let Some(value) = dict.get_item(key)? {
                return Ok(Some(value.extract::<bool>()?));
            }
        }
    }
    Ok(None)
}

fn get_string(dict: Option<&PyDict>, keys: &[&str]) -> PyResult<Option<String>> {
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
    dict: Option<&PyDict>,
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
    dict: Option<&PyDict>,
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

pub fn update_targets(tuple: &PyTuple, cli: &mut Vec<String>) -> PyResult<()> {
    for target in tuple.iter() {
        cli.push(target.extract::<String>()?);
    }
    Ok(())
}
