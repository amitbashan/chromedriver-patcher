# chromedriver-patcher
Patch `cdc_` signature in `chromedriver` to avoid detection of bot automation

## Usage
`chromedriver-patcher [-s] <chromedriver path> [patched chromedriver path]` 

If the path to the patched `chromedriver` is not given, the original `chromedriver` would be patched. \
The argument `-s` specifies whether to code-sign the patched executable.
