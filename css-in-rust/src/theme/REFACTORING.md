# Theme 系统重构文档

## 重构概述

本次重构基于思考计划中的分层架构设计，对 Theme 系统进行了模块化重构，明确了各层职责，优化了代码结构，并增强了性能优化相关功能。

## 分层架构

重构后的 Theme 系统采用三层架构：

1. **核心层 (core)**：
   - 提供基础的 Token 系统、CSS 生成、缓存机制等核心功能
   - 职责清晰，依赖最小化
   - 位于 `src/theme/core/`

2. **适配层 (adapter)**：
   - 连接核心层和框架层
   - 提供通用的适配接口
   - 位于 `src/theme/adapter/`

3. **系统层 (systems)**：
   - 提供特定领域的系统实现，如颜色系统、排版系统等
   - 位于 `src/theme/systems/`

## 已完成的重构内容

1. **目录结构重构**：
   - 创建了清晰的分层目录结构
   - 将相关功能按职责归类到对应模块

2. **核心层优化**：
   - 拆分了 Token 系统相关功能
   - 增强了 CSS 生成和变量管理
   - 实现了基础缓存机制
   - 添加了主题历史记录功能

3. **适配层实现**：
   - 创建了样式注入适配器
   - 实现了 SSR 支持和客户端水合
   - 增强了主题提供者功能
   - 添加了框架适配器（Dioxus、React）

4. **性能优化**：
   - 实现了组件级样式缓存
   - 增加了哈希计算和依赖追踪
   - 优化了 CSS 生成流程
   - 实现了样式依赖追踪

## 主要功能增强

1. **样式依赖追踪**：
   - 实现了 `DependencyTracker` 用于追踪样式依赖关系
   - 支持变量依赖、主题依赖、组件依赖和媒体查询依赖
   - 提供依赖环检测功能

2. **SSR 支持**：
   - 实现了 `SsrSupport` 用于服务端渲染
   - 添加了 `HydrationEngine` 用于客户端水合
   - 支持多种水合模式（完全、部分、延迟、渐进式）

3. **框架适配器**：
   - 实现了 `DioxusAdapter` 用于 Dioxus 框架
   - 实现了 `ReactAdapter` 用于 React 框架
   - 提供了框架特定的钩子和工具函数

4. **组件级缓存**：
   - 实现了 `ComponentStyleCache` 用于缓存组件样式
   - 基于组件、属性和主题的哈希计算
   - 支持缓存统计和优化

## 测试覆盖

为了确保重构的质量，我们添加了以下测试：

1. **单元测试**：
   - 主题管理器测试
   - 主题提供者测试
   - 组件缓存测试
   - SSR 支持测试
   - 框架适配器测试

2. **集成测试**：
   - 主题系统集成测试
   - 样式生成和注入测试

## 文档完善

为了便于使用和维护，我们添加了以下文档：

1. **README.md**：
   - 架构概述
   - 模块说明
   - 使用示例
   - 性能优化
   - 扩展指南

2. **REFACTORING.md**：
   - 重构概述
   - 分层架构
   - 已完成的重构内容
   - 主要功能增强
   - 测试覆盖

## 后续工作

虽然我们已经完成了大部分重构工作，但仍有一些后续工作需要完成：

1. **性能基准测试**：
   - 对比重构前后的性能差异
   - 识别潜在的性能瓶颈

2. **更多框架适配器**：
   - 添加 Yew 框架适配器
   - 添加 Leptos 框架适配器

3. **高级功能**：
   - 主题动画过渡
   - 主题预设管理
   - 主题编辑器

4. **文档和示例**：
   - 添加更多使用示例
   - 完善 API 文档
   - 创建示例应用

## 待完成的工作

1. **完善 SSR 支持**：
   - 增强服务端渲染的样式处理
   - 实现客户端水合优化

2. **样式依赖追踪**：
   - 完善变量依赖关系的追踪
   - 优化样式更新策略

3. **框架适配器**：
   - 实现针对不同框架的适配器
   - 提供框架特定的生命周期管理

4. **测试覆盖**：
   - 增加单元测试覆盖率
   - 添加集成测试

5. **文档完善**：
   - 更新 API 文档
   - 提供使用示例

## 迁移指南

### 从旧版本迁移

1. **导入变更**：
   ```rust
   // 旧版导入
   use crate::theme::{theme_provider::ThemeProvider, theme_manager::ThemeManager};

   // 新版导入
   use crate::theme::{
       core::provider::ThemeProvider,
       adapter::provider::ThemeProviderAdapter,
   };
   ```

2. **API 变更**：
   ```rust
   // 旧版 API
   let provider = ThemeProvider::new();
   provider.switch_theme("dark");

   // 新版 API
   let core_provider = ThemeProvider::new(ThemeManager::default(), ThemeProviderConfig::default());
   let mut adapter = ThemeProviderAdapter::new(Arc::new(core_provider));
   adapter.switch_theme("dark").unwrap();
   ```

### 性能优化建议

1. **使用组件级缓存**：
   ```rust
   let mut cache = ComponentStyleCache::new();

   // 计算组件属性哈希
   let props_hash = compute_props_hash(&props);
   let theme_hash = compute_theme_hash(&theme.name, &format!("{:?}", theme.mode));

   let key = ComponentCacheKey {
       component: "Button".to_string(),
       props_hash,
       theme_hash,
   };

   // 尝试从缓存获取
   if let Some(style) = cache.get(&key) {
       return style.css.clone();
   }

   // 生成样式并缓存
   let css = generate_component_css();
   let style = CachedComponentStyle {
       class_name: "btn-primary".to_string(),
       css: css.clone(),
       variables: extract_variables(&css),
       timestamp: current_timestamp(),
       usage_count: 0,
       style_hash: compute_style_hash(&css),
   };

   cache.set(key, style);
   ```

2. **优化 CSS 生成**：
   ```rust
   let optimizer = StyleOptimizer::new(OptimizeConfig {
       minify: true,
       remove_unused: true,
       merge_rules: true,
       optimize_selectors: true,
   });

   let optimized_css = optimizer.optimize(&raw_css);
   ```

## 下一步计划

1. **完成迁移**：将旧的 `theme_manager.rs` 和 `theme_provider.rs` 中的所有功能迁移到新架构
2. **删除旧文件**：完成迁移后删除旧文件
3. **增强测试**：为新架构添加全面的测试
4. **性能基准测试**：对比新旧架构的性能差异
5. **文档更新**：更新所有文档以反映新架构
