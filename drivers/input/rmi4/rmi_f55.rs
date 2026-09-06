//! Automatically rewritten from C to Rust
//! Source: drivers/input/rmi4/rmi_f55.c
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
// Copyright (c) 2012-2015 Synaptics Incorporated
// Copyright (C) 2016 Zodiac Inflight Innovations
//

// F55 data offsets
pub const F55_NUM_RX_OFFSET: c_int = 0;
pub const F55_NUM_TX_OFFSET: c_int = 1;
pub const F55_PHYS_CHAR_OFFSET: c_int = 2;
// Only read required query registers
pub const F55_QUERY_LEN: c_int = 3;
// F55 capabilities

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f55_data {
    pub fn: *mut rmi_function,
    pub qry: [u8; F55_QUERY_LEN],
    pub num_rx_electrodes: u8,
    pub cfg_num_rx_electrodes: u8,
    pub num_tx_electrodes: u8,
    pub cfg_num_tx_electrodes: u8,
}

#[no_mangle]
unsafe extern "C" fn rmi_f55_detect(fn: *mut rmi_function) -> c_int {
    static int rmi_f55_detect(struct rmi_function *fn)
    {
    struct rmi_device *rmi_dev = fn.rmi_dev;
    struct rmi_driver_data *drv_data = dev_get_drvdata(&rmi_dev.dev);
    struct f55_data *f55;
    int error;
    f55 = dev_get_drvdata(&fn.dev);
    error = rmi_read_block(fn.rmi_dev, fn.fd.query_base_addr,
    &f55.qry, sizeof(f55.qry));
    if (error) {
    dev_err(&fn.dev, "%s: Failed to query F55 properties\n",
    __func__);
    return error;
    }
    f55.num_rx_electrodes = f55.qry[F55_NUM_RX_OFFSET];
    f55.num_tx_electrodes = f55.qry[F55_NUM_TX_OFFSET];
    f55.cfg_num_rx_electrodes = f55.num_rx_electrodes;
    f55.cfg_num_tx_electrodes = f55.num_tx_electrodes;
    drv_data.num_rx_electrodes = f55.cfg_num_rx_electrodes;
    drv_data.num_tx_electrodes = f55.cfg_num_tx_electrodes;
    if (f55.qry[F55_PHYS_CHAR_OFFSET] & F55_CAP_SENSOR_ASSIGN) {
    int i, total;
    u8 buf[256];
//
// Calculate the number of enabled receive and transmit
// electrodes by reading F55:Ctrl1 (sensor receiver assignment)
// and F55:Ctrl2 (sensor transmitter assignment). The number of
// enabled electrodes is the sum of all field entries with a
// value other than 0xff.
//
    error = rmi_read_block(fn.rmi_dev,
    fn.fd.control_base_addr + 1,
    buf, f55.num_rx_electrodes);
    if (!error) {
    total = 0;
    for (i = 0; i < f55.num_rx_electrodes; i++) {
    if (buf[i] != 0xff)
    total++;
    }
    f55.cfg_num_rx_electrodes = total;
    drv_data.num_rx_electrodes = total;
    }
    error = rmi_read_block(fn.rmi_dev,
    fn.fd.control_base_addr + 2,
    buf, f55.num_tx_electrodes);
    if (!error) {
    total = 0;
    for (i = 0; i < f55.num_tx_electrodes; i++) {
    if (buf[i] != 0xff)
    total++;
    }
    f55.cfg_num_tx_electrodes = total;
    drv_data.num_tx_electrodes = total;
    }
    }
    rmi_dbg(RMI_DEBUG_FN, &fn.dev, "F55 num_rx_electrodes: %d (raw %d)\n",
    f55.cfg_num_rx_electrodes, f55.num_rx_electrodes);
    rmi_dbg(RMI_DEBUG_FN, &fn.dev, "F55 num_tx_electrodes: %d (raw %d)\n",
    f55.cfg_num_tx_electrodes, f55.num_tx_electrodes);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f55_probe(fn: *mut rmi_function) -> c_int {
    static int rmi_f55_probe(struct rmi_function *fn)
    {
    struct f55_data *f55;
    f55 = devm_kzalloc(&fn.dev, sizeof(struct f55_data), GFP_KERNEL);
    if (!f55)
    return -ENOMEM;
    f55.fn = fn;
    dev_set_drvdata(&fn.dev, f55);
    return rmi_f55_detect(fn);
    }
    struct rmi_function_handler rmi_f55_handler = {
    .driver = {
    .name = F55_NAME,
    },
    .func = 0x55,
    .probe = rmi_f55_probe,
    };
