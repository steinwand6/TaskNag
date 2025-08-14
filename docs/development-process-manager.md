# Development Process Manager

TaskNagの開発環境を適切に管理するためのプロセス管理システムです。

## 問題の背景

従来の `npm run tauri dev` では、Windowsでプロセス終了時に子プロセス（npm、Viteサーバー）が適切にクリーンアップされず、以下の問題が発生していました：

- ポート5173が前回のViteサーバープロセスに占有される
- 孤立したNode.jsプロセスが残り続ける
- 新しい開発セッションが起動できない
- 手動でプロセスキルが必要

## 解決策

専用のPowerShellベースのプロセス管理システムを実装しました。

### 主な機能

1. **プロセスツリー管理**: 親プロセス終了時に子プロセスも確実に終了
2. **ポート管理**: 開発用ポートの占有状況をチェック・解放
3. **孤立プロセス検出**: 前セッションで残った開発関連プロセスを自動検出・クリーンアップ
4. **状態追跡**: 実行中の開発プロセス情報をJSONファイルで保持

### ファイル構成

```
scripts/
├── dev-process-manager.ps1  # メインのPowerShellスクリプト
└── dev.bat                  # Windowsバッチファイルラッパー
```

## 使用方法

### 基本コマンド

```bash
# 開発環境の開始
npm run tauri:dev
# または
scripts/dev.bat start

# 開発環境の停止
npm run tauri:stop
# または  
scripts/dev.bat stop

# 開発環境の再起動
npm run tauri:restart
# または
scripts/dev.bat restart

# 現在の状態確認
npm run tauri:status
# または
scripts/dev.bat status
```

### コマンド詳細

#### `start`
- 孤立プロセスのクリーンアップ
- ポート5173の可用性確認・解放
- `npm run tauri dev`の安全な起動
- プロセス情報の追跡開始

#### `stop`
- 開発プロセスツリーの完全終了
- 関連する子プロセスの確実なクリーンアップ
- プロセス追跡情報のクリア

#### `restart`
- `stop` → `start` の安全な実行

#### `status`
- 現在実行中のプロセス状況
- ポート占有状況（5173, 1420）
- プロセス追跡情報の表示

## 技術的詳細

### プロセス検出ロジック

```powershell
# 開発関連プロセスの判定条件
$commandLine -match "(vite|npm.*dev|task-manager)" -and $commandLine -notmatch "claude"
```

- Viteサーバー、npm dev、task-manager関連のプロセスを検出
- Claude Code自体のプロセスは除外

### ポート管理

監視対象ポート:
- `5173`: Viteサーバー（フロントエンド）
- `1420`: Tauriアプリケーション（デスクトップアプリ）

### プロセス追跡

実行中のプロセス情報を `scripts/dev-processes.json` に保存：

```json
{
  "PID": 12345,
  "ProcessName": "npm",
  "Command": "npm run tauri dev", 
  "StartTime": "2025-08-14 19:30:00",
  "Ports": [5173, 1420]
}
```

## トラブルシューティング

### ポートが占有されている場合

```bash
# 状態確認
npm run tauri:status

# 強制クリーンアップ
npm run tauri:stop
```

### PowerShell実行ポリシーエラー

```bash
# 一時的に実行ポリシーを変更（管理者権限で実行）
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### 手動でのプロセス確認・削除

```powershell
# プロセス確認
Get-Process -Name node | Where-Object { $_.Id -ne $PID }

# 特定プロセスの終了
Stop-Process -Id <PID> -Force
```

## 従来方式との比較

| 項目 | 従来 (`npm run tauri dev`) | 新方式 (`npm run tauri:dev`) |
|------|---------------------------|-------------------------------|
| プロセスクリーンアップ | ❌ 不完全 | ✅ 完全 |
| ポート管理 | ❌ 手動対応必要 | ✅ 自動管理 |  
| 状態監視 | ❌ 不可 | ✅ 詳細な状態表示 |
| 孤立プロセス対策 | ❌ なし | ✅ 自動検出・削除 |
| 開発体験 | ❌ ポート競合で停止 | ✅ スムーズな開発 |

## 注意事項

1. **Windows専用**: このスクリプトはWindows環境専用です
2. **PowerShell必須**: PowerShell 5.0以上が必要です
3. **権限**: 場合によっては管理者権限が必要な場合があります
4. **プロセス追跡**: 異常終了時は追跡ファイルが残る可能性があります（次回起動時に自動クリーンアップ）

## 改善効果

- **開発効率向上**: ポート競合による開発中断を解消
- **手動作業削減**: 孤立プロセス削除の手動作業が不要
- **安定性向上**: 確実なプロセス終了により環境が安定
- **可視性向上**: プロセス・ポート状況の明確な把握が可能

この管理システムにより、TaskNagの開発環境は大幅に改善され、開発者の生産性が向上しました。