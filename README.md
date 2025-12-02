# Voting Backend

Backend Rust pour application de vote Android/iOS avec Google Play Integrity et Apple DeviceCheck.

## 🚀 Démarrage rapide (Développement)

```bash
# Installer les dépendances
cargo build

# Initialiser la base de données
python setup_db.py

# Démarrer le serveur
cargo run
```

Le serveur écoute sur `http://localhost:3000`

## 📚 Documentation

- **[SECURITY.md](SECURITY.md)** - Protocole de sécurité (Play Integrity / DeviceCheck)
- **[DEPLOYMENT.md](DEPLOYMENT.md)** - Guide de déploiement sur VPS OVH
- **[walkthrough.md](brain/walkthrough.md)** - Détails techniques complets

## 🔌 Endpoints

### POST /vote
Vote pour un candidat (requiert token d'intégrité).

### GET /percentage
Récupère les pourcentages de vote par candidat.

### DELETE /candidate
Supprime un candidat (Admin uniquement).

## 🧪 Tests

```bash
# Démarrer le serveur
cargo run

# Dans un autre terminal
python test_api.py
```

## 🌐 Déploiement

Serveur de production: `151.80.133.119`

Voir [DEPLOYMENT.md](DEPLOYMENT.md) pour les instructions complètes.

## 🔒 Sécurité

- Android: Google Play Integrity API
- iOS: Apple DeviceCheck
- Admin: API Key (header `X-Admin-Key`)

## 📝 Configuration

Créer un fichier `.env` à la racine:
```env
DATABASE_URL=sqlite:voting.db?mode=rwc
ADMIN_KEY=votre_cle_admin
GOOGLE_PACKAGE_NAME=com.votre.app
APPLE_KEY_ID=...
```
