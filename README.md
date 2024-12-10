# Cleanfile

A standard format, like .gitignore, for projects to specify how to clean up their artifacts.

## Install

```bash
cargo install --git=https://github.com/chris13524/cleanfile.git
```

## `cleanfile` file format

cleanfiles are in YAML format and specify configuration on how to clean up things:

```yaml
frameworks:
  - cargo
```

## Cleaning a project

```bash
cleanfile --file ./cleanfile
```

## Full system cleaning

Developers can place a cleanfile at the root of their projects directory and by specifying `recurse_depth` > 0 cleanfile will recurse into subfolders and find `cleanfile`s

```yaml
#!/usr/bin/env cleanfile --file
recurse_depth: 3
docker_prune_all: true
```

Run with:

```bash
chmod +x ./cleanfile
./cleanfile
```

## TODO

- [ ] Package manager cache cleaning
- [ ] Security: `docker_prune_all: true` should only be possible when ran directly
