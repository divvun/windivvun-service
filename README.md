# Compile
1) Install Visual Studio with the C++ Compiler and the Windows 10 SDK enabled, or just the Windows build tools.
2) Install rust nightly for Windows https://github.com/rust-lang-nursery/rustup.rs/#other-installation-methods

Open up the x86_64 compilation Visual Studio command line and run `cargo build`.

# Install
- Run install.bat as Administrator
- Install dictionary files in C:\Program Files\SpellCheckTest\dicts
- If you want a log file in C:\Program Files\SpellCheckTest\ make the folder writable by all users (might not be necessary)

# Bugs
- Right now the spell checker DLL gets unloaded shortly after loading. To have the spell checker be available to applications you have to open the Windows Settings page, and go to the options of a keyboard. At that point the DLL is loaded for exactly 6 seconds! Now quickly open up an application that uses the spell checker.
- Applications don't seem to instantiate the spell checker all the time?

# Applications on Windows that use the spell checker
- Windows Mail client. Needs some restarting sometimes with the correct keyboard layout active to actually use the correct language spell checker.
- Skype??
- Slack but it doesn't seem to care about changing keyboard layouts
