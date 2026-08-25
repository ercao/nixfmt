---
labels:
  - ready-for-agent
---

# Align trailing comments

## 问题陈述

nixfmt 会把行尾注释前的空白规范化为一个空格。用户无法选择让一组相关的行尾 `#` 注释主动落在同一列，因此包含简短并列配置的代码块可能不易纵向扫描。依赖输入中的手工空格不能解决问题，因为 formatter 不承诺保留任意输入空白。

## 解决方案

在稳定 TOML 配置中增加默认关闭的 `align_trailing_comments` 布尔选项。启用后，nixfmt 先产生正常格式化结果，再识别其中的 alignment group，并将每组行尾 `#` 注释移动到组内最长代码前缀之后一列。

Alignment group 是至少两条连续、同缩进层级且各自包含行尾 `#` 注释的格式化后代码行。空行、没有行尾注释的行、独占一行的注释或缩进变化都会结束当前组。

## 用户故事

1. 作为 nixfmt 用户，我希望选择主动对齐相关的行尾注释，以便快速纵向阅读并列配置。
2. 作为现有 nixfmt 用户，我希望该功能默认关闭，以便升级 formatter 时不产生未请求的全仓库 diff。
3. 作为项目维护者，我希望通过 `.nixfmt.toml` 启用该风格，以便团队获得一致的格式化结果。
4. 作为使用显式配置文件的用户，我希望 `--config PATH` 中的同一选项生效，以便自动发现和显式配置具有相同语义。
5. 作为用户，我希望连续且同缩进的相关注释形成独立对齐组，以便不会跨越无关代码产生大量填充空格。
6. 作为用户，我希望空行、无行尾注释的行、独立注释和缩进变化切断对齐组，以便代码结构仍然决定视觉分区。
7. 作为用户，我希望只有至少两行的组发生对齐，以便单独一条注释继续使用普通单空格格式。
8. 作为用户，我希望属性集、列表、`let` 绑定及其他 Nix 结构遵循相同规则，以便功能不依赖具体语法节点。
9. 作为用户，我希望字符串中的 `#` 保持原样，以便格式化不会改变程序内容。
10. 作为用户，我希望块注释保持现有行为，以便第一版不会混入多行注释布局问题。
11. 作为使用 Tab 缩进的用户，我希望行内对齐仅使用空格，以便结果不依赖编辑器的 Tab 宽度。
12. 作为配置了 `max_width` 的用户，我希望对齐填充不会制造新的宽度超限，以便注释对齐服从已有宽度偏好。
13. 作为包含长注释的用户，我希望注释文本本身造成的既有软超限不会关闭对齐，以便该选项不尝试重排不可拆分的注释内容。
14. 作为未配置 `max_width` 的用户，我希望 formatter 直接使用组内最长代码前缀，以便不引入另一个隐式宽度或填充限制。
15. 作为反复运行 formatter 的用户，我希望第一次输出再次格式化后保持不变，以便该功能满足 nixfmt 的幂等性要求。

## 实现决策

- 稳定 TOML 配置新增布尔字段 `align_trailing_comments`，默认值为 `false`。
- 不新增对应的 CLI 格式风格参数；自动发现和显式选择的 TOML 配置共用该字段。
- 基础 formatter 的现有语法规则先完整运行；仅在选项启用时执行额外的 syntax-aware alignment pass。
- Alignment pass 重新解析基础格式化结果并依据 rnix comment token 工作，以排除字符串中的 `#`，而不是使用纯文本查找。
- Alignment group 从基础格式化输出识别，不继承输入文件中的手工空白或分组。
- 只有行尾 `#` line comment 参与；standalone comment 和 block comment 不参与。
- Alignment group 必须包含至少两条连续、同缩进层级的参与行。
- 空行、缺少行尾注释的行、standalone comment 或缩进变化结束当前 alignment group。
- 分组规则跨所有 Nix 语法结构统一应用，不为属性集、列表或其他节点建立专用对齐规则。
- 目标注释列为组内最长代码前缀的列数加一，不向制表位取整。
- 填充始终使用 ASCII 空格；Tab 只用于已有的行首缩进。
- 列计算复用 formatter 现有模型：每个 Unicode scalar value 计一列，不引入终端 display-width 依赖。
- 配置了 `max_width` 时，如果新增的 alignment padding 会让原本未超宽的任一行越过该限制，则整个组保留基础 formatter 的单空格分隔，不做部分对齐。
- 注释文本本身造成的既有软超限不关闭该组的对齐。
- 未配置 `max_width` 时，不设置额外的最大填充量。
- 采用整组对齐或整组回退，不允许同一 alignment group 中只有部分行对齐。
- 该实现遵循 ADR 0005。Ormolu 和 Fourmolu 的单空格策略以及 Brittany 的语法级列布局仅作为对比，不改用 Brittany 式逐语法规则分组。

## 测试决策

- 复用公共 in-memory formatter 作为主要行为 seam；测试完整格式化输入输出，不直接测试 alignment pass 的内部辅助函数。
- 复用 CLI 集成 seam 验证稳定 TOML 契约，包括自动发现或显式配置能够解析并启用 `align_trailing_comments`。
- 不创建新的测试 seam。
- 验证缺省配置和显式 `false` 均保持当前行尾注释单空格输出。
- 验证显式 `true` 会把两行及以上的有效 alignment group 对齐到最长代码前缀后一列。
- 验证单行注释不发生无意义对齐。
- 分别验证空行、无行尾注释行、standalone comment 和缩进变化会切断分组。
- 验证相同规则覆盖不同 Nix 语法结构，而不是只覆盖属性赋值。
- 验证字符串中的 `#`、standalone `#` comment 和 block comment 不参与对齐且内容不变。
- 验证 Tab 缩进仍保留，而行内新增填充只包含空格。
- 验证 Unicode 列计算与现有 scalar-value 模型一致。
- 验证没有 `max_width` 时允许对齐到最长代码前缀，不应用额外填充上限。
- 验证 alignment padding 会制造新超限时整组回退到基础单空格输出。
- 验证只有注释文本造成既有超限时仍执行对齐。
- 验证一次格式化和再次格式化的输出完全相同。
- 验证未知配置字段的错误契约仍然成立，并包含新增的合法字段名。

## 范围外

- 保留输入中的任意水平空白或已有手工对齐。
- 对齐赋值符号、操作符、记录字段或其他非注释列。
- 对齐 block comment 或调整多行注释内部缩进。
- 重排、换行或截断注释文本。
- 使用终端 display width 处理 CJK、emoji 或其他宽字符。
- 新增注释对齐专用 CLI flag。
- 默认启用注释对齐。
- 新增最大填充量、对齐阈值或多数投票配置。
- 采用 Brittany 式逐语法节点 column-layout 系统。
- 改变 `.nixfmt.toml` 的发现规则、`--config` 行为或 `max_width` 的既有软限制语义。

## 进一步说明

- 领域术语和完整规则记录在 Formatter Configuration context 中。
- 架构取舍记录在 ADR 0005：在基础格式化后统一对齐行尾注释。
- Haskell formatter 对比记录在 `research-haskell-comment-alignment.md`。Ormolu 和 Fourmolu 不主动对齐连续行尾注释；Brittany 的对齐依赖 syntax-specific column blocks，因此不是本功能的直接实现模板。
