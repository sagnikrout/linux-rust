//! Automatically rewritten from C to Rust
//! Source: drivers/input/rmi4/rmi_f21.c
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
// Copyright (c) 2012-2025 Synaptics Incorporated
//

pub const RMI_F21_MAX_SENSORS: c_int = 16;
pub const RMI_F21_MAX_FINGERS: c_int = 16;

    RMI_F21_MAX_FINGERS * 2 + 1)

pub const RMI_F21_FORCEPAD_BUTTON_COUNT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f21_data {
    pub input: *mut input_dev,
    pub key_code: u16,
    pub attn_data_size: c_uint,
    pub attn_data_button_offset: c_uint,
    pub data_reg_size: c_uint,
    pub data_reg_button_offset: c_uint,
    pub data_regs: [u8; RMI_F21_DATA_REGS_MAX_SIZE],
}

#[no_mangle]
unsafe extern "C" fn rmi_f21_attention(irq: c_int, ctx: *mut c_void) -> irqreturn_t {
    static irqreturn_t rmi_f21_attention(int irq, void *ctx)
    {
    struct rmi_function *fn = ctx;
    struct f21_data *f21 = dev_get_drvdata(&fn.dev);
    struct rmi_driver_data *drvdata = dev_get_drvdata(&fn.rmi_dev.dev);
    u8 *pdata;
    int error;
    bool pressed;
    if (drvdata.attn_data.data) {
    if (drvdata.attn_data.size < f21.attn_data_size) {
    dev_warn(&fn.dev, "f21 interrupt, but data is missing\n");
    return IRQ_HANDLED;
    }
    pdata = drvdata.attn_data.data + f21.attn_data_button_offset;
    drvdata.attn_data.data += f21.attn_data_size;
    drvdata.attn_data.size -= f21.attn_data_size;
    } else {
    error = rmi_read_block(fn.rmi_dev, fn.fd.data_base_addr,
    f21.data_regs, f21.data_reg_size);
    if (error) {
    dev_err(&fn.dev, "failed to read f21 data registers: %d\n",
    error);
    return IRQ_RETVAL(error);
    }
    pdata = f21.data_regs + f21.data_reg_button_offset;
    }
    pressed = *pdata & RMI_F21_FORCE_CLICK_BIT;
    input_report_key(f21.input, f21.key_code, pressed);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f21_config(fn: *mut rmi_function) -> c_int {
    static int rmi_f21_config(struct rmi_function *fn)
    {
    struct rmi_driver *drv = fn.rmi_dev.driver;
    drv.set_irq_bits(fn.rmi_dev, fn.irq_mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f21_initialize(fn: *mut rmi_function, f21: *mut f21_data) -> c_int {
    static int rmi_f21_initialize(struct rmi_function *fn, struct f21_data *f21)
    {
    struct input_dev *input = f21.input;
    f21.key_code = BTN_LEFT;
    input.keycode = &f21.key_code;
    input.keycodesize = sizeof(f21.key_code);
    input.keycodemax = RMI_F21_FORCEPAD_BUTTON_COUNT;
    input_set_capability(input, EV_KEY, f21.key_code);
    __set_bit(INPUT_PROP_BUTTONPAD, input.propbit);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f21_probe(fn: *mut rmi_function) -> c_int {
    static int rmi_f21_probe(struct rmi_function *fn)
    {
    struct rmi_device *rmi_dev = fn.rmi_dev;
    struct rmi_driver_data *drv_data = dev_get_drvdata(&rmi_dev.dev);
    struct f21_data *f21;
    unsigned int sensor_count;
    unsigned int max_fingers;
    unsigned int query15_offset;
    u8 query15_data;
    int error;
    if (!drv_data.input) {
    dev_info(&fn.dev, "f21: no input device found, ignoring\n");
    return -ENXIO;
    }
    f21 = devm_kzalloc(&fn.dev, sizeof(*f21), GFP_KERNEL);
    if (!f21)
    return -ENOMEM;
    f21.input = drv_data.input;
    error = rmi_f21_initialize(fn, f21);
    if (error)
    return error;
    dev_set_drvdata(&fn.dev, f21);
    sensor_count = fn.fd.query_base_addr & RMI_F21_SENSOR_COUNT_MASK;
    if (fn.fd.query_base_addr & RMI_F21_FINGER_COUNT_PRESENT) {
    query15_offset = fn.fd.query_base_addr & RMI_F21_NEW_REPORT_FORMAT ? 2 : 1;
    error = rmi_read_block(fn.rmi_dev,
    fn.fd.query_base_addr + query15_offset,
    &query15_data, sizeof(query15_data));
    if (error)
    return dev_err_probe(&fn.dev, error,
    "failed to read 'query15' data");
    max_fingers = query15_data & RMI_F21_FINGER_COUNT_MASK;
    } else {
    max_fingers = 5;
    }
    if (fn.fd.query_base_addr & RMI_F21_NEW_REPORT_FORMAT) {
// Each finger uses one byte, and the button state uses one byte.
    f21.attn_data_size = max_fingers + 1;
    f21.attn_data_button_offset = f21.attn_data_size - 1;
//
// Each sensor uses two bytes, the button state uses one byte,
// and each finger uses two bytes.
//
    f21.data_reg_size = sensor_count * 2 + 1 + max_fingers * 2;
    f21.data_reg_button_offset = sensor_count * 2;
    } else {
//
// Regardless of the transport each finger uses two bytes,
// and the button state uses one byte.
//
    f21.attn_data_size = sensor_count * 2 + 1;
    f21.attn_data_button_offset = sensor_count * 2;
    f21.data_reg_size = f21.attn_data_size;
    f21.data_reg_button_offset = f21.attn_data_button_offset;
    }
    return 0;
    }
    struct rmi_function_handler rmi_f21_handler = {
    .driver = {
    .name = "rmi4_f21",
    },
    .func = 0x21,
    .probe = rmi_f21_probe,
    .config = rmi_f21_config,
    .attention = rmi_f21_attention,
    };
