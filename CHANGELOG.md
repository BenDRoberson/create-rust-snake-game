## [1.0.3] - 2025-09-27

### 🚀 Features

- Add high score tracking
- Adding pre-commit hooks for linting/formatting

### 🐛 Bug Fixes

- Ci bug where certain jobs won't run on manual kickoff
- Don't build images on manual ci run

### ⚙️ Miscellaneous Tasks

- Only run quick tests on pr
- Only run code coverage and security audit on merge
## [1.0.2] - 2025-09-27

### ⚙️ Miscellaneous Tasks

- Commit CHANGLOG.md on new release
## [1.0.1] - 2025-09-27

### 🚀 Features

- Initial changelog version

### 📚 Documentation

- README update about robot usage

### ⚙️ Miscellaneous Tasks

- Remove depenency checks because thats overkill
- Add workflow_dispatch trigger to CI workflow
- Enable all CI jobs on manual workflow dispatch
- Update release.yml to install git-cliff correctly
- Fix git-cliff download in release
## [1.0.0] - 2025-09-16

### 🚀 Features

- Initial commit with hello-world
- First working version
- Add game over screen and fix a few edge cases

### 📚 Documentation

- Update readme.md no roboting here
- Update docs

### 🧪 Testing

- Insane amount of tests from the robot
- Fix tests in CI

### ⚙️ Miscellaneous Tasks

- Adding some comments for my future self
- Add github workflows from the robot
- Update version on upload-artifact - thanks for nothing cursor
- Add some audio drivers for linux per LLM
- More small changes for macos CI
- Fix release workflow
- Fix release workflow again - darn audio drivers on linux
- Match dependencies by platform to ci workflow
- Fix the cargo.toml version update, need to check out main
- Fix windows build powershell issue
- Fix windows again, force it to use bash bc i hate powershell
