//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/hvm/hvm_op.h
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

// Get/set subcommands: the second argument of the hypercall is a
// pointer to a xen_hvm_param struct.
pub const HVMOP_set_param: c_int = 0;
pub const HVMOP_get_param: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_hvm_param {
    pub /: *mut *mut domid_t domid; / IN,
    pub /: *mut *mut uint32_t index; / IN,
    pub /: *mut *mut uint64_t value; / IN/OUT,
}

// Hint from PV drivers for pagetable destruction.
pub const HVMOP_pagetable_dying: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_hvm_pagetable_dying {
// Domain with a pagetable about to be destroyed.
    pub domid: domid_t,
// guest physical address of the toplevel pagetable dying
    pub gpa: aligned_u64,
}

pub type xen_hvm_pagetable_dying_t = xen_hvm_pagetable_dying;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hvmmem_type_t {
    HVMMEM_ram_rw,             /* Normal read/write guest RAM */
    HVMMEM_ram_ro,             /* Read-only; writes are discarded */
    HVMMEM_mmio_dm,            /* Reads and write go to the device model */
}

pub const HVMOP_get_mem_type: c_int = 15;
// Return hvmmem_type_t for the specified pfn.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_hvm_get_mem_type {
// Domain to be queried.
    pub domid: domid_t,
// OUT variable.
    pub mem_type: u16,
    pub /: *mut *mut uint16_t pad[2]; / align next field on 8-byte boundary,
// IN variable.
    pub pfn: u64,
}

//
// HVMOP_set_evtchn_upcall_vector: Set a <vector> that should be used for event
// channel upcalls on the specified <vcpu>. If set,
// this vector will be used in preference to the
// domain global callback via (see
// HVM_PARAM_CALLBACK_IRQ).
//
pub const HVMOP_set_evtchn_upcall_vector: c_int = 23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_hvm_evtchn_upcall_vector {
    pub vcpu: u32,
    pub vector: u8,
}

pub type xen_hvm_evtchn_upcall_vector_t = xen_hvm_evtchn_upcall_vector;

