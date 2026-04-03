import type { ReactNode } from "react"

interface EmptyStateProps {
  icon: ReactNode
  title: string
  description: string
}

export function EmptyState({ icon, title, description }: EmptyStateProps) {
  return (
    <div style={{ 
      display: "flex", 
      flexDirection: "column", 
      alignItems: "center", 
      justifyContent: "center", 
      padding: "80px 20px",
      color: "var(--color-muted-foreground)"
    }}>
      <div style={{
        width: "80px",
        height: "80px",
        borderRadius: "50%",
        backgroundColor: "var(--color-muted)",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        marginBottom: "16px"
      }}>
        {icon}
      </div>
      <h3 style={{ fontSize: "18px", fontWeight: 600, color: "var(--color-foreground)", marginBottom: "4px" }}>{title}</h3>
      <p style={{ fontSize: "14px" }}>{description}</p>
    </div>
  )
}