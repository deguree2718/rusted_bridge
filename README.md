# EN
## Rusted Bridge
A rust lib for automatizing printing documents
- Automatically send documents to the queue using the lib
- No need for user confirmation of settings for the printer (some configuration may be needed on first use, WIP)

crates being used:

- [ipp.rs - by Dmitry Pankratov](https://github.com/ancwrd1/ipp.rs)

### Halt
the project is halted, due to the deadline of the article closing in, and me haven't found any solutions to the problems at hand  
the branch master will be synchronized and all other branches cleansed, for purposes of keeping a single version of it, until I have more time (or better skills)

## Development Log

### winprint
Tried using this crate for a while, worked fine on Windows, doens't work on Unix systems (obviously), but also doesn't work with WASM and WASI
 - Needs PDFium or other crate to properly print certain files types
 - Windows only
 - Uncompatible with WASM

### printers
This crate was used for both Windows and Unix systems, but didn't compile to WASM nor WASI
- Works on multiple OSs
- System needs to be configured to English
- printing was harder to implement
- Uncompatible with WASM

### ipp
The ipp.rs crate worked well for Unix systems, and wasn't tested on Windows systems, it uses pure IPP (Internet Printing Protocol) for communication with the printers, however, many errors arose from trying to compile into WASM or WASI
- Works well with CUPS
- Prints various documents
- For printers with different data types, need to speficy the document type
- Doesn't compile to WASI

[//]: # (crates being reviewed for use)

[//]: # (- [rsspy - by Rustin]&#40;https://crates.io/crates/rsspy&#41;)

[//]: # (- [cups-sys - by Christian Legnitto]&#40;https://crates.io/crates/cups-sys&#41;)

# PT-Br
## Ponte Enferrujada
Uma biblioteca em rust para automatizar a impressão de documentos
- Automaticamente envie documentos para a fila de impressão
- Sem a necessidade do usuário confirmar as configurações para a impressora (um certo nível de configuração poderá ser necessário no primeiro uso WIP)

crates sendo usadas:

- [ipp.rs - by Dmitry Pankratov](https://github.com/ancwrd1/ipp.rs)

### Pausa
o projeto está pausado, por conta do prazo para o TCC estar se aproximando, e eu não ter encontrado soluções para os problemas que encontrei
a branch master vai ser sincronizada com as últimas mudanças, e todas as outras branches serão limpas, para manter uma versão única do projeto, até que eu tenha mais tempo (ou mais conhecimento)

## Log de Desenvolvimento

### winprint
Tentei usar essa crate por um tempo, funcionou bem no Windows, mas não funciona em sistemas Unix (por motivos óbvios), e também, não compilou para WASM nem para WASI
 - Precisa da crate PDFium ou semelhante para imprimir certos tipos de arquivos
 - Apenas Windows
 - Incompatível com WASM

### printers
Essa crate foi usada tanto em Windows quanto em sistemas Unix, mas não compilou para WASM ou WASI
- Funciona em múltiplos SOs
- Sistema precisa estar em Inglês
- Impressão é mais complexa de ser implementada (se comparado a winprint ou ipp)
- Incompatível com WASM

### ipp
A crate funcionou bem pra sistemas Unix, não foi testada em Windows, ela usa IPP puro (Internet Printing Protocol, ou Protocolo de Impressão pela Internet), para se comunicar com as impressoras, no entanto, diversos erros apareceram ao tentar compilar para WASM e para WASI
- Funciona bem com CUPS
- Imprimi diversos documentos
- Para impressoras com tipos de arquivo diferentes é necessário especificação
- Não compila para WASI

[//]: # (crates em revisão para uso)

[//]: # (- [rsspy - by Rustin]&#40;https://crates.io/crates/rsspy&#41;)

[//]: # (- [cups-sys - by Christian Legnitto]&#40;https://crates.io/crates/cups-sys&#41;)
