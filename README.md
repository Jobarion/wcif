# wcif
This crate implements types for the [WCA Competition Interchange Format](https://github.com/thewca/wcif/tree/master).
It closely follows the specification, parsing string values into more expressive types when possible.

## Features
- `parse_puzzle_type` Parse and verify puzzle and event types into an enum (i.e. instead of the string "333", it's `OfficialPuzzleType::Cube333`).
- `parse_attempt_result` Results of attempts are stored as integers with some values having special meaning. This parses the integer into an enum.
- `parse_activity_code` Parses activity code strings into a struct representing the activity code with individual fields for event, round, group and attempt.
- `groupifier` Read extensions defined by [Groupifier](https://groupifier.jonatanklosko.com/)
