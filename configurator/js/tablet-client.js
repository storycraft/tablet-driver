/*
 * Created on Mon Nov 09 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */
'use strict';

(function() {

globalThis.TabletClient = class TabletClient {

    // addr: string
    // port: number
    // _socket: WebSocket
    // _connected: boolean
    // _idGenerator: function*<number>
    // _commandMap: Map<number, function>
    
    // _device: object
    // _currentConfig: object

    constructor(addr, port) {
        this.addr = addr;
        this.port = port;
        
        this._socket = null;
        this._connected = false;
        this._idGenerator = null;
        this._device = null;
        this._currentConfig = null;
        this._commandMap = new Map();
    }

    get socket() {
        return this._socket;
    }

    get nextCommandId() {
        return this._idGenerator.next().value;
    }

    get connected() {
        return this._connected;
    }

    async initalize() {
        if (this._connected) {
            throw 'Client already connected!';
        }

        try {
            await this._connectSocket();
        } catch(e) {
            throw `Couldn't connect to server: ${e}`;
        }

        let deviceRes = await this.sendCommand('GetDevice');
        this._device = deviceRes.data.device;
        console.debug(`Device: ${deviceRes.data.device.name}`);
    
        let configRes = await this.sendCommand('GetConfig');
        this._currentConfig = configRes.data.config;
        console.debug(`Config: ${configRes.data.config}`);
    }

    disconnect() {
        if (!this._connected) {
            throw 'Client is not connected';
        }
        this._connected = false;
        this._socket.close();
    }

    _connectSocket() {
        this._socket = new WebSocket(`ws://${this.addr}:${this.port}`);
        this._connected = false;
        this._idGenerator = idGenerator();
        this._commandMap.clear();

        this._socket.addEventListener('open', this.onConnect.bind(this));
        this._socket.addEventListener('error', this.onError.bind(this));
        this._socket.addEventListener('close', this.onClose.bind(this));
        this._socket.addEventListener('message', this._onCommandRes.bind(this));

        return new Promise((resolve, reject) => {
            this._socket.addEventListener('open', () => {
                this._connected = true;
                resolve();
            });
            this._socket.addEventListener('error', () => {
                reject();
            });
        });
        
    }

    async sendCommand(commandName, params = {}) {
        let id = this.nextCommandId;
        this._socket.send(JSON.stringify({ id: id, data: { type: commandName, ...params } }));
    
        return new Promise((resolver, reject) => {
            let taskId = setTimeout(() => {
                if (this._commandMap.has(id)) reject('Timeout');
            }, 5000);
    
            this._commandMap.set(id, (...args) => {
                clearTimeout(taskId);
                resolver(...args);
            });
        });
    }

    _onCommandRes(e) {
        try {
            let res = JSON.parse(e.data);
            
            if (typeof(res.id) != 'number' || !res.data) {
                throw 'Invalid command';
            }
    
            let resolver = this._commandMap.get(res.id);
            this._commandMap.delete(res.id);

            this._handleCommand(res);
            if (resolver) resolver(res);
        } catch (err) {
            console.error(`Invalid command. err: ${err} raw: ${e.data}`);
        }
    }

    _handleCommand(res) {

    }

    onConnect() {

    }

    onClose() {

    }

    onError() {

    }

}

function* idGenerator() {
    var id = 0;
    while(true)
        yield id++;
}

})();