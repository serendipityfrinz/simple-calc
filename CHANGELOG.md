# Changelog

All notable changes to `simple-calc` will be documented in this file.

## [0.1.75] - 2026-08-11
### Added
- Executing simple-calc now clears the terminal so the terminal looks clean when you do your work.
- Added "CLI" into the description. "A (not so) simple **-> CLI <-** calculator made in Rust."

## [0.1.7] - 2026-08-11
### Added
- Square root in advanced mode (exponentiation finally has a friend!)
- Rounding
- Yes/No vector variable that i will use in next update (0.1.8)
- simple-calc header at the top of the application (the missing piece of the puzzle)

### Changed
- Numbering of Clear screen and Exit for the favor of Rounding
- Changed description of simple-calc because i found out that it isn't actually overengineered at all! it is just pragmatic and straightforward code.
- Changed header size in README.md and changed text there along the way.

## [0.1.5] - 2026-08-11
### Added
- New advanced options with exponentiation for now
- Option to clear the screen
- New dependency: clearscreen (4.0.6)
- Note from creator in README.md

### Changed
- Changed `inquire::*` to `inquire::{Select, CustomType}`
- Changed "What? How did that happen?" message to unreachable! message with 
  "You selected an option that isn't on the menu!"
- Moved num1 and num2 to the start and added _ at the start (_num1, _num2)
  
  *(Reason: i moved the inquiry of num1 and num2 to an else condition,
    i did this solely to silence 2 warnings.)*

## [0.1.1] - 2026-08-10

### Added
- *Dependencies*, *usage*, and *license* part in README.md.

## [0.1.0] - 2026-08-10

### Added
- Initial release with basic math operations.
