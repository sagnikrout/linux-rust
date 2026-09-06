//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/kvm/s390/pci.h
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
//
// s390 kvm PCI passthrough support
//
// Copyright IBM Corp. 2022
//
// Author(s): Matthew Rosato <mjrosato@linux.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_zdev {
    pub zdev: *mut zpci_dev,
    pub kvm: *mut kvm,
    pub fib: zpci_fib,
    pub entry: list_head,
    pub user_account: *mut user_struct,
    pub mm_account: *mut mm_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_gaite {
    pub gisa: u32,
    pub gisc: u8,
    pub count: u8,
    pub reserved: u8,
    pub aisbo: u8,
    pub aisb: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_aift {
    pub gait: *mut zpci_gaite,
    pub sbv: *mut airq_iv,
    pub kzdev: *mut kvm_zdev,
    pub /: *mut *mut spinlock_t gait_lock; / Protects the gait, used during AEN forward,
    pub /: *mut *mut mutex aift_lock; / Protects the other structures in aift,
}

extern "C" {
    pub fn kvm_s390_pci_aen_init(nisc: u8) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pci_aen_exit();
}
extern "C" {
    pub fn kvm_s390_pci_init_list(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_pci_clear_list(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_pci_zpci_op(kvm: *mut kvm, args: *mut kvm_s390_zpci_op) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pci_init() -> int __init;
}
extern "C" {
    pub fn kvm_s390_pci_exit();
}
// No SHM on certain machines
