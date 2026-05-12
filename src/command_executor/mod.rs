mod command_router;
pub mod adapter;
pub mod command_handler;

pub type CommandExecutor = command_router::CommandRouter;