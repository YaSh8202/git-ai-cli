# GitAI

A command-line tool that uses AI to streamline your git workflow - from generating commit messages to explaining complex changes, all without requiring an API key.


## Features 🔅

- **Smart Commit Messages**: Generate conventional commit messages for your staged changes
- **Git History Insights**: Understand what changed in any commit, branch, or your current work
- **Zero Config**: Works instantly without an API key, using Phind by default
- **Flexible**: Works with any git workflow and supports multiple AI providers

## Getting Started 🔅

### Prerequisites
Before you begin, ensure you have `git` installed on your system

### Installation

#### Using Cargo
> [!IMPORTANT]
> `cargo` is a package manager for `rust`,
> and is installed automatically when you install `rust`.
> See [installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```bash
cargo install gitai
```

## Usage 🔅

### Generate Commit Messages

Create meaningful commit messages for your staged changes:

```bash
# Basic usage - generates a commit message based on staged changes
gitai generate
# Output: "feat(button.tsx): Update button color to blue"
```


### Explain Changes

Understand what changed and why:

```bash
# Explain current changes in your working directory
gitai explain --diff                  # All changes
gitai explain --diff --staged         # Only staged changes

# Explain specific commits
gitai explain HEAD                    # Latest commit
gitai explain abc123f                 # Specific commit
```




## AI Providers 🔅

Configure your preferred AI provider:

```bash
# Using CLI arguments
gitai -p openai -k "your-api-key" -m "gpt-4o" generate

# Using environment variables
export GITAI_PROVIDER="openai"
export GITAI_API_KEY="your-api-key"
export GITAI_MODEL="gpt-4o"
```

