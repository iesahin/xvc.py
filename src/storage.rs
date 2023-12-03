use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::Xvc;
use crate::{run, update_cli_flag, update_cli_opt};

#[pyclass]
pub struct XvcStorage {
    xvc_opts: Xvc,
}

impl XvcStorage {
    pub fn init(xvc_opts: &Xvc) -> PyResult<Self> {
        Ok(Self {
            xvc_opts: xvc_opts.clone(),
        })
    }

    pub fn cli(&self) -> PyResult<Vec<String>> {
        let mut cli_opts = self.xvc_opts.cli()?;
        cli_opts.push("file".to_string());
        Ok(cli_opts)
    }
}

#[pymethods]
impl XvcStorage {
    fn list(&self) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("list".to_string());
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }
    fn remove(&self, name: &str) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("remove".to_string());
        cli_opts.push(name.to_string());
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn new_local(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        cli_opts.push("local".to_string());

        update_cli_opt(opts, &mut cli_opts, &["name"], "--name")?;
        update_cli_opt(opts, &mut cli_opts, &["path"], "--path")?;

        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn new_generic(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        cli_opts.push("generic".to_string());

        update_cli_opt(opts, &mut cli_opts, &["name"], "--name")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["init", "init_command", "init-command"],
            "--init",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["list", "list_command", "list-command"],
            "--list",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["download", "download_command", "download-command"],
            "--download",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["upload", "upload_command", "upload-command"],
            "--upload",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["delete", "delete_command", "delete-command"],
            "--delete",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["processes", "max_processes", "max-processes"],
            "--processes",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["url"], "--url")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["storage_dir", "storage-dir"],
            "--storage-dir",
        )?;

        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn new_rsync(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        cli_opts.push("rsync".to_string());

        update_cli_opt(opts, &mut cli_opts, &["name"], "--name")?;
        update_cli_opt(opts, &mut cli_opts, &["host"], "--host")?;
        update_cli_opt(opts, &mut cli_opts, &["port"], "--port")?;
        update_cli_opt(opts, &mut cli_opts, &["user"], "--user")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["storage_dir", "storage-dir"],
            "--storage-dir",
        )?;

        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn new_s3(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        cli_opts.push("s3".to_string());

        update_cli_opt(opts, &mut cli_opts, &["name"], "--name")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["storage_prefix", "storage-prefix"],
            "--storage-prefix",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["bucket_name", "bucket-name"],
            "--bucket-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["region"], "--region")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn new_minio(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        cli_opts.push("minio".to_string());
        update_cli_opt(opts, &mut cli_opts, &["name"], "--name")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["storage_prefix", "storage-prefix"],
            "--storage-prefix",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["bucket_name", "bucket-name"],
            "--bucket-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["endpoint"], "--endpoint")?;
        update_cli_opt(opts, &mut cli_opts, &["region"], "--region")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn new_digital_ocean(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        cli_opts.push("digital-ocean".to_string());
        update_cli_opt(opts, &mut cli_opts, &["name"], "--name")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["storage_prefix", "storage-prefix"],
            "--storage-prefix",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["bucket_name", "bucket-name"],
            "--bucket-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["region"], "--region")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn new_r2(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        cli_opts.push("r2".to_string());
        update_cli_opt(opts, &mut cli_opts, &["name"], "--name")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["storage_prefix", "storage-prefix"],
            "--storage-prefix",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["account_id", "account-id"],
            "--account-id",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["bucket_name", "bucket-name"],
            "--bucket-name",
        )?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn new_gcs(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        cli_opts.push("gcs".to_string());
        update_cli_opt(opts, &mut cli_opts, &["name"], "--name")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["storage_prefix", "storage-prefix"],
            "--storage-prefix",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["bucket_name", "bucket-name"],
            "--bucket-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["region"], "--region")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }

    fn new_wasabi(&self, opts: Option<&PyDict>) -> PyResult<String> {
        let mut cli_opts = self.cli()?;
        cli_opts.push("new".to_string());
        cli_opts.push("wasabi".to_string());
        update_cli_opt(opts, &mut cli_opts, &["name"], "--name")?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["storage_prefix", "storage-prefix"],
            "--storage-prefix",
        )?;
        update_cli_opt(
            opts,
            &mut cli_opts,
            &["bucket_name", "bucket-name"],
            "--bucket-name",
        )?;
        update_cli_opt(opts, &mut cli_opts, &["endpoint"], "--endpoint")?;
        run(cli_opts.iter().map(|s| s.as_str()).collect())
    }
}
