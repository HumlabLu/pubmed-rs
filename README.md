# pubmed-rs

Simple tool to extract `{text: ...}` from the articles in the COVID-19 corpus.

Optionally includes filenames or section names in the output.

Output is written to standard out.

## Usage

```
Usage: pubmed-rs [OPTIONS]

Options:
  -f, --filename <FILENAME>  Filename of the XML file to parse
  -d, --dirname <DIRNAME>    Directory name
  -m, --maxfiles <MAXFILES>  Maximum number of files to process
  -s, --sectionnames         Include the section names in the output
      --filenames            Include the file names in the output
  -r, --remove               Remove some stuff with regular expressions
```

## Examples

```
cargo run --release -- -f PMC7405720.xml.json > out.txt
cargo run --release -- -d ./pmc_json/ -r -s -f
```