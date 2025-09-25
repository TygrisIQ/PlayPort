# PlayPort 
---
## A client-server virtual game controller application for linux built on uinput




### build the server application with cargo 

'cargo build --release'

### You will need the [Android client](https://github.com/TygrisIQ/PlayPort_Client)

> **Note: 'uinput' must be loaded**
> The server uses linux uinput. if the 'uinput' kernel module is not loaded, the app will **crash at startup**


Load the module:

```bash
sudo modprobe uinput

Verify it is loaded:

```bash 
lsmod | grep uinput



