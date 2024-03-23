# TODO

- [x] add nested attribute `#>[meta]` support
- [x] fix where clause
- [ ] improve documentation
- [ ] write README.md
- [ ] add recursive generic support
- [ ] fix bug where `_ : <ty> struct Name {}` is valid when it should not be
- [x] rework fish support
- [x] rename fish to FishHook
- [ ] fix spans
- [ ] fix issue where `struct { };` "unexpected `;`" error is not spanned correctly
- [ ] add diagnostic warnings and possibly errors behind a feature flag for nightly users
    - [ ] add warning to put `#>[meta]` after `#[meta]`
- [ ] write more tests