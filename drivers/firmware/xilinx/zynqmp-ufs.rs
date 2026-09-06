//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/xilinx/zynqmp-ufs.c
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
// Firmware Layer for UFS APIs
//
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//

// Register Node IDs
pub const PM_REGNODE_PMC_IOU_SLCR: c_uint = 0x30000002 /* PMC IOU SLCR */;
pub const PM_REGNODE_EFUSE_CACHE: c_uint = 0x30000003 /* EFUSE Cache */;
// Register Offsets for PMC IOU SLCR
pub const SRAM_CSR_OFFSET: c_uint = 0x104C /* SRAM Control and Status */;
pub const TXRX_CFGRDY_OFFSET: c_uint = 0x1054 /* M-PHY TX-RX Config ready */;
// Masks for SRAM Control and Status Register

// Mask to check M-PHY TX-RX configuration readiness

// Register Offsets for EFUSE Cache
pub const UFS_CAL_1_OFFSET: c_uint = 0xBE8 /* UFS Calibration Value */;
//
// zynqmp_pm_is_mphy_tx_rx_config_ready - check M-PHY TX-RX config readiness
// @is_ready:	Store output status (true/false)
//
// Return:	Returns 0 on success or error value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_is_mphy_tx_rx_config_ready(is_ready: *mut bool) -> c_int {
    int zynqmp_pm_is_mphy_tx_rx_config_ready(bool *is_ready)
    {
    u32 regval;
    int ret;
    if (!is_ready)
    return -EINVAL;
    ret = zynqmp_pm_sec_read_reg(PM_REGNODE_PMC_IOU_SLCR, TXRX_CFGRDY_OFFSET, &regval);
    if (ret)
    return ret;
    regval &= TX_RX_CFG_RDY_MASK;
    if (regval)
// is_ready = true;
    else
// is_ready = false;
    return ret;
    }
    EXPORT_SYMBOL_GPL(zynqmp_pm_is_mphy_tx_rx_config_ready);
//
// zynqmp_pm_is_sram_init_done - check SRAM initialization
// @is_done:	Store output status (true/false)
//
// Return:	Returns 0 on success or error value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_is_sram_init_done(is_done: *mut bool) -> c_int {
    int zynqmp_pm_is_sram_init_done(bool *is_done)
    {
    u32 regval;
    int ret;
    if (!is_done)
    return -EINVAL;
    ret = zynqmp_pm_sec_read_reg(PM_REGNODE_PMC_IOU_SLCR, SRAM_CSR_OFFSET, &regval);
    if (ret)
    return ret;
    regval &= SRAM_CSR_INIT_DONE_MASK;
    if (regval)
// is_done = true;
    else
// is_done = false;
    return ret;
    }
    EXPORT_SYMBOL_GPL(zynqmp_pm_is_sram_init_done);
//
// zynqmp_pm_set_sram_bypass - Set SRAM bypass Control
//
// Return:	Returns 0 on success or error value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_set_sram_bypass() -> c_int {
    int zynqmp_pm_set_sram_bypass(void)
    {
    u32 sram_csr;
    int ret;
    ret = zynqmp_pm_sec_read_reg(PM_REGNODE_PMC_IOU_SLCR, SRAM_CSR_OFFSET, &sram_csr);
    if (ret)
    return ret;
    sram_csr &= ~SRAM_CSR_EXT_LD_DONE_MASK;
    sram_csr |= SRAM_CSR_BYPASS_MASK;
    return zynqmp_pm_sec_mask_write_reg(PM_REGNODE_PMC_IOU_SLCR, SRAM_CSR_OFFSET,
    GENMASK(2, 1), sram_csr);
    }
    EXPORT_SYMBOL_GPL(zynqmp_pm_set_sram_bypass);
//
// zynqmp_pm_get_ufs_calibration_values - Read UFS calibration values
// @val:	Store the calibration value
//
// Return:	Returns 0 on success or error value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_get_ufs_calibration_values(val: *mut u32) -> c_int {
    int zynqmp_pm_get_ufs_calibration_values(u32 *val)
    {
    return zynqmp_pm_sec_read_reg(PM_REGNODE_EFUSE_CACHE, UFS_CAL_1_OFFSET, val);
    }
    EXPORT_SYMBOL_GPL(zynqmp_pm_get_ufs_calibration_values);
