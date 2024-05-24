# pubmed-rs

Simple tool to extract `{text: ...}` from PubMed articles in JSON format.

Optionally includes filenames or section names in the output. Output can be plain text or JSON.

Output is written to standard out.

A separate option to generate a list with abbreviations is available. 

## Section Types

These are the counts in about 75000 articles.

| Count  | Section Type | :x: Ignored |
| ------------: | ------------- | ---| 
|    2851       | `KEYWORD` |  | 
|    4916       | `REVIEW_INFO` | :x: |
|   18038       | `APPENDIX` | :x: |
|   19429       | `CASE` | :x: |
|   27063       | `ACK_FUND` | :x: |
|   33374       | `AUTH_CONT` | :x: |
|   40131       | `COMP_INT` | :x: |
|   60772       | `SUPPL` | :x: |
|   74833       | `TITLE` |  |
|  112995       | `ABBR` |  (special) |
|  133909       | `CONCL` | |
|  230237       | `ABSTRACT` |  |
|  326559       | `FIG` | :x: |
|  399120       | `TABLE` | :x: |
|  451921       | `DISCUSS` |  |
|  707622       | `RESULTS` |  |
|  786911       | `INTRO` |  |
|  994629       | `METHODS` | :x: |
| 3035484       | `REF` | :x: |

Text from the sections is included if their type is `paragraph` and the `section_type` is not on the ignore list.

A typical text to be included looks like this.
```json
{
      "offset": 1879,
      "infons": {
          "section_type": "INTRO",
          "type": "paragraph"
      },
      "text": "DNA methylation is an epigenetic modification [...] activity.",
      "sentences": [],
      "annotations": [],
      "relations": []
},
```


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
  -a, --abbreviations        Output only abbreviations
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples

```
cargo run --release -- -f PMC7405720.xml.json > out.txt
cargo run --release -- -d ./pmc_json/ -s --filenames
```

## Output

The simplest form of output is plain text without section and file names.
```text
Tendon injuries have a high incidence and limited treatment options. Stem cell transplantation isessential for several medical conditions like tendon injuries. However, high local concentrations of reactive oxygenspecies (ROS) inhibit the activity of transplanted stem cells and hinder tendon repair. Cerium oxide nanoparticles (CeONPs) have emerged as antioxidant agents with reproducible reducibility.
However, high local concentrations of reactive oxygenspecies (ROS)
```

With section names, the names and text are tab-separated.
```text
ABSTRACT	Tendon injuries have a high incidence and limited treatment options. Stem cell transplantation isessential for several medical conditions like tendon injuries. However, high local concentrations of reactive oxygenspecies (ROS) inhibit the activity of transplanted stem cells and hinder tendon repair. Cerium oxide nanoparticles (CeONPs) have emerged as antioxidant agents with reproducible reducibility.
ABSTRACT	However, high local concentrations of reactive oxygenspecies (ROS)
```

The abbreviation list is a tab-separated keyword value list. This can be shown with the `-a` option.
```text
ROS	Reactive oxygen species
```

The JSON output contains all the available information.
```json
{
  "abbreviations": {
    "ROS": "Reactive oxygen species"
  },
  "paragraphs": [
    {
      "par_type": "ABSTRACT",
      "text": "Tendon injuries have a high incidence and limited treatment options. Stem cell transplantation isessential for several medical conditions like tendon injuries. However, high local concentrations of reactive oxygenspecies (ROS) inhibit the activity of transplanted stem cells and hinder tendon repair. Cerium oxide nanoparticles (CeONPs) have emerged as antioxidant agents with reproducible reducibility."
    },
    {
      "par_type": "ABSTRACT",
      "text": "However, high local concentrations of reactive oxygenspecies (ROS) inhibit the activity of transplanted stem cells and hinder tendon repair. Cerium oxide nanoparticles (CeONPs) have emerged as antioxidant agents with reproducible reducibility."
    },
    {
      "par_type": "RESULTS",
      "text": "The hydrophobic CeONPs were synthesized by a previously reported thermal decomposition method. After coating with mPEG2k-DSPE, CeONPs were transferred to the aqueous phase. As shown in Fig. 2A, the obtained PEG-CeONPs exhibited uniform morphologies with sizes of 5.45 ± 1.08 nm. Moreover, CeONPs were stable in an aqueous solution for at least 2 weeks, as evidenced by their appearance and the results of the UV-visible spectra analysis (Fig. 2E, F)."
    }
  ]
}
```

## Debug

Rus with `RUST_LOG` set to `info` or `´debug`.
```shell
RUST_LOG=info cargo run --release ...
```
