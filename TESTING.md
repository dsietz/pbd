### Testing Strategy and Standards

Our project follows the [Test Driven Development](https://en.wikipedia.org/wiki/Test-driven_development) approach. This means that all tests are written __prior__ to the development of the working code. Our goal is to have a 90% or high code coverage whenever released to the `Master` branch.

#### Standards
- Unit tests are located in local directory in the nested `tests` module
> Example
```
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_myfunc_bad_result() {
       assert!(true);
   }
}
```
- All tests should have names that describe what they are testing (e.g.: new_from_string_good_result)
- Tests should include both the positive and negative scenarios
- Test should cover exceptions and how they are handled
- There should be tests that represent how the users will use the crate's functionalitiy  