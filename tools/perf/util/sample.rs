//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/sample.h
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

// number of register is bound by the number of bits in regs_dump::mask (64)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regs_dump {
    pub abi: u64,
    pub mask: u64,
    pub regs: *mut u64,
// Cached values/mask filled by first register access.
    pub cache_regs: [u64; PERF_SAMPLE_REGS_CACHE_SIZE],
    pub cache_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_dump {
    pub offset: u16,
    pub size: u64,
    pub data: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample_read_value {
    pub value: u64,
    pub /: *mut *mut u64 id; / only if PERF_FORMAT_ID,
    pub /: *mut *mut u64 lost; / only if PERF_FORMAT_LOST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample_read {
    pub time_enabled: u64,
    pub time_running: u64,
    pub nr: u64,
    pub values: *mut sample_read_value,
    pub group: },
    pub one: sample_read_value,
}

// PERF_FORMAT_ID is forced for PERF_SAMPLE_READ
extern "C" {
    pub fn sizeof(sample_read_value: struct) -> return;
}
extern "C" {
    pub fn offsetof(sample_read_value: struct, _arg: lost) -> return;
}

pub const MAX_INSN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_sample {
    pub size: u64,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_flags {
    pub /: *mut *mut resv: 3; / reserved,
}

// simd architecture flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum simd_op_flags {
    SIMD_OP_FLAGS_ARCH_NONE = 0x0,	/* No SIMD operation */
    SIMD_OP_FLAGS_ARCH_SVE,		/* Arm SVE */
    SIMD_OP_FLAGS_ARCH_SME,		/* Arm SME */
    SIMD_OP_FLAGS_ARCH_ASE,		/* Arm Advanced SIMD */
}

// simd predicate flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum simd_pred_flags {
    SIMD_OP_FLAGS_PRED_NONE = 0x0,	/* Not available */
    SIMD_OP_FLAGS_PRED_PARTIAL,	/* partial predicate */
    SIMD_OP_FLAGS_PRED_EMPTY,	/* empty predicate */
    SIMD_OP_FLAGS_PRED_FULL,	/* full predicate */
    SIMD_OP_FLAGS_PRED_DISABLED,	/* disabled predicate */
}

//
// struct perf_sample
//
// A sample is generally filled in by evlist__parse_sample/evsel__parse_sample
// which fills in the variables from a "union perf_event *event" which is data
// from a perf ring buffer or perf.data file. The "event" sample is variable in
// length as determined by the perf_event_attr (in the evsel) and details within
// the sample event itself. A struct perf_sample avoids needing to care about
// the variable length nature of the original event.
//
// To avoid being excessively large parts of the struct perf_sample are pointers
// into the original sample event. In general the lifetime of a struct
// perf_sample needs to be less than the "union perf_event *event" it was
// derived from.
//
// The struct regs_dump user_regs and intr_regs are lazily allocated again for
// size reasons, due to them holding a cache of looked up registers. The
// function pair of perf_sample__init and perf_sample__exit correctly initialize
// and clean up these values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_sample {
// @evsel: Backward reference to the evsel used when constructing the sample.
    pub evsel: *mut evsel,
// @ip: The sample event PERF_SAMPLE_IP value.
    pub ip: u64,
// @pid: The sample event PERF_SAMPLE_TID pid value.
    pub pid: u32,
// @tid: The sample event PERF_SAMPLE_TID tid value.
    pub tid: u32,
// @time: The sample event PERF_SAMPLE_TIME value.
    pub time: u64,
// @addr: The sample event PERF_SAMPLE_ADDR value.
    pub addr: u64,
// @id: The sample event PERF_SAMPLE_ID or PERF_SAMPLE_IDENTIFIER value.
    pub id: u64,
// @stream_id: The sample event PERF_SAMPLE_STREAM_ID value.
    pub stream_id: u64,
// @period: The sample event PERF_SAMPLE_PERIOD value.
    pub period: u64,
// @weight: Data determined by PERF_SAMPLE_WEIGHT or PERF_SAMPLE_WEIGHT_STRUCT.
    pub weight: u64,
// @transaction: The sample event PERF_SAMPLE_TRANSACTION value.
    pub transaction: u64,
// @insn_cnt: Filled in and used by intel-pt.
    pub insn_cnt: u64,
// @cyc_cnt: Filled in and used by intel-pt.
    pub cyc_cnt: u64,
// @cpu: The sample event PERF_SAMPLE_CPU value.
    pub cpu: u32,
//
// @raw_size: The size in bytes of raw data from PERF_SAMPLE_RAW. For
// alignment reasons this should always be sizeof(u32)
// followed by a multiple of sizeof(u64).
//
    pub raw_size: u32,
// @data_src: The sample event PERF_SAMPLE_DATA_SRC value.
    pub data_src: u64,
// @phys_addr: The sample event PERF_SAMPLE_PHYS_ADDR value.
    pub phys_addr: u64,
// @data_page_size: The sample event PERF_SAMPLE_DATA_PAGE_SIZE value.
    pub data_page_size: u64,
// @code_page_size: The sample event PERF_SAMPLE_CODE_PAGE_SIZE value.
    pub code_page_size: u64,
// @cgroup: The sample event PERF_SAMPLE_CGROUP value.
    pub cgroup: u64,
// @file_offset: Byte offset of this event in the perf.data file.
    pub file_offset: u64,
// @flags: Extra flag data from auxiliary events like intel-pt.
    pub flags: u32,
// @machine_pid: The guest machine pid derived from the sample id.
    pub machine_pid: u32,
// @vcpu: The guest machine vcpu derived from the sample id.
    pub vcpu: u32,
//
// @insn_len: Instruction length from auxiliary events like
// intel-pt. The instruction itself is held in insn.
//
    pub insn_len: u16,
// @misc: The entire struct perf_event_header misc variable.
    pub misc: u16,
//
// @ins_lat: Instruction latency information from weight2 in
// PERF_SAMPLE_WEIGHT_STRUCT or auxiliary events like
// intel-pt.
//
    pub ins_lat: u16,
//
// @weight3: From PERF_SAMPLE_WEIGHT_STRUCT. On x86 holds retire_lat, on
// powerpc holds p_stage_cyc.
//
    pub weight3: u16,
//
// @cpumode: The cpumode from struct perf_event_header misc variable
// masked with CPUMODE_MASK. Gives user, kernel and hypervisor
// information.
//
    pub cpumode: u8,
//
// @no_hw_idx: For PERF_SAMPLE_BRANCH_STACK, true when
// PERF_SAMPLE_BRANCH_HW_INDEX isn't set.
//
    pub no_hw_idx: bool,
//
// @deferred_callchain: When processing PERF_SAMPLE_CALLCHAIN a deferred
// user callchain marker was encountered.
//
    pub deferred_callchain: bool,
//
// @merged_callchain: A synthesized merged callchain that is allocated
// and needs freeing.
//
    pub merged_callchain: bool,
//
// @deferred_cookie: Identifier of the deferred callchain in the later
// PERF_RECORD_CALLCHAIN_DEFERRED event.
//
    pub deferred_cookie: u64,
// @insn: A copy of the sampled instruction filled in by perf_sample__fetch_insn.
    pub insn: [c_char; MAX_INSN],
// @raw_data: Pointer into the original event for PERF_SAMPLE_RAW data.
    pub raw_data: *mut c_void,
//
// @callchain: Pointer into the original event for PERF_SAMPLE_CALLCHAIN
// data. For deferred callchains this may be a copy that
// needs freeing, see sample__merge_deferred_callchain.
//
    pub callchain: *mut ip_callchain,
// @branch_stack: Pointer into the original event for PERF_SAMPLE_BRANCH_STACK data.
    pub branch_stack: *mut branch_stack,
//
// @branch_stack_cntr: Pointer into the original event for
// PERF_SAMPLE_BRANCH_COUNTERS data.
//
    pub branch_stack_cntr: *mut u64,
// @user_regs: Values and pointers into the sample for PERF_SAMPLE_REGS_USER.
    pub user_regs: *mut regs_dump,
// @intr_regs: Values and pointers into the sample for PERF_SAMPLE_REGS_INTR.
    pub intr_regs: *mut regs_dump,
// @user_stack: Size and pointer into the sample for PERF_SAMPLE_STACK_USER.
    pub user_stack: stack_dump,
//
// @read: The sample event PERF_SAMPLE_READ counter values. The valid
// values depend on the attr.read_format PERF_FORMAT_ values.
//
    pub read: sample_read,
//
// @aux_sample: Similar to raw data but with a 64-bit size and
// alignment, PERF_SAMPLE_AUX data.
//
    pub aux_sample: aux_sample,
// @simd_flags: SIMD flag information from ARM SPE auxiliary events.
    pub simd_flags: simd_flags,
}

extern "C" {
    pub fn perf_sample__init(sample: *mut perf_sample, all: bool);
}
extern "C" {
    pub fn perf_sample__exit(sample: *mut perf_sample);
}
//
// raw_data is always 4 bytes from an 8-byte boundary, so subtract 4 to get
// 8-byte alignment.
//
