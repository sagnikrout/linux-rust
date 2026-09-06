//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/cx231xx/cx231xx-input.c
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
// cx231xx IR glue driver
//
// Copyright (c) 2010 Mauro Carvalho Chehab <mchehab@kernel.org>
//
// Polaris (cx231xx) has its support for IR's with a design close to MCE.
// however, a few designs are using an external I2C chip for IR, instead
// of using the one provided by the chip.
// This driver provides support for those extra devices

    static int get_key_isdbt(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *pscancode, u8 *toggle)
    {
    int	rc;
    u8	cmd, scancode;
    dev_dbg(&ir.rc.dev, "%s\n", __func__);
// poll IR chip
    rc = i2c_master_recv(ir.c, &cmd, 1);
    if (rc < 0)
    return rc;
    if (rc != 1)
    return -EIO;
// it seems that 0xFE indicates that a button is still hold
    down, while 0xff indicates that no button is hold
    down. 0xfe sequences are sometimes interrupted by 0xFF */
    if (cmd == 0xff)
    return 0;
    scancode = bitrev8(cmd);
    dev_dbg(&ir.rc.dev, "cmd %02x, scan = %02x\n", cmd, scancode);
// protocol = RC_PROTO_OTHER;
// pscancode = scancode;
// toggle = 0;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn cx231xx_ir_init(dev: *mut cx231xx) -> c_int {
    int cx231xx_ir_init(struct cx231xx *dev)
    {
    struct i2c_board_info info;
    u8 ir_i2c_bus;
    dev_dbg(dev.dev, "%s\n", __func__);
// Only initialize if a rc keycode map is defined
    if (!cx231xx_boards[dev.model].rc_map_name)
    return -ENODEV;
    request_module("ir-kbd-i2c");
    memset(&info, 0, sizeof(struct i2c_board_info));
    memset(&dev.init_data, 0, sizeof(dev.init_data));
    dev.init_data.rc_dev = rc_allocate_device(RC_DRIVER_SCANCODE);
    if (!dev.init_data.rc_dev)
    return -ENOMEM;
    dev.init_data.name = cx231xx_boards[dev.model].name;
    strscpy(info.type, "ir_video", I2C_NAME_SIZE);
    info.platform_data = &dev.init_data;
//
// Board-dependent values
//
// For now, there's just one type of hardware design using
// an i2c device.
//
    dev.init_data.get_key = get_key_isdbt;
    dev.init_data.ir_codes = cx231xx_boards[dev.model].rc_map_name;
// The i2c micro-controller only outputs the cmd part of NEC protocol
    dev.init_data.rc_dev.scancode_mask = 0xff;
    dev.init_data.rc_dev.driver_name = "cx231xx";
    dev.init_data.type = RC_PROTO_BIT_NEC;
    info.addr = 0x30;
// Load and bind ir-kbd-i2c
    ir_i2c_bus = cx231xx_boards[dev.model].ir_i2c_master;
    dev_dbg(dev.dev, "Trying to bind ir at bus %d, addr 0x%02x\n",
    ir_i2c_bus, info.addr);
    dev.ir_i2c_client = i2c_new_client_device(
    cx231xx_get_i2c_adap(dev, ir_i2c_bus), &info);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cx231xx_ir_exit(dev: *mut cx231xx) {
    void cx231xx_ir_exit(struct cx231xx *dev)
    {
    i2c_unregister_device(dev.ir_i2c_client);
    dev.ir_i2c_client = core::ptr::null_mut();
    }
