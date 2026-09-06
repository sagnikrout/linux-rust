//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-picolcd_cir.c
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
// Copyright (C) 2010-2012 by Bruno Prémont <bonbons@linux-vserver.org>
//
// Based on Logitech G13 driver (v0.4)
// Copyright (C) 2009 by Rick L. Vinyard, Jr. <rvinyard@cs.nmsu.edu>
//

    int picolcd_raw_cir(struct picolcd_data *data,
    struct hid_report *report, u8 *raw_data, int size)
    {
    unsigned long flags;
    int i, w, sz;
    let mut rawir: ir_raw_event = {};
// ignore if rc_dev is NULL or status is shunned
    spin_lock_irqsave(&data.lock, flags);
    if (!data.rc_dev || (data.status & PICOLCD_CIR_SHUN)) {
    spin_unlock_irqrestore(&data.lock, flags);
    return 1;
    }
    spin_unlock_irqrestore(&data.lock, flags);
// PicoLCD USB packets contain 16-bit intervals in network order,
// with value negated for pulse. Intervals are in microseconds.
//
// Note: some userspace LIRC code for PicoLCD says negated values
// for space - is it a matter of IR chip? (pulse for my TSOP2236)
//
// In addition, the first interval seems to be around 15000 + base
// interval for non-first report of IR data - thus the quirk below
// to get RC_CODE to understand Sony and JVC remotes I have at hand
//
    sz = size > 0 ? min((int)raw_data[0], size-1) : 0;
    for (i = 0; i+1 < sz; i += 2) {
    w = (raw_data[i] << 8) | (raw_data[i+1]);
    rawir.pulse = !!(w & 0x8000);
    rawir.duration = rawir.pulse ? (65536 - w) : w;
// Quirk!! - see above
    if (i == 0 && rawir.duration > 15000)
    rawir.duration -= 15000;
    ir_raw_event_store(data.rc_dev, &rawir);
    }
    ir_raw_event_handle(data.rc_dev);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn picolcd_cir_open(dev: *mut rc_dev) -> c_int {
    static int picolcd_cir_open(struct rc_dev *dev)
    {
    struct picolcd_data *data = dev.priv;
    unsigned long flags;
    spin_lock_irqsave(&data.lock, flags);
    data.status &= ~PICOLCD_CIR_SHUN;
    spin_unlock_irqrestore(&data.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn picolcd_cir_close(dev: *mut rc_dev) {
    static void picolcd_cir_close(struct rc_dev *dev)
    {
    struct picolcd_data *data = dev.priv;
    unsigned long flags;
    spin_lock_irqsave(&data.lock, flags);
    data.status |= PICOLCD_CIR_SHUN;
    spin_unlock_irqrestore(&data.lock, flags);
    }
// initialize CIR input device
#[no_mangle]
pub unsafe extern "C" fn picolcd_init_cir(data: *mut picolcd_data, report: *mut hid_report) -> c_int {
    int picolcd_init_cir(struct picolcd_data *data, struct hid_report *report)
    {
    struct rc_dev *rdev;
    let mut ret: c_int = 0;
    rdev = rc_allocate_device(RC_DRIVER_IR_RAW);
    if (!rdev)
    return -ENOMEM;
    rdev.priv             = data;
    rdev.allowed_protocols = RC_PROTO_BIT_ALL_IR_DECODER;
    rdev.open             = picolcd_cir_open;
    rdev.close            = picolcd_cir_close;
    rdev.device_name      = data.hdev.name;
    rdev.input_phys       = data.hdev.phys;
    rdev.input_id.bustype = data.hdev.bus;
    rdev.input_id.vendor  = data.hdev.vendor;
    rdev.input_id.product = data.hdev.product;
    rdev.input_id.version = data.hdev.version;
    rdev.dev.parent       = &data.hdev.dev;
    rdev.driver_name      = PICOLCD_NAME;
    rdev.map_name         = RC_MAP_RC6_MCE;
    rdev.timeout          = MS_TO_US(100);
    rdev.rx_resolution    = 1;
    ret = rc_register_device(rdev);
    if (ret)
    goto err;
    data.rc_dev = rdev;
    return 0;
    err:
    rc_free_device(rdev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn picolcd_exit_cir(data: *mut picolcd_data) {
    void picolcd_exit_cir(struct picolcd_data *data)
    {
    struct rc_dev *rdev = data.rc_dev;
    data.rc_dev = core::ptr::null_mut();
    rc_unregister_device(rdev);
    rc_free_device(rdev);
    }
