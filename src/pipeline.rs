use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::Xvc;
use crate::{update_cli_opt, update_cli_flag, run};

#[pyclass]
#[derive(Clone)]
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
}

#[pymethods]
impl XvcPipeline {
    fn new(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["workdir"], "--workdir")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn update(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("update".to_string());
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
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn delete(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("delete".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn run(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("run".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn list(&self) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("list".to_string());
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn dag(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("dag".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--file")?;
        update_cli_opt(opts, &mut cli_opts, &["format"], "--format")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn export(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("export".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--file")?;
        update_cli_opt(opts, &mut cli_opts, &["format"], "--format")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn import(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("import".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "pipeline_name", "pipeline-name"],
            "--pipeline-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--file")?;
        update_cli_opt(opts, &mut cli_opts, &["format"], "--format")?;
        update_cli_flag(opts, &mut cli_opts, &["overwrite"], "--overwrite")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn step(&self, opts: Option<&PyDict>) -> PyResult<XvcPipelineStep> {
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
}

#[pymethods]
impl XvcPipelineStep {
    fn new(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["command"], "--command")?;
        update_cli_opt(opts, &mut cli_opts, &["when"], "--when")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn update(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("update".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["command"], "--command")?;
        update_cli_opt(opts, &mut cli_opts, &["when"], "--when")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn dependency(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("dependency".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--file")?;
        update_cli_opt(opts, &mut cli_opts, &["url"], "--url")?;
        update_cli_opt(opts, &mut cli_opts, &["glob"], "--glob")?;
        update_cli_opt(opts, &mut cli_opts, &["glob-items"], "--glob-items")?;
        update_cli_opt(opts, &mut cli_opts, &["step"], "--step")?;
        update_cli_opt(opts, &mut cli_opts, &["param"], "--param")?;
        update_cli_opt(opts, &mut cli_opts, &["regex"], "--regex")?;
        update_cli_opt(opts, &mut cli_opts, &["regex-items"], "--regex-items")?;
        update_cli_opt(opts, &mut cli_opts, &["line", "lines"], "--line")?;
        update_cli_opt(opts, &mut cli_opts, &["line-items"], "--line-items")?;
        update_cli_opt(opts, &mut cli_opts, &["generic"], "--generic")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn output(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("output".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["file"], "--output-file")?;
        update_cli_opt(opts, &mut cli_opts, &["metric"], "--output-metric")?;
        update_cli_opt(opts, &mut cli_opts, &["image"], "--output-images")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn show(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("show".to_string());
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["name", "step_name", "step-name"],
            "--step-name",
        )?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }
}

