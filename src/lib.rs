pub mod file;
pub mod output;
pub mod pipeline;
pub mod storage;

use std::cell::RefCell;

use file::XvcFile;
use output::dispatch_with_root;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use xvc_rust::cli::XvcSubCommand;
use xvc_rust::core::default_project_config;
use xvc_rust::core::root::RootCLI;
use xvc_rust::core::types::xvcroot::load_xvc_root;
use xvc_rust::{cli, watch, AbsolutePath, XvcConfigParams, XvcRootOpt};
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
    let cli_opts = match cli::XvcCLI::from_str_slice(&args) {
        Ok(opts) => opts,
        Err(e) => {
            return Ok(e.to_string());
        }
    };


    let xvc_config_params = XvcConfigParams {
        current_dir: AbsolutePath::from(&cli_opts.workdir),
        include_system_config: !cli_opts.no_system_config,
        include_user_config: !cli_opts.no_user_config,
        project_config_path: None,
        local_config_path: None,
        include_environment_config: !cli_opts.no_env_config,
        command_line_config: Some(cli_opts.consolidate_config_options()),
        default_configuration: default_project_config(true),
    };

    let xvc_root_opt = match load_xvc_root(xvc_config_params) {
        Ok(r) => Some(r),
        Err(e) => {
            e.debug();
            None
        }
    };

    watch!(cli_opts);
    let py_output = dispatch_with_root(xvc_root_opt, cli_opts)?;

    Ok(py_output.output)
}

struct XvcPyError(XvcError);

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
    xvc_config_params: XvcConfigParams,
    verbosity: Option<u8>,
    quiet: Option<bool>,
    debug: Option<bool>,
    workdir: Option<String>,
    skip_git: Option<bool>,
    from_ref: Option<String>,
    to_branch: Option<String>,
    xvc_root_opt: RefCell<XvcRootOpt>,
}

impl Xvc { 
    fn run(&self, args: Vec<String>) -> PyResult<String> {
    let cli_opts = match cli::XvcCLI::from_str_slice(&args.iter().map(|s| s.as_str()).collect::<Vec<&str>>()) {
        Ok(opts) => opts,
        Err(e) => {
            let out = e.to_string();
            return Ok(out)
        }
    };


    watch!(cli_opts);


    let cmd = &cli_opts.command.clone();
    let xvc_root_opt = self.xvc_root_opt.take();
    watch!(&xvc_root_opt);

    let out = dispatch_with_root(xvc_root_opt, cli_opts)?;

    watch!(&out.xvc_root_opt);
    self.xvc_root_opt.replace(out.xvc_root_opt);
    
    Ok(out.output)
}
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
        no_env_config: Option<bool>,
        skip_git: Option<bool>,
        from_ref: Option<String>,
        to_branch: Option<String>,
    ) -> PyResult<Self> {

    let xvc_config_params = XvcConfigParams {
        current_dir: AbsolutePath::from(workdir.clone().unwrap_or_else(|| ".".to_owned())),
        include_system_config: !no_system_config.unwrap_or_default(),
        include_user_config: !no_user_config.unwrap_or_default(),
        project_config_path: None,
        local_config_path: None,
        include_environment_config: !no_env_config.unwrap_or_default(),
        command_line_config: None,
        default_configuration: default_project_config(true),
    };

        watch!(xvc_config_params);
        
    let xvc_root_opt = match load_xvc_root(xvc_config_params.clone()) {
        Ok(r) => RefCell::new(Some(r)),
        Err(e) => {
            e.debug();
            RefCell::new(None)
        }
    };
        watch!(&xvc_root_opt);

        Ok(Self {
            xvc_config_params,
            verbosity,
            quiet,
            debug,
            workdir,
            skip_git,
            from_ref,
            to_branch,
            xvc_root_opt
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

        if !self.xvc_config_params.include_system_config {
            cli_opts.push("--no-system-config".to_string());
        }

        if !self.xvc_config_params.include_user_config{
            cli_opts.push("--no-user-config".to_string());
        }

        // TODO: We don't consider project and local config options for now.
        //if !self.xvc_config_params.include_project_config {
        //    cli_opts.push("--no-project-config".to_string());
        //}

        if !self.xvc_config_params.include_environment_config {
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
        update_cli_flag(opts, &mut cli_opts, &["absolute"], "--absolute")?;
        watch!(cli_opts);
        assert!(self.xvc_root_opt.borrow().is_some());
        self.run(cli_opts)
    }

    #[pyo3(signature = (*targets, **opts))]
    fn check_ignore(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
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

        self.run(cli_opts)
    }

    /// Initialize an Xvc project
    #[pyo3(signature = (**opts))]
    fn init(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("init".to_string());

        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;
        update_cli_opt(opts, &mut cli_opts, &["path"], "--path")?;
        update_cli_flag(opts, &mut cli_opts, &["no-git"], "--no-git")?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;

        let out = self.run(cli_opts);
        assert!(self.xvc_root_opt.borrow().is_some());

        out

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

pub fn update_targets(tuple: &Bound<PyTuple>, cli: &mut Vec<String>) -> PyResult<()> {
    for target in tuple.iter() {
        cli.push(target.extract::<String>()?);
    }
    Ok(())
}
