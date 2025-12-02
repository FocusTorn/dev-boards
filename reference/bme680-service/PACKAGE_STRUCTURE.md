# BME680 Service Package Structure

## Installation Locations

When installed, the package creates a self-contained installation in:

```
~/.local/share/bme680-service/
├── monitor/              # BME680 library (monitor module)
├── monitor-iaq.py        # IAQ monitoring script
├── monitor-heatsoak.py        # Heat soak monitoring script
├── .venv/                # Virtual environment with dependencies
└── config/               # Configuration files (if needed)

~/.local/bin/
├── bme680-iaq            # Wrapper script (optional)
└── bme680-temperature    # Wrapper script (optional)
```

## Package Source Structure

```
bme680-service/
├── data/                  # Data files to be installed
│   ├── requirements.txt   # Python dependencies
│   ├── monitor/           # BME680 library code
│   │   ├── __init__.py
│   │   └── constants.py
│   ├── monitor-iaq.py     # IAQ monitor script
│   └── monitor-heatsoak.py     # Heat soak monitor script
├── detectors/             # Detection scripts
│   └── detect-bme680.sh   # Sensor detection
├── services/              # Systemd service files
│   ├── bme680-iaq.service
│   └── bme680-temperature.service
├── scripts/               # Installation scripts
│   ├── install.sh         # Main installer
│   └── uninstall.sh       # Uninstaller
└── README.md              # Documentation
```

## Installation Process

1. Copy data files to `~/.local/share/bme680-service/`
2. Create virtual environment in `~/.local/share/bme680-service/.venv/`
3. Install Python dependencies from `requirements.txt`
4. Install systemd service files to `/etc/systemd/system/`
5. Optionally create wrapper scripts in `~/.local/bin/`

## Uninstallation Process

1. Stop and disable systemd services
2. Remove service files from `/etc/systemd/system/`
3. Remove `~/.local/share/bme680-service/` directory
4. Remove wrapper scripts from `~/.local/bin/` (if created)

