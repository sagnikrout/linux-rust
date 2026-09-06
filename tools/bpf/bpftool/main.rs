//! Automatically rewritten from C Header to Rust Module
//! Source: tools/bpf/bpftool/main.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017-2018 Netronome Systems, Inc.
// BFD and kernel.h both define GCC_VERSION, differently

// Make sure we do not use kernel-only integer typedefs

pub const ERR_MAX_LEN: c_int = 1024;
pub const MAX_SIG_SIZE: c_int = 4096;

// keep in sync with the definition in skeleton/pid_iter.bpf.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_obj_type {
    BPF_OBJ_UNKNOWN,
    BPF_OBJ_PROG,
    BPF_OBJ_MAP,
    BPF_OBJ_LINK,
    BPF_OBJ_BTF,
}

extern "C" {
    pub fn __printf(_arg: 1, fmt: *const 2) p_err(char, ...);
}
extern "C" {
    pub fn __printf(_arg: 1, fmt: *const 2) p_info(char, ...);
}
extern "C" {
    pub fn is_prefix(pfx: *const c_char, str: *const c_char) -> bool;
}
extern "C" {
    pub fn detect_common_prefix(arg: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn fprint_hex(f: *mut FILE, arg: *mut c_void, n: c_uint, sep: *const c_char);
}
extern "C" {
    pub fn set_max_rlimit();
}
extern "C" {
    pub fn mount_tracefs(target: *const c_char) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct obj_ref {
    pub pid: c_int,
    pub comm: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct obj_refs {
    pub ref_cnt: c_int,
    pub has_bpf_cookie: bool,
    pub refs: *mut obj_ref,
    pub bpf_cookie: __u64,
}

extern "C" {
    pub fn delete_pinned_obj_table(table: *mut hashmap);
}
extern "C" {
    pub fn delete_obj_refs_table(table: *mut hashmap) -> __weak void;
}
extern "C" {
    pub fn print_dev_plain(ifindex: __u32, ns_dev: __u64, ns_inode: __u64);
}
extern "C" {
    pub fn print_dev_json(ifindex: __u32, ns_dev: __u64, ns_inode: __u64);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd {
    pub cmd: *const c_char,
    pub argv): *mut *mut int (func)(int argc, char,
}

pub const MAX_PROG_FULL_NAME: c_int = 128;
extern "C" {
    pub fn get_fd_type(fd: c_int) -> c_int;
}
extern "C" {
    pub fn mount_bpffs_for_file(file_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn create_and_mount_bpffs_dir(dir_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn do_pin_any(argc: c_int, argv: *mut c_char, : *mut *mut int (get_fd_by_id)(int, ): *mut c_char) -> c_int;
}
extern "C" {
    pub fn do_pin_fd(fd: c_int, name: *const c_char) -> c_int;
}
// commands available in bootstrap mode
extern "C" {
    pub fn do_gen(argc: c_int, argv: *mut c_char) -> c_int;
}
extern "C" {
    pub fn do_btf(argc: c_int, argv: *mut c_char) -> c_int;
}
// non-bootstrap only commands
extern "C" {
    pub fn parse_u32_arg(argc: *mut c_int, argv: *mut c_char, val: *mut __u32, what: *const c_char) -> c_int;
}
extern "C" {
    pub fn prog_parse_fd(argc: *mut c_int, argv: *mut c_char) -> c_int;
}
extern "C" {
    pub fn prog_parse_fds(argc: *mut c_int, argv: *mut c_char, fds: *mut c_int) -> c_int;
}
extern "C" {
    pub fn map_parse_fd(argc: *mut c_int, argv: *mut c_char, open_flags: __u32) -> c_int;
}
extern "C" {
    pub fn map_parse_fds(argc: *mut c_int, argv: *mut c_char, fds: *mut c_int, open_flags: __u32) -> c_int;
}

extern "C" {
    pub fn disasm_init() -> c_int;
}

extern "C" {
    pub fn print_data_json(data: *mut u8, len: usize);
}
extern "C" {
    pub fn print_hex_data_json(data: *mut u8, len: usize);
}
extern "C" {
    pub fn get_page_size() -> c_uint;
}
extern "C" {
    pub fn get_possible_cpus() -> c_uint;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_dumper {
    pub btf: *const btf,
    pub jw: *mut json_writer_t,
    pub is_plain_text: bool,
    pub prog_id_as_func_ptr: bool,
}

// btf_dumper_type - print data along with type information
// @d: an instance containing context for dumping types
// @type_id: index in btf->types array. this points to the type to be dumped
// @data: pointer the actual data, i.e. the values to be printed
//
// Returns zero on success and negative error code otherwise
//
extern "C" {
    pub fn do_xdp_dump(ifinfo: *mut ifinfomsg, tb: *mut nlattr) -> c_int;
}
extern "C" {
    pub fn hash_fn_for_key_as_id(key: c_long, ctx: *mut c_void) -> usize;
}
extern "C" {
    pub fn equal_fn_for_key_as_id(k1: c_long, k2: c_long, ctx: *mut c_void) -> bool;
}
// bpf_attach_type_input_str - convert the provided attach type value into a
// textual representation that we accept for input purposes.
//
// This function is similar in nature to libbpf_bpf_attach_type_str, but
// recognizes some attach type names that have been used by the program in the
// past and which do not follow the string inference scheme that libbpf uses.
// These textual representations should only be used for user input.
//
// @t: The attach type
// Returns a pointer to a static string identifying the attach type. NULL is
// returned for unknown bpf_attach_type values.
//
// print netfilter bpf_link info
extern "C" {
    pub fn netfilter_dump_plain(info: *const bpf_link_info);
}
extern "C" {
    pub fn netfilter_dump_json(info: *const bpf_link_info, wtr: *mut json_writer_t);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_config_option {
    pub name: *const c_char,
    pub macro_dump: bool,
}

extern "C" {
    pub fn bpftool_prog_sign(opts: *mut bpf_load_and_run_opts) -> c_int;
}
extern "C" {
    pub fn register_session_key(key_der_path: *const c_char) -> __u32;
}

