# diffx-python

Python/pip向けのdiffxバインディング。Rustのdiffx-coreをPyO3でラップ。

## アーキテクチャ

```
diffx-core (crates.io 0.6.1)  ←  Rustネイティブライブラリ
      ↓
diffx-python (this)           ←  PyO3でPythonバインディング
      ↓
pip package                   ←  maturinでwheelを生成
```

## 構造

```
diffx-python/
├── src/lib.rs              # PyO3バインディング実装
├── src/diffx_python/       # Pythonモジュール
│   └── __init__.py         # re-export + ユーティリティ関数
├── Cargo.toml              # diffx-core依存（crates.io版）
├── pyproject.toml          # maturin設定 + pytest設定
├── tests/                  # pytestテスト
│   ├── __init__.py
│   └── test_unified_api.py
└── .github/workflows/
    ├── ci.yml              # push/PR → fmt + clippy + build + test
    └── release.yml         # タグ → マルチプラットフォームビルド + Release作成
```

## ビルド

```bash
# 開発環境セットアップ
uv sync --all-extras

# pre-commitフックをインストール
uv run pre-commit install

# ビルド（開発モード）
uv run maturin develop

# テスト
uv run pytest

# Rustフォーマット＆lint
cargo fmt --check
cargo clippy

# 手動でpre-commitを実行
uv run pre-commit run --all-files
```

## GitHub Actions

| ワークフロー | トリガー | 動作 |
|-------------|---------|------|
| ci.yml | push/PR to main | fmt + clippy + Linux x64ビルド + テスト |
| release.yml | タグ v* | マルチプラットフォームビルド + GitHub Release作成 |

### ビルドターゲット（release.yml）

- x86_64-unknown-linux-gnu
- x86_64-unknown-linux-musl
- aarch64-unknown-linux-gnu
- x86_64-apple-darwin
- aarch64-apple-darwin
- x86_64-pc-windows-msvc

## リリース手順

1. `pyproject.toml`、`Cargo.toml`、`src/lib.rs`のバージョンを更新
2. コミット & プッシュ
3. `git tag v0.6.1 && git push origin v0.6.1`
4. GitHub Actionsがビルド → Release作成 → wheelを添付
5. `pip install diffx-python`（PyPIから）または wheelから直接インストール

## API

### diff(old, new, **kwargs)
2つのオブジェクトを比較し、差分を返す。

オプション（kwargs）:
- `epsilon` - 数値比較の許容誤差
- `array_id_key` - 配列要素の識別キー
- `ignore_keys_regex` - 無視するキーの正規表現
- `path_filter` - パスフィルタ
- `output_format` - 出力フォーマット
- `ignore_whitespace` - 空白を無視
- `ignore_case` - 大文字小文字を無視
- `brief_mode` - 簡略モード
- `quiet_mode` - 静粛モード

### パーサー
- `parse_json(content)` - JSON
- `parse_yaml(content)` - YAML
- `parse_toml(content)` - TOML
- `parse_csv(content)` - CSV
- `parse_ini(content)` - INI
- `parse_xml(content)` - XML

### ユーティリティ
- `format_output(results, format)` - 差分結果をフォーマット（"json", "yaml", "diffx"）
- `diff_files(file1, file2, **kwargs)` - ファイル同士を比較
- `diff_strings(str1, str2, format, **kwargs)` - 文字列同士を比較

## 開発ルール

- diffx-coreはcrates.ioの公開版を使用（ローカルパス依存禁止）
- コミット前にcargo fmtが自動実行される（husky）
- バージョンは3箇所を同期: pyproject.toml, Cargo.toml, src/lib.rs
