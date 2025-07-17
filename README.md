# yt-dl-rust

Este projeto é um wrapper com interface gráfica (GUI) para o yt-dlp (binário .exe, Windows), utilizando Tauri (Rust) para backend e React + Typescript para frontend.

## Estrutura de Pastas e Arquivos

Abaixo está a descrição detalhada da estrutura atual do projeto e a função de cada pasta/arquivo:

```
/yt-dl-rust
├── .gitignore                # Arquivos e pastas ignorados pelo Git
├── .vscode/                  # Configurações do Visual Studio Code para o projeto
│   └── extensions.json       # Sugestão de extensões recomendadas para o VSCode
├── docs/                     # Documentação e registros de decisões (ADRs)
│   └── adr-2025-07-17-estrutura-de-pastas.md # Decisão sobre a estrutura de pastas
├── index.html                # Ponto de entrada HTML do frontend (usado pelo Vite)
├── package.json              # Configuração de dependências e scripts do frontend React
├── package-lock.json         # Lockfile das dependências do frontend
├── public/                   # Arquivos estáticos acessíveis pelo frontend
│   ├── tauri.svg             # Imagem/logo do Tauri
│   └── vite.svg              # Imagem/logo do Vite
├── src/                      # Código-fonte do frontend (React + Typescript)
│   ├── App.css               # Estilos principais do app React
│   ├── App.tsx               # Componente principal do app React
│   ├── assets/               # Imagens e outros assets do frontend
│   ├── main.tsx              # Ponto de entrada do React
│   └── vite-env.d.ts         # Tipagens globais do Vite
├── src-tauri/                # Backend do projeto (Rust + Tauri)
│   ├── .gitignore            # Ignora arquivos gerados pelo build do Rust
│   ├── Cargo.lock            # Lockfile das dependências Rust
│   ├── Cargo.toml            # Configuração das dependências Rust
│   ├── build.rs              # Script de build opcional do Rust
│   ├── capabilities/         # Recursos/capacidades customizadas do Tauri
│   ├── gen/                  # Código gerado automaticamente
│   ├── icons/                # Ícones do app Tauri
│   ├── src/                  # Código-fonte Rust do backend Tauri
│   ├── target/               # Saída de builds do Rust (gerado automaticamente)
│   └── tauri.conf.json       # Configuração do Tauri
├── tsconfig.json             # Configuração de TypeScript para o frontend
├── tsconfig.node.json        # Configuração de TypeScript para scripts de node
├── vite.config.ts            # Configuração do bundler Vite
```

## Descrição dos Principais Diretórios

- **.gitignore**: Define arquivos/pastas a serem ignorados pelo Git.
- **.vscode/**: Configurações e recomendações para quem usa VSCode.
- **docs/**: Documentação do projeto, incluindo decisões arquiteturais (ADRs).
- **index.html**: HTML base carregado pelo Vite/React.
- **package.json / package-lock.json**: Gerenciamento de dependências/scripts do frontend.
- **public/**: Imagens e arquivos estáticos do frontend.
- **src/**: Todo o código React/Typescript do frontend.
- **src-tauri/**: Backend em Rust, integrações com o sistema operacional, e onde ficarão binários auxiliares (ex: src-tauri/bin/yt-dlp.exe).
- **tsconfig.json / tsconfig.node.json**: Configuração do TypeScript.
- **vite.config.ts**: Configuração do bundler Vite.

## Observações
- Binários auxiliares como o yt-dlp.exe devem ser adicionados em `src-tauri/bin/` (crie essa pasta caso não exista).
- O backend Rust/Tauri será responsável por executar o yt-dlp.exe e repassar informações ao frontend React.
- Toda documentação técnica e decisões arquiteturais devem ser registradas em `docs/`.

---

Se precisar de mais detalhes sobre algum arquivo ou pasta, ou quiser adicionar novas seções ao README, é só avisar!
