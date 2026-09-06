//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zlib_dfltcc/dfltcc.h
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


// SPDX-License-Identifier: Zlib

//
// Tuning parameters.
//
pub const DFLTCC_LEVEL_MASK: c_uint = 0x2 /* DFLTCC compression for level 1 only */;
pub const DFLTCC_LEVEL_MASK_DEBUG: c_uint = 0x3fe /* DFLTCC compression for all levels */;
pub const DFLTCC_BLOCK_SIZE: c_int = 1048576;
pub const DFLTCC_FIRST_FHT_BLOCK_SIZE: c_int = 4096;
pub const DFLTCC_DHT_MIN_SAMPLE_SIZE: c_int = 4096;
pub const DFLTCC_RIBM: c_int = 0;
pub const DFLTCC_FACILITY: c_int = 151;
//
// Parameter Block for Query Available Functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfltcc_qaf_param {
    pub fns: [c_char; 16],
    pub reserved1: [c_char; 8],
    pub fmts: [c_char; 2],
    pub reserved2: [c_char; 6],
}

pub const DFLTCC_FMT0: c_int = 0;
//
// Parameter Block for Generate Dynamic-Huffman Table, Compress and Expand.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfltcc_param_v0 {
    pub /: *mut *mut uint16_t pbvn; / Parameter-Block-Version Number,
    pub /: *mut *mut uint8_t mvn; / Model-Version Number,
    pub /: *mut *mut uint8_t ribm; / Reserved for IBM use,
    pub 31: unsigned reserved32 :,
    pub /: *mut *mut unsigned cf : 1; / Continuation Flag,
    pub reserved64: [u8; 8],
    pub /: *mut *mut unsigned nt : 1; / New Task,
    pub 1: unsigned reserved129 :,
    pub /: *mut *mut unsigned cvt : 1; / Check Value Type,
    pub 1: unsigned reserved131 :,
    pub /: *mut *mut unsigned htt : 1; / Huffman-Table Type,
    pub /: *mut *mut unsigned bcf : 1; / Block-Continuation Flag,
    pub /: *mut *mut unsigned bcc : 1; / Block Closing Control,
    pub /: *mut *mut unsigned bhf : 1; / Block Header Final,
    pub 1: unsigned reserved136 :,
    pub 1: unsigned reserved137 :,
    pub /: *mut *mut unsigned dhtgc : 1; / DHT Generation Control,
    pub 5: unsigned reserved139 :,
    pub 5: unsigned reserved144 :,
    pub /: *mut *mut unsigned sbb : 3; / Sub-Byte Boundary,
    pub /: *mut *mut uint8_t oesc; / Operation-Ending-Supplemental Code,
    pub 12: unsigned reserved160 :,
    pub /: *mut *mut unsigned ifs : 4; / Incomplete-Function Status,
    pub /: *mut *mut uint16_t ifl; / Incomplete-Function Length,
    pub reserved192: [u8; 8],
    pub reserved256: [u8; 8],
    pub reserved320: [u8; 4],
    pub /: *mut *mut uint16_t hl; / History Length,
    pub 1: unsigned reserved368 :,
    pub /: *mut *mut uint16_t ho : 15; / History Offset,
    pub /: *mut *mut uint32_t cv; / Check Value,
    pub /: *mut *mut unsigned eobs : 15; / End-of-block Symbol,
    pub 1: unsigned reserved431:,
    pub /: *mut *mut uint8_t eobl : 4; / End-of-block Length,
    pub 12: unsigned reserved436 :,
    pub 4: unsigned reserved448 :,
    pub Table: *mut *mut uint16_t cdhtl : 12; / Compressed-Dynamic-Huffman,
    pub reserved464: [u8; 6],
    pub cdht: [u8; 288],
    pub reserved: [u8; 32],
    pub csb: [u8; 1152],
}

pub const CVT_CRC32: c_int = 0;
pub const CVT_ADLER32: c_int = 1;
pub const HTT_FIXED: c_int = 0;
pub const HTT_DYNAMIC: c_int = 1;
//
// Extension of inflate_state and deflate_state for DFLTCC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfltcc_state {
    pub /: *mut *mut dfltcc_param_v0 param; / Parameter block,
    pub /: *mut *mut dfltcc_qaf_param af; / Available functions,
    pub /: *mut *mut char msg[64]; / Buffer for strm->msg,
}

//
// Extension of inflate_state and deflate_state for DFLTCC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfltcc_deflate_state {
    pub /: *mut *mut dfltcc_state common; / Parameter block,
    pub /: *mut *mut uLong level_mask; / Levels on which to use DFLTCC,
    pub /: *mut *mut uLong block_size; / New block each X bytes,
    pub /: *mut *mut uLong block_threshold; / New block after total_in > X,
    pub /: *mut *mut uLong dht_threshold; / New block only if avail_in >= X,
}

// Resides right after inflate_state or deflate_state

extern "C" {
    pub fn dfltcc_reset_state(dfltcc_state: *mut dfltcc_state);
}

