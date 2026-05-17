use crate::models::hermes_model::hermes_error::HermesError;
use redis::{Cmd, Pipeline};
use std::collections::HashMap;
use std::mem;
pub struct RedisCmd {
    pub cmd: String,
    pub args: Option<Vec<String>>,
}
pub struct RedisCmdBuilder {
    cmds: Vec<RedisCmd>,
    cmd: String,
    args: Option<Vec<String>>,
}
impl RedisCmdBuilder {
    pub fn new() -> Self {
        Self {
            cmds: Vec::new(),
            cmd: "".to_string(),
            args: None,
        }
    }
    pub fn cmd(&mut self, command: &str) -> &mut Self {
        self.args = None;
        self.cmd = command.to_owned();
        self
    }
    fn push_args<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let args = self.args.get_or_insert_with(Vec::new);
        args.extend(iter.into_iter().map(|x| x.to_string()));
    }
    pub fn string_arg(&mut self, arg: &str) -> &mut Self {
        self.push_args([arg]);
        self
    }
    pub fn vec_arg(&mut self, vec_arg: &[String]) -> &mut Self {
        self.push_args(vec_arg.iter());
        self
    }
    pub fn hash_arg(&mut self, hash_arg: HashMap<String, String>) -> &mut Self {
        let pairs = hash_arg.into_iter().flat_map(|(k, v)| [k, v]);
        self.push_args(pairs);
        self
    }
    pub fn tuple_arg(&mut self, tuple_arg: (&str, &str)) -> &mut Self {
        self.push_args([tuple_arg.0, tuple_arg.1]);
        self
    }
    pub fn build(&mut self) -> Result<(), HermesError> {
        if self.cmd.is_empty() {
            return Err(HermesError::Internal("Command cannot be empty".to_string()));
        }
        self.cmds.push(RedisCmd {
            cmd: mem::take(&mut self.cmd),
            args: self.args.take(),
        });
        Ok(())
    }
    pub fn to_pipeline(&self) -> Pipeline {
        let mut pipeline = Pipeline::new();
        for RedisCmd { cmd, args } in &self.cmds {
            let mut redis_cmd = Cmd::new();
            redis_cmd.arg(&cmd);
            if let Some(args) = args {
                for arg in args {
                    redis_cmd.arg(&arg);
                }
            }
            pipeline.add_command(redis_cmd);
        }
        pipeline
    }
    pub fn cmd_count(&self) -> usize {
        self.cmds.len()
    }
    pub fn to_cmd(self) -> Cmd {
        let mut redis_cmd = Cmd::new();
        if self.cmds.len() == 1 {
            let RedisCmd { cmd, args } = &self.cmds[0];
            redis_cmd.arg(cmd);
            if let Some(args_) = args {
                for arg in args_ {
                    redis_cmd.arg(arg);
                }
            }
        }
        redis_cmd
    }

}
