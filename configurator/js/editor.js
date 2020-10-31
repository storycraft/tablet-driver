/*
 * Created on Tue Oct 27 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */
'use strict';

(function() {

// tablet WebSocket
var socket;
// Editing config
var currentConfig;

// DOM loaded
var loaded = false;

// DOM elements
var xSettingsBox, ySettingsBox;
var widthSettingsBox, heightSettingsBox;
var areaBox, areaText;
var configNameBox, prettyCheckbox, exportBtn;

function main() {
    window.addEventListener('load', onLoaded);
    socket = new WebSocket('ws://127.0.0.1:55472');

    socket.addEventListener('open', onConnect);
    socket.addEventListener('error', () => {
        alert('Cannot connect to Tablet driver server :(');
    });
    socket.addEventListener('close', () => {
        alert('Tablet driver server closed?!');
    });
    socket.addEventListener('message', onMessage);

    // Keep updating status
    setInterval(() => sendCommand('GetStatus'), 5000);
}

function onLoaded() {
    xSettingsBox = document.getElementById('xSettingsBox');
    ySettingsBox = document.getElementById('ySettingsBox');

    widthSettingsBox = document.getElementById('widthSettingsBox');
    heightSettingsBox = document.getElementById('heightSettingsBox');

    areaBox = document.getElementById('areaBox');
    areaText = document.getElementById('areaText');

    configNameBox = document.getElementById('configNameBox');
    prettyCheckbox = document.getElementById('prettyCheckbox');
    exportBtn = document.getElementById('exportBtn');

    loaded = true;

    exportBtn.addEventListener('click', exportConfig);

    if (currentConfig) updateDOM();
}

function onMessage(e) {
    try {
        let res = JSON.parse(e.data);
        
        if (!res.id) {
            throw 'Invalid command';
        }

        console.debug(`Res command received: ${e.data}`);

        switch (res.id) {

            case 'GetStatus': {
                if (res.status !== 'Connected') alert('Tablet is not connected');
                
                console.log(`status: ${res.status}`);
                break;
            }

            case 'GetConfig': {
                currentConfig = res.config;

                if (loaded) updateDOM();
                break;
            }

            default: break;
        }
    } catch (err) {
        console.error(`Invalid command. raw: ${e.data}`);
    }
}

function onConnect(e) {
    console.log('StoryTabletDriver configurator 1.0.0');

    sendCommand('GetStatus');
    sendCommand('GetConfig');
}



function sendCommand(commandName, params = {}) {
    socket.send(JSON.stringify({ id: commandName, ...params }))
}

function updateDOM() {
    xSettingsBox.value = currentConfig.mapping.x;
    ySettingsBox.value = currentConfig.mapping.y;

    widthSettingsBox.value = currentConfig.mapping.width;
    heightSettingsBox.value = currentConfig.mapping.height;

    syncAreaBox();
}

function syncConfig() {
    currentConfig.mapping.x = Math.min(Math.max(xSettingsBox.value, 0), 15200);
    currentConfig.mapping.y = Math.min(Math.max(ySettingsBox.value, 0), 9500);

    currentConfig.mapping.width = widthSettingsBox.value;
    currentConfig.mapping.height = heightSettingsBox.value;
}

function syncAreaBox() {
    areaBox.style.left = `${(currentConfig.mapping.x / 15200) * 45}vw`;
    areaBox.style.top = `${(currentConfig.mapping.y / 9500) * 28.125}vw`;

    areaBox.style.width = `${(currentConfig.mapping.width / 15200) * 45}vw`;
    areaBox.style.height = `${(currentConfig.mapping.height / 9500) * 28.125}vw`;

    areaText.innerText = `1 : ${(currentConfig.mapping.width / currentConfig.mapping.height).toFixed(4)}`;
}

function exportConfig() {
    syncConfig();
    updateDOM();

    let tmp = document.createElement('a');
    tmp.download = configNameBox.value || 'config.json';
    tmp.href = URL.createObjectURL(new Blob([JSON.stringify(currentConfig, null, prettyCheckbox.checked ? 4 : 0)], {type : 'application/json'}));
    tmp.click();
}

// run main
main();

})();