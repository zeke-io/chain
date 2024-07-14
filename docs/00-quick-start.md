# Quick Start

## Creating a server

To begin, run the `new` command:

```bash
# crafty new <path>
crafty new my_server
```

The `new` command will prompt you with some questions and create a directory for your project.

If you provided a source url for the server jar file, you can download it with the `install` command:

```bash
crafty install
```

Once it is done, it is recommended to run the server once to generate its files:

```bash
crafty run --no-setup
```

*The `--no-setup` argument skips setup, only recommended when running the server for the first time to generate files.*

*Crafty cannot detect if the server is done setting up, so you have to stop the server manually.*

## Adding server plugins

If you are using a server brand that supports plugins (Spigot, Paper, etc.),
you can add plugins in the `crafty.yml` file.

As an example, we will install the [spark](https://github.com/lucko/spark) plugin.

```yml
dependencies:
  # Spark plugin [https://github.com/lucko/spark]
  spark:
    source: https://ci.lucko.me/job/spark/384/artifact/spark-bukkit/build/libs/spark-1.10.43-bukkit.jar
```

Each dependency needs a `source` and by default is treated as a plugin
and will be installed in the `plugins` folder inside the server directory.

The source can be a URL or a local path to a file.

When adding or removing dependencies, you need to install them again with the `install` command.

## Settings

If you want to change the jvm options, the server arguments, or the java path to run the server,
you can create a `settings.yml` file.

```yml
jvm-options:
  - "-Dfile.encoding=UTF-8"
  - "-Xmx4G"

server-args:
  - "--nogui"

files:
  'server.properties': "src/server.properties"
```

Here is an explanation of all the properties:

- #### `jvm-options`
  An array of options that will be passed to the Java Virtual Machine.
- #### `server-args`
  An array of arguments that will be passed to the server jar.
- #### `files`
  A key/value map that defines what server files to create/override.
  Both key and value are paths,the key *(target)* is a path within the server directory of the file/folder you want to
  replace,
  and the value *(source)* is a path relative to the root directory of the file/folder you want to replace it with.

  **If the target file does not exist, it will be copied, if it exists, it will be replaced.**

It is recommended that you add the `settings.yml` file to your VCS, and create a duplicate of the settings file
as `settings.dev.yml` (this file can be ignored by the VCS),
this allows developers/admins to have different settings without having to modify the "production" settings.

This is also useful if you have different config files for both production and development, as you can change the files
in the `settings.dev.yml` file.

To run the server with your development settings, use:

```bash
crafty run
```

If the settings file `settings.dev.yml` does not exists, it will load the default `settings.yml` file.

## Pack the server

You can package the files, plugins, and the server jar into a zip file by running the command:

```bash
crafty pack
```

By running the command, Crafty will create an `out` directory,
where it is going to place the server folder with all the files needed,
and it will also generate a zip file with the contents.

*Crafty will also generate both `batch` (Windows) and `bash` (Linux/macOS) `start` scripts, and it will use your
settings
file to add the jvm options and server arguments.*
