# ViaHorizonPlus

[English](README.md) | **Français**

**ViaHorizonPlus** est le portage et l'adaptation native en **Rust** inspiré de [ViaFabricPlus](https://github.com/ViaVersion/ViaFabricPlus), conçu spécifiquement pour le moteur [Horizon](https://playhorizon.dev/) (réécriture complète de Minecraft en Rust).

---

## 🔍 Pourquoi un portage dédié pour Horizon ?

1. **Pas de machine virtuelle Java (JVM) ni de Fabric :**  
   Horizon est écrit à 100% en Rust (avec WGPU, Tokio, Rayon). Les mods Java classiques (`.jar`), les chargeurs comme Fabric/NeoForge, ainsi que les bytecode mixins (SpongePowered Mixins / ASM) ne peuvent pas s'exécuter dans Horizon.
2. **Le système de modding natif d'Horizon (`horizon-mixins`) :**  
   Horizon propose deux types de mods :
   - **Content Mods (`.hmod`)** : Uniquement déclaratifs (JSON pour blocs, items, recettes - aucun code exécutable).
   - **Native Mods (`.hnmod`)** : Du code Rust natif injecté à la compilation dans l'AST (arbre syntaxique abstrait) du jeu via `horizon-mixins` et configuré via `horizon-mixin.toml`.
3. **Le défi de ViaVersion en Rust :**  
   ViaFabricPlus en Java dépend d'une chaîne gigantesque de projets Java (`ViaVersion`, `ViaBackwards`, `ViaLegacy` et `classic4j`). Dans Horizon, la couche protocole (`crates/horizon-protocol`) et le client (`crates/horizon-client`) nécessitent des hooks d'interception réseau et de gameplay natifs en Rust.

---

## ✨ Fonctionnalités

- **Sélecteur de version en jeu :** Accessible directement en haut à droite de l'écran Multijoueur.
- **Auto-détection (Ping) ou Forçage manuel :** Détection automatique du protocole du serveur ou sélection manuelle de versions (de 1.8.x jusqu'à 26.3).
- **Émulation du combat et de la physique 1.8 :**
  - Blocage à l'épée avec clic droit
  - Suppression du délai d'attaque / cooldown 1.9+
  - Animation classique de block-hit
  - Hauteur des yeux en sneak 1.8 (1.54m au lieu de 1.27m)
- **100% Rust Natif :** Aucune dépendance Java, performances optimales.

---

## 🏗 Architecture du projet

Le projet est structuré comme suit :

* `bundle/mixins/viahorizonplus/` : Les sources du mod natif injectées dans Horizon (`horizon-mixin.toml`, `runtime.rs`, `legacy_run.rs`, `older_server.rs`, etc.).
* `compatibility.json` : Fichier de compatibilité Horizon épinglant la révision exacte du moteur.
* `src/protocol/` :
  - `version.rs` : Registre des versions de protocole (de Classic 0.30, Beta 1.7.3, 1.8.x jusqu'à Horizon Native 26.3).
  - `handshake.rs` : Encodage et réécriture du paquet Handshake.
  - `translator.rs` : Trait d'interception et de translation des paquets réseau.
* `src/emulation/` :
  - `combat.rs` : Émulation des mécaniques de combat 1.8 (blocage à l'épée, suppression du cooldown).
  - `movement.rs` : Émulation de la physique legacy (sneak à 1.54m, etc.).
  - `visuals.rs` : Rendu classique (block-hitting).

---

## 🚀 Utilisation & Compilation

### Vérification du code Rust :
```bash
cargo test
cargo check
```

### Intégration dans Horizon :
Lancez le jeu en mode développement avec le lanceur Horizon :
```bash
horizon-launcher --dev-mod "/chemin/vers/viahorizonplus"
```

---

## 📜 Licence & Crédits

Ce projet est distribué sous la licence libre et permissive **[MIT](LICENSE)**.

### Crédits & Inspiration
- Inspiré par le concept de [ViaFabricPlus](https://github.com/ViaVersion/ViaFabricPlus) développé par FlorianMichael et l'équipe ViaVersion.
- Implémentation 100% native en Rust pour Horizon développée par rraattrruuee et les contributeurs.
