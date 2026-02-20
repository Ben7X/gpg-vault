## Why this tool exists?

If you handle a lot of secret data it might be a good idea to store them somewhere save. 
If you handle a lot of secret data on your local machine it might be a good idea to not make them accessible at once.

This is a tool for a cli, gpg, yubikey centric workflow to protect your data on a local machine.


## What it really is?

In a nutshell this could have been a simple bash script as it is a wrapper around a existing software gpg, but I wanted it to be a bin with autocompletion. And also I love to write rust. 


# Current state
![Rust project pipeline](https://github.com/Ben7X/gpg-vault/actions/workflows/rust.yml/badge.svg)

## How to use

- init your config with default values in $HOME/.config/gpg-vault/config.yaml
- very simple data structure, you need to change the gpg-key id you want to use

```shell
gpg-vault init -d
```

- show the status of you tracked files

```shell
gpg-vault status
```

- unseal all/group

```shell
gpg-vault unseal
gpg-vault unseal <group>
```

- seal all/group

```shell
gpg-vault seal
gpg-vault seal <group>
```

Use the GPG_VAULT_GROUPS to have a specific groups filter set by default otherwise all is being used. You can also parse the group you want to filter on everytime on the cli.

# Releases

## 0.1.1

- Fixed typos in README

## 0.1.0

- Initial commit

# You want to help

- If you want to help in the development feel free
- If you are a pro rust developer, and you have some tips appreciate it

# Disclaimer

If you use this tool, you do it on your own risk.
