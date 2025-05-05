# Contributing to SpringKeys

Thank you for your interest in contributing to SpringKeys! This document outlines the process for contributing to the project and some guidelines to follow.

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md). We expect all contributors to adhere to this code when interacting with the project.

## Getting Started

1. **Fork the repository**: Start by forking the repository on GitHub.

2. **Clone your fork**: Clone your fork to your local machine.
   ```bash
   git clone https://github.com/your-username/spring-keys.git
   cd spring-keys
   ```

3. **Set up remotes**: Add the original repository as an upstream remote.
   ```bash
   git remote add upstream https://github.com/SpringKeyers/spring-keys.git
   ```

4. **Create a branch**: Create a branch for your feature or bug fix.
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Process

1. **Keep your fork up to date**: Before you start working, make sure your fork is up to date with the upstream repository.
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   git push origin main
   ```

2. **Branch naming convention**:
   - `feature/your-feature-name` for new features
   - `bugfix/issue-description` for bug fixes
   - `docs/what-you-changed` for documentation changes
   - `refactor/what-you-refactored` for code refactoring

3. **Commit messages**: Write clear, concise commit messages that explain what changes were made and why. Follow the convention:
   ```
   [Component] Brief description of change

   More detailed explanation if necessary.

   Closes #123 (if this commit closes an issue)
   ```

4. **Code style**: Follow the Rust style guide. Run `cargo fmt` before committing to ensure your code is properly formatted.

5. **Testing**: Add tests for new features and ensure all tests pass before submitting a pull request.
   ```bash
   cargo test
   ```

6. **Linting**: Run clippy to catch common mistakes and improve your code.
   ```bash
   cargo clippy
   ```

## Pull Request Process

1. **Create a pull request**: Push your changes to your fork and create a pull request against the main repository.

2. **PR description**: Include a clear description of what changes were made and why. Reference any related issues.

3. **Review process**: Your PR will be reviewed by maintainers. Address any requested changes promptly.

4. **CI checks**: Make sure all CI checks pass before requesting a review.

5. **Approval and merge**: Once your PR is approved, it will be merged by a maintainer.

## Feature Requests and Bug Reports

- Use the GitHub issue tracker to submit feature requests and bug reports.
- Search for existing issues before creating a new one.
- Provide as much detail as possible when creating an issue.

## Documentation

- Update documentation when adding or modifying features.
- Ensure code is well-documented with comments.
- Consider adding examples for new features.

## Testing

- Add unit tests for all new features.
- Update existing tests when modifying functionality.
- Aim for high test coverage.

## Questions?

If you have any questions, feel free to open an issue or contact the maintainers.

Thank you for contributing to SpringKeys! 