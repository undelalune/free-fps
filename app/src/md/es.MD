<img src="logo.png" style="border-radius: 8px; margin-right:16px; margin-top: 20px; height: 89px;" align="right" alt="Free FPS Logo" />

- [Acerca de esta herramienta](#about-this-tool)
- [Página principal](#main-page)
- [Página de conversión](#processing-page)
- [Licencia](#license)
- [Software de terceros](#3d-party-license)
- [Código fuente](#source-code)

---

<a id="about-this-tool"></a>
### Acerca de esta herramienta

Una utilidad para cambiar la tasa de fotogramas (FPS) de archivos de video usando FFmpeg. A diferencia de los editores de video, no añade efectos ni altera el contenido: solo ajusta la velocidad de reproducción y, si es necesario, vuelve a codificar el audio.

Útil si trabajas con varios videos grabados a diferentes tasas de fotogramas que no pueden combinarse ni editarse sin interpolación o pérdida de cuadros.
También sirve como un compresor de vídeo rápido: mantén el FPS original y aumenta la compresión (p. ej., CRF más alto o bitrate más bajo) para reducir el tamaño del archivo.

Importante:
El programa está destinado a usuarios que entienden qué hace la conversión de FPS. Cambiar la tasa de fotogramas afecta la velocidad y la duración del video. ¡La herramienta no modifica los archivos originales!

Este programa se distribuye de forma gratuita ("tal cual") bajo la Licencia Pública General de GNU v3.0 (GPL-3.0).
Si quieres apoyar al autor, puedes hacerlo aquí:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; " alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; " alt="suppi Logo" />
</a>

<br>

---

<a id="main-page"></a>
### Página principal

- **Carpeta de entrada** - elige una carpeta con los archivos a convertir. (En este paso no puedes seleccionar un solo archivo, pero luego podrás elegir los videos que necesites.)
- **Carpeta de salida** - especifica la carpeta donde guardar los resultados. Si la dejas vacía, los archivos se guardarán en una subcarpeta creada automáticamente junto a los originales.
- **FPS de destino** - tasa de fotogramas objetivo.
- **FPS personalizado** - establece tu propio valor de FPS si el deseado no está en la lista.
- **Mantener audio** - mantiene el audio del archivo de origen. El sonido puede **ralentizarse o acelerarse**, por eso la opción está desactivada por defecto.
- **Bitrate de audio** - establece el bitrate si mantienes el audio.
- **Calidad de video personalizada** - calidad del video de salida (CRF). Por defecto, la app usa el modo Auto, que estima el bitrate según las propiedades del video de origen.<br/>**IMPORTANTE:** El modo Auto puede producir artefactos visibles (bloques, bandas, desenfoque) en escenas complejas con movimientos rápidos, movimiento de cámara o detalles finos. También puede funcionar mal con fuentes ya comprimidas o grabaciones de pantalla. Para videos importantes o visualmente complejos, prefiera la calidad **Alta** o **Máxima**.<br/>
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

<a id="license"></a>
### Licencia
Free FPS es software libre bajo la **GNU General Public License v3.0**. Usted puede:
- Usar este software para cualquier propósito
- Estudiar y modificar el código fuente
- Redistribuir copias
- Distribuir versiones modificadas (bajo GPL v3)

Lea la [licencia](FreeFPSLicense) del proyecto.

---

<a id="3d-party-license"></a>
### Software de terceros
Este programa utiliza **FFmpeg**, que también está licenciado bajo GPL v3.0.
- Lea [FFmpeg Notice](FFmpegNotice)
- Lea [FFmpeg License](FFmpegLicense)

---

<a id="source-code"></a>
### Código fuente

El código fuente completo está disponible en:  
https://github.com/undelalune/free-fps

<br/>

---

<p style="text-align:center;">Copyright © {{YEAR}} undelalune</p>
