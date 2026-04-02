import { cn } from "@/lib/utils"

interface SidebarProps {
  className?: string
  children: React.ReactNode
}

export function Sidebar({ className, children }: SidebarProps) {
  return (
    <aside
      className={cn(
        "flex h-full w-64 flex-col border-r bg-card",
        className
      )}
    >
      {children}
    </aside>
  )
}

export function SidebarHeader({ className, children }: SidebarProps) {
  return (
    <div className={cn("border-b p-4", className)}>
      {children}
    </div>
  )
}

export function SidebarContent({ className, children }: SidebarProps) {
  return (
    <div className={cn("flex-1 overflow-y-auto p-2", className)}>
      {children}
    </div>
  )
}

export function SidebarFooter({ className, children }: SidebarProps) {
  return (
    <div className={cn("border-t p-4", className)}>
      {children}
    </div>
  )
}

interface NavItemProps {
  icon?: React.ReactNode
  label: string
  active?: boolean
  onClick?: () => void
}

export function NavItem({ icon, label, active, onClick }: NavItemProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors",
        active
          ? "bg-accent text-accent-foreground"
          : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
      )}
    >
      {icon}
      <span>{label}</span>
    </button>
  )
}
