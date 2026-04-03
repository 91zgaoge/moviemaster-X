import type { ReactNode } from "react"

interface FilterButtonProps {
  children: ReactNode
  active?: boolean
  onClick?: () => void
}

export function FilterButton({
  children,
  active,
  onClick
}: FilterButtonProps) {
  return (
    <button
      onClick={onClick}
      style={{
        padding: "6px 16px",
        borderRadius: "6px",
        fontSize: "14px",
        fontWeight: 500,
        border: "none",
        cursor: "pointer",
        transition: "all 0.2s",
        backgroundColor: active ? "var(--color-background)" : "transparent",
        color: active ? "var(--color-foreground)" : "var(--color-muted-foreground)",
        boxShadow: active ? "0 1px 3px rgba(0,0,0,0.1)" : "none"
      }}
    >
      {children}
    </button>
  )
}