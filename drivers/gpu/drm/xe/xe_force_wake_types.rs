//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_force_wake_types.h
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
// Copyright © 2022 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_force_wake_domain_id {
    XE_FW_DOMAIN_ID_GT = 0,
    XE_FW_DOMAIN_ID_RENDER,
    XE_FW_DOMAIN_ID_MEDIA,
    XE_FW_DOMAIN_ID_MEDIA_VDBOX0,
    XE_FW_DOMAIN_ID_MEDIA_VDBOX1,
    XE_FW_DOMAIN_ID_MEDIA_VDBOX2,
    XE_FW_DOMAIN_ID_MEDIA_VDBOX3,
    XE_FW_DOMAIN_ID_MEDIA_VDBOX4,
    XE_FW_DOMAIN_ID_MEDIA_VDBOX5,
    XE_FW_DOMAIN_ID_MEDIA_VDBOX6,
    XE_FW_DOMAIN_ID_MEDIA_VDBOX7,
    XE_FW_DOMAIN_ID_MEDIA_VEBOX0,
    XE_FW_DOMAIN_ID_MEDIA_VEBOX1,
    XE_FW_DOMAIN_ID_MEDIA_VEBOX2,
    XE_FW_DOMAIN_ID_MEDIA_VEBOX3,
    XE_FW_DOMAIN_ID_GSC,
    XE_FW_DOMAIN_ID_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_force_wake_domains {
    XE_FW_GT		= BIT(XE_FW_DOMAIN_ID_GT),
    XE_FW_RENDER		= BIT(XE_FW_DOMAIN_ID_RENDER),
    XE_FW_MEDIA		= BIT(XE_FW_DOMAIN_ID_MEDIA),
    XE_FW_MEDIA_VDBOX0	= BIT(XE_FW_DOMAIN_ID_MEDIA_VDBOX0),
    XE_FW_MEDIA_VDBOX1	= BIT(XE_FW_DOMAIN_ID_MEDIA_VDBOX1),
    XE_FW_MEDIA_VDBOX2	= BIT(XE_FW_DOMAIN_ID_MEDIA_VDBOX2),
    XE_FW_MEDIA_VDBOX3	= BIT(XE_FW_DOMAIN_ID_MEDIA_VDBOX3),
    XE_FW_MEDIA_VDBOX4	= BIT(XE_FW_DOMAIN_ID_MEDIA_VDBOX4),
    XE_FW_MEDIA_VDBOX5	= BIT(XE_FW_DOMAIN_ID_MEDIA_VDBOX5),
    XE_FW_MEDIA_VDBOX6	= BIT(XE_FW_DOMAIN_ID_MEDIA_VDBOX6),
    XE_FW_MEDIA_VDBOX7	= BIT(XE_FW_DOMAIN_ID_MEDIA_VDBOX7),
    XE_FW_MEDIA_VEBOX0	= BIT(XE_FW_DOMAIN_ID_MEDIA_VEBOX0),
    XE_FW_MEDIA_VEBOX1	= BIT(XE_FW_DOMAIN_ID_MEDIA_VEBOX1),
    XE_FW_MEDIA_VEBOX2	= BIT(XE_FW_DOMAIN_ID_MEDIA_VEBOX2),
    XE_FW_MEDIA_VEBOX3	= BIT(XE_FW_DOMAIN_ID_MEDIA_VEBOX3),
    XE_FW_GSC		= BIT(XE_FW_DOMAIN_ID_GSC),
    XE_FORCEWAKE_ALL	= BIT(XE_FW_DOMAIN_ID_COUNT)
}

//
// struct xe_force_wake_domain - Xe force wake power domain
//
// Represents an individual device-internal power domain.  The driver must
// ensure the power domain is awake before accessing registers or other
// hardware functionality that is part of the power domain.  Since different
// driver threads may access hardware units simultaneously, a reference count
// is used to ensure that the domain remains awake as long as any software
// is using the part of the hardware covered by the power domain.
//
// Hardware provides a register interface to allow the driver to request
// wake/sleep of power domains, although in most cases the actual action of
// powering the hardware up/down is handled by firmware (and may be subject to
// requirements and constraints outside of the driver's visibility) so the
// driver needs to wait for an acknowledgment that a wake request has been
// acted upon before accessing the parts of the hardware that reside within the
// power domain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_force_wake_domain {
// @id: domain force wake id
    pub id: xe_force_wake_domain_id,
// @reg_ctl: domain wake control register address
    pub reg_ctl: xe_reg,
// @reg_ack: domain ack register address
    pub reg_ack: xe_reg,
// @val: domain wake write value
    pub val: u32,
// @mask: domain mask
    pub mask: u32,
// @ref: domain reference
    pub ref: u32,
}

//
// struct xe_force_wake - Xe force wake collection
//
// Represents a collection of related power domains (struct
// xe_force_wake_domain) associated with a subunit of the device.
//
// Currently only used for GT power domains (where the term "forcewake" is used
// in the hardware documentation), although the interface could be extended to
// power wells in other parts of the hardware in the future.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_force_wake {
// @gt: back pointers to GT
    pub gt: *mut xe_gt,
// @lock: protects everything force wake struct
    pub lock: spinlock_t,
// @awake_domains: mask of all domains awake
    pub awake_domains: c_uint,
// @initialized_domains: mask of all initialized domains
    pub initialized_domains: c_uint,
// @domains: force wake domains
    pub domains: [xe_force_wake_domain; XE_FW_DOMAIN_ID_COUNT],
}
