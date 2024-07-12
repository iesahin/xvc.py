def test_pipeline_list(empty_xvc_repo):
    pipeline_table = empty_xvc_repo.pipeline().list()
    expected = """
+---------+---------+
| Name    | Run Dir |
+===================+
| default |         |
+---------+---------+
""".strip()
    assert pipeline_table == expected


def test_pipeline_new(empty_xvc_repo):
    empty_xvc_repo.pipeline().new(pipeline_name="test", workdir="dir-0001")
    pipeline_table = empty_xvc_repo.pipeline().list()
    print(pipeline_table)
    expected = """
+---------+----------+
| Name    | Run Dir  |
+====================+
| default |          |
|---------+----------|
| test    | dir-0001 |
+---------+----------+
""".strip()
    assert pipeline_table == expected


def test_pipeline_update(empty_xvc_repo):
    empty_xvc_repo.pipeline().new(pipeline_name="test")
    empty_xvc_repo.pipeline().update(pipeline_name="test", workdir="dir-0001")
    pipeline_table = empty_xvc_repo.pipeline().list()
    print(pipeline_table)
    expected = """
+---------+----------+
| Name    | Run Dir  |
+====================+
| default |          |
|---------+----------|
| test    | dir-0001 |
+---------+----------+
""".strip()
    assert pipeline_table == expected


def test_pipeline_delete(empty_xvc_repo):
    empty_xvc_repo.pipeline().new(pipeline_name="test")
    empty_xvc_repo.pipeline().delete(pipeline_name="test")
    pipeline_table = empty_xvc_repo.pipeline().list()
    print(pipeline_table)
    expected = """
+---------+---------+
| Name    | Run Dir |
+===================+
| default |         |
+---------+---------+
""".strip()
    assert pipeline_table == expected


def test_pipeline_step_list(empty_xvc_repo):
    empty_xvc_repo.pipeline().step().new(step_name="hello", command="echo 'hello xvc'")
    pipeline_steps = empty_xvc_repo.pipeline().step().list(names_only=True)
    assert pipeline_steps.strip() == "hello"


def test_pipeline_step_new(empty_xvc_repo):
    empty_xvc_repo.pipeline().step().new(step_name="hello", command="echo 'hello xvc'")
    pipeline_steps = empty_xvc_repo.pipeline().step().list()
    assert pipeline_steps.strip() == "hello: echo 'hello xvc' (by_dependencies)"


def test_pipeline_step_update(empty_xvc_repo):
    empty_xvc_repo.pipeline().step().new(step_name="hello", command="echo 'hello xvc'")
    empty_xvc_repo.pipeline().step().update(
        step_name="hello", command="echo 'hello world'", when="always"
    )
    pipeline_steps = empty_xvc_repo.pipeline().step().list()
    assert pipeline_steps.strip() == "hello: echo 'hello world' (always)"


def test_pipeline_step_dependency(empty_xvc_repo):
    dep_help = empty_xvc_repo.pipeline().step().dependency(help=True)
    assert False
    expected = """
Usage: xvc pipeline step dependency [OPTIONS] --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>
          Name of the step to add the dependency to

      --generic <GENERICS>
          Add a generic command output as a dependency. Can be used multiple times. Please delimit the command with ' ' to avoid shell expansion

      --url <URLS>
          Add a URL dependency to the step. Can be used multiple times

      --file <FILES>
          Add a file dependency to the step. Can be used multiple times

      --step <STEPS>
          Add a step dependency to a step. Can be used multiple times. Steps are referred with their names

      --glob_items <GLOB_ITEMS>
          Add a glob items dependency to the step.
          
          You can depend on multiple files and directories with this dependency.
          
          The difference between this and the glob option is that this option keeps track of all matching files, but glob only keeps track of the matched files' digest. When you want to use ${XVC_GLOB_ITEMS}, ${XVC_ADDED_GLOB_ITEMS}, or ${XVC_REMOVED_GLOB_ITEMS} environment variables in the step command, use the glob-items dependency. Otherwise, you can use the glob option to save disk space.

      --glob <GLOBS>
          Add a glob dependency to the step. Can be used multiple times.
          
          You can depend on multiple files and directories with this dependency.
          
          The difference between this and the glob-items option is that the glob-items option keeps track of all matching files individually, but this option only keeps track of the matched files' digest. This dependency uses considerably less disk space.

      --param <PARAMS>
          Add a parameter dependency to the step in the form filename.yaml::model.units . Can be used multiple times

      --regex_items <REGEX_ITEMS>
          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times.
          
          The difference between this and the regex option is that the regex-items option keeps track of all matching lines, but regex only keeps track of the matched lines' digest. When you want to use ${XVC_REGEX_ITEMS}, ${XVC_ADDED_REGEX_ITEMS}, ${XVC_REMOVED_REGEX_ITEMS} environment variables in the step command, use the regex option. Otherwise, you can use the regex-digest option to save disk space.

      --regex <REGEXES>
          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times.
          
          The difference between this and the regex option is that the regex option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest.

      --line_items <LINE_ITEMS>
          Add a line dependency in the form filename.txt::123-234
          
          The difference between this and the lines option is that the line-items option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest. When you want to use ${XVC_ALL_LINE_ITEMS}, ${XVC_ADDED_LINE_ITEMS}, ${XVC_CHANGED_LINE_ITEMS} options in the step command, use the line option. Otherwise, you can use the lines option to save disk space.

      --lines <LINES>
          Add a line digest dependency in the form filename.txt::123-234
          
          The difference between this and the line-items dependency is that the line option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest. If you don't need individual lines to be kept, use this option to save space.

  -h, --help
          Print help (see a summary with '-h')
""".strip()

    assert dep_help == expected


# TODO: def test_pipeline_step_dependency_file(xvc_repo_with_dir):
# assert False
# TODO: def test_pipeline_step_dependency_url(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_glob(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_glob-items(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_step(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_param(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_regex(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_regex-items(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_line(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_line-items(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_generic(xvc_repo_with_dir):
#   assert False
#
# TODO: def test_pipeline_dag(xvc_repo_with_dir):
#     assert False
#
#
# TODO: def test_pipeline_export(xvc_repo_with_dir):
#     assert False
#
#
# TODO: def test_pipeline_import(xvc_repo_with_dir):
#     assert False
#
