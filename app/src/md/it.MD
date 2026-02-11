<img src="logo.png" style="border-radius: 8px; margin-right:16px; margin-top: 20px; height: 89px;" align="right" alt="Free FPS Logo" />

- [Informazioni su questo strumento](#about-this-tool)
- [Pagina principale](#main-page)
- [Pagina di conversione](#processing-page)
- [Licenza](#license)
- [Software di terze parti](#3d-party-license)
- [Codice sorgente](#source-code)

---

<a id="about-this-tool"></a>
### Informazioni su questo strumento

Un programma di utilità per modificare la frequenza dei fotogrammi (FPS) dei file video utilizzando FFmpeg. A differenza degli editor video, non aggiunge effetti né altera il contenuto: regola solo la velocità di riproduzione e, se necessario, ricodifica anche l'audio.

Utile se lavori con più video girati a frequenze diverse che non possono essere combinati o montati senza interpolazione o perdita di fotogrammi.
Funziona anche come compressore video veloce: mantieni l’FPS originale e aumenta la compressione (ad es., CRF più alto o bitrate più basso) per ridurre le dimensioni del file.

Importante:
Il programma è destinato a utenti che comprendono cosa comporti la conversione degli FPS. Cambiare la frequenza dei fotogrammi influisce su velocità e durata del video. Lo strumento non modifica i file originali!

Questo programma è distribuito gratuitamente ("così com'è") sotto la licenza GNU General Public License v3.0 (GPL-3.0).
Se vuoi sostenere l'autore, puoi farlo qui:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; " alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; " alt="suppi Logo" />
</a>

<br>

---

<a id="main-page"></a>
### Pagina principale

- **Cartella di input** - scegli una cartella con i file da convertire. (In questo passaggio non puoi selezionare un singolo file, ma dopo potrai scegliere i video necessari.)
- **Cartella di output** - specifica la cartella in cui salvare i risultati. Se lasciata vuota, i file verranno salvati in una sottocartella creata automaticamente accanto agli originali.
- **FPS di destinazione** - frequenza fotogrammi di destinazione.
- **FPS personalizzato** - imposta un valore FPS personalizzato se quello desiderato non è nell'elenco.
- **Mantieni audio** - mantiene l'audio del file sorgente. L’audio può essere **rallentato o accelerato**, quindi l’opzione è disattivata per impostazione predefinita.
- **Bitrate audio** - imposta il bitrate se mantieni l'audio.
- **Qualità video personalizzata** - qualità del video in uscita (CRF). Per impostazione predefinita, l'app usa la modalità Auto, che stima il bitrate in base alle proprietà del video di origine.<br/>**IMPORTANTE:** La modalità Auto può produrre artefatti visibili (blocchi, banding, sfocatura) in scene complesse con movimenti rapidi, movimenti della fotocamera o dettagli fini. Può anche avere prestazioni inferiori con sorgenti già compresse o registrazioni dello schermo. Per video importanti o visivamente complessi, preferire la qualità **Alta** o **Massima**.<br/>
  Attenzione: un valore troppo basso produce scarsa qualità; troppo alto - file enormi. Maggiori info: [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
- **Accelerazione GPU** - usa la scheda grafica per una codifica video più veloce. Supporta NVIDIA (NVENC), AMD (AMF) e Intel (QuickSync). Può essere 3-5 volte più veloce della codifica CPU. Se non viene rilevata una GPU compatibile, l'opzione sarà disabilitata.
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

<a id="license"></a>
### Licenza
Free FPS è un software libero con licenza **GNU General Public License v3.0**. Sei libero di:
- Usare questo software per qualsiasi scopo
- Studiare e modificare il codice sorgente
- Ridistribuire copie
- Distribuire versioni modificate (sotto GPL v3)

Leggi la [licenza](FreeFPSLicense) del progetto.

---

<a id="3d-party-license"></a>
### Software di terze parti
Questo programma utilizza **FFmpeg**, anch’esso con licenza GPL v3.0.
- Leggi [FFmpeg Notice](FFmpegNotice)
- Leggi [FFmpeg License](FFmpegLicense)

---

<a id="source-code"></a>
### Codice sorgente

Il codice sorgente completo è disponibile su:  
https://github.com/undelalune/free-fps

<br/>

---

<p style="text-align:center;">Copyright © {{YEAR}} undelalune</p>
