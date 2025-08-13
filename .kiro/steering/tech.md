# Technology Steering

## Tech Stack

### Desktop Application (Primary)
- **Framework**: Tauri (Rust + Web Technologies)
- **Backend Language**: Rust
- **Database**: SQLite (embedded)
- **Frontend**: HTML/CSS/TypeScript
- **UI Framework**: React (with TypeScript)
- **State Management**: Zustand / Jotai
- **Build Tool**: Vite + Tauri CLI
- **Styling**: Tailwind CSS / CSS Modules

### Backend (Rust)
- **Async Runtime**: Tokio
- **Database ORM**: sqlx / diesel
- **Serialization**: serde
- **Error Handling**: anyhow / thiserror
- **Logging**: tracing / env_logger
- **Testing**: built-in test framework

### Infrastructure
- **Version Control**: Git, GitHub
- **CI/CD**: GitHub Actions
- **Package Manager**: Cargo (Rust), pnpm (Frontend)
- **Code Signing**: Windows Code Signing Certificate
- **Auto-Update**: Tauri Updater

## Development Principles
- Type Safety: TypeScriptを全面的に使用
- Code Quality: ESLint, Prettierによる自動フォーマット
- Testing: Jest, React Testing Library
- Documentation: JSDoc, README
- Version Control: Git, GitHub

## Architecture Patterns
- Clean Architecture
- Repository Pattern
- Dependency Injection
- Event-Driven Architecture

## Security Standards
- HTTPS通信
- JWT認証
- Input Validation
- SQL Injection対策
- XSS対策
- CSRF対策

## Performance Goals
- アプリケーション起動時間: < 1秒
- メモリ使用量: < 50MB (アイドル時)
- CPU使用率: < 2% (アイドル時)
- データベースクエリ応答: < 10ms
- UIレスポンス時間: < 100ms
- バイナリサイズ: < 20MB