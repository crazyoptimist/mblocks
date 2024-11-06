use crate::block::Block;
#[allow(unused_imports)]
use crate::block::BlockType::{Once, Periodic, PeriodicOrSignal, Signal};
#[allow(unused_imports)]
use crate::block::CommandType::{Function, Shell};

use crate::blocks::cpu::cpu_usage;
use crate::blocks::memory::memory_usage;

pub const SEPARATOR: &str = " | ";
pub const PREFIX: &str = " ";
pub const SUFFIX: &str = " ";

pub const BLOCKS: &[Block] = &[
    Block {
        kind: PeriodicOrSignal(5, 1),
        command: Shell(&["date", "+%I:%M %p %Z, %Y-%m-%d (%a)"]),
        prefix: PREFIX,
        suffix: "",
    },
    Block {
        kind: Periodic(1),
        command: Function(memory_usage),
        prefix: "MEM: ",
        suffix: "",
    },
    Block {
        kind: Periodic(1),
        command: Function(cpu_usage),
        prefix: "CPU: ",
        suffix: "%",
    },
];
