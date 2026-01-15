<img src="logo.png" style="border-radius: 8px; margin-right:16px; margin-top: 20px; height: 89px;" align="right" alt="Free FPS Logo" />

- [O narzędziu](#about-this-tool)
- [Strona główna](#main-page)
- [Strona konwersji](#processing-page)
- [Licencja](#license)
- [Oprogramowanie stron trzecich](#3d-party-license)
- [Kod źródłowy](#source-code)

---

<a id="about-this-tool"></a>
### O narzędziu

Program narzędziowy do zmiany liczby klatek na sekundę (FPS) plików wideo z użyciem FFmpeg. W odróżnieniu od edytorów wideo, nie dodaje efektów ani nie zmienia treści - jedynie dostosowuje prędkość odtwarzania i, w razie potrzeby, ponownie koduje audio.

Przydatny, jeśli pracujesz z wieloma nagraniami wykonanymi z różnymi częstotliwościami klatek, których nie da się połączyć ani edytować bez interpolacji lub utraty klatek.
Może też służyć jako szybki kompresor wideo: zachowaj oryginalny FPS i zwiększ kompresję (np. wyższy CRF lub niższy bitrate), aby zmniejszyć rozmiar pliku.

Ważne:
Program jest przeznaczony dla użytkowników rozumiejących działanie konwersji FPS. Zmiana częstotliwości klatek wpływa na prędkość i długość wideo. Narzędzie nie modyfikuje oryginalnych plików!

Program jest rozpowszechniany bezpłatnie ("As Is") na licencji GNU General Public License v3.0 (GPL-3.0).
Jeśli chcesz wesprzeć autora, możesz to zrobić tutaj:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Przejdź do buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; " alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Przejdź do suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; " alt="suppi Logo" />
</a>

<br>

---

<a id="main-page"></a>
### Strona główna

- **Folder wejściowy** - wybierz folder z plikami do konwersji. (Na tym etapie nie można wybrać pojedynczego pliku, później będziesz mógł/mogła zaznaczyć potrzebne wideo.)
- **Folder wyjściowy** - określ folder do zapisu wyników. Jeśli pozostawisz puste, pliki zostaną zapisane w automatycznie utworzonym podfolderze obok oryginałów.
- **Docelowy FPS** - docelowa liczba klatek.
- **Własny FPS** - ustaw własną wartość FPS, jeśli żądanej nie ma na liście.
- **Zachowaj dźwięk** - zachowuje dźwięk z pliku źródłowego. Dźwięk może być **spowolniony lub przyspieszony**, dlatego domyślnie opcja jest wyłączona.
- **Bitrate audio** - ustaw bitrate, jeśli zachowujesz audio.
- **Własna jakość wideo** - jakość wyjściowego wideo (CRF). Domyślnie aplikacja używa trybu Auto, który szacuje bitrate na podstawie właściwości wideo źródłowego.<br/>**WAŻNE:** Tryb Auto może powodować widoczne artefakty (bloki, paski, rozmycie) w złożonych scenach z szybkim ruchem, szybkim ruchem kamery lub drobnymi szczegółami. Może też działać gorzej z już skompresowanymi źródłami lub nagraniami ekranu. Dla ważnych lub wizualnie złożonych filmów wybierz jakość **Wysoką** lub **Maksymalną**.<br/>
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

<a id="license"></a>
### Licencja

Free FPS to wolne oprogramowanie licencjonowane na podstawie \*\*GNU General Public License v3.0\*\*. Masz prawo do:
- Używania tego oprogramowania w jakimkolwiek celu
- Badania i modyfikowania kodu źródłowego
- Ponownego rozpowszechniania kopii
- Dystrybuowania zmodyfikowanych wersji (na warunkach GPL v3)

Przeczytaj [licencję](FreeFPSLicense) projektu.

---

<a id="3d-party-license"></a>
### Oprogramowanie stron trzecich

Ten program wykorzystuje **FFmpeg**, który także jest licencjonowany na warunkach GPL v3.0.
- Przeczytaj [FFmpeg Notice](FFmpegNotice)
- Przeczytaj [FFmpeg License](FFmpegLicense)

---

<a id="source-code"></a>
### Kod źródłowy

Pełny kod źródłowy jest dostępny pod adresem: https://github.com/undelalune/free-fps

<br/>

---

<p style="text-align:center;">Copyright © {{YEAR}} undelalune</p>
