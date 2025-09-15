Simulates a ransomware attack and reports any user who attempts to "pay the ransom" to Knowbe4 KMSAT

# Requirements
- `cargo`
- `dioxus-cli`
- an active subcription to Knowbe4

# Setup
with `cargo` and `dioxus-cli` installed, Set the following environment variables:

- `COMPANY_DOMAIN` : this would be the email domain used by the company.
- `KNOWBE4_SERVER` : the appropriate server address for Knowbe4, ([See documentation](https://developer.knowbe4.com/rest/userEvents#tag/Base-URL))
- `KNOWBE4_TOKEN` : an api token from your KMSAT Console ([See documentation](https://developer.knowbe4.com/rest/userEvents#tag/Authentication))

the program will fail to compile unless these environment variables are present in the terminal session at compile time.

## to compile

run `dx bundle --platform desktop` this will compile and create an installer for the simulation. you can supply the installer the `/s` flag to install silently.
