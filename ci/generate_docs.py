"""
Automatic generator of documentation about CI jobs.
Analyzes all .yml files connected with CI, takes the most important information
and writes it into a README file.

Features:
- reads a job description from a comment above job definition
- includes a link to each file and also to job definition
- includes a script definition in a code block

Usage:
- put comments (starting with "#") directly above the job definition in .yml file

Running the script:
- `python generate_docs.py` to generate the documentation
- `python generate_docs.py --test` to check if documentation is up-to-date
"""

import argparse
import filecmp
import os
import re
import sys
from collections import OrderedDict
from pathlib import Path
from typing import Any, Dict, List

import yaml

# TODO: could read sections inside the file with some overall description of below jobs
# TODO: could add something like "Generated automatically, do not edit by hand" into the README
# TODO: when CI jobs are calling some make target, we could make a comment in
#   that Makefile and take the text from there (so that Makefiles are documented as well)

parser = argparse.ArgumentParser()
parser.add_argument(
    "--test",
    action="store_true",
    help="Check if there are no new changes in all CI .yml files",
)
args = parser.parse_args()


class DocsGenerator:
    def __init__(self) -> None:
        # Going to the root directory, so the relative
        # locations of CI files are valid
        os.chdir(Path(__file__).resolve().parent.parent)

        self.GITLAB_CI_FILE = ".gitlab-ci.yml"
        self.DOC_FILE = "docs/ci/jobs.md"

        # Some keywords that are not job definitions and we should not care about them
        self.NOT_JOBS = [
            "variables:",
            "image:",
            ".gitlab_caching:",
        ]

        self.ALL_JOBS: Dict[str, Dict[str, Any]] = OrderedDict()

        self.FILES = self.get_all_ci_files()

    def generate_docs(self) -> None:
        """Whole pipeline of getting and saving the CI information."""
        for file in self.FILES:
            self.ALL_JOBS[file] = {
                "jobs": self.get_jobs_from_file(file),
                "overall_description": self.get_overall_description_from_file(file),
            }

        self.save_docs_into_file()

    def verify_docs(self) -> None:
        """Checking if the docs are up-to-date with current CI .yml files.

        Creating a new doc file and comparing it against already existing one.
        Exit with non-zero exit code when these files do not match.
        """
        already_filled_doc_file = self.DOC_FILE
        self.DOC_FILE = "new_file_temp.md"

        try:
            self.generate_docs()
            if filecmp.cmp(already_filled_doc_file, self.DOC_FILE):
                print("SUCCESS: Documentation is up-to-date!")
                sys.exit(0)
            else:
                print("FAIL: Documentation is not up-to-date with CI .yml files!")
                sys.exit(1)
        finally:
            os.remove(self.DOC_FILE)

    def get_all_ci_files(self) -> List[str]:
        """Loading all the CI files which are used in Gitlab."""
        if not os.path.exists(self.GITLAB_CI_FILE):
            raise RuntimeError(
                f"Main Gitlab CI file under {self.GITLAB_CI_FILE} does not exist!"
            )

        with open(self.GITLAB_CI_FILE, "r") as f:
            gitlab_file_content = yaml.safe_load(f)

        all_ci_files = gitlab_file_content["include"]

        for file in all_ci_files:
            if not os.path.isfile(file):
                raise RuntimeError(f"File {file} does not exist!")

        return all_ci_files

    @staticmethod
    def get_overall_description_from_file(file: str) -> List[str]:
        """Looking for comments at the very beginning of the file."""
        description_lines: List[str] = []
        with open(file, "r") as f:
            for line in f:
                if line.startswith("#"):
                    # Taking just the text - no hashes, no whitespace
                    description_lines.append(line.strip("# \n"))
                else:
                    break

        return description_lines

    def get_jobs_from_file(self, file: str) -> Dict[str, Dict[str, Any]]:
        """Extract all jobs and their details from a certain file."""
        all_jobs: Dict[str, Dict[str, Any]] = OrderedDict()

        # Getting the parsed content of the file, so we can get the script array
        with open(file, "r") as f:
            gitlab_file_content = yaml.safe_load(f)

        # Taking all the comments above a non-indented non-comment, which is
        # always a job definition, unless defined in NOT_JOBS
        with open(file, "r") as f:
            comment_buffer: List[str] = []
            for index, line in enumerate(f):
                if line.startswith("#"):
                    # Taking just the text - no hashes, no whitespace
                    comment_buffer.append(line.strip("# \n"))
                else:
                    # regex: first character of a line is a word-character or a dot
                    if re.search(r"\A[\w\.]", line) and not any(
                        [line.startswith(not_job) for not_job in self.NOT_JOBS]
                    ):
                        job_name = line.rstrip(":\n")
                        if job_name in gitlab_file_content:
                            all_jobs[job_name] = {
                                "description": comment_buffer,
                                "line_no": index + 1,
                                "script": gitlab_file_content[job_name].get(
                                    "script",
                                    ["No script defined, probably extends another job"],
                                ),
                            }
                    comment_buffer = []

        return all_jobs

    def save_docs_into_file(self) -> None:
        """Dump all the information into a documentation file."""
        with open(self.DOC_FILE, "w") as doc_file:
            # Some general info for the whole CI
            doc_file.write("# CI pipeline\n")
            doc_file.write(
                "It consists of multiple stages below, each having one or more jobs\n"
            )
            latest_master = "https://gitlab.com/satoshilabs/trezor/trezor-firmware/-/pipelines/master/latest"
            doc_file.write(
                f"Latest CI pipeline of master branch can be seen at [{latest_master}]({latest_master})\n"
            )

            # TODO: test-hw are run inside test stage, maybe unite it under it
            for file, file_info in self.ALL_JOBS.items():
                # Generating header with a link to the file
                doc_file.write(
                    f"## {Path(file).stem.upper()} stage - [file](../{file})\n\n"
                )

                # TODO: are we alright using this python >= 3.8 feature (walrus operator)?
                if description := file_info["overall_description"]:
                    for line in description:
                        doc_file.write(f"{line}\n")
                    doc_file.write("\n")

                job_amount = f"{len(file_info['jobs'])} job{'s' if len(file_info['jobs']) > 1 else ''}"
                doc_file.write(f"Consists of **{job_amount}** below:\n")

                for job_name, job_info in file_info["jobs"].items():
                    # Generating smaller header with link to the exact line of
                    # this job in the master branch
                    # (will work properly only after merging changes to master)
                    github_job_link = f"https://github.com/trezor/trezor-firmware/blob/master/{file}#L{job_info['line_no']}"
                    doc_file.write(f"- ### [{job_name}]({github_job_link})\n")
                    if not job_info["description"]:
                        doc_file.write("Missing description\n")
                    for line in job_info["description"]:
                        doc_file.write(f"{line}\n")

                    # Code block for the whole script
                    # TODO: for longer scripts, we could make a collapsible section
                    # https://gist.github.com/pierrejoubert73/902cc94d79424356a8d20be2b382e1ab
                    doc_file.write("```sh\n")
                    for line in job_info["script"]:
                        doc_file.write(f"{line}\n")
                    doc_file.write("```")

                    doc_file.write("\n")

                doc_file.write("---")
                doc_file.write("\n")


if __name__ == "__main__":
    if args.test:
        DocsGenerator().verify_docs()
    else:
        DocsGenerator().generate_docs()
