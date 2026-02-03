# mneb-tool
A tool to animate or convert MNEB files. Documentation on the format can be found [here](https://swiftshine.github.io/doc/key/mneb.html).

## Usage
### Animation
To animate an MNEB file, use the `animate` command.
```
mneb-tool animate my_file.mneb
```
You can also specify a framerate with the `-f` or `--framerate` flags. The default value is `60.0`.
```
mneb-tool animate my_file.mneb -f 30.0
```
```
mneb-tool animate my_file.mneb --framerate 30.0
```

You can also animate multiple files with a wildcard (`*`). You must also specify that the format is `.mneb`.

**Correct**
```
mneb-tool animate my_files*.mneb
```
**Incorrect**
```
mneb-tool animate my_files*
```

### JSON Conversion
To convert an MNEB file to JSON, use the `convert` command. The default output filename is `out.json`.
```
mneb-tool convert my_file.mneb output.json
```
You can also use the `-p` or `--pretty` flags to make your JSON output pretty.
```
mneb-tool convert my_file.mneb output.json -p
```
```
mneb-tool convert my_file.mneb output.json --pretty
```
