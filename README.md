# PLUME-INSTALLER

PLUME-INSTALL 是一个快速安装[`plume`](https://github.com/zhangbaitong/plume) 的工具。

## 依赖

**plume-install**依赖**git**和**composer**，确保它们在PATH内。

~~注:Windows下需要安装VS运行时.~~

## 编译

```bash
cargo build --release
```
在target目录下生成可执行文件。

~~注：Windows需要安装Visual Studio 2013或Visual Studio 2015。~~

## 跨平台

PLUME-INSTALL支持Windows,Linux,MacOSX。

在dist目录可以直接使用对应可执行文件。

不支持32位操作系统。

## 用法
```
plume-install new $project_name $version
```
例如：  
安装plume1：
```
plume-install new testpro 1
```
安装plume2：
```
plume-install new testpro 2
```
## 备注  
version 目前支持 1 和 2，分别安装 plume1 和 plume2。

由于墙的原因，**安装依赖**可能会很慢，可以`ctrl`+`c`取消后手动`composer install`。

