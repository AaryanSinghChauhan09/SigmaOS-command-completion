// Integration test for SigmaOS-command-completion
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use command_completion as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
