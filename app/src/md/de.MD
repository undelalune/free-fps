- [Über dieses Programm](#about-this-tool) <img src="logo.png" style="float: right; margin-right:24px; margin-top:12px; height: 89px; " alt="Free FPS Logo" />
- [Wie starten?](#how-to-start)
- [Einstellungsseite](#settings-page)
- [Hauptseite](#main-page)
- [Konvertierungsseite](#processing-page)

---

<a id="about-this-tool"></a>
### Über dieses Programm

Eine Hilfsanwendung zum Ändern der Bildwiederholfrequenz (FPS) von Videodateien mittels FFmpeg. Im Gegensatz zu Videoschnittprogrammen fügt sie keine Effekte hinzu und verändert keinen Inhalt - sie passt nur die Wiedergabegeschwindigkeit an und kodiert bei Bedarf das Audio neu.

Nützlich, wenn Sie mit mehreren Videos arbeiten, die mit unterschiedlichen Bildraten aufgenommen wurden und sich ohne Interpolation oder Frameverlust nicht zusammenfügen oder bearbeiten lassen.

Wichtig:
Das Programm richtet sich an Anwender, die verstehen, was eine FPS-Konvertierung bewirkt. Die Änderung der Bildrate beeinflusst die Geschwindigkeit und Dauer des Videos. Das Werkzeug verändert nicht die Originaldateien!

Dieses Programm wird kostenlos („As Is“) unter der MIT-Lizenz verteilt.
Wenn Sie den Autor unterstützen möchten, können Sie das hier tun:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; " alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; " alt="suppi Logo" />
</a>

<br>

---

<a id="how-to-start"></a>
### Wie starten?

Beim Start prüft die Anwendung automatisch, ob die Pakete ffmpeg und ffprobe im System-`PATH` verfügbar sind, und bietet an, diese zu verwenden.
Wenn sie nicht vorhanden sind, laden Sie FFmpeg von der offiziellen Seite herunter und entpacken das Archiv.<br>
Windows: <a href="https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" target="_blank" rel="noopener" title="Download ffmpeg/ffprobe archive">ffmpeg/ffprobe</a><br>
macOS: <a href="https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip" target="_blank" rel="noopener" title="Download ffmpeg">ffmpeg</a> |
<a href="https://evermeet.cx/ffmpeg/ffprobe-8.0.zip" target="_blank" rel="noopener" title="Download ffprobe archive">ffprobe</a>

Eine Installation im System ist nicht erforderlich - geben Sie einfach die Pfade zu den ausführbaren Dateien in den Anwendungseinstellungen an.

- **Windows**: `bin/ffmpeg.exe` und `bin/ffprobe.exe`
- **macOS**: `ffmpeg` und `ffprobe` (im Stammverzeichnis des Ordners)

Danach können Sie mit der Arbeit beginnen.

---

<a id="settings-page"></a>
### Einstellungsseite

- **Pfad zu ffmpeg** - Pfad zur ausführbaren Datei `ffmpeg`. Dieses Feld ist erforderlich.
- **Pfad zu ffprobe** - Pfad zu `ffprobe`. Optional, aber empfohlen für stabilere Funktion.
- **Installierte Version verwenden (ffmpeg / ffprobe)** - wird verfügbar, wenn installierte Versionen gefunden werden.
- **Sprache** - wählen Sie die Programmsprache.
- **Design** - wählen Sie das Farbschema der Anwendung.
- **Einstellungen zurücksetzen** - gespeicherte Daten zurücksetzen (nur Formfeldwerte).

Die Anwendung speichert Ihre Einstellungen automatisch - ein erneutes Eingeben bei jedem Start ist nicht nötig.

---

<a id="main-page"></a>
### Hauptseite

- **Eingabeordner** - wählen Sie den Ordner mit den zu konvertierenden Dateien. (An dieser Stelle kann keine einzelne Datei ausgewählt werden, später können Sie jedoch die benötigten Videos auswählen.)
- **Ausgabeordner** - geben Sie den Ordner zum Speichern der Ergebnisse an. Wenn das Feld leer bleibt, werden die Dateien in einem automatisch erstellten Unterordner neben den Originalen gespeichert.
- **Ziel-FPS** - gewünschte Bildrate.
- **Benutzerdefinierte FPS** - geben Sie einen eigenen FPS-Wert ein, falls der gewünschte nicht in der Liste enthalten ist.
- **Audio behalten** - behält die Audiospur der Quelldatei. Der Ton kann **verlangsamt oder beschleunigt werden**, daher ist die Option standardmäßig deaktiviert.
- **Audio-Bitrate** - legen Sie die Bitrate fest, wenn Sie das Audio behalten.
- **Benutzerdefinierte Videoqualität** - Ausgabequalität des Videos (CRF). Standardmäßig versucht die Anwendung, die Quellqualität zu erhalten - das ist empfohlen.
  Achtung: zu niedrige Werte führen zu schlechter Qualität; zu hohe Werte erzeugen sehr große Dateien. Mehr: [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
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

<p style="text-align:center;">© 2025 undelalune</p>
