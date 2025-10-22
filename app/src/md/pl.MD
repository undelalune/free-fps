- [O narzędziu](#about-this-tool) <img src="logo.png" style="float: right; margin-right:24px; margin-top:12px; height: 89px; " alt="Free FPS Logo" />
- [Jak zacząć?](#how-to-start)
- [Ustawienia](#settings-page)
- [Strona główna](#main-page)
- [Strona konwersji](#processing-page)

---

<a id="about-this-tool"></a>
### O narzędziu

Program narzędziowy do zmiany liczby klatek na sekundę (FPS) plików wideo z użyciem FFmpeg. W odróżnieniu od edytorów wideo, nie dodaje efektów ani nie zmienia treści - jedynie dostosowuje prędkość odtwarzania i, w razie potrzeby, ponownie koduje audio.

Przydatny, jeśli pracujesz z wieloma nagraniami wykonanymi z różnymi częstotliwościami klatek, których nie da się połączyć ani edytować bez interpolacji lub utraty klatek.

Ważne:
Program jest przeznaczony dla użytkowników rozumiejących działanie konwersji FPS. Zmiana częstotliwości klatek wpływa na prędkość i długość wideo. Narzędzie nie modyfikuje oryginalnych plików!

Program jest rozpowszechniany bezpłatnie ("As Is") na licencji MIT.
Jeśli chcesz wesprzeć autora, możesz to zrobić tutaj:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Przejdź do buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; background-color: #eee;" alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Przejdź do suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; background-color: #eee;" alt="suppi Logo" />
</a>

<br>

---

<a id="how-to-start"></a>
### Jak zacząć?

Przy uruchomieniu aplikacja automatycznie sprawdza, czy pakiety ffmpeg i ffprobe są dostępne w systemowej zmiennej `PATH` i zaproponuje ich użycie.
Jeśli nie są obecne, pobierz FFmpeg z oficjalnej strony i rozpakuj archiwum.<br>
Windows: <a href="https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" target="_blank" rel="noopener" title="Pobierz archiwum ffmpeg/ffprobe">ffmpeg/ffprobe</a><br>
macOS: <a href="https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip" target="_blank" rel="noopener" title="Pobierz ffmpeg">ffmpeg</a> |
<a href="https://evermeet.cx/ffmpeg/ffprobe-8.0.zip" target="_blank" rel="noopener" title="Pobierz archiwum ffprobe">ffprobe</a>

Instalacja w systemie nie jest wymagana - wystarczy podać ścieżki do plików wykonywalnych w ustawieniach aplikacji.

- **Windows**: `bin/ffmpeg.exe` i `bin/ffprobe.exe`
- **macOS**: `ffmpeg` i `ffprobe` (w katalogu głównym folderu)

Po tym możesz rozpocząć pracę.

---

<a id="settings-page"></a>
### Ustawienia

- **Ścieżka do ffmpeg** - ścieżka do pliku wykonywalnego `ffmpeg`. Pole wymagane.
- **Ścieżka do ffprobe** - ścieżka do `ffprobe`. Opcjonalne, ale zalecane dla stabilniejszego działania.
- **Użyj zainstalowanego (ffmpeg / ffprobe)** - dostępne, jeśli wykryto zainstalowane wersje.
- **Język** - wybór języka interfejsu.
- **Motyw** - wybór motywu kolorystycznego aplikacji.
- **Resetuj ustawienia** - reset zapisanych danych (tylko wartości pól formularza).

Aplikacja automatycznie zapisuje ustawienia - nie trzeba wprowadzać ich ponownie przy każdym uruchomieniu.

---

<a id="main-page"></a>
### Strona główna

- **Folder wejściowy** - wybierz folder z plikami do konwersji. (Na tym etapie nie można wybrać pojedynczego pliku, później będziesz mógł/mogła zaznaczyć potrzebne wideo.)
- **Folder wyjściowy** - określ folder do zapisu wyników. Jeśli pozostawisz puste, pliki zostaną zapisane w automatycznie utworzonym podfolderze obok oryginałów.
- **Docelowy FPS** - docelowa liczba klatek.
- **Własny FPS** - ustaw własną wartość FPS, jeśli żądanej nie ma na liście.
- **Zachowaj dźwięk** - zachowuje dźwięk z pliku źródłowego. Dźwięk może być **spowolniony lub przyspieszony**, dlatego domyślnie opcja jest wyłączona.
- **Bitrate audio** - ustaw bitrate, jeśli zachowujesz audio.
- **Własna jakość wideo** - jakość wyjściowego wideo (CRF). Domyślnie aplikacja stara się zachować jakość źródłową - to zalecane.
  Uwaga: zbyt niska wartość da słabą jakość; zbyt wysoka - ogromne pliki. Więcej: [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
- **Limit CPU** - ogranicz użycie procesora. Może nieznacznie zmniejszyć obciążenie, ale nie jest gwarantowane.

---

<a id="processing-page"></a>
### Strona konwersji

- Użyj pól wyboru, aby wskazać, które wideo konwertować. Domyślnie zaznaczone są wszystkie.
- **Przeskanuj ponownie** - ponownie przeskanuj folder i odśwież listę plików.
- Po uruchomieniu konwertera możesz w każdej chwili anulować proces.
- Konwersja wideo jest zasobożerna i może zająć czas. Postęp i status są pokazywane na ekranie.

> Jeśli konwersja dowolnego wideo zakończy się błędem, mało prawdopodobne, że ponowna próba się powiedzie.
> Możliwe, że FFmpeg nie mógł odczytać wymaganych danych z pliku.

---

<p style="text-align:center;">© 2025 undelalune</p>
