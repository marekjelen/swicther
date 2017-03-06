# Switcher

Manages profiles of different OpenShift clusters.

## Command

The basic syntax is always

```
switcher <action> <profile>
```

Profiles are stored as `~/.switcher-<profile>` and symlinked into `~/.kube`.

## Actions

### init &lt;profile&gt;

Takes existing `~/.kube` directory, moves it to `~/.switcher-<profile>` and
switches to `<profile>`.

### create &lt;profile&gt;

Creates new `<profile>` by creating new directory at `~/.switcher-<profile>`.

### switch &lt;profile&gt;

Switches to `<profile>` by removing existing symlink at `~/.kube` and creating new
symlink to `~/.switcher-<profile>`.

### destroy &lt;profile&gt;

Destroys data in `<profile>` by deleting the `~/.switcher-<profile>` directory.

## License

Licensed under the terms of the MIT license.
