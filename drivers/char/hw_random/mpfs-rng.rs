//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/mpfs-rng.c
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
// Microchip PolarFire SoC (MPFS) hardware random driver
//
// Copyright (c) 2020-2022 Microchip Corporation. All rights reserved.
//
// Author: Conor Dooley <conor.dooley@microchip.com>
//

pub const CMD_OPCODE: c_uint = 0x21;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_rng {
    pub sys_controller: *mut mpfs_sys_controller,
    pub rng: hwrng,
}

#[no_mangle]
unsafe extern "C" fn mpfs_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int mpfs_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct mpfs_rng *rng_priv = container_of(rng, struct mpfs_rng, rng);
    u32 response_msg[RNG_RESP_BYTES / sizeof(u32)];
    let mut count: c_uint = 0, copy_size_bytes;
    int ret;
    struct mpfs_mss_response response = {
    .resp_status = 0U,
    .resp_msg = (u32 *)response_msg,
    .resp_size = RNG_RESP_BYTES
    };
    struct mpfs_mss_msg msg = {
    .cmd_opcode = CMD_OPCODE,
    .cmd_data_size = CMD_DATA_SIZE,
    .response = &response,
    .cmd_data = CMD_DATA,
    .mbox_offset = MBOX_OFFSET,
    .resp_offset = RESP_OFFSET
    };
    while (count < max) {
    ret = mpfs_blocking_transaction(rng_priv.sys_controller, &msg);
    if (ret)
    return ret;
    copy_size_bytes = max - count > RNG_RESP_BYTES ? RNG_RESP_BYTES : max - count;
    memcpy(buf + count, response_msg, copy_size_bytes);
    count += copy_size_bytes;
    if (!wait)
    break;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_rng_probe(pdev: *mut platform_device) -> c_int {
    static int mpfs_rng_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mpfs_rng *rng_priv;
    int ret;
    rng_priv = devm_kzalloc(dev, sizeof(*rng_priv), GFP_KERNEL);
    if (!rng_priv)
    return -ENOMEM;
    rng_priv.sys_controller =  mpfs_sys_controller_get(&pdev.dev);
    if (IS_ERR(rng_priv.sys_controller))
    return dev_err_probe(dev, PTR_ERR(rng_priv.sys_controller),
    "Failed to register system controller hwrng sub device\n");
    rng_priv.rng.read = mpfs_rng_read;
    rng_priv.rng.name = pdev.name;
    ret = devm_hwrng_register(&pdev.dev, &rng_priv.rng);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to register MPFS hwrng\n");
    dev_info(&pdev.dev, "Registered MPFS hwrng\n");
    return 0;
    }
    static struct platform_driver mpfs_rng_driver = {
    .driver = {
    .name = "mpfs-rng",
    },
    .probe = mpfs_rng_probe,
    };
    module_platform_driver(mpfs_rng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Conor Dooley <conor.dooley@microchip.com>");
    MODULE_DESCRIPTION("PolarFire SoC (MPFS) hardware random driver");
