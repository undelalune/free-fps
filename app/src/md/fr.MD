- [À propos de cet outil](#about-this-tool) <img src="logo.png" style="float: right; margin-right:24px; margin-top:12px; height: 89px; " alt="Free FPS Logo" />
- [Comment démarrer ?](#how-to-start)
- [Page des paramètres](#settings-page)
- [Page principale](#main-page)
- [Page de conversion](#processing-page)

---

<a id="about-this-tool"></a>
### À propos de cet outil

Une utilitaire pour modifier la fréquence d'images (FPS) des fichiers vidéo en utilisant FFmpeg. Contrairement aux éditeurs vidéo, il n'ajoute pas d'effets et ne modifie pas le contenu - il ajuste uniquement la vitesse de lecture et, si nécessaire, ré-encode l'audio.

Utile si vous travaillez avec plusieurs vidéos tournées à des fréquences d'images différentes et qu'il est impossible de les assembler ou de les éditer sans interpolation ou perte d'images.

Important:
Le programme s'adresse aux utilisateurs qui comprennent ce que fait une conversion de FPS. Changer la fréquence d'images affecte la vitesse et la durée de la vidéo. L'outil ne modifie pas les fichiers originaux !

Ce programme est distribué gratuitement (\"As Is\") sous licence MIT.
Si vous souhaitez soutenir l'auteur, vous pouvez le faire ici :

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; " alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; " alt="suppi Logo" />
</a>

<br>

---

<a id="how-to-start"></a>
### Comment démarrer ?

Au lancement, l'application vérifie automatiquement si les paquets ffmpeg et ffprobe sont disponibles dans le `PATH` système et proposera de les utiliser.
S'ils ne sont pas présents, téléchargez FFmpeg depuis le site officiel et extrayez l'archive.<br>
Windows: <a href="https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" target="_blank" rel="noopener" title="Download ffmpeg/ffprobe archive">ffmpeg/ffprobe</a><br>
macOS: <a href="https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip" target="_blank" rel="noopener" title="Download ffmpeg">ffmpeg</a> |
<a href="https://evermeet.cx/ffmpeg/ffprobe-8.0.zip" target="_blank" rel="noopener" title="Download ffprobe archive">ffprobe</a>

L'installation dans le système n'est pas requise - il suffit de renseigner les chemins vers les exécutables dans les paramètres de l'application.

- **Windows**: `bin/ffmpeg.exe` et `bin/ffprobe.exe`
- **macOS**: `ffmpeg` et `ffprobe` (à la racine du dossier)

Après cela, vous pouvez commencer à utiliser l'application.

---

<a id="settings-page"></a>
### Page des paramètres

- Chemin vers ffmpeg - chemin vers l'exécutable `ffmpeg`. Ce champ est requis.
- Chemin vers ffprobe - chemin vers `ffprobe`. Optionnel, mais recommandé pour un fonctionnement plus stable.
- Utiliser la version installée (ffmpeg / ffprobe) - devient disponible si des versions installées sont détectées.
- Langue - sélectionnez la langue de l'interface.
- Thème - choisissez le thème de couleurs de l'application.
- Réinitialiser les paramètres - réinitialise les données sauvegardées (seules les valeurs des champs du formulaire).

L'application enregistre automatiquement vos paramètres - vous n'avez pas besoin de les saisir à chaque lancement.

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

<p style="text-align:center;">© 2025 undelalune</p>
