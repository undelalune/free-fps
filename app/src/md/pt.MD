- [Sobre esta ferramenta](#about-this-tool) <img src="logo.png" style="float: right; margin-right:24px; margin-top:12px; height: 89px; " alt="Free FPS Logo" />
- [Como começar?](#how-to-start)
- [Página de definições](#settings-page)
- [Página principal](#main-page)
- [Página de conversão](#processing-page)

---

<a id="about-this-tool"></a>
### Sobre esta ferramenta

Um utilitário para alterar a taxa de fotogramas (FPS) de ficheiros de vídeo usando o FFmpeg. Ao contrário dos editores de vídeo, não adiciona efeitos nem altera o conteúdo: apenas ajusta a velocidade de reprodução e, se necessário, recodifica também o áudio.

Útil se trabalha com vários vídeos gravados a diferentes taxas de fotogramas que não podem ser combinados ou editados sem interpolação ou perda de fotogramas.

Importante:
O programa destina-se a utilizadores que entendem o que faz a conversão de FPS. Alterar a taxa de fotogramas afeta a velocidade e a duração do vídeo. A ferramenta não modifica os ficheiros originais!

Este programa é distribuído gratuitamente ("tal como está") sob a licença MIT.
Se quiser apoiar o autor, pode fazê-lo aqui:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; background-color: #eee;" alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px; background-color: #eee;" alt="suppi Logo" />
</a>

<br>

---

<a id="how-to-start"></a>
### Como começar?

Ao iniciar, a aplicação verifica automaticamente se os pacotes ffmpeg e ffprobe estão disponíveis no `PATH` do sistema e irá propor utilizá-los.
Se não estiverem presentes, transfira o FFmpeg do site oficial e extraia o arquivo.<br>
Windows: <a href="https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" target="_blank" rel="noopener" title="Download ffmpeg/ffprobe archive">ffmpeg/ffprobe</a><br>
macOS: <a href="https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip" target="_blank" rel="noopener" title="Download ffmpeg">ffmpeg</a> |
<a href="https://evermeet.cx/ffmpeg/ffprobe-8.0.zip" target="_blank" rel="noopener" title="Download ffprobe archive">ffprobe</a>

Não é necessária instalação no sistema - basta indicar os caminhos para os executáveis nas definições da aplicação.

- **Windows**: `bin/ffmpeg.exe` e `bin/ffprobe.exe`
- **macOS**: `ffmpeg` e `ffprobe` (na raiz da pasta)

Depois disso, pode começar a trabalhar.

---

<a id="settings-page"></a>
### Página de definições

- **Caminho para o ffmpeg** - caminho para o executável `ffmpeg`. Este campo é obrigatório.
- **Caminho para o ffprobe** - caminho para `ffprobe`. Opcional, mas recomendado para funcionamento mais estável.
- **Usar versão instalada (ffmpeg / ffprobe)** - fica disponível se forem encontradas versões instaladas.
- **Idioma** - selecione o idioma da interface.
- **Tema** - escolha o tema de cores da aplicação.
- **Repor definições** - repõe os valores predefinidos (apenas os valores dos campos).

A aplicação guarda automaticamente as suas definições - não precisa de voltar a introduzi-las em cada arranque.

---

<a id="main-page"></a>
### Página principal

- **Pasta de entrada** - escolha uma pasta com os ficheiros a converter. (Nesta etapa não pode selecionar um único ficheiro, mas depois poderá escolher os vídeos de que precisa.)
- **Pasta de saída** - indique a pasta onde guardar os resultados. Se deixar em branco, os ficheiros serão guardados numa subpasta criada automaticamente junto aos originais.
- **FPS de destino** - taxa de fotogramas pretendida.
- **FPS personalizado** - defina o seu próprio valor de FPS se o desejado não estiver na lista.
- **Manter áudio** - mantém o áudio do ficheiro de origem. O som pode ser **desacelerado ou acelerado**, por isso a opção está desativada por padrão.
- **Bitrate de áudio** - defina o bitrate se mantiver o áudio.
- **Qualidade de vídeo personalizada** - qualidade do vídeo de saída (CRF). Por defeito, a aplicação tenta preservar a qualidade de origem - recomendado.
  Atenção: um valor demasiado baixo resulta em má qualidade; demasiado alto - ficheiros enormes. Mais: [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
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

<p style="text-align:center;">© 2025 undelalune</p>
