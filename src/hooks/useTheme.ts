import { useState, useEffect, useCallback, useRef } from 'react';

export type Theme = 'spring' | 'summer' | 'autumn' | 'winter';

interface ThemeConfig {
  id: Theme;
  name: string;
  icon: string;
  description: string;
  clashColors: [string, string];
}

export const themes: ThemeConfig[] = [
  {
    id: 'spring',
    name: '春',
    icon: '🌸',
    description: '粉绿撞色 - 樱花粉与新芽绿',
    clashColors: ['#FFB7C5', '#90EE90']
  },
  {
    id: 'summer',
    name: '夏',
    icon: '☀️',
    description: '蓝橙撞色 - 海洋蓝与阳光橙',
    clashColors: ['#00BFFF', '#FFA500']
  },
  {
    id: 'autumn',
    name: '秋',
    icon: '🍁',
    description: '红黄撞色 - 枫叶红与银杏黄',
    clashColors: ['#FF6B35', '#FFD700']
  },
  {
    id: 'winter',
    name: '冬',
    icon: '❄️',
    description: '蓝白撞色 - 冰蓝与雪白',
    clashColors: ['#87CEEB', '#E0F6FF']
  }
];

const STORAGE_KEY = 'moviemaster-theme';

// 全局主题状态，避免多个组件使用时的重复渲染
let globalTheme: Theme = 'spring';
let listeners: Set<(theme: Theme) => void> = new Set();

const notifyListeners = (theme: Theme) => {
  listeners.forEach(listener => listener(theme));
};

export function useTheme() {
  const [currentTheme, setCurrentTheme] = useState<Theme>(() => {
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem(STORAGE_KEY) as Theme;
      if (saved && themes.find(t => t.id === saved)) {
        globalTheme = saved;
        return saved;
      }
    }
    return globalTheme;
  });

  // 同步全局状态
  useEffect(() => {
    const handleThemeChange = (theme: Theme) => {
      setCurrentTheme(theme);
    };

    listeners.add(handleThemeChange);
    return () => {
      listeners.delete(handleThemeChange);
    };
  }, []);

  // 应用主题到 document - 只执行一次 DOM 操作
  useEffect(() => {
    const root = document.documentElement;

    // 直接设置属性，避免触发 React 重渲染
    root.setAttribute('data-theme', currentTheme);
    localStorage.setItem(STORAGE_KEY, currentTheme);

    // 同步全局状态
    globalTheme = currentTheme;
  }, [currentTheme]);

  // 切换主题 - 使用函数式更新避免依赖
  const setTheme = useCallback((theme: Theme) => {
    if (theme !== globalTheme) {
      globalTheme = theme;
      setCurrentTheme(theme);
      notifyListeners(theme);
    }
  }, []);

  // 切换到下一个主题
  const cycleTheme = useCallback(() => {
    const currentIndex = themes.findIndex(t => t.id === globalTheme);
    const nextIndex = (currentIndex + 1) % themes.length;
    const nextTheme = themes[nextIndex].id;
    setTheme(nextTheme);
  }, [setTheme]);

  // 获取当前主题配置 - 使用 ref 缓存
  const themeConfigRef = useRef(themes.find(t => t.id === currentTheme) || themes[0]);
  useEffect(() => {
    themeConfigRef.current = themes.find(t => t.id === currentTheme) || themes[0];
  }, [currentTheme]);

  return {
    currentTheme,
    currentThemeConfig: themeConfigRef.current,
    setTheme,
    cycleTheme,
    themes
  };
}

export default useTheme;
