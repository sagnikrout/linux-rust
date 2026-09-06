//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/strobemeta.h
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
// Copyright (c) 2019 Facebook

pub type pid_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct {
pub const TASK_COMM_LEN: c_int = 16;
pub const PERF_MAX_STACK_DEPTH: c_int = 127;
pub const STROBE_TYPE_INVALID: c_int = 0;
pub const STROBE_TYPE_INT: c_int = 1;
pub const STROBE_TYPE_STR: c_int = 2;
pub const STROBE_TYPE_MAP: c_int = 3;
pub const STACK_TABLE_EPOCH_SHIFT: c_int = 20;
pub const STROBE_MAX_STR_LEN: c_int = 1;
pub const STROBE_MAX_CFGS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_value_header {
//
// meaning depends on type:
// 1. int: 0, if value not set, 1 otherwise
// 2. str: 1 always, whether value is set or not is determined by ptr
// 3. map: 1 always, pointer points to additional struct with number
// of entries (up to STROBE_MAX_MAP_ENTRIES)
//
    pub len: u16,
//
// _reserved might be used for some future fields/flags, but we always
// want to keep strobe_value_header to be 8 bytes, so BPF can read 16
// bytes in one go and get both header and value
//
    pub _reserved: [u8; 6],
}

//
// strobe_value_generic is used from BPF probe only, but needs to be a union
// of strobe_value_int/strobe_value_str/strobe_value_map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_value_generic {
    pub header: strobe_value_header,
    pub val: i64,
    pub ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_value_int {
    pub header: strobe_value_header,
    pub value: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_value_str {
    pub header: strobe_value_header,
    pub value: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_value_map {
    pub header: strobe_value_header,
    pub value: *const *const strobe_map_raw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_map_entry {
    pub key: *const *const c_char,
    pub val: *const *const c_char,
}

//
// Map of C-string key/value pairs with fixed maximum capacity. Each map has
// corresponding int64 ID, which application can use (or ignore) in whatever
// way appropriate. Map is "write-only", there is no way to get data out of
// map. Map is intended to be used to provide metadata for profilers and is
// not to be used for internal in-app communication. All methods are
// thread-safe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_map_raw {
//
// general purpose unique ID that's up to application to decide
// whether and how to use; for request metadata use case id is unique
// request ID that's used to match metadata with stack traces on
// Strobelight backend side
//
    pub id: i64,
// number of used entries in map
    pub cnt: i64,
//
// having volatile doesn't change anything on BPF side, but clang
// emits warnings for passing `volatile const char *` into
// bpf_probe_read_user_str that expects just `const char *`
//
    pub tag: *const *const c_char,
//
// key/value entries, each consisting of 2 pointers to key and value
// C strings
//
    pub entries: [strobe_map_entry; STROBE_MAX_MAP_ENTRIES],
}

// Following values define supported values of TLS mode

pub const TLS_LOCAL_EXEC: c_int = 0;
pub const TLS_IMM_EXEC: c_int = 1;
pub const TLS_GENERAL_DYN: c_int = 2;
//
// structure that universally represents TLS location (both for static
// executables and shared libraries)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_value_loc {
//
// tls_mode defines what TLS mode was used for particular metavariable:
// - -1 (TLS_NOT_SET) - no metavariable;
// - 0 (TLS_LOCAL_EXEC) - Local Executable mode;
// - 1 (TLS_IMM_EXEC) - Immediate Executable mode;
// - 2 (TLS_GENERAL_DYN) - General Dynamic mode;
// Local Dynamic mode is not yet supported, because never seen in
// practice.  Mode defines how offset field is interpreted. See
// calc_location() in below for details.
//
    pub tls_mode: i64,
//
// TLS_LOCAL_EXEC: offset from thread pointer (fs:0 for x86-64,
// tpidr_el0 for aarch64).
// TLS_IMM_EXEC: absolute address of GOT entry containing offset
// from thread pointer;
// TLS_GENERAL_DYN: absolute address of double GOT entry
// containing tls_index_t struct;
//
    pub offset: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobemeta_cfg {
    pub req_meta_idx: i64,
    pub int_locs: [strobe_value_loc; STROBE_MAX_INTS],
    pub str_locs: [strobe_value_loc; STROBE_MAX_STRS],
    pub map_locs: [strobe_value_loc; STROBE_MAX_MAPS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_map_descr {
    pub id: u64,
    pub tag_len: i16,
//
// cnt <0 - map value isn't set;
// 0 - map has id set, but no key/value entries
//
    pub cnt: i16,
//
// both key_lens[i] and val_lens[i] should be >0 for present key/value
// entry
//
    pub key_lens: [u16; STROBE_MAX_MAP_ENTRIES],
    pub val_lens: [u16; STROBE_MAX_MAP_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobemeta_payload {
// req_id has valid request ID, if req_meta_valid == 1
    pub req_id: i64,
    pub req_meta_valid: u8,
//
// mask has Nth bit set to 1, if Nth metavar was present and
// successfully read
//
    pub int_vals_set_mask: u64,
    pub int_vals: [i64; STROBE_MAX_INTS],
// len is >0 for present values
    pub str_lens: [u16; STROBE_MAX_STRS],
// if map_descrs[i].cnt == -1, metavar is not present/set
    pub map_descrs: [strobe_map_descr; STROBE_MAX_MAPS],
//
// payload has compactly packed values of str and map variables in the
// form: strval1\0strval2\0map1key1\0map1val1\0map2key1\0map2val1\0
// (and so on); str_lens[i], key_lens[i] and val_lens[i] determines
// value length
//
    pub payload: [c_char; STROBE_MAX_PAYLOAD],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobelight_bpf_sample {
    pub ktime: u64,
    pub comm: [c_char; TASK_COMM_LEN],
    pub pid: pid_t,
    pub user_stack_id: c_int,
    pub kernel_stack_id: c_int,
    pub has_meta: c_int,
    pub metadata: strobemeta_payload,
//
// makes it possible to pass (<real payload size> + 1) as data size to
// perf_submit() to avoid perf_submit's paranoia about passing zero as
// size, as it deduces that <real payload size> might be
// **theoretically** zero
//
    pub dummy_safeguard: c_char,
}

// Type for the dtv.
// https://github.com/lattera/glibc/blob/master/nptl/sysdeps/x86_64/tls.h#L34
// Partial definition for tcbhead_t
// https://github.com/bminor/glibc/blob/master/sysdeps/x86_64/nptl/tls.h#L42
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcbhead {
    pub tcb: *mut *mut c_void,
    pub dtv: *mut *mut dtv_t,
}

//
// TLS module/offset information for shared library case.
// For x86-64, this is mapped onto two entries in GOT.
// For aarch64, this is pointed to by second GOT entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_index {
    pub module: u64,
    pub offset: u64,
}

//
// tls_mode value is:
// - -1 (TLS_NOT_SET), if no metavar is present;
// - 0 (TLS_LOCAL_EXEC), if metavar uses Local Executable mode of TLS
// (offset from fs:0 for x86-64 or tpidr_el0 for aarch64);
// - 1 (TLS_IMM_EXEC), if metavar uses Immediate Executable mode of TLS;
// - 2 (TLS_GENERAL_DYN), if metavar uses General Dynamic mode of TLS;
// This schema allows to use something like:
// (tls_mode + 1) * (tls_base + offset)
// to get NULL for "no metavar" location, or correct pointer for local
// executable mode without doing extra ifs.
//
// static executable is simple, we just have offset from
// tls_base
// multiply by (tls_mode + 1) to get NULL, if we have no
// metavar in this slot
//
// Other modes are more complicated, we need to jump through few hoops.
//
// For immediate executable mode (currently supported only for aarch64):
// - loc->offset is pointing to a GOT entry containing fixed offset
// relative to tls_base;
//
// For general dynamic mode:
// - loc->offset is pointing to a beginning of double GOT entries;
// - (for aarch64 only) second entry points to tls_index_t struct;
// - (for x86-64 only) two GOT entries are already tls_index_t;
// - tls_index_t->module is used to find start of TLS section in
// which variable resides;
// - tls_index_t->offset provides offset within that TLS section,
// pointing to value of variable.
//
// valid module index is always positive
// dtv = ((struct tcbhead *)tls_base)->dtv[tls_index.module]
// if pointer has (void *)-1 value, then TLS wasn't initialized yet

//
// if bpf_probe_read_user_str returns error (<0), due to casting to
// unsigned int, it will become big number, so next check is
// sufficient to check for errors AND prove to BPF verifier, that
// bpf_probe_read_user_str won't return anything bigger than
// STROBE_MAX_STR_LEN
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum read_type {
    READ_INT_VAR,
    READ_MAP_VAR,
    READ_STR_VAR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_var_ctx {
    pub data: *mut strobemeta_payload,
    pub tls_base: *mut c_void,
    pub cfg: *mut strobemeta_cfg,
    pub payload_off: usize,
// value gets mutated
    pub value: *mut strobe_value_generic,
    pub type: read_type,
}

// lose precision info for ctx->payload_off, verifier won't track
// double xor, barrier_var() is needed to force clang keep both xors.
//

//
// read_strobe_meta returns NULL, if no metadata was read; otherwise returns
// pointer to *right after* payload ends
//

//
// we don't have struct task_struct definition, it should be:
// tls_base = (void *)task->thread.fsbase;
//

// this should not really happen, here only to satisfy verifier

//
// return pointer right after end of payload, so it's possible to
// calculate exact amount of useful data that needs to be sent
//
// should always be true
