use crate::init_enum;

init_enum! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum RedisCommands {
        SET,
        DEL,
        DUMP,
        EXISTS,
        EXPIRE,
        EXPIREAT,
        PEXPIRE,
        PEXPIREAT,
        KEYS,
        MOVE,
        PERSIST,
        PTTL,
        TTL,
        RANDOMKEY,
        RENAME,
        RENAMENX,
        TYPE,
    }
}