# aws-sso-util

It's a simple utility to help you fast switch between AWS SSO profiles.

## Prerequisites

- Works only on Linux and MacOS
- You need to have the `~/.aws/config` file with the AWS SSO profiles.
- You need to have the `~/.aws/sso/cache` after login with `aws sso login` command.

## Features

- Can fuzzy search for AWS SSO profiles, it is looking into the `~/.aws/config` file.
- Can generate credentials given the profile name including `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_SESSION_TOKEN`;
- Can generate the AWS Console URL for the profile.

## Installation

```bash
cargo install aws-sso-util
```

## Usage

```bash
aws-sso-util --help

AWS SSO utility

USAGE:
    aws-sso-util [FLAGS]

FLAGS:
    -c, --console-ui    Get aws console url for the selected profile
    -e, --env           Get the export env for aws credentials for the selected profile
    -h, --help          Prints help information
    -p, --profile       Fuzzy search for aws profiles
    -V, --version       Prints version information
```

## Examples

Given you have the following `~/.aws/config` file:

```
[profile first_profile]
sso_session = some_session_name
region = us-west-1
sso_account_id = 123456789012
sso_role_name = AdministratorAccess
output = json

[profile second_profile]
...
[profile fifth_profile]
```

```bash
aws-sso-util -p

Select profile: › s
  third_profile
  first_profile
  fifth_profile
  fourth_profile
❯ second_profile
```
