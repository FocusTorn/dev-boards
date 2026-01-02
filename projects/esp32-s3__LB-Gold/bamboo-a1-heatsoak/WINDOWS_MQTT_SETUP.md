# Windows MQTT Broker Setup Guide

This guide helps you set up an MQTT broker on Windows to test the heatsoak monitor before your Raspberry Pi with Mosquitto is ready.

## Option 1: Mosquitto for Windows (Recommended)

**Best choice** - Same broker as Raspberry Pi, ensures compatibility.

### Installation

1. **Download Mosquitto for Windows** (Direct Download - Recommended):
   - Go to: https://mosquitto.org/download/
   - Click on "Windows Installer" link
   - Download the installer (e.g., `mosquitto-2.0.18-install-windows-x64.exe`)
   - **Note**: Chocolatey package may not be available - use direct download instead

2. **Install**:
   - Run the downloaded installer
   - Follow the installation wizard
   - **Important**: During installation, check "Install as Windows Service" (recommended)
   - Default port: 1883
   - Installation location: Usually `C:\Program Files\mosquitto\`

3. **Alternative: Manual Installation** (if installer doesn't work):
   - Download the ZIP version from the same page
   - Extract to a folder (e.g., `C:\mosquitto\`)
   - Add `C:\mosquitto\` to your PATH environment variable
   - Create a config file: `C:\mosquitto\mosquitto.conf` with:
     ```
     listener 1883
     allow_anonymous true
     ```

3. **Verify Installation**:
   ```powershell
   # Check if Mosquitto service is installed
   Get-Service mosquitto
   
   # Check if port 1883 is listening
   netstat -an | findstr 1883
   
   # Test mosquitto command (should show help if installed correctly)
   mosquitto --help
   ```

4. **Start Mosquitto**:
   ```powershell
   # If installed as service (recommended)
   Start-Service mosquitto
   
   # Check service status
   Get-Service mosquitto
   
   # If not installed as service, run manually:
   # Navigate to installation directory
   cd "C:\Program Files\mosquitto"
   # Run mosquitto
   .\mosquitto.exe -c mosquitto.conf
   ```

5. **Troubleshooting Installation**:
   - **If installer fails**: Try running as Administrator
   - **If service won't start**: Check Windows Event Viewer for errors
   - **If mosquitto command not found**: Add installation directory to PATH or use full path
   - **If port 1883 is in use**: Change port in config or stop conflicting service

### Configuration

Mosquitto should work out of the box for local testing. For network access (ESP32-S3 on same LAN):

1. **Find your Windows IP address**:
   ```powershell
   ipconfig
   # Look for IPv4 Address (e.g., 192.168.1.50)
   ```

2. **Edit Mosquitto config** (if needed):
   - Location: `C:\Program Files\mosquitto\mosquitto.conf`
   - Add these lines if not present:
     ```
     listener 1883
     allow_anonymous true
     ```
   - Restart Mosquitto service

### Testing

```powershell
# Subscribe to test topic (in one terminal)
mosquitto_sub -h localhost -t test/topic -v

# Publish test message (in another terminal)
mosquitto_pub -h localhost -t test/topic -m "Hello MQTT"
```

---

## Option 2: EMQX (Alternative)

**High-performance option** - Good for testing, has web UI.

### Installation

1. **Download EMQX**:
   - Go to: https://www.emqx.io/downloads
   - Download Windows version
   - Extract ZIP file

2. **Run EMQX**:
   ```powershell
   cd emqx\bin
   .\emqx start
   ```

3. **Access Web UI**:
   - Open browser: http://localhost:18083
   - Default username: `admin`
   - Default password: `public`

4. **Default Port**: 1883 (same as Mosquitto)

---

## Option 3: HiveMQ Community Edition (Alternative)

**User-friendly option** - Good web interface, easy setup.

### Installation

1. **Download HiveMQ**:
   - Go to: https://www.hivemq.com/downloads/
   - Download Community Edition (free)
   - Extract ZIP file

2. **Run HiveMQ**:
   ```powershell
   cd hivemq\bin
   .\run.bat
   ```

3. **Access Web UI**:
   - Open browser: http://localhost:8080
   - Default port: 1883

---

## Option 4: MQTTX (Quick Testing Tool)

**Lightweight option** - Good for quick testing, includes broker.

### Installation

1. **Download MQTTX**:
   - Go to: https://mqttx.app/
   - Download Windows version
   - Install

2. **Use Built-in Broker**:
   - MQTTX includes a simple broker for testing
   - Or use it as a client to test your broker

---

## Configuring ESP32-S3 to Use Windows Broker

Your ESP32-S3 needs to connect to your Windows machine instead of the Raspberry Pi.

### Find Windows IP Address

```powershell
ipconfig
# Note your IPv4 Address (e.g., 192.168.1.50)
```

### Update ESP32-S3 Code

In your `sht21-solo.ino` file, update the MQTT server:

```cpp
// OLD (Raspberry Pi)
const char* mqtt_server = "192.168.1.XXX";  // Pi IP

// NEW (Windows machine)
const char* mqtt_server = "192.168.1.50";  // Your Windows IP
```

Or if using `MQTT_RPi_Client` library, you may need to update the library's default settings.

### Re-upload to ESP32-S3

After updating the IP address, re-upload the sketch to your ESP32-S3.

---

## Testing the Setup

### 1. Test MQTT Broker

```powershell
# Terminal 1: Subscribe to SHT21 topic
mosquitto_sub -h localhost -t "sensors/sht21/readings" -v

# Terminal 2: Publish test message (simulate ESP32-S3)
mosquitto_pub -h localhost -t "sensors/sht21/readings" -m '{"temperature":25.5,"humidity":50.0,"timestamp":1234567890}'
```

### 2. Test ESP32-S3 Connection

1. **Check ESP32-S3 Serial Monitor**:
   - Should show "MQTT connected" or similar
   - Should show "Published to sensors/sht21/readings"

2. **Subscribe to verify ESP32-S3 is publishing**:
   ```powershell
   mosquitto_sub -h localhost -t "sensors/sht21/readings" -v
   ```

### 3. Test Heatsoak Monitor

```powershell
# Run the monitor script
python monitor_heatsoak.py --printer-ip 192.168.1.100 --mqtt-host localhost

# Or if broker is on different machine
python monitor_heatsoak.py --printer-ip 192.168.1.100 --mqtt-host 192.168.1.50
```

---

## Windows Firewall Configuration

If ESP32-S3 can't connect, you may need to allow MQTT through Windows Firewall:

```powershell
# Allow MQTT port 1883
New-NetFirewallRule -DisplayName "MQTT Broker" -Direction Inbound -LocalPort 1883 -Protocol TCP -Action Allow
```

Or via GUI:
1. Windows Security → Firewall & network protection
2. Advanced settings
3. Inbound Rules → New Rule
4. Port → TCP → Specific local ports: 1883
5. Allow the connection

---

## Troubleshooting

### Mosquitto Installation Issues

1. **Chocolatey package not found**:
   - Use direct download from https://mosquitto.org/download/ instead
   - The Chocolatey package may not be maintained

2. **Installer fails**:
   - Run installer as Administrator
   - Check Windows Event Viewer for detailed errors
   - Try the ZIP version and manual installation

3. **Service won't start**:
   - Check if port 1883 is already in use: `netstat -an | findstr 1883`
   - Check Windows Event Viewer: `eventvwr.msc` → Windows Logs → Application
   - Verify config file exists: `C:\Program Files\mosquitto\mosquitto.conf`

4. **Command not found**:
   - Add Mosquitto to PATH (see commands below)
   - Or use full path: `"C:\Program Files\mosquitto\mosquitto.exe"`

### Adding Mosquitto to PATH

If `mosquitto` command is not found, add it to your PATH:

```powershell
# 1. Find Mosquitto installation directory (usually one of these)
$mosquittoPath = "C:\Program Files\mosquitto"
# Or if installed elsewhere:
# $mosquittoPath = "C:\mosquitto"

# 2. Verify the directory exists and contains mosquitto.exe
Test-Path "$mosquittoPath\mosquitto.exe"

# 3. Add to PATH for current session (temporary)
$env:Path += ";$mosquittoPath"

# 4. Add to PATH permanently (user-level - recommended)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$mosquittoPath", "User")

# 5. Verify it was added
$env:Path -split ';' | Select-String mosquitto

# 6. Test the command (may need to restart terminal)
mosquitto --help



$mosquittoPath = "C:\Program Files\mosquitto" 
Test-Path "$mosquittoPath\mosquitto.exe"
$env:Path += ";$mosquittoPath"
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$mosquittoPath", "User")
$env:Path -split ';' | Select-String mosquitto


```

**Note**: After adding to PATH permanently, you may need to:
- Close and reopen your terminal/PowerShell window
- Or refresh the PATH in current session: `$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")`

### ESP32-S3 Can't Connect

1. **Check Windows IP address** - Make sure it matches what's in ESP32-S3 code
2. **Check firewall** - Port 1883 must be open
3. **Check broker is running** - `Get-Service mosquitto` or check process
4. **Check network** - ESP32-S3 and Windows must be on same network

### No Messages Received

1. **Verify topic name** - Check ESP32-S3 is publishing to correct topic
2. **Check broker logs** - Look for connection attempts
3. **Test with mosquitto_sub** - Verify messages are being published

### Monitor Script Can't Connect

1. **Check MQTT host** - Use `localhost` if broker is on same machine
2. **Check topic** - Default is `sensors/sht21/readings`
3. **Verify messages** - Use `mosquitto_sub` to verify messages are arriving

---

## Switching Back to Raspberry Pi

When your Raspberry Pi is ready:

1. **Update ESP32-S3 code** with Raspberry Pi IP address
2. **Re-upload sketch** to ESP32-S3
3. **Update monitor script** to use Raspberry Pi MQTT host:
   ```powershell
   python monitor_heatsoak.py --printer-ip 192.168.1.100 --mqtt-host 192.168.1.XX
   ```

---

## Recommended Setup for Testing

### If Mosquitto Installation Fails

If you're having trouble with Mosquitto installation, **EMQX (Option 2) is the easiest alternative**:
- Simple ZIP extract (no installer needed)
- Web UI included
- Works immediately after extraction
- Good for testing and development

### Installation Priority

1. **First try**: Mosquitto direct download (Option 1) - matches Raspberry Pi setup
2. **If that fails**: EMQX (Option 2) - easiest alternative, just extract and run
3. **For quick testing**: MQTTX (Option 4) - includes built-in broker

### Quick Start with EMQX (Easiest Option)

If Mosquitto gives you trouble, use EMQX instead:

1. Download EMQX: https://www.emqx.io/downloads
2. Extract ZIP file
3. Run: `emqx\bin\emqx start`
4. Access Web UI: http://localhost:18083 (admin/public)
5. Use in monitor script: `--mqtt-host localhost` (same as Mosquitto)

**Both Mosquitto and EMQX use port 1883 and are compatible with your ESP32-S3 and monitor script.**
