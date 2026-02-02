use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use sqlx::MySqlPool;
use dashmap::DashMap;


pub struct DBPoolManager {
    pools: Arc<DashMap<String, MySqlPool>>, // 数据库连接池
    count_dash: Arc<DashMap<String, AtomicU32>>, // 数据库连接池对应计数器
    global_call_count: Arc<AtomicU32>, // 全局计数器
}

impl DBPoolManager {
    pub fn new() -> Self {
        DBPoolManager {
            pools: Arc::new(DashMap::new()),
            count_dash: Arc::new(DashMap::new()),
            global_call_count: Arc::new(AtomicU32::new(0)),
        }
    }
    /// -----------------全局计数相关方法-----------------
    /// 重置全局计数
    fn reset_global_call(&self) {
        self.global_call_count.store(0, Ordering::SeqCst);
    }
    /// 增加全局计数并返回
    fn add_and_get_global_call(&self) -> u32 {
        // fetch_add 返回旧值，所以加1后就是新值
        self.global_call_count.fetch_add(1, Ordering::SeqCst) + 1
    }
    /// -----------------指定 pool 计数相关方法-----------------
    /// 获取某个 pool 的调用次数
    pub fn get_call_count(&self, key: &str) -> Option<u32> {
        self.count_dash.get(key).map(|c| c.load(Ordering::SeqCst))
    }
    /// 重置某个 pool 的调用次数
    pub fn reset_call_count(&self, key: &str) {
        if let Some(counter) = self.count_dash.get(key) {
            counter.store(0, Ordering::SeqCst);
        }
    }
    /// 为某个 pool 添加调用次数
    fn add_call_count(&self, key: &str) {
        if let Some(counter) = self.count_dash.get(key) {
            counter.fetch_add(1, Ordering::SeqCst);
        }
    }
    /// -----------------指定 pool 控制方法-----------------
    /// 插入新的 pool
    pub fn insert_pool(&self, key: String, pool: MySqlPool) {
        self.count_dash.entry(key.clone()).or_insert_with(|| AtomicU32::new(0));
        // 再插入 pool
        self.pools.insert(key, pool);
    }
    /// 获取指定 pool
    pub async fn get_pool(&self, pool_name: &str) -> Result<MySqlPool, Box<dyn std::error::Error>> {
        let pool = {
            match self.pools.get(pool_name) {
                Some(pool_ref) => pool_ref.value().clone(),
                None => return Err("Pool not found".into()),
            }
        };

        self.add_call_count(pool_name);
        // 获取增加后的当前值
        let current_global = self.add_and_get_global_call();
        if current_global >= 30 {
            // 触发清理逻辑
            self.reset_global_call(); // 先重置计数器
            self.evict_lowest_pool(); // 执行淘汰
        }
        Ok(pool)
    }
    /// 检查某个 pool 是否存在
    pub fn check_pool(&self, pool_name: &str) -> bool {
        self.pools.contains_key(pool_name)
    }
    /// 删除某个 pool
    pub fn remove_pool(&self, pool_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self.pools.remove(pool_name) {
            Some(_) => {
                // 同时移除计数器
                self.count_dash.remove(pool_name);
                Ok(())
            }
            None => Err("Pool not found".into()),
        }
    }
    /// 寻找并移除调用次数最少的 Pool
    fn evict_lowest_pool(&self) {
        // 遍历计数器找到最小值
        let mut min_key: Option<String> = None;
        let mut min_count: u32 = u32::MAX;

        // 只读遍历计数器
        for entry in self.count_dash.iter() {
            let count = entry.value().load(Ordering::SeqCst);
            if count < min_count {
                min_count = count;
                min_key = Some(entry.key().clone());
            }
        }
        if let Some(key) = min_key {
            let _ = self.remove_pool(&key);
        }
    }
}