<img src="logo.png" style="border-radius: 8px; margin-right:16px; margin-top: 20px; height: 89px;" align="right" alt="Free FPS Logo" />

- [Sobre esta ferramenta](#about-this-tool)
- [Página principal](#main-page)
- [Página de conversão](#processing-page)
- [Licença](#license)
- [Software de terceiros](#3d-party-license)
- [Código-fonte](#source-code)

---

<a id="about-this-tool"></a>
### Sobre esta ferramenta

Um utilitário para alterar a taxa de fotogramas (FPS) de ficheiros de vídeo usando o FFmpeg. Ao contrário dos editores de vídeo, não adiciona efeitos nem altera o conteúdo: apenas ajusta a velocidade de reprodução e, se necessário, recodifica também o áudio.

Útil se trabalha com vários vídeos gravados a diferentes taxas de fotogramas que não podem ser combinados ou editados sem interpolação ou perda de fotogramas.
Também funciona como um compressor de vídeo rápido: mantenha o FPS original e aumente a compressão (p. ex., CRF mais alto ou bitrate mais baixo) para reduzir o tamanho do arquivo.

Importante:
O programa destina-se a utilizadores que entendem o que faz a conversão de FPS. Alterar a taxa de fotogramas afeta a velocidade e a duração do vídeo. A ferramenta não modifica os ficheiros originais!

Este programa é distribuído gratuitamente ("tal como está") sob a GNU General Public License v3.0 (GPL-3.0).
Se quiser apoiar o autor, pode fazê-lo aqui:

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

- **Pasta de entrada** - escolha uma pasta com os ficheiros a converter. (Nesta etapa não pode selecionar um único ficheiro, mas depois poderá escolher os vídeos de que precisa.)
- **Pasta de saída** - indique a pasta onde guardar os resultados. Se deixar em branco, os ficheiros serão guardados numa subpasta criada automaticamente junto aos originais.
- **FPS de destino** - taxa de fotogramas pretendida.
- **FPS personalizado** - defina o seu próprio valor de FPS se o desejado não estiver na lista.
- **Manter áudio** - mantém o áudio do ficheiro de origem. O som pode ser **desacelerado ou acelerado**, por isso a opção está desativada por padrão.
- **Bitrate de áudio** - defina o bitrate se mantiver o áudio.
- **Qualidade de vídeo personalizada** - qualidade do vídeo de saída (CRF). Por defeito, a aplicação usa o modo Auto, que estima o bitrate com base nas propriedades do vídeo de origem.<br/>**IMPORTANTE:** O modo Auto pode produzir artefactos visíveis (blocos, bandas, desfoque) em cenas complexas com movimento rápido, movimento de câmara ou detalhes finos. Também pode ter desempenho inferior com fontes já comprimidas ou gravações de ecrã. Para vídeos importantes ou visualmente complexos, prefira a qualidade **Alta** ou **Máxima**.<br/>
  Atenção: um valor demasiado baixo resulta em má qualidade; demasiado alto - ficheiros enormes. Mais: [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
- **Aceleração por GPU** - usa a placa gráfica para codificação de vídeo mais rápida. Suporta NVIDIA (NVENC), AMD (AMF) e Intel (QuickSync). Pode ser 3-5x mais rápida que a codificação por CPU. Se nenhuma GPU compatível for detetada, a opção será desativada.
- **Limite de CPU** - limite o uso da CPU. Pode reduzir ligeiramente a carga, mas não é garantido.

---

<a id="processing-page"></a>
### Página de conversão

- Use as caixas de seleção para escolher quais vídeos converter. Todos vêm selecionados por defeito.
- **Reanalisar pasta** - voltar a analisar a pasta e atualizar a lista de ficheiros.
- Após iniciar o conversor, pode cancelar o processo a qualquer momento.
- A conversão de vídeo consome muitos recursos e pode demorar. O progresso e o estado são mostrados no ecrã.

> Se a conversão de algum vídeo terminar com erro, é improvável que uma nova tentativa tenha sucesso.
> É possível que o FFmpeg não tenha conseguido ler os dados necessários do ficheiro.

---

<a id="license"></a>
### Licença
Free FPS é um software livre sob a **GNU General Public License v3.0**. Você pode:
- Usar este software para qualquer finalidade
- Estudar e modificar o código-fonte
- Redistribuir cópias
- Distribuir versões modificadas (sob GPL v3)

Leia a [licença](FreeFPSLicense) do projeto.

---

<a id="3d-party-license"></a>
### Software de terceiros
Este programa usa **FFmpeg**, que também está licenciado sob GPL v3.0.
- Leia [FFmpeg Notice](FFmpegNotice)
- Leia [FFmpeg License](FFmpegLicense)

---

<a id="source-code"></a>
### Código-fonte

O código-fonte completo está disponível em:  
https://github.com/undelalune/free-fps

<br/>

---

<p style="text-align:center;">Copyright © {{YEAR}} undelalune</p>
