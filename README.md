# Wiiuse-sys
NOTE: This is a fork!

## Compiling for Windows
Here is what I learned while attemping to compile this for Windows
* Install Visual Studio 2017. I could not get VS2019 to work at all.
* You may need to install LLVM for bindgen, available here: https://releases.llvm.org/download.html
* Download, compile, and install [WiiUse](https://github.com/wiiuse/wiiuse)
    * Invoke `cmake` with `-DCMAKE_BUILD_TYPE=Release`
    * Open VS2017 as administrator when building `INSTALL.vcxproj`
* Set the following environment variable in powershell before compiling:
    * `$Env:BINDGEN_EXTRA_CLANG_ARGS='-I"C:\Program Files (x86)\WiiUse\include"'`

## Fixed
* Add type/function blacklist as per https://github.com/rust-lang/rust-bindgen/issues/1556
    * Also note: https://github.com/rust-lang/rust/issues/59154
* `$Env:BINDGEN_EXTRA_CLANG_ARGS='-I"C:\Program Files (x86)\WiiUse\include"'` (but in build.rs)