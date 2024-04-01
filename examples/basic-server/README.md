# Basic Server

This example shows the basics of Chain.

It uses [Paper](https://github.com/PaperMC/Paper) 1.20.1, and it has the [spark](https://github.com/lucko/spark) plugin
defined in the `chain.yml` file.

Inside the `src` folder there are two versions of the server's `server.properties` file with different values.

When running the server, one of these will be copied and used by the server, depending on which settings file is used.

## How to set it up

1. Rename the file `settings.dev.yml.template` to `settings.dev.yml`
2. Install the server and its plugins with:

```bash
chain install
```

## Run the server

You can run the server using the development settings `settings.dev.yml` with:

```bash
chain run
```

If you don't want to load the development settings, you can set the profile name with the `--profile` argument.

```bash
chain --profile=prod run
```
