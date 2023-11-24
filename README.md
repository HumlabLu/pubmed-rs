# pubmed-rs

Simple tool to extract `{text: ...}` from the articles in the COVID-19 corpus.

Optionally includes filenames or section names in the output.

Output is written to standard out.

## Usage

```
cargo run --release -- -f PMC7405720.xml.json > out.txt
cargo run --release -- -d ./pmc_json/ -r -s -f

```