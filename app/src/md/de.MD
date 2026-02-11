<img src="logo.png" style="border-radius: 8px; margin-right:16px; margin-top: 20px; height: 89px;" align="right" alt="Free FPS Logo" />

- [Über dieses Programm](#about-this-tool)
- [Hauptseite](#main-page)
- [Konvertierungsseite](#processing-page)
- [Lizenz](#license)
- [Software von Drittanbietern](#3d-party-license)
- [Quellcode](#source-code)

---

<a id="about-this-tool"></a>
### Über dieses Programm

Eine Hilfsanwendung zum Ändern der Bildwiederholfrequenz (FPS) von Videodateien mittels FFmpeg. Im Gegensatz zu Videoschnittprogrammen fügt sie keine Effekte hinzu und verändert keinen Inhalt - sie passt nur die Wiedergabegeschwindigkeit an und kodiert bei Bedarf das Audio neu.

Nützlich, wenn Sie mit mehreren Videos arbeiten, die mit unterschiedlichen Bildraten aufgenommen wurden und sich ohne Interpolation oder Frameverlust nicht zusammenfügen oder bearbeiten lassen.
Dient auch als schneller Videokompressor: Behalten Sie die ursprüngliche FPS bei und erhöhen Sie die Kompression (z. B. höherer CRF oder niedrigere Bitrate), um die Dateigröße zu reduzieren.

Wichtig:
Das Programm richtet sich an Anwender, die verstehen, was eine FPS-Konvertierung bewirkt. Die Änderung der Bildrate beeinflusst die Geschwindigkeit und Dauer des Videos. Das Werkzeug verändert nicht die Originaldateien!

Dieses Programm wird kostenlos („As Is“) unter der GNU General Public License v3.0 (GPL-3.0) verteilt.
Wenn Sie den Autor unterstützen möchten, können Sie das hier tun:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; " alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; " alt="suppi Logo" />
</a>

<br>

---

<a id="main-page"></a>
### Hauptseite

- **Eingabeordner** - wählen Sie den Ordner mit den zu konvertierenden Dateien. (An dieser Stelle kann keine einzelne Datei ausgewählt werden, später können Sie jedoch die benötigten Videos auswählen.)
- **Ausgabeordner** - geben Sie den Ordner zum Speichern der Ergebnisse an. Wenn das Feld leer bleibt, werden die Dateien in einem automatisch erstellten Unterordner neben den Originalen gespeichert.
- **Ziel-FPS** - gewünschte Bildrate.
- **Benutzerdefinierte FPS** - geben Sie einen eigenen FPS-Wert ein, falls der gewünschte nicht in der Liste enthalten ist.
- **Audio behalten** - behält die Audiospur der Quelldatei. Der Ton kann **verlangsamt oder beschleunigt werden**, daher ist die Option standardmäßig deaktiviert.
- **Audio-Bitrate** - legen Sie die Bitrate fest, wenn Sie das Audio behalten.
- **Benutzerdefinierte Videoqualität** - Ausgabequalität des Videos (CRF). Standardmäßig verwendet die App den Auto-Modus, der die Bitrate basierend auf den Eigenschaften des Quellvideos schätzt.<br/>**WICHTIG:** Der Auto-Modus kann sichtbare Artefakte (Blockbildung, Banding, Unschärfe) in komplexen Szenen mit schnellen Bewegungen, schnellen Kamerabewegungen oder feinen Details verursachen. Er kann auch bei bereits komprimierten Quellen oder Bildschirmaufnahmen unterperformen. Für wichtige oder visuell komplexe Videos bevorzugen Sie **Hoch** oder **Maximum** Qualität.<br/>
  Achtung: zu niedrige Werte führen zu schlechter Qualität; zu hohe Werte erzeugen sehr große Dateien. Mehr: [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
- **GPU-Beschleunigung** - verwendet Ihre Grafikkarte für schnellere Videokodierung. Unterstützt NVIDIA (NVENC), AMD (AMF) und Intel (QuickSync). Kann 3-5x schneller sein als CPU-Kodierung. Wenn keine kompatible GPU erkannt wird, ist die Option deaktiviert.
- **CPU-Limit** - Begrenzung der CPU-Nutzung. Kann die Systembelastung leicht reduzieren, aber nicht garantiert.

---

<a id="processing-page"></a>
### Konvertierungsseite

- Verwenden Sie die Kontrollkästchen, um auszuwählen, welche Videos konvertiert werden sollen. Standardmäßig sind alle ausgewählt.
- **Ordner neu scannen** - den Ordner neu scannen und die Dateiliste aktualisieren.
- Nach dem Start des Konverters können Sie den Prozess jederzeit abbrechen.
- Videokonvertierung ist ressourcenintensiv und kann Zeit in Anspruch nehmen. Fortschritt und Status werden auf dem Bildschirm angezeigt.

> Wenn die Konvertierung eines Videos mit einem Fehler endet, ist es unwahrscheinlich, dass ein erneuter Versuch erfolgreich ist.
> Möglicherweise konnte FFmpeg die erforderlichen Daten aus der Datei nicht lesen.

---

<a id="license"></a>
### Lizenz
Free FPS ist freie Software unter der **GNU General Public License v3.0**. Sie dürfen:
- Diese Software für jeden Zweck verwenden
- Den Quellcode studieren und ändern
- Kopien weitergeben
- Geänderte Versionen verbreiten (unter GPL v3)

Lesen Sie die [Projektlizenz](FreeFPSLicense).

---

<a id="3d-party-license"></a>
### Software von Drittanbietern
Dieses Programm verwendet **FFmpeg**, das ebenfalls unter GPL v3.0 lizenziert ist.
- Lesen Sie [FFmpeg Notice](FFmpegNotice)
- Lesen Sie [FFmpeg License](FFmpegLicense)

---

<a id="source-code"></a>
### Quellcode

Der vollständige Quellcode ist verfügbar unter:  
https://github.com/undelalune/free-fps

<br/>

---

<p style="text-align:center;">Copyright © {{YEAR}} undelalune</p>
