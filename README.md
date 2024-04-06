
# Makinilya

<p align="center">
    <img
        src="https://raw.githubusercontent.com/coreapostrophe/makinilya/main/assets/makinilya-logo.png"
        style="height:200px"/>
</p>
<p align="center">
    A manuscript generator for ascetic writers.
</p>
<p align="center">
    <img src="https://github.com/coreapostrophe/makinilya/actions/workflows/release.yml/badge.svg">
    <img src="https://github.com/coreapostrophe/makinilya/actions/workflows/build.yml/badge.svg">
</p>

## Installation

- Install prebuilt binaries via shell script

```shell
curl --proto '=https' --tlsv1.2 -LsSf <https://github.com/coreapostrophe/makinilya/releases/download/v0.1.0-alpha.1/makinilya-installer.sh> | sh
```

- Install prebuilt binaries via powershell script

```powershell
irm https://github.com/coreapostrophe/makinilya/releases/download/v0.1.0-alpha.1/makinilya-installer.ps1 | iex
```

- Alternatively you can download binary installers in the [releases page](https://github.com/coreapostrophe/makinilya/releases).

## About

For the longest time, writing fiction has been a challenge of organizing ideas. There are a lot of overlapping details you need to keep track of to maintain continuity, such as names of places and characters, time of events, and et cetera. Traditionally, if we want to change these information in the middle of a composition (perhaps after discovering an apt alternative), we'd have to go through all of the earlier sections and rewrite them. This has been a source of frustration for most writers, I included. Makinilya is my solution to such problem.

Makinilya is a manuscript generator that parses a project tree and allows authors to have a free-flowing writing workflow through powerful layouting features such as string-interpolation.

## Brief Example

### Project structure

The project is simple. Makinilya parses the contents of the `draft` directory to a story. The folders are parsed as chapters, and the files with the `.mt` extension are parsed as scenes.

```plaintext
draft/
├─ Chapter 1/
│  ├─ Scene 1.mt
│  ├─ Scene 2.mt
├─ Chapter 2/
│  ├─ Scene 1.mt
│  ├─ Scene 2.mt
Config.toml
Context.toml
```

### `Config.toml`

This is the configuration of the manuscript. It will include all of the information that will be rendered in the title page of the document when we run the `build` command.

```toml
[story]
title = "Untitled"
pen_name = "Brutus Ellis"

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

### `Context.toml`

Instead of explicitly writing tentative information. We can store information in a tree-structured context which can later be referenced in scenes. This way, when we can change them at any time.

```toml
# This is a toml header. They are like a namespace of some sort.
[mc]                        

# This is a toml property. We can reference this property through`mc.age`
age = 21

# We can also nest objects inside properties.
name = { short = "Core" }
```

>The possible values in the context are currently limited to `Strings`, `Booleans`, `Numbers`, and nested `Objects`. To learn more about toml and how to structure them, refer to the official [toml language spec](https://toml.io/en/v1.0.0).

### `draft/"Chapter 1"/"Scene 1".mt`

Inside the actual scenes, we can write our actual narrative. Here, we can interpolate the context information by writing their identifiers inside brace enclosures `{{ }}`.

```plaintext
Hi, my name is {{ mc.name.short }}. I'm {{ mc.age }} years old.
```

In order to build the manuscript. We have to run the build command at the root of the project tree.

```bash
makinilya build
```

This will generate a `manuscript.docx` file inside a generated `out` directory within the project.
