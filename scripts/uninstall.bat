@echo off
title Story Tablet driver uninstaller

set INSTALL_PATH=%appdata%\story-tablet-driver
set EXE_NAME=story-tablet-driver.exe
set CONFIG_NAME=config.json

if not exist %INSTALL_PATH% (
    echo Driver not installed
) else (
    echo Uninstalling..
    tasklist | find /i "%EXE_NAME%" && taskkill /f /im "%EXE_NAME%"
	
	rmdir /q /s %INSTALL_PATH%
	reg delete HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run /v StoryTabletDriver /f
	
	echo Driver uninstalled
)
pause
exit