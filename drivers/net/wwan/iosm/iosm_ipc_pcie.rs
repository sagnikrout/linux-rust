//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_pcie.h
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
// Copyright (C) 2020-21 Intel Corporation.
//

// Device ID
pub const INTEL_CP_DEVICE_7560_ID: c_uint = 0x7560;
pub const INTEL_CP_DEVICE_7360_ID: c_uint = 0x7360;
// Define for BAR area usage
pub const IPC_DOORBELL_BAR0: c_int = 0;
pub const IPC_SCRATCHPAD_BAR2: c_int = 2;
// Defines for DOORBELL registers information

// Number of MSI used for IPC
pub const IPC_MSI_VECTORS: c_int = 1;
// Total number of Maximum IPC IRQ vectors used for IPC

//
// enum ipc_pcie_sleep_state - Enum type to different sleep state transitions
// @IPC_PCIE_D0L12:	Put the sleep state in D0L12
// @IPC_PCIE_D3L2:	Put the sleep state in D3L2
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_pcie_sleep_state {
    IPC_PCIE_D0L12,
    IPC_PCIE_D3L2,
}

//
// struct iosm_pcie - IPC_PCIE struct.
// @pci:			Address of the device description
// @dev:			Pointer to generic device structure
// @ipc_regs:			Remapped CP doorbell address of the irq register
// set, to fire the doorbell irq.
// @scratchpad:			Remapped CP scratchpad address, to send the
// configuration. tuple and the IPC descriptors
// to CP in the ROM phase. The config tuple
// information are saved on the MSI scratchpad.
// @imem:			Pointer to imem data struct
// @ipc_regs_bar_nr:		BAR number to be used for IPC doorbell
// @scratchpad_bar_nr:		BAR number to be used for Scratchpad
// @nvec:			number of requested irq vectors
// @doorbell_reg_offset:	doorbell_reg_offset
// @doorbell_write:		doorbell write register
// @doorbell_capture:		doorbell capture resgister
// @suspend:			S2IDLE sleep/active
// @d3l2_support:		Read WWAN RTD3 BIOS setting for D3L2 support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_pcie {
    pub pci: *mut pci_dev,
    pub dev: *mut device,
    pub ipc_regs: *mut void __iomem,
    pub scratchpad: *mut void __iomem,
    pub imem: *mut iosm_imem,
    pub ipc_regs_bar_nr: c_int,
    pub scratchpad_bar_nr: c_int,
    pub nvec: c_int,
    pub doorbell_reg_offset: u32,
    pub doorbell_write: u32,
    pub doorbell_capture: u32,
    pub suspend: c_ulong,
    pub d3l2_support: ipc_pcie_sleep_state,
}

//
// struct ipc_skb_cb - Struct definition of the socket buffer which is mapped to
// the cb field of sbk
// @mapping:	Store physical or IOVA mapped address of skb virtual add.
// @direction:	DMA direction
// @len:	Length of the DMA mapped region
// @op_type:    Expected values are defined about enum ipc_ul_usr_op.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_skb_cb {
    pub mapping: dma_addr_t,
    pub direction: c_int,
    pub len: c_int,
    pub op_type: u8,
}

//
// enum ipc_ul_usr_op - Control operation to execute the right action on
// the user interface.
// @UL_USR_OP_BLOCKED:	The uplink app was blocked until CP confirms that the
// uplink buffer was consumed triggered by the IRQ.
// @UL_MUX_OP_ADB:	In MUX mode the UL ADB shall be addedd to the free list.
// @UL_DEFAULT:		SKB in non muxing mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_ul_usr_op {
    UL_USR_OP_BLOCKED,
    UL_MUX_OP_ADB,
    UL_DEFAULT,
}

//
// ipc_pcie_addr_map - Maps the kernel's virtual address to either IOVA
// address space or Physical address space, the mapping is
// stored in the skb's cb.
// @ipc_pcie:	Pointer to struct iosm_pcie
// @data:	Skb mem containing data
// @size:	Data size
// @mapping:	Dma mapping address
// @direction:	Data direction
//
// Returns: 0 on success and failure value on error
//
// ipc_pcie_addr_unmap - Unmaps the skb memory region from IOVA address space
// @ipc_pcie:	Pointer to struct iosm_pcie
// @size:	Data size
// @mapping:	Dma mapping address
// @direction:	Data direction
//
// ipc_pcie_alloc_skb - Allocate an uplink SKB for the given size.
// @ipc_pcie:	Pointer to struct iosm_pcie
// @size:	Size of the SKB required.
// @flags:	Allocation flags
// @mapping:	Copies either mapped IOVA add. or converted Phy address
// @direction:	DMA data direction
// @headroom:	Header data offset
//
// Returns: Pointer to ipc_skb on Success, NULL on failure.
//
// ipc_pcie_alloc_local_skb - Allocate a local SKB for the given size.
// @ipc_pcie:	Pointer to struct iosm_pcie
// @flags:	Allocation flags
// @size:	Size of the SKB required.
//
// Returns: Pointer to ipc_skb on Success, NULL on failure.
//
// ipc_pcie_kfree_skb - Free skb allocated by ipc_pcie_alloc_*_skb().
// @ipc_pcie:	Pointer to struct iosm_pcie
// @skb:	Pointer to the skb
//
extern "C" {
    pub fn ipc_pcie_kfree_skb(ipc_pcie: *mut iosm_pcie, skb: *mut sk_buff);
}
//
// ipc_pcie_check_data_link_active - Check Data Link Layer Active
// @ipc_pcie:	Pointer to struct iosm_pcie
//
// Returns: true if active, otherwise false
//
extern "C" {
    pub fn ipc_pcie_check_data_link_active(ipc_pcie: *mut iosm_pcie) -> bool;
}
//
// ipc_pcie_suspend - Callback invoked by pm_runtime_suspend. It decrements
// the device's usage count then, carry out a suspend,
// either synchronous or asynchronous.
// @ipc_pcie:	Pointer to struct iosm_pcie
//
// Returns: 0 on success and failure value on error
//
extern "C" {
    pub fn ipc_pcie_suspend(ipc_pcie: *mut iosm_pcie) -> c_int;
}
//
// ipc_pcie_resume - Callback invoked by pm_runtime_resume. It increments
// the device's usage count then, carry out a resume,
// either synchronous or asynchronous.
// @ipc_pcie:	Pointer to struct iosm_pcie
//
// Returns: 0 on success and failure value on error
//
extern "C" {
    pub fn ipc_pcie_resume(ipc_pcie: *mut iosm_pcie) -> c_int;
}
//
// ipc_pcie_check_aspm_enabled - Check if ASPM L1 is already enabled
// @ipc_pcie:			 Pointer to struct iosm_pcie
// @parent:			 True if checking ASPM L1 for parent else false
//
// Returns: true if ASPM is already enabled else false
//
// ipc_pcie_config_aspm - Configure ASPM L1
// @ipc_pcie:	Pointer to struct iosm_pcie
//
extern "C" {
    pub fn ipc_pcie_config_aspm(ipc_pcie: *mut iosm_pcie);
}
