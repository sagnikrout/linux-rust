//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/rseq.h
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
// linux/rseq.h
//
// Restartable sequences system call API
//
// Copyright (c) 2015-2018 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rseq_cpu_id_state {
    RSEQ_CPU_ID_UNINITIALIZED		= -1,
    RSEQ_CPU_ID_REGISTRATION_FAILED		= -2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rseq_flags {
    RSEQ_FLAG_UNREGISTER			= (1 << 0),
    RSEQ_FLAG_SLICE_EXT_DEFAULT_ON		= (1 << 1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rseq_cs_flags_bit {
// Historical and unsupported bits
    RSEQ_CS_FLAG_NO_RESTART_ON_PREEMPT_BIT	= 0,
    RSEQ_CS_FLAG_NO_RESTART_ON_SIGNAL_BIT	= 1,
    RSEQ_CS_FLAG_NO_RESTART_ON_MIGRATE_BIT	= 2,
// (3) Intentional gap to keep new bits separate

// User read only feature flags
    RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE_BIT	= 4,
    RSEQ_CS_FLAG_SLICE_EXT_ENABLED_BIT	= 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rseq_cs_flags {
    RSEQ_CS_FLAG_NO_RESTART_ON_PREEMPT	=
    (1U << RSEQ_CS_FLAG_NO_RESTART_ON_PREEMPT_BIT),
    RSEQ_CS_FLAG_NO_RESTART_ON_SIGNAL	=
    (1U << RSEQ_CS_FLAG_NO_RESTART_ON_SIGNAL_BIT),
    RSEQ_CS_FLAG_NO_RESTART_ON_MIGRATE	=
    (1U << RSEQ_CS_FLAG_NO_RESTART_ON_MIGRATE_BIT),

    RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE	=
    (1U << RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE_BIT),
    RSEQ_CS_FLAG_SLICE_EXT_ENABLED		=
    (1U << RSEQ_CS_FLAG_SLICE_EXT_ENABLED_BIT),
}

//
// struct rseq_cs is aligned on 4 * 8 bytes to ensure it is always
// contained within a single cache-line. It is usually declared as
// link-time constant data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_cs {
// Version of this structure.
    pub version: __u32,
// enum rseq_cs_flags
    pub flags: __u32,
    pub start_ip: __u64,
// Offset from start_ip.
    pub post_commit_offset: __u64,
    pub abort_ip: __u64,
// C attribute field omitted
//
// rseq_slice_ctrl - Time slice extension control structure
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
pub struct rseq_slice_ctrl {
    pub all: __u32,
    pub request: __u8,
    pub granted: __u8,
    pub __reserved: __u16,
}

//
// The original size and alignment of the allocation for struct rseq is
// 32 bytes.
//
// The allocation size needs to be greater or equal to
// max(getauxval(AT_RSEQ_FEATURE_SIZE), 32), and the allocation needs to
// be aligned on max(getauxval(AT_RSEQ_ALIGN), 32).
//
// As an alternative, userspace is allowed to use both the original size
// and alignment of 32 bytes for backward compatibility.
//
// A single active struct rseq registration per thread is allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq {
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
// 32-bit architectures should update the low order bits of the
// rseq_cs field, leaving the high order bits initialized to 0.
//
    pub rseq_cs: __u64,
//
// Restartable sequences flags field.
//
// This field was initially intended to allow event masking for
// single-stepping through rseq critical sections with debuggers.
// The kernel does not support this anymore and the relevant bits
// are checked for being always false:
// - RSEQ_CS_FLAG_NO_RESTART_ON_PREEMPT
// - RSEQ_CS_FLAG_NO_RESTART_ON_SIGNAL
// - RSEQ_CS_FLAG_NO_RESTART_ON_MIGRATE
//
// It is now used for feature status advertisement by the kernel.
// See: enum rseq_cs_flags_bit for further information.
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
    pub slice_ctrl: rseq_slice_ctrl,
//
// Before rseq became extensible, its original size was 32 bytes even
// though the active rseq area was only 20 bytes.
// Exposing a 32 bytes feature size would make life needlessly painful
// for userspace. Therefore, add a reserved byte after byte 32
// to bump the rseq feature size from 32 to 33.
// The next field to be added to the rseq area will be larger
// than one byte, and will replace this reserved byte.
//
    pub __reserved: __u8,
//
// Flexible array member at end of structure, after last feature field.
//
    pub end: [c_char; ],
    pub __attribute__((aligned(32))): },
