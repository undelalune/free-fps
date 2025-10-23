<img src="logo.png" style="border-radius: 8px; float: right; margin-right:16px; margin-top:12px; height: 89px;" alt="Free FPS Logo" />

- [Informazioni su questo strumento](#about-this-tool)
- [Come iniziare?](#how-to-start)
- [Pagina impostazioni](#settings-page)
- [Pagina principale](#main-page)
- [Pagina di conversione](#processing-page)

---

<a id="about-this-tool"></a>
### Informazioni su questo strumento

Un programma di utilità per modificare la frequenza dei fotogrammi (FPS) dei file video utilizzando FFmpeg. A differenza degli editor video, non aggiunge effetti né altera il contenuto: regola solo la velocità di riproduzione e, se necessario, ricodifica anche l'audio.

Utile se lavori con più video girati a frequenze diverse che non possono essere combinati o montati senza interpolazione o perdita di fotogrammi.

Importante:
Il programma è destinato a utenti che comprendono cosa comporti la conversione degli FPS. Cambiare la frequenza dei fotogrammi influisce su velocità e durata del video. Lo strumento non modifica i file originali!

Questo programma è distribuito gratuitamente ("così com'è") sotto licenza MIT.
Se vuoi sostenere l'autore, puoi farlo qui:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; " alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; " alt="suppi Logo" />
</a>

<br>

---

<a id="how-to-start"></a>
### Come iniziare?

All'avvio, l'app verifica automaticamente se i pacchetti ffmpeg e ffprobe sono disponibili nel `PATH` di sistema e propone di usarli.
Se non sono presenti, scarica FFmpeg dal sito ufficiale ed estrai l'archivio.<br>
Windows: <a href="https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" target="_blank" rel="noopener" title="Download ffmpeg/ffprobe archive">ffmpeg/ffprobe</a><br>
macOS: <a href="https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip" target="_blank" rel="noopener" title="Download ffmpeg">ffmpeg</a> |
<a href="https://evermeet.cx/ffmpeg/ffprobe-8.0.zip" target="_blank" rel="noopener" title="Download ffprobe archive">ffprobe</a>

L'installazione nel sistema non è necessaria: è sufficiente indicare i percorsi degli eseguibili nelle impostazioni dell'app.

- **Windows**: `bin/ffmpeg.exe` e `bin/ffprobe.exe`
- **macOS**: `ffmpeg` e `ffprobe` (nella root della cartella)

Dopodiché puoi iniziare a lavorare.

---

<a id="settings-page"></a>
### Pagina impostazioni

- **Percorso ffmpeg** - percorso all'eseguibile `ffmpeg`. Campo obbligatorio.
- **Percorso ffprobe** - percorso a `ffprobe`. Facoltativo, ma consigliato per un funzionamento più stabile.
- **Usa versione installata (ffmpeg / ffprobe)** - disponibile se vengono trovate versioni installate.
- **Lingua** - seleziona la lingua dell'interfaccia.
- **Tema** - scegli il tema colore dell'app.
- **Reimposta impostazioni** - ripristina i valori predefiniti (solo i valori dei campi).

L'app salva automaticamente le impostazioni: non è necessario reinserirle a ogni avvio.

---

<a id="main-page"></a>
### Pagina principale

- **Cartella di input** - scegli una cartella con i file da convertire. (In questo passaggio non puoi selezionare un singolo file, ma dopo potrai scegliere i video necessari.)
- **Cartella di output** - specifica la cartella in cui salvare i risultati. Se lasciata vuota, i file verranno salvati in una sottocartella creata automaticamente accanto agli originali.
- **FPS di destinazione** - frequenza fotogrammi di destinazione.
- **FPS personalizzato** - imposta un valore FPS personalizzato se quello desiderato non è nell'elenco.
- **Mantieni audio** - mantiene l'audio del file sorgente. L’audio può essere **rallentato o accelerato**, quindi l’opzione è disattivata per impostazione predefinita.
- **Bitrate audio** - imposta il bitrate se mantieni l'audio.
- **Qualità video personalizzata** - qualità del video in uscita (CRF). Per impostazione predefinita, l'app cerca di preservare la qualità di origine: è consigliato.
  Attenzione: un valore troppo basso produce scarsa qualità; troppo alto - file enormi. Maggiori info: [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
- **Limite CPU** - limita l'uso della CPU. Può ridurre leggermente il carico, ma non è garantito.

---

<a id="processing-page"></a>
### Pagina di conversione

- Usa le caselle di spunta per selezionare quali video convertire. Tutti sono selezionati per impostazione predefinita.
- **Scansiona di nuovo** - analizza di nuovo la cartella e aggiorna l'elenco dei file.
- Dopo l'avvio del convertitore, puoi annullare il processo in qualsiasi momento.
- La conversione video è intensiva in termini di risorse e può richiedere tempo. Avanzamento e stato sono mostrati sullo schermo.

> Se la conversione di un video termina con un errore, è improbabile che un nuovo tentativo abbia successo.
> È possibile che FFmpeg non sia riuscito a leggere i dati necessari dal file.

---

<p style="text-align:center;">© 2025 undelalune</p>
