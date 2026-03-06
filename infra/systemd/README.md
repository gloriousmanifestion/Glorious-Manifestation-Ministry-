# Systemd units for VM deployment

CI/CD deploys to two environments:

| Branch   | Environment | Deploy path (on VM)              | Systemd unit          |
|----------|-------------|----------------------------------|------------------------|
| `main`   | **staging** | `/opt/pgm/backend/staging`, `/opt/pgm/frontend/staging` | `pgm-backend-staging`, `pgm-frontend-staging` |
| `prod`   | **production** | `/opt/pgm/backend/production`, `/opt/pgm/frontend/production` | `pgm-backend-production`, `pgm-frontend-production` |

## GitHub setup

1. **Environments**  
   In the repo: **Settings → Environments**. Create:
   - `staging` (used when a PR is merged to `main`)
   - `production` (used when pushing/merging to `prod`)

2. **Secrets** (repository or environment-level)  
   - `VM_IP` – VM host  
   - `VM_USER` – SSH user  
   - `SSH_PRIVATE_KEY` – Private key for SSH  
   - `VM_SSH_PORT` – (optional) SSH port, default 22  
   - `VM_APP_NAME` – (optional) App name used in `/opt/<name>/...`, default `pgm`

## VM setup

1. Create directories (replace `pgm` if using `VM_APP_NAME`):
   ```bash
   sudo mkdir -p /opt/pgm/backend/{staging,production}/bin
   sudo mkdir -p /opt/pgm/frontend/{staging,production}
   ```

2. **Service files**: Run `pnpm run apply-template` (from repo root) so that the script creates the actual systemd unit files from the `.example` templates (e.g. `pgm-frontend-staging.service`, `pgm-backend-staging.service`, etc.). Commit the created files under `infra/systemd/`; CI/CD copies them during deploy and installs them on the VM. If you prefer to install by hand, copy the generated files (not the `.example` ones) and ensure paths match your app name.

3. Enable and start (after first deploy):
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable pgm-backend-staging pgm-frontend-staging pgm-backend-production pgm-frontend-production
   sudo systemctl start pgm-backend-staging pgm-frontend-staging   # or production
   ```
