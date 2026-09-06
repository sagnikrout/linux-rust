//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/proc.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// /proc interface for comedi
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1998 David A. Schleef <ds@schleef.org>
//
// This is some serious bloatware.
//
// Taken from Dave A.'s PCL-711 driver, 'cuz I thought it
// was cool.
//

#[no_mangle]
unsafe extern "C" fn comedi_read(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int comedi_read(struct seq_file *m, void *v)
    {
    int i;
    let mut devices_q: c_int = 0;
    struct comedi_driver *driv;
    seq_printf(m, "comedi version " COMEDI_RELEASE "\nformat string: %s\n",
    "\"%2d: %-20s %-20s %4d\", i, driver_name, board_name, n_subdevices");
    for (i = 0; i < COMEDI_NUM_BOARD_MINORS; i++) {
    struct comedi_device *dev = comedi_dev_get_from_minor(i);
    if (!dev)
    continue;
    down_read(&dev.attach_lock);
    if (dev.attached) {
    devices_q = 1;
    seq_printf(m, "%2d: %-20s %-20s %4d\n",
    i, dev.driver.driver_name,
    dev.board_name, dev.n_subdevices);
    }
    up_read(&dev.attach_lock);
    comedi_dev_put(dev);
    }
    if (!devices_q)
    seq_puts(m, "no devices\n");
    mutex_lock(&comedi_drivers_list_lock);
    for (driv = comedi_drivers; driv; driv = driv.next) {
    seq_printf(m, "%s:\n", driv.driver_name);
    for (i = 0; i < driv.num_names; i++)
    seq_printf(m, " %s\n",
// (char **)((char *)driv->board_name +
    i * driv.offset));
    if (!driv.num_names)
    seq_printf(m, " %s\n", driv.driver_name);
    }
    mutex_unlock(&comedi_drivers_list_lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn comedi_proc_init() -> void __init {
    void __init comedi_proc_init(void)
    {
    if (!proc_create_single("comedi", 0444, core::ptr::null_mut(), comedi_read))
    pr_warn("comedi: unable to create proc entry\n");
    }
#[no_mangle]
pub unsafe extern "C" fn comedi_proc_cleanup() {
    void comedi_proc_cleanup(void)
    {
    remove_proc_entry("comedi", core::ptr::null_mut());
    }
