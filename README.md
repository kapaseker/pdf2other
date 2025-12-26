# pdf2other

Command-line tool to convert a specified page range of a PDF into images (PNG or JPEG).

## Usage (English)

### Syntax

```bash
pdf2other <PDF_PATH> --pages <RANGE> [OPTIONS]
```

### Options

- **`<PDF_PATH>`**: Path to the input PDF file.
- **`-p, --pages <RANGE>`** *(required)*: Page range. Supported formats:
  - `1-5`
  - `1..5`
- **`-f, --format <png|jpeg|jpg>`**: Output format. Default: `png`.
- **`--dpi <DPI>`**: Render DPI. Default: `480`.
- **`-d, --dir <DIR>`**: Output directory. If omitted, output goes to a folder named after the PDF (next to the PDF).

### Examples

Convert pages 1 to 5 to PNG:

```bash
pdf2other document.pdf --pages 1-5
```

Convert pages 3 to 10 using `..` syntax:

```bash
pdf2other document.pdf --pages 3..10
```

Convert to JPEG:

```bash
pdf2other document.pdf --pages 1-5 --format jpeg
```

Render with a custom DPI:

```bash
pdf2other document.pdf --pages 1-10 --dpi 300
```

Write output to a specific directory:

```bash
pdf2other document.pdf --pages 1-5 --dir output_images
```

### Output

- **Location**:
  - If `--dir` is provided: images are saved there.
  - Otherwise: images are saved under a sibling folder named after the PDF file stem.
- **File naming**: `<pdf_stem>_page_<page>.<ext>`
  - Example: `document_page_3.png`

### Notes

- Page numbers are **1-based**.
- The range must not exceed the PDF’s total pages.
- Run `pdf2other --help` to see CLI help.

## 使用方法（中文）

将 PDF 的指定页码范围转换为图片（PNG 或 JPEG）的命令行工具。

### 基本语法

```bash
pdf2other <PDF_PATH> --pages <RANGE> [OPTIONS]
```

### 参数说明

- **`<PDF_PATH>`**：输入 PDF 文件路径
- **`-p, --pages <RANGE>`**（必填）：页码范围，支持：
  - `1-5`
  - `1..5`
- **`-f, --format <png|jpeg|jpg>`**：输出格式，默认 `png`
- **`--dpi <DPI>`**：渲染 DPI，默认 `480`
- **`-d, --dir <DIR>`**：输出目录；不指定则默认输出到与 PDF 同目录下、以 PDF 同名命名的文件夹

### 示例

将第 1 到第 5 页转换为 PNG：

```bash
pdf2other document.pdf --pages 1-5
```

使用 `..` 格式指定页码范围：

```bash
pdf2other document.pdf --pages 3..10
```

转换为 JPEG：

```bash
pdf2other document.pdf --pages 1-5 --format jpeg
```

使用自定义 DPI 渲染：

```bash
pdf2other document.pdf --pages 1-10 --dpi 300
```

输出到指定目录：

```bash
pdf2other document.pdf --pages 1-5 --dir output_images
```

### 输出说明

- **输出位置**：
  - 指定 `--dir`：输出到该目录
  - 未指定：输出到 PDF 同目录下、以 PDF 文件名（不含扩展名）命名的文件夹
- **命名规则**：`<pdf_stem>_page_<页码>.<扩展名>`
  - 示例：`document_page_3.png`

### 注意事项

- 页码从 **1** 开始计数
- 页码范围不能超过 PDF 总页数
- 可使用 `pdf2other --help` 查看命令行帮助


