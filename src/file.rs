use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use crate::{run, update_cli_flag, update_cli_opt};
use crate::{update_targets, Xvc};

#[pyclass]
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
}

#[pymethods]
impl XvcFile {
    #[pyo3( signature = (*targets, **opts))]
    fn track(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
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

        println!("{:?}", cli_opts);

        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn hash(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
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
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn carry_in(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
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
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn recheck(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
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
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn list(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("list".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

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
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(opts, &mut cli_opts, &["remote, to"], "--remote")?;
        update_cli_flag(opts, &mut cli_opts, &["force"], "--force")?;
        update_targets(targets, &mut cli_opts)?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn bring(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("bring".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

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
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

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

    #[pyo3( signature = (**opts))]
    fn move_(&self, opts: Option<&PyDict>) -> PyResult<String> {
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
        update_cli_opt(opts, &mut cli_opts, &["source"], "--source")?;
        update_cli_opt(opts, &mut cli_opts, &["destination"], "--destination")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn untrack(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
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
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    #[pyo3( signature = (*targets, **opts))]
    fn remove(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("remove".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

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

    #[pyo3( signature = (*targets, **opts))]
    fn share(&self, targets: &PyTuple, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("share".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(opts, &mut cli_opts, &["remote"], "--remote")?;
        update_cli_opt(opts, &mut cli_opts, &["duration"], "--duration")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }
}
