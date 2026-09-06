//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/ixd/ixd_dev.c
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
// Copyright (C) 2025 Intel Corporation

//
// ixd_ctlq_reg_init - Initialize default mailbox registers
// @adapter: PCI device driver-specific private data
// @ctlq_reg_tx: Transmit queue registers info to be filled
// @ctlq_reg_rx: Receive queue registers info to be filled
//
    void ixd_ctlq_reg_init(struct ixd_adapter *adapter,
    struct libie_ctlq_reg *ctlq_reg_tx,
    struct libie_ctlq_reg *ctlq_reg_rx)
    {
    struct libie_mmio_info *mmio_info = &adapter.cp_ctx.mmio_info;
// ctlq_reg_tx = (struct libie_ctlq_reg) {
    .head = libie_pci_get_mmio_addr(mmio_info, PF_FW_ATQH),
    .tail = libie_pci_get_mmio_addr(mmio_info, PF_FW_ATQT),
    .len = libie_pci_get_mmio_addr(mmio_info, PF_FW_ATQLEN),
    .addr_high = libie_pci_get_mmio_addr(mmio_info, PF_FW_ATQBAH),
    .addr_low = libie_pci_get_mmio_addr(mmio_info, PF_FW_ATQBAL),
    .len_mask = PF_FW_ATQLEN_ATQLEN_M,
    .len_ena_mask = PF_FW_ATQLEN_ATQENABLE_M,
    .head_mask = PF_FW_ATQH_ATQH_M,
    };
// ctlq_reg_rx = (struct libie_ctlq_reg) {
    .head = libie_pci_get_mmio_addr(mmio_info, PF_FW_ARQH),
    .tail = libie_pci_get_mmio_addr(mmio_info, PF_FW_ARQT),
    .len = libie_pci_get_mmio_addr(mmio_info, PF_FW_ARQLEN),
    .addr_high = libie_pci_get_mmio_addr(mmio_info, PF_FW_ARQBAH),
    .addr_low = libie_pci_get_mmio_addr(mmio_info, PF_FW_ARQBAL),
    .len_mask = PF_FW_ARQLEN_ARQLEN_M,
    .len_ena_mask = PF_FW_ARQLEN_ARQENABLE_M,
    .head_mask = PF_FW_ARQH_ARQH_M,
    };
    }
    static const struct ixd_reset_reg ixd_reset_reg = {
    .rstat  = PFGEN_RSTAT,
    .rstat_m = PFGEN_RSTAT_PFR_STATE_M,
    .rstat_ok_v = 0b01,
    .rtrigger = PFGEN_CTRL,
    .rtrigger_m = PFGEN_CTRL_PFSWR,
    };
//
// ixd_trigger_reset - Trigger PFR reset
// @adapter: the device with mapped reset register
//
#[no_mangle]
pub unsafe extern "C" fn ixd_trigger_reset(adapter: *mut ixd_adapter) {
    void ixd_trigger_reset(struct ixd_adapter *adapter)
    {
    void __iomem *addr;
    u32 reg_val;
    addr = libie_pci_get_mmio_addr(&adapter.cp_ctx.mmio_info,
    ixd_reset_reg.rtrigger);
    reg_val = readl(addr);
    writel(reg_val | ixd_reset_reg.rtrigger_m, addr);
    }
//
// ixd_check_reset_complete - Check if the PFR reset is completed
// @adapter: CPF being reset
//
// Return: %true if the register read indicates reset has been finished,
// %false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn ixd_check_reset_complete(adapter: *mut ixd_adapter) -> bool {
    bool ixd_check_reset_complete(struct ixd_adapter *adapter)
    {
    u32 reg_val, reset_status;
    void __iomem *addr;
    addr = libie_pci_get_mmio_addr(&adapter.cp_ctx.mmio_info,
    ixd_reset_reg.rstat);
    reg_val = readl(addr);
    reset_status = reg_val & ixd_reset_reg.rstat_m;
// 0xFFFFFFFF might be read if the other side hasn't cleared
// the register for us yet.
//
    if (reg_val != GENMASK(31, 0) &&
    reset_status == ixd_reset_reg.rstat_ok_v)
    return true;
    return false;
    }
