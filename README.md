# ADO CLI Tool

This is a command-line tool for creating Azure DevOps (ADO) pull requests directly from the terminal. It streamlines the process by allowing you to specify branches, organization, and other options in a quick and efficient manner.

## Features

- Quickly create pull requests in Azure DevOps.
- Specify source and target branches.
- Customize the organization name.
- Open the PR in a browser.

## Installation

To install the tool, clone the repository and build it using Cargo:

```bash
git clone https://github.com/kriticalflare/ado.git
cd ado
cargo build --release
```

Then, add the compiled binary to your PATH:

```bash
export PATH="$PATH:$(pwd)/target/release"
```

## Usage

Here’s how you can use the ADO CLI tool:

```bash
ado [OPTIONS] -s <SOURCE_BRANCH> <REPO>
```

### Arguments

- `<REPO>`: The repository name in which the pull request should be created.

### Options

- `-t <TARGET_BRANCH>`: The target branch for the pull request. Defaults to `main`.
- `-s <SOURCE_BRANCH>`: The source branch for the pull request (required).
- `-o <ORGANIZATION>`: The Azure DevOps organization. Defaults to `YourOrg`.
- `-b <BROWSER>`: Optionally specify a browser to open the pull request. Defaults to your system default browser.
- `-h, --help`: Display help information about the tool.
- `-V, --version`: Display the version of the tool.

### Example

```bash
ado -s feature-branch my-repo
```

This will open a pull request from `feature-branch` to `main` in the `my-repo` repository under the default organization `YourOrg` in your browser. 

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Why?

I am a 10x dev who ends up opening way too many PRs and I would rather not suffer the ADO web ui any longer than i have to.