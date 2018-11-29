@echo off
if not exist "C:\Program Files\SpellCheckTest\" mkdir "C:\Program Files\SpellCheckTest\"
if not exist "C:\Program Files\SpellCheckTest\dicts" mkdir "C:\Program Files\SpellCheckTest\dicts"
copy target\debug\winspellcheck.dll "C:\Program Files\SpellCheckTest\"
regedit.exe /s localmachine.reg
