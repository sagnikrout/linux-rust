//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/agilent_82350b/agilent_82350b.h
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
// copyright            : (C) 2002, 2004 by Frank Mori Hess
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_vendor_ids {
    PCI_VENDOR_ID_AGILENT = 0x15bc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_device_ids {
    PCI_DEVICE_ID_82350B = 0x0b01,
    PCI_DEVICE_ID_82351A = 0x1218
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_subdevice_ids {
    PCI_SUBDEVICE_ID_82350A = 0x10b0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_regions_82350a {
    PLX_MEM_REGION  = 0,
    PLX_IO_REGION   = 1,
    GPIB_82350A_REGION = 2,
    SRAM_82350A_REGION = 3,
    BORG_82350A_REGION = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_regions_82350b {
    GPIB_REGION = 0,
    SRAM_REGION = 1,
    MISC_REGION = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum board_model {
    MODEL_82350A,
    MODEL_82350B,
    MODEL_82351A
}

// struct which defines private_data for board
#[repr(C)]
#[derive(Copy, Clone)]
pub struct agilent_82350b_priv {
    pub tms9914_priv: tms9914_priv,
    pub pci_device: *mut pci_dev,
    pub /: *mut *mut *mut void __iomem plx_base; / 82350a only,
    pub gpib_base: *mut void __iomem,
    pub sram_base: *mut void __iomem,
    pub misc_base: *mut void __iomem,
    pub borg_base: *mut void __iomem,
    pub irq: c_int,
    pub card_mode_bits: c_ushort,
    pub event_status_bits: c_ushort,
    pub model: board_model,
    pub using_fifos: bool,
}

// registers
