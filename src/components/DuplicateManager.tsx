import { useState, useEffect } from "react"
import { scanDuplicates, deleteDuplicates, type DuplicateGroup } from "@/lib/api"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Check, Trash2, FileVideo } from "lucide-react"

interface DuplicateManagerProps {
  onClose?: () => void
}

export function DuplicateManager({ onClose }: DuplicateManagerProps) {
  const [groups, setGroups] = useState<DuplicateGroup[]>([])
  const [loading, setLoading] = useState(false)
  const [deleting, setDeleting] = useState<number | null>(null)

  useEffect(() => {
    handleScan()
  }, [])

  const handleScan = async () => {
    setLoading(true)
    try {
      const result = await scanDuplicates()
      setGroups(result)
    } catch (error) {
      console.error("Scan failed:", error)
    } finally {
      setLoading(false)
    }
  }

  const handleDelete = async (group: DuplicateGroup, deleteIds: number[]) => {
    if (deleteIds.length === 0) return

    setDeleting(group.suggested_keep)
    try {
      await deleteDuplicates(group.suggested_keep, deleteIds)
      // Refresh list
      await handleScan()
    } catch (error) {
      console.error("Delete failed:", error)
    } finally {
      setDeleting(null)
    }
  }

  const formatSize = (bytes: number | null) => {
    if (!bytes) return "Unknown"
    const gb = bytes / (1024 * 1024 * 1024)
    if (gb >= 1) return `${gb.toFixed(2)} GB`
    const mb = bytes / (1024 * 1024)
    return `${mb.toFixed(2)} MB`
  }

  if (loading) {
    return (
      <div style={{ padding: "24px", textAlign: "center" }}>
        <div>Scanning for duplicates...</div>
      </div>
    )
  }

  if (groups.length === 0) {
    return (
      <div style={{ padding: "24px", textAlign: "center" }}>
        <Check style={{ width: "48px", height: "48px", color: "#22c55e", margin: "0 auto" }} />
        <p>No duplicates found!</p>
        <Button variant="outline" onClick={onClose}>Close</Button>
      </div>
    )
  }

  return (
    <div style={{ padding: "24px", maxHeight: "80vh", overflow: "auto" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "24px" }}>
        <h2 style={{ margin: 0 }}>Duplicate Files ({groups.length} groups)</h2>
        <div style={{ display: "flex", gap: "8px" }}>
          <Button variant="outline" onClick={handleScan} disabled={loading}>
            {loading ? "Scanning..." : "Rescan"}
          </Button>
          <Button variant="outline" onClick={onClose}>Close</Button>
        </div>
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
        {groups.map((group, idx) => (
          <Card key={idx}>
            <CardContent style={{ padding: "16px" }}>
              <div style={{ marginBottom: "12px" }}>
                <span style={{
                  fontSize: "12px",
                  padding: "4px 8px",
                  borderRadius: "4px",
                  backgroundColor: group.match_type === "hash" ? "#dbeafe" : "#f3f4f6",
                  color: group.match_type === "hash" ? "#1d4ed8" : "#374151"
                }}>
                  {group.match_type.toUpperCase()}
                </span>
                <span style={{ marginLeft: "12px", fontSize: "14px", color: "#6b7280" }}>
                  {group.movies.length} files
                </span>
              </div>

              <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
                {group.movies.map((movie) => (
                  <div
                    key={movie.id}
                    style={{
                      display: "flex",
                      alignItems: "center",
                      gap: "12px",
                      padding: "12px",
                      borderRadius: "8px",
                      backgroundColor: movie.id === group.suggested_keep ? "#dcfce7" : "#f9fafb",
                      border: movie.id === group.suggested_keep ? "1px solid #22c55e" : "1px solid #e5e7eb"
                    }}
                  >
                    <FileVideo style={{ width: "20px", height: "20px", color: "#6b7280" }} />

                    <div style={{ flex: 1, minWidth: 0 }}>
                      <div style={{ fontWeight: 500, fontSize: "14px" }}>
                        {movie.cnname || movie.filename}
                      </div>
                      <div style={{ fontSize: "12px", color: "#6b7280", marginTop: "4px" }}>
                        {movie.path}
                      </div>
                      <div style={{ fontSize: "12px", color: "#9ca3af", marginTop: "2px" }}>
                        Size: {formatSize(movie.file_size)} | Score: {movie.completeness_score}/100
                        {movie.id === group.suggested_keep && (
                          <span style={{ color: "#22c55e", marginLeft: "8px" }}>✓ Keep this</span>
                        )}
                      </div>
                    </div>
                  </div>
                ))}
              </div>

              <div style={{ marginTop: "16px", display: "flex", gap: "8px", justifyContent: "flex-end" }}>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => {
                    const deleteIds = group.movies
                      .filter(m => m.id !== group.suggested_keep)
                      .map(m => m.id)
                    handleDelete(group, deleteIds)
                  }}
                  disabled={deleting === group.suggested_keep}
                >
                  <Trash2 style={{ width: "14px", height: "14px", marginRight: "4px" }} />
                  {deleting === group.suggested_keep ? "Deleting..." : "Delete Duplicates"}
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  )
}
