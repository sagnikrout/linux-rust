//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xenfs/xensyms.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensyms {
    pub op: xen_platform_op,
    pub name: *mut c_char,
    pub namelen: u32,
}

// Grab next output page from the hypervisor
#[no_mangle]
unsafe extern "C" fn xensyms_next_sym(xs: *mut xensyms) -> c_int {
    static int xensyms_next_sym(struct xensyms *xs)
    {
    int ret;
    struct xenpf_symdata *symdata = &xs.op.u.symdata;
    uint64_t symnum;
    memset(xs.name, 0, xs.namelen);
    symdata.namelen = xs.namelen;
    symnum = symdata.symnum;
    ret = HYPERVISOR_platform_op(&xs.op);
    if (ret < 0)
    return ret;
//
// If hypervisor's symbol didn't fit into the buffer then allocate
// a larger buffer and try again.
//
    if (unlikely(symdata.namelen > xs.namelen)) {
    kfree(xs.name);
    xs.namelen = symdata.namelen;
    xs.name = kzalloc(xs.namelen, GFP_KERNEL);
    if (!xs.name)
    return -ENOMEM;
    set_xen_guest_handle(symdata.name, xs.name);
    symdata.symnum = symnum; /* Rewind */
    ret = HYPERVISOR_platform_op(&xs.op);
    if (ret < 0)
    return ret;
    }
    if (symdata.symnum == symnum)
// End of symbols
    return 1;
    return 0;
    }
    static void *xensyms_start(struct seq_file *m, loff_t *pos)
    {
    struct xensyms *xs = m.private;
    xs.op.u.symdata.symnum = *pos;
    if (xensyms_next_sym(xs))
    return core::ptr::null_mut();
    return m.private;
    }
    static void *xensyms_next(struct seq_file *m, void *p, loff_t *pos)
    {
    struct xensyms *xs = m.private;
// pos = xs->op.u.symdata.symnum;
    if (xensyms_next_sym(xs))
    return core::ptr::null_mut();
    return p;
    }
#[no_mangle]
unsafe extern "C" fn xensyms_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int xensyms_show(struct seq_file *m, void *p)
    {
    struct xensyms *xs = m.private;
    struct xenpf_symdata *symdata = &xs.op.u.symdata;
    seq_printf(m, "%016llx %c %s\n", symdata.address,
    symdata.type, xs.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xensyms_stop(m: *mut seq_file, p: *mut c_void) {
    static void xensyms_stop(struct seq_file *m, void *p)
    {
    }
    static const struct seq_operations xensyms_seq_ops = {
    .start = xensyms_start,
    .next = xensyms_next,
    .show = xensyms_show,
    .stop = xensyms_stop,
    };
#[no_mangle]
unsafe extern "C" fn xensyms_open(inode: *mut inode, file: *mut file) -> c_int {
    static int xensyms_open(struct inode *inode, struct file *file)
    {
    struct seq_file *m;
    struct xensyms *xs;
    int ret;
    ret = seq_open_private(file, &xensyms_seq_ops,
    sizeof(struct xensyms));
    if (ret)
    return ret;
    m = file.private_data;
    xs = m.private;
    xs.namelen = XEN_KSYM_NAME_LEN + 1;
    xs.name = kzalloc(xs.namelen, GFP_KERNEL);
    if (!xs.name) {
    seq_release_private(inode, file);
    return -ENOMEM;
    }
    set_xen_guest_handle(xs.op.u.symdata.name, xs.name);
    xs.op.cmd = XENPF_get_symbol;
    xs.op.u.symdata.namelen = xs.namelen;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xensyms_release(inode: *mut inode, file: *mut file) -> c_int {
    static int xensyms_release(struct inode *inode, struct file *file)
    {
    struct seq_file *m = file.private_data;
    struct xensyms *xs = m.private;
    kfree(xs.name);
    return seq_release_private(inode, file);
    }
    const struct file_operations xensyms_ops = {
    .open = xensyms_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .release = xensyms_release
    };
