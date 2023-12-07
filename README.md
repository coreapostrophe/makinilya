
## ✒️ Makinilya

![Release](https://github.com/coreapostrophe/makinilya/actions/workflows/release.yml/badge.svg) ![Build](https://github.com/coreapostrophe/makinilya/actions/workflows/build.yml/badge.svg)
![Makinilya Cover](./assets/makinilya-cover.jpg)

### About

Makinilya is a simple manuscript generator for ascetic writers. It provides a simple string interpolation functionality that targets those free-flowing authors who wants to  write narratives without worrying about names or any other arbitrary information.

Instead of explicitly naming characters like,

```plaintext
Hi, my name is Alyssa.
```

Makinilya allows you to store the name in a tree context which can then be referenced via `{{ }}` enclosures.

```plaintext
Hi, my name is {{ names.main_character }}
```

### Crates

1. [`makinilya-cli`](./makinilya-cli/) - The main crate that serves the command-line interface of the application.
2. [`makinilya-core`](./makinilya-core/) - The core crate that contains the functionalities that reads the project, interpolates the content, and builds the manuscript.
3. [`makinilya-text`](./makinilya-text/) - The laanguage crate that contains the implementation of the parser and its grammar rules.

### Installation

- Install prebuilt binaries via shell script

```shell
curl --proto '=https' --tlsv1.2 -LsSf <https://github.com/coreapostrophe/makinilya/releases/download/v0.1.0-alpha.1/makinilya-installer.sh> | sh
```

- Install prebuilt binaries via powershell script

```powershell
irm https://github.com/coreapostrophe/makinilya/releases/download/v0.1.0-alpha.1/makinilya-installer.ps1 | iex
```

- Alternatively you can download binary installers in the [releases page](https://github.com/coreapostrophe/makinilya/releases).

### Tutorial

The structure of your project is straightforward. The cli, upon building, will scan the whole directory tree that's inside the `draft/` folder to parse the story. It is necessary that you place all of your final work inside that directory.

Try to base the contents of your project from the tree below:

```plaintext
draft/
├─ Chapter 1/
│  ├─ Scene 1.mt
│  ├─ Scene 2.mt
├─ Chapter 2/
│  ├─ Scene 1.mt
│  ├─ Scene 2.mt
Context.toml
```

The `Context.toml` file in the root of your project will contain all of the global values that you could use to interpolate in your story. It doesn't matter how you structure the values inside the file. You only need to remember how to reference them from it.

For example, the nickname of the main character, with this context,

```toml
[names]
main_character = { short = "Alyssa", long = "Alyssa Grandau", nick = "Aly" }

```

is referenced like the following:

```plaintext
"{{ names.main_character.nick }}! I've looked everywhere for you. Where have you been?" he said.
```

Once you're done writing your chapters. All you have to do is build the project. To do that, you need to type the following command on the root of your project.

```bash
makinilya build
```

Makinilya will then create a `manuscript.docx` with all of the interpolated strings in a generated `out/` directory.

### Future Plans

The application is currently in alpha and is still wildly unstable. I only created an early release so that I could use it personally on my own writing projects. It's still far from the final release (v1.0.0) as I still have a lot of things I need to check on my list. Before then, aside from additional optimizations, you can anticipate the following features:

- a cli new command, possibly with the syntax of `makinilya new [PATH]`, that creates a blank project.
- a cli generator command, possibly with the syntax of `makinilya gen [OPTIONS]`, that generates default chapters and scenes.
- a language server extension for the text content linting and error handling. It will mainly just target issues revolving around context variable resolutions.

After the final release, I also plan to create a cross-platform native text-editor application that syncs the users work along devices.
