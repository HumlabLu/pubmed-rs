# pubmed-rs

Simple tool to extract `{text: ...}` from PubMed articles in JSON format.

Optionally includes filenames or section names in the output. Output can be plain text or JSON.

Output is written to standard out.

A separate option to generate a list with abbreviations is available. 

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
  -a, --abbreviations        Output only abbreviations
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples

```
cargo run --release -- -f PMC7405720.xml.json > out.txt
cargo run --release -- -d ./pmc_json/ -r -s --filenames
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

The abbreviation list is a tab-separated keyword value list.
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


