//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/vcpu.h
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
// vcpu.h
//
// VCPU initialisation, query, and hotplug.
//
// Copyright (c) 2005, Keir Fraser <keir@xensource.com>
//
// Prototype for this hypercall is:
// int vcpu_op(int cmd, int vcpuid, void *extra_args)
// @cmd		   == VCPUOP_??? (VCPU operation).
// @vcpuid	   == VCPU to operate on.
// @extra_args == Operation-specific extra arguments (NULL if none).
//
// Initialise a VCPU. Each VCPU can be initialised only once. A
// newly-initialised VCPU will not run until it is brought up by VCPUOP_up.
//
// @extra_arg == pointer to vcpu_guest_context structure containing initial
// state for the VCPU.
//
pub const VCPUOP_initialise: c_int = 0;
//
// Bring up a VCPU. This makes the VCPU runnable. This operation will fail
// if the VCPU has not been initialised (VCPUOP_initialise).
//
pub const VCPUOP_up: c_int = 1;
//
// Bring down a VCPU (i.e., make it non-runnable).
// There are a few caveats that callers should observe:
// 1. This operation may return, and VCPU_is_up may return false, before the
// VCPU stops running (i.e., the command is asynchronous). It is a good
// idea to ensure that the VCPU has entered a non-critical loop before
// bringing it down. Alternatively, this operation is guaranteed
// synchronous if invoked by the VCPU itself.
// 2. After a VCPU is initialised, there is currently no way to drop all its
// references to domain memory. Even a VCPU that is down still holds
// memory references via its pagetable base pointer and GDT. It is good
// practise to move a VCPU onto an 'idle' or default page table, LDT and
// GDT before bringing it down.
//
pub const VCPUOP_down: c_int = 2;
// Returns 1 if the given VCPU is up.
pub const VCPUOP_is_up: c_int = 3;
//
// Return information about the state and running time of a VCPU.
// @extra_arg == pointer to vcpu_runstate_info structure.
//
pub const VCPUOP_get_runstate_info: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_runstate_info {
// VCPU's current state (RUNSTATE_*).
    pub state: c_int,
// When was current state entered (system time, ns)?
    pub state_entry_time: u64,
//
// Update indicator set in state_entry_time:
// When activated via VMASST_TYPE_runstate_update_flag, set during
// updates in guest memory mapped copy of vcpu_runstate_info.
//

//
// Time spent in each RUNSTATE_* (ns). The sum of these times is
// guaranteed not to drift from system time.
//
    pub time: [u64; 4],
}

// VCPU is currently running on a physical CPU.
pub const RUNSTATE_running: c_int = 0;
// VCPU is runnable, but not currently scheduled on any physical CPU.
pub const RUNSTATE_runnable: c_int = 1;
// VCPU is blocked (a.k.a. idle). It is therefore not runnable.
pub const RUNSTATE_blocked: c_int = 2;
//
// VCPU is not runnable, but it is not blocked.
// This is a 'catch all' state for things like hotplug and pauses by the
// system administrator (or for critical sections in the hypervisor).
// RUNSTATE_blocked dominates this state (it is the preferred state).
//
pub const RUNSTATE_offline: c_int = 3;
//
// Register a shared memory area from which the guest may obtain its own
// runstate information without needing to execute a hypercall.
// Notes:
// 1. The registered address may be virtual or physical, depending on the
// platform. The virtual address should be registered on x86 systems.
// 2. Only one shared area may be registered per VCPU. The shared area is
// updated by the hypervisor each time the VCPU is scheduled. Thus
// runstate.state will always be RUNSTATE_running and
// runstate.state_entry_time will indicate the system time at which the
// VCPU was last scheduled to run.
// @extra_arg == pointer to vcpu_register_runstate_memory_area structure.
//
pub const VCPUOP_register_runstate_memory_area: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_register_runstate_memory_area {
    pub h: GUEST_HANDLE(vcpu_runstate_info),
    pub v: *mut vcpu_runstate_info,
    pub p: u64,
    pub addr: },
}

//
// Set or stop a VCPU's periodic timer. Every VCPU has one periodic timer
// which can be set via these commands. Periods smaller than one millisecond
// may not be supported.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_set_periodic_timer {
    pub period_ns: u64,
}

//
// Set or stop a VCPU's single-shot timer. Every VCPU has one single-shot
// timer which can be set via these commands.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_set_singleshot_timer {
    pub timeout_abs_ns: u64,
    pub /: *mut *mut uint32_t flags; / VCPU_SSHOTTMR_???,
}

// Flags to VCPUOP_set_singleshot_timer.
// Require the timeout to be in the future (return -ETIME if it's passed).

//
// Register a memory location in the guest address space for the
// vcpu_info structure.  This allows the guest to place the vcpu_info
// structure in a convenient place, such as in a per-cpu data area.
// The pointer need not be page aligned, but the structure must not
// cross a page boundary.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_register_vcpu_info {
    pub /: *mut *mut uint64_t mfn; / mfn of page to place vcpu_info,
    pub /: *mut *mut uint32_t offset; / offset within page,
    pub /: *mut *mut uint32_t rsvd; / unused,
}

// Send an NMI to the specified VCPU. @extra_arg == NULL.
pub const VCPUOP_send_nmi: c_int = 11;
//
// Get the physical ID information for a pinned vcpu's underlying physical
// processor.  The physical ID informmation is architecture-specific.
// On x86: id[31:0]=apic_id, id[63:32]=acpi_id.
// This command returns -EINVAL if it is not a valid operation for this VCPU.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_get_physid {
    pub phys_id: u64,
}

//
// Register a memory location to get a secondary copy of the vcpu time
// parameters.  The master copy still exists as part of the vcpu shared
// memory area, and this secondary copy is updated whenever the master copy
// is updated (and using the same versioning scheme for synchronisation).
//
// The intent is that this copy may be mapped (RO) into userspace so
// that usermode can compute system time using the time info and the
// tsc.  Usermode will see an array of vcpu_time_info structures, one
// for each vcpu, and choose the right one by an existing mechanism
// which allows it to get the current vcpu number (such as via a
// segment limit).  It can then apply the normal algorithm to compute
// system time from the tsc.
//
// @extra_arg == pointer to vcpu_register_time_info_memory_area structure.
//
pub const VCPUOP_register_vcpu_time_memory_area: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_register_time_memory_area {
    pub h: GUEST_HANDLE(vcpu_time_info),
    pub v: *mut pvclock_vcpu_time_info,
    pub p: u64,
    pub addr: },
}
