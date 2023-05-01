# **Disclaimer: Any command marked with not-yet-implemented (NYIMPL) will not work properly or at all**

# Usage
As a first point of reference, you may always call `--help` (including sub-commands such as `wt_ext_cli --unpack_vromf --help`), as it is always up-to-date.  
Example help-output: `wt_ext_cli --help`
```
WarThunder datamining extraction tools

Usage: wt_ext_cli [OPTIONS] <COMMAND>

Commands:
  unpack_raw_blk, --unpack_raw_blk
          Unpacks a folder of raw/binary blk files into their unpacked format
  unpack_vromf, --unpack_vromf
          Unpacks vromf into raw or human readable formats, such as Json or Blk
  unpack_dxp_and_grp, --unpack_dxp
          Unpacks folder and subfolder DXP and GRP files to text-formatted file
  diff_yup NYIMPL, --diff_yup
          Creates diff from .yup
  update_check NYIMPL, --check_update
          Checks folder for client update
  get_instruction_manual, --instruction_manual
          Opens or writes the manual
  help
          Print this message or the help of the given subcommand(s)

Options:
      --log_path <log_path>    When provided, writes the traced logs to a file
      --log_level <log_level>  Set log level, may be one of [Trace, Debug, Info, Warn, Error], default: Warn
  -h, --help                   Print help
  -V, --version                Print version

```

# Commands and their purpose
**`wt_ext_cli COMMAND_NAME --help` always prints information about the command and its arguments**

## unpack_vromf
Expects:
- single vromf file or folder with vromf files
Optional:
- Output directory, defaults to the input directory

For usage, the help output describes this best:
```
Unpacks vromf into raw or human readable formats, such as Json or Blk

Usage: wt_ext_cli {unpack_vromf|--unpack_vromf} [OPTIONS] --input_dir_or_file <Input file/directory> [format]

Arguments:
  [format]  Output format, can be one of: [Json, BlkText, BlkRaw]

Options:
  -i, --input_dir_or_file <Input file/directory>
          A single vromf file, or a folder of Vromf files. Does not recurse subdirs
  -o, --output_dir <Output directory>
          Target folder that will be created to contain new files
  -h, --help
          Print help
```

## unpack_dxp_and_grp
Expects: Input directory, containing `dxp.bin` and `grp.bin`
For usage, the help output describes this best:
```
Unpacks folder and subfolder DXP and GRP files to text-formatted file

Usage: wt_ext_cli {unpack_dxp_and_grp|--unpack_dxp} [OPTIONS] --input_dir <Input directory>

Options:
  -i, --input_dir <Input directory>    Folder with DXP/GRP files inside
  -o, --output_dir <Output directory>  Target folder that will be created to contain new files, preserving file structure
      --keep_suffix                    Paths and Names inside the final DXP/GRP are always followed by "water_garbage_pile_b_tex_d$hq*" or random unicode chars "u+4575"
  -h, --help                           Print help
```

## unpack_raw_blk
Expects a folder containing:
- One file called `nm` in the folder-root, which should be a compressed NameMap
- One file ending in `.dict` in the root, which is the ZSTD dictionary required for some, if not most binary BLK files
The remaining files may be either one of these two:
- Plaintext / unrelated files, which will be directly copied to the output directory
- Binary-block-files, which will be converted to a human-readable format, as specified
- Folder, containing any of the two above

The tool will recursively run through any subdirectories, so only the root of the target folder must be specified

For usage, the help output describes this best:
```
Unpacks a folder of raw/binary blk files into their unpacked format

Usage: wt_ext_cli {unpack_raw_blk|--unpack_raw_blk} [OPTIONS] --input_dir <Input directory>

Options:
  -i, --input_dir <Input directory>    Folder containing blk files, sub-folders will be recursively searched
  -o, --output_dir <Output directory>  Target folder that will be created to contain new files
      --overwrite                      Overwrites binary BLk files in input folder
  -h, --help                           Print help
```
