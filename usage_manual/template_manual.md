# Usage

As a first point of reference, you may always call `--help` (including sub-commands such as
`wt_ext_cli --unpack_vromf --help`), as it is always up-to-date.  
Example help-output: `wt_ext_cli --help`

```
{{TOP_LEVEL_HELP}}
```

# Commands and their purpose

**`wt_ext_cli COMMAND_NAME --help` always prints information about the command and its arguments**

## unpack_vromf

Expects:

- single vromf file or folder with vromf files

Optional:

- Output directory, defaults to the input directory
- `--export_meta` dumps vromf metadata as `meta.toml` inside the extracted folder for later repacking

For usage, the help output describes this best:

```
{{VROMF_HELP}}
```

## unpack_raw_blk

Takes a **single** BLK file (directories disabled for now) and unpacks it to plaintext json

```
{{BLK_HELP}}
```

## vromf_version

Prints versions found either inside the vromf (version file) and/or the header of the vromf

```
{{VROMF_VERSION}}
```

## repack_vromf

Packs a directory of files into a vromfs.bin archive, or round-trips an existing vromf.
When repacking from a directory, the tool looks for `meta.toml` inside the input directory
(produced by `unpack_vromf --export_meta`) to preserve original metadata settings.

```
{{REPACK_HELP}}
```

# Environment variables

`FORCE_SET_COLOR`:  
Behaviour: Disables any color when printing errors or backtraces  
Possible values: [true, false]  
Default value (windows): false  
Default value (not-windows): true

`CAPTURE_IMAGE_CONVERTER`:  
Behaviour: Captures and prints output from FFMPEG or Imagemagick
Possible values: [true, false]  
Default value : false

`CONVERTER_PATH`:  
Behaviour: Overrides used path of ffmpeg  
Possible values: Path to executable  
Default value: Looks into $PATH

`RAYON_NUM_THREADS`:  
Behaviour: Limits amount of threads used by rayon  
Possible values: 32-bit unsigned integer  
Default value: Amount of (virtualized) cpus available on system  
