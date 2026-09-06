//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/rseq/rseq-abi.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// rseq-abi.h
//
// Restartable sequences system call API
//
// Copyright (c) 2015-2022 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rseq_abi_cpu_id_state {
    RSEQ_ABI_CPU_ID_UNINITIALIZED			= -1,
    RSEQ_ABI_CPU_ID_REGISTRATION_FAILED		= -2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rseq_abi_flags {
    RSEQ_ABI_FLAG_UNREGISTER = (1 << 0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rseq_abi_cs_flags_bit {
    RSEQ_ABI_CS_FLAG_NO_RESTART_ON_PREEMPT_BIT	= 0,
    RSEQ_ABI_CS_FLAG_NO_RESTART_ON_SIGNAL_BIT	= 1,
    RSEQ_ABI_CS_FLAG_NO_RESTART_ON_MIGRATE_BIT	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rseq_abi_cs_flags {
    RSEQ_ABI_CS_FLAG_NO_RESTART_ON_PREEMPT	=
    (1U << RSEQ_ABI_CS_FLAG_NO_RESTART_ON_PREEMPT_BIT),
    RSEQ_ABI_CS_FLAG_NO_RESTART_ON_SIGNAL	=
    (1U << RSEQ_ABI_CS_FLAG_NO_RESTART_ON_SIGNAL_BIT),
    RSEQ_ABI_CS_FLAG_NO_RESTART_ON_MIGRATE	=
    (1U << RSEQ_ABI_CS_FLAG_NO_RESTART_ON_MIGRATE_BIT),
}

//
// struct rseq_abi_cs is aligned on 4 * 8 bytes to ensure it is always
// contained within a single cache-line. It is usually declared as
// link-time constant data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_abi_cs {
// Version of this structure.
    pub version: __u32,
// enum rseq_abi_cs_flags
    pub flags: __u32,
    pub start_ip: __u64,
// Offset from start_ip.
    pub post_commit_offset: __u64,
    pub abort_ip: __u64,
// C attribute field omitted
//
// rseq_abi_slice_ctrl - Time slice extension control structure
// @all:	Compound value
// @request:	Request for a time slice extension
// @granted:	Granted time slice extension
//
// @request is set by user space and can be cleared by user space or kernel
// space.  @granted is set and cleared by the kernel and must only be read
// by user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_abi_slice_ctrl {
    pub all: __u32,
    pub request: __u8,
    pub granted: __u8,
    pub __reserved: __u16,
}

//
// struct rseq_abi is aligned on 4 * 8 bytes to ensure it is always
// contained within a single cache-line.
//
// A single struct rseq_abi per thread is allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_abi {
//
// Restartable sequences cpu_id_start field. Updated by the
// kernel. Read by user-space with single-copy atomicity
// semantics. This field should only be read by the thread which
// registered this data structure. Aligned on 32-bit. Always
// contains a value in the range of possible CPUs, although the
// value may not be the actual current CPU (e.g. if rseq is not
// initialized). This CPU number value should always be compared
// against the value of the cpu_id field before performing a rseq
// commit or returning a value read from a data structure indexed
// using the cpu_id_start value.
//
    pub cpu_id_start: __u32,
//
// Restartable sequences cpu_id field. Updated by the kernel.
// Read by user-space with single-copy atomicity semantics. This
// field should only be read by the thread which registered this
// data structure. Aligned on 32-bit. Values
// RSEQ_CPU_ID_UNINITIALIZED and RSEQ_CPU_ID_REGISTRATION_FAILED
// have a special semantic: the former means "rseq uninitialized",
// and latter means "rseq initialization failed". This value is
// meant to be read within rseq critical sections and compared
// with the cpu_id_start value previously read, before performing
// the commit instruction, or read and compared with the
// cpu_id_start value before returning a value loaded from a data
// structure indexed using the cpu_id_start value.
//
    pub cpu_id: __u32,
//
// Restartable sequences rseq_cs field.
//
// Contains NULL when no critical section is active for the current
// thread, or holds a pointer to the currently active struct rseq_cs.
//
// Updated by user-space, which sets the address of the currently
// active rseq_cs at the beginning of assembly instruction sequence
// block, and set to NULL by the kernel when it restarts an assembly
// instruction sequence block, as well as when the kernel detects that
// it is preempting or delivering a signal outside of the range
// targeted by the rseq_cs. Also needs to be set to NULL by user-space
// before reclaiming memory that contains the targeted struct rseq_cs.
//
// Read and set by the kernel. Set by user-space with single-copy
// atomicity semantics. This field should only be updated by the
// thread which registered this data structure. Aligned on 64-bit.
//
    pub ptr64: __u64,
//
// The "arch" field provides architecture accessor for
// the ptr field based on architecture pointer size and
// endianness.
//

    pub ptr: __u64,

    pub /: *mut *mut __u32 padding; / Initialized to zero.,
    pub ptr: __u32,

    pub ptr: __u32,
    pub /: *mut *mut __u32 padding; / Initialized to zero.,

    pub arch: },
    pub rseq_cs: },
//
// Restartable sequences flags field.
//
// This field should only be updated by the thread which
// registered this data structure. Read by the kernel.
// Mainly used for single-stepping through rseq critical sections
// with debuggers.
//
// - RSEQ_ABI_CS_FLAG_NO_RESTART_ON_PREEMPT
// Inhibit instruction sequence block restart on preemption
// for this thread.
// - RSEQ_ABI_CS_FLAG_NO_RESTART_ON_SIGNAL
// Inhibit instruction sequence block restart on signal
// delivery for this thread.
// - RSEQ_ABI_CS_FLAG_NO_RESTART_ON_MIGRATE
// Inhibit instruction sequence block restart on migration for
// this thread.
//
    pub flags: __u32,
//
// Restartable sequences node_id field. Updated by the kernel. Read by
// user-space with single-copy atomicity semantics. This field should
// only be read by the thread which registered this data structure.
// Aligned on 32-bit. Contains the current NUMA node ID.
//
    pub node_id: __u32,
//
// Restartable sequences mm_cid field. Updated by the kernel. Read by
// user-space with single-copy atomicity semantics. This field should
// only be read by the thread which registered this data structure.
// Aligned on 32-bit. Contains the current thread's concurrency ID
// (allocated uniquely within a memory map).
//
    pub mm_cid: __u32,
//
// Time slice extension control structure. CPU local updates from
// kernel and user space.
//
    pub slice_ctrl: rseq_abi_slice_ctrl,
//
// Place holder to push the size above 32 bytes.
//
    pub __reserved: __u8,
//
// Flexible array member at end of structure, after last feature field.
//
    pub end: [c_char; ],
    pub __attribute__((aligned(256))): },
