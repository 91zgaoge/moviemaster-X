import * as React from "react"

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link"
  size?: "default" | "sm" | "lg" | "icon"
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ style, variant = "default", size = "default", ...props }, ref) => {
    const baseStyles: React.CSSProperties = {
      display: "inline-flex",
      alignItems: "center",
      justifyContent: "center",
      whiteSpace: "nowrap",
      borderRadius: "6px",
      fontSize: "14px",
      fontWeight: 500,
      cursor: "pointer",
      border: "none",
      transition: "all 0.2s"
    }

    const variantStyles: Record<string, React.CSSProperties> = {
      default: {
        backgroundColor: "var(--color-primary)",
        color: "var(--color-primary-foreground)",
        boxShadow: "0 1px 3px rgba(0,0,0,0.1)"
      },
      destructive: {
        backgroundColor: "var(--color-destructive)",
        color: "var(--color-destructive-foreground)"
      },
      outline: {
        backgroundColor: "transparent",
        border: "1px solid var(--color-border)",
        color: "var(--color-foreground)"
      },
      secondary: {
        backgroundColor: "var(--color-secondary)",
        color: "var(--color-secondary-foreground)"
      },
      ghost: {
        backgroundColor: "transparent",
        color: "var(--color-foreground)"
      },
      link: {
        backgroundColor: "transparent",
        color: "var(--color-primary)",
        textDecoration: "underline"
      }
    }

    const sizeStyles: Record<string, React.CSSProperties> = {
      default: {
        height: "36px",
        padding: "0 16px"
      },
      sm: {
        height: "32px",
        padding: "0 12px",
        fontSize: "12px"
      },
      lg: {
        height: "40px",
        padding: "0 32px"
      },
      icon: {
        height: "36px",
        width: "36px",
        padding: 0
      }
    }

    return (
      <button
        ref={ref}
        style={{
          ...baseStyles,
          ...variantStyles[variant],
          ...sizeStyles[size],
          ...style
        }}
        {...props}
      />
    )
  }
)
Button.displayName = "Button"

export { Button }