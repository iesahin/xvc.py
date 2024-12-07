use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{update_cli_flag, update_cli_opt};
use crate::{update_cli_tuple, Xvc};

#[pyclass]
#[derive(Clone, Debug)]
pub struct XvcPipeline {
    xvc_opts: Xvc,
    pipeline_name: Option<String>,
}

impl XvcPipeline {
    pub fn init(xvc_opts: &Xvc, pipeline_name: Option<String>) -> PyResult<Self> {
        Ok(Self {
            xvc_opts: xvc_opts.clone(),
            pipeline_name,
        })
    }

    fn cli(&self) -> PyResult<Vec<String>> {
        let mut cli_opts = self.xvc_opts.cli()?;
        cli_opts.push("pipeline".to_string());
        if let Some(pipeline_name) = &self.pipeline_name {
            cli_opts.push("--pipeline-name".to_string());
            cli_opts.push(pipeline_name.to_string());
        }

        Ok(cli_opts)
    }

    fn xvc_run(&self, args: Vec<String>) -> PyResult<String> {
        self.xvc_opts.run(args)
    }
}

#[pymethods]
impl XvcPipeline {
    #[pyo3(signature = (**opts))]
    #[allow(clippy::new_ret_no_self)]
    #[allow(clippy::wrong_self_convention)]
    fn new(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["workdir"], "--workdir")?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn update(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("update".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["rename"], "--rename")?;
        update_cli_opt(opts, &mut cli_opts, &["workdir"], "--workdir")?;
        update_cli_flag(
            opts,
            &mut cli_opts,
            &["set_default", "set-default"],
            "--set-default",
        )?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn delete(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("delete".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn run(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("run".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn list(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("list".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn dag(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("dag".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--file")?;
        update_cli_opt(opts, &mut cli_opts, &["format"], "--format")?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn export(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("export".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--file")?;
        update_cli_opt(opts, &mut cli_opts, &["format"], "--format")?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn import_pipeline(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("import".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--file")?;
        update_cli_opt(opts, &mut cli_opts, &["format"], "--format")?;
        update_cli_flag(opts, &mut cli_opts, &["overwrite"], "--overwrite")?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn step(&self, opts: Option<&Bound<PyDict>>) -> PyResult<XvcPipelineStep> {
        Ok(XvcPipelineStep {
            xvc_pipeline_opts: self.clone(),
        })
    }
}

#[pyclass]
struct XvcPipelineStep {
    xvc_pipeline_opts: XvcPipeline,
}

impl XvcPipelineStep {
    fn cli(&self) -> PyResult<Vec<String>> {
        let mut cli_opts = self.xvc_pipeline_opts.cli()?;
        cli_opts.push("step".to_string());
        Ok(cli_opts)
    }

    fn xvc_run(&self, args: Vec<String>) -> PyResult<String> {
        self.xvc_pipeline_opts.xvc_run(args)
    }
}

#[pymethods]
impl XvcPipelineStep {
    #[pyo3(signature = (**opts))]
    #[allow(clippy::wrong_self_convention)]
    #[allow(clippy::new_ret_no_self)]
    fn new(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["command"], "--command")?;
        update_cli_opt(opts, &mut cli_opts, &["when"], "--when")?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn update(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("update".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["command"], "--command")?;
        update_cli_opt(opts, &mut cli_opts, &["when"], "--when")?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn dependency(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("dependency".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;
        update_cli_flag(opts, &mut cli_opts, &["no_recheck"], "--no-recheck")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--file")?;
        update_cli_opt(opts, &mut cli_opts, &["url"], "--url")?;
        update_cli_opt(opts, &mut cli_opts, &["glob"], "--glob")?;
        update_cli_opt(opts, &mut cli_opts, &["glob_items"], "--glob-items")?;
        update_cli_opt(opts, &mut cli_opts, &["step"], "--step")?;
        update_cli_opt(opts, &mut cli_opts, &["param"], "--param")?;
        update_cli_opt(opts, &mut cli_opts, &["regex"], "--regex")?;
        update_cli_opt(opts, &mut cli_opts, &["regex_items"], "--regex-items")?;
        update_cli_opt(opts, &mut cli_opts, &["line", "lines"], "--line")?;
        update_cli_opt(opts, &mut cli_opts, &["line_items"], "--line-items")?;
        update_cli_opt(opts, &mut cli_opts, &["generic"], "--generic")?;
        update_cli_tuple(
            opts,
            &mut cli_opts,
            ("sqlite_file", "sqlite_query"),
            "--sqlite-query",
        )?;

        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn output(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("output".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;
        update_cli_flag(opts, &mut cli_opts, &["no-recheck"], "--no-recheck")?;

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--output-file")?;
        update_cli_opt(opts, &mut cli_opts, &["metric"], "--output-metric")?;
        update_cli_opt(opts, &mut cli_opts, &["image"], "--output-images")?;
        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn list(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("list".to_string());
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;
        update_cli_flag(opts, &mut cli_opts, &["names_only"], "--names-only")?;

        self.xvc_run(cli_opts)
    }

    #[pyo3(signature = (**opts))]
    fn show(&self, opts: Option<&Bound<PyDict>>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        update_cli_flag(opts, &mut cli_opts, &["help"], "--help")?;
        cli_opts.push("show".to_string());

        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        self.xvc_run(cli_opts)
    }
}
