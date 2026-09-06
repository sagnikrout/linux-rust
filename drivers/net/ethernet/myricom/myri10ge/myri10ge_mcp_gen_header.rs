//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/myricom/myri10ge/myri10ge_mcp_gen_header.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
pub const MCP_HEADER_PTR_OFFSET: c_uint = 0x3c;
pub const MCP_TYPE_MX: c_uint = 0x4d582020	/* "MX  " */;
pub const MCP_TYPE_PCIE: c_uint = 0x70636965	/* "PCIE" pcie-only MCP */;
pub const MCP_TYPE_ETH: c_uint = 0x45544820	/* "ETH " */;
pub const MCP_TYPE_MCP0: c_uint = 0x4d435030	/* "MCP0" */;
pub const MCP_TYPE_DFLT: c_uint = 0x20202020	/* "    " */;
pub const MCP_TYPE_ETHZ: c_uint = 0x4554485a	/* "ETHZ" */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_gen_header {
// the first 4 fields are filled at compile time
    pub header_length: unsigned,
    pub mcp_type: __be32,
    pub version: [c_char; 128],
    pub /: *mut *mut unsigned mcp_private; / pointer to mcp-type specific structure,
// filled by the MCP at run-time
    pub sram_size: unsigned,
    pub /: *mut *mut unsigned string_specs; / either the original STRING_SPECS or a superset,
    pub string_specs_len: unsigned,
// Fields above this comment are guaranteed to be present.
//
// Fields below this comment are extensions added in later versions
// of this struct, drivers should compare the header_length against
// offsetof(field) to check whether a given MCP implements them.
//
// Never remove any field.  Keep everything naturally align.
//
// Specifies if the running mcp is mcp0, 1, or 2.
    pub mcp_index: c_uchar,
    pub disable_rabbit: c_uchar,
    pub unaligned_tlp: c_uchar,
    pub pcie_link_algo: c_uchar,
    pub counters_addr: unsigned,
    pub /: *mut *mut unsigned copy_block_info; / for small mcps loaded with "lload -d",
    pub /: *mut *mut unsigned short handoff_id_major; / must be equal,
    pub /: *mut *mut unsigned short handoff_id_caps; / bitfield: new mcp must have superset,
    pub /: *mut *mut unsigned msix_table_addr; / start address of msix table in firmware,
    pub /: *mut *mut unsigned bss_addr; / start of bss,
    pub features: unsigned,
    pub ee_hdr_addr: unsigned,
    pub led_pattern: unsigned,
    pub led_pattern_dflt: unsigned,
// 8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zmcp_info {
    pub info_len: unsigned,
    pub zmcp_addr: unsigned,
    pub zmcp_len: unsigned,
    pub mcp_edata: unsigned,
}
