#define MyAppName "OpenIL2"
#define MyAppVersion GetEnv('RELEASE_VERSION')
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
AppendDefaultDirName=no
DirExistsWarning=no
DisableDirPage=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[InstallDelete]
Type: filesandordirs; Name: "{app}\bin"
Type: filesandordirs; Name: "{app}\lib"

[Files]
Source: "..\launcher\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion; BeforeInstall: SetProgressMax(2)
Source: "..\launcher\target\release\openil2.pdb"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\repacker\target\release\repacker.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\repacker\target\release\repacker.pdb"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_jni\target\release\physfs_jni.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_jni\target\release\deps\physfs_jni.pdb"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_rts\target\release\physfs_rts.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_rts\target\release\deps\physfs_rts.pdb"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_java\physfs.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_java\rts.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\physfs_java\build\libs\physfs_java.jar"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\classload_agent\build\libs\classload_agent.jar"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\classload_agent\build\executable\class-transformer.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\classload_agent\build\executable\class-transformer.pdb"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\jre\*.*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs
Source: "..\sfs_db.sqlite"; DestDir: "{app}"; Flags: ignoreversion
; fb_3do10.SFS - No identified entries as yet so we need to provide a dummy .zip with at least one entry
Source: "fb_3do10p.zip"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Redist\MSVC\v142\vcredist_x64.exe"; DestDir: {tmp}; Flags: deleteafterinstall

[Run]
Filename: "{tmp}\vcredist_x64.exe"; Parameters: "/install /quiet /norestart"; StatusMsg: "Installing Visual C++ 2019 Redistributable"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do.SFS"; StatusMsg: "Repacking fb_3do.SFS"; AfterInstall: UpdateProgress(51)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do01.SFS"; StatusMsg: "Repacking fb_3do01.SFS"; AfterInstall: UpdateProgress(56)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do02.SFS"; StatusMsg: "Repacking fb_3do02.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do03.SFS"; StatusMsg: "Repacking fb_3do03.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do04.SFS"; StatusMsg: "Repacking fb_3do04.SFS"; AfterInstall: UpdateProgress(57)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do05.SFS"; StatusMsg: "Repacking fb_3do05.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do06.SFS"; StatusMsg: "Repacking fb_3do06.SFS"; AfterInstall: UpdateProgress(59)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do07.SFS"; StatusMsg: "Repacking fb_3do07.SFS"; AfterInstall: UpdateProgress(60)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do08.SFS"; StatusMsg: "Repacking fb_3do08.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do08p.SFS"; StatusMsg: "Repacking fb_3do08p.SFS"; AfterInstall: UpdateProgress(66)
; fb_3do10.SFS - No identified entries as yet so we can't repack it
; Filename: "{app}\repacker.exe";; Parameters: """{app}"" ""{tmp}"" fb_3do10.SFS"; StatusMsg: "Repacking fb_3do10.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do11.SFS"; StatusMsg: "Repacking fb_3do11.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do11p.SFS"; StatusMsg: "Repacking fb_3do11p.SFS"; AfterInstall: UpdateProgress(67)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do12.SFS"; StatusMsg: "Repacking fb_3do12.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do12p.SFS"; StatusMsg: "Repacking fb_3do12p.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do13.SFS"; StatusMsg: "Repacking fb_3do13.SFS"; AfterInstall: UpdateProgress(68)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do13p.SFS"; StatusMsg: "Repacking fb_3do13p.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do14.SFS"; StatusMsg: "Repacking fb_3do14p.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do14p.SFS"; StatusMsg: "Repacking fb_3do14p.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do15.SFS"; StatusMsg: "Repacking fb_3do15.SFS"; AfterInstall: UpdateProgress(69)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do16.SFS"; StatusMsg: "Repacking fb_3do16.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do16p.SFS"; StatusMsg: "Repacking fb_3do16p.SFS"; AfterInstall: UpdateProgress(70)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do17.SFS"; StatusMsg: "Repacking fb_3do17.SFS"; AfterInstall: UpdateProgress(71)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do18.SFS"; StatusMsg: "Repacking fb_3do18.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do19.SFS"; StatusMsg: "Repacking fb_3do19.SFS"; AfterInstall: UpdateProgress(73)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do20.SFS"; StatusMsg: "Repacking fb_3do20.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do21.SFS"; StatusMsg: "Repacking fb_3do21.SFS"; AfterInstall: UpdateProgress(75)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do22.SFS"; StatusMsg: "Repacking fb_3do22.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do23.SFS"; StatusMsg: "Repacking fb_3do23.SFS"; AfterInstall: UpdateProgress(80)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do24.SFS"; StatusMsg: "Repacking fb_3do24.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do25.SFS"; StatusMsg: "Repacking fb_3do25.SFS"; AfterInstall: UpdateProgress(81)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do26.SFS"; StatusMsg: "Repacking fb_3do26.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do27.SFS"; StatusMsg: "Repacking fb_3do27.SFS"; AfterInstall: UpdateProgress(83)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do28.SFS"; StatusMsg: "Repacking fb_3do28.SFS"; AfterInstall: UpdateProgress(84)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do29.SFS"; StatusMsg: "Repacking fb_3do29.SFS"; AfterInstall: UpdateProgress(86)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do30.SFS"; StatusMsg: "Repacking fb_3do30.SFS"; AfterInstall: UpdateProgress(89)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do31.SFS"; StatusMsg: "Repacking fb_3do31.SFS"; AfterInstall: UpdateProgress(90)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_3do32.SFS"; StatusMsg: "Repacking fb_3do32.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps.SFS"; StatusMsg: "Repacking fb_maps.SFS"; AfterInstall: UpdateProgress(91)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps01.SFS"; StatusMsg: "Repacking fb_maps01.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps02.SFS"; StatusMsg: "Repacking fb_maps02.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps03.SFS"; StatusMsg: "Repacking fb_maps03.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps04.SFS"; StatusMsg: "Repacking fb_maps04.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps05.SFS"; StatusMsg: "Repacking fb_maps05.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps06.SFS"; StatusMsg: "Repacking fb_maps06.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps07p.SFS"; StatusMsg: "Repacking fb_maps07p.SFS"; AfterInstall: UpdateProgress(92)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps08p.SFS"; StatusMsg: "Repacking fb_maps08p.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps09p.SFS"; StatusMsg: "Repacking fb_maps09p.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps10.SFS"; StatusMsg: "Repacking fb_maps10.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps10p.SFS"; StatusMsg: "Repacking fb_maps10p.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps11.SFS"; StatusMsg: "Repacking fb_maps11.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps12.SFS"; StatusMsg: "Repacking fb_maps12.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps12p.SFS"; StatusMsg: "Repacking fb_maps12p.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps13.SFS"; StatusMsg: "Repacking fb_maps13.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps14.SFS"; StatusMsg: "Repacking fb_maps14.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps15.SFS"; StatusMsg: "Repacking fb_maps15.SFS"; AfterInstall: UpdateProgress(92)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps16.SFS"; StatusMsg: "Repacking fb_maps16.SFS"; AfterInstall: UpdateProgress(93)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps17.SFS"; StatusMsg: "Repacking fb_maps17.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps18.SFS"; StatusMsg: "Repacking fb_maps18.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps19.SFS"; StatusMsg: "Repacking fb_maps19.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps21.SFS"; StatusMsg: "Repacking fb_maps21.SFS"; AfterInstall: UpdateProgress(94)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps22.SFS"; StatusMsg: "Repacking fb_maps22.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_maps23.SFS"; StatusMsg: "Repacking fb_maps23.SFS"; AfterInstall: UpdateProgress(97)
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" fb_sounds.SFS"; StatusMsg: "Repacking fb_sounds.SFS"
Filename: "{app}\repacker.exe"; Parameters: """{app}"" ""{tmp}"" files.SFS"; StatusMsg: "Repacking files.SFS"

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Code]

procedure SetProgressMax(Ratio: Integer);
begin
  WizardForm.ProgressGauge.Max := WizardForm.ProgressGauge.Max * Ratio;
end;

procedure UpdateProgress(Position: Integer);
begin
  WizardForm.ProgressGauge.Position := Position * WizardForm.ProgressGauge.Max div 100;
end;
