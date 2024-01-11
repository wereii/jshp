# JSHP

## What Is This?

Memory efficient :roller_coaster:, fast ⚡, and easy to use Node.js hypertext preprocessor, like PHP but better™.

### Memory Efficient

It's written in Rust, so it's memory efficient by default.  
<sup><sub>
It definitely does not store entire files in memory.  
And even if it did, how much RAM do you have? How much RAM does your cloud have? Right, I thought so.  
Look, reading from disk is slow, and reading from memory is fast and everyone wants fast.
</sub></sup>

## How To Use It?

<!--- TODO -->

## Installation

<!--- TODO -->

## Benchmarks

<!--- Write some bullshit, cherry-picked benchmarks here (characters per second, etc.) -->

## TODO:

#### Base Functionality

- [x] Parsing code from jshp tags
- [ ] HTTP serving
- [ ] Tag parsing
    - [ ] Implement better parsing errors (unclosed tag, missing closing tag, etc.)
    - [ ] Finish echo tag `<?= ... ?>`
- [ ] Implement CLI
- [ ] Implement `check_syntax` flag - checking the syntax of code fragments

#### Big If True

- [ ] Typescript support ?
- [ ] node_modules support / require() support ??
- [ ] wasm ???

#### Misc

- [ ] Readme needs more emojis
- [ ] Colored status output for startup checks/warnings/errors
- [ ] Add a `LICENSE` file
- [ ] Benchmarking

---

<details>
  <summary>But... Why?</summary>

### Learning experience.

It might not look like much but this is the best first project in any language, see:

- It's made of multiple primitive parts that are rather easy to implement
- Simple syntax parsing, simple file IO stuff, almost simple HTTP serving, ignore the V8 stuff 
  (originally it should use Node.js directly, but embedding V8 means faster, and everyone wants faster)
</details>
