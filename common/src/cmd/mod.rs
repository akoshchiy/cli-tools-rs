use anyhow::Result;
use futures::Stream;

pub mod ssh;

type CmdStream = Box<dyn Stream<Item = Result<String>> + Unpin>;

pub trait CmdExecutor {
    fn execute(&self) -> CmdStream;
}


struct MergeExecutor {
    inner: Vec<Box<dyn CmdExecutor>>,
}

impl CmdExecutor for MergeExecutor {
    fn execute(&self) -> CmdStream {
        // futures::stream::Chain
        todo!()
    }
}
