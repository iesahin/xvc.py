pub mod output;
pub mod pipeline;
pub mod storage;

use std::collections::HashMap;

use output::dispatch;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use xvc_rust::cli;
use xvc_rust::error::Error as XvcError;

pub use pipeline::XvcPipeline;
pub use storage::XvcStorage;

/// Call Xvc with the command line arguments
#[pyfunction]
pub fn run(args: Vec<&str>) -> PyResult<String> {
    let opts = cli::XvcCLI::from_str_slice(args.as_ref()).map_err(|e| XvcPyError(e.into()))?;
    dispatch(opts)
}

struct XvcPyError(XvcError);

impl From<XvcPyError> for PyErr {
    fn from(error: XvcPyError) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error.0.to_string())
    }
}

#[pymodule]
fn xvc(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Xvc>()?;
    // register_file_module(py, m)?;
    // register_storage_module(py, m)?;
    // register_pipeline_module(py, m)?;
    Ok(())
}

struct XvcOptions {
    pub str_opt: HashMap<String, Option<String>>,
    pub bool_opt: HashMap<String, Option<bool>>,
    pub u8_opt: HashMap<String, Option<u8>>,
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
        let mut cli_opts = Vec::new();
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
            cli_opts.push("--workdir".to_string());
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

        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn check_ignore(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("check-ignore".to_string());

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
    fn init(
        &self,
        path: Option<String>,
        no_git: Option<bool>,
        force: Option<bool>,
    ) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("init".to_string());

        update_cli_opt(None, &mut cli_opts, &["path"], "--path")?;
        update_cli_flag(None, &mut cli_opts, &["no-git"], "--no-git")?;
        update_cli_flag(None, &mut cli_opts, &["force"], "--force")?;

        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }
}

#[pyclass]
struct XvcFile {
    xvc_opts: Xvc,
}

impl XvcFile {
    fn new(xvc_opts: &Xvc) -> PyResult<Self> {
        Ok(Self {
            xvc_opts: xvc_opts.clone(),
        })
    }

    fn cli(&self) -> PyResult<Vec<String>> {
        let mut cli_opts = self.xvc_opts.cli()?;
        cli_opts.push("file".to_string());
        Ok(cli_opts)
    }
}

#[pymethods]
impl XvcFile {
    #[pyo3( signature = (*targets, **opts))]
    fn track(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("track".to_string());

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["recheck-method", "recheck_method"],
            "--recheck-method",
        )?;
        update_cli_flag(
            opts,
            &mut cli_opts,
            &["no-commit", "no_commit"],
            "--no-commit",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["text-or-binary", "text_or_binary"],
            "--text-or-binary",
        )?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(
            opts,
            &mut cli_opts,
            &["no-parallel", "no_parallel"],
            "--no-parallel",
        )?;

        update_targets(targets, cli_opts.as_mut())?;

        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn hash(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("hash".to_string());

        update_cli_opt(opts, &mut cli_opts, &["algorithm"], "--algorithm")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["text-or-binary", "text_or_binary"],
            "--text-or-binary",
        )?;
        update_targets(targets, &mut cli_opts)?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn carry_in(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("carry-in".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["text-or-binary", "text_or_binary"],
            "--text-or-binary",
        )?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(
            opts,
            &mut cli_opts,
            &["no-parallel", "no_parallel"],
            "--no-parallel",
        )?;
        update_targets(targets, &mut cli_opts)?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn recheck(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("recheck".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["recheck-method", "recheck_method"],
            "--recheck-method",
        )?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(
            opts,
            &mut cli_opts,
            &["no-parallel", "no_parallel"],
            "--no-parallel",
        )?;
        update_targets(targets, &mut cli_opts)?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn list(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("list".to_string());
        update_cli_opt(opts, &mut cli_opts, &["format"], "--format")?;
        update_cli_opt(opts, &mut cli_opts, &["sort"], "--sort")?;
        update_cli_flag(opts, &mut cli_opts, &["no-summary"], "--no-summary")?;
        update_targets(targets, &mut cli_opts)?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn send(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("send".to_string());
        update_cli_opt(opts, &mut cli_opts, &["remote, to"], "--remote")?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_targets(targets, &mut cli_opts)?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn bring(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("bring".to_string());
        update_cli_opt(opts, &mut cli_opts, &["remote", "from"], "--remote")?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(opts, &mut cli_opts, &["no-recheck"], "--no-recheck")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["recheck-as", "recheck_as"],
            "--recheck-as",
        )?;
        update_targets(targets, &mut cli_opts)?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (**opts))]
    fn copy(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("copy".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["recheck-method", "recheck_method"],
            "--recheck-method",
        )?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(opts, &mut cli_opts, &["no-recheck"], "--no-recheck")?;
        update_cli_flag(opts, &mut cli_opts, &["no-recheck"], "--no-recheck")?;
        update_cli_opt(opts, &mut cli_opts, &["source"], "--source")?;
        update_cli_opt(opts, &mut cli_opts, &["destination"], "--destination")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (**opts))]
    fn move_(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("move".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["recheck-method", "recheck_method"],
            "--recheck-method",
        )?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(opts, &mut cli_opts, &["no-recheck"], "--no-recheck")?;
        update_cli_opt(opts, &mut cli_opts, &["source"], "--source")?;
        update_cli_opt(opts, &mut cli_opts, &["destination"], "--destination")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn untrack(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("untrack".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["restore-versions", "restore_versions"],
            "--recheck-method",
        )?;
        update_targets(targets, &mut cli_opts)?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn remove(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("remove".to_string());
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["from-cache", "from_cache"],
            "--from-cache",
        )?;
        update_cli_flag(
            opts,
            &mut cli_opts,
            &["all_versions", "all-versions"],
            "--all-versions",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["only_version", "only-version"],
            "--only-version",
        )?;
        update_targets(targets, &mut cli_opts)?;
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
