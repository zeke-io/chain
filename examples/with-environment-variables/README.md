# Environment variables

This example shows how to use environment variables.

This example uses the [Velocity](https://github.com/PaperMC/Velocity) proxy.

In the `config` folder there are two files used by Velocity, the `forwarding.secret` and `velocity.toml`.

Inside the `velocity.toml` configuration file, in the `bind` property you can see the value using two environment variables,
`CHAIN_HOST` and `CHAIN_PORT`, separated by a colon `:`.

And inside the `forwarding.secret` file you can find the `CHAIN_FORWARDING_SECRET` environment variable.

When running or packaging the server, these "placeholders" will be replaced by their provided values.

You can provide these values from the terminal:

```bash
CHAIN_HOST=127.0.0.1 CHAIN_PORT=25565 CHAIN_FORWARDING_SECRET=MySecret chain run
# Or
CHAIN_HOST=127.0.0.1 CHAIN_PORT=25565 CHAIN_FORWARDING_SECRET=MySecret chain pack
```

Or from your settings files:

```yaml
env:
  CHAIN_HOST: 127.0.0.1
  CHAIN_PORT: 25565
  CHAIN_FORWARDING_SECRET: SECRET-AsfQRit
```

## How to set it up

1. Rename the file `settings.dev.yml.template` to `settings.dev.yml`
2. Install the proxy with:

```bash
chain install
```

## Run the server (dev)

You can run the server using the development settings `settings.dev.yml` with:

```bash
chain run
```

If you don't want to load the development settings, you can add the `--prod` argument.
