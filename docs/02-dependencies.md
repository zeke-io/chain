# Dependencies

All dependencies should be added in the `crafty.yml` file under the `dependencies` property.

```yml
# crafty.yml
dependencies:
  my_plugin:
    source: ../plugins/my_plugin.jar
    required: false
    type: plugin
```

Here are the available properties for each dependency:

- #### `source`
  The source tells where Crafty should find the dependency, this can be either a path or a URL.
- #### `required` (optional)
  This tells whether the dependency is required or not, if `false`, Crafty will skip it if it cannot download/install
  it. (Default: `true`)
- #### `type` (optional)
  This determines what type of dependency it is, `mod` or `plugin`, this will determine where to install the dependency
  on the server directory.
