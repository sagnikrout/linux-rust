//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/scsi_sas_internal.h
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
pub const SAS_HOST_ATTRS: c_int = 0;
pub const SAS_PHY_ATTRS: c_int = 17;
pub const SAS_PORT_ATTRS: c_int = 1;
pub const SAS_RPORT_ATTRS: c_int = 8;
pub const SAS_END_DEV_ATTRS: c_int = 5;
pub const SAS_EXPANDER_ATTRS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_internal {
    pub t: scsi_transport_template,
    pub f: *mut sas_function_template,
    pub dft: *mut sas_domain_function_template,
    pub private_host_attrs: [device_attribute; SAS_HOST_ATTRS],
    pub private_phy_attrs: [device_attribute; SAS_PHY_ATTRS],
    pub private_port_attrs: [device_attribute; SAS_PORT_ATTRS],
    pub private_rphy_attrs: [device_attribute; SAS_RPORT_ATTRS],
    pub private_end_dev_attrs: [device_attribute; SAS_END_DEV_ATTRS],
    pub private_expander_attrs: [device_attribute; SAS_EXPANDER_ATTRS],
    pub phy_attr_cont: transport_container,
    pub port_attr_cont: transport_container,
    pub rphy_attr_cont: transport_container,
    pub end_dev_attr_cont: transport_container,
    pub expander_attr_cont: transport_container,
//
// The array of null terminated pointers to attributes
// needed by scsi_sysfs.c
//
    pub 1]: *mut *mut device_attribute host_attrs[SAS_HOST_ATTRS +,
    pub 1]: *mut *mut device_attribute phy_attrs[SAS_PHY_ATTRS +,
    pub 1]: *mut *mut device_attribute port_attrs[SAS_PORT_ATTRS +,
    pub 1]: *mut *mut device_attribute rphy_attrs[SAS_RPORT_ATTRS +,
    pub 1]: *mut *mut device_attribute end_dev_attrs[SAS_END_DEV_ATTRS +,
    pub 1]: *mut *mut device_attribute expander_attrs[SAS_EXPANDER_ATTRS +,
}

