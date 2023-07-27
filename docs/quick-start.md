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
