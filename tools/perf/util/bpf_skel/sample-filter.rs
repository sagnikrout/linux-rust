//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/bpf_skel/sample-filter.h
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


pub const MAX_FILTERS: c_int = 64;

// supported filter operations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_bpf_filter_op {
    PBF_OP_EQ,
    PBF_OP_NEQ,
    PBF_OP_GT,
    PBF_OP_GE,
    PBF_OP_LT,
    PBF_OP_LE,
    PBF_OP_AND,
    PBF_OP_GROUP_BEGIN,
    PBF_OP_GROUP_END,
    PBF_OP_DONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_bpf_filter_term {
// No term is in use.
    PBF_TERM_NONE = 0,
// Terms that correspond to PERF_SAMPLE_xx values.
    PBF_TERM_SAMPLE_START	= PBF_TERM_NONE + 1,
    PBF_TERM_IP		= PBF_TERM_SAMPLE_START + 0, /* SAMPLE_IP = 1U << 0 */
    PBF_TERM_TID		= PBF_TERM_SAMPLE_START + 1, /* SAMPLE_TID = 1U << 1 */
    PBF_TERM_TIME		= PBF_TERM_SAMPLE_START + 2, /* SAMPLE_TIME = 1U << 2 */
    PBF_TERM_ADDR		= PBF_TERM_SAMPLE_START + 3, /* SAMPLE_ADDR = 1U << 3 */
    __PBF_UNUSED_TERM4	= PBF_TERM_SAMPLE_START + 4, /* SAMPLE_READ = 1U << 4 */
    __PBF_UNUSED_TERM5	= PBF_TERM_SAMPLE_START + 5, /* SAMPLE_CALLCHAIN = 1U << 5 */
    PBF_TERM_ID		= PBF_TERM_SAMPLE_START + 6, /* SAMPLE_ID = 1U << 6 */
    PBF_TERM_CPU		= PBF_TERM_SAMPLE_START + 7, /* SAMPLE_CPU = 1U << 7 */
    PBF_TERM_PERIOD		= PBF_TERM_SAMPLE_START + 8, /* SAMPLE_PERIOD = 1U << 8 */
    __PBF_UNUSED_TERM9	= PBF_TERM_SAMPLE_START + 9, /* SAMPLE_STREAM_ID = 1U << 9 */
    __PBF_UNUSED_TERM10	= PBF_TERM_SAMPLE_START + 10, /* SAMPLE_RAW = 1U << 10 */
    __PBF_UNUSED_TERM11	= PBF_TERM_SAMPLE_START + 11, /* SAMPLE_BRANCH_STACK = 1U << 11 */
    __PBF_UNUSED_TERM12	= PBF_TERM_SAMPLE_START + 12, /* SAMPLE_REGS_USER = 1U << 12 */
    __PBF_UNUSED_TERM13	= PBF_TERM_SAMPLE_START + 13, /* SAMPLE_STACK_USER = 1U << 13 */
    PBF_TERM_WEIGHT		= PBF_TERM_SAMPLE_START + 14, /* SAMPLE_WEIGHT = 1U << 14 */
    PBF_TERM_DATA_SRC	= PBF_TERM_SAMPLE_START + 15, /* SAMPLE_DATA_SRC = 1U << 15 */
    __PBF_UNUSED_TERM16	= PBF_TERM_SAMPLE_START + 16, /* SAMPLE_IDENTIFIER = 1U << 16 */
    PBF_TERM_TRANSACTION	= PBF_TERM_SAMPLE_START + 17, /* SAMPLE_TRANSACTION = 1U << 17 */
    __PBF_UNUSED_TERM18	= PBF_TERM_SAMPLE_START + 18, /* SAMPLE_REGS_INTR = 1U << 18 */
    PBF_TERM_PHYS_ADDR	= PBF_TERM_SAMPLE_START + 19, /* SAMPLE_PHYS_ADDR = 1U << 19 */
    __PBF_UNUSED_TERM20	= PBF_TERM_SAMPLE_START + 20, /* SAMPLE_AUX = 1U << 20 */
    PBF_TERM_CGROUP		= PBF_TERM_SAMPLE_START + 21, /* SAMPLE_CGROUP = 1U << 21 */
    PBF_TERM_DATA_PAGE_SIZE	= PBF_TERM_SAMPLE_START + 22, /* SAMPLE_DATA_PAGE_SIZE = 1U << 22 */
    PBF_TERM_CODE_PAGE_SIZE	= PBF_TERM_SAMPLE_START + 23, /* SAMPLE_CODE_PAGE_SIZE = 1U << 23 */
    PBF_TERM_WEIGHT_STRUCT	= PBF_TERM_SAMPLE_START + 24, /* SAMPLE_WEIGHT_STRUCT = 1U << 24 */
    PBF_TERM_SAMPLE_END	= PBF_TERM_WEIGHT_STRUCT,
// Terms computed from BPF helpers.
    PBF_TERM_UID,
    PBF_TERM_GID,
}

// BPF map entry for filtering
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_bpf_filter_entry {
    pub op: perf_bpf_filter_op,
    pub /: *mut *mut __u32 part; / sub-sample type info when it has multiple values,
    pub term: perf_bpf_filter_term,
    pub value: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idx_hash_key {
    pub evt_id: __u64,
    pub tgid: __u32,
    pub reserved: __u32,
}
