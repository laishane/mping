# MPing

一个跨平台的多目标ping工具，支持ICMP和TCP协议，可同时监控多个目标服务器的连通性。

## 功能特点

- 支持同时ping多个目标服务器
- 支持ICMP和TCP协议
- 支持实时显示或隐藏ping结果
- 支持将结果保存到指定日志文件
- 支持后台运行
- 支持自定义ping间隔和次数
- 优雅处理Ctrl+C中断
- 提供详细的统计信息
- 跨平台支持(Windows/Linux)

## 系统要求

### Windows
- Windows 7 或更高版本
- 如果使用ICMP协议，需要管理员权限

### Linux
- 内核版本 2.6 或更高
- 如果使用ICMP协议，需要root权限或设置 `CAP_NET_RAW` capability

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/laishane/mping.git
cd mping

# 编译
cargo build --release

# 编译后的可执行文件位于 target/release/mping
```

### 交叉编译

#### Linux到Windows (x86_64)
```bash
# 安装Windows目标
rustup target add x86_64-pc-windows-gnu
# 安装MinGW-w64
sudo apt-get install mingw-w64  # Ubuntu/Debian
sudo dnf install mingw64-gcc    # Fedora

# 编译
cargo build --target x86_64-pc-windows-gnu --release
```

#### Windows到Linux (x86_64)
```powershell
# 安装Linux目标
rustup target add x86_64-unknown-linux-gnu
# 安装LLVM和Clang
choco install llvm

# 编译
cargo build --target x86_64-unknown-linux-gnu --release
```

## 使用方法

### 基本用法
```bash
# Windows (管理员权限)
mping.exe -t www.google.com www.apple.com

# Linux (root权限)
sudo ./mping -t www.google.com www.apple.com
```

### 命令行参数

```
选项：
    -t, --targets <TARGETS>...     要ping的目标服务器列表
    -d, --display                  是否在终端显示实时结果 [默认: true]
    -l, --log <LOG>               日志文件保存路径
    -b, --background              是否在后台运行 [默认: false]
    -p, --protocol <PROTOCOL>     使用的协议 [可选: icmp, tcp] [默认: icmp]
    -i, --interval <INTERVAL>     ping间隔(秒) [默认: 1]
    -c, --count <COUNT>           ping总次数 [默认: 0 (无限)]
    -h, --help                    显示帮助信息
```

### 使用示例

1. 使用ICMP协议ping多个目标：
```bash
# Windows
mping.exe -t www.google.com www.apple.com

# Linux
sudo ./mping -t www.google.com www.apple.com
```

2. 使用TCP协议（不需要特权）：
```bash
mping -t www.google.com -p tcp
```

3. 在后台运行，每5秒ping一次，总共ping 100次：
```bash
mping -t www.google.com -b -i 5 -c 100 -l ping.log
```

4. 不显示实时结果，仅保存到日志：
```bash
mping -t www.google.com -d false -l ping.log
```

### 日志格式

日志文件使用CSV格式，包含以下字段：
```
目标,时间戳,状态,RTT(ms)
```

示例：
```
www.google.com,2024-03-20 10:30:15,Success,45
```

最后会包含统计信息：
```
--- Final Statistics ---
Statistics for www.google.com:
    Packets: Sent = 100, Received = 98, Lost = 2 (2% loss)
    Round Trip Times: Min = 42ms, Max = 128ms, Avg = 56ms
--- End of Statistics ---
```

## Linux特权设置

如果不想使用root权限运行ICMP ping，可以设置capability：

```bash
sudo setcap cap_net_raw+ep ./target/release/mping
```

## 注意事项

1. 在Linux系统上使用ICMP协议需要root权限或适当的capability
2. 使用TCP协议时会默认使用目标的80端口
3. 后台运行时，统计信息会保存在日志文件中
4. 程序只响应Ctrl+C信号，不会被其他终端操作影响

## 贡献

欢迎提交Issue和Pull Request！

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件 