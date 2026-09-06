//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_common/adf_hw_arbiter.c
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

pub const ADF_ARB_NUM: c_int = 4;
pub const ADF_ARB_REG_SIZE: c_uint = 0x4;

    ADF_CSR_WR(csr_addr, (arb_offset) + \
    (ADF_ARB_REG_SIZE * (index)), value)

    ADF_CSR_WR(csr_addr, ((arb_offset) + (wt_offset)) + \
    (ADF_ARB_REG_SIZE * (index)), value)
#[no_mangle]
pub unsafe extern "C" fn adf_init_arb(accel_dev: *mut adf_accel_dev) -> c_int {
    int adf_init_arb(struct adf_accel_dev *accel_dev)
    {
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    void __iomem *csr = accel_dev.transport.banks[0].csr_addr;
    let mut ae_mask: c_ulong = hw_data.ae_mask;
    u32 arb_off, wt_off, arb_cfg;
    const u32 *thd_2_arb_cfg;
    struct arb_info info;
    int arb, i;
    hw_data.get_arb_info(&info);
    arb_cfg = info.arb_cfg;
    arb_off = info.arb_offset;
    wt_off = info.wt2sam_offset;
// Service arb configured for 32 bytes responses and
// ring flow control check enabled.
    for (arb = 0; arb < ADF_ARB_NUM; arb++)
    WRITE_CSR_ARB_SARCONFIG(csr, arb_off, arb, arb_cfg);
// Map worker threads to service arbiters
    thd_2_arb_cfg = hw_data.get_arb_mapping(accel_dev);
    for_each_set_bit(i, &ae_mask, hw_data.num_engines)
    WRITE_CSR_ARB_WT2SAM(csr, arb_off, wt_off, i, thd_2_arb_cfg[i]);
    return 0;
    }
    EXPORT_SYMBOL_GPL(adf_init_arb);
#[no_mangle]
pub unsafe extern "C" fn adf_update_ring_arb(ring: *mut adf_etr_ring_data) {
    void adf_update_ring_arb(struct adf_etr_ring_data *ring)
    {
    struct adf_accel_dev *accel_dev = ring.bank.accel_dev;
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    struct adf_hw_csr_ops *csr_ops = GET_CSR_OPS(accel_dev);
    let mut tx_ring_mask: u32 = hw_data.tx_rings_mask;
    let mut shift: u32 = hw_data.tx_rx_gap;
    u32 arben, arben_tx, arben_rx;
    u32 rx_ring_mask;
//
// Enable arbitration on a ring only if the TX half of the ring mask
// matches the RX part. This results in writes to CSR on both TX and
// RX update - only one is necessary, but both are done for
// simplicity.
//
    rx_ring_mask = tx_ring_mask << shift;
    arben_tx = (ring.bank.ring_mask & tx_ring_mask) >> 0;
    arben_rx = (ring.bank.ring_mask & rx_ring_mask) >> shift;
    arben = arben_tx & arben_rx;
    csr_ops.write_csr_ring_srv_arb_en(ring.bank.csr_addr,
    ring.bank.bank_number, arben);
    }
#[no_mangle]
pub unsafe extern "C" fn adf_exit_arb(accel_dev: *mut adf_accel_dev) {
    void adf_exit_arb(struct adf_accel_dev *accel_dev)
    {
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    struct adf_hw_csr_ops *csr_ops = GET_CSR_OPS(accel_dev);
    u32 arb_off, wt_off;
    struct arb_info info;
    void __iomem *csr;
    unsigned int i;
    hw_data.get_arb_info(&info);
    arb_off = info.arb_offset;
    wt_off = info.wt2sam_offset;
    if (!accel_dev.transport)
    return;
    csr = accel_dev.transport.banks[0].csr_addr;
    hw_data.get_arb_info(&info);
// Unmap worker threads to service arbiters
    for (i = 0; i < hw_data.num_engines; i++)
    WRITE_CSR_ARB_WT2SAM(csr, arb_off, wt_off, i, 0);
// Disable arbitration on all rings
    for (i = 0; i < GET_MAX_BANKS(accel_dev); i++)
    csr_ops.write_csr_ring_srv_arb_en(csr, i, 0);
    }
    EXPORT_SYMBOL_GPL(adf_exit_arb);
