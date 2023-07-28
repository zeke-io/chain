# Quick Start

## Creating a server

To begin, create a directory for your server:

```bash
mkdir my-server
cd my-server
```
Once inside the directory run the `init` command:

```bash
chain init
```

Running the `init` command will prompt you with some questions.

After this, run the `install` command to download the server jar and the dependencies (plugins).

```bash
chain install
```

Once it is done, it is recommended to run the server once to generate its files:

```bash
chainr --no-setup
```

*The `--no-setup` argument skips setup, only recommended when running the server for the first time to generate files.*

*Chain cannot detect if the server is done setting up, so you have to stop the server manually.*

## Adding server plugins

If you are using a server jar that supports plugins (Spigot, Paper, etc.), you can add plugins in the `chain.yml` file.

As an example, we will install the [spark](https://github.com/lucko/spark) plugin.

```yml
dependencies:
  # key: value
  # Spark plugin [https://github.com/lucko/spark]
  spark: https://ci.lucko.me/job/spark/384/artifact/spark-bukkit/build/libs/spark-1.10.43-bukkit.jar
```

The `dependencies` property is a key/value map where you define your plugins and where they come from.

The `key` can be anything, but it is recommended to put the name of the plugin in lowercase and using dashes instead of spaces,
and the `value` is a file path or download url for the plugin.

When running the `install` command, Chain will download all the plugins defined in the `chain.yml` file, and save them so they can be used when running the server or when generating the server zip.

## Settings

If you want to change the jvm options, the server arguments, or the java path to run the server,
you can create a `settings.yml` file.

```yml
java-runtime: java

jvm-options:
  - "-Dfile.encoding=UTF-8"
  - "-Xmx4G"

server-args:
  - "--nogui"

files:
  'server.properties': "config/server.properties"
```

Here is an explanation of all the properties:

- #### `java-runtime`
    The `java` command or a path to the java binary.
- #### `jvm-options`
    An array of options that will be passed to the Java Virtual Machine.
- #### `server-args`
    An array of arguments that will be passed to the server jar.
- #### `files`
    A key/value map that defines what server files to create/override.
    Both key and value are paths,the key *(target)* is a path within the server directory of the file you want to replace,
    and the value *(source)* is a path relative to the root directory of the file you want to replace it with.

    **If the target file does not exist, it will be copied, if it exists, it will be replaced.**

It is recommended that you add the `settings.yml` file to your VCS, and create a duplicate of the settings file as `settings.dev.yml` (this file can be ignored by the VCS),
this allows developers/admins to have different settings without having to modify the "production" settings.

This is also useful if you have different config files for both production and development, as you can change the files in the `settings.dev.yml` file.

When running the server, add the `--dev` argument to `chainr` so it tries to load the `settings.dev.yml` file (if it exists):

```bash
chainr --dev
```

## Pack the server

You can package the files, plugins and the server jar into a zip file by running the command:

```bash
chain pack
```

By running the command, Chain will create an `out` directory, where it is gonna place the server folder with all the files needed, and it will also generate a zip file with the contents.

*Chain will also generate both `batch` (Windows) and `bash` (Linux/MacOS) `start` scripts, and it will use your settings file to add the jvm options and server arguments.*
