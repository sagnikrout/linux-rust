//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/vmcp.c
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
// Copyright IBM Corp. 2004, 2010
// Interface implementation for communication with the z/VM control program
//
// Author(s): Christian Borntraeger <borntraeger@de.ibm.com>
//
// z/VMs CP offers the possibility to issue commands via the diagnose code 8
// this driver implements a character device that issues these commands and
// returns the answer of CP.
//
// The idea of this driver is based on cpint from Neale Ferguson and #CP in CMS
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcp_session {
    pub response: *mut c_char,
    pub bufsize: c_uint,
    pub 1: unsigned int cma_alloc :,
    pub resp_size: c_int,
    pub resp_code: c_int,
    pub mutex: mutex,
}

    static debug_info_t *vmcp_debug;
    let mut __initdata: static unsigned long vmcp_cma_size = CONFIG_VMCP_CMA_SIZE * 1024 * 1024;
    static struct cma *vmcp_cma;
#[no_mangle]
unsafe extern "C" fn early_parse_vmcp_cma(p: *mut c_char) -> int __init {
    static int __init early_parse_vmcp_cma(char *p)
    {
    if (!p)
    return 1;
    vmcp_cma_size = ALIGN(memparse(p, core::ptr::null_mut()), PAGE_SIZE);
    return 0;
    }
    early_param("vmcp_cma", early_parse_vmcp_cma);
#[no_mangle]
pub unsafe extern "C" fn vmcp_cma_reserve() -> void __init {
    void __init vmcp_cma_reserve(void)
    {
    if (!machine_is_vm())
    return;
    cma_declare_contiguous(0, vmcp_cma_size, 0, 0, 0, false, "vmcp", &vmcp_cma);
    }
#[no_mangle]
unsafe extern "C" fn vmcp_response_alloc(session: *mut vmcp_session) {
    static void vmcp_response_alloc(struct vmcp_session *session)
    {
    struct page *page = core::ptr::null_mut();
    int nr_pages, order;
    order = get_order(session.bufsize);
    nr_pages = ALIGN(session.bufsize, PAGE_SIZE) >> PAGE_SHIFT;
//
// For anything below order 3 allocations rely on the buddy
// allocator. If such low-order allocations can't be handled
// anymore the system won't work anyway.
//
    if (order > 2)
    page = cma_alloc(vmcp_cma, nr_pages, 0, false);
    if (page) {
    session.response = (char *)page_to_virt(page);
    session.cma_alloc = 1;
    return;
    }
    session.response = (char *)__get_free_pages(GFP_KERNEL | __GFP_RETRY_MAYFAIL, order);
    }
#[no_mangle]
unsafe extern "C" fn vmcp_response_free(session: *mut vmcp_session) {
    static void vmcp_response_free(struct vmcp_session *session)
    {
    int nr_pages, order;
    struct page *page;
    if (!session.response)
    return;
    order = get_order(session.bufsize);
    nr_pages = ALIGN(session.bufsize, PAGE_SIZE) >> PAGE_SHIFT;
    if (session.cma_alloc) {
    page = virt_to_page(session.response);
    cma_release(vmcp_cma, page, nr_pages);
    session.cma_alloc = 0;
    } else {
    free_pages((unsigned long)session.response, order);
    }
    session.response = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn vmcp_open(inode: *mut inode, file: *mut file) -> c_int {
    static int vmcp_open(struct inode *inode, struct file *file)
    {
    struct vmcp_session *session;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    session = kmalloc_obj(*session);
    if (!session)
    return -ENOMEM;
    session.bufsize = PAGE_SIZE;
    session.response = core::ptr::null_mut();
    session.resp_size = 0;
    mutex_init(&session.mutex);
    file.private_data = session;
    return nonseekable_open(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn vmcp_release(inode: *mut inode, file: *mut file) -> c_int {
    static int vmcp_release(struct inode *inode, struct file *file)
    {
    struct vmcp_session *session;
    session = file.private_data;
    file.private_data = core::ptr::null_mut();
    vmcp_response_free(session);
    kfree(session);
    return 0;
    }
    static ssize_t
    vmcp_read(struct file *file, char __user *buff, size_t count, loff_t *ppos)
    {
    ssize_t ret;
    size_t size;
    struct vmcp_session *session;
    session = file.private_data;
    if (mutex_lock_interruptible(&session.mutex))
    return -ERESTARTSYS;
    if (!session.response) {
    mutex_unlock(&session.mutex);
    return 0;
    }
    size = min_t(size_t, session.resp_size, session.bufsize);
    ret = simple_read_from_buffer(buff, count, ppos,
    session.response, size);
    mutex_unlock(&session.mutex);
    return ret;
    }
    static ssize_t
    vmcp_write(struct file *file, const char __user *buff, size_t count,
    loff_t *ppos)
    {
    char *cmd;
    struct vmcp_session *session;
    if (count > 240)
    return -EINVAL;
    cmd = memdup_user_nul(buff, count);
    if (IS_ERR(cmd))
    return PTR_ERR(cmd);
    session = file.private_data;
    if (mutex_lock_interruptible(&session.mutex)) {
    kfree(cmd);
    return -ERESTARTSYS;
    }
    if (!session.response)
    vmcp_response_alloc(session);
    if (!session.response) {
    mutex_unlock(&session.mutex);
    kfree(cmd);
    return -ENOMEM;
    }
    debug_text_event(vmcp_debug, 1, cmd);
    session.resp_size = cpcmd(cmd, session.response, session.bufsize,
    &session.resp_code);
    mutex_unlock(&session.mutex);
    kfree(cmd);
// ppos = 0;		/* reset the file pointer after a command
    return count;
    }
//
// These ioctls are available, as the semantics of the diagnose 8 call
// does not fit very well into a Linux call. Diagnose X'08' is described in
// CP Programming Services SC24-6084-00
//
// VMCP_GETCODE: gives the CP return code back to user space
// VMCP_SETBUF: sets the response buffer for the next write call. diagnose 8
// expects adjacent pages in real storage and to make matters worse, we
// dont know the size of the response. Therefore we default to PAGESIZE and
// let userspace to change the response size, if userspace expects a bigger
// response
//
#[no_mangle]
unsafe extern "C" fn vmcp_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long vmcp_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct vmcp_session *session;
    let mut ret: c_int = -ENOTTY;
    int __user *argp;
    session = file.private_data;
    argp = (int __user *)arg;
    if (mutex_lock_interruptible(&session.mutex))
    return -ERESTARTSYS;
    switch (cmd) {
    case VMCP_GETCODE:
    ret = put_user(session.resp_code, argp);
    break;
    case VMCP_SETBUF:
    vmcp_response_free(session);
    ret = get_user(session.bufsize, argp);
    if (ret)
    session.bufsize = PAGE_SIZE;
    if (!session.bufsize || get_order(session.bufsize) > 8) {
    session.bufsize = PAGE_SIZE;
    ret = -EINVAL;
    }
    break;
    case VMCP_GETSIZE:
    ret = put_user(session.resp_size, argp);
    break;
    default:
    break;
    }
    mutex_unlock(&session.mutex);
    return ret;
    }
    static const struct file_operations vmcp_fops = {
    .owner		= THIS_MODULE,
    .open		= vmcp_open,
    .release	= vmcp_release,
    .read		= vmcp_read,
    .write		= vmcp_write,
    .unlocked_ioctl	= vmcp_ioctl,
    };
    static struct miscdevice vmcp_dev = {
    .name	= "vmcp",
    .minor	= MISC_DYNAMIC_MINOR,
    .fops	= &vmcp_fops,
    };
#[no_mangle]
unsafe extern "C" fn vmcp_init() -> int __init {
    static int __init vmcp_init(void)
    {
    int ret;
    if (!machine_is_vm())
    return 0;
    vmcp_debug = debug_register("vmcp", 1, 1, 240);
    if (!vmcp_debug)
    return -ENOMEM;
    ret = debug_register_view(vmcp_debug, &debug_hex_ascii_view);
    if (ret) {
    debug_unregister(vmcp_debug);
    return ret;
    }
    ret = misc_register(&vmcp_dev);
    if (ret)
    debug_unregister(vmcp_debug);
    return ret;
    }
    device_initcall(vmcp_init);
