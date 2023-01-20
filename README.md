# pompom

This crate is a simple pomodoro cli based terminal timer.

## About Pomodoro Technique

Following are the excerpts from [Wikipedia](https://en.wikipedia.org/wiki/Pomodoro_Technique):

-   The Pomodoro Technique is a time management method developed by Francesco Cirillo in the late 1980s.[1] It uses a
    kitchen timer to break work into intervals, typically 25 minutes in length, separated by short breaks. Each interval
    is known as a pomodoro, from the Italian word for tomato, after the tomato-shaped kitchen timer Cirillo used as a
    university student.
-   The technique has been widely popularized by apps and websites providing timers and instructions. Closely related to
    concepts such as timeboxing and iterative and incremental development used in software design, the method has been
    adopted in pair programming contexts.

### Description

The original technique has six steps:

-   Decide on the task to be done.
-   Set the pomodoro timer (typically for 25 minutes).[1]
-   Work on the task.
-   End work when the timer rings and take a short break (typically 5-10 minutes).[5]
-   If you have finished fewer than **three** pomodoros, go back to Step 2 and repeat until you go through all three
    pomodoros.
-   After three pomodoros are done, take the fourth pomodoro and then take a long break (typically 20 to 30 minutes).
    Once the long break is finished, return to step 2.

For the purposes of the technique, a pomodoro is an interval of work time.[1]

Regular breaks are taken, aiding assimilation. A 10-minute break separates consecutive pomodoros. Four pomodoros form a
set. There is a longer 20-30 minute break between sets.[1][6]

A goal of the technique is to reduce the effect of internal and external interruptions on focus and flow. A pomodoro is
indivisible; when interrupted during a pomodoro, either the other activity must be recorded and postponed (using the
inform - negotiate - schedule - call back strategy[7]) or the pomodoro must be abandoned.[1][6][8]

After task completion in a pomodoro, any remaining time should be devoted to activities, for example:

Review your work just completed (optional)
Review the activities from a learning point of view (ex: What learning objective did you accomplish? What learning
outcome did you accomplish? Did you fulfill your learning target, objective, or outcome for the task?)
Review the list of upcoming tasks for the next planned pomodoro time blocks, and start reflecting on or updating them.
Cirillo suggests:

Specific cases should be handled with common sense: If you finish a task while the Pomodoro is still ticking, the
following rule applies: If a Pomodoro begins, it has to ring. It's a good idea to take advantage of the opportunity for
overlearning, using the remaining portion of the Pomodoro to review or repeat what you've done, make small improvements,
and note what you've learned until the Pomodoro rings.[9]

The stages of planning, tracking, recording, processing and visualizing are fundamental to the technique.[10] In the
planning phase, tasks are prioritized by recording them in a "To Do Today" list, enabling users to estimate the effort
they will require. As pomodoros are completed, they are recorded, adding to a sense of accomplishment and providing raw
data for subsequent self-observation and improvement.[1]

## Dependencies

-   Works on Linux and MacOS, and Windows Subsystem For Linux (WSL).
-   TODO: Install `libdbus-1` to integrate pompom with linux's notification system.

## Installation

Works both on stable and nightly rust.

Install the application with the command:

### Crates.io

_Stable_

```terminal
$ cargo install pompom
```

_Nightly_

```terminal
$ cargo +nightly install pompom
```

### Install or reinstall the package in the source current directory

You'll need to either Clone or Download the source code before building & installing the binary.

_1.Build binary._

```terminal
$ cd pompom
$ cargo build --release --bin pompom
```

_2. Install binary to your OS._

```terminal
$ cargo install --path .
```

## Using pompom

Run pompom directly in your terminal. It currently defaults to:

-   25 minutes of work time.
-   5 minutes of break.
-   20 minutes of long break.

```terminal
$ pompom
```

### Flags

To customize the pompom settings, you can pass flags into the terminal.

-   `-w` | `--work` - sets the work time.
-   `-s` | `--shortbreak` - sets the short break time.
-   `-l` | `--longbreak` - sets the long break time.

## Examples

```terminal
$ pompom -w 45 -s 15 -l 25
```

```terminal
$ pompom --work 45 --shortbreak 15 --longbreak 25
```
