//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/binfmt_misc.h
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

pub const BINFMT_MISC_OPS_NAME_MAX: c_int = 16;
// Longest name a 'B' entry can bind an interpreter under.
pub const BINFMT_MISC_INTERP_NAME_MAX: c_int = 32;
// Most interpreters one entry can bind.
pub const BINFMT_MISC_INTERP_MAX: c_int = 100;
//
// struct binfmt_misc_interp - an interpreter an entry was registered with
// @list: link in the entry's list, in registration order
// @file: the file, opened at registration and never resolved again
// @ucounts: the UCOUNT_BINFMT_MISC_INTERPRETERS charge the binding took
// @path: the path it was registered under, used as the name the interpreter
// runs under; stored after @name in the same allocation
// @name: the name the load program selects it by; empty for the fixed
// interpreter of a static 'F' entry
//
// Owned by the entry and living exactly as long as it does. The list head
// is handed to the handler's load program for the duration of one exec,
// which picks one with bpf_binprm_select_interp().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binfmt_misc_interp {
    pub list: list_head,
    pub file: *mut file,
    pub ucounts: *mut ucounts,
    pub path: *const c_char,
    pub name: [c_char; ],
}

//
// enum bpf_binprm_flags - per-exec invocation flags a load program can request
// @BPF_BINPRM_PRESERVE_ARGV0: keep the caller's argv[0] (like the 'P' flag)
// @BPF_BINPRM_CREDENTIALS: compute credentials from the binary; implies execfd
// (like the 'C' flag)
// @BPF_BINPRM_EXECFD: pass the binary via AT_EXECFD (like the 'O' flag)
// @BPF_BINPRM_TRANSPARENT: leave argv untouched, the interpreter takes the
// binary from AT_EXECFD (like the 'T' flag); implies
// execfd, excludes preserve-argv0
// @BPF_BINPRM_LOADER: substitute the interpreter for the binary's PT_INTERP
// and run the binary as a native exec (like the 'L'
// flag); excludes every other flag
//
// Set from a load program with bpf_binprm_set_flags(). Unlike a static entry,
// a bpf handler chooses these per exec rather than once at registration.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_binprm_flags {
    BPF_BINPRM_PRESERVE_ARGV0	= (1ULL << 0),
    BPF_BINPRM_CREDENTIALS		= (1ULL << 1),
    BPF_BINPRM_EXECFD		= (1ULL << 2),
    BPF_BINPRM_TRANSPARENT		= (1ULL << 3),
    BPF_BINPRM_LOADER		= (1ULL << 4),
}

//
// struct binfmt_misc_ops - bpf-backed binary type handler
// @match: decide whether the handler applies to @bprm; consulted from the
// entry lookup walk like static magic and extension matching, in
// registration order with first-match-wins semantics; sleepable,
// so it can read the binary to decide, but the verifier rejects
// the interpreter selection kfuncs in it
// @load:  select an interpreter for the matched @bprm via
// bpf_binprm_set_interp(), or one the entry bound via
// bpf_binprm_select_interp(), and return zero; a match is
// committed, so a failure fails the exec instead of falling
// through to later entries; -ENOEXEC does not fail the exec but
// moves on to the remaining binary formats
// @name: name that 'B' entries reference the handler by
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binfmt_misc_ops {
    pub bprm): *mut *mut bool (match)(struct linux_binprm,
    pub bprm): *mut *mut int (load)(struct linux_binprm,
    pub name: [c_char; BINFMT_MISC_OPS_NAME_MAX],
}

extern "C" {
    pub fn binfmt_misc_put_ops(ops: *const binfmt_misc_ops);
}
extern "C" {
    pub fn bpf_prog_is_binfmt_misc_ops(prog: *const bpf_prog) -> bool;
}

