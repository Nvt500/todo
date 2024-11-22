
# Todo List

Simple todo list cli written in rust.

Make sure a file called ```todo.txt``` is in the directory of the executable.
Or use the add command to create the file (del and show don't create the file). 

# Usage

```text
Simple todo list cli.

Usage: todo.exe <COMMAND>

Commands:
  show  Show list
  add   Add task to list
  del   Remove task from list
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Add

```text
Add task to list

Usage: todo.exe add <TASK>

Arguments:
  <TASK>  Task to add

Options:
  -h, --help  Print help
```

## Delete

```text
Remove task from list

Usage: todo.exe del <TASK>

Arguments:
  <TASK>  Task, index, or range (eg. 1-5) to remove

Options:
  -h, --help  Print help
```

## Example

```text
> todo add "First Task"
Adding task First Task
> todo add "Second Task"
Adding task Second Task
> todo add "Third Task"
Adding task Third Task
> todo add "Fourth Task"
Adding task Fourth Task
> todo show
1: First Task
2: Second Task
3: Third Task
4: Fourth Task
> todo del "First Task"
Deleting task First Task
> todo show
1: Second Task
2: Third Task
3: Fourth Task
> todo del 1
Deleting task 1
> todo show
1: Third Task
2: Fourth Task
> todo del 1-2
Deleting task 1-2
> todo show
No tasks.
```