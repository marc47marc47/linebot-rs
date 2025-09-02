# 貢獻指南

感謝您對 LINE Bot RS 專案的興趣！我們歡迎社群的貢獻。

## 開始之前

請先閱讀我們的 [行為準則](CODE_OF_CONDUCT.md)。

## 如何貢獻

### 回報問題

如果您發現了 bug 或有功能建議：

1. 先搜尋 [現有 Issues](https://github.com/your-repo/linebot-rs/issues) 確認問題是否已存在
2. 如果沒有找到相關問題，請建立新的 Issue
3. 使用適當的 Issue 模板
4. 提供詳細的資訊

### 提交程式碼

1. **Fork 專案**
   ```bash
   git clone https://github.com/your-username/linebot-rs.git
   cd linebot-rs
   ```

2. **建立功能分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **遵循編碼規範**
   - 使用 `cargo fmt` 格式化程式碼
   - 使用 `cargo clippy` 檢查程式碼品質
   - 遵循 Rust 社群最佳實務

4. **編寫測試**
   - 為新功能編寫單元測試
   - 為 API 變更編寫整合測試
   - 確保所有測試通過：`cargo test`

5. **更新文件**
   - 更新相關的 README 內容
   - 為公開 API 編寫文件註釋
   - 如需要，更新 DEPLOYMENT.md 或 TROUBLESHOOTING.md

6. **提交變更**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   git push origin feature/your-feature-name
   ```

7. **建立 Pull Request**
   - 使用 Pull Request 模板
   - 詳細描述變更內容
   - 連結相關的 Issues

## 開發環境設定

### 系統需求
- Rust 1.75+
- Git
- Docker（可選，用於測試）

### 設定步驟
```bash
# 1. Clone 專案
git clone https://github.com/your-repo/linebot-rs.git
cd linebot-rs

# 2. 設定環境變數
cp .env.example .env
# 編輯 .env 檔案

# 3. 安裝依賴並測試
cargo build
cargo test

# 4. 執行程式
cargo run
```

### 開發工具
推薦使用以下工具：
- **IDE**: VS Code 或 RustRover
- **擴充套件**: rust-analyzer, CodeLLDB
- **Git Hook**: pre-commit（自動格式化和檢查）

## 編碼規範

### Rust 風格
- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 使用 `rustfmt` 格式化程式碼
- 使用 `clippy` 檢查程式碼品質
- 程式碼必須通過 `cargo clippy -- -D warnings`

### 命名規範
- 函數和變數使用 `snake_case`
- 類型和 trait 使用 `PascalCase`
- 常數使用 `SCREAMING_SNAKE_CASE`
- 模組使用 `snake_case`

### 文件註釋
```rust
/// 處理 LINE webhook 事件
///
/// # 參數
/// * `event` - LINE 事件物件
///
/// # 回傳
/// * `Result<(), Box<dyn Error>>` - 處理結果
///
/// # 範例
/// ```
/// let event = Event::Message(message_event);
/// process_event(event).await?;
/// ```
pub async fn process_event(event: Event) -> Result<(), Box<dyn Error>> {
    // 實作內容
}
```

### 錯誤處理
- 使用 `Result<T, E>` 進行錯誤處理
- 為自訂錯誤類型實作 `std::error::Error`
- 提供有意義的錯誤訊息
- 使用 `anyhow` 或類似套件處理錯誤鏈

## 測試

### 單元測試
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // 測試內容
        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_async_function() {
        // 異步測試內容
    }
}
```

### 整合測試
- 放在 `tests/` 目錄下
- 測試完整的功能流程
- 使用模擬的 LINE API 回應

### 測試覆蓋率
- 目標：單元測試覆蓋率 > 80%
- 使用 `cargo tarpaulin` 生成覆蓋率報告

## Commit 訊息格式

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

```
<類型>[可選範圍]: <描述>

[可選的正文]

[可選的頁腳]
```

### 類型
- `feat`: 新功能
- `fix`: bug 修復
- `docs`: 文件變更
- `style`: 程式碼格式（不影響程式碼意義）
- `refactor`: 重構（既不修復 bug 也不新增功能）
- `perf`: 效能改進
- `test`: 新增或修改測試
- `chore`: 建置工具或輔助工具變更

### 範例
```
feat(webhook): add support for flex messages

Implement support for LINE flex messages including:
- Flex message parsing
- Template rendering
- Response generation

Closes #123
```

## 版本發佈

我們使用 [語義化版本](https://semver.org/)：

- `MAJOR`: 不相容的 API 變更
- `MINOR`: 向後相容的功能新增
- `PATCH`: 向後相容的 bug 修復

### 發佈流程
1. 更新 `Cargo.toml` 中的版本
2. 更新 `CHANGELOG.md`
3. 建立 git tag
4. GitHub Actions 會自動發佈

## Pull Request 指南

### PR 檢查清單
- [ ] 程式碼通過所有測試
- [ ] 程式碼通過 clippy 檢查
- [ ] 程式碼已格式化（rustfmt）
- [ ] 新功能有對應的測試
- [ ] 文件已更新
- [ ] Commit 訊息符合規範
- [ ] PR 描述清楚說明變更

### Code Review
- 所有 PR 需要至少一位維護者審核
- 回應 review 意見
- 根據回饋修改程式碼
- 保持討論專業和建設性

## 社群

- **GitHub Discussions**: 討論新功能和想法
- **Issues**: 回報 bug 和功能請求
- **Discord/Slack**: 即時討論（如果有）

## 授權

提交程式碼即表示您同意將您的貢獻以 [MIT 授權條款](LICENSE) 授權。

## 維護者

目前的專案維護者：
- [@maintainer1](https://github.com/maintainer1)
- [@maintainer2](https://github.com/maintainer2)

## 常見問題

### Q: 我應該從哪裡開始？
A: 查看標記為 `good first issue` 的 Issues，這些是適合新貢獻者的問題。

### Q: 我的 PR 被拒絕了，為什麼？
A: 常見原因包括：
- 不符合編碼規範
- 缺少測試
- 功能不符合專案目標
- 需要更多討論

### Q: 如何同步我的 fork？
A: ```bash
git remote add upstream https://github.com/original-repo/linebot-rs.git
git fetch upstream
git checkout main
git merge upstream/main
```

---

感謝您的貢獻！🎉