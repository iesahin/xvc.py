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


## FIXME: Debug and fix glob dependency
@pytest.mark.skip(
    reason="Needs to be debugged. The third run doesn't detect the change though the CLI run does. "
)
def test_pipeline_step_dependency_glob(xvc_pipeline_single_step):
    pipeline = xvc_pipeline_single_step.pipeline()
    dependency_file = "dir-0001/file-0001.bin"
    pipeline.step().dependency(step_name="hello", glob="dir-0001/*.bin")

    first_run = pipeline.run()
    second_run = pipeline.run()
    os.system(f"xvc-test-helper generate-random-file {dependency_file}")
    third_run = pipeline.run()

    print(first_run)
    print(second_run)
    print(third_run)

    assert first_run == third_run
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
[OUT] [hello] hello xvc
[DONE] hello (echo 'hello xvc')
[OUT] [world] and the world
[DONE] world (echo 'and the world')
        """.strip()
    )


def test_pipeline_step_dependency_regex(xvc_repo_with_people_csv):
    pipeline = xvc_repo_with_people_csv.pipeline()
    pipeline.step().new(step_name="a", command='echo "New names starting with A added"')
    pipeline.step().dependency(step_name="a", regex="people.csv:/^A.*$")

    first_run = pipeline.run()
    print(first_run)
    second_run = pipeline.run()
    print(second_run)
    with open("people.csv", "a") as f:
        f.write("Ali,M,13,74,170\n")
    third_run = pipeline.run()
    print(third_run)

    assert first_run.strip() == third_run.strip()
    assert second_run.strip() == ""


def test_pipeline_step_dependency_regex_items(xvc_repo_with_people_csv):
    pipeline = xvc_repo_with_people_csv.pipeline()
    pipeline.step().new(
        step_name="a", command='echo "Lines with A: ${XVC_ADDED_REGEX_ITEMS}"'
    )
    pipeline.step().dependency(step_name="a", regex_items="people.csv:/^A.*$")
    first_run = pipeline.run()
    print(first_run)
    second_run = pipeline.run()
    print(second_run)
    with open("people.csv", "a") as f:
        f.write("Ali,M,13,74,170\n")

    third_run = pipeline.run()
    print(third_run)

    assert second_run.strip() == ""

    assert (
        third_run.strip()
        == """
[OUT] [a] Lines with A: Ali,M,13,74,170
[DONE] a (echo "Lines with A: ${XVC_ADDED_REGEX_ITEMS}")
""".strip()
    )


def test_pipeline_step_dependency_line(xvc_repo_with_people_csv):
    pipeline = xvc_repo_with_people_csv.pipeline()
    pipeline.step().new(step_name="a", command='echo "New lines added to the file"')
    pipeline.step().dependency(step_name="a", lines="people.csv::10-")

    first_run = pipeline.run()
    print(first_run)
    second_run = pipeline.run()
    print(second_run)
    with open("people.csv", "a") as f:
        f.write("Ali,M,13,74,170\n")
    third_run = pipeline.run()
    print(third_run)

    assert first_run.strip() == third_run.strip()
    assert second_run.strip() == ""


def test_pipeline_step_dependency_line_items(xvc_repo_with_people_csv):
    pipeline = xvc_repo_with_people_csv.pipeline()
    pipeline.step().new(
        step_name="a", command='echo "Added lines: ${XVC_ADDED_LINE_ITEMS}"'
    )
    pipeline.step().dependency(step_name="a", line_items="people.csv::10-")
    first_run = pipeline.run()
    print(first_run)
    second_run = pipeline.run()
    print(second_run)
    with open("people.csv", "a") as f:
        f.write("Ali,M,13,74,170\n")

    third_run = pipeline.run()
    print(third_run)

    assert second_run.strip() == ""

    assert (
        third_run.strip()
        == """
[OUT] [a] Added lines: Ali,M,13,74,170
[DONE] a (echo "Added lines: ${XVC_ADDED_LINE_ITEMS}")
""".strip()
    )


# TODO: def test_pipeline_step_dependency_generic(xvc_repo_with_dir):
#   assert False
#
# TODO: def test_pipeline_step_dependency_param(xvc_repo_with_dir):
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
