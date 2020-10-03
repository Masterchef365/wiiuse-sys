# Wiiuse-sys
NOTE: This is a fork!

## Compiling for Windows
Here is what I learned while attemping to compile this for Windows
* Install Visual Studio 2017. I could not get VS2019 to work at all.
* You may need to install LLVM for bindgen, available here: https://releases.llvm.org/download.html
* Download, compile, and install [WiiUse](https://github.com/wiiuse/wiiuse)
    * Remember to use cmake-gui to configure for your target platform and to use VS2017; x64 is not the default.
    * Open VS2017 as administrator when building `INSTALL.vcxproj`
* Set the following environment variable in powershell before compiling:
    * `$Env:BINDGEN_EXTRA_CLANG_ARGS='-I"C:\Program Files (x86)\WiiUse\include"'`
    * (cmd) `set "BINDGEN_EXTRA_CLANG_ARGS=-IC:\Program Files (x86)\WiiUse\include\"`
* For now, I copy the following to the root of the project I'm using (there are paths I can set but for right now I just want it to work...)
    * `C:\Program Files (x86)\WiiUse\bin\wiiuse.dll`
    * `C:\Program Files (x86)\WiiUse\lib\wiiuse.lib`

## Note
On Windows, I needed to use [WiiPair](https://github.com/jmandawg/wiipair) in order to connect to my Wii devices.

## Fixed
* Add type/function blacklist as per https://github.com/rust-lang/rust-bindgen/issues/1556
    * Also note: https://github.com/rust-lang/rust/issues/59154

## TODO
* Add `-I"C:\Program Files (x86)\WiiUse\include"` to build.rs
* Figure out how not to copy `wiiuse.{dll,lib}` into the project root