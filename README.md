Simulates a ransomware attack and reports any user who attempts to "pay the ransom" to Knowbe4 KMSAT

# Requirements
- `cargo`
- an active subcription to Knowbe4

# Setup
with `cargo`  installed, Set the following environment variables:

- `COMPANY_DOMAIN` : this would be the email domain used by the company.
- `KNOWBE4_SERVER` : the appropriate server address for Knowbe4, ([See documentation](https://developer.knowbe4.com/rest/userEvents#tag/Base-URL))
- `KNOWBE4_TOKEN` : an api token from your KMSAT Console ([See documentation](https://developer.knowbe4.com/rest/userEvents#tag/Authentication))

the program will fail to compile unless these environment variables are present in the terminal session at compile time.

## to compile

run `cargo build --release`. This will produce a single PE. usually this PE is found at: `<project dir>/target/release/`. In this directory you will find the compiled binary (for example, on windows you will be looking for `simple_ransomware.exe`) 
