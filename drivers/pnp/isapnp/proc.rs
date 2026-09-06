//! Automatically rewritten from C to Rust
//! Source: drivers/pnp/isapnp/proc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ISA Plug & Play support
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

    extern struct pnp_protocol isapnp_protocol;
    static struct proc_dir_entry *isapnp_proc_bus_dir = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn isapnp_proc_bus_lseek(file: *mut file, off: loff_t, whence: c_int) -> loff_t {
    static loff_t isapnp_proc_bus_lseek(struct file *file, loff_t off, int whence)
    {
    return fixed_size_llseek(file, off, whence, 256);
    }
    static ssize_t isapnp_proc_bus_read(struct file *file, char __user * buf,
    size_t nbytes, loff_t * ppos)
    {
    struct pnp_dev *dev = pde_data(file_inode(file));
    let mut pos: c_int = *ppos;
    int cnt, size = 256;
    if (pos >= size)
    return 0;
    if (nbytes >= size)
    nbytes = size;
    if (pos + nbytes > size)
    nbytes = size - pos;
    cnt = nbytes;
    if (!access_ok(buf, cnt))
    return -EINVAL;
    isapnp_cfg_begin(dev.card.number, dev.number);
    for (; pos < 256 && cnt > 0; pos++, buf++, cnt--) {
    unsigned char val;
    val = isapnp_read_byte(pos);
    __put_user(val, buf);
    }
    isapnp_cfg_end();
// ppos = pos;
    return nbytes;
    }
    static const struct proc_ops isapnp_proc_bus_proc_ops = {
    .proc_lseek	= isapnp_proc_bus_lseek,
    .proc_read	= isapnp_proc_bus_read,
    };
#[no_mangle]
unsafe extern "C" fn isapnp_proc_attach_device(dev: *mut pnp_dev) -> c_int {
    static int isapnp_proc_attach_device(struct pnp_dev *dev)
    {
    struct pnp_card *bus = dev.card;
    char name[16];
    if (!bus.procdir) {
    sprintf(name, "%02x", bus.number);
    bus.procdir = proc_mkdir(name, isapnp_proc_bus_dir);
    if (!bus.procdir)
    return -ENOMEM;
    }
    sprintf(name, "%02x", dev.number);
    dev.procent = proc_create_data(name, S_IFREG | S_IRUGO, bus.procdir,
    &isapnp_proc_bus_proc_ops, dev);
    if (!dev.procent)
    return -ENOMEM;
    proc_set_size(dev.procent, 256);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn isapnp_proc_init() -> int __init {
    int __init isapnp_proc_init(void)
    {
    struct pnp_dev *dev;
    isapnp_proc_bus_dir = proc_mkdir("bus/isapnp", core::ptr::null_mut());
    protocol_for_each_dev(&isapnp_protocol, dev) {
    isapnp_proc_attach_device(dev);
    }
    return 0;
    }
