#!/bin/bash

# LINE Bot Rust 專案建構腳本
# 用於編譯、測試和打包 release 版本

set -e  # 遇到錯誤時立即退出

# 顏色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 輔助函數
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_step() {
    echo
    echo -e "${BLUE}==== $1 ====${NC}"
}

# 檢查是否安裝 Rust
check_rust() {
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi
    
    local rust_version=$(rustc --version)
    print_info "使用 Rust 版本: $rust_version"
}

# 清理之前的建構
clean_build() {
    print_step "清理之前的建構"
    cargo clean
    print_success "清理完成"
}

# 檢查程式碼格式
check_format() {
    print_step "檢查程式碼格式"
    if ! cargo fmt -- --check; then
        print_warning "程式碼格式不符合標準，自動格式化中..."
        cargo fmt
        print_success "程式碼格式化完成"
    else
        print_success "程式碼格式檢查通過"
    fi
}

# 執行 Clippy 檢查
run_clippy() {
    print_step "執行 Clippy 靜態分析"
    cargo clippy --all-targets --all-features -- -D warnings
    print_success "Clippy 檢查通過"
}

# 執行測試
run_tests() {
    print_step "執行單元測試和整合測試"
    cargo test --verbose
    print_success "所有測試通過"
}

# 建構 Debug 版本
build_debug() {
    print_step "建構 Debug 版本"
    cargo build --verbose
    print_success "Debug 版本建構完成"
}

# 建構 Release 版本
build_release() {
    print_step "建構 Release 版本"
    cargo build --release --verbose
    print_success "Release 版本建構完成"
}

# 檢查產生的二進位檔案
check_binary() {
    print_step "檢查建構結果"
    
    local debug_binary="target/debug/linebot-rs"
    local release_binary="target/release/linebot-rs"
    
    # Windows 系統需要 .exe 副檔名
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        debug_binary="${debug_binary}.exe"
        release_binary="${release_binary}.exe"
    fi
    
    if [[ -f "$debug_binary" ]]; then
        local debug_size=$(stat -f%z "$debug_binary" 2>/dev/null || stat -c%s "$debug_binary" 2>/dev/null || echo "unknown")
        print_info "Debug 版本: $debug_binary (大小: $debug_size bytes)"
    fi
    
    if [[ -f "$release_binary" ]]; then
        local release_size=$(stat -f%z "$release_binary" 2>/dev/null || stat -c%s "$release_binary" 2>/dev/null || echo "unknown")
        print_info "Release 版本: $release_binary (大小: $release_size bytes)"
        
        # 測試 release 版本是否可以執行
        print_info "測試 release 版本執行..."
        if timeout 3s "$release_binary" --help >/dev/null 2>&1 || [[ $? -eq 124 ]]; then
            print_success "Release 版本可正常執行"
        else
            print_warning "無法測試 release 版本執行狀態"
        fi
    fi
}

# 產生建構報告
generate_report() {
    print_step "產生建構報告"
    
    local report_file="build-report.txt"
    {
        echo "LINE Bot Rust 專案建構報告"
        echo "=============================="
        echo "建構時間: $(date)"
        echo "Rust 版本: $(rustc --version)"
        echo "Cargo 版本: $(cargo --version)"
        echo ""
        echo "專案資訊:"
        cargo metadata --no-deps --format-version 1 | jq -r '.packages[0] | "名稱: \(.name)\n版本: \(.version)\n作者: \(.authors[])\n描述: \(.description // "無")"'
        echo ""
        echo "依賴套件數量: $(cargo metadata --format-version 1 | jq '.packages | length')"
        echo ""
        echo "建構目標:"
        if [[ -f "target/debug/linebot-rs" || -f "target/debug/linebot-rs.exe" ]]; then
            echo "✓ Debug 版本"
        fi
        if [[ -f "target/release/linebot-rs" || -f "target/release/linebot-rs.exe" ]]; then
            echo "✓ Release 版本"
        fi
    } > "$report_file"
    
    print_success "建構報告已儲存至: $report_file"
}

# 主要建構流程
main() {
    local start_time=$(date +%s)
    
    print_info "開始 LINE Bot Rust 專案建構流程"
    print_info "專案目錄: $(pwd)"
    
    # 檢查 Rust 環境
    check_rust
    
    # 根據參數決定建構類型
    case "${1:-full}" in
        "clean")
            clean_build
            ;;
        "format")
            check_format
            ;;
        "clippy")
            run_clippy
            ;;
        "test")
            run_tests
            ;;
        "debug")
            check_format
            run_clippy
            run_tests
            build_debug
            check_binary
            ;;
        "release")
            check_format
            run_clippy
            run_tests
            build_release
            check_binary
            ;;
        "full"|"")
            clean_build
            check_format
            run_clippy
            run_tests
            build_debug
            build_release
            check_binary
            generate_report
            ;;
        *)
            echo "用法: $0 [clean|format|clippy|test|debug|release|full]"
            echo ""
            echo "選項說明:"
            echo "  clean   - 清理建構目錄"
            echo "  format  - 檢查和修正程式碼格式"
            echo "  clippy  - 執行 Clippy 靜態分析"
            echo "  test    - 執行所有測試"
            echo "  debug   - 建構 debug 版本"
            echo "  release - 建構 release 版本"
            echo "  full    - 完整建構流程 (預設)"
            exit 1
            ;;
    esac
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    print_success "建構完成！總耗時: ${duration} 秒"
}

# 執行主程式
main "$@"