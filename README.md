# Rust OS（基于 Writing an OS in Rust 的学习实现）

这是一个用 Rust 编写的玩具操作系统项目，用于学习 x86_64 裸机开发、引导启动、内核初始化与基础系统机制。

本项目主要参考 phil-opp 的系列教程《Writing an OS in Rust》。我在学习过程中按照教程步骤完成实现，并对代码与工程结构做了自己的整理与维护。

## 项目状态
- 已完成：最小内核启动与基本输出
- 已完成：异常/中断基础设施（如 IDT、panic 处理等）
- 已完成：内存相关基础（如分页/帧分配/堆分配器）

## 环境要求
- Rust（nightly）
- cargo
- QEMU（用于运行与调试）

## 组件：
- rust-src
- llvm-tools-preview

## 构建与运行

1. 安装 nightly（如需要）
```bash
rustup toolchain install nightly
rustup default nightly
````

2. 安装可选组件

```bash
rustup component add rust-src --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly
```
3. 构建

```bash
cargo build
```
4. 运行

```bash
cargo run
```

bootimage 需要：

```bash
cargo install bootimage
rustup component add llvm-tools-preview
cargo bootimage
cargo run
```

## 调试

你可以使用 QEMU 的调试参数配合 GDB/LLDB 进行调试，例如使用 QEMU 的 gdb stub（具体命令按你的运行脚本/runner 配置为准）。

如果项目启用了串口输出，建议在 QEMU 中打开串口重定向以便查看日志。


## 致谢与参考

* Writing an OS in Rust（phil-opp）：[https://os.phil-opp.com/](https://os.phil-opp.com/)

说明：

* 本仓库用于学习与交流。
* 若你从 phil-opp 的配套仓库直接复制/引用了部分代码或配置文件，请确保遵循其仓库的许可证要求，并保留必要的版权声明与许可证文本。

