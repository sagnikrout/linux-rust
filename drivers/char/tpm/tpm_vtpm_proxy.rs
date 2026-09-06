//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpm_vtpm_proxy.c
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
// Copyright (C) 2015, 2016 IBM Corporation
// Copyright (C) 2016 Intel Corporation
//
// Author: Stefan Berger <stefanb@us.ibm.com>
//
// Maintained by: <tpmdd-devel@lists.sourceforge.net>
//
// Device driver for vTPM (vTPM proxy driver)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proxy_dev {
    pub chip: *mut tpm_chip,
    pub /: *mut *mut u32 flags; / public API flags,
    pub wq: wait_queue_head_t,
    pub /: *mut *mut mutex buf_lock; / protect buffer and flags,
    pub /: *mut *mut long state; / internal state,

    pub /: *mut *mut size_t req_len; / length of queued TPM request,
    pub /: *mut *mut size_t resp_len; / length of queued TPM response,
    pub /: *mut *mut u8 buffer[TPM_BUFSIZE]; / request/response buffer,
    pub /: *mut *mut work_work; / task that retrieves TPM timeouts,
}

// all supported flags

    static struct workqueue_struct *workqueue;
    static void vtpm_proxy_delete_device(struct proxy_dev *proxy_dev);
//
// Functions related to 'server side'
//
// vtpm_proxy_fops_read - Read TPM commands on 'server side'
//
// @filp: file pointer
// @buf: read buffer
// @count: number of bytes to read
// @off: offset
//
// Return:
// Number of bytes read or negative error code
//
    static ssize_t vtpm_proxy_fops_read(struct file *filp, char __user *buf,
    size_t count, loff_t *off)
    {
    struct proxy_dev *proxy_dev = filp.private_data;
    size_t len;
    int sig, rc;
    sig = wait_event_interruptible(proxy_dev.wq,
    proxy_dev.req_len != 0 ||
    !(proxy_dev.state & STATE_OPENED_FLAG));
    if (sig)
    return -EINTR;
    mutex_lock(&proxy_dev.buf_lock);
    if (!(proxy_dev.state & STATE_OPENED_FLAG)) {
    mutex_unlock(&proxy_dev.buf_lock);
    return -EPIPE;
    }
    len = proxy_dev.req_len;
    if (count < len || len > sizeof(proxy_dev.buffer)) {
    mutex_unlock(&proxy_dev.buf_lock);
    pr_debug("Invalid size in recv: count=%zd, req_len=%zd\n",
    count, len);
    return -EIO;
    }
    rc = copy_to_user(buf, proxy_dev.buffer, len);
    memset(proxy_dev.buffer, 0, len);
    proxy_dev.req_len = 0;
    if (!rc)
    proxy_dev.state |= STATE_WAIT_RESPONSE_FLAG;
    mutex_unlock(&proxy_dev.buf_lock);
    if (rc)
    return -EFAULT;
    return len;
    }
//
// vtpm_proxy_fops_write - Write TPM responses on 'server side'
//
// @filp: file pointer
// @buf: write buffer
// @count: number of bytes to write
// @off: offset
//
// Return:
// Number of bytes read or negative error value
//
    static ssize_t vtpm_proxy_fops_write(struct file *filp, const char __user *buf,
    size_t count, loff_t *off)
    {
    struct proxy_dev *proxy_dev = filp.private_data;
    mutex_lock(&proxy_dev.buf_lock);
    if (!(proxy_dev.state & STATE_OPENED_FLAG)) {
    mutex_unlock(&proxy_dev.buf_lock);
    return -EPIPE;
    }
    if (count > sizeof(proxy_dev.buffer) ||
    !(proxy_dev.state & STATE_WAIT_RESPONSE_FLAG)) {
    mutex_unlock(&proxy_dev.buf_lock);
    return -EIO;
    }
    proxy_dev.state &= ~STATE_WAIT_RESPONSE_FLAG;
    proxy_dev.req_len = 0;
    if (copy_from_user(proxy_dev.buffer, buf, count)) {
    mutex_unlock(&proxy_dev.buf_lock);
    return -EFAULT;
    }
    proxy_dev.resp_len = count;
    mutex_unlock(&proxy_dev.buf_lock);
    wake_up_interruptible(&proxy_dev.wq);
    return count;
    }
//
// vtpm_proxy_fops_poll - Poll status on 'server side'
//
// @filp: file pointer
// @wait: poll table
//
// Return: Poll flags
//
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_fops_poll(filp: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t vtpm_proxy_fops_poll(struct file *filp, poll_table *wait)
    {
    struct proxy_dev *proxy_dev = filp.private_data;
    __poll_t ret;
    poll_wait(filp, &proxy_dev.wq, wait);
    ret = EPOLLOUT;
    mutex_lock(&proxy_dev.buf_lock);
    if (proxy_dev.req_len)
    ret |= EPOLLIN | EPOLLRDNORM;
    if (!(proxy_dev.state & STATE_OPENED_FLAG))
    ret |= EPOLLHUP;
    mutex_unlock(&proxy_dev.buf_lock);
    return ret;
    }
//
// vtpm_proxy_fops_open - Open vTPM device on 'server side'
//
// @filp: file pointer
//
// Called when setting up the anonymous file descriptor
//
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_fops_open(filp: *mut file) {
    static void vtpm_proxy_fops_open(struct file *filp)
    {
    struct proxy_dev *proxy_dev = filp.private_data;
    proxy_dev.state |= STATE_OPENED_FLAG;
    }
//
// vtpm_proxy_fops_undo_open - counter-part to vtpm_fops_open
// Call to undo vtpm_proxy_fops_open
//
// @proxy_dev: tpm proxy device
//
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_fops_undo_open(proxy_dev: *mut proxy_dev) {
    static void vtpm_proxy_fops_undo_open(struct proxy_dev *proxy_dev)
    {
    mutex_lock(&proxy_dev.buf_lock);
    proxy_dev.state &= ~STATE_OPENED_FLAG;
    mutex_unlock(&proxy_dev.buf_lock);
// no more TPM responses -- wake up anyone waiting for them
    wake_up_interruptible(&proxy_dev.wq);
    }
//
// vtpm_proxy_fops_release - Close 'server side'
//
// @inode: inode
// @filp: file pointer
// Return:
// Always returns 0.
//
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_fops_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int vtpm_proxy_fops_release(struct inode *inode, struct file *filp)
    {
    struct proxy_dev *proxy_dev = filp.private_data;
    filp.private_data = core::ptr::null_mut();
    vtpm_proxy_delete_device(proxy_dev);
    return 0;
    }
    static const struct file_operations vtpm_proxy_fops = {
    .owner = THIS_MODULE,
    .read = vtpm_proxy_fops_read,
    .write = vtpm_proxy_fops_write,
    .poll = vtpm_proxy_fops_poll,
    .release = vtpm_proxy_fops_release,
    };
//
// Functions invoked by the core TPM driver to send TPM commands to
// 'server side' and receive responses from there.
//
// Called when core TPM driver reads TPM responses from 'server side'
//
// @chip: tpm chip to use
// @buf: receive buffer
// @count: bytes to read
// Return:
// Number of TPM response bytes read, negative error value otherwise
//
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_tpm_op_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> c_int {
    static int vtpm_proxy_tpm_op_recv(struct tpm_chip *chip, u8 *buf, size_t count)
    {
    struct proxy_dev *proxy_dev = dev_get_drvdata(&chip.dev);
    size_t len;
// process gone ?
    mutex_lock(&proxy_dev.buf_lock);
    if (!(proxy_dev.state & STATE_OPENED_FLAG)) {
    mutex_unlock(&proxy_dev.buf_lock);
    return -EPIPE;
    }
    len = proxy_dev.resp_len;
    if (count < len) {
    dev_err(&chip.dev,
    "Invalid size in recv: count=%zd, resp_len=%zd\n",
    count, len);
    len = -EIO;
    goto out;
    }
    memcpy(buf, proxy_dev.buffer, len);
    proxy_dev.resp_len = 0;
    out:
    mutex_unlock(&proxy_dev.buf_lock);
    return len;
    }
    static int vtpm_proxy_is_driver_command(struct tpm_chip *chip,
    u8 *buf, size_t count)
    {
    struct tpm_header *hdr = (struct tpm_header *)buf;
    if (count < sizeof(struct tpm_header))
    return 0;
    if (chip.flags & TPM_CHIP_FLAG_TPM2) {
    switch (be32_to_cpu(hdr.ordinal)) {
    case TPM2_CC_SET_LOCALITY:
    return 1;
    }
    } else {
    switch (be32_to_cpu(hdr.ordinal)) {
    case TPM_ORD_SET_LOCALITY:
    return 1;
    }
    }
    return 0;
    }
//
// Called when core TPM driver forwards TPM requests to 'server side'.
//
// @chip: tpm chip to use
// @buf: send buffer
// @bufsiz: size of the buffer
// @count: bytes to send
//
// Return:
// 0 in case of success, negative error value otherwise.
//
    static int vtpm_proxy_tpm_op_send(struct tpm_chip *chip, u8 *buf, size_t bufsiz,
    size_t count)
    {
    struct proxy_dev *proxy_dev = dev_get_drvdata(&chip.dev);
    if (count > sizeof(proxy_dev.buffer)) {
    dev_err(&chip.dev,
    "Invalid size in send: count=%zd, buffer size=%zd\n",
    count, sizeof(proxy_dev.buffer));
    return -EIO;
    }
    if (!(proxy_dev.state & STATE_DRIVER_COMMAND) &&
    vtpm_proxy_is_driver_command(chip, buf, count))
    return -EFAULT;
    mutex_lock(&proxy_dev.buf_lock);
    if (!(proxy_dev.state & STATE_OPENED_FLAG)) {
    mutex_unlock(&proxy_dev.buf_lock);
    return -EPIPE;
    }
    proxy_dev.resp_len = 0;
    proxy_dev.req_len = count;
    memcpy(proxy_dev.buffer, buf, count);
    proxy_dev.state &= ~STATE_WAIT_RESPONSE_FLAG;
    mutex_unlock(&proxy_dev.buf_lock);
    wake_up_interruptible(&proxy_dev.wq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_tpm_op_cancel(chip: *mut tpm_chip) {
    static void vtpm_proxy_tpm_op_cancel(struct tpm_chip *chip)
    {
// not supported
    }
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_tpm_op_status(chip: *mut tpm_chip) -> u8 {
    static u8 vtpm_proxy_tpm_op_status(struct tpm_chip *chip)
    {
    struct proxy_dev *proxy_dev = dev_get_drvdata(&chip.dev);
    if (proxy_dev.resp_len)
    return VTPM_PROXY_REQ_COMPLETE_FLAG;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_tpm_req_canceled(chip: *mut tpm_chip, status: u8) -> bool {
    static bool vtpm_proxy_tpm_req_canceled(struct tpm_chip  *chip, u8 status)
    {
    struct proxy_dev *proxy_dev = dev_get_drvdata(&chip.dev);
    bool ret;
    mutex_lock(&proxy_dev.buf_lock);
    ret = !(proxy_dev.state & STATE_OPENED_FLAG);
    mutex_unlock(&proxy_dev.buf_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_request_locality(chip: *mut tpm_chip, locality: c_int) -> c_int {
    static int vtpm_proxy_request_locality(struct tpm_chip *chip, int locality)
    {
    int rc;
    const struct tpm_header *header;
    struct proxy_dev *proxy_dev = dev_get_drvdata(&chip.dev);
    struct tpm_buf *buf __free(kfree) = kzalloc(TPM_BUFSIZE, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    tpm_buf_init(buf, TPM_BUFSIZE);
    if (chip.flags & TPM_CHIP_FLAG_TPM2)
    tpm_buf_reset(buf, TPM2_ST_SESSIONS, TPM2_CC_SET_LOCALITY);
    else
    tpm_buf_reset(buf, TPM_TAG_RQU_COMMAND, TPM_ORD_SET_LOCALITY);
    tpm_buf_append_u8(buf, locality);
    proxy_dev.state |= STATE_DRIVER_COMMAND;
    rc = tpm_transmit_cmd(chip, buf, 0, "attempting to set locality");
    proxy_dev.state &= ~STATE_DRIVER_COMMAND;
    if (rc < 0)
    return rc;
    header = (const struct tpm_header *)buf.data;
    rc = be32_to_cpu(header.return_code);
    if (rc)
    locality = -1;
    return locality;
    }
    static const struct tpm_class_ops vtpm_proxy_tpm_ops = {
    .flags = TPM_OPS_AUTO_STARTUP,
    .recv = vtpm_proxy_tpm_op_recv,
    .send = vtpm_proxy_tpm_op_send,
    .cancel = vtpm_proxy_tpm_op_cancel,
    .status = vtpm_proxy_tpm_op_status,
    .req_complete_mask = VTPM_PROXY_REQ_COMPLETE_FLAG,
    .req_complete_val = VTPM_PROXY_REQ_COMPLETE_FLAG,
    .req_canceled = vtpm_proxy_tpm_req_canceled,
    .request_locality = vtpm_proxy_request_locality,
    };
//
// Code related to the startup of the TPM 2 and startup of TPM 1.2 +
// retrieval of timeouts and durations.
//
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_work(work: *mut work_struct) {
    static void vtpm_proxy_work(struct work_struct *work)
    {
    struct proxy_dev *proxy_dev = container_of(work, struct proxy_dev,
    work);
    int rc;
    rc = tpm_chip_register(proxy_dev.chip);
    if (rc)
    vtpm_proxy_fops_undo_open(proxy_dev);
    else
    proxy_dev.state |= STATE_REGISTERED_FLAG;
    }
//
// vtpm_proxy_work_stop: make sure the work has finished
//
// This function is useful when user space closed the fd
// while the driver still determines timeouts.
//
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_work_stop(proxy_dev: *mut proxy_dev) {
    static void vtpm_proxy_work_stop(struct proxy_dev *proxy_dev)
    {
    vtpm_proxy_fops_undo_open(proxy_dev);
    flush_work(&proxy_dev.work);
    }
//
// vtpm_proxy_work_start: Schedule the work for TPM 1.2 & 2 initialization
//
#[no_mangle]
pub unsafe extern "C" fn vtpm_proxy_work_start(proxy_dev: *mut proxy_dev) {
    static inline void vtpm_proxy_work_start(struct proxy_dev *proxy_dev)
    {
    queue_work(workqueue, &proxy_dev.work);
    }
//
// Code related to creation and deletion of device pairs
//
    static struct proxy_dev *vtpm_proxy_create_proxy_dev(void)
    {
    struct proxy_dev *proxy_dev;
    struct tpm_chip *chip;
    int err;
    proxy_dev = kzalloc_obj(*proxy_dev);
    if (proxy_dev == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    init_waitqueue_head(&proxy_dev.wq);
    mutex_init(&proxy_dev.buf_lock);
    INIT_WORK(&proxy_dev.work, vtpm_proxy_work);
    chip = tpm_chip_alloc(core::ptr::null_mut(), &vtpm_proxy_tpm_ops);
    if (IS_ERR(chip)) {
    err = PTR_ERR(chip);
    goto err_proxy_dev_free;
    }
    dev_set_drvdata(&chip.dev, proxy_dev);
    proxy_dev.chip = chip;
    return proxy_dev;
    err_proxy_dev_free:
    kfree(proxy_dev);
    return ERR_PTR(err);
    }
//
// Undo what has been done in vtpm_create_proxy_dev
//
#[no_mangle]
pub unsafe extern "C" fn vtpm_proxy_delete_proxy_dev(proxy_dev: *mut proxy_dev) {
    static inline void vtpm_proxy_delete_proxy_dev(struct proxy_dev *proxy_dev)
    {
    put_device(&proxy_dev.chip.dev); /* frees chip */
    kfree(proxy_dev);
    }
//
// Create a /dev/tpm%d and 'server side' file descriptor pair
//
// Return:
// Returns file pointer on success, an error value otherwise
//
    static struct file *vtpm_proxy_create_device(
    struct vtpm_proxy_new_dev *vtpm_new_dev)
    {
    struct proxy_dev *proxy_dev;
    int rc, fd;
    struct file *file;
    if (vtpm_new_dev.flags & ~VTPM_PROXY_FLAGS_ALL)
    return ERR_PTR(-EOPNOTSUPP);
    proxy_dev = vtpm_proxy_create_proxy_dev();
    if (IS_ERR(proxy_dev))
    return ERR_CAST(proxy_dev);
    proxy_dev.flags = vtpm_new_dev.flags;
// setup an anonymous file for the server-side
    fd = get_unused_fd_flags(O_RDWR);
    if (fd < 0) {
    rc = fd;
    goto err_delete_proxy_dev;
    }
    file = anon_inode_getfile("[vtpms]", &vtpm_proxy_fops, proxy_dev,
    O_RDWR);
    if (IS_ERR(file)) {
    rc = PTR_ERR(file);
    goto err_put_unused_fd;
    }
// from now on we can unwind with put_unused_fd() + fput()
// simulate an open() on the server side
    vtpm_proxy_fops_open(file);
    if (proxy_dev.flags & VTPM_PROXY_FLAG_TPM2)
    proxy_dev.chip.flags |= TPM_CHIP_FLAG_TPM2;
    vtpm_proxy_work_start(proxy_dev);
    vtpm_new_dev.fd = fd;
    vtpm_new_dev.major = MAJOR(proxy_dev.chip.dev.devt);
    vtpm_new_dev.minor = MINOR(proxy_dev.chip.dev.devt);
    vtpm_new_dev.tpm_num = proxy_dev.chip.dev_num;
    return file;
    err_put_unused_fd:
    put_unused_fd(fd);
    err_delete_proxy_dev:
    vtpm_proxy_delete_proxy_dev(proxy_dev);
    return ERR_PTR(rc);
    }
//
// Counter part to vtpm_create_device.
//
#[no_mangle]
unsafe extern "C" fn vtpm_proxy_delete_device(proxy_dev: *mut proxy_dev) {
    static void vtpm_proxy_delete_device(struct proxy_dev *proxy_dev)
    {
    vtpm_proxy_work_stop(proxy_dev);
//
// A client may hold the 'ops' lock, so let it know that the server
// side shuts down before we try to grab the 'ops' lock when
// unregistering the chip.
//
    vtpm_proxy_fops_undo_open(proxy_dev);
    if (proxy_dev.state & STATE_REGISTERED_FLAG)
    tpm_chip_unregister(proxy_dev.chip);
    vtpm_proxy_delete_proxy_dev(proxy_dev);
    }
//
// Code related to the control device /dev/vtpmx
//
// vtpmx_ioc_new_dev - handler for the %VTPM_PROXY_IOC_NEW_DEV ioctl
// @file:	/dev/vtpmx
// @ioctl:	the ioctl number
// @arg:	pointer to the struct vtpmx_proxy_new_dev
//
// Creates an anonymous file that is used by the process acting as a TPM to
// communicate with the client processes. The function will also add a new TPM
// device through which data is proxied to this TPM acting process. The caller
// will be provided with a file descriptor to communicate with the clients and
// major and minor numbers for the TPM device.
//
    static long vtpmx_ioc_new_dev(struct file *file, unsigned int ioctl,
    unsigned long arg)
    {
    void __user *argp = (void __user *)arg;
    struct vtpm_proxy_new_dev __user *vtpm_new_dev_p;
    struct vtpm_proxy_new_dev vtpm_new_dev;
    struct file *vtpm_file;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    vtpm_new_dev_p = argp;
    if (copy_from_user(&vtpm_new_dev, vtpm_new_dev_p,
    sizeof(vtpm_new_dev)))
    return -EFAULT;
    vtpm_file = vtpm_proxy_create_device(&vtpm_new_dev);
    if (IS_ERR(vtpm_file))
    return PTR_ERR(vtpm_file);
    if (copy_to_user(vtpm_new_dev_p, &vtpm_new_dev,
    sizeof(vtpm_new_dev))) {
    put_unused_fd(vtpm_new_dev.fd);
    fput(vtpm_file);
    return -EFAULT;
    }
    fd_install(vtpm_new_dev.fd, vtpm_file);
    return 0;
    }
//
// vtpmx_fops_ioctl: ioctl on /dev/vtpmx
//
// Return:
// Returns 0 on success, a negative error code otherwise.
//
    static long vtpmx_fops_ioctl(struct file *f, unsigned int ioctl,
    unsigned long arg)
    {
    switch (ioctl) {
    case VTPM_PROXY_IOC_NEW_DEV:
    return vtpmx_ioc_new_dev(f, ioctl, arg);
    default:
    return -ENOIOCTLCMD;
    }
    }
    static const struct file_operations vtpmx_fops = {
    .owner = THIS_MODULE,
    .unlocked_ioctl = vtpmx_fops_ioctl,
    .compat_ioctl = compat_ptr_ioctl,
    .llseek = noop_llseek,
    };
    static struct miscdevice vtpmx_miscdev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "vtpmx",
    .fops = &vtpmx_fops,
    };
#[no_mangle]
unsafe extern "C" fn vtpm_module_init() -> int __init {
    static int __init vtpm_module_init(void)
    {
    int rc;
    workqueue = create_workqueue("tpm-vtpm");
    if (!workqueue) {
    pr_err("couldn't create workqueue\n");
    return -ENOMEM;
    }
    rc = misc_register(&vtpmx_miscdev);
    if (rc) {
    pr_err("couldn't create vtpmx device\n");
    destroy_workqueue(workqueue);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn vtpm_module_exit() -> void __exit {
    static void __exit vtpm_module_exit(void)
    {
    destroy_workqueue(workqueue);
    misc_deregister(&vtpmx_miscdev);
    }
    module_init(vtpm_module_init);
    module_exit(vtpm_module_exit);
    MODULE_AUTHOR("Stefan Berger <stefanb@us.ibm.com>");
    MODULE_DESCRIPTION("vTPM Driver");
    MODULE_VERSION("0.1");
    MODULE_LICENSE("GPL");
