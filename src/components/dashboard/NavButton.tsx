import { useState } from "react"
import type { ReactNode } from "react"

interface NavButtonProps {
  icon: ReactNode
  label: string
  active?: boolean
  onClick?: () => void
  variant?: "default" | "orange"
}

export function NavButton({
  icon,
  label,
  active,
  onClick,
  variant = "default"
}: NavButtonProps) {
  const [isHovered, setIsHovered] = useState(false)
  const isOrange = variant === "orange"

  const getBackground = () => {
    if (active) {
      return isOrange
        ? "linear-gradient(135deg, #f97316 0%, #ea580c 50%, #dc2626 100%)"
        : "var(--color-primary)"
    }
    if (isHovered && isOrange) {
      return "linear-gradient(135deg, #f97316 0%, #ea580c 100%)"
    }
    return "transparent"
  }

  const getColor = () => {
    if (active) return "#ffffff"
    if (isHovered && isOrange) return "#ffffff"
    return "var(--color-muted-foreground)"
  }

  return (
    <button
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      style={{
        display: "flex",
        width: "100%",
        alignItems: "center",
        gap: "12px",
        padding: "10px 12px",
        borderRadius: "8px",
        fontSize: "14px",
        fontWeight: 500,
        border: "none",
        cursor: "pointer",
        transition: "all 0.2s ease",
        background: getBackground(),
        color: getColor(),
        boxShadow: active && isOrange ? "0 4px 12px rgba(249, 115, 22, 0.4)" : "none",
        transform: isHovered && isOrange && !active ? "scale(1.02)" : "scale(1)"
      }}
    >
      {icon}
      <span style={{ color: getColor() }}>{label}</span>
      {active && (
        <div style={{
          marginLeft: "auto",
          width: "6px",
          height: "6px",
          borderRadius: "50%",
          backgroundColor: "white",
          boxShadow: isOrange ? "0 0 8px rgba(255, 255, 255, 0.6)" : "none"
        }} />
      )}
    </button>
  )
}