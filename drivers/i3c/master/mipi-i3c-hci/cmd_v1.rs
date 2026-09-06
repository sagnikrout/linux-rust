//! Automatically rewritten from C to Rust
//! Source: drivers/i3c/master/mipi-i3c-hci/cmd_v1.c
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020, MIPI Alliance, Inc.
//
// Author: Nicolas Pitre <npitre@baylibre.com>
//
// I3C HCI v1.0/v1.1 Command Descriptor Handling
//

//
// Address Assignment Command
//

//
// Immediate Data Transfer Command
//

//
// Regular Data Transfer Command
//

//
// Combo Transfer (Write + Write/Read) Command
//

//
// Internal Control Command
//

// Data Transfer Speed and Mode
    enum hci_cmd_mode {
    MODE_I3C_SDR0		= 0x0,
    MODE_I3C_SDR1		= 0x1,
    MODE_I3C_SDR2		= 0x2,
    MODE_I3C_SDR3		= 0x3,
    MODE_I3C_SDR4		= 0x4,
    MODE_I3C_HDR_TSx	= 0x5,
    MODE_I3C_HDR_DDR	= 0x6,
    MODE_I3C_HDR_BT		= 0x7,
    MODE_I3C_Fm_FmP		= 0x8,
    MODE_I2C_Fm		= 0x0,
    MODE_I2C_FmP		= 0x1,
    MODE_I2C_UD1		= 0x2,
    MODE_I2C_UD2		= 0x3,
    MODE_I2C_UD3		= 0x4,
    };
#[no_mangle]
unsafe extern "C" fn get_i3c_mode(hci: *mut i3c_hci) -> enum hci_cmd_mode {
    static enum hci_cmd_mode get_i3c_mode(struct i3c_hci *hci)
    {
    struct i3c_bus *bus = i3c_master_get_bus(&hci.master);
    if (bus.scl_rate.i3c > 8000000)
    return MODE_I3C_SDR0;
    if (bus.scl_rate.i3c > 6000000)
    return MODE_I3C_SDR1;
    if (bus.scl_rate.i3c > 4000000)
    return MODE_I3C_SDR2;
    if (bus.scl_rate.i3c > 2000000)
    return MODE_I3C_SDR3;
    return MODE_I3C_SDR4;
    }
#[no_mangle]
unsafe extern "C" fn get_i2c_mode(hci: *mut i3c_hci) -> enum hci_cmd_mode {
    static enum hci_cmd_mode get_i2c_mode(struct i3c_hci *hci)
    {
    struct i3c_bus *bus = i3c_master_get_bus(&hci.master);
    if (bus.scl_rate.i2c >= 1000000)
    return MODE_I2C_FmP;
    return MODE_I2C_Fm;
    }
    static void fill_data_bytes(struct hci_xfer *xfer, u8 *data,
    unsigned int data_len)
    {
    xfer.cmd_desc[1] = 0;
    switch (data_len) {
    case 4:
    xfer.cmd_desc[1] |= CMD_I1_DATA_BYTE_4(data[3]);
    fallthrough;
    case 3:
    xfer.cmd_desc[1] |= CMD_I1_DATA_BYTE_3(data[2]);
    fallthrough;
    case 2:
    xfer.cmd_desc[1] |= CMD_I1_DATA_BYTE_2(data[1]);
    fallthrough;
    case 1:
    xfer.cmd_desc[1] |= CMD_I1_DATA_BYTE_1(data[0]);
    fallthrough;
    case 0:
    break;
    }
// we consumed all the data with the cmd descriptor
    xfer.data = core::ptr::null_mut();
    }
    static int hci_cmd_v1_prep_ccc(struct i3c_hci *hci,
    struct hci_xfer *xfer,
    u8 ccc_addr, u8 ccc_cmd, bool raw)
    {
    let mut dat_idx: c_uint = 0;
    let mut mode: enum hci_cmd_mode = get_i3c_mode(hci);
    u8 *data = xfer.data;
    let mut data_len: c_uint = xfer.data_len;
    let mut rnw: bool = xfer.rnw;
    int ret;
// this should never happen
    if (WARN_ON(raw))
    return -EINVAL;
    if (ccc_addr != I3C_BROADCAST_ADDR) {
    ret = mipi_i3c_hci_dat_v1.get_index(hci, ccc_addr);
    if (ret < 0)
    return ret;
    dat_idx = ret;
    }
    xfer.cmd_tid = hci_get_tid();
    if (!rnw && data_len <= 4) {
// we use an Immediate Data Transfer Command
    xfer.cmd_desc[0] =
    CMD_0_ATTR_I |
    CMD_I0_TID(xfer.cmd_tid) |
    CMD_I0_CMD(ccc_cmd) | CMD_I0_CP |
    CMD_I0_DEV_INDEX(dat_idx) |
    CMD_I0_DTT(data_len) |
    CMD_I0_MODE(mode);
    fill_data_bytes(xfer, data, data_len);
    } else {
// we use a Regular Data Transfer Command
    xfer.cmd_desc[0] =
    CMD_0_ATTR_R |
    CMD_R0_TID(xfer.cmd_tid) |
    CMD_R0_CMD(ccc_cmd) | CMD_R0_CP |
    CMD_R0_DEV_INDEX(dat_idx) |
    CMD_R0_MODE(mode) |
    (rnw ? CMD_R0_RNW : 0);
    xfer.cmd_desc[1] =
    CMD_R1_DATA_LENGTH(data_len);
    }
    return 0;
    }
    static void hci_cmd_v1_prep_i3c_xfer(struct i3c_hci *hci,
    struct i3c_dev_desc *dev,
    struct hci_xfer *xfer)
    {
    struct i3c_hci_dev_data *dev_data = i3c_dev_get_master_data(dev);
    let mut dat_idx: c_uint = dev_data.dat_idx;
    let mut mode: enum hci_cmd_mode = get_i3c_mode(hci);
    u8 *data = xfer.data;
    let mut data_len: c_uint = xfer.data_len;
    let mut rnw: bool = xfer.rnw;
    xfer.cmd_tid = hci_get_tid();
    if (!rnw && data_len <= 4) {
// we use an Immediate Data Transfer Command
    xfer.cmd_desc[0] =
    CMD_0_ATTR_I |
    CMD_I0_TID(xfer.cmd_tid) |
    CMD_I0_DEV_INDEX(dat_idx) |
    CMD_I0_DTT(data_len) |
    CMD_I0_MODE(mode);
    fill_data_bytes(xfer, data, data_len);
    } else {
// we use a Regular Data Transfer Command
    xfer.cmd_desc[0] =
    CMD_0_ATTR_R |
    CMD_R0_TID(xfer.cmd_tid) |
    CMD_R0_DEV_INDEX(dat_idx) |
    CMD_R0_MODE(mode) |
    (rnw ? CMD_R0_RNW : 0);
    xfer.cmd_desc[1] =
    CMD_R1_DATA_LENGTH(data_len);
    }
    }
    static void hci_cmd_v1_prep_i2c_xfer(struct i3c_hci *hci,
    struct i2c_dev_desc *dev,
    struct hci_xfer *xfer)
    {
    struct i3c_hci_dev_data *dev_data = i2c_dev_get_master_data(dev);
    let mut dat_idx: c_uint = dev_data.dat_idx;
    let mut mode: enum hci_cmd_mode = get_i2c_mode(hci);
    u8 *data = xfer.data;
    let mut data_len: c_uint = xfer.data_len;
    let mut rnw: bool = xfer.rnw;
    xfer.cmd_tid = hci_get_tid();
    if (!rnw && data_len <= 4) {
// we use an Immediate Data Transfer Command
    xfer.cmd_desc[0] =
    CMD_0_ATTR_I |
    CMD_I0_TID(xfer.cmd_tid) |
    CMD_I0_DEV_INDEX(dat_idx) |
    CMD_I0_DTT(data_len) |
    CMD_I0_MODE(mode);
    fill_data_bytes(xfer, data, data_len);
    } else {
// we use a Regular Data Transfer Command
    xfer.cmd_desc[0] =
    CMD_0_ATTR_R |
    CMD_R0_TID(xfer.cmd_tid) |
    CMD_R0_DEV_INDEX(dat_idx) |
    CMD_R0_MODE(mode) |
    (rnw ? CMD_R0_RNW : 0);
    xfer.cmd_desc[1] =
    CMD_R1_DATA_LENGTH(data_len);
    }
    }
#[no_mangle]
unsafe extern "C" fn hci_cmd_v1_daa(hci: *mut i3c_hci) -> c_int {
    static int hci_cmd_v1_daa(struct i3c_hci *hci)
    {
    struct hci_xfer *xfer;
    int ret, dat_idx = -1;
    let mut next_addr: u8 = 0;
    u64 pid;
    unsigned int dcr, bcr;
    DECLARE_COMPLETION_ONSTACK(done);
    xfer = hci_alloc_xfer(1);
    if (!xfer)
    return -ENOMEM;
//
// Simple for now: we allocate a temporary DAT entry, do a single
// DAA, register the device which will allocate its own DAT entry
// via the core callback, then free the temporary DAT entry.
// Loop until there is no more devices to assign an address to.
// Yes, there is room for improvements.
//
    for (;;) {
    ret = mipi_i3c_hci_dat_v1.alloc_entry(hci);
    if (ret < 0)
    break;
    dat_idx = ret;
    ret = i3c_master_get_free_addr(&hci.master, next_addr);
    if (ret < 0)
    break;
    next_addr = ret;
    dev_dbg(&hci.master.dev,
    "next_addr = 0x%02x, DAA using DAT %d",
    next_addr, dat_idx);
    mipi_i3c_hci_dat_v1.set_dynamic_addr(hci, dat_idx, next_addr);
    mipi_i3c_hci_dct_index_reset(hci);
    xfer.cmd_tid = hci_get_tid();
    xfer.cmd_desc[0] =
    CMD_0_ATTR_A |
    CMD_A0_TID(xfer.cmd_tid) |
    CMD_A0_CMD(I3C_CCC_ENTDAA) |
    CMD_A0_DEV_INDEX(dat_idx) |
    CMD_A0_DEV_COUNT(1) |
    CMD_A0_ROC | CMD_A0_TOC;
    xfer.cmd_desc[1] = 0;
    xfer.completion = &done;
    xfer.timeout = HZ;
    ret = i3c_hci_process_xfer(hci, xfer, 1);
    if (ret)
    break;
    if ((RESP_STATUS(xfer.response) == RESP_ERR_ADDR_HEADER ||
    RESP_STATUS(xfer.response) == RESP_ERR_NACK) &&
    RESP_DATA_LENGTH(xfer.response) == 1) {
    ret = 0;  /* no more devices to be assigned */
    break;
    }
    if (RESP_STATUS(xfer.response) != RESP_SUCCESS) {
    ret = -EIO;
    break;
    }
    i3c_hci_dct_get_val(hci, 0, &pid, &dcr, &bcr);
    dev_dbg(&hci.master.dev,
    "assigned address %#x to device PID=0x%llx DCR=%#x BCR=%#x",
    next_addr, pid, dcr, bcr);
    mipi_i3c_hci_dat_v1.free_entry(hci, dat_idx);
    dat_idx = -1;
//
// TODO: Extend the subsystem layer to allow for registering
// new device and provide BCR/DCR/PID at the same time.
//
    i3c_master_add_i3c_dev_locked(&hci.master, next_addr);
    }
    if (dat_idx >= 0)
    mipi_i3c_hci_dat_v1.free_entry(hci, dat_idx);
    hci_free_xfer(xfer, 1);
    return ret;
    }
    const struct hci_cmd_ops mipi_i3c_hci_cmd_v1 = {
    .prep_ccc		= hci_cmd_v1_prep_ccc,
    .prep_i3c_xfer		= hci_cmd_v1_prep_i3c_xfer,
    .prep_i2c_xfer		= hci_cmd_v1_prep_i2c_xfer,
    .perform_daa		= hci_cmd_v1_daa,
    };
