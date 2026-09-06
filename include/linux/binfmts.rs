//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/binfmts.h
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

pub const CORENAME_MAX_SIZE: c_int = 128;
// Interpreter selection staged by a bpf binfmt_misc handler.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binfmt_misc_bpf {
// interpreters the matched entry bound, selectable by name
    pub bpf_interps: *const list_head,
    pub /: *const *const *const char bpf_interp; / interpreter selected by a bpf handler,
    pub /: *mut *mut *mut file bpf_interp_file; / the bound interpreter it selected,
    pub /: *const *const *const char bpf_interp_arg; / interpreter argument from a bpf handler,
    pub /: *mut *mut u64 bpf_flags; / enum bpf_binprm_flags from a bpf handler,
}

//
// This structure is used to hold the arguments that are used when loading binaries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_binprm {

    pub vma: *mut vm_area_struct,
    pub vma_pages: c_ulong,
    pub /: *mut *mut unsigned long argmin; / rlimit marker for copy_strings(),
    pub page: [*mut page; MAX_ARG_PAGES],
    pub mm: *mut mm_struct,
    pub /: *mut *mut *mut mm_old_mm; / replaced address space, freed by setup_new_exec(),
// user_ns published to task->exec_state at execve, narrowed by would_dump().
    pub user_ns: *mut user_namespace,
    pub /: *mut *mut unsigned long p; / current top of mem,
// Should an execfd be passed to userspace?
// Use the creds of a script (see binfmt_misc)
//
// Set by bprm_creds_for_exec hook to indicate a
// privilege-gaining exec has happened. Used to set
// AT_SECURE auxv for glibc.
//
// Set when errors can no longer be returned to the
// original userspace.
//
// Set when "comm" must come from the dentry.
//
// Set by user space to check executability according to the
// caller's environment.
//
    pub /: *mut *mut *mut file executable; / Executable to pass to the interpreter,
    pub interpreter: *mut file,
    pub loader: *mut file,
    pub file: *mut file,
    pub /: *mut *mut *mut cred cred; / new credentials,
    pub /: *mut *mut *mut int unsafe; / how unsafe this exec is (mask of LSM_UNSAFE_),
    pub /: *mut *mut unsigned int per_clear; / bits to clear in current->personality,
    pub envc: int argc,,
    pub /: *const *const *const char filename; / Name of binary as seen by procps,
    pub Most: *const *const *const char interp; / Name of the binary really executed.,
    pub /: *const *const *const char fdpath; / generated filename for execveat,
    pub /: *mut *mut binfmt_misc_bpf; / bpf handler interpreter selection,
    pub interp_flags: unsigned,
    pub /: *mut *mut int execfd; / File descriptor of the executable,
    pub exec: c_ulong,
    pub /: *mut *mut rlimit rlim_stack; / Saved RLIMIT_STACK used during exec.,
    pub buf: [c_char; BINPRM_BUF_SIZE],
    pub __randomize_layout: },
pub const BINPRM_FLAGS_ENFORCE_NONDUMP_BIT: c_int = 0;

// filename of the binary will be inaccessible after exec
pub const BINPRM_FLAGS_PATH_INACCESSIBLE_BIT: c_int = 2;

// preserve argv0 for the interpreter
pub const BINPRM_FLAGS_PRESERVE_ARGV0_BIT: c_int = 3;

// binfmt_misc dispatched to the interpreter transparently
pub const BINPRM_FLAGS_TRANSPARENT_INTERP_BIT: c_int = 4;

//
// bprm_at_flags - the AT_FLAGS this invocation implies
// @bprm: binary that is being executed
//
// Tell the program on the receiving end which dispatch contract it got.
//
// Return: the AT_FLAGS value for this exec
//
// Transparency preserves the whole argv, argv[0] included.
    pub AT_FLAGS_TRANSPARENT_INTERP: return,
    pub AT_FLAGS_PRESERVE_ARGV0: return,
    pub 0: return,
//
// This structure defines the functions that are used to load the binary formats that
// linux accepts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_binfmt {
    pub lh: list_head,
    pub module: *mut module,
    pub ): *mut *mut int (load_binary)(struct linux_binprm,

    pub cprm): *mut *mut int (core_dump)(struct coredump_params,
    pub /: *mut *mut unsigned long min_coredump; / minimal dump size,

    pub __randomize_layout: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binfmt_misc {
    pub entries: hlist_head,
    pub entries_lock: spinlock_t,
    pub enabled: bool,
    pub __randomize_layout: },
    pub init_binfmt_misc: extern struct binfmt_misc,

    pub insert): *mut *mut extern void __register_binfmt(struct linux_binfmt fmt, int,
// Registration of default binfmt handlers
    pub 0): __register_binfmt(fmt,,
// Same as above, but adds a new binfmt at the top of the list
    pub 1): __register_binfmt(fmt,,
    pub ): *mut extern void unregister_binfmt(struct linux_binfmt,
    pub ): *mut extern int __must_check remove_arg_zero(struct linux_binprm,
    pub bprm): *mut *mut extern int begin_new_exec(struct linux_binprm,
    pub bprm): *mut *mut extern void setup_new_exec(struct linux_binprm,
    pub bprm): *mut extern void finalize_exec(struct linux_binprm,
    pub ): *mut *mut extern void would_dump(struct linux_binprm , struct file,
    pub path): *const *const *const file bprm_open_interpreter(linux_binprm bprm, char,
    pub bprm): *mut void bprm_drop_loader(struct linux_binprm,
    pub suid_dumpable: extern int,
// Stack area protections

    pub executable_stack): c_int,
    pub sp_location): *mut c_ulong,
    pub bprm): *const *const extern int bprm_change_interp(char interp, struct linux_binprm,
    pub bprm): *const *const int copy_string_kernel(char arg, struct linux_binprm,
    pub new): *mut extern void set_binfmt(struct linux_binfmt,
    pub size_t): *mut *mut extern ssize_t read_code(struct file , unsigned long, loff_t,,
    pub envp): *const *const *const *const char argv, char,
