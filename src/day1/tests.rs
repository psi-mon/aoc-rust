// write a simple test which tests the sum of 2 numbers

use crate::day1::solution::solve;
use assert_fs::{prelude::* };

#[test]
fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("1\n2\n3\n\n5\n3")?;

    // Run the solve() function on the temporary file
    let result = solve(file.path().to_string_lossy().to_string());

    // Check that the result matches the expected output
    assert_eq!(result, Ok("8".to_string()));

    Ok(())
}
