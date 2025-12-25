# pdf2other

一个用 Rust 编写的命令行工具，用于将 PDF 文件的指定页面范围转换为图片格式（PNG 或 JPEG）。

## 功能特性

- ✅ 支持将 PDF 文件的指定页面范围转换为图片
- ✅ 支持 PNG 和 JPEG 两种输出格式
- ✅ 可自定义渲染 DPI（分辨率）
- ✅ 支持灵活的页码范围格式（`1-5` 或 `1..5`）
- ✅ 自动处理像素格式转换（BGRA → RGB）
- ✅ 完善的错误处理和用户友好的中文提示

## 系统要求

- **Rust**: 1.89.0 或更高版本
- **操作系统**: Windows、Linux、macOS
- **PDFium 库**: 本项目使用静态链接，PDFium 库会被编译到可执行文件中

### PDFium 静态链接说明

本项目已配置为**静态链接** PDFium 库，这意味着：
- ✅ 可执行文件可以独立运行，无需外部 DLL 文件
- ✅ 无需在系统 PATH 中安装 PDFium
- ✅ 分发更方便，只需一个可执行文件
- ⚠️ 可执行文件会更大（增加约 20-50MB）

### 构建静态链接版本

#### 1. 获取 PDFium 静态库

**Windows:**
- 下载预编译的静态库（`.lib` 文件）：
  - [pdfium-binaries](https://github.com/bblanchon/pdfium-binaries/releases) - 提供预编译版本
  - 或从 [PDFium 官方](https://pdfium.googlesource.com/pdfium/) 源码编译

**Linux/macOS:**
- 下载静态库（`.a` 文件）或从源码编译

#### 2. 设置静态库路径

有两种方式指定静态库位置：

**方式1：使用环境变量（推荐）**
```bash
# Windows PowerShell
$env:PDFIUM_STATIC_LIB_PATH="C:\path\to\pdfium\lib"
cargo build --release

# Windows CMD
set PDFIUM_STATIC_LIB_PATH=C:\path\to\pdfium\lib
cargo build --release

# Linux/macOS
export PDFIUM_STATIC_LIB_PATH=/path/to/pdfium/lib
cargo build --release
```

**方式2：将静态库放在项目根目录**
- 将 `pdfium.lib`（Windows）或 `libpdfium.a`（Linux/macOS）放在项目根目录
- 构建工具会自动查找

#### 3. 构建项目

```bash
cargo build --release
```

构建完成后，可执行文件位于 `target/release/pdf2other`（Windows 为 `pdf2other.exe`）。

### 动态链接方式（备选）

如果您希望使用动态链接（需要外部 DLL），可以修改 `Cargo.toml`：

```toml
[dependencies]
pdfium-render = "0.8"  # 移除 static feature
```

然后参考下面的动态链接说明。

### 动态链接说明（备选方案）

如果使用动态链接，需要：

#### Windows

1. **下载 PDFium DLL**
   - 从 [pdfium-binaries](https://github.com/bblanchon/pdfium-binaries/releases) 下载 `pdfium.dll`

2. **放置 DLL 文件**
   - 将 `pdfium.dll` 放在以下任一位置：
     - 应用程序可执行文件所在目录
     - 系统 PATH 环境变量中的目录
     - 当前工作目录

#### Linux

```bash
# Ubuntu/Debian
sudo apt-get install libpdfium-dev
```

#### macOS

```bash
brew install pdfium
```

**注意**: 如果遇到问题，请参考 [pdfium-render 文档](https://crates.io/crates/pdfium-render) 获取最新安装说明。

## 安装

### 从源码构建（静态链接）

1. **克隆或下载项目代码**
   ```bash
   git clone <repository-url>
   cd pdf2other
   ```

2. **获取 PDFium 静态库**
   - 从 [pdfium-binaries](https://github.com/bblanchon/pdfium-binaries/releases) 下载对应平台的静态库
   - Windows: 下载 `pdfium.lib`
   - Linux/macOS: 下载 `libpdfium.a`

3. **设置静态库路径**
   
   **方式1：使用环境变量**
   ```bash
   # Windows PowerShell
   $env:PDFIUM_STATIC_LIB_PATH="C:\path\to\pdfium\lib"
   
   # Windows CMD
   set PDFIUM_STATIC_LIB_PATH=C:\path\to\pdfium\lib
   
   # Linux/macOS
   export PDFIUM_STATIC_LIB_PATH=/path/to/pdfium/lib
   ```
   
   **方式2：将静态库放在项目根目录**
   - 将 `pdfium.lib`（Windows）或 `libpdfium.a`（Linux/macOS）放在项目根目录

4. **构建项目**
   ```bash
   cargo build --release
   ```

5. **编译后的可执行文件**
   - 位于 `target/release/pdf2other`（Windows 下为 `target/release/pdf2other.exe`）
   - 可执行文件已包含 PDFium 库，可独立运行

### 安装到系统路径（可选）

```bash
# 确保已设置 PDFIUM_STATIC_LIB_PATH 环境变量
cargo install --path .
```

## 使用方法

### 基本语法

```bash
pdf2other <PDF_PATH> --pages <RANGE> [OPTIONS]
```

### 参数说明

| 参数 | 简写 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `PDF_PATH` | - | ✅ | PDF 文件路径 | - |
| `--pages <RANGE>` | `-p` | ✅ | 页码范围，支持格式：`1-5` 或 `1..5` | - |
| `--format <FORMAT>` | `-f` | ❌ | 输出格式：`png` 或 `jpeg` | `png` |
| `--dpi <DPI>` | - | ❌ | 渲染分辨率（DPI） | `150` |

### 使用示例

#### 示例 1: 转换第 1 到第 5 页为 PNG

```bash
pdf2other document.pdf --pages 1-5
```

#### 示例 2: 转换指定页面为 JPEG 格式

```bash
pdf2other document.pdf --pages 1-5 --format jpeg
```

或使用简写：

```bash
pdf2other document.pdf -p 1-5 -f jpeg
```

#### 示例 3: 使用高分辨率（300 DPI）渲染

```bash
pdf2other document.pdf --pages 1-10 --dpi 300
```

#### 示例 4: 使用 `..` 格式指定页码范围

```bash
pdf2other document.pdf --pages 3..10
```

#### 示例 5: 转换单页

```bash
pdf2other document.pdf --pages 5-5
```

## 输出说明

- **输出位置**: 图片文件保存在 PDF 文件所在的目录
- **文件命名格式**: `原文件名_page_页码.扩展名`
  - 例如：`document.pdf` 的第 3 页会保存为 `document_page_3.png`
- **文件格式**: 根据 `--format` 参数决定（PNG 或 JPEG）

## 错误处理

工具会检查并报告以下错误：

- ❌ PDF 文件不存在
- ❌ 页码范围格式无效
- ❌ 页码范围超出 PDF 总页数
- ❌ 输出格式不支持
- ❌ 图片保存失败

所有错误消息均为中文，便于理解。

## 注意事项

1. **页码范围**: 
   - 页码从 1 开始计数
   - 支持 `1-5` 或 `1..5` 两种格式
   - 起始页码不能大于结束页码

2. **DPI 设置**:
   - 默认 DPI 为 150，适合一般用途
   - 更高的 DPI（如 300）会产生更清晰的图片，但文件大小也会增加
   - 建议根据实际需求选择合适的 DPI

3. **输出格式**:
   - PNG: 无损压缩，文件较大，适合需要高质量的场景
   - JPEG: 有损压缩，文件较小，适合需要较小文件大小的场景

4. **PDFium 静态链接**:
   - 本项目默认使用静态链接，PDFium 库会被编译到可执行文件中
   - 可执行文件可以独立运行，无需外部 DLL 文件
   - 构建时需要提供 PDFium 静态库（`.lib` 或 `.a` 文件）
   - 如果遇到初始化错误，请检查 `PDFIUM_STATIC_LIB_PATH` 环境变量是否正确设置
   - 详细构建说明请参考上面的"PDFium 静态链接说明"部分

## 开发

### 项目结构

```
pdf2other/
├── Cargo.toml          # 项目配置和依赖
├── README.md           # 项目文档
└── src/
    └── main.rs         # 主程序代码
```

### 依赖库

- `clap`: 命令行参数解析
- `pdfium-render`: PDF 渲染库
- `image`: 图片处理库

### 构建和测试

```bash
# 检查代码
cargo check

# 构建调试版本
cargo build

# 构建发布版本
cargo build --release

# 运行测试（如果有）
cargo test
```

## 许可证

本项目采用 MIT 许可证。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 更新日志

### v0.1.0
- 初始版本
- 支持 PDF 转 PNG/JPEG
- 支持页码范围选择
- 支持自定义 DPI

