//! Faerie trap manifests record every `TrapCode` that cretonne outputs during code generation,
//! for every function in the module. This data may be useful at runtime.

use cretonne_codegen::{ir, binemit};

/// Record of the arguments cretonne passes to `TrapSink::trap`
pub struct FaerieTrapSite {
    /// Offset into function
    pub offset: binemit::CodeOffset,
    /// Source location given to cretonne
    pub srcloc: ir::SourceLoc,
    /// Trap code, as determined by cretonne
    pub code: ir::TrapCode,
}

/// Record of the trap sites for a given function
pub struct FaerieTrapSink {
    /// Name of function
    pub name: String,
    /// Total code size of function
    pub code_size: u32,
    /// All trap sites collected in function
    pub sites: Vec<FaerieTrapSite>,
}


impl FaerieTrapSink {
    /// Create an empty `FaerieTrapSink`
    pub fn new(name: &str, code_size: u32) -> Self {
        Self {
            sites: Vec::new(),
            name: name.to_owned(),
            code_size,
        }
    }
}

impl binemit::TrapSink for FaerieTrapSink {
    fn trap(&mut self, offset: binemit::CodeOffset, srcloc: ir::SourceLoc, code: ir::TrapCode) {
        self.sites.push(FaerieTrapSite {
            offset,
            srcloc,
            code,
        });
    }
}

/// Collection of all `FaerieTrapSink`s for the module
pub struct FaerieTrapManifest {
    /// All `FaerieTrapSink` for the module
    pub sinks: Vec<FaerieTrapSink>,
}

impl FaerieTrapManifest {
    /// Create an empty `FaerieTrapManifest`
    pub fn new() -> Self {
        Self { sinks: Vec::new() }
    }

    /// Put a `FaerieTrapSink` into manifest
    pub fn add_sink(&mut self, sink: FaerieTrapSink) {
        self.sinks.push(sink);
    }
}
