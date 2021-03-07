# pcache

Program cache is a tool used in testing to cache calls to programs.

## State

> README only project

## Purpose

### The problem

Some programs rely on other commandline tools to perform a certain task.

When the program is using a basic tool which is almost always available like `ls`
this can case little problems, so having a call to the third party tool in a test
is not a big deal.

But when you need something specific, you need to make sure the tool is installed
while running the test. An example is f.e. ORT which is written in kotlin, but
requires f.e. `dep` to be installed for getting go-lang dependencies.

One way to improve on this, is to use dummy output of the tools in your unit-test
in stead of actually calling the tool.

This is fine when you also have integration tests, because the integration tests
will reveal problems when the 3rd party tools changed their output.

Once the integration test fails, you have to see the new output of the tool, and
need to update the unit test.

Feels a little bit cumbersome, and sometimes projects do not have integration tests.

### A solution

We can also make some kind of recorder to record the output of a call to a tool
and write that in a file which is included in the repository.

The second time the test is executed the tool will take the output from the file
and no actual call is made to the tool.

This way you only need a specific tool installed when caching the output, not for
every environment which runs the unit test.

In case of a license-scanner like ORT, this can be really useful, because it depends
on >10 external tools for 10 different languages.

Adding a retention date or a scheduled workflow in project which removes the
cached files and pushes the new cache will make sure you're constantly testing
against the latest output of the tools.

This way of working I've learned from [the Ruby Project - VCR](https://github.com/vcr/vcr).
VCR does this for HTTP calls.

`pcache` aims to do this for calling linux based tools.


## Usage

### Cache output text file
```bash
$pcache cat test.txt
```

### Cache date 
```
$pcache date -u +"%Y-%m-%dT%H:%M:%SZ"
```

### Arguments

| Argument  | Default value | Description |
| --------- | ------------- | ----------- |
| `--scope` |  | Scope of a pcache-file. Will be appended to the filename |

### Configuration

Configuration can be stored in a `.pcache.yml` file.

| Parameter | Default value | Description |
| --------- | ------------- | ----------- |
| `directory` | `./pcache-files` | Directory where to store the files |
| `retention` | `∞` | Retention time in days |

## Development 

```bash
cargo build
```

```bash
cargo run date 
```

## Inspirations

This way of working I've learned from [the Ruby Project - VCR](https://github.com/vcr/vcr).

## License

MIT
