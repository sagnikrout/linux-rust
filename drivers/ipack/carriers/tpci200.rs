//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ipack/carriers/tpci200.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// driver for the carrier TEWS TPCI-200
//
// Copyright (C) 2009-2012 CERN (www.cern.ch)
// Author: Nicolas Serafini, EIC2 SA
// Author: Samuel Iglesias Gonsalvez <siglesias@igalia.com>
//

pub const TPCI200_NB_SLOT: c_uint = 0x4;
pub const TPCI200_NB_BAR: c_uint = 0x6;
pub const TPCI200_VENDOR_ID: c_uint = 0x1498;
pub const TPCI200_DEVICE_ID: c_uint = 0x30C8;
pub const TPCI200_SUBVENDOR_ID: c_uint = 0x1498;
pub const TPCI200_SUBDEVICE_ID: c_uint = 0x300A;
pub const TPCI200_CFG_MEM_BAR: c_int = 0;
pub const TPCI200_IP_INTERFACE_BAR: c_int = 2;
pub const TPCI200_IO_ID_INT_SPACES_BAR: c_int = 3;
pub const TPCI200_MEM16_SPACE_BAR: c_int = 4;
pub const TPCI200_MEM8_SPACE_BAR: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpci200_regs {
    pub revision: __le16,
// writes to control should occur with the mutex held to protect
// read-modify-write operations
    pub control: [__le16; 4],
    pub reset: __le16,
    pub status: __le16,
    pub reserved: [u8; 242],
    pub __packed: },
pub const TPCI200_IFACE_SIZE: c_uint = 0x100;
pub const TPCI200_IO_SPACE_OFF: c_uint = 0x0000;
pub const TPCI200_IO_SPACE_INTERVAL: c_uint = 0x0100;
pub const TPCI200_IO_SPACE_SIZE: c_uint = 0x0080;
pub const TPCI200_ID_SPACE_OFF: c_uint = 0x0080;
pub const TPCI200_ID_SPACE_INTERVAL: c_uint = 0x0100;
pub const TPCI200_ID_SPACE_SIZE: c_uint = 0x0040;
pub const TPCI200_INT_SPACE_OFF: c_uint = 0x00C0;
pub const TPCI200_INT_SPACE_INTERVAL: c_uint = 0x0100;
pub const TPCI200_INT_SPACE_SIZE: c_uint = 0x0040;
pub const TPCI200_IOIDINT_SIZE: c_uint = 0x0400;
pub const TPCI200_MEM8_SPACE_INTERVAL: c_uint = 0x00400000;
pub const TPCI200_MEM8_SPACE_SIZE: c_uint = 0x00400000;
pub const TPCI200_MEM16_SPACE_INTERVAL: c_uint = 0x00800000;
pub const TPCI200_MEM16_SPACE_SIZE: c_uint = 0x00800000;
// control field in tpci200_regs
pub const TPCI200_INT0_EN: c_uint = 0x0040;
pub const TPCI200_INT1_EN: c_uint = 0x0080;
pub const TPCI200_INT0_EDGE: c_uint = 0x0010;
pub const TPCI200_INT1_EDGE: c_uint = 0x0020;
pub const TPCI200_ERR_INT_EN: c_uint = 0x0008;
pub const TPCI200_TIME_INT_EN: c_uint = 0x0004;
pub const TPCI200_RECOVER_EN: c_uint = 0x0002;
pub const TPCI200_CLK32: c_uint = 0x0001;
// reset field in tpci200_regs
pub const TPCI200_A_RESET: c_uint = 0x0001;
pub const TPCI200_B_RESET: c_uint = 0x0002;
pub const TPCI200_C_RESET: c_uint = 0x0004;
pub const TPCI200_D_RESET: c_uint = 0x0008;
// status field in tpci200_regs
pub const TPCI200_A_TIMEOUT: c_uint = 0x1000;
pub const TPCI200_B_TIMEOUT: c_uint = 0x2000;
pub const TPCI200_C_TIMEOUT: c_uint = 0x4000;
pub const TPCI200_D_TIMEOUT: c_uint = 0x8000;
pub const TPCI200_A_ERROR: c_uint = 0x0100;
pub const TPCI200_B_ERROR: c_uint = 0x0200;
pub const TPCI200_C_ERROR: c_uint = 0x0400;
pub const TPCI200_D_ERROR: c_uint = 0x0800;
pub const TPCI200_A_INT0: c_uint = 0x0001;
pub const TPCI200_A_INT1: c_uint = 0x0002;
pub const TPCI200_B_INT0: c_uint = 0x0004;
pub const TPCI200_B_INT1: c_uint = 0x0008;
pub const TPCI200_C_INT0: c_uint = 0x0010;
pub const TPCI200_C_INT1: c_uint = 0x0020;
pub const TPCI200_D_INT0: c_uint = 0x0040;
pub const TPCI200_D_INT1: c_uint = 0x0080;
pub const TPCI200_SLOT_INT_MASK: c_uint = 0x00FF;
// PCI Configuration registers. The PCI bridge is a PLX Technology PCI9030.
pub const LAS1_DESC: c_uint = 0x2C;
pub const LAS2_DESC: c_uint = 0x30;
// Bits in the LAS?_DESC registers
pub const LAS_BIT_BIGENDIAN: c_int = 24;

//
// struct slot_irq - slot IRQ definition.
// @vector	Vector number
// @handler	Handler called when IRQ arrives
// @arg		Handler argument
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot_irq {
    pub holder: *mut ipack_device,
    pub vector: c_int,
    pub ): *mut *mut irqreturn_t (handler)(void,
    pub arg: *mut c_void,
}

//
// struct tpci200_slot - data specific to the tpci200 slot.
// @slot_id	Slot identification gived to external interface
// @irq		Slot IRQ infos
// @io_phys	IO physical base address register of the slot
// @id_phys	ID physical base address register of the slot
// @int_phys	INT physical base address register of the slot
// @mem_phys	MEM physical base address register of the slot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpci200_slot {
    pub irq: *mut slot_irq,
}

//
// struct tpci200_infos - informations specific of the TPCI200 tpci200.
// @pci_dev		PCI device
// @interface_regs	Pointer to IP interface space (Bar 2)
// @ioidint_space	Pointer to IP ID, IO and INT space (Bar 3)
// @mem8_space		Pointer to MEM space (Bar 4)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpci200_infos {
    pub pdev: *mut pci_dev,
    pub interface_regs: *mut tpci200_regs __iomem,
    pub cfg_regs: *mut void __iomem,
    pub ipack_bus: *mut ipack_bus_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpci200_board {
    pub number: c_uint,
    pub mutex: mutex,
    pub regs_lock: spinlock_t,
    pub slots: *mut tpci200_slot,
    pub info: *mut tpci200_infos,
    pub mod_mem: [phys_addr_t; IPACK_SPACE_COUNT],
}
