# pubmed-rs

Simple tool to extract `{text: ...}` from the articles in the COVID-19 corpus.

Optionally includes filenames or section names in the output.

Output is written to standard out.

## Usage

```
Usage: pubmed-rs [OPTIONS]

Options:
  -f, --filename <FILENAME>  Filename of the JSON file to parse
  -d, --dirname <DIRNAME>    Directory name
  -j, --json                 Output JSON instead of plain text
  -m, --maxfiles <MAXFILES>  If specified, maximum number of files to process from directory
  -s, --sectionnames         Include the section names in the output
      --filenames            Include the file names in the output
  -r, --remove               Remove some stuff with hard-coded regular expressions
```

## Examples

```
cargo run --release -- -f PMC7405720.xml.json > out.txt
cargo run --release -- -d ./pmc_json/ -r -s --filenames
```