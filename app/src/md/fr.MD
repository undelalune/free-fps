<img src="logo.png" style="border-radius: 8px; margin-right:16px; margin-top: 20px; height: 89px;" align="right" alt="Free FPS Logo" />

- [À propos de cet outil](#about-this-tool)
- [Page principale](#main-page)
- [Page de conversion](#processing-page)
- [Licence](#license)
- [Logiciels tiers](#3d-party-license)
- [Code source](#source-code)

---

<a id="about-this-tool"></a>
### À propos de cet outil

Une utilitaire pour modifier la fréquence d'images (FPS) des fichiers vidéo en utilisant FFmpeg. Contrairement aux éditeurs vidéo, il n'ajoute pas d'effets et ne modifie pas le contenu - il ajuste uniquement la vitesse de lecture et, si nécessaire, ré-encode l'audio.

Utile si vous travaillez avec plusieurs vidéos tournées à des fréquences d'images différentes et qu'il est impossible de les assembler ou de les éditer sans interpolation ou perte d'images.
Fait aussi office de compresseur vidéo rapide : conservez le FPS d’origine et augmentez la compression (par exemple, CRF plus élevé ou débit binaire plus faible) pour réduire la taille du fichier.

Important:
Le programme s'adresse aux utilisateurs qui comprennent ce que fait une conversion de FPS. Changer la fréquence d'images affecte la vitesse et la durée de la vidéo. L'outil ne modifie pas les fichiers originaux !

Ce programme est distribué gratuitement ("As Is") sous la licence GNU General Public License v3.0 (GPL-3.0).
Si vous souhaitez soutenir l'auteur, vous pouvez le faire ici :

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; " alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; " alt="suppi Logo" />
</a>

<br>

---

<a id="main-page"></a>
### Page principale

- **Dossier d'entrée** - choisissez un dossier contenant les fichiers à convertir. (À cette étape, vous ne pouvez pas sélectionner un fichier unique, mais plus tard vous pourrez choisir les vidéos souhaitées.)
- **Dossier de sortie** - indiquez le dossier pour enregistrer les résultats. Si laissé vide, les fichiers seront sauvegardés dans un sous-dossier créé automatiquement à côté des originaux.
- **FPS cible** - fréquence d'images souhaitée.
- **FPS personnalisé** - définissez votre propre valeur FPS si celle souhaitée n'est pas dans la liste.
- **Conserver l'audio** - conserve l'audio du fichier source. Le son peut être **ralenti ou accéléré**, c’est pourquoi l’option est désactivée par défaut.
- **Débit audio** - définissez le débit si vous conservez l'audio.
- **Qualité vidéo personnalisée** - qualité vidéo de sortie (CRF). Par défaut, l'application tente de préserver la qualité source - c'est recommandé.
  Attention : une valeur trop basse donnera une mauvaise qualité ; une valeur trop élevée produira des fichiers très volumineux. Plus d'infos : [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
- **Limite CPU** - limite l'utilisation du processeur. Peut réduire légèrement la charge, mais ce n'est pas garanti.

---

<a id="processing-page"></a>
### Page de conversion

- Utilisez les cases à cocher pour sélectionner les vidéos à convertir. Toutes sont sélectionnées par défaut.
- **Re-scanner le dossier** - rescanner le dossier et actualiser la liste des fichiers.
- Après le démarrage du convertisseur, vous pouvez annuler le processus à tout moment.
- La conversion vidéo est gourmande en ressources et peut prendre du temps. La progression et le statut sont affichés à l'écran.

> Si la conversion d'une vidéo se termine par une erreur, il est peu probable qu'une nouvelle tentative réussisse.
> Il est possible que FFmpeg n'ait pas pu lire les données nécessaires dans le fichier.

---

<a id="license"></a>
### Licence
Free FPS est un logiciel libre sous licence **GNU General Public License v3.0**. Vous êtes libre de :
- Utiliser ce logiciel à toute fin
- Étudier et modifier le code source
- Redistribuer des copies
- Distribuer des versions modifiées (sous GPL v3)

Lisez le [licence](FreeFPSLicense) du projet.

---

<a id="3d-party-license"></a>
### Logiciels tiers
Ce programme utilise **FFmpeg**, également sous licence GPL v3.0.
- Site FFmpeg : https://ffmpeg.org
- Lire [FFmpeg Notice](FFmpegNotice)
- Lire [FFmpeg License](FFmpegLicense)

---

<a id="source-code"></a>
### Code source

Le code source complet est disponible sur :  
https://github.com/undelalune/free-fps

<br/>

---

<p style="text-align:center;">Copyright © 2025 undelalune</p>
