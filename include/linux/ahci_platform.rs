//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ahci_platform.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AHCI SATA platform driver
//
// Copyright 2004-2005  Red Hat, Inc.
// Jeff Garzik <jgarzik@pobox.com>
// Copyright 2010  MontaVista Software, LLC.
// Anton Vorontsov <avorontsov@ru.mvista.com>
//

extern "C" {
    pub fn ahci_platform_enable_phys(hpriv: *mut ahci_host_priv) -> c_int;
}
extern "C" {
    pub fn ahci_platform_disable_phys(hpriv: *mut ahci_host_priv);
}
extern "C" {
    pub fn ahci_platform_enable_clks(hpriv: *mut ahci_host_priv) -> c_int;
}
extern "C" {
    pub fn ahci_platform_disable_clks(hpriv: *mut ahci_host_priv);
}
extern "C" {
    pub fn ahci_platform_deassert_rsts(hpriv: *mut ahci_host_priv) -> c_int;
}
extern "C" {
    pub fn ahci_platform_assert_rsts(hpriv: *mut ahci_host_priv) -> c_int;
}
extern "C" {
    pub fn ahci_platform_enable_regulators(hpriv: *mut ahci_host_priv) -> c_int;
}
extern "C" {
    pub fn ahci_platform_disable_regulators(hpriv: *mut ahci_host_priv);
}
extern "C" {
    pub fn ahci_platform_enable_resources(hpriv: *mut ahci_host_priv) -> c_int;
}
extern "C" {
    pub fn ahci_platform_disable_resources(hpriv: *mut ahci_host_priv);
}
extern "C" {
    pub fn ahci_platform_shutdown(pdev: *mut platform_device);
}
extern "C" {
    pub fn ahci_platform_suspend_host(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ahci_platform_resume_host(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ahci_platform_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ahci_platform_resume(dev: *mut device) -> c_int;
}

