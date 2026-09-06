//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/sched.h
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


// SPDX-License-Identifier: MIT
//
// sched.h
//
// Scheduler state interactions
//
// Copyright (c) 2005, Keir Fraser <keir@xensource.com>
//

//
// Guest Scheduler Operations
//
// The SCHEDOP interface provides mechanisms for a guest to interact
// with the scheduler, including yield, blocking and shutting itself
// down.
//
// The prototype for this hypercall is:
// long HYPERVISOR_sched_op(enum sched_op cmd, void *arg, ...)
//
// @cmd == SCHEDOP_??? (scheduler operation).
// @arg == Operation-specific extra argument(s), as described below.
// ...  == Additional Operation-specific extra arguments, described below.
//
// Versions of Xen prior to 3.0.2 provided only the following legacy version
// of this hypercall, supporting only the commands yield, block and shutdown:
// long sched_op(int cmd, unsigned long arg)
// @cmd == SCHEDOP_??? (scheduler operation).
// @arg == 0               (SCHEDOP_yield and SCHEDOP_block)
// == SHUTDOWN_* code (SCHEDOP_shutdown)
//
// This legacy version is available to new guests as:
// long HYPERVISOR_sched_op_compat(enum sched_op cmd, unsigned long arg)
//
// Voluntarily yield the CPU.
// @arg == NULL.
//
pub const SCHEDOP_yield: c_int = 0;
//
// Block execution of this VCPU until an event is received for processing.
// If called with event upcalls masked, this operation will atomically
// reenable event delivery and check for pending events before blocking the
// VCPU. This avoids a "wakeup waiting" race.
// @arg == NULL.
//
pub const SCHEDOP_block: c_int = 1;
//
// Halt execution of this domain (all VCPUs) and notify the system controller.
// @arg == pointer to sched_shutdown structure.
//
// If the sched_shutdown_t reason is SHUTDOWN_suspend then
// x86 PV guests must also set RDX (EDX for 32-bit guests) to the MFN
// of the guest's start info page.  RDX/EDX is the third hypercall
// argument.
//
// In addition, which reason is SHUTDOWN_suspend this hypercall
// returns 1 if suspend was cancelled or the domain was merely
// checkpointed, and 0 if it is resuming in a new domain.
//
pub const SCHEDOP_shutdown: c_int = 2;
//
// Poll a set of event-channel ports. Return when one or more are pending. An
// optional timeout may be specified.
// @arg == pointer to sched_poll structure.
//
pub const SCHEDOP_poll: c_int = 3;
//
// Declare a shutdown for another domain. The main use of this function is
// in interpreting shutdown requests and reasons for fully-virtualized
// domains.  A para-virtualized domain may use SCHEDOP_shutdown directly.
// @arg == pointer to sched_remote_shutdown structure.
//
pub const SCHEDOP_remote_shutdown: c_int = 4;
//
// Latch a shutdown code, so that when the domain later shuts down it
// reports this code to the control tools.
// @arg == sched_shutdown, as for SCHEDOP_shutdown.
//
pub const SCHEDOP_shutdown_code: c_int = 5;
//
// Setup, poke and destroy a domain watchdog timer.
// @arg == pointer to sched_watchdog structure.
// With id == 0, setup a domain watchdog timer to cause domain shutdown
// after timeout, returns watchdog id.
// With id != 0 and timeout == 0, destroy domain watchdog timer.
// With id != 0 and timeout != 0, poke watchdog timer and set new timeout.
//
pub const SCHEDOP_watchdog: c_int = 6;
//
// Override the current vcpu affinity by pinning it to one physical cpu or
// undo this override restoring the previous affinity.
// @arg == pointer to sched_pin_override structure.
//
// A negative pcpu value will undo a previous pin override and restore the
// previous cpu affinity.
// This call is allowed for the hardware domain only and requires the cpu
// to be part of the domain's cpupool.
//
pub const SCHEDOP_pin_override: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_shutdown {
    pub /: *mut *mut *mut unsigned int reason; / SHUTDOWN_ => shutdown reason,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_poll {
    pub ports: GUEST_HANDLE(evtchn_port_t),
    pub nr_ports: c_uint,
    pub timeout: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_remote_shutdown {
    pub /: *mut *mut domid_t domain_id; / Remote domain ID,
    pub /: *mut *mut *mut unsigned int reason; / SHUTDOWN_ => shutdown reason,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_watchdog {
    pub /: *mut *mut uint32_t id; / watchdog ID,
    pub /: *mut *mut uint32_t timeout; / timeout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_pin_override {
    pub pcpu: i32,
}

//
// Reason codes for SCHEDOP_shutdown. These may be interpreted by control
// software to determine the appropriate action. For the most part, Xen does
// not care about the shutdown code.
//

//
// Domain asked to perform 'soft reset' for it. The expected behavior is to
// reset internal Xen state for the domain returning it to the point where it
// was created but leaving the domain's memory contents and vCPU contexts
// intact. This will allow the domain to start over and set up all Xen specific
// interfaces again.
//
pub const SHUTDOWN_soft_reset: c_int = 5;

