use super::compiletime::*;
use super::runtime::*;

pub enum General {
    CompileTime(CompileTime),
    RunTime(RunTime),
}
