//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/monwriter.c
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
// Character device driver for writing z/VM *MONITOR service records.
//
// Copyright IBM Corp. 2006, 2009
//
// Author(s): Melissa Howland <Melissa.Howland@us.ibm.com>
//

pub const MONWRITE_MAX_DATALEN: c_int = 4010;
    let mut mon_max_bufs: static int = 255;
    static int mon_buf_count;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mon_buf {
    pub list: list_head,
    pub hdr: monwrite_hdr,
    pub diag_done: c_int,
    pub data: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mon_private {
    pub list: list_head,
    pub hdr: monwrite_hdr,
    pub hdr_to_read: usize,
    pub data_to_read: usize,
    pub current_buf: *mut mon_buf,
    pub thread_mutex: mutex,
}

//
// helper functions
//
#[no_mangle]
unsafe extern "C" fn monwrite_diag(myhdr: *mut monwrite_hdr, buffer: *mut c_char, fcn: c_int) -> c_int {
    static int monwrite_diag(struct monwrite_hdr *myhdr, char *buffer, int fcn)
    {
    struct appldata_parameter_list *parm_list;
    struct appldata_product_id *id;
    int rc;
    id = kmalloc_obj(*id);
    parm_list = kmalloc_obj(*parm_list);
    rc = -ENOMEM;
    if (!id || !parm_list)
    goto out;
    memcpy(id.prod_nr, "LNXAPPL", 7);
    id.prod_fn = myhdr.applid;
    id.record_nr = myhdr.record_num;
    id.version_nr = myhdr.version;
    id.release_nr = myhdr.release;
    id.mod_lvl = myhdr.mod_level;
    rc = appldata_asm(parm_list, id, fcn,
    (void *) buffer, myhdr.datalen);
    if (rc <= 0)
    goto out;
    pr_err("Writing monitor data failed with rc=%i\n", rc);
    rc = (rc == 5) ? -EPERM : -EINVAL;
    out:
    kfree(id);
    kfree(parm_list);
    return rc;
    }
    static struct mon_buf *monwrite_find_hdr(struct mon_private *monpriv,
    struct monwrite_hdr *monhdr)
    {
    struct mon_buf *entry, *next;
    list_for_each_entry_safe(entry, next, &monpriv.list, list)
    if ((entry.hdr.mon_function == monhdr.mon_function ||
    monhdr.mon_function == MONWRITE_STOP_INTERVAL) &&
    entry.hdr.applid == monhdr.applid &&
    entry.hdr.record_num == monhdr.record_num &&
    entry.hdr.version == monhdr.version &&
    entry.hdr.release == monhdr.release &&
    entry.hdr.mod_level == monhdr.mod_level)
    return entry;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn monwrite_new_hdr(monpriv: *mut mon_private) -> c_int {
    static int monwrite_new_hdr(struct mon_private *monpriv)
    {
    struct monwrite_hdr *monhdr = &monpriv.hdr;
    struct mon_buf *monbuf;
    let mut rc: c_int = 0;
    if (monhdr.datalen > MONWRITE_MAX_DATALEN ||
    monhdr.mon_function > MONWRITE_START_CONFIG ||
    monhdr.hdrlen != sizeof(struct monwrite_hdr))
    return -EINVAL;
    monbuf = core::ptr::null_mut();
    if (monhdr.mon_function != MONWRITE_GEN_EVENT)
    monbuf = monwrite_find_hdr(monpriv, monhdr);
    if (monbuf) {
    if (monhdr.mon_function == MONWRITE_STOP_INTERVAL) {
    monhdr.datalen = monbuf.hdr.datalen;
    rc = monwrite_diag(monhdr, monbuf.data,
    APPLDATA_STOP_REC);
    list_del(&monbuf.list);
    mon_buf_count--;
    kfree(monbuf.data);
    kfree(monbuf);
    monbuf = core::ptr::null_mut();
    } else if (monbuf.hdr.datalen != monhdr.datalen) {
// Data with buffer reuse must not change its length
    return -EINVAL;
    }
    } else if (monhdr.mon_function != MONWRITE_STOP_INTERVAL) {
    if (mon_buf_count >= mon_max_bufs)
    return -ENOSPC;
    monbuf = kzalloc_obj(struct mon_buf);
    if (!monbuf)
    return -ENOMEM;
    monbuf.data = kzalloc(monhdr.datalen,
    GFP_KERNEL | GFP_DMA);
    if (!monbuf.data) {
    kfree(monbuf);
    return -ENOMEM;
    }
    monbuf.hdr = *monhdr;
    list_add_tail(&monbuf.list, &monpriv.list);
    if (monhdr.mon_function != MONWRITE_GEN_EVENT)
    mon_buf_count++;
    }
    monpriv.current_buf = monbuf;
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn monwrite_new_data(monpriv: *mut mon_private) -> c_int {
    static int monwrite_new_data(struct mon_private *monpriv)
    {
    struct monwrite_hdr *monhdr = &monpriv.hdr;
    struct mon_buf *monbuf = monpriv.current_buf;
    let mut rc: c_int = 0;
    switch (monhdr.mon_function) {
    case MONWRITE_START_INTERVAL:
    if (!monbuf.diag_done) {
    rc = monwrite_diag(monhdr, monbuf.data,
    APPLDATA_START_INTERVAL_REC);
    monbuf.diag_done = 1;
    }
    break;
    case MONWRITE_START_CONFIG:
    if (!monbuf.diag_done) {
    rc = monwrite_diag(monhdr, monbuf.data,
    APPLDATA_START_CONFIG_REC);
    monbuf.diag_done = 1;
    }
    break;
    case MONWRITE_GEN_EVENT:
    rc = monwrite_diag(monhdr, monbuf.data,
    APPLDATA_GEN_EVENT_REC);
    list_del(&monpriv.current_buf.list);
    kfree(monpriv.current_buf.data);
    kfree(monpriv.current_buf);
    monpriv.current_buf = core::ptr::null_mut();
    break;
    default:
// monhdr->mon_function is checked in monwrite_new_hdr
    BUG();
    }
    return rc;
    }
//
// file operations
//
#[no_mangle]
unsafe extern "C" fn monwrite_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int monwrite_open(struct inode *inode, struct file *filp)
    {
    struct mon_private *monpriv;
    monpriv = kzalloc_obj(struct mon_private);
    if (!monpriv)
    return -ENOMEM;
    INIT_LIST_HEAD(&monpriv.list);
    monpriv.hdr_to_read = sizeof(monpriv.hdr);
    mutex_init(&monpriv.thread_mutex);
    filp.private_data = monpriv;
    return nonseekable_open(inode, filp);
    }
#[no_mangle]
unsafe extern "C" fn monwrite_close(inode: *mut inode, filp: *mut file) -> c_int {
    static int monwrite_close(struct inode *inode, struct file *filp)
    {
    struct mon_private *monpriv = filp.private_data;
    struct mon_buf *entry, *next;
    list_for_each_entry_safe(entry, next, &monpriv.list, list) {
    if (entry.hdr.mon_function != MONWRITE_GEN_EVENT)
    monwrite_diag(&entry.hdr, entry.data,
    APPLDATA_STOP_REC);
    mon_buf_count--;
    list_del(&entry.list);
    kfree(entry.data);
    kfree(entry);
    }
    kfree(monpriv);
    return 0;
    }
    static ssize_t monwrite_write(struct file *filp, const char __user *data,
    size_t count, loff_t *ppos)
    {
    struct mon_private *monpriv = filp.private_data;
    size_t len, written;
    void *to;
    int rc;
    mutex_lock(&monpriv.thread_mutex);
    for (written = 0; written < count; ) {
    if (monpriv.hdr_to_read) {
    len = min(count - written, monpriv.hdr_to_read);
    to = (char *) &monpriv.hdr +
    sizeof(monpriv.hdr) - monpriv.hdr_to_read;
    if (copy_from_user(to, data + written, len)) {
    rc = -EFAULT;
    goto out_error;
    }
    monpriv.hdr_to_read -= len;
    written += len;
    if (monpriv.hdr_to_read > 0)
    continue;
    rc = monwrite_new_hdr(monpriv);
    if (rc)
    goto out_error;
    monpriv.data_to_read = monpriv.current_buf ?
    monpriv.current_buf.hdr.datalen : 0;
    }
    if (monpriv.data_to_read) {
    len = min(count - written, monpriv.data_to_read);
    to = monpriv.current_buf.data +
    monpriv.hdr.datalen - monpriv.data_to_read;
    if (copy_from_user(to, data + written, len)) {
    rc = -EFAULT;
    goto out_error;
    }
    monpriv.data_to_read -= len;
    written += len;
    if (monpriv.data_to_read > 0)
    continue;
    rc = monwrite_new_data(monpriv);
    if (rc)
    goto out_error;
    }
    monpriv.hdr_to_read = sizeof(monpriv.hdr);
    }
    mutex_unlock(&monpriv.thread_mutex);
    return written;
    out_error:
    monpriv.data_to_read = 0;
    monpriv.hdr_to_read = sizeof(struct monwrite_hdr);
    mutex_unlock(&monpriv.thread_mutex);
    return rc;
    }
    static const struct file_operations monwrite_fops = {
    .owner	 = THIS_MODULE,
    .open	 = &monwrite_open,
    .release = &monwrite_close,
    .write	 = &monwrite_write,
    .llseek  = noop_llseek,
    };
    static struct miscdevice mon_dev = {
    .name	= "monwriter",
    .fops	= &monwrite_fops,
    .minor	= MISC_DYNAMIC_MINOR,
    };
//
// module init/exit
//
#[no_mangle]
unsafe extern "C" fn mon_init() -> int __init {
    static int __init mon_init(void)
    {
    if (!machine_is_vm())
    return -ENODEV;
//
// misc_register() has to be the last action in module_init(), because
// file operations will be available right after this.
//
    return misc_register(&mon_dev);
    }
#[no_mangle]
unsafe extern "C" fn mon_exit() -> void __exit {
    static void __exit mon_exit(void)
    {
    misc_deregister(&mon_dev);
    }
    module_init(mon_init);
    module_exit(mon_exit);
    module_param_named(max_bufs, mon_max_bufs, int, 0644);
    MODULE_PARM_DESC(max_bufs, "Maximum number of sample monitor data buffers "
    "that can be active at one time");
    MODULE_AUTHOR("Melissa Howland <Melissa.Howland@us.ibm.com>");
    MODULE_DESCRIPTION("Character device driver for writing z/VM "
    "APPLDATA monitor records.");
    MODULE_LICENSE("GPL");
