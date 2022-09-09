# Stalwart JMAP CLI

The Stalwart JMAP Command Line Interface (CLI) is a command line utility that allows system 
administrators to perform management tasks on a Stalwart JMAP server. 

The default location of the Stalwart CLI is ``/usr/bin/stalwart-cli`` and when executed
without any parameters it prints a brief help page such as this one:

```bash
$ stalwart-cli

Stalwart JMAP Server CLI

USAGE:
    stalwart-cli [OPTIONS] --url <URL> <SUBCOMMAND>

OPTIONS:
    -c, --credentials <CREDENTIALS>    Authentication credentials
    -h, --help                         Print help information
    -u, --url <URL>                    JMAP server base URL
    -V, --version                      Print version information

SUBCOMMANDS:
    account    Manage user accounts
    domain     Manage domains
    group      Manage groups
    help       Print this message or the help of the given subcommand(s)
    import     Import accounts and domains
    list       Manage mailing lists
```

The CLI tool expects two required arguments: the base URL of your Stalwart JMAP server (which is 
specified with the ``-u`` option) server and the system administrator credentials (which 
may be specified with the ``-c`` option or at the prompt).

For example, to list all existing accounts:

```bash
$ stalwart-cli -u https://jmap.example.org -c PASSWORD account list
```

Please refer to the [Stalwart JMAP server documentation](https://stalw.art/jmap/) for more details.

## License

Licensed under the terms of the [GNU Affero General Public License](https://www.gnu.org/licenses/agpl-3.0.en.html) as published by
the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
See [LICENSE](LICENSE) for more details.

You can be released from the requirements of the AGPLv3 license by purchasing
a commercial license. Please contact licensing@stalw.art for more details.
  
## Copyright

Copyright (C) 2020-2022, Stalwart Labs Ltd.
