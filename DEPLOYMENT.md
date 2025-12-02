# Déploiement sur VPS OVH Ubuntu

## Informations Serveur
- **IP**: 151.80.133.119
- **OS**: Ubuntu
- **Port**: 3000 (ou 80/443 avec reverse proxy)

## 1. Installation sur le VPS

### Prérequis
```bash
# Connexion SSH
ssh root@151.80.133.119

# Mise à jour système
sudo apt update && sudo apt upgrade -y

# Installation de Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Installation SQLite
sudo apt install sqlite3 libsqlite3-dev -y
```

### Transfert du projet
```bash
# Sur votre machine locale
scp -r "d:\D\projet sondage backend" root@151.80.133.119:/opt/voting-backend

# Ou via git (recommandé)
# Sur le VPS:
cd /opt
git clone <votre-repo> voting-backend
cd voting-backend
```

## 2. Configuration

### Fichier .env
Créer `/opt/voting-backend/.env`:
```env
DATABASE_URL=sqlite:/opt/voting-backend/voting.db?mode=rwc
ADMIN_KEY=CHANGEZ_MOI_EN_PRODUCTION
RUST_LOG=info

# Google Play Integrity
GOOGLE_PACKAGE_NAME=com.votreapp.nom
GOOGLE_SERVICE_ACCOUNT_JSON=/opt/voting-backend/service_account.json

# Apple DeviceCheck
APPLE_KEY_ID=VOTRE_KEY_ID
APPLE_TEAM_ID=VOTRE_TEAM_ID
APPLE_P8_FILE_CONTENT=-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----
```

### Initialiser la base de données
```bash
cd /opt/voting-backend
python3 setup_db.py
```

## 3. Build de production
```bash
cd /opt/voting-backend
cargo build --release
```

## 4. Service systemd (démarrage automatique)

Créer `/etc/systemd/system/voting-backend.service`:
```ini
[Unit]
Description=Voting Backend Service
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/voting-backend
Environment="PATH=/root/.cargo/bin:/usr/local/bin:/usr/bin:/bin"
ExecStart=/opt/voting-backend/target/release/voting-backend
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Activer et démarrer:
```bash
sudo systemctl daemon-reload
sudo systemctl enable voting-backend
sudo systemctl start voting-backend
sudo systemctl status voting-backend
```

## 5. Configuration du pare-feu

```bash
# Ouvrir le port 3000
sudo ufw allow 3000/tcp
sudo ufw enable
```

## 6. Test de l'API

Depuis votre machine locale:
```bash
curl http://151.80.133.119:3000/percentage
```

## 7. (Optionnel) Reverse Proxy Nginx

Pour utiliser le port 80 (HTTP) ou 443 (HTTPS):

```bash
sudo apt install nginx -y
```

Créer `/etc/nginx/sites-available/voting-backend`:
```nginx
server {
    listen 80;
    server_name 151.80.133.119;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

Activer:
```bash
sudo ln -s /etc/nginx/sites-available/voting-backend /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
sudo ufw allow 'Nginx Full'
```

## 8. Logs et monitoring

```bash
# Voir les logs
sudo journalctl -u voting-backend -f

# Redémarrer le service
sudo systemctl restart voting-backend

# Arrêter le service
sudo systemctl stop voting-backend
```

## URLs de l'API

- Vote: `http://151.80.133.119:3000/vote` (POST)
- Pourcentages: `http://151.80.133.119:3000/percentage` (GET)
- Supprimer candidat: `http://151.80.133.119:3000/candidate` (DELETE)

Ou avec Nginx:
- `http://151.80.133.119/vote`
- `http://151.80.133.119/percentage`
- `http://151.80.133.119/candidate`
