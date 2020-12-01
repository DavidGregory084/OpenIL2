#define MyAppName "OpenIL2"
#define MyAppVersion "0.1.0"
#define MyAppExeName "openil2.exe"

[Setup]
AppId={{39EF55F9-0664-43CC-A3A2-7780264F1184}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputBaseFilename=openil2-installer
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\launcher\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\launcher\target\release\openil2.pdb"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_jni\target\release\physfs_jni.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_jni\target\release\deps\physfs_jni.pdb"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_java\physfs.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_java\rts.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_java\build\libs\physfs_java.jar"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon
