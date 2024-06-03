# pubmed-rs

Simple tool to extract `{text: ...}` from PubMed articles in JSON format.

Optionally includes filenames or section names in the output. Output can be plain text or JSON.

Output is written to standard out.

A separate option to generate a list with abbreviations is available. 

## Section Types

These are the counts in about 75000 articles.

| Count  | Section Type | :x: Ignored |
| ------------: | ------------- | ---| 
|    2851       | `KEYWORD` | :x: | 
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
|  994629       | `METHODS` |  |
| 3035484       | `REF` | :x: |

Text from the sections is included if their type is `paragraph` and the `section_type` is not on the ignore list. If the `--allowed` parameter has been specified, only those types will be added to the output. In that case the ignore list is disbaled.

A typical PubMed text looks like this.
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
  -f, --filename <FILENAME>   Filename of the JSON file to parse
  -d, --dirname <DIRNAME>     Directory name
  -j, --json                  Output JSON instead of plain text
  -m, --maxfiles <MAXFILES>   If specified, maximum number of files to process from directory
  -s, --sectionnames          Include the section names in the output
  -F, --filenames             Include the file names in the output
  -S, --sentences             Sentence splitter
  -a, --abbreviations         Remove some stuff with hard-coded regular expressions. Output only abbreviations
  -A, --allowed <ALLOWED>...  Allowed sections
  -h, --help                  Print help
  -V, --version               Print version
```

## Examples

```
cargo run --release -- -f PMC7405720.xml.json > out.txt
cargo run --release -- -d ./pmc_json/ -s --filenames
```

## Abbreviations

Abbreviations are taken from the `ABBR` sections in the documents. There are no markers identifying which is abbreviation or definition, so this is determined heuristically by the program.

Sometimes the abbreviations are in one big paragraph. On other occasions it just contains text.
```text
AD      Alzheimer’s disease
AD, Alzheimer’s disease; aMCI, amnestic mild cognitive impairment; PID, persistent insomnia disorder; DSM V, Dia
gnostic and statistical manual of mental disorders, 5th edition; HADS-A/D, Hospital anxiety and depression scale
, Anxiety/Depression; ISI, Insomnia severity index; MCI, mild cognitive impairment; MoCA, Montreal cognitive ass
essment tool; PSQI, Pittsburgh sleep quality index; TIB, Time in bed; REM, rapid eye movement.  As much of the d
ata pertains to patient information, the data will not be made publicly available. However, the data that suppor
t the findings of this study are available upon reasonable request from the corresponding author.
AD-MSCs Adipose-derived mesenchymal stem cells
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
  "sentences": [
    {
      "type": "ABSTRACT",
      "text": "Tendon injuries have a high incidence and limited treatment options. Stem cell transplantation isessential for several medical conditions like tendon injuries. However, high local concentrations of reactive oxygenspecies (ROS) inhibit the activity of transplanted stem cells and hinder tendon repair. Cerium oxide nanoparticles (CeONPs) have emerged as antioxidant agents with reproducible reducibility."
    },
    {
      "type": "ABSTRACT",
      "text": "However, high local concentrations of reactive oxygenspecies (ROS) inhibit the activity of transplanted stem cells and hinder tendon repair. Cerium oxide nanoparticles (CeONPs) have emerged as antioxidant agents with reproducible reducibility. See https://durian.org/foo."
    }
  ],
  "abbreviations": {
    "ROS": "Reactive oxygen species"
  },
  "year": "2023",
  "pmid": "10546722",
  "title": "Cerium oxide nanoparticles-carrying human umbilical cord mesenchymal stem cells counteract oxidative damage and facilitate tendon regeneration"
}
```

## Installation

After cloning the git repo, run the following.
```shell
cd pubmed-rs
cargo install --path .
```
