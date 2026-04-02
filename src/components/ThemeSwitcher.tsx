import { useState, useEffect } from 'react';

export type Theme = 'spring' | 'summer' | 'autumn' | 'winter';

const themes = [
  { id: 'spring' as Theme, name: '春', icon: '🌸', desc: '粉绿撞色' },
  { id: 'summer' as Theme, name: '夏', icon: '☀️', desc: '蓝橙撞色' },
  { id: 'autumn' as Theme, name: '秋', icon: '🍁', desc: '红黄撞色' },
  { id: 'winter' as Theme, name: '冬', icon: '❄️', desc: '蓝白撞色' },
];

const STORAGE_KEY = 'moviemaster-theme-v2';

// 获取保存的主题
function getSavedTheme(): Theme {
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved && themes.find(t => t.id === saved)) {
    return saved as Theme;
  }
  return 'spring';
}

// 设置主题
function setThemeAttr(theme: Theme) {
  document.documentElement.setAttribute('data-theme', theme);
  localStorage.setItem(STORAGE_KEY, theme);
}

export function ThemeSwitcher() {
  const [currentTheme, setCurrentTheme] = useState<Theme>(getSavedTheme);
  const [showPanel, setShowPanel] = useState(false);

  // 初始化主题
  useEffect(() => {
    setThemeAttr(currentTheme);
  }, []);

  const handleThemeChange = (theme: Theme) => {
    setCurrentTheme(theme);
    setThemeAttr(theme);
    setShowPanel(false);
  };

  const current = themes.find(t => t.id === currentTheme) || themes[0];

  return (
    <div style={{ position: 'relative' }}>
      <button
        onClick={() => setShowPanel(!showPanel)}
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: '6px',
          padding: '6px 12px',
          borderRadius: '6px',
          border: '1px solid var(--color-border)',
          backgroundColor: 'var(--color-secondary)',
          color: 'var(--color-foreground)',
          cursor: 'pointer',
          fontSize: '14px',
        }}
      >
        <span>{current.icon}</span>
        <span>{current.name}</span>
      </button>

      {showPanel && (
        <>
          <div
            style={{
              position: 'fixed',
              inset: 0,
              backgroundColor: 'rgba(0,0,0,0.5)',
              zIndex: 40,
            }}
            onClick={() => setShowPanel(false)}
          />
          <div
            style={{
              position: 'fixed',
              top: '50%',
              left: '50%',
              transform: 'translate(-50%, -50%)',
              backgroundColor: 'var(--color-card)',
              borderRadius: '12px',
              padding: '20px',
              boxShadow: '0 25px 50px -12px rgba(0,0,0,0.25)',
              zIndex: 50,
              minWidth: '300px',
            }}
          >
            <h3 style={{ margin: '0 0 16px 0', fontSize: '16px' }}>选择主题</h3>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '10px' }}>
              {themes.map(theme => (
                <button
                  key={theme.id}
                  onClick={() => handleThemeChange(theme.id)}
                  style={{
                    padding: '12px',
                    borderRadius: '8px',
                    border: `2px solid ${currentTheme === theme.id ? 'var(--color-primary)' : 'var(--color-border)'}`,
                    backgroundColor: currentTheme === theme.id ? 'var(--color-accent)' : 'var(--color-secondary)',
                    cursor: 'pointer',
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    gap: '4px',
                  }}
                >
                  <span style={{ fontSize: '24px' }}>{theme.icon}</span>
                  <span style={{ fontSize: '12px', fontWeight: 600 }}>{theme.name}</span>
                  <span style={{ fontSize: '10px', color: 'var(--color-muted-foreground)' }}>{theme.desc}</span>
                </button>
              ))}
            </div>
          </div>
        </>
      )}
    </div>
  );
}

