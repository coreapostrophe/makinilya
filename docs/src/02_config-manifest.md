# Config Manifest

The configuration of Makinilya is defined on a `Config.toml` file at the root of the project directory. It is written on a [TOML](https://toml.io/) format. **Every property inside the configuration is defaulted** and could be omitted if so desired.

The manifest could have any of the following sections:

- [`project`](#project) - section that defines the paths and directories of the project
  - [`base_directory`](#base_directory) - prefix root of project files
  - [`draft_directory`](#draft_directory ) - directory path of the manuscript draft
  - [`output_path`](#output_path) - file path of the final generated manuscript
  - [`context_path`](#context_path) - file path to the narrative's context
- [`story`](#story) - section that defines the manuscript details
  - [`title`](#title) - title of the narrative
  - [`pen_name`](#pen_name) - cover pseudonym of the author
- [`author`](#author-and-agent) - section that defines the contact information of the author
  - [`name`](#name) - name of the author
  - [`address_1`](#address_1) - first address of the author
  - [`address_2`](#address_2) - second address of the author
  - [`mobile_number`](#mobile_number) - mobile number of the author
  - [`email_address`](#email_address) - email of the author
- [`agent`](#author-and-agent) - section that defines the contact information of the author's agent
  - [`name`](#name) - name of the author's agent
  - [`address_1`](#address_1) - first address of the author's agent
  - [`address_2`](#address_2) - second address of the author's agent
  - [`mobile_number`](#mobile_number) - mobile number of the author's agent
  - [`email_address`](#email_address) - email of the author's agent

## Project

This section defines paths and directories of all relevant project files.

```toml
[project]
base_directory = "./"
draft_directory = "draft"
output_path = "./out/manuscript.docx"
context_path = "Context.toml"
```

### `base_directory`

Default: `./`

The root directory path of the project files. Every other path definitions inside the [`project`](#project) section is prefixed by the base directory. This property is useful when the project files are separated from the directory of the manifest.

### `draft_directory`

Default: `draft`

The directory path where all of the chapters and scenes of the narrative is found. When makinilya initializes its `Story` structure, it recursively searches for scenes inside the draft directory that contains a `.mt` extension.

### `output_path`

Default: `./out/manuscript.docx`

The file path where the generated manuscript is outputted. The file that Makinilya generates is in a word document `docx` format, thus it is recommended that the path also includes the extension.

### `context_path`

The file path where the narrative context is defined. The context contains all of the user-created values that is interpolated to the narrative when building the manuscript.

## `story`

This section defines general details of the narrative.

```toml
[story]
title = "Mock Story"
pen_name = "Brutus Ellis"
```

### `title`

Default: `Unitled`

The title of the narrative.

### `pen_name`

Default: `Unknown Author`

The pen name of the author. This is the pseudonym that'll be used in the title page of the manuscript.

## `author` and `agent`

This sections defines the contact information of the author and the author's agent respectively. They both have the same set of properties. Each of their properties do not have defaults and are omitted from the manuscript when left undefined.

```toml
[author]
name = "Brutus Ellis"
address_1 = "2688 South Avenue"
address_2 = "Barangay Olympia, Makati City"
mobile_number = "+63 895 053 4757"
email_address = "brutusellis@email.com"

[agent]
name = "Cymone Sabina"
address_1 = "755 Maria Clara Street"
address_2 = "Mandaluyong City"
mobile_number = "+63 908 524 4125"
email_address = "cymonesabina.@email.com"
```

### `name`

The full name of the individual.

### `address_1`

The first address of the individual.

### `address_2`

The second address of the individual.

### `mobile_number`

The mobile number of the individual.

### `email_address`

The email address of the individual.
