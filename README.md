# `composer-json-modifier`

Modifies composer.json files using provided configuration.

```
Modifies composer.json files using provided configuration.

USAGE:
    composer-json-modifier [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -V, --Version               Print Version info and exit
        --list                  List installed commands
    -v, --verbose               Use verbose output
    -vv, --very-verbose         Use very verbose output
    -vvv, --very-very-verbose   Use very very verbose output
    -q, --quiet                 Do not print log messages
    -h, --help                  Print help information

Some common commands are (see all commands with --list):
    modify, m   Modify a composer.json file
    config, c   Interact with a configuration file

See 'composer-json-modifier help <command>' for more information on a specific command.
```

## Subcommands

### `modify`

Modify a composer.json file

```
Modifies composer.json files using provided configuration.

USAGE:
    composer-json-modifier modify [OPTIONS] <PATH> [<CONFIG_FILE>]
                                            
ARGS:
    <PATH>         Path to the composer.json file to modify
    <CONFIG_FILE>  Path to the configuration file to use
                                            
OPTIONS:
    -V, --Version               Print Version info and exit
    -v, --verbose               Use verbose output
    -vv, --very-verbose         Use very verbose output
    -vvv, --very-very-verbose   Use very very verbose output
    -q, --quiet                 Do not print log messages
    -c, --config <KEY=VALUE>    Override a configuration value
        --dry-run               Don't actually modify the file
    -h, --help                  Print help information

Run `composer-json-modifier help modify` for more detailed information.
```

### `config`

Interact with a configuration file.

```
Interact with a configuration file

USAGE:
    composer-json-modifier config [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -V, --Version               Print Version info and exit
        --list                  List installed commands
    -v, --verbose               Use verbose output
    -vv, --very-verbose         Use very verbose output
    -vvv, --very-very-verbose   Use very very verbose output
    -q, --quiet                 Do not print log messages
    -h, --help                  Print help information

Some common commands are (see all commands with --list):
    check, c    Check a configuration file for errors

See 'composer-json-modifier modify help <command>' for more information on a specific command.
```

#### `check`

Check a configuration file for errors.

```
Check a configuration file for errors

USAGE:
    composer-json-modifier config check [OPTIONS] <PATH>
                                            
ARGS:
    <PATH>         Path to the configuration file to check
                                            
OPTIONS:
    -V, --Version               Print Version info and exit
    -v, --verbose               Use verbose output
    -vv, --very-verbose         Use very verbose output
    -vvv, --very-very-verbose   Use very very verbose output
    -q, --quiet                 Do not print log messages
    -h, --help                  Print help information

Run `composer-json-modifier modify help check` for more detailed information.
```
