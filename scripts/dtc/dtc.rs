//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/dtc/dtc.h
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
// (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005.
//

// Macro flag: #define debug(...)

pub const DEFAULT_FDT_VERSION: c_int = 17;
//
// Command line options
//
pub const PHANDLE_LEGACY: c_uint = 0x1;
pub const PHANDLE_EPAPR: c_uint = 0x2;
pub const PHANDLE_BOTH: c_uint = 0x3;
pub type cell_t = u32;

extern "C" {
    pub fn streq(suffix_len: str + len -, _arg: suffix) -> return;
}

// Data blobs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum markertype {
    TYPE_NONE,
    REF_PHANDLE,
    REF_PATH,
    LABEL,
    TYPE_UINT8,
    TYPE_UINT16,
    TYPE_UINT32,
    TYPE_UINT64,
    TYPE_STRING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct marker {
    pub type: markertype,
    pub offset: c_uint,
    pub ref: *mut c_char,
    pub next: *mut marker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data {
    pub len: c_uint,
    pub val: *mut c_char,
    pub markers: *mut marker,
}

extern "C" {
    pub fn data_free(d: data);
}
extern "C" {
    pub fn data_grow_for(d: data, xlen: c_uint) -> data;
}
extern "C" {
    pub fn data_copy_mem(mem: *const c_char, len: c_int) -> data;
}
extern "C" {
    pub fn data_copy_escape_string(s: *const c_char, len: c_int) -> data;
}
extern "C" {
    pub fn data_copy_file(f: *mut FILE, len: usize) -> data;
}
extern "C" {
    pub fn data_append_data(d: data, p: *const c_void, len: c_int) -> data;
}
extern "C" {
    pub fn data_merge(d1: data, d2: data) -> data;
}
extern "C" {
    pub fn data_append_cell(d: data, word: cell_t) -> data;
}
extern "C" {
    pub fn data_append_integer(d: data, word: u64, bits: c_int) -> data;
}
extern "C" {
    pub fn data_append_re(d: data, address: u64, size: u64) -> data;
}
extern "C" {
    pub fn data_append_addr(d: data, addr: u64) -> data;
}
extern "C" {
    pub fn data_append_byte(d: data, byte: u8) -> data;
}
extern "C" {
    pub fn data_append_zeroes(d: data, len: c_int) -> data;
}
extern "C" {
    pub fn data_append_align(d: data, align: c_int) -> data;
}
extern "C" {
    pub fn data_insert_data(d: data, m: *mut marker, old: data) -> data;
}
extern "C" {
    pub fn data_add_marker(d: data, type: markertype, ref: *mut c_char) -> data;
}
extern "C" {
    pub fn data_is_one_string(d: data) -> bool;
}
// DT constraints
pub const MAX_PROPNAME_LEN: c_int = 31;
pub const MAX_NODENAME_LEN: c_int = 31;
// Live trees
#[repr(C)]
#[derive(Copy, Clone)]
pub struct label {
    pub deleted: bool,
    pub label: *mut c_char,
    pub next: *mut label,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_type {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct property {
    pub deleted: bool,
    pub name: *mut c_char,
    pub val: data,
    pub next: *mut property,
    pub labels: *mut label,
    pub srcpos: *mut srcpos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node {
    pub deleted: bool,
    pub name: *mut c_char,
    pub proplist: *mut property,
    pub children: *mut node,
    pub parent: *mut node,
    pub next_sibling: *mut node,
    pub fullpath: *mut c_char,
    pub basenamelen: usize,
    pub phandle: cell_t,
    pub size_cells: int addr_cells,,
    pub labels: *mut label,
    pub bus: *const bus_type,
    pub srcpos: *mut srcpos,
    pub is_referenced: bool omit_if_unused,,
}

extern "C" {
    pub fn add_label(labels: *mut label, label: *mut c_char);
}
extern "C" {
    pub fn delete_labels(labels: *mut label);
}
extern "C" {
    pub fn add_property(node: *mut node, prop: *mut property);
}
extern "C" {
    pub fn delete_property_by_name(node: *mut node, name: *mut c_char);
}
extern "C" {
    pub fn delete_property(prop: *mut property);
}
extern "C" {
    pub fn add_child(parent: *mut node, child: *mut node);
}
extern "C" {
    pub fn delete_node_by_name(parent: *mut node, name: *mut c_char);
}
extern "C" {
    pub fn delete_node(node: *mut node);
}
extern "C" {
    pub fn propval_cell(prop: *mut property) -> cell_t;
}
extern "C" {
    pub fn propval_cell_n(prop: *mut property, n: c_uint) -> cell_t;
}
extern "C" {
    pub fn get_node_phandle(root: *mut node, node: *mut node) -> cell_t;
}
extern "C" {
    pub fn guess_boot_cpuid(tree: *mut node) -> u32;
}
// Boot info (tree plus memreserve information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reserve_info {
    pub size: uint64_t address,,
    pub next: *mut reserve_info,
    pub labels: *mut label,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dt_info {
    pub dtsflags: c_uint,
    pub reservelist: *mut reserve_info,
    pub boot_cpuid_phys: u32,
    pub /: *mut *mut *mut node dt; / the device tree,
    pub /: *const *const *const char outname; / filename being written to, "-" for stdout,
}

// DTS version flags definitions
pub const DTSF_V1: c_uint = 0x0001	/* /dts-v1/ */;
pub const DTSF_PLUGIN: c_uint = 0x0002	/* /plugin/ */;
extern "C" {
    pub fn sort_tree(dti: *mut dt_info);
}
extern "C" {
    pub fn generate_labels_from_tree(dti: *mut dt_info, name: *const c_char);
}
extern "C" {
    pub fn generate_label_tree(dti: *mut dt_info, name: *const c_char, allocph: bool);
}
extern "C" {
    pub fn generate_fixups_tree(dti: *mut dt_info, name: *const c_char);
}
extern "C" {
    pub fn fixup_phandles(dti: *mut dt_info, name: *const c_char);
}
extern "C" {
    pub fn generate_local_fixups_tree(dti: *mut dt_info, name: *const c_char);
}
extern "C" {
    pub fn local_fixup_phandles(dti: *mut dt_info, name: *const c_char);
}
// Checks
extern "C" {
    pub fn parse_checks_option(warn: bool, error: bool, arg: *const c_char);
}
extern "C" {
    pub fn process_checks(force: bool, dti: *mut dt_info);
}
// Flattened trees
extern "C" {
    pub fn dt_to_blob(f: *mut FILE, dti: *mut dt_info, version: c_int);
}
extern "C" {
    pub fn dt_to_asm(f: *mut FILE, dti: *mut dt_info, version: c_int);
}
// Tree source
extern "C" {
    pub fn add_phandle_marker(dti: *mut dt_info, prop: *mut property, offset: c_uint);
}
extern "C" {
    pub fn dt_to_source(f: *mut FILE, dti: *mut dt_info);
}
// YAML source
extern "C" {
    pub fn dt_to_yaml(f: *mut FILE, dti: *mut dt_info);
}
// FS trees
