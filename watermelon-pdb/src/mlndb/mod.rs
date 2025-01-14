// Copyright (C) Back Engineering Labs, Inc. - All Rights Reserved

// This is for the MLNDB format which is contains all info we need from the PDB
// to do binary rewriting. There's a lot of info in the PDB we don't care about
// and can be left behind.

pub const MLNDB_HEADER_MAGIC: u32 = 0x4D4C4E44; // MLND
pub const MLNDB_CURRENT_VERSION: u32 = 1;

// Header structure.
// magic: u32,
// version: u32,
// /// Address of the function table.
// /// Since version 1.
// function_table: u64,
// /// Number of function entries in the table.
// /// Since Version 1.
// function_table_count: u64,

#[derive(Debug, Clone)]
pub struct MlnDb {
    pub functions: Vec<MlnDbFunction>,
}
impl MlnDb {
    pub fn from_bytes(bytes: &[u8]) -> Option<MlnDb> {
        None
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct MlnDbFunction {
    pub address: u64,
    pub noreturn: bool,
    pub name: Option<String>,
}
