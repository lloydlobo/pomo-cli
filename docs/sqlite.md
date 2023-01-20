# Sqlite

## Installation

[Source for Installation](https://developer.fedoraproject.org/tech/database/sqlite/about.html)

### SQLite installation

To install basic SQLite (if it is not already), simply type:

```bash
$ sudo dnf install sqlite
This package provides the basic library and the command-line client sqlite. In order to access SQLite databases from various programming languages (C, Tcl, Java), the language bindings need to be installed separately:
```

```bash
$ sudo dnf install sqlite-devel sqlite-tcl sqlite-jdbc
```

### Graphical clients

The sqlite client shipped with the basic database engine is command line (CLI) based. If you prefer an application with graphical user interface (GUI), install the sqlitebrowser package:

```bash
$ sudo dnf install sqlitebrowser
```

### Working with SQLite

SQLite stores it’s data in single database file. To open such file (which will be created if necessary), pass it’s name as CLI argument to sqlite3 executable:

```bash
$ sqlite3 hello-world.db
```

After executing this command, you will be greeted with a SQLite prompt and can now insert the SQL commands to execute.

If you prefer using GUI, the Sqlitebrowser application enables you to construct your SQL queries using visual tool.

If you are new to SQL databases and would like to learn more, you can visit a W3CSchools SQL tutorial, which should give you a nice head start.

### Getting help with SQLite

As is the custom in Fedora, SQLite documentation is available in -doc sub-package. Additionally, the JavaDoc
documentation for JDBC bindings is also available:

```bash
$ sudo dnf install sqlite-doc sqlite-jdbc-javadoc
```

After the packages are installed, the actual documentation is located within /usr/share/doc/PACKAGE directory and is formatted in HTML. For example, to view documentation for SQLite itself, you can open this URL in your browser:

```
file:///usr/share/doc/sqlite-doc/index.html
```

If the documentation does not contain what you are looking for, you can visit the project homepage, or ask in the mailing lists.

## Troubleshooting

### error: linking with `cc` failed: exit code: 1

#### [Source](https://stackoverflow.com/a/65698711)

First: If you have any issue with writing files/dirs by ld just remove that files and try to recompile. I don't know why, but on Mac this issue happens time to time.

Second: If you have other ld errors (not about file access): try to add the following sections to your ~/.cargo/config (if you don't have this file feel free to create):

```bash
[target.x86_64-apple-darwin]
rustflags = [
    "-C", "link-arg=-undefined",
    "-C", "link-arg=dynamic_lookup",
]
```

```bash
[target.aarch64-apple-darwin]
rustflags = [
    "-C", "link-arg=-undefined",
    "-C", "link-arg=dynamic_lookup",
]
```

Third: Sometimes your Mac lack of some dev tools/dependencies. Install the most important of them automatically with the command:

```bash
xcode-select --install
```

#### [Source](https://stackoverflow.com/a/68343731)

if you have `"note: /usr/bin/ld: cannot find -lsqlite3"`
then install libsqlite3-dev:

```bash
$ sudo apt install libsqlite3-dev
```

This works on Rust 1.53.0, Linux Mint 20.2(based on Ubuntu 20.04 LTS)
