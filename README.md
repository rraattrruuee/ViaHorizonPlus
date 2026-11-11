# ViaHorizonPlus

**ViaHorizonPlus** est le portage et l'adaptation native en **Rust** de [ViaFabricPlus](https://github.com/ViaVersion/ViaFabricPlus) conçu spécifiquement pour le moteur [Horizon](https://playhorizon.dev/) (réécriture complète de Minecraft en Rust).

---

## 🔍 Pourquoi un portage dédié pour Horizon ?

1. **Pas de machine virtuelle Java (JVM) ni de Fabric :**  
   Horizon est écrit à 100% en Rust (avec WGPU, Tokio, Rayon). Les mods Java classiques (`.jar`), les chargeurs comme Fabric/NeoForge, ainsi que les bytecode mixins (SpongePowered Mixins / ASM) ne peuvent pas s'exécuter dans Horizon.
2. **Le système de modding natif d'Horizon (`horizon-mixins`) :**  
   Horizon propose deux types de mods :
   - **Content Mods (`.hmod`)** : Uniquement déclaratifs (JSON pour blocs, items, recettes - aucun code exécutable).
   - **Native Mods (`.hnmod`)** : Du code Rust natif injecté à la compilation dans l'AST (arbre syntaxique abstrait) du jeu via `horizon-mixins` et configuré via `horizon-mixin.toml`.
3. **Le défi de ViaVersion en Rust :**  
   ViaFabricPlus en Java dépend d'une chaîne gigantesque de projets (> 500 000 lignes de code Java) : `ViaVersion`, `ViaBackwards`, `ViaLegacy` et `classic4j`.  
   Dans Horizon, la couche protocole (`crates/horizon-protocol`) et le client (`crates/horizon-client`) nécessitent des hooks d'interception réseau et de gameplay natifs en Rust.

---

## 🏗 Architecture de ViaHorizonPlus

Le projet est structuré comme suit :

* `horizon-mixin.toml` : Manifeste officiel de mod natif Horizon décrivant les modules et points d'injection (hooks dans `Game::native_tick`, `Game::native_render_hud`, etc.).
* `src/protocol/` :
  - `version.rs` : Registre exhaustif des versions de protocole (de Classic 0.30, Alpha, Beta 1.7.3, 1.8.x jusqu'à Horizon Native 26.3).
  - `handshake.rs` : Encodage, décodage et réécriture du paquet Handshake (modification du `protocol_version` envoyé au serveur).
  - `translator.rs` : Trait d'interception et de translation bidirectionnelle des paquets (serveur <-> client).
* `src/emulation/` :
  - `combat.rs` : Émulation des mécaniques de combat 1.8 (blocage à l'épée, suppression du cooldown d'attaque 1.9+, sons d'impact legacy).
  - `movement.rs` : Émulation de la physique ancienne (hauteur des yeux en sneak à 1.54m au lieu de 1.27m, échelles 1.8, etc.).
  - `visuals.rs` : Rendu et animations classiques (block-hitting 1.7/1.8, teinte de dégâts rouge classique).
* `src/state.rs` : Gestion de l'état global (`ViaHorizonState`) thread-safe avec sélection de version dynamique.
* `src/gui/` : Menu et sélecteur de version pour l'écran multijoueur d'Horizon.
* `src/hooks/` : Implémentation des fonctions helpers injectées par `horizon-mixins`.

---

## 🚀 Utilisation & Compilation

### Vérification du code Rust :
```bash
cargo test
cargo check
```

### Intégration dans Horizon :
Dans le dossier du mod natif, compilez le paquet Horizon ou lancez le jeu en mode développement avec le lanceur Horizon :
```bash
# Lancement en mode dev mod avec Horizon Launcher
horizon-launcher --dev-mod "/chemin/vers/viahorizonplus"
```

---

## 📜 Licence & Crédits

Ce projet est distribué sous la licence libre et permissive **[MIT](LICENSE)**.

### Crédits & Inspiration
- Inspiré par le concept de [ViaFabricPlus](https://github.com/ViaVersion/ViaFabricPlus) développé par FlorianMichael et l'équipe ViaVersion.
- Implémentation 100% native en Rust pour Horizon par rraattrruuee et les contributeurs.


