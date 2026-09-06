//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/annotate-data.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum type_state_kind {
    TSR_KIND_INVALID = 0,
    TSR_KIND_TYPE,
    TSR_KIND_PERCPU_BASE,
    TSR_KIND_CONST,
    TSR_KIND_PERCPU_POINTER,
    TSR_KIND_POINTER,
    TSR_KIND_CANARY,
}

//
// struct annotated_member - Type of member field
// @node: List entry in the parent list
// @children: List head for child nodes
// @type_name: Name of the member type
// @var_name: Name of the member variable
// @offset: Offset from the outer data type
// @size: Size of the member field
//
// This represents a member type in a data type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_member {
    pub node: list_head,
    pub children: list_head,
    pub type_name: *mut c_char,
    pub var_name: *mut c_char,
    pub offset: c_int,
    pub size: c_int,
}

//
// struct type_hist_entry - Histogram entry per offset
// @nr_samples: Number of samples
// @period: Count of event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_hist_entry {
    pub nr_samples: c_int,
    pub period: u64,
}

//
// struct type_hist - Type histogram for each event
// @nr_samples: Total number of samples in this data type
// @period: Total count of the event in this data type
// @offset: Array of histogram entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_hist {
    pub nr_samples: u64,
    pub period: u64,
    pub addr: [type_hist_entry; ],
}

//
// struct annotated_data_type - Data type to profile
// @node: RB-tree node for dso->type_tree
// @self: Actual type information
// @nr_histogram: Number of histogram entries
// @histograms: An array of pointers to histograms
//
// This represents a data type accessed by samples in the profile data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_data_type {
    pub node: rb_node,
    pub self: annotated_member,
    pub nr_histograms: c_int,
    pub histograms: *mut type_hist,
}

//
// struct data_loc_info - Data location information
// @arch: CPU architecture info
// @thread: Thread info
// @ms: Map and Symbol info
// @ip: Instruction address
// @var_addr: Data address (for global variables)
// @cpumode: CPU execution mode
// @op: Instruction operand location (regs and offset)
// @di: Debug info
// @fbreg: Frame base register
// @fb_cfa: Whether the frame needs to check CFA
// @type_offset: Final offset in the type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_loc_info {
// These are input field, should be filled by caller
    pub arch: *const arch,
    pub thread: *mut thread,
    pub ms: *mut map_symbol,
    pub ip: u64,
    pub var_addr: u64,
    pub cpumode: u8,
    pub op: *mut annotated_op_loc,
    pub di: *mut debuginfo,
// These are used internally
    pub fbreg: c_int,
    pub fb_cfa: bool,
// This is for the result
    pub type_offset: c_int,
}

//
// struct annotated_data_stat - Debug statistics
// @total: Total number of entry
// @no_sym: No symbol or map found
// @no_insn: Failed to get disasm line
// @no_insn_ops: The instruction has no operands
// @no_mem_ops: The instruction has no memory operands
// @no_reg: Failed to extract a register from the operand
// @no_dbginfo: The binary has no debug information
// @no_cuinfo: Failed to find a compile_unit
// @no_var: Failed to find a matching variable
// @no_typeinfo: Failed to get a type info for the variable
// @invalid_size: Failed to get a size info of the type
// @bad_offset: The access offset is out of the type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_data_stat {
    pub total: c_int,
    pub no_sym: c_int,
    pub no_insn: c_int,
    pub no_insn_ops: c_int,
    pub no_mem_ops: c_int,
    pub no_reg: c_int,
    pub no_dbginfo: c_int,
    pub no_cuinfo: c_int,
    pub no_var: c_int,
    pub no_typeinfo: c_int,
    pub invalid_size: c_int,
    pub bad_offset: c_int,
    pub insn_track: c_int,
}

//
// Type information in a register, valid when @ok is true.
// The @caller_saved registers are invalidated after a function call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_state_reg {
    pub type: Dwarf_Die,
    pub imm_value: u32,
//
// The offset within the struct that the register points to.
// A value of 0 means the register points to the beginning.
// type_offset = op->offset + reg->offset
//
    pub offset: i32,
    pub ok: bool,
    pub caller_saved: bool,
// DWARF location range tracking for register lifetime
    pub lifetime_active: bool,
    pub lifetime_end: u64,
    pub kind: u8,
    pub copied_from: u8,
}

// Type information in a stack location, dynamically allocated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_state_stack {
    pub list: list_head,
    pub type: Dwarf_Die,
    pub offset: c_int,
// pointer offset, saves tsr->offset on the stack state
    pub ptr_offset: c_int,
    pub size: c_int,
    pub compound: bool,
    pub kind: u8,
}

//
// Maximum number of registers tracked in type_state.
//
// This limit must cover all supported architectures, since perf
// may analyze perf.data files generated on systems with a different
// register set. Use 32 as a safe upper bound instead of relying on
// build-arch specific values.
//
pub const TYPE_STATE_MAX_REGS: c_int = 32;
//
// State table to maintain type info in each register and stack location.
// It'll be updated when new variable is allocated or type info is moved
// to a new location (register or stack).  As it'd be used with the
// shortest path of basic blocks, it only maintains a single table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_state {
// state of general purpose registers
    pub regs: [type_state_reg; TYPE_STATE_MAX_REGS],
// state of stack location
    pub stack_vars: list_head,
// return value register
    pub ret_reg: c_int,
// stack pointer register
    pub stack_reg: c_int,
}

// Returns data type at the location (ip, reg, offset)
// Update type access histogram at the given offset
// Release all data type information in the tree
extern "C" {
    pub fn annotated_data_type__tree_delete(root: *mut rb_root);
}
// Release all global variable information in the tree
extern "C" {
    pub fn global_var_type__tree_delete(root: *mut rb_root);
}
// Print data type annotation (including members) on stdout
extern "C" {
    pub fn hist_entry__annotate_data_tty(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
}
// Get name of member field at the given offset in the data type
extern "C" {
    pub fn has_reg_type(state: *mut type_state, reg: c_int) -> bool;
}
extern "C" {
    pub fn pr_debug_type_name(die: *mut Dwarf_Die, kind: type_state_kind);
}

