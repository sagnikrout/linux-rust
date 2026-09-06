//! Automatically rewritten from C to Rust
//! Source: fs/coda/psdev.c
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
// An implementation of a loadable kernel mode driver providing
// multiple kernel/user space bidirectional communications links.
//
// Author: 	Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// Adapted to become the Linux 2.0 Coda pseudo device
// Peter  Braam  <braam@maths.ox.ac.uk>
// Michael Callahan <mjc@emmy.smith.edu>
//
// Changes for Linux 2.1
// Copyright (c) 1997 Carnegie-Mellon University
//

// statistics
    int           coda_hard;         /* allows signals during upcalls */
    unsigned long coda_timeout = 30; /* .. secs, then signals will dequeue */
    struct venus_comm coda_comms[MAX_CODADEVS];
    static struct class *coda_psdev_class;
//
// Device operations
//
#[no_mangle]
unsafe extern "C" fn coda_psdev_poll(file: *mut file, wait: *mut *mut poll_table) -> __poll_t {
    static __poll_t coda_psdev_poll(struct file *file, poll_table * wait)
    {
    struct venus_comm *vcp = (struct venus_comm *) file.private_data;
    let mut mask: __poll_t = EPOLLOUT | EPOLLWRNORM;
    poll_wait(file, &vcp.vc_waitq, wait);
    mutex_lock(&vcp.vc_mutex);
    if (!list_empty(&vcp.vc_pending))
    mask |= EPOLLIN | EPOLLRDNORM;
    mutex_unlock(&vcp.vc_mutex);
    return mask;
    }
#[no_mangle]
unsafe extern "C" fn coda_psdev_ioctl(filp: *mut *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long coda_psdev_ioctl(struct file * filp, unsigned int cmd, unsigned long arg)
    {
    unsigned int data;
    switch(cmd) {
    case CIOC_KERNEL_VERSION:
    data = CODA_KERNEL_VERSION;
    return put_user(data, (int __user *) arg);
    default:
    return -ENOTTY;
    }
    return 0;
    }
//
// Receive a message written by Venus to the psdev
//
    static ssize_t coda_psdev_write(struct file *file, const char __user *buf,
    size_t nbytes, loff_t *off)
    {
    struct venus_comm *vcp = (struct venus_comm *) file.private_data;
    struct upc_req *req = core::ptr::null_mut();
    struct upc_req *tmp;
    struct list_head *lh;
    struct coda_in_hdr hdr;
    let mut retval: isize = 0, count = 0;
    int error;
// make sure there is enough to copy out the (opcode, unique) values
    if (nbytes < (2 * sizeof(u_int32_t)))
    return -EINVAL;
// Peek at the opcode, uniquefier
    if (copy_from_user(&hdr, buf, 2 * sizeof(u_int32_t)))
    return -EFAULT;
    if (DOWNCALL(hdr.opcode)) {
    union outputArgs *dcbuf;
    let mut size: c_int = sizeof(*dcbuf);
    if  ( nbytes < sizeof(struct coda_out_hdr) ) {
    pr_warn("coda_downcall opc %d uniq %d, not enough!\n",
    hdr.opcode, hdr.unique);
    count = nbytes;
    goto out;
    }
    if ( nbytes > size ) {
    pr_warn("downcall opc %d, uniq %d, too much!",
    hdr.opcode, hdr.unique);
    nbytes = size;
    }
    dcbuf = vmemdup_user(buf, nbytes);
    if (IS_ERR(dcbuf)) {
    retval = PTR_ERR(dcbuf);
    goto out;
    }
// what downcall errors does Venus handle ?
    error = coda_downcall(vcp, hdr.opcode, dcbuf, nbytes);
    kvfree(dcbuf);
    if (error) {
    pr_warn("%s: coda_downcall error: %d\n",
    __func__, error);
    retval = error;
    goto out;
    }
    count = nbytes;
    goto out;
    }
// Look for the message on the processing queue.
    mutex_lock(&vcp.vc_mutex);
    list_for_each(lh, &vcp.vc_processing) {
    tmp = list_entry(lh, struct upc_req , uc_chain);
    if (tmp.uc_unique == hdr.unique) {
    req = tmp;
    list_del(&req.uc_chain);
    break;
    }
    }
    mutex_unlock(&vcp.vc_mutex);
    if (!req) {
    pr_warn("%s: msg (%d, %d) not found\n",
    __func__, hdr.opcode, hdr.unique);
    retval = -ESRCH;
    goto out;
    }
// move data into response buffer.
    if (req.uc_outSize < nbytes) {
    pr_warn("%s: too much cnt: %d, cnt: %ld, opc: %d, uniq: %d.\n",
    __func__, req.uc_outSize, (long)nbytes,
    hdr.opcode, hdr.unique);
    nbytes = req.uc_outSize; /* don't have more space! */
    }
    if (copy_from_user(req.uc_data, buf, nbytes)) {
    req.uc_flags |= CODA_REQ_ABORT;
    wake_up(&req.uc_sleep);
    retval = -EFAULT;
    goto out;
    }
// adjust outsize. is this useful ??
    req.uc_outSize = nbytes;
    req.uc_flags |= CODA_REQ_WRITE;
    count = nbytes;
// Convert filedescriptor into a file handle
    if (req.uc_opcode == CODA_OPEN_BY_FD) {
    struct coda_open_by_fd_out *outp =
    (struct coda_open_by_fd_out *)req.uc_data;
    if (!outp.oh.result) {
    outp.fh = fget(outp.fd);
    if (!outp.fh)
    return -EBADF;
    }
    }
    wake_up(&req.uc_sleep);
    out:
    return(count ? count : retval);
    }
//
// Read a message from the kernel to Venus
//
    static ssize_t coda_psdev_read(struct file * file, char __user * buf,
    size_t nbytes, loff_t *off)
    {
    DECLARE_WAITQUEUE(wait, current);
    struct venus_comm *vcp = (struct venus_comm *) file.private_data;
    struct upc_req *req;
    let mut retval: isize = 0, count = 0;
    if (nbytes == 0)
    return 0;
    mutex_lock(&vcp.vc_mutex);
    add_wait_queue(&vcp.vc_waitq, &wait);
    set_current_state(TASK_INTERRUPTIBLE);
    while (list_empty(&vcp.vc_pending)) {
    if (file.f_flags & O_NONBLOCK) {
    retval = -EAGAIN;
    break;
    }
    if (signal_pending(current)) {
    retval = -ERESTARTSYS;
    break;
    }
    mutex_unlock(&vcp.vc_mutex);
    schedule();
    mutex_lock(&vcp.vc_mutex);
    }
    set_current_state(TASK_RUNNING);
    remove_wait_queue(&vcp.vc_waitq, &wait);
    if (retval)
    goto out;
    req = list_entry(vcp.vc_pending.next, struct upc_req,uc_chain);
    list_del(&req.uc_chain);
// Move the input args into userspace
    count = req.uc_inSize;
    if (nbytes < req.uc_inSize) {
    pr_warn("%s: Venus read %ld bytes of %d in message\n",
    __func__, (long)nbytes, req.uc_inSize);
    count = nbytes;
    }
    if (copy_to_user(buf, req.uc_data, count))
    retval = -EFAULT;
// If request was not a signal, enqueue and don't free
    if (!(req.uc_flags & CODA_REQ_ASYNC)) {
    req.uc_flags |= CODA_REQ_READ;
    list_add_tail(&(req.uc_chain), &vcp.vc_processing);
    goto out;
    }
    kvfree(req.uc_data);
    kfree(req);
    out:
    mutex_unlock(&vcp.vc_mutex);
    return (count ? count : retval);
    }
#[no_mangle]
unsafe extern "C" fn coda_psdev_open(inode: *mut *mut inode, file: *mut *mut file) -> c_int {
    static int coda_psdev_open(struct inode * inode, struct file * file)
    {
    struct venus_comm *vcp;
    int idx, err;
    if (task_active_pid_ns(current) != &init_pid_ns)
    return -EINVAL;
    if (current_user_ns() != &init_user_ns)
    return -EINVAL;
    idx = iminor(inode);
    if (idx < 0 || idx >= MAX_CODADEVS)
    return -ENODEV;
    err = -EBUSY;
    vcp = &coda_comms[idx];
    mutex_lock(&vcp.vc_mutex);
    if (!vcp.vc_inuse) {
    vcp.vc_inuse++;
    INIT_LIST_HEAD(&vcp.vc_pending);
    INIT_LIST_HEAD(&vcp.vc_processing);
    init_waitqueue_head(&vcp.vc_waitq);
    vcp.vc_sb = core::ptr::null_mut();
    vcp.vc_seq = 0;
    file.private_data = vcp;
    err = 0;
    }
    mutex_unlock(&vcp.vc_mutex);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn coda_psdev_release(inode: *mut *mut inode, file: *mut *mut file) -> c_int {
    static int coda_psdev_release(struct inode * inode, struct file * file)
    {
    struct venus_comm *vcp = (struct venus_comm *) file.private_data;
    struct upc_req *req, *tmp;
    if (!vcp || !vcp.vc_inuse ) {
    pr_warn("%s: Not open.\n", __func__);
    return -1;
    }
    mutex_lock(&vcp.vc_mutex);
// Wakeup clients so they can return.
    list_for_each_entry_safe(req, tmp, &vcp.vc_pending, uc_chain) {
    list_del(&req.uc_chain);
// Async requests need to be freed here
    if (req.uc_flags & CODA_REQ_ASYNC) {
    kvfree(req.uc_data);
    kfree(req);
    continue;
    }
    req.uc_flags |= CODA_REQ_ABORT;
    wake_up(&req.uc_sleep);
    }
    list_for_each_entry_safe(req, tmp, &vcp.vc_processing, uc_chain) {
    list_del(&req.uc_chain);
    req.uc_flags |= CODA_REQ_ABORT;
    wake_up(&req.uc_sleep);
    }
    file.private_data = core::ptr::null_mut();
    vcp.vc_inuse--;
    mutex_unlock(&vcp.vc_mutex);
    return 0;
    }
    static const struct file_operations coda_psdev_fops = {
    .owner		= THIS_MODULE,
    .read		= coda_psdev_read,
    .write		= coda_psdev_write,
    .poll		= coda_psdev_poll,
    .unlocked_ioctl	= coda_psdev_ioctl,
    .open		= coda_psdev_open,
    .release	= coda_psdev_release,
    .llseek		= noop_llseek,
    };
#[no_mangle]
unsafe extern "C" fn init_coda_psdev() -> int __init {
    static int __init init_coda_psdev(void)
    {
    int i, err = 0;
    if (register_chrdev(CODA_PSDEV_MAJOR, "coda", &coda_psdev_fops)) {
    pr_err("%s: unable to get major %d\n",
    __func__, CODA_PSDEV_MAJOR);
    return -EIO;
    }
    coda_psdev_class = class_create("coda");
    if (IS_ERR(coda_psdev_class)) {
    err = PTR_ERR(coda_psdev_class);
    goto out_chrdev;
    }
    for (i = 0; i < MAX_CODADEVS; i++) {
    mutex_init(&(&coda_comms[i]).vc_mutex);
    device_create(coda_psdev_class, core::ptr::null_mut(),
    MKDEV(CODA_PSDEV_MAJOR, i), core::ptr::null_mut(), "cfs%d", i);
    }
    coda_sysctl_init();
    goto out;
    out_chrdev:
    unregister_chrdev(CODA_PSDEV_MAJOR, "coda");
    out:
    return err;
    }
    MODULE_AUTHOR("Jan Harkes, Peter J. Braam");
    MODULE_DESCRIPTION("Coda Distributed File System VFS interface");
    MODULE_ALIAS_CHARDEV_MAJOR(CODA_PSDEV_MAJOR);
    MODULE_LICENSE("GPL");
    MODULE_VERSION("7.2");
#[no_mangle]
unsafe extern "C" fn init_coda() -> int __init {
    static int __init init_coda(void)
    {
    int status;
    int i;
    status = coda_init_inodecache();
    if (status)
    goto out2;
    status = init_coda_psdev();
    if ( status ) {
    pr_warn("Problem (%d) in init_coda_psdev\n", status);
    goto out1;
    }
    status = register_filesystem(&coda_fs_type);
    if (status) {
    pr_warn("failed to register filesystem!\n");
    goto out;
    }
    return 0;
    out:
    for (i = 0; i < MAX_CODADEVS; i++)
    device_destroy(coda_psdev_class, MKDEV(CODA_PSDEV_MAJOR, i));
    class_destroy(coda_psdev_class);
    unregister_chrdev(CODA_PSDEV_MAJOR, "coda");
    coda_sysctl_clean();
    out1:
    coda_destroy_inodecache();
    out2:
    return status;
    }
#[no_mangle]
unsafe extern "C" fn exit_coda() -> void __exit {
    static void __exit exit_coda(void)
    {
    int err, i;
    err = unregister_filesystem(&coda_fs_type);
    if (err != 0)
    pr_warn("failed to unregister filesystem\n");
    for (i = 0; i < MAX_CODADEVS; i++)
    device_destroy(coda_psdev_class, MKDEV(CODA_PSDEV_MAJOR, i));
    class_destroy(coda_psdev_class);
    unregister_chrdev(CODA_PSDEV_MAJOR, "coda");
    coda_sysctl_clean();
    coda_destroy_inodecache();
    }
    module_init(init_coda);
    module_exit(exit_coda);
