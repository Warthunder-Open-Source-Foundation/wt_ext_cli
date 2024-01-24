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
  get_instruction_manual, --instruction_manual
          Opens or writes the manual
  hash, --hash
          Prints commit hash from binary (link)
  vromf_version, --vromf_version
          Prints version(s) from file or folder of vromfs
  help
          Print this message or the help of the given subcommand(s)

Options:
      --log_path <log_path>    When provided, writes the traced logs to a file
      --log_level <log_level>  Set log level, may be one of [Trace, Debug, Info, Warn, Error], default: Warn
      --crashlog               Runs at maximum log level and writes logfile to aid in debugging
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

Usage: wt_ext_cli {unpack_vromf|--unpack_vromf} [OPTIONS] --input_dir_or_file <Input file/directory>

Options:
  -i, --input_dir_or_file <Input file/directory>
          A single vromf file, or a folder of Vromf files. Does not recurse subdir
  -o, --output_dir <Output directory>
          Target folder that will be created to contain new files
      --format <format>
          Output format, can be one of: [Json, BlkText, Raw] [default: Json]
      --crlf
          Returns files with \r\n instead of \n newlines
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
Takes a **single** BLK file (directories disabled for now) and unpacks it to plaintext json
```
Unpacks a folder of raw/binary blk files into their unpacked format

Usage: wt_ext_cli {unpack_raw_blk|--unpack_raw_blk} [OPTIONS] --input_dir <Input directory>

Options:
  -i, --input_dir <Input directory>    Folder containing blk files, sub-folders will be recursively searched
  -o, --output_dir <Output directory>  Target folder that will be created to contain new files
  -h, --help                           Print help
```

## vromf_version
Prints versions found either inside the vromf (version file) and/or the header of the vromf
```
Prints version(s) from file or folder of vromfs

Usage: wt_ext_cli {vromf_version|--vromf_version} --input_dir_or_file <input>

Options:
  -i, --input_dir_or_file <input>  A single vromf file, or a folder of Vromf files. Does not recurse subdirs
  -h, --help                       Print help
```

# Environment variables

`FORCE_SET_COLOR`:  
Behaviour: Disables any color when printing errors or backtraces  
Possible values: [true, false]  
Default value (windows): false  
Default value (not-windows): true  

`NO_UPDT_CHK`:  
Behaviour: Does not check the github API if an updated version is available  
Possible values: (any, just needs to be present)  
Default value: unset  