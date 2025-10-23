- [Acerca de esta herramienta](#about-this-tool) <img src="logo.png" style="float: right; margin-right:24px; margin-top:12px; height: 89px; " alt="Free FPS Logo" />
- [¿Cómo empezar?](#how-to-start)
- [Página de ajustes](#settings-page)
- [Página principal](#main-page)
- [Página de conversión](#processing-page)

---

<a id="about-this-tool"></a>
### Acerca de esta herramienta

Una utilidad para cambiar la tasa de fotogramas (FPS) de archivos de video usando FFmpeg. A diferencia de los editores de video, no añade efectos ni altera el contenido: solo ajusta la velocidad de reproducción y, si es necesario, vuelve a codificar el audio.

Útil si trabajas con varios videos grabados a diferentes tasas de fotogramas que no pueden combinarse ni editarse sin interpolación o pérdida de cuadros.

Importante:
El programa está destinado a usuarios que entienden qué hace la conversión de FPS. Cambiar la tasa de fotogramas afecta la velocidad y la duración del video. ¡La herramienta no modifica los archivos originales!

Este programa se distribuye de forma gratuita ("tal cual") bajo la licencia MIT.
Si quieres apoyar al autor, puedes hacerlo aquí:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; " alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; " alt="suppi Logo" />
</a>

<br>

---

<a id="how-to-start"></a>
### ¿Cómo empezar?

Al iniciar, la app comprueba automáticamente si los paquetes ffmpeg y ffprobe están disponibles en el `PATH` del sistema y propondrá usarlos.
Si no están presentes, descarga FFmpeg del sitio oficial y extrae el archivo.<br>
Windows: <a href="https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" target="_blank" rel="noopener" title="Download ffmpeg/ffprobe archive">ffmpeg/ffprobe</a><br>
macOS: <a href="https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip" target="_blank" rel="noopener" title="Download ffmpeg">ffmpeg</a> |
<a href="https://evermeet.cx/ffmpeg/ffprobe-8.0.zip" target="_blank" rel="noopener" title="Download ffprobe archive">ffprobe</a>

No es necesaria la instalación en el sistema: basta con proporcionar las rutas a los ejecutables en los ajustes de la app.

- **Windows**: `bin/ffmpeg.exe` y `bin/ffprobe.exe`
- **macOS**: `ffmpeg` y `ffprobe` (en la raíz de la carpeta)

Después de eso, puedes empezar a trabajar.

---

<a id="settings-page"></a>
### Página de ajustes

- **Ruta a ffmpeg** - ruta al ejecutable `ffmpeg`. Campo obligatorio.
- **Ruta a ffprobe** - ruta a `ffprobe`. Opcional, pero recomendado para un funcionamiento más estable.
- **Usar versión instalada (ffmpeg / ffprobe)** - se habilita si se encuentran versiones instaladas.
- **Idioma** - selecciona el idioma de la interfaz.
- **Tema** - elige el tema de color de la app.
- **Restablecer ajustes** - restablece los datos guardados (solo los valores de los campos).

La app guarda automáticamente tus ajustes: no necesitas volver a introducirlos en cada inicio.

---

<a id="main-page"></a>
### Página principal

- **Carpeta de entrada** - elige una carpeta con los archivos a convertir. (En este paso no puedes seleccionar un solo archivo, pero luego podrás elegir los videos que necesites.)
- **Carpeta de salida** - especifica la carpeta donde guardar los resultados. Si la dejas vacía, los archivos se guardarán en una subcarpeta creada automáticamente junto a los originales.
- **FPS de destino** - tasa de fotogramas objetivo.
- **FPS personalizado** - establece tu propio valor de FPS si el deseado no está en la lista.
- **Mantener audio** - mantiene el audio del archivo de origen. El sonido puede **ralentizarse o acelerarse**, por eso la opción está desactivada por defecto.
- **Bitrate de audio** - establece el bitrate si mantienes el audio.
- **Calidad de video personalizada** - calidad del video de salida (CRF). De forma predeterminada, la app intenta conservar la calidad de origen: es lo recomendado.
  Precaución: un valor demasiado bajo resulta en mala calidad; demasiado alto - archivos enormes. Más: [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
- **Límite de CPU** - limita el uso de CPU. Puede reducir ligeramente la carga, pero no está garantizado.

---

<a id="processing-page"></a>
### Página de conversión

- Usa las casillas de verificación para seleccionar qué videos convertir. Todos están seleccionados por defecto.
- **Volver a escanear carpeta** - volver a escanear la carpeta y refrescar la lista de archivos.
- Después de iniciar el conversor, puedes cancelar el proceso en cualquier momento.
- La conversión de video requiere muchos recursos y puede llevar tiempo. El progreso y el estado se muestran en pantalla.

> Si la conversión de algún video termina con error, es poco probable que un reintento tenga éxito.
> Es posible que FFmpeg no haya podido leer los datos necesarios del archivo.

---

<p style="text-align:center;">© 2025 undelalune</p>
