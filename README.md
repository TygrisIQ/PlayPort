# PlayPort 🎮

> Use your Android phone as a wireless gamepad on Linux — no drivers, no dongles, no BS.

PlayPort is a client-server virtual gamepad system for Linux. The server runs on your machine and creates a virtual `uinput` controller. The Android app connects over your local network and streams button presses and analog stick input in real time.

Works with any application that reads standard gamepad input — emulators, Steam games, native Linux games.

---

## How It Works

```
[ Android App ] ──── TCP (binary protocol) ────► [ Linux Server ] ──► /dev/uinput ──► Game
                ◄─── UDP broadcast discovery ────
```

- **UDP broadcast** — the Android app auto-discovers the server on your LAN, no IP config needed
- **Persistent TCP connection** — a single socket stays open for the session, no per-packet overhead
- **Binary protocol** — input events are 2–4 bytes, not strings. Zero parsing overhead
- **uinput kernel driver** — the server registers as a real kernel-level gamepad, indistinguishable from hardware

---

## Requirements

- Linux distro with `uinput` support (most distros support it)
- Rust toolchain (`cargo`)
- Android device on the same LAN as the server
- [PlayPort Android Client](https://github.com/TygrisIQ/PlayPort_Client)

---

## Setup

### 1. Load the uinput module

```bash
sudo modprobe uinput
```

To verify it loaded:

```bash
lsmod | grep uinput
```

> **Note:** On some kernels `uinput` is compiled in (`CONFIG_INPUT_UINPUT=y`) rather than as a module. If `modprobe` fails but `/dev/uinput` exists, you're already good.

### 2. Set permissions

```bash
sudo chmod 666 /dev/uinput
```

Or add yourself to the `input` group permanently:

```bash
sudo usermod -aG input $USER
# log out and back in for this to take effect
```

### 3. Build and run the server

```bash
cargo build --release
./target/release/playport
```

By default the server listens on port `8007`. You can specify a different port:

```bash
./target/release/playport 9000
```

### 4. Connect the Android client

Install the [Android client](https://github.com/TygrisIQ/PlayPort_Client), open it, and tap **Search for Server**. It will auto-discover the server via UDP broadcast and connect automatically.

---

## Protocol

PlayPort uses a compact binary protocol over TCP.

| Type    | Bytes | Layout                                          |
|---------|-------|-------------------------------------------------|
| Button  | 2     | `[msg_type: u8] [button_id: u8]`                |
| Axis    | 4     | `[msg_type: u8] [axis_id: u8] [value: i16 LE]`  |

**Message types:**

| Value  | Meaning  |
|--------|----------|
| `0x01` | PRESS    |
| `0x02` | RELEASE  |
| `0x03` | AXIS     |

**Button IDs:**

| Value  | Button      |
|--------|-------------|
| `0x00` | A (South)   |
| `0x01` | B (East)    |
| `0x02` | X (North)   |
| `0x03` | Y (West)    |
| `0x04` | LB (TL)     |
| `0x05` | LB2 (TL2)   |
| `0x06` | RB (TR)     |
| `0x07` | RB2 (TR2)   |
| `0x08` | D-Pad Up    |
| `0x09` | D-Pad Down  |
| `0x0A` | D-Pad Left  |
| `0x0B` | D-Pad Right |
| `0x0C` | Select      |
| `0x0D` | Start       |

**Axis IDs:**

| Value  | Axis  |
|--------|-------|
| `0x00` | LS X  |
| `0x01` | LS Y  |
| `0x02` | RS X  |
| `0x03` | RS Y  |

Axis values are `i16` in little-endian, range `-32768` to `32767`.

---

## Ports Used

| Port   | Protocol | Purpose                        |
|--------|----------|--------------------------------|
| `8005` | UDP      | LAN server discovery           |
| `8007` | TCP      | Input event stream (default)   |

---

## Troubleshooting

**Server crashes immediately on startup**
→ `uinput` is not loaded. Run `sudo modprobe uinput` and try again.

**Android app can't find the server**
→ UDP broadcast is blocked on some networks (university/corporate WiFi). Make sure both devices are on the same network and broadcast is not firewalled.

**Device shows up but no input is registered**
→ Run `evtest` to verify events are reaching the kernel:
```bash
sudo evtest
# select the PlayPortDevice from the list
```

**Buttons get stuck (not releasing)**
→ This is a known edge case when the TCP connection drops mid-session. Reconnect the Android client to re-establish the session.

---

## License

MIT