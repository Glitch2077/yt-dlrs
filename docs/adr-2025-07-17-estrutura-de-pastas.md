# ADR: Estrutura de Pastas do Projeto yt-dl-rust

**Data:** 2025-07-17

## Contexto

O projeto yt-dl-rust tem como objetivo criar um wrapper com interface gráfica (GUI) para o yt-dlp (Python), utilizando Tauri (Rust) e React (Typescript). É fundamental definir uma estrutura de pastas clara e organizada para facilitar o desenvolvimento, manutenção e escalabilidade do projeto.

## Decisão

Adotaremos a seguinte estrutura inicial de pastas:

```
/yt-dl-rust
├── src-tauri/         # Código backend (Rust, Tauri)
│   └── bin/           # Binários auxiliares (ex: yt-dlp.exe)
├── src/               # Código frontend (React, Typescript)
├── public/            # Arquivos estáticos do frontend
├── docs/              # Documentação e ADRs
├── .github/           # Workflows e configurações do GitHub Actions
├── package.json       # Configuração do frontend React
├── tauri.conf.json    # Configuração do Tauri
├── README.md          # Documentação principal do projeto
```

### Justificativa
- **src-tauri/**: Mantém o padrão do Tauri para o backend em Rust, incluindo integrações e binários auxiliares.
- **src-tauri/bin/**: Centraliza binários necessários para o backend, como o yt-dlp.exe.
- **src/**: Onde ficará o código do frontend React/Typescript.
- **docs/**: Facilita o registro de decisões arquiteturais e documentação geral.
- **public/**: Padrão para arquivos estáticos do frontend.

## Consequências
- Organização clara entre backend, frontend e binários auxiliares.
- Facilidade para novos colaboradores entenderem a estrutura.
- Suporte para documentação contínua via ADRs.

---

*Esta decisão pode ser revisada conforme o projeto evoluir.*
