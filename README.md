# JSHP

## What Is This?
Memory efficient, fast, and easy to use Node.js hypertext preprocessor, kind of like php but definitely better :tm:.

### Memory Efficient

It's written in Rust, so it's memory efficient by default.  
It definitely does not store the entire files in memory.  
And even if it did, how much RAM do you have? How much RAM does your cloud have? Right, I thought so.

## TODO:

#### Base Functionality

- [x] Parsing jshp tags
- [ ] Reprocessing files
- [ ] Implement CLI
- [ ] Implement `check_syntax` - checking the syntax of code fragments (`node --check` or directly with node:vm module)
  before startup
- [ ] Better Node subprocess "management"

#### Big If True

- [ ] Typescript support
- [ ] node_modules support / require() support

#### Misc

- [ ] Colored status output for startup checks/warnings/errors
- [ ] Add a `LICENSE` file
- [ ] Benchmarking


---

<details>
  <summary>But... Why?</summary>

  ### Learning experience.  

  It might not look like much but this is the best first project in any language, see:
  - It's made of multiple primitive parts that are rather easy to implement  
  - Simple syntax parsing, simple web serving, file IO stuff, almost simple (sub-)process management

</details> 
