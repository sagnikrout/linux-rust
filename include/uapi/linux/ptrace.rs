//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ptrace.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// ptrace.h
// structs and defines to help the user use the ptrace system call.
// has the defines to get at the registers.

pub const PTRACE_TRACEME: c_int = 0;
pub const PTRACE_PEEKTEXT: c_int = 1;
pub const PTRACE_PEEKDATA: c_int = 2;
pub const PTRACE_PEEKUSR: c_int = 3;
pub const PTRACE_POKETEXT: c_int = 4;
pub const PTRACE_POKEDATA: c_int = 5;
pub const PTRACE_POKEUSR: c_int = 6;
pub const PTRACE_CONT: c_int = 7;
pub const PTRACE_KILL: c_int = 8;
pub const PTRACE_SINGLESTEP: c_int = 9;
pub const PTRACE_ATTACH: c_int = 16;
pub const PTRACE_DETACH: c_int = 17;
pub const PTRACE_SYSCALL: c_int = 24;
// 0x4200-0x4300 are reserved for architecture-independent additions.
pub const PTRACE_SETOPTIONS: c_uint = 0x4200;
pub const PTRACE_GETEVENTMSG: c_uint = 0x4201;
pub const PTRACE_GETSIGINFO: c_uint = 0x4202;
pub const PTRACE_SETSIGINFO: c_uint = 0x4203;
//
// Generic ptrace interface that exports the architecture specific regsets
// using the corresponding NT_* types (which are also used in the core dump).
// Please note that the NT_PRSTATUS note type in a core dump contains a full
// 'struct elf_prstatus'. But the user_regset for NT_PRSTATUS contains just the
// elf_gregset_t that is the pr_reg field of 'struct elf_prstatus'. For all the
// other user_regset flavors, the user_regset layout and the ELF core dump note
// payload are exactly the same layout.
//
// This interface usage is as follows:
// struct iovec iov = { buf, len};
//
// ret = ptrace(PTRACE_GETREGSET/PTRACE_SETREGSET, pid, NT_XXX_TYPE, &iov);
//
// On the successful completion, iov.len will be updated by the kernel,
// specifying how much the kernel has written/read to/from the user's iov.buf.
//
pub const PTRACE_GETREGSET: c_uint = 0x4204;
pub const PTRACE_SETREGSET: c_uint = 0x4205;
pub const PTRACE_SEIZE: c_uint = 0x4206;
pub const PTRACE_INTERRUPT: c_uint = 0x4207;
pub const PTRACE_LISTEN: c_uint = 0x4208;
pub const PTRACE_PEEKSIGINFO: c_uint = 0x4209;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_peeksiginfo_args {
    pub /: *mut *mut __u64 off; / from which siginfo to start,
    pub flags: __u32,
    pub /: *mut *mut __s32 nr; / how may siginfos to take,
}

pub const PTRACE_GETSIGMASK: c_uint = 0x420a;
pub const PTRACE_SETSIGMASK: c_uint = 0x420b;
pub const PTRACE_SECCOMP_GET_FILTER: c_uint = 0x420c;
pub const PTRACE_SECCOMP_GET_METADATA: c_uint = 0x420d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_metadata {
    pub /: *mut *mut __u64 filter_off; / Input: which filter,
    pub /: *mut *mut __u64 flags; / Output: filter's flags,
}

pub const PTRACE_GET_SYSCALL_INFO: c_uint = 0x420e;
pub const PTRACE_SET_SYSCALL_INFO: c_uint = 0x4212;
pub const PTRACE_SYSCALL_INFO_NONE: c_int = 0;
pub const PTRACE_SYSCALL_INFO_ENTRY: c_int = 1;
pub const PTRACE_SYSCALL_INFO_EXIT: c_int = 2;
pub const PTRACE_SYSCALL_INFO_SECCOMP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_syscall_info {
    pub /: *mut *mut *mut __u8 op; / PTRACE_SYSCALL_INFO_,
    pub reserved: __u8,
    pub flags: __u16,
    pub arch: __u32,
    pub instruction_pointer: __u64,
    pub stack_pointer: __u64,
    pub nr: __u64,
    pub args: [__u64; 6],
    pub entry: },
    pub rval: __s64,
    pub is_error: __u8,
    pub exit: },
    pub nr: __u64,
    pub args: [__u64; 6],
    pub ret_data: __u32,
    pub reserved2: __u32,
    pub seccomp: },
}

pub const PTRACE_GET_RSEQ_CONFIGURATION: c_uint = 0x420f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_rseq_configuration {
    pub rseq_abi_pointer: __u64,
    pub rseq_abi_size: __u32,
    pub signature: __u32,
    pub flags: __u32,
    pub pad: __u32,
}

pub const PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG: c_uint = 0x4210;
pub const PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG: c_uint = 0x4211;
//
// struct ptrace_sud_config - Per-task configuration for Syscall User Dispatch
// @mode:	One of PR_SYS_DISPATCH_ON or PR_SYS_DISPATCH_OFF
// @selector:	Tracees user virtual address of SUD selector
// @offset:	SUD exclusion area (virtual address)
// @len:	Length of SUD exclusion area
//
// Used to get/set the syscall user dispatch configuration for a tracee.
// Selector is optional (may be NULL), and if invalid will produce
// a SIGSEGV in the tracee upon first access.
//
// If mode is PR_SYS_DISPATCH_ON, syscall dispatch will be enabled. If
// PR_SYS_DISPATCH_OFF, syscall dispatch will be disabled and all other
// parameters must be 0.  The value in *selector (if not null), also determines
// whether syscall dispatch will occur.
//
// The Syscall User Dispatch Exclusion area described by offset/len is the
// virtual address space from which syscalls will not produce a user
// dispatch.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_sud_config {
    pub mode: __u64,
    pub selector: __u64,
    pub offset: __u64,
    pub len: __u64,
}

// 0x4212 is PTRACE_SET_SYSCALL_INFO
//
// These values are stored in task->ptrace_message
// by ptrace_stop to describe the current syscall-stop.
//
pub const PTRACE_EVENTMSG_SYSCALL_ENTRY: c_int = 1;
pub const PTRACE_EVENTMSG_SYSCALL_EXIT: c_int = 2;
// Read signals from a shared (process wide) queue

// Wait extended result codes for the above trace options.
pub const PTRACE_EVENT_FORK: c_int = 1;
pub const PTRACE_EVENT_VFORK: c_int = 2;
pub const PTRACE_EVENT_CLONE: c_int = 3;
pub const PTRACE_EVENT_EXEC: c_int = 4;
pub const PTRACE_EVENT_VFORK_DONE: c_int = 5;
pub const PTRACE_EVENT_EXIT: c_int = 6;
pub const PTRACE_EVENT_SECCOMP: c_int = 7;
// Extended result codes which enabled by means other than options.
pub const PTRACE_EVENT_STOP: c_int = 128;
// Options set using PTRACE_SETOPTIONS or using PTRACE_SEIZE @data param
pub const PTRACE_O_TRACESYSGOOD: c_int = 1;

// eventless options

