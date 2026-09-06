//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/dwarf-aux.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// dwarf-aux.h : libdw auxiliary interfaces
//

// Find the realpath of the target file
// Get DW_AT_comp_dir (should be NULL with older gcc)
// Get a line number and file name for given address
// Walk on functions at given address
// Get DW_AT_linkage_name (should be NULL for C binary)
// Get the lowest PC in DIE (including range list)
extern "C" {
    pub fn die_entrypc(dw_die: *mut Dwarf_Die, addr: *mut Dwarf_Addr) -> c_int;
}
// Ensure that this DIE is a subprogram and definition (not declaration)
extern "C" {
    pub fn die_is_func_def(dw_die: *mut Dwarf_Die) -> bool;
}
// Ensure that this DIE is an instance of a subprogram
extern "C" {
    pub fn die_is_func_instance(dw_die: *mut Dwarf_Die) -> bool;
}
// Compare diename and tname
extern "C" {
    pub fn die_compare_name(dw_die: *mut Dwarf_Die, tname: *const c_char) -> bool;
}
// Matching diename with glob pattern
extern "C" {
    pub fn die_match_name(dw_die: *mut Dwarf_Die, glob: *const c_char) -> bool;
}
// Get callsite line number of inline-function instance
extern "C" {
    pub fn die_get_call_lineno(in_die: *mut Dwarf_Die) -> c_int;
}
// Get callsite file name of inlined function instance
// Get declared file name of a DIE
// Get type die
// Get a type die, but skip qualifiers
// Get a type die, but skip qualifiers and typedef
// Get a pointer/array type, following typedefs/qualifiers
// Check whether the DIE is signed or not
extern "C" {
    pub fn die_is_signed_type(tp_die: *mut Dwarf_Die) -> bool;
}
// Get data_member_location offset
extern "C" {
    pub fn die_get_data_member_location(mb_die: *mut Dwarf_Die, offs: *mut Dwarf_Word) -> c_int;
}
// Return values for die_find_child() callbacks
// Search child DIEs
// Search a non-inlined function including given address
// Search a non-inlined function with tail call at given address
// Search the top inlined function including given address
// Search the deepest inlined function including given address
// Search a non-inlined function by name and returns its return type
// Walk on the instances of given DIE
// Walker on lines (Note: line number will not be sorted)
//
// Walk on lines inside given DIE. If the DIE is a subprogram, walk only on
// the lines inside the subprogram, otherwise the DIE must be a CU DIE.
//
extern "C" {
    pub fn die_walk_lines(rt_die: *mut Dwarf_Die, callback: line_walk_callback_t, data: *mut c_void) -> c_int;
}
// Find a variable called 'name' at given address
// Find a member called 'name'
// Get the name of given type DIE
extern "C" {
    pub fn die_get_typename_from_type(type_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int;
}
// Get the name of given variable DIE
extern "C" {
    pub fn die_get_typename(vr_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int;
}
// Get the name and type of given variable DIE, stored as "type\tname"
extern "C" {
    pub fn die_get_varname(vr_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int;
}
// Check if target program is compiled with optimization
extern "C" {
    pub fn die_is_optimized_target(cu_die: *mut Dwarf_Die) -> bool;
}
// Use next address after prologue as probe location
// Get the list of including scopes
extern "C" {
    pub fn die_get_scopes(cu_die: *mut Dwarf_Die, pc: Dwarf_Addr, scopes: *mut Dwarf_Die) -> c_int;
}
// Variable type information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct die_var_type {
    pub next: *mut die_var_type,
    pub die_off: u64,
    pub addr: u64,
    pub /: *mut *mut u64 end; / end address of location range,
    pub reg: c_int,
    pub offset: c_int,
// Whether the register holds a address to the type
    pub is_reg_var_addr: bool,
    pub /: *mut *mut bool has_range; / whether end is valid,
}

// Return type info of a member at offset
// Return type info where the pointer and offset point to
// Get byte offset range of given variable DIE
extern "C" {
    pub fn die_get_var_range(sp_die: *mut Dwarf_Die, vr_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int;
}
// Find a variable saved in the 'reg' at given address
// Find a (global) variable located in the 'addr'
// Save all variables and parameters in this scope
extern "C" {
    pub fn die_collect_vars(sc_die: *mut Dwarf_Die, var_types: *mut die_var_type);
}
// Save all global variables in this CU
extern "C" {
    pub fn die_collect_global_vars(cu_die: *mut Dwarf_Die, var_types: *mut die_var_type);
}
// Get the frame base information from CFA
extern "C" {
    pub fn die_get_cfa(dwarf: *mut Dwarf, pc: u64, preg: *mut c_int, poffset: *mut c_int) -> c_int;
}
