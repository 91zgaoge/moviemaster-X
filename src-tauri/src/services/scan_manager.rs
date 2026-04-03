//! 扫描状态管理 - 支持后台扫描和进度追踪

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 扫描进度数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgressData {
    pub directory_id: i64,
    pub directory_name: String,
    pub current_file: String,
    pub processed: usize,
    pub found: usize,
}

/// 扫描完成数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCompletedData {
    pub directory_id: i64,
    pub total_found: usize,
}

/// 扫描错误数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanErrorData {
    pub directory_id: i64,
    pub message: String,
}

/// 扫描状态 - 使用扁平结构便于前端解析
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ScanStatus {
    Idle,
    Scanning(ScanProgressData),
    Completed(ScanCompletedData),
    Error(ScanErrorData),
}

impl Default for ScanStatus {
    fn default() -> Self {
        ScanStatus::Idle
    }
}

/// 扫描状态管理器
#[derive(Debug, Default)]
pub struct ScanManager {
    statuses: Mutex<HashMap<i64, ScanStatus>>,
    global_status: Mutex<ScanStatus>,
}

impl ScanManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// 开始扫描
    pub fn start_scan(&self, directory_id: i64, directory_name: String) {
        let status = ScanStatus::Scanning(ScanProgressData {
            directory_id,
            directory_name: directory_name.clone(),
            current_file: String::new(),
            processed: 0,
            found: 0,
        });

        let mut statuses = self.statuses.lock().unwrap();
        statuses.insert(directory_id, status.clone());

        let mut global = self.global_status.lock().unwrap();
        *global = status;
    }

    /// 更新扫描进度
    pub fn update_progress(
        &self,
        directory_id: i64,
        current_file: String,
        processed: usize,
        found: usize,
    ) {
        let mut statuses = self.statuses.lock().unwrap();
        if let Some(status) = statuses.get_mut(&directory_id) {
            let directory_name = match status {
                ScanStatus::Scanning(data) => data.directory_name.clone(),
                _ => String::new(),
            };

            let new_status = ScanStatus::Scanning(ScanProgressData {
                directory_id,
                directory_name,
                current_file,
                processed,
                found,
            });
            *status = new_status.clone();

            let mut global = self.global_status.lock().unwrap();
            *global = new_status;
        }
    }

    /// 完成扫描
    pub fn complete_scan(&self, directory_id: i64, total_found: usize) {
        let status = ScanStatus::Completed(ScanCompletedData {
            directory_id,
            total_found,
        });

        let mut statuses = self.statuses.lock().unwrap();
        statuses.insert(directory_id, status.clone());

        let mut global = self.global_status.lock().unwrap();
        *global = status;
    }

    /// 扫描出错
    pub fn error_scan(&self, directory_id: i64, message: String) {
        let status = ScanStatus::Error(ScanErrorData {
            directory_id,
            message,
        });

        let mut statuses = self.statuses.lock().unwrap();
        statuses.insert(directory_id, status.clone());

        let mut global = self.global_status.lock().unwrap();
        *global = status;
    }

    /// 获取指定目录的扫描状态
    pub fn get_status(&self, directory_id: i64) -> ScanStatus {
        let statuses = self.statuses.lock().unwrap();
        statuses.get(&directory_id).cloned().unwrap_or(ScanStatus::Idle)
    }

    /// 获取全局扫描状态
    pub fn get_global_status(&self) -> ScanStatus {
        self.global_status.lock().unwrap().clone()
    }

    /// 清除扫描状态
    pub fn clear_status(&self, directory_id: i64) {
        let mut statuses = self.statuses.lock().unwrap();
        statuses.remove(&directory_id);
        
        let mut global = self.global_status.lock().unwrap();
        if matches!(*global, ScanStatus::Completed { .. } | ScanStatus::Error { .. }) {
            *global = ScanStatus::Idle;
        }
    }

    /// 是否正在扫描
    pub fn is_scanning(&self, directory_id: i64) -> bool {
        matches!(self.get_status(directory_id), ScanStatus::Scanning(_))
    }

    /// 获取所有活跃的扫描
    pub fn get_active_scans(&self) -> Vec<ScanStatus> {
        let statuses = self.statuses.lock().unwrap();
        statuses
            .values()
            .filter(|s| matches!(s, ScanStatus::Scanning(_)))
            .cloned()
            .collect()
    }
}

impl Clone for ScanManager {
    fn clone(&self) -> Self {
        Self::default()
    }
}

// 创建全局扫描管理器单例
pub fn create_scan_manager() -> Arc<ScanManager> {
    Arc::new(ScanManager::new())
}