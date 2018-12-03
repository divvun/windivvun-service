# Compile
1) Install Visual Studio with the C++ Compiler and the Windows 10 SDK enabled, or just the Windows build tools.
2) Install rust nightly for Windows https://github.com/rust-lang-nursery/rustup.rs/#other-installation-methods

Open up the x86_64 compilation Visual Studio command line and run `cargo build`.

# Install
- Run install.bat as Administrator
- Install dictionary files in C:\Program Files\SpellCheckTest\dicts
- If you want a log file in C:\Program Files\SpellCheckTest\ make the folder writable by all users (might not be necessary)

# Applications on Windows that use the spell checker
- Windows Mail client. Spell checking language can be selected in the "Options" tab when writing a mail.
- Skype??
- Slack but it doesn't seem to care about changing keyboard layouts

# Inno Setup Installers
Run `build_installers.bat` with dictionaries downloaded into `dict`.

