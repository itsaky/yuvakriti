pub use cp::ConstantEntry;
pub use cp::ConstantPool;
pub use cp::CpSize;
pub use disassembler::YKBDisassembler;
pub use file::YKBFile;
pub use reader::YKBFileReader;
pub use version::YKBVersion;
pub use writer::YKBFileWriter;

pub const MAGIC_NUMBER: u32 = 0x59754B72;
pub const EXT_YK: &str = "yk";
pub const EXT_YKB: &str = "ykb";

#[cfg(doctest)]
mod tests;

#[cfg(test)]
mod tests;

pub mod attrs;
pub mod bytes;
mod cp;
pub mod cp_info;
mod decls;
mod disassembler;
mod file;
pub mod opcode;
mod reader;
mod version;
mod writer;
