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
// Device data
var device;

// DOM loaded
var loaded = false;

// DOM elements
var xSettingsBox, ySettingsBox;
var widthSettingsBox, heightSettingsBox;
var button1Type, button2Type, button3Type;
var button1Value, button2Value, button3Value;
var areaBox, areaText;
var configNameBox, prettyCheckbox, exportBtn;

const COMMMAND_ID_GENERATOR = idGenerator();

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
    socket.addEventListener('message', onCommandRes);
}

function onLoaded() {
    xSettingsBox = document.getElementById('xSettingsBox');
    ySettingsBox = document.getElementById('ySettingsBox');

    widthSettingsBox = document.getElementById('widthSettingsBox');
    heightSettingsBox = document.getElementById('heightSettingsBox');

    button1Type = document.getElementById('button1Type');
    button2Type = document.getElementById('button2Type');
    button3Type = document.getElementById('button3Type');

    button1Value = document.getElementById('button1Value');
    button2Value = document.getElementById('button2Value');
    button3Value = document.getElementById('button3Value');

    areaBox = document.getElementById('areaBox');
    areaText = document.getElementById('areaText');

    configNameBox = document.getElementById('configNameBox');
    prettyCheckbox = document.getElementById('prettyCheckbox');
    exportBtn = document.getElementById('exportBtn');

    loaded = true;

    exportBtn.addEventListener('click', exportConfig);

    if (currentConfig) updateDOM();
}

async function onConnect(e) {
    console.log('StoryTabletDriver configurator 1.0.0');

    // Keep updating status
    setInterval(updateStatus, 5000);
    updateStatus();

    let deviceRes = await sendCommand('GetDevice');
    device = deviceRes.data.device;
    console.debug(`Device: ${device.name}`);

    let configRes = await sendCommand('GetConfig');
    currentConfig = configRes.data.config;

    if (loaded) updateDOM();
}

async function updateStatus() {
    let res = await sendCommand('GetStatus');

    console.debug(`status: ${res.data.status}`);
    if (res.data.status !== 'Connected') {
        alert('Device not connected?!');
    }
}


var commandMap = new Map();
async function sendCommand(commandName, params = {}) {
    let id = COMMMAND_ID_GENERATOR.next().value;

    socket.send(JSON.stringify({ id: id, data: { type: commandName, ...params } }));

    return new Promise((resolver, reject) => {
        let taskId = setTimeout(() => {
            if (commandMap.has(id)) reject('Timeout');
        }, 5000);

        commandMap.set(id, (...args) => {
            clearTimeout(taskId);
            resolver(...args);
        });
    });
}

function onCommandRes(e) {
    try {
        let res = JSON.parse(e.data);
        
        if (typeof(res.id) != 'number' || !res.data) {
            throw 'Invalid command';
        }

        let resolver = commandMap.get(res.id);
        commandMap.delete(res.id);
        if (resolver) resolver(res);
    } catch (err) {
        console.error(`Invalid command. raw: ${e.data}`);
    }
}

function updateDOM() {
    xSettingsBox.value = currentConfig.mapping.x;
    ySettingsBox.value = currentConfig.mapping.y;

    widthSettingsBox.value = currentConfig.mapping.width;
    heightSettingsBox.value = currentConfig.mapping.height;

    button1Type.value = currentConfig.buttons[0].mode;
    button2Type.value = currentConfig.buttons[1].mode;
    button3Type.value = currentConfig.buttons[2].mode;

    if (currentConfig.buttons[0].mode != 'Disabled') {
        button1Value.disabled = false;
        button1Value.value = currentConfig.buttons[0].button || currentConfig.buttons[0].keys.join(' + ');
    } else {
        button1Value.disabled = true;
        button1Value.value = '';
    }
    if (currentConfig.buttons[1].mode != 'Disabled') {
        button1Value.disabled = false;
        button2Value.value = currentConfig.buttons[1].button || currentConfig.buttons[1].keys.join(' + ');
    } else {
        button2Value.disabled = true;
        button2Value.value = '';
    }
    if (currentConfig.buttons[2].mode != 'Disabled') {
        button1Value.disabled = false;
        button3Value.value = currentConfig.buttons[2].button || currentConfig.buttons[2].keys.join(' + ');
    } else {
        button3Value.disabled = true;
        button3Value.value = '';
    }

    syncAreaBox();
}

function syncConfig() {
    currentConfig.mapping.x = Math.min(Math.max(xSettingsBox.value, 0), device.area.x);
    currentConfig.mapping.y = Math.min(Math.max(ySettingsBox.value, 0), device.area.y);

    currentConfig.mapping.width = widthSettingsBox.value;
    currentConfig.mapping.height = heightSettingsBox.value;
}

function syncAreaBox() {
    areaBox.style.left = `${(currentConfig.mapping.x / device.area.width) * 45}vw`;
    areaBox.style.top = `${(currentConfig.mapping.y / device.area.height) * 28.125}vw`;

    areaBox.style.width = `${(currentConfig.mapping.width / device.area.width) * 45}vw`;
    areaBox.style.height = `${(currentConfig.mapping.height / device.area.height) * 28.125}vw`;

    areaText.innerText = `${(currentConfig.mapping.width / currentConfig.mapping.height).toFixed(4)} : 1`;
}

function exportConfig() {
    syncConfig();
    updateDOM();

    let tmp = document.createElement('a');
    tmp.download = configNameBox.value || 'config.json';
    tmp.href = URL.createObjectURL(new Blob([JSON.stringify(currentConfig, null, prettyCheckbox.checked ? 4 : 0)], {type : 'application/json'}));
    tmp.click();
}

function* idGenerator() {
    var id = 0;
    while(true)
        yield id++;
}

// run main
main();

})();