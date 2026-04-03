import type { ReactNode } from "react"

interface SettingItemProps {
  label: string
  description?: string
  children: ReactNode
}

export function SettingItem({
  label,
  description,
  children
}: SettingItemProps) {
  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
      <div>
        <label style={{ fontSize: "14px", fontWeight: 500, color: "var(--color-foreground)" }}>{label}</label>
        {description && <p style={{ fontSize: "12px", color: "var(--color-muted-foreground)", marginTop: "2px" }}>{description}</p>}
      </div>
      {children}
    </div>
  )
}