//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_pci.h
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
// Copyright (c) 2021, MediaTek Inc.
// Copyright (c) 2021-2022, Intel Corporation.
//
// Authors:
// Haijun Liu <haijun.liu@mediatek.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
// Moises Veleta <moises.veleta@intel.com>
//

// struct t7xx_addr_base - holds base addresses
// @pcie_mac_ireg_base: PCIe MAC register base
// @pcie_ext_reg_base: used to calculate base addresses for CLDMA, DPMA and MHCCIF registers
// @pcie_dev_reg_trsl_addr: used to calculate the register base address
// @infracfg_ao_base: base address used in CLDMA reset operations
// @mhccif_rc_base: host view of MHCCIF rc base addr
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_addr_base {
    pub pcie_mac_ireg_base: *mut void __iomem,
    pub pcie_ext_reg_base: *mut void __iomem,
    pub pcie_dev_reg_trsl_addr: u32,
    pub infracfg_ao_base: *mut void __iomem,
    pub mhccif_rc_base: *mut void __iomem,
}

extern "C" {
    pub fn irqreturn_t(irq: *mut *mut t7xx_intr_callback)(int, param: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_mode {
    T7XX_UNKNOWN,
    T7XX_READY,
    T7XX_RESET,
    T7XX_FASTBOOT_SWITCHING,
    T7XX_FASTBOOT_DOWNLOAD,
    T7XX_FASTBOOT_DUMP,
    T7XX_MODE_LAST, /* must always be last */
}

// struct t7xx_pci_dev - MTK device context structure
// @intr_handler: array of handler function for request_threaded_irq
// @intr_thread: array of thread_fn for request_threaded_irq
// @callback_param: array of cookie passed back to interrupt functions
// @pdev: PCI device
// @base_addr: memory base addresses of HW components
// @md: modem interface
// @ccmni_ctlb: context structure used to control the network data path
// @rgu_pci_irq_en: RGU callback ISR registered and active
// @md_pm_entities: list of pm entities
// @md_pm_entity_mtx: protects md_pm_entities list
// @pm_sr_ack: ack from the device when went to sleep or woke up
// @md_pm_state: state for resume/suspend
// @md_pm_lock: protects PCIe sleep lock
// @sleep_disable_count: PCIe L1.2 lock counter
// @sleep_lock_acquire: indicates that sleep has been disabled
// @mode: indicates the device mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_pci_dev {
    pub intr_handler: [t7xx_intr_callback; EXT_INT_NUM],
    pub intr_thread: [t7xx_intr_callback; EXT_INT_NUM],
    pub callback_param: [*mut c_void; EXT_INT_NUM],
    pub pdev: *mut pci_dev,
    pub base_addr: t7xx_addr_base,
    pub md: *mut t7xx_modem,
    pub ccmni_ctlb: *mut t7xx_ccmni_ctrl,
    pub rgu_pci_irq_en: bool,
    pub init_done: completion,
// Low Power Items
    pub md_pm_entities: list_head,
    pub /: *mut *mut mutex md_pm_entity_mtx; / Protects MD PM entities list,
    pub pm_sr_ack: completion,
    pub md_pm_state: core::sync::atomic::AtomicI32,
    pub /: *mut *mut spinlock_t md_pm_lock; / Protects PCI resource lock,
    pub sleep_disable_count: c_uint,
    pub sleep_lock_acquire: completion,

    pub debugfs_dir: *mut dentry,

    pub mode: u32,
    pub debug_ports_show: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_pm_id {
    PM_ENTITY_ID_CTRL1,
    PM_ENTITY_ID_CTRL2,
    PM_ENTITY_ID_DATA,
    PM_ENTITY_ID_INVALID
}

// struct md_pm_entity - device power management entity
// @entity: list of PM Entities
// @suspend: callback invoked before sending D3 request to device
// @suspend_late: callback invoked after getting D3 ACK from device
// @resume_early: callback invoked before sending the resume request to device
// @resume: callback invoked after getting resume ACK from device
// @id: unique PM entity identifier
// @entity_param: parameter passed to the registered callbacks
//
// This structure is used to indicate PM operations required by internal
// HW modules such as CLDMA and DPMA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md_pm_entity {
    pub entity: list_head,
    pub entity_param): *mut *mut *mut int (suspend)(struct t7xx_pci_dev t7xx_dev, void,
    pub entity_param): *mut *mut *mut void (suspend_late)(struct t7xx_pci_dev t7xx_dev, void,
    pub entity_param): *mut *mut *mut void (resume_early)(struct t7xx_pci_dev t7xx_dev, void,
    pub entity_param): *mut *mut *mut int (resume)(struct t7xx_pci_dev t7xx_dev, void,
    pub id: t7xx_pm_id,
    pub entity_param: *mut c_void,
}

extern "C" {
    pub fn t7xx_pci_disable_sleep(t7xx_dev: *mut t7xx_pci_dev);
}
extern "C" {
    pub fn t7xx_pci_enable_sleep(t7xx_dev: *mut t7xx_pci_dev);
}
extern "C" {
    pub fn t7xx_pci_sleep_disable_complete(t7xx_dev: *mut t7xx_pci_dev) -> c_int;
}
extern "C" {
    pub fn t7xx_pci_pm_entity_register(t7xx_dev: *mut t7xx_pci_dev, pm_entity: *mut md_pm_entity) -> c_int;
}
extern "C" {
    pub fn t7xx_pci_pm_entity_unregister(t7xx_dev: *mut t7xx_pci_dev, pm_entity: *mut md_pm_entity) -> c_int;
}
extern "C" {
    pub fn t7xx_pci_pm_init_late(t7xx_dev: *mut t7xx_pci_dev);
}
extern "C" {
    pub fn t7xx_pci_pm_exp_detected(t7xx_dev: *mut t7xx_pci_dev);
}
extern "C" {
    pub fn t7xx_mode_update(t7xx_dev: *mut t7xx_pci_dev, mode: t7xx_mode);
}
extern "C" {
    pub fn t7xx_pci_reprobe(t7xx_dev: *mut t7xx_pci_dev, boot: bool) -> c_int;
}
extern "C" {
    pub fn t7xx_pci_reprobe_early(t7xx_dev: *mut t7xx_pci_dev) -> c_int;
}
