# freeCodeCamp - External Project Template

Shaun is having fun testing a minimal way to run courses locally.

## Shaun's TODO:

- [ ] Refactor tooling/file names and structure to something that makes sense
- [ ] Add workflow for translations to Crowdin
- [x] Fix `package.json` issue with script `$PWD` in `.bashrc`
- [x] `Go to Next Lesson` button?
- [ ] Enable seed parsing for resetting
- Feature Flags:
  - [ ] Always use seed on next lesson
  - [ ] Use Git build for seed on production
  - [x] Run tests on watch
- [ ] Enable use of Git build on production

## Main Features

- Dockerised setup for consistency
- Familiar Markdown syntax for lessons
- Quick and simple Gitpod setup for testing/developing
- Testing can be done with **any** valid Nodejs
- _freeCodeCamp-stylised_ preview for instruction delivery

### Feature Flags

- `RUN_TESTS_ON_WATCH` - whether or not to run the tests on every input/save/change - defaults to `false`
- `CURRENT_PROJECT` - must match `/tooling/locales/en/<project_name>.md` filename
- `LAST_KNOWN_LESSON_WITH_HASH` - legacy, but left in case we want a Git-saving workflow
- `TEST_POLLING_RATE` - how often to refresh for changes in `ms`
- `LOCALE` - one of `/tooling/locales/conf.js` - defaults to `english`

## How To Run the Course

### In Gitpod!

1. Ensure you have the Gitpod extension. _Otherwise [click here](https://gitpod.io/#https://github.com/ShaunSHamilton/external-project)_
2. Once Gitpod opens, ensure it is done doing its stuffs
3. `source .freeCodeCamp/tooling/.bashrc` likely will not work. So, manually run this.
4. Open Command Palette, run `freeCodeCamp: Develop Course`

**Notes**

- When doing the course, ensure you are in the `curriculum/` directory
- The preview window is forced to refresh quite often - it might look like it is flickering

### Locally

1. Create an empty directory:

```bash
mkdir course
cd course
```

2. Download/Install `freeCodeCamp Courses` extension in VSCode.

```bash
wget https://github.com/ShaunSHamilton/courses-plus/raw/main/freecodecamp-courses-patch.vsix
code --install-extension freecodecamp-courses-patch.vsix
```

3. Open VSCode

```bash
code .
```

4. `Ctrl + Shift + P` and select `freeCodeCamp: Open Course`

5. Choose the course you want to run

6. `Ctrl + Shift + P` and select `Remote-Containers: Rebuild and Reopen in Container`

7. `Ctrl + Shift + P` and select `freeCodeCamp: Run Course`

## Ideal Dev Setup

1. Create an empty directory:

```bash
mkdir course
cd course
```

2. Download/Install `freeCodeCamp Courses` extension in VSCode.

```bash
wget https://github.com/ShaunSHamilton/courses-plus/raw/main/freecodecamp-courses-patch.vsix
code --install-extension freecodecamp-courses-patch.vsix
```

3. Open VSCode

```bash
code .
```

4 `Ctrl + Shift + P` -> `freeCodeCam: Develop New Course`

5. Follow development steps in [Creating a Course](#creating-a-course)

## Creating a Course (_In Progress_)

### General Information

- You should not need to make any changes other than to the following files:
  - `.freeCodeCamp/tooling/locales/english/project-1.md`
  - Edit this `README.md` to include information about the course

### Adding the Course to the `freeCodeCamp Courses` Extension

1. Go to the [freeCodeCamp/freecodecamp-course repo](https://github.com/freeCodeCamp/freecodecamp-course)
2. Open a PR adding to the [resources/courses.json](https://github.com/freeCodeCamp/freecodecamp-course/blob/master/resources/courses.json) file.

## Markdown Syntax

### Headers

- `## <number>` - Lesson number heading _(required)_
- `### --description--` - Lesson description _(required)_
- `### --tests--` - Tests for lesson _(required)_
- `### --seed--` - Seed for lesson _(optional)_
- `#### --cmd--` - Command for lesson seed (Camper will see) _(optional)_
- `#### --cmd-hidden--` - Command for lesson seed (Hidden from Camper) _(optional)_
- `#### --"<string>"--` - File name and seed heading _(optional)_

### Example

````markdown
## 1

### --description--

Teach Campers amazing things here...

### --tests--

At least one line as a hint in the tests

```js
assert("Chai.js tests with async code");
```

### --seed--

#### --cmd--

```bash
cd somewhere
```

#### --cmd-hidden--

```bash
echo Campers should not see this
```

#### --"somewhere/index.rs"--

```rust
let seed_for_file = "I am what the 'somewhere/index.rs' file should start with'";
```

#### --cmd--

```bash
echo "All seed is run in order"
```

## 2
````

### Notables

- Below is the template for the course README

---

# freeCodeCamp - <course name>

## Course Desciption

Add a description of the course here. Example:

_This course will introduce you to the Rust programming language._

## Prerequisites

- Intermediate knowledge of at least one other programming language
- Basic knowledge of Bash

## Learning Outcomes

- Variables
- Primitives
- Typing
- Ownership
- Handling Environment Arguments
- Parsing Strings into Floats
- Functions
- Structs
- Tuples
- Enums
- Vecs
- Control Flow
- Testing
- Traits
- Decoding Images
- Building
- Error Handling

## How to Get Started

Instructions for how to run the course on different platforms (e.g. Locally, Gitpod, Replit)
