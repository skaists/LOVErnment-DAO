# Astra audit — restore the formatting gate

Base: `dca913306455002c2e5260540f9110b9c833715b`.
The HEAD workflow [run 32360589417](https://github.com/skaists/LOVErnment-DAO/actions/runs/32360589417)
successfully builds and tests the workspace, then fails `cargo fmt --all --check`.
The oldest of the five runs inspected, [run 29815927802](https://github.com/skaists/LOVErnment-DAO/actions/runs/29815927802),
has the same failed step. This is inherited formatting drift, not a new test failure.

Applied `cargo fmt --all` to the 13 Rust source/test files named by rustfmt.
Local `cargo fmt --all --check` and `git diff --check` now pass. No live PDS
tests were enabled and no records were written. Linux CI supplies the full
workspace build/test validation; compiling on the production box is unnecessary.

This lane changes formatting only. Governance, keys, behavior and dependencies
retain their existing scope and founder gates. Revert this commit to roll back.
