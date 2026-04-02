//! SMB客户端模块 - 用于访问Windows网络共享

use std::path::Path;
use std::process::Command;

/// SMB连接信息
#[derive(Debug, Clone)]
pub struct SMBConnection {
    pub id: String,
    pub name: String,
    pub host: String,
    pub share: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub domain: Option<String>,
}

/// SMB客户端
pub struct SMBClient {
    connection: SMBConnection,
}

impl SMBClient {
    /// 创建新的SMB客户端
    pub fn new(connection: SMBConnection) -> Self {
        Self { connection }
    }

    /// 测试连接
    pub async fn test_connection(&self
    ) -> Result<bool, String> {
        #[cfg(target_os = "windows")]
        {
            // Windows: 使用net use测试
            let output = Command::new("net")
                .args([
                    "use",
                    &format!("\\\\{}\\{}", self.connection.host, self.connection.share),
                    "/user",
                    &self.connection.username.clone().unwrap_or_default(),
                    &self.connection.password.clone().unwrap_or_default(),
                ])
                .output()
                .map_err(|e| format!("Failed to test connection: {}", e))?;

            if output.status.success() {
                // 断开测试连接
                let _ = Command::new("net")
                    .args([
                        "use",
                        &format!("\\\\{}\\{}", self.connection.host, self.connection.share),
                        "/delete",
                    ])
                    .output();
                Ok(true)
            } else {
                Ok(false)
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            // macOS/Linux: 使用smbclient
            let output = Command::new("smbclient")
                .args([
                    "-L",
                    &self.connection.host,
                    "-U",
                    &format!(
                        "{}%{}",
                        self.connection.username.clone().unwrap_or_default(),
                        self.connection.password.clone().unwrap_or_default()
                    ),
                ])
                .output()
                .map_err(|e| format!("Failed to test connection: {}", e))?;

            Ok(output.status.success())
        }
    }

    /// 列出目录内容
    pub async fn list_directory(
        &self,
        path: &str,
    ) -> Result<Vec<SMBFileInfo>, String> {
        #[cfg(target_os = "windows")]
        {
            // Windows: 使用dir命令
            let unc_path = if path.is_empty() {
                format!("\\\\{}\\{}", self.connection.host, self.connection.share)
            } else {
                format!(
                    "\\\\{}\\{}\\{}",
                    self.connection.host, self.connection.share, path
                )
            };

            let output = Command::new("cmd")
                .args(["/C", "dir", "/B", &unc_path])
                .output()
                .map_err(|e| format!("Failed to list directory: {}", e))?;

            if !output.status.success() {
                return Err("Failed to list directory".to_string());
            }

            let content = String::from_utf8_lossy(&output.stdout);
            let files: Vec<SMBFileInfo> = content
                .lines()
                .filter(|line| !line.is_empty())
                .map(|name| SMBFileInfo {
                    name: name.to_string(),
                    is_directory: !name.contains('.'),
                    size: None,
                    modified: None,
                })
                .collect();

            Ok(files)
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("SMB browsing not implemented for this platform".to_string())
        }
    }

    /// 复制文件到本地
    pub async fn copy_file(
        &self,
        remote_path: &str,
        local_path: &Path,
    ) -> Result<(), String> {
        let unc_path = format!(
            "\\\\{}\\{}\\{}",
            self.connection.host, self.connection.share, remote_path
        );

        std::fs::copy(&unc_path, local_path)
            .map_err(|e| format!("Failed to copy file: {}", e))?;

        Ok(())
    }
}

/// SMB文件信息
#[derive(Debug, Clone)]
pub struct SMBFileInfo {
    pub name: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub modified: Option<String>,
}