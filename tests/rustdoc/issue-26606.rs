// aux-build:issue-26606-macro.rs
// ignore-cross-compile
// build-aux-docs

#![crate_name="issue_26606"]

// @has issue_26606_macro/macro.make_item.html
#[macro_use]
extern crate issue_26606_macro;

// @has issue_26606/constant.FOO.html
// @has - '//a[@href="../src/issue_26606/issue-26606.rs.html#13"]' 'source'
make_item!(FOO);
