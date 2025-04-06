# TBUD
tbud is a Rust CLI aplication used for saving, listing, deleting and running named commands to help developers. It features support for single and multiple line commands. All from the Windows command line.

tbud shines in repetitive tasks, allowing custom scripts to be organized ,or simply avoiding copy-paste hell. tbud also works in vscode terminal.


## Install

### Windows

* Clone and build the app (process listed below) or download the exe on releases

```
git clone https://github.com/yourusername/tbot
cd tbot
cargo build --release
```

* Add to the PATH variables

Now you can use tbud in your terminal.

## Usage

### Add a new command

```
tbot add myscript
```

or with alias:

```
tbot a myscript
```
Then type your command(s) between  ```::start ``` and ```::end:```

```
::start
cd C:\Projects\my-app
git pull
cargo run
::end
```
### List saved commands
``` tbot list``` 

or with alias:

``` tbot ls ```


### Run a saved command

``` tbot run myscript ```

or with alias:

```tbot r myscript```

### Delete a command
``` tbot delete myscript ```

or with alias:

```tbot d myscript ```