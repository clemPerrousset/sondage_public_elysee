# 📘 Voting Backend – Serveur Rust sécurisé pour application de vote Android/iOS

Ce backend fournit un système de vote robuste destiné à des applications mobiles Android et iOS **sans création de compte utilisateur**.  
L’objectif est d’assurer que **chaque vote provient réellement d’une application officielle** installée sur un appareil valide, sans possibilité de falsification ni d’automatisation.

Pour cela, le backend ne repose **sur aucun secret embarqué dans l’application** (technique facilement contournable).  
À la place, il utilise des mécanismes cryptographiques **fournis nativement par Google et Apple** :

- **Google Play Integrity API** pour Android  
- **Apple DeviceCheck / App Attest** pour iOS  

Ces services permettent de vérifier l’intégrité de l'application, de l’appareil, ainsi que la provenance réelle de chaque opération de vote.

---

## 🔒 Sécurisation des votes : principe général

### 🟦 1. Android – Google Play Integrity  
Lorsqu'un utilisateur vote depuis Android, l’application doit obtenir auprès de Google un **Play Integrity Token**.  
Ce token certifie :

- que l’application est **authentique** (signée par ta clé officielle),
- qu’elle n’a pas été modifiée ou re-signée,
- qu’elle provient du Play Store,
- que l’appareil n’est pas compromis (root, émulateur, etc.),
- que la requête provient réellement d’un **device physique**.

Le backend contacte ensuite les serveurs Google pour valider cryptographiquement ce token.

👉 Si le verdict n’est pas parfaitement conforme → **vote rejeté**.

---

### 🟩 2. iOS – Apple DeviceCheck / App Attest  
Côté iOS, l’application obtient un jeton signé via `DCDevice` (ou App Attest pour niveau supérieur).  
Ce jeton certifie auprès des serveurs Apple :

- que l’app provient de ton **bundle officiel**,
- qu'elle n’a pas été altérée,
- que l’appareil est réel et non compromis.

Le backend valide ensuite ce jeton auprès d’Apple avant d’accepter le vote.

---

### 🛡️ Pourquoi ce système est robuste ?

- **Aucun secret n’est stocké dans l’application** → impossible à extraire ou falsifier.
- **Les tokens sont émis et signés par Google/Apple** → impossibles à contrefaire.
- **Impossible de voter depuis un script ou un serveur pirate**.
- **Protection anti-app modifiée** : clones, APK resignés ou versions altérées → rejet immédiat.
- **Fonctionne sans compte utilisateur**, tout en empêchant les abus.

Ce mécanisme correspond aux standards modernes utilisés dans les apps sensibles (paiement, authentification, etc.).

---

# 🚀 Démarrage rapide (Développement)

```bash
# Installer les dépendances
cargo build

# Initialiser la base de données
python setup_db.py

# Démarrer le serveur
cargo run
```

Le serveur écoute sur :  
👉 `http://localhost:3000`

---

# 📚 Documentation

- **SECURITY.md** – Protocole complet (Play Integrity & DeviceCheck)
- **DEPLOYMENT.md** – Guide de déploiement sur VPS OVH
- **walkthrough.md** – Détails techniques internes

---

# 🔌 Endpoints disponibles

### **POST /vote**  
Effectue un vote pour un candidat.  
⚠️ Requiert un token d'intégrité Android/iOS.

### **GET /percentage**  
Retourne les pourcentages de votes par candidat.

### **DELETE /candidate**  
Supprime un candidat.  
🔑 Requiert : `X-Admin-Key`.

---

# 🧪 Tests rapides

```bash
# Démarrer le serveur
cargo run

# Dans un autre terminal
python test_api.py
```

---

# 🌐 Déploiement

Serveur de production (exemple) :  
`151.80.133.119`

---

# 🔒 Sécurité récapitulée

- **Android** : Google Play Integrity API  
- **iOS** : Apple DeviceCheck / App Attest  
- **Admin** : API Key via header `X-Admin-Key`

Aucun secret dans les apps, aucune dépendance à des comptes utilisateurs.  
Le backend valide directement auprès des serveurs Apple/Google toute preuve d’intégrité.

---

# 📝 Configuration

Créer un fichier `.env` à la racine :

```env
DATABASE_URL=sqlite:voting.db?mode=rwc
ADMIN_KEY=votre_cle_admin

# Android
GOOGLE_PACKAGE_NAME=com.votre.app
GOOGLE_SERVICE_ACCOUNT_JSON=path/to/service_account.json

# iOS
APPLE_KEY_ID=ABC1234567
APPLE_TEAM_ID=DEF1234567
APPLE_P8_FILE_CONTENT=...
```
