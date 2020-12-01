#define MyAppName "OpenIL2 (Debug Build)"
#define MyAppVersion "0.1.0"
#define MyAppExeName "openil2.exe"

[Setup]
AppId={{0C8CF246-19D7-4DDD-8836-9242E2EBDBA2}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputBaseFilename=openil2-installer-debug
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\launcher\target\debug\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\launcher\target\debug\openil2.pdb"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_jni\target\debug\physfs_jni.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_jni\target\debug\deps\physfs_jni.pdb"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_java\physfs.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_java\rts.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_java\build\libs\physfs_java.jar"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon
