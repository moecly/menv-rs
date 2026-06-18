# Project

## Codebase Navigation
**Always read `CODEBASE_INDEX.md` before opening any source file.**
It contains the complete file map with exports and purpose for every file.
Use it to locate the exact file you need, then read only that file.

## Commands
```
# build
cargo build

# check
cargo check

# lint
cargo clippy
cargo fmt

# test
RUST_LOG=debug cargo test -- --show-output

# run (using just)
just run            # fmt + cargo run
just run -- tools list    # 传参
```

## Architecture
`cli_main()` → 匹配子命令 → 对应模块的 `handle_cmd()` 处理。
`Common::process_command()` 统一处理外部命令的执行。
`CliConfig` 跨子命令共享配置，编译期通过 `include_str!` 内嵌默认配置。

## Conventions
- 默认不写注释；只在 WHY 不明显时加一行
- 错误处理用 `color-eyre`
- config/ 下 TOML 配置文件：`repos.toml`（仓库列表）、`tools.toml`（包列表）
- `tools.toml` 中 `package_source` 支持 `pacman` 或 `paru`
- 跨模块传递配置统一用 `CliConfig`

## Notes
- 子命令：status / repos / tools / scripts / update
- scripts 子命令：boot-default, hibernate, locale, paru, service, swapfile, trim
- `.claude/settings.json` 有 tokenmiser 集成，编辑文件后自动更新索引
