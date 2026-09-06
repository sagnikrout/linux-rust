//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xenbus/xenbus_dev_backend.c
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

#[no_mangle]
unsafe extern "C" fn xenbus_backend_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int xenbus_backend_open(struct inode *inode, struct file *filp)
    {
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    return nonseekable_open(inode, filp);
    }
#[no_mangle]
unsafe extern "C" fn xenbus_alloc(domid: domid_t) -> c_long {
    static long xenbus_alloc(domid_t domid)
    {
    struct evtchn_alloc_unbound arg;
    let mut err: c_int = -EEXIST;
    xs_suspend();
// If xenstored_ready is nonzero, that means we have already talked to
// xenstore and set up watches. These watches will be restored by
// xs_resume, but that requires communication over the port established
// below that is not visible to anyone until the ioctl returns.
//
// This can be resolved by splitting the ioctl into two parts
// (postponing the resume until xenstored is active) but this is
// unnecessarily complex for the intended use where xenstored is only
// started once - so return -EEXIST if it's already running.
//
    if (xenstored_ready)
    goto out_err;
    gnttab_grant_foreign_access_ref(GNTTAB_RESERVED_XENSTORE, domid,
    virt_to_gfn(xen_store_interface), 0 /* writable */);
    arg.dom = DOMID_SELF;
    arg.remote_dom = domid;
    err = HYPERVISOR_event_channel_op(EVTCHNOP_alloc_unbound, &arg);
    if (err)
    goto out_err;
    if (xen_store_evtchn > 0)
    xb_deinit_comms();
    xen_store_evtchn = arg.port;
    xs_resume();
    return arg.port;
    out_err:
    xs_suspend_cancel();
    return err;
    }
    static long xenbus_backend_ioctl(struct file *file, unsigned int cmd,
    unsigned long data)
    {
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    switch (cmd) {
    case IOCTL_XENBUS_BACKEND_EVTCHN:
    if (xen_store_evtchn > 0)
    return xen_store_evtchn;
    return -ENODEV;
    case IOCTL_XENBUS_BACKEND_SETUP:
    return xenbus_alloc(data);
    default:
    return -ENOTTY;
    }
    }
#[no_mangle]
unsafe extern "C" fn xenbus_backend_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int xenbus_backend_mmap(struct file *file, struct vm_area_struct *vma)
    {
    let mut size: usize = vma.vm_end - vma.vm_start;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    if ((size > PAGE_SIZE) || (vma.vm_pgoff != 0))
    return -EINVAL;
    if (remap_pfn_range(vma, vma.vm_start,
    virt_to_pfn(xen_store_interface),
    size, vma.vm_page_prot))
    return -EAGAIN;
    return 0;
    }
    static const struct file_operations xenbus_backend_fops = {
    .open = xenbus_backend_open,
    .mmap = xenbus_backend_mmap,
    .unlocked_ioctl = xenbus_backend_ioctl,
    };
    static struct miscdevice xenbus_backend_dev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "xen/xenbus_backend",
    .fops = &xenbus_backend_fops,
    };
#[no_mangle]
unsafe extern "C" fn xenbus_backend_init() -> int __init {
    static int __init xenbus_backend_init(void)
    {
    int err;
    if (!xen_initial_domain())
    return -ENODEV;
    err = misc_register(&xenbus_backend_dev);
    if (err)
    pr_err("Could not register xenbus backend device\n");
    return err;
    }
    device_initcall(xenbus_backend_init);
