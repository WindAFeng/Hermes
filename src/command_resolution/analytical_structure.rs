use serde_json::Value;
use crate::command_resolution::command_model::{Command, Judge, Condition, LogicalOp, OrderBy};
use crate::command_resolution::{judge_text::judge_symbol, judge_text::judge_text};
pub fn get_command(cmd: &str) -> Command {
    match cmd {
        "create" => Command::Create,
        "new" => Command::NewTable,
        "insert" => Command::Insert,
        "update" => Command::Update,
        "delete" => Command::Delete,
        "query" => Command::Query,
        _ => match cmd.to_lowercase().as_str() {
            "create" => Command::Create,
            "new" => Command::NewTable,
            "insert" => Command::Insert,
            "update" => Command::Update,
            "delete" => Command::Delete,
            "query" => Command::Query,
            _ => Command::NONE,
        },
    }
}
pub fn get_order_by(order_by: &str) -> OrderBy {
    match order_by {
        "asc" => OrderBy::ASC,
        "desc" => OrderBy::DESC,
        _ => match order_by {
            "ASC" => OrderBy::ASC,
            "DESC" => OrderBy::DESC,
            _ => OrderBy::NONE,
        },
    }
}
fn get_judge(judge: &str) -> Judge {
    // 快速路径：常见的小写和符号
    match judge {
        judge_text::BETWEEN => Judge::Between,
        judge_text::IS_NOT => Judge::IsNot,
        judge_text::EQUAL | judge_symbol::EQUAL => Judge::EQ,
        judge_text::NOT_EQUAL | judge_symbol::NOT_EQUAL => Judge::NE,
        judge_text::GREATER_THAN | judge_symbol::GREATER_THAN => Judge::GT,
        judge_text::GREATER_THAN_EQUAL | judge_symbol::GREATER_THAN_EQUAL => Judge::GTE,
        judge_text::LESS_THAN | judge_symbol::LESS_THAN => Judge::LT,
        judge_text::LESS_THAN_EQUAL | judge_symbol::LESS_THAN_EQUAL => Judge::LTE,
        _ => {
            // 慢速路径：需要转换大小写
            match judge.to_lowercase().as_str() {
                judge_text::EQUAL => Judge::EQ,
                judge_text::NOT_EQUAL => Judge::NE,
                judge_text::GREATER_THAN => Judge::GT,
                judge_text::GREATER_THAN_EQUAL => Judge::GTE,
                judge_text::LESS_THAN => Judge::LT,
                judge_text::LESS_THAN_EQUAL => Judge::LTE,
                _ => Judge::NONE,
            }
        }
    }
}
pub fn parse_condition(value: &Value) -> Result<Condition, Box<dyn std::error::Error>> {
    match value {
        Value::Object(map) => {
            // 情况1：最顶层为 $or 或是 $and
            // 检查是否包含 $or
            if let Some(or_array) = map.get("$or") {
                // 获取其value
                let children = parse_array_of_conditions(or_array)?;
                return Ok(Condition::Group {
                    op: LogicalOp::Or,
                    children,
                });
            }
            // 检查是否包含 $and
            if let Some(and_array) = map.get("$and") {
                // 获取其value
                let children = parse_array_of_conditions(and_array)?;
                return Ok(Condition::Group {
                    op: LogicalOp::And,
                    children,
                })
            };
            // 情况2：这是一个普通的列条件 {"a": ["eq", 1], "b": ["between", 2, 3]}
            // 因为上面没命中 $or/$and，说明这是叶子节点
            // 遍历对象里的每一项
            let mut exprs = Vec::new();
            for (col_name, col_value) in map {
                // 解析 ["eq", 1] 这种结构
                let (op, args) = parse_expr_array(col_value)?;
                exprs.push(Condition::Expr {
                    col: col_name.clone(),
                    op,
                    args,
                });
            }
            // 如果有多列（a和b），默认是 AND 关系
            // 所以我们把它们包装成一个 And 组
            if exprs.len() == 1 {
                Ok(exprs.pop().unwrap())
            } else {
                Ok(Condition::Group {
                    op: LogicalOp::And,
                    children: exprs,
                })
            }
        },
        Value::Array(_) => {
            // 这里通常不会直接进来，除非顶层是数组，我们交给数组解析器
            Err("Unexpected array at top level".into())
        }
        _ => Err("Invalid condition format".into()),
    }
}
// 解析 ["eq", 1] 或 ["between", 2, 3]
fn parse_expr_array(value: &Value) -> Result<(Judge, Vec<Value>), Box<dyn std::error::Error>> {
    match value {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err("Empty operator array".into());
            }

            // 0号位是操作符
            let op_str = arr.get(0).and_then(|v| v.as_str()).ok_or("Invalid op")?;

            // 1号位及以后都是参数
            let args = arr.iter().skip(1).cloned().collect::<Vec<_>>();

            // 将字符串转换为 Op 枚举
            let op = get_judge(op_str);

            Ok((op, args))
        }
        Value::String(str) => {
            Ok((Judge::EQ, vec![Value::String(str.clone())]))
        }
        Value::Number(num) => {
            Ok((Judge::EQ, vec![Value::Number(num.clone())]))
        }
        Value::Bool(bool) => {
            Ok((Judge::EQ, vec![Value::Bool(bool.clone())]))
        }
        _ => Err("Expr must be an array".into())
    }
}
// 解析逻辑组内部的数组 [ ... ]
fn parse_array_of_conditions(value: &Value) -> Result<Vec<Condition>, Box<dyn std::error::Error>> {
    if let Value::Array(items) = value {
        let mut conditions = Vec::new();
        for item in items {
            // 递归解析数组里的每一项
            let cond = parse_condition(item)?;
            conditions.push(cond);
        }
        Ok(conditions)
    } else {
        Err("Expected array".into())
    }
}
