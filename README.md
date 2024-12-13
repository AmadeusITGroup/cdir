# cdir

cdir is a command-line tool that allows you to quickly navigate to directories in your terminal.

It records your directory history and provides a simple interface to jump back to previously visited directories using a
GUI.
It also supports creating shortcuts for frequently used directories.
And it supports text search.

<p align="center">
  <img src="doc/demo.gif" alt="animated" />
</p>

## Features

* Records your directory history
* Quickly navigate to previously visited directories with a console UI
* Directory shortcuts
* Text search for directories and shortcuts
* Supports multiple shells (zsh, bash)
* Import predefined shortcuts from a YAML file
* Customizable colors
* Supports multiple platforms (Linux, macOS)

## Commands

There are two main commands:

Open the cdir ui:

```
$ c
```

Create a named shortcut to the current directory

```
$ p myshortcut
```

You can also use the `c` command to go directly to a directory by using a shortcut name:

```
$ c myshortcut
```

## Navigating the UI

You can navigate through your directory history using the following keys:

* The `up` and `down` arrow keys (`shift` for bigger jumps);

* The `page up` and `page down` to scroll through the list;

* Use `Home` to go to the most recent directory in the history;

* Use `Enter` to actually 'cd' into the directory;

* Use `Esc` to exit.

Press `Tab` to switch between the directory history and shortcuts views.
The navigation is the name into the shortcut view.

You can type any text to filter the directory history or shortcuts.

Tip: You can use `ctrl+a` to see the full directory path in the directory history view.

## Installation

Download the latest release from the [releases page](https://github.com/to_define/cdir/releases).

Next, extract the archive, run the `install.sh` script located in the extracted directory, and follow the on-screen
instructions.

## Customization

Several aspects of `cdir` can be customized to fit your needs.
You can report to the configuration file for the available options.
The path to the configuration file can be found using:

```aiignore
$ cdir config-file
```

Colors are customizable, which is mandatory if you use a dark terminal theme.

To do so, you need to edit the configuration file and, for instance, add

```yaml
colors:
  date: "#80c0ff"
  path: "#ffffff"
  shortcut_name: "#70eeb0"
```

## Going further

Using directly the `cdir` cli provides more features, such as importing shortcuts for a file:

To do so, you need to have a YAML file with a list of shortcuts defined by a `name` and a `path`.

Example:

```yaml
- name: t
  path: /tmp
- ...
```

Then you can import it using the `cdir import` command:

```
$ cdir import-shortcuts /path/to/shortcuts.yaml
```

You can also delete a shortcut using the `cdir delete-shortcut` command:

```
$ cdir delete-shortcut myshortcut
```

## License

This project is licensed under the Apache License 2.0.
See the [LICENSE](LICENSE) file for details.