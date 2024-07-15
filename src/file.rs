use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use xvc_rust::watch;

use crate::{update_cli_flag, update_cli_opt};
use crate::{update_targets, Xvc};

#[pyclass]
#[derive(Clone, Debug)]
pub struct XvcFile {
    xvc_opts: Xvc,
}

impl XvcFile {
    pub fn new(xvc_opts: &Xvc) -> PyResult<Self> {
        Ok(Self {
            xvc_opts: xvc_opts.clone(),
        })
    }

    fn cli(&self) -> PyResult<Vec<String>> {
        let mut cli_opts = self.xvc_opts.cli()?;
        cli_opts.push("file".to_string());
        Ok(cli_opts)
    }

    fn run(&self, args: Vec<String>) -> PyResult<String> {
        watch!(args);
        self.xvc_opts.run(args)
    }
}

#[pymethods]
impl XvcFile {
    #[pyo3( signature = (*targets, **opts))]
    fn track(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("track".to_string());

        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;
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
        watch!(self);

        self.run(cli_opts)
    }

    #[pyo3( signature = (*targets, **opts))]
    fn hash(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("hash".to_string());

        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;
        update_cli_opt(opts, &mut cli_opts, &["algorithm"], "--algorithm")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["text-or-binary", "text_or_binary"],
            "--text-or-binary",
        )?;
        update_targets(targets, &mut cli_opts)?;
        self.run(cli_opts)
    }

    #[pyo3( signature = (*targets, **opts))]
    fn carry_in(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("carry-in".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

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
        self.run(cli_opts)
    }

    #[pyo3( signature = (*targets, **opts))]
    fn recheck(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("recheck".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

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
        self.run(cli_opts)
    }

    #[pyo3( signature = (*targets, **opts))]
    fn list(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("list".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(opts, &mut cli_opts, &["format"], "--format")?;
        update_cli_opt(opts, &mut cli_opts, &["sort"], "--sort")?;
        update_cli_flag(opts, &mut cli_opts, &["no-summary"], "--no-summary")?;
        update_targets(targets, &mut cli_opts)?;
        self.run(cli_opts)
    }

    #[pyo3( signature = (*targets, **opts))]
    fn send(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("send".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(opts, &mut cli_opts, &["remote", "to", "storage"], "--storage")?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_targets(targets, &mut cli_opts)?;
        self.run(cli_opts)
    }

    #[pyo3( signature = (*targets, **opts))]
    fn bring(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("bring".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(opts, &mut cli_opts, &["remote", "frm", "storage"], "--storage")?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(opts, &mut cli_opts, &["no-recheck"], "--no-recheck")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["recheck-as", "recheck_as"],
            "--recheck-as",
        )?;
        update_targets(targets, &mut cli_opts)?;
        self.run(cli_opts)
    }

    #[pyo3( signature = (source, destination, **opts))]
    fn copy(&self, source: String, destination: String, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("copy".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["recheck-method", "recheck_method"],
            "--recheck-method",
        )?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(opts, &mut cli_opts, &["no-recheck"], "--no-recheck")?;
        cli_opts.push(source);
        cli_opts.push(destination);
        self.run(cli_opts)
    }

    #[pyo3( signature = (source, destination, **opts))]
    fn mv(&self, source: String, destination: String, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("move".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["recheck-method", "recheck_method"],
            "--recheck-method",
        )?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(opts, &mut cli_opts, &["no-recheck"], "--no-recheck")?;
        cli_opts.push(source);
        cli_opts.push(destination);
        self.run(cli_opts)
    }

    #[pyo3( signature = (*targets, **opts))]
    fn untrack(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("untrack".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["restore-versions", "restore_versions"],
            "--recheck-method",
        )?;
        update_targets(targets, &mut cli_opts)?;
        self.run(cli_opts)
    }

    #[pyo3( signature = (*targets, **opts))]
    fn remove(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("remove".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_cli_flag(
            opts,
            &mut cli_opts,
            &["from-cache", "from_cache"],
            "--from-cache",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["from-storage", "from_storage"],
            "--from-storage",
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
        self.run(cli_opts)
    }

    #[pyo3( signature = (*targets, **opts))]
    fn share(&self, targets: &Bound<PyTuple>, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("share".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(opts, &mut cli_opts, &["remote", "storage"], "--storage")?;
        update_cli_opt(opts, &mut cli_opts, &["duration"], "--duration")?;
        update_targets(targets, &mut cli_opts)?;
        self.run(cli_opts)
    }
}
