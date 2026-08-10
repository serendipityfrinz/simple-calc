# Changelog

All notable changes to `simple-calc` will be documented in this file.

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
