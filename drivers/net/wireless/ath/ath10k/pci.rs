//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/pci.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2017 Qualcomm Atheros, Inc.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

//
// maximum number of bytes that can be
// handled atomically by DiagRead/DiagWrite
//
pub const DIAG_TRANSFER_LIMIT: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi_xfer {
    pub tx_done: bool,
    pub rx_done: bool,
    pub wait_for_resp: bool,
    pub resp_len: u32,
}

//
// PCI-specific Target state
//
// NOTE: Structure is shared between Host software and Target firmware!
//
// Much of this may be of interest to the Host so
// HOST_INTEREST->hi_interconnect_state points here
// (and all members are 32-bit quantities in order to
// facilitate Host access). In particular, Host software is
// required to initialize pipe_cfg_addr and svc_to_pipe_map.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_state {
// Pipe configuration Target address
// NB: ce_pipe_config[CE_COUNT]
    pub pipe_cfg_addr: u32,
// Service to pipe map Target address
// NB: service_to_pipe[PIPE_TO_CE_MAP_CN]
    pub svc_to_pipe_map: u32,
// number of MSI interrupts requested
    pub msi_requested: u32,
// number of MSI interrupts granted
    pub msi_granted: u32,
// Message Signalled Interrupt address
    pub msi_addr: u32,
// Base data
    pub msi_data: u32,
//
// Data for firmware interrupt;
// MSI data for other interrupts are
// in various SoC registers
//
    pub msi_fw_intr_data: u32,
// PCIE_PWR_METHOD_*
    pub power_mgmt_method: u32,
// PCIE_CONFIG_FLAG_*
    pub config_flags: u32,
}

// PCIE_CONFIG_FLAG definitions
pub const PCIE_CONFIG_FLAG_ENABLE_L1: c_uint = 0x0000001;
// Per-pipe state.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_pci_pipe {
// Handle of underlying Copy Engine
    pub ce_hdl: *mut ath10k_ce_pipe,
// Our pipe number; facilitates use of pipe_info ptrs.
    pub pipe_num: u8,
// Convenience back pointer to hif_ce_state.
    pub hif_ce_state: *mut ath10k,
    pub buf_sz: usize,
// protects compl_free and num_send_allowed
    pub pipe_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_pci_supp_chip {
    pub dev_id: u32,
    pub rev_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_pci_irq_mode {
    ATH10K_PCI_IRQ_AUTO = 0,
    ATH10K_PCI_IRQ_INTX = 1,
    ATH10K_PCI_IRQ_MSI = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_pci {
    pub pdev: *mut pci_dev,
    pub dev: *mut device,
    pub ar: *mut ath10k,
    pub mem: *mut void __iomem,
    pub mem_len: usize,
// Operating interrupt mode
    pub oper_irq_mode: ath10k_pci_irq_mode,
    pub pipe_info: [ath10k_pci_pipe; CE_COUNT_MAX],
// Copy Engine used for Diagnostic Accesses
    pub ce_diag: *mut ath10k_ce_pipe,
// For protecting ce_diag
    pub ce_diag_mutex: mutex,
    pub dump_work: work_struct,
    pub ce: ath10k_ce,
    pub rx_post_retry: timer_list,
// Due to HW quirks it is recommended to disable ASPM during device
// bootup. To do that the original PCI-E Link Control is stored before
// device bootup is executed and re-programmed later.
//
    pub link_ctl: u16,
// Protects ps_awake and ps_wake_refcount
    pub ps_lock: spinlock_t,
// The device has a special powersave-oriented register. When device is
// considered asleep it drains less power and driver is forbidden from
// accessing most MMIO registers. If host were to access them without
// waking up the device might scribble over host memory or return
// 0xdeadbeef readouts.
//
    pub ps_wake_refcount: c_ulong,
// Waking up takes some time (up to 2ms in some cases) so it can be bad
// for latency. To mitigate this the device isn't immediately allowed
// to sleep after all references are undone - instead there's a grace
// period after which the powersave register is updated unless some
// activity to/from device happened in the meantime.
//
// Also see comments on ATH10K_PCI_SLEEP_GRACE_PERIOD_MSEC.
//
    pub ps_timer: timer_list,
// MMIO registers are used to communicate with the device. With
// intensive traffic accessing powersave register would be a bit
// wasteful overhead and would needlessly stall CPU. It is far more
// efficient to rely on a variable in RAM and update it only upon
// powersave register state changes.
//
    pub ps_awake: bool,
// pci power save, disable for QCA988X and QCA99X0.
// Writing 'false' to this variable avoids frequent locking
// on MMIO read/write.
//
    pub pci_ps: bool,
// Chip specific pci reset routine used to do a safe reset
    pub ar): *mut *mut int (pci_soft_reset)(struct ath10k,
// Chip specific pci full reset function
    pub ar): *mut *mut int (pci_hard_reset)(struct ath10k,
// chip specific methods for converting target CPU virtual address
// space to CE address space
//
    pub addr): *mut *mut *mut u32 (targ_cpu_to_ce_addr)(struct ath10k ar, u32,
    pub attr: *mut ce_attr,
    pub pipe_config: *mut ce_pipe_config,
    pub serv_to_pipe: *mut ce_service_to_pipe,
// Keep this entry in the last, memory for struct ath10k_ahb is
// allocated (ahb support enabled case) in the continuation of
// this struct.
//
    pub ahb: [ath10k_ahb; ],
}

pub const ATH10K_PCI_RX_POST_RETRY_MS: c_int = 50;

pub const BAR_NUM: c_int = 0;
pub const CDC_WAR_MAGIC_STR: c_uint = 0xceef0000;
pub const CDC_WAR_DATA_CE: c_int = 4;
// Wait up to this many Ms for a Diagnostic Access CE operation to complete

pub const DIAG_ACCESS_CE_WAIT_US: c_int = 50;
extern "C" {
    pub fn ath10k_pci_write32(ar: *mut ath10k, offset: u32, value: u32);
}
extern "C" {
    pub fn ath10k_pci_soc_write32(ar: *mut ath10k, addr: u32, val: u32);
}
extern "C" {
    pub fn ath10k_pci_reg_write32(ar: *mut ath10k, addr: u32, val: u32);
}
extern "C" {
    pub fn ath10k_pci_read32(ar: *mut ath10k, offset: u32) -> u32;
}
extern "C" {
    pub fn ath10k_pci_soc_read32(ar: *mut ath10k, addr: u32) -> u32;
}
extern "C" {
    pub fn ath10k_pci_reg_read32(ar: *mut ath10k, addr: u32) -> u32;
}
extern "C" {
    pub fn ath10k_pci_hif_get_free_queue_number(ar: *mut ath10k, pipe: u8) -> u16;
}
extern "C" {
    pub fn ath10k_pci_hif_power_down(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_pci_alloc_pipes(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_pci_free_pipes(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_pci_rx_replenish_retry(t: *mut timer_list);
}
extern "C" {
    pub fn ath10k_pci_ce_deinit(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_pci_init_napi(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_pci_init_pipes(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_pci_init_config(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_pci_rx_post(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_pci_flush(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_pci_enable_intx_irq(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_pci_irq_pending(ar: *mut ath10k) -> bool;
}
extern "C" {
    pub fn ath10k_pci_disable_and_clear_intx_irq(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_pci_irq_msi_fw_mask(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_pci_wait_for_target_init(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_pci_setup_resource(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_pci_release_resource(ar: *mut ath10k);
}
// QCA6174 is known to have Tx/Rx issues when SOC_WAKE register is poked too
// frequently. To avoid this put SoC to sleep after a very conservative grace
// period. Adjust with great care.
//
pub const ATH10K_PCI_SLEEP_GRACE_PERIOD_MSEC: c_int = 60;
