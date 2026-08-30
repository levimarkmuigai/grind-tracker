# Deployment – grind-tracker

## Architecture

- Single EC2 instance (t3g.small, Ubuntu 24.04, af-south-1)
- Rust binary (Axum + SQLx + SQLite) managed by systemd
- Caddy as reverse proxy (ports 80/443)
- Application listens only on 127.0.0.1:3000
- Daily SQLite backups via systemd timer

## Key locations

- Binary: `/opt/grind-tracker/server`
- Environment: `/opt/grind-tracker/env`
- Database: `/var/lib/grind-tracker/app.db`
- Backups: `/var/lib/grind-tracker/backups/`
- Service: `grind-tracker.service`

## Common operations

- View logs: `journalctl -u grind-tracker -f`
- Restart: `sudo systemctl restart grind-tracker`
- Manual backup: `~/backup-db.sh`

## Security notes

- SSH restricted to my IP
- Application not exposed directly to the internet
- Non-root service user
- Automatic security updates enabled
