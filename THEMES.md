# 🎨 MovieMaster 撞色主题系统

## 概述

MovieMaster 引入了春夏秋冬四季撞色主题系统，为应用带来活力四射的视觉体验。

## 主题预览

### 🌸 春季主题 (Spring)
**撞色方案**: 樱花粉 + 新芽绿
- **主色**: `hsl(340 80% 65%)` - 温暖的樱花粉色
- **强调色**: `hsl(140 70% 75%)` - 清新的新芽绿色
- **背景**: 柔和的粉白色调
- **氛围**: 春意盎然，生机勃勃

```css
[data-theme="spring"]
```

### ☀️ 夏季主题 (Summer)
**撞色方案**: 海洋蓝 + 阳光橙
- **主色**: `hsl(195 100% 50%)` - 清凉的海洋蓝
- **强调色**: `hsl(35 100% 60%)` - 热情的阳光橙
- **背景**: 清爽的蓝白色调
- **氛围**: 夏日海滩，活力四射

```css
[data-theme="summer"]
```

### 🍁 秋季主题 (Autumn)
**撞色方案**: 枫叶红 + 银杏黄
- **主色**: `hsl(15 100% 55%)` - 热烈的枫叶红
- **强调色**: `hsl(50 100% 55%)` - 温暖的银杏黄
- **背景**: 温馨的米黄色调
- **氛围**: 金秋时节，温暖怀旧

```css
[data-theme="autumn"]
```

### ❄️ 冬季主题 (Winter)
**撞色方案**: 冰蓝 + 雪白 + 深青点缀
- **主色**: `hsl(195 70% 50%)` - 纯净的冰蓝色
- **强调色**: `hsl(180 100% 35%)` - 深邃的青色
- **背景**: 纯净的雪白色调
- **氛围**: 冰雪世界，纯净优雅

```css
[data-theme="winter"]
```

## 使用方式

### 1. 主题切换组件

```tsx
import { ThemeSwitcher } from "@/components/ThemeSwitcher"

// 完整版下拉菜单
<ThemeSwitcher />

// 简洁版图标按钮
<ThemeSwitcherCompact />

// 主题卡片网格
<ThemeCards />
```

### 2. Hook API

```tsx
import { useTheme } from "@/hooks/useTheme"

function MyComponent() {
  const { 
    currentTheme,      // 当前主题ID
    currentThemeConfig, // 当前主题配置
    setTheme,          // 设置主题
    cycleTheme,        // 循环切换
    themes             // 所有主题列表
  } = useTheme()

  return (
    <button onClick={() => setTheme('autumn')}>
      切换到秋季主题
    </button>
  )
}
```

### 3. CSS 工具类

```css
/* 撞色背景 */
.bg-clash-primary      /* 主撞色 */
.bg-clash-secondary    /* 次撞色 */
.bg-clash-gradient     /* 渐变撞色 */

/* 撞色文字 */
.text-clash-primary
.text-clash-secondary

/* 撞色边框 */
.border-clash-primary
.border-clash-secondary

/* 发光效果 */
.shadow-clash          /* 强发光 */
.shadow-clash-sm       /* 弱发光 */

/* 装饰效果 */
.seasonal-decoration   /* 季节背景装饰 */
.seasonal-glow         /* 发光边框效果 */
```

## 主题存储

主题选择会自动保存到 `localStorage`，键名为 `moviemaster-theme`。

## 文件结构

```
src/
├── hooks/
│   └── useTheme.ts          # 主题管理 Hook
├── components/
│   └── ThemeSwitcher.tsx    # 主题切换组件
├── index.css                # 主题样式定义
└── main.tsx                 # 主题初始化
```

## 自定义主题

要添加新主题，请在 `src/index.css` 中添加新的 `[data-theme="xxx"]` 区块：

```css
[data-theme="custom"] {
  --color-background: hsl(...);
  --color-foreground: hsl(...);
  --color-primary: hsl(...);
  --color-clash-primary: hsl(...);
  --color-clash-secondary: hsl(...);
  --color-clash-gradient: linear-gradient(...);
  /* ... */
}
```

然后在 `src/hooks/useTheme.ts` 的 `themes` 数组中添加主题配置。

## 设计原则

1. **撞色对比**: 每个主题使用互补色或对比色创造视觉冲击
2. **季节情感**: 色彩方案传达季节特有的情感和氛围
3. **可读性优先**: 确保文字在所有主题下都有良好的可读性
4. **平滑过渡**: 主题切换时带有流畅的过渡动画
