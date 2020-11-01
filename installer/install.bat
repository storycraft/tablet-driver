@echo off
title Story Tablet driver installer

set INSTALL_PATH=%appdata%\story-tablet-driver
set EXE_NAME=story-tablet-driver.exe
set CONFIG_NAME=config.json

if not exist %INSTALL_PATH% mkdir %INSTALL_PATH%
cd %INSTALL_PATH%

if not exist %INSTALL_PATH% (
    echo Cannot find target program
    pause
    exit
) else (
    tasklist | find /i "%EXE_NAME%" && taskkill /f /im "%EXE_NAME%"
    copy /B %EXE_NAME% %INSTALL_PATH%
    copy /B %CONFIG_NAME% %INSTALL_PATH%
)

echo Configuring..

reg add HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run /v StoryTabletDriver /f /d "\"%INSTALL_PATH%\%EXE_NAME%\" %INSTALL_PATH%\%CONFIG_NAME%"

echo Driver installed

start %INSTALL_PATH%\%EXE_NAME% %CONFIG_NAME% && echo Driver started

pause
exit