//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/probe-finder.h
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

pub const MAX_PROBE_BUFFER: c_int = 1024;
pub const MAX_PROBES: c_int = 128;
pub const MAX_PROBE_ARGS: c_int = 128;

// TODO

// Check the language code is known C
extern "C" {
    pub fn is_known_C_lang(lang: c_int) -> bool;
}
// Find probe_trace_events specified by perf_probe_event from debuginfo
// Find a perf_probe_point from debuginfo
// Find a line range
extern "C" {
    pub fn debuginfo__find_line_range(dbg: *mut debuginfo, lr: *mut line_range) -> c_int;
}
// Find available variables
// Find a src file from a DWARF tag path
#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_finder {
    pub /: *mut *mut *mut perf_probe_event pev; / Target probe event,
    pub dbg: *mut debuginfo,
// Callback when a probe point is found
    pub pf): *mut *mut *mut int (callback)(Dwarf_Die sc_die, struct probe_finder,
// For function searching
    pub /: *mut *mut int lno; / Line number,
    pub /: *mut *mut Dwarf_Addr addr; / Address,
    pub /: *const *const *const char fname; / Real file name,
    pub /: *mut *mut Dwarf_Die cu_die; / Current CU,
    pub sp_die: Dwarf_Die,
    pub abstrace_dieoffset: Dwarf_Off,
    pub /: *mut *mut *mut intlist lcache; / Line cache for lazy match,
// For variable searching
// Call Frame Information from .eh_frame. Owned by this struct.
    pub cfi_eh: *mut Dwarf_CFI,
// Call Frame Information from .debug_frame. Not owned.
    pub cfi_dbg: *mut Dwarf_CFI,
    pub /: *mut *mut *mut Dwarf_Op fb_ops; / Frame base attribute,
    pub /: *mut *mut unsigned int e_machine; / ELF target machine arch,
    pub /: *mut *mut unsigned int e_flags; / ELF target machine flags,
    pub /: *mut *mut *mut perf_probe_arg pvar; / Current target variable,
    pub /: *mut *mut *mut probe_trace_arg tvar; / Current result variable,
    pub /: *mut *mut bool skip_empty_arg; / Skip non-exist args,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_finder {
    pub pf: probe_finder,
    pub /: *mut *mut *mut Dwfl_Module mod; / For solving symbols,
    pub /: *mut *mut *mut probe_trace_event tevs; / Found trace events,
    pub /: *mut *mut int ntevs; / Number of trace events,
    pub /: *mut *mut int max_tevs; / Max number of trace events,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct available_var_finder {
    pub pf: probe_finder,
    pub /: *mut *mut *mut Dwfl_Module mod; / For solving symbols,
    pub /: *mut *mut *mut variable_list vls; / Found variable lists,
    pub /: *mut *mut int nvls; / Number of variable lists,
    pub /: *mut *mut int max_vls; / Max no. of variable lists,
    pub /: *mut *mut bool child; / Search child scopes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct line_finder {
    pub /: *mut *mut *mut line_range lr; / Target line range,
    pub /: *const *const *const char fname; / File name,
    pub /: *mut *mut int lno_s; / Start line number,
    pub /: *mut *mut int lno_e; / End line number,
    pub /: *mut *mut Dwarf_Die cu_die; / Current CU,
    pub sp_die: Dwarf_Die,
    pub found: c_int,
}

