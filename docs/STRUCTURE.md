# 项目结构

## 关键概念

- Track：mcat 管理的音乐曲目，包括元数据与音频文件。
- Repository：存储 track 元数据的仓库。目前以 TOML 文件形式存在，但在项目中 repository 被抽象为 `Repo` trait，可以有不同的后端实现。

> 从开发者角度来看，track 有时特指元数据中直接存储到 repository 中的文本数据集合（如 `artist` 字段，而歌词和专辑封面则因单独以文件形式存储而不在此列）。下文常混用这两个概念。

## 代码层次

从业务流程来看，mcat 被分为入口层、CLI 层、业务层和存储层。

### 入口层

入口层位于 `crate::main::main()`，负责初始化配置（主要是相关目录信息的初始化）、执行 CLI 入口函数与打印错误信息。

绝大多数错误信息均返回到入口层进行打印。

### CLI 层

CLI 层主体位于 `mcat::cli`，定义了面向用户的 CLI 接口，通过 `clap` 的派生宏实现。

CLI 被设计为“子命令 + 选项”的形式：

- 子命令定义操作，如 `init` 代表初始化仓库；
- 子命令选项精细化控制操作。

#### CLI 层过滤器

由于大多数涉及查询的操作都需要实现范围查询的功能，因此定义了统一的 `FilterArgs` 结构体用于收集查询相关的条件。这些条件统称为“过滤器”（filter）。

过滤条件被收集到 `FilterArgs` 各个字段，但为方便用户使用，CLI 层直接提供指定各类过滤条件的选项。

为避免反向依赖问题，业务层定义了 `TrackFilter` 结构体对应于 `FilterArgs`，并在 CLI 层给出了 `TrackFilter` 的 `TryFrom<FilterArgs>` 实现用于将 `FilterArgs` 转换为 `TrackFilter`。

过滤条件的指定有如下规则：

- 过滤条件被分为多个类别，基本按照元数据类型分类。不同类别的过滤条件取 **与** 的关系。
- 每个类别的过滤条件可以重复指定。相同类别的过滤条件取 **或** 的关系。

举例来说：

```sh
mcat display --title="黄金时代" --title="鹤顶红" --artist="麦浚龙"
```

这将筛选出标题为《黄金时代》或《鹤顶红》、且艺术家为麦浚龙的 track。

另外 `mcat::commands` 同时属于 CLI 层和业务层，其 `run()` 函数是 CLI 层的入口，被入口层调用后执行 CLI 解析和参数分发。

### 业务层

业务层主体位于 `mcat::commands::*`，按子命令类型实现。`mcat::commands::run()` 解析子命令，然后将参数分发给对应子命令的 `execut()` 函数。

各个子命令的 `execute()` 负责执行业务逻辑，直接使用存储层的 `Repo` trait 暴露出的接口完成和 repository 相关的操作。

### 存储层

存储层主体位于 `mcat::repos`，其中 `Repo` trait 定义了 repository 可用的操作，而 `TomlDb` 则是对 `Repo` 的具体实现。

对于 `TomlDb` 实现，repository 对应文件在业务逻辑开始时通过 `TomlDb::try_from()` 被反序列化加载到内存成为结构体，而在业务逻辑结束前通过 `Repo::persist()` 被序列化，从内存写回文件。
