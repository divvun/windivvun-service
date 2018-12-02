SET ISCC="C:\Program Files (x86)\Inno Setup 5\iscc"
cd installer
%ISCC% installer_spell_checker.iss
%ISCC% installer_dictionary.iss
cd ..