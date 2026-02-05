use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum MaybeMany {
    One(HashMap<String, Value>),
    Many(Vec<HashMap<String, Value>>),
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Response {
    pub code: u8,
    pub message: String,
    pub data: Option<MaybeMany>,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DataBase {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: Option<u16>,
    pub database_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Request {
    pub command: String,
    pub database: DataBase,
    pub table: String,
    pub columns: Option<Vec<String>>,
    pub args: HashMap<String, Value>,
    pub data: Value,
}

/// 数据库操作指令枚举
#[derive(Debug)]
pub(crate) enum Command {
    Create,   // 创建数据库或表
    NewTable, // 创建新表单
    Insert,   // 向表中插入数据
    Update,   // 更新表中现有数据
    Delete,   // 从表中删除数据
    Query,    // 查询表中数据
    NONE, // 无指令
}

/// 条件判断操作符枚举
/// 用于定义数值或字符串的比较逻辑
#[derive(Debug, Clone)]
pub(crate) enum Judge {
    Between, // 介于 a 和 b 之间 (b > a)
    GT,      // 大于 (>)
    GTE,     // 大于等于 (>=)
    LT,      // 小于 (<)
    LTE,     // 小于等于 (<=)
    NE,      // 不等于 (!=)
    EQ,      // 等于 (=)
    IsNot,   // 非条件(通常为Is Not Null)
    NONE,     // 该判断条件不存在
}
#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}
#[derive(Debug, Clone)]
pub enum Condition {
    // 逻辑组：包含操作符 (And/Or) 和 子条件列表
    Group {
        op: LogicalOp,
        children: Vec<Condition>,
    },
    // 表达式：列名 + 操作符 + 参数列表
    // 例如: col="a", op="eq", args=[1]
    Expr {
        col: String,
        op: Judge,
        args: Vec<Value>, // 这里存放 ["eq", 1] 中的 1，或者 ["between", 2, 3] 中的 2, 3
    },
    NONE
}
#[derive(Debug)]
pub struct AnalysisArgs{
    pub where_logic: Condition,
    pub limit: Option<u32>,
    pub order_by: Option<HashMap<String, OrderBy>>,
    pub data: Option<MaybeMany>
}

#[derive(Debug)]
pub enum OrderBy{
    ASC,
    DESC,
    NONE
}