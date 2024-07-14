import os
import pytest


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


def test_pipeline_step_dependency_file(xvc_pipeline_single_step):
    pipeline = xvc_pipeline_single_step.pipeline()
    dependency_file = "dir-0001/file-0001.bin"
    pipeline.step().dependency(step_name="hello", file=dependency_file)
    first_run = pipeline.run()
    second_run = pipeline.run()
    os.system(f"xvc-test-helper generate-random-file {dependency_file}")
    third_run = pipeline.run()

    assert first_run == third_run
    assert second_run.strip() == ""


def test_pipeline_step_dependency_url(xvc_pipeline_single_step):
    pipeline = xvc_pipeline_single_step.pipeline()
    expected = """
[OUT] [hello] hello xvc
[DONE] hello (echo 'hello xvc')
    """.strip()

    pipeline.step().dependency(step_name="hello", url="https://xvc.dev")
    first_run = pipeline.run()
    second_run = pipeline.run()
    print(first_run.strip())
    assert first_run.strip() == expected
    assert second_run.strip() == ""


def test_pipeline_step_dependency_glob(xvc_pipeline_single_step):
    pipeline = xvc_pipeline_single_step.pipeline()
    dependency_file = "dir-0001/file-0001.bin"
    pipeline.step().dependency(step_name="hello", glob="dir-0001/*.bin")

    first_run = pipeline.run()
    second_run = pipeline.run()
    os.system(f"xvc-test-helper generate-random-file {dependency_file}")
    third_run = pipeline.run()

    assert first_run == third_run
    print(second_run)
    assert second_run.strip().endswith("[DONE] hello (echo 'hello xvc')")


## FIXME: Debug and fix glob_items dependency
@pytest.mark.skip(
    reason="Needs to be debugged. The third run doesn't detect the change though the CLI run does. "
)
def test_pipeline_step_dependency_glob_items(xvc_repo_with_dir):
    pipeline = xvc_repo_with_dir.pipeline()
    dependency_file = "dir-0001/new-file.bin"
    xvc_repo_with_dir.pipeline().step().new(
        step_name="files", command='echo "ADDED_FILES: ${XVC_ADDED_GLOB_ITEMS}"'
    )
    pipeline.step().dependency(step_name="files", glob_items="dir-0001/*.bin")

    first_run = pipeline.run()
    second_run = pipeline.run()
    os.system(f"xvc-test-helper generate-random-file {dependency_file}")
    third_run = pipeline.run()

    print("FIRST RUN")
    print(first_run)
    print("SECOND RUN")
    print(second_run)
    print("THIRD RUN")
    print(third_run)

    assert first_run == third_run
    assert second_run.strip() == ""


def test_pipeline_step_dependency_step(xvc_pipeline_single_step):
    xvc_pipeline_single_step.pipeline().step().new(
        step_name="world", command="echo 'and the world'"
    )
    pipeline = xvc_pipeline_single_step.pipeline()
    pipeline.step().dependency(step_name="world", step="hello")
    first_run = pipeline.run()

    print(first_run)

    assert (
        first_run.strip()
        == """
        [OUT] [world] and the world
        """.strip()
    )


# TODO: def test_pipeline_step_dependency_param(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_regex(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_regex_items(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_line(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_line_items(xvc_repo_with_dir):
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
