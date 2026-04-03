interface StatItemProps {
  label: string
  value: number
}

export function StatItem({ label, value }: StatItemProps) {
  return (
    <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", fontSize: "14px" }}>
      <span style={{ color: "var(--color-muted-foreground)" }}>{label}</span>
      <span style={{ 
        fontWeight: 600, 
        color: "var(--color-foreground)",
        backgroundColor: "var(--color-background)",
        padding: "2px 8px",
        borderRadius: "4px",
        minWidth: "32px",
        textAlign: "center"
      }}>
        {value}
      </span>
    </div>
  )
}