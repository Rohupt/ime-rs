$scriptDir = Split-Path -Path $MyInvocation.MyCommand.Definition -Parent
Push-Location $scriptDir/../rust
cargo +nightly build
cbindgen --crate composition_processor --output ../cpp/SampleIME/cbindgen/composition_processor.h
cbindgen --crate dictionary_parser --output ../cpp/SampleIME/cbindgen/dictionary_parser.h
cbindgen --crate itf_components --output ../cpp/SampleIME/cbindgen/itf_components.h
cbindgen --crate globals --output ../cpp/SampleIME/cbindgen/globals.h
cbindgen --crate ime --output ../cpp/SampleIME/cbindgen/ime.h
cbindgen --crate numberkey_windows --output ../cpp/SampleIME/cbindgen/numberkey_windows.h
cbindgen --crate ruststringrange --output ../cpp/SampleIME/cbindgen/ruststringrange.h
Pop-Location
