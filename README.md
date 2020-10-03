# WiiUse-sys
NOTE: This is a fork!

## Linux
* Install WiiUse, available through the Arch Linux package `wiiuse`. If your distribution does not have a package for WiiUse, you may need to compile from source, see https://github.com/wiiuse/wiiuse#for-linux
* Turns out bluetooth was disabled on my machine via `rfkill`. All I had to do was run `sudo rfkill unblock all`. 

## Windows
* Install Visual Studio 2017. I could not get VS2019 to work at all.
* You may need to install LLVM for bindgen, available here: https://releases.llvm.org/download.html
* Download, compile, and install [WiiUse](https://github.com/wiiuse/wiiuse)
    * Remember to use cmake-gui to configure for your target platform and to use VS2017; x64 is not the default.
    * Open VS2017 as administrator when building `INSTALL.vcxproj`
* Add the following to your `PATH`:
    * `C:\Program Files (x86)\WiiUse\bin\`
* On Windows, I needed to use [WiiPair](https://github.com/jmandawg/wiipair) in order to connect to my Wii devices.

## Fixed
* Add type/function blacklist as per https://github.com/rust-lang/rust-bindgen/issues/1556
    * Also note: https://github.com/rust-lang/rust/issues/59154