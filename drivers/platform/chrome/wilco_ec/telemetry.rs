//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/wilco_ec/telemetry.c
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
// Telemetry communication for Wilco EC
//
// Copyright 2019 Google LLC
//
// The Wilco Embedded Controller is able to send telemetry data
// which is useful for enterprise applications. A daemon running on
// the OS sends a command to the EC via a write() to a char device,
// and can read the response with a read(). The write() request is
// verified by the driver to ensure that it is performing only one
// of the allowlisted commands, and that no extraneous data is
// being transmitted to the EC. The response is passed directly
// back to the reader with no modification.
//
// The character device will appear as /dev/wilco_telemN, where N
// is some small non-negative integer, starting with 0. Only one
// process may have the file descriptor open at a time. The calling
// userspace program needs to keep the device file descriptor open
// between the calls to write() and read() in order to preserve the
// response. Up to 32 bytes will be available for reading.
//
// For testing purposes, try requesting the EC's firmware build
// date, by sending the WILCO_EC_TELEM_GET_VERSION command with
// argument index=3. i.e. write [0x38, 0x00, 0x03]
// to the device node. An ASCII string of the build date is
// returned.
//

    static struct class telem_class = {
    .name	= TELEM_CLASS_NAME,
    };
// Keep track of all the device numbers used.
pub const TELEM_MAX_DEV: c_int = 128;
    static int telem_major;
    static DEFINE_IDA(telem_ida);
// EC telemetry command codes
pub const WILCO_EC_TELEM_GET_LOG: c_uint = 0x99;
pub const WILCO_EC_TELEM_GET_VERSION: c_uint = 0x38;
pub const WILCO_EC_TELEM_GET_FAN_INFO: c_uint = 0x2E;
pub const WILCO_EC_TELEM_GET_DIAG_INFO: c_uint = 0xFA;
pub const WILCO_EC_TELEM_GET_TEMP_INFO: c_uint = 0x95;
pub const WILCO_EC_TELEM_GET_TEMP_READ: c_uint = 0x2C;
pub const WILCO_EC_TELEM_GET_BATT_EXT_INFO: c_uint = 0x07;
pub const WILCO_EC_TELEM_GET_BATT_PPID_INFO: c_uint = 0x8A;
pub const TELEM_ARGS_SIZE_MAX: c_int = 30;
//
// The following telem_args_get_* structs are embedded within the |args| field
// of wilco_ec_telem_request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_args_get_log {
    pub log_type: u8,
    pub log_index: u8,
    pub __packed: },
//
// Get a piece of info about the EC firmware version:
// 0 = label
// 1 = svn_rev
// 2 = model_no
// 3 = build_date
// 4 = frio_version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_args_get_version {
    pub index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_args_get_fan_info {
    pub command: u8,
    pub fan_number: u8,
    pub arg: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_args_get_diag_info {
    pub type: u8,
    pub sub_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_args_get_temp_info {
    pub command: u8,
    pub index: u8,
    pub field: u8,
    pub zone: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_args_get_temp_read {
    pub sensor_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_args_get_batt_ext_info {
    pub var_args: [u8; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_args_get_batt_ppid_info {
    pub /: *mut *mut u8 always1; / Should always be 1,
    pub __packed: },
//
// struct wilco_ec_telem_request - Telemetry command and arguments sent to EC.
// @command: One of WILCO_EC_TELEM_GET_* command codes.
// @reserved: Must be 0.
// @args: The first N bytes are one of telem_args_get_* structs, the rest is 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilco_ec_telem_request {
    pub command: u8,
    pub reserved: u8,
    union {
    pub buf: [u8; TELEM_ARGS_SIZE_MAX],
    pub get_log: telem_args_get_log,
    pub get_version: telem_args_get_version,
    pub get_fan_info: telem_args_get_fan_info,
    pub get_diag_info: telem_args_get_diag_info,
    pub get_temp_info: telem_args_get_temp_info,
    pub get_temp_read: telem_args_get_temp_read,
    pub get_batt_ext_info: telem_args_get_batt_ext_info,
    pub get_batt_ppid_info: telem_args_get_batt_ppid_info,
    pub args: },
    pub __packed: },
//
// check_telem_request() - Ensure that a request from userspace is valid.
// @rq: Request buffer copied from userspace.
// @size: Number of bytes copied from userspace.
//
// Return: 0 if valid, -EINVAL if bad command or reserved byte is non-zero,
// -EMSGSIZE if the request is too long.
//
// We do not want to allow userspace to send arbitrary telemetry commands to
// the EC. Therefore we check to ensure that
// 1. The request follows the format of struct wilco_ec_telem_request.
// 2. The supplied command code is one of the allowlisted commands.
// 3. The request only contains the necessary data for the header and arguments.
//
    static int check_telem_request(struct wilco_ec_telem_request *rq,
    size_t size)
    {
    pub args): size_t max_size = offsetof(struct wilco_ec_telem_request,,
    if (rq.reserved)
    pub -EINVAL: return,
    switch (rq.command) {
    case WILCO_EC_TELEM_GET_LOG:
    pub sizeof(rq->args.get_log): max_size +=,
    case WILCO_EC_TELEM_GET_VERSION:
    pub sizeof(rq->args.get_version): max_size +=,
    case WILCO_EC_TELEM_GET_FAN_INFO:
    pub sizeof(rq->args.get_fan_info): max_size +=,
    case WILCO_EC_TELEM_GET_DIAG_INFO:
    pub sizeof(rq->args.get_diag_info): max_size +=,
    case WILCO_EC_TELEM_GET_TEMP_INFO:
    pub sizeof(rq->args.get_temp_info): max_size +=,
    case WILCO_EC_TELEM_GET_TEMP_READ:
    pub sizeof(rq->args.get_temp_read): max_size +=,
    case WILCO_EC_TELEM_GET_BATT_EXT_INFO:
    pub sizeof(rq->args.get_batt_ext_info): max_size +=,
    case WILCO_EC_TELEM_GET_BATT_PPID_INFO:
    if (rq.args.get_batt_ppid_info.always1 != 1)
    pub -EINVAL: return,
    pub sizeof(rq->args.get_batt_ppid_info): max_size +=,
    default:
    pub -EINVAL: return,
    }
    pub -EMSGSIZE: return (size <= max_size) ? 0 :,
    }
//
// struct telem_device_data - Data for a Wilco EC device that queries telemetry.
// @cdev: Char dev that userspace reads and polls from.
// @dev: Device associated with the %cdev.
// @ec: Wilco EC that we will be communicating with using the mailbox interface.
// @available: Boolean of if the device can be opened.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_device_data {
    pub dev: device,
    pub cdev: cdev,
    pub ec: *mut wilco_ec_device,
    pub available: core::sync::atomic::AtomicI32,
}

//
// struct telem_session_data - Data that exists between open() and release().
// @dev_data: Pointer to get back to the device data and EC.
// @request: Command and arguments sent to EC.
// @response: Response buffer of data from EC.
// @has_msg: Is there data available to read from a previous write?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_session_data {
    pub dev_data: *mut telem_device_data,
    pub request: wilco_ec_telem_request,
    pub response: [u8; TELEM_RESPONSE_SIZE],
    pub has_msg: bool,
}

//
// telem_open() - Callback for when the device node is opened.
// @inode: inode for this char device node.
// @filp: file for this char device node.
//
// We need to ensure that after writing a command to the device,
// the same userspace process reads the corresponding result.
// Therefore, we increment a refcount on opening the device, so that
// only one process can communicate with the EC at a time.
//
// Return: 0 on success, or negative error code on failure.
//
#[no_mangle]
unsafe extern "C" fn telem_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int telem_open(struct inode *inode, struct file *filp)
    {
    struct telem_device_data *dev_data;
    struct telem_session_data *sess_data;
// Ensure device isn't already open
    dev_data = container_of(inode.i_cdev, struct telem_device_data, cdev);
    if (atomic_cmpxchg(&dev_data.available, 1, 0) == 0)
    return -EBUSY;
    get_device(&dev_data.dev);
    sess_data = kzalloc_obj(*sess_data);
    if (!sess_data) {
    atomic_set(&dev_data.available, 1);
    return -ENOMEM;
    }
    sess_data.dev_data = dev_data;
    sess_data.has_msg = false;
    stream_open(inode, filp);
    filp.private_data = sess_data;
    return 0;
    }
    static ssize_t telem_write(struct file *filp, const char __user *buf,
    size_t count, loff_t *pos)
    {
    struct telem_session_data *sess_data = filp.private_data;
    let mut msg: wilco_ec_message = {};
    int ret;
    if (count > sizeof(sess_data.request))
    return -EMSGSIZE;
    memset(&sess_data.request, 0, sizeof(sess_data.request));
    if (copy_from_user(&sess_data.request, buf, count))
    return -EFAULT;
    ret = check_telem_request(&sess_data.request, count);
    if (ret < 0)
    return ret;
    memset(sess_data.response, 0, sizeof(sess_data.response));
    msg.type = WILCO_EC_MSG_TELEMETRY;
    msg.request_data = &sess_data.request;
    msg.request_size = sizeof(sess_data.request);
    msg.response_data = sess_data.response;
    msg.response_size = sizeof(sess_data.response);
    ret = wilco_ec_mailbox(sess_data.dev_data.ec, &msg);
    if (ret < 0)
    return ret;
    if (ret != sizeof(sess_data.response))
    return -EMSGSIZE;
    sess_data.has_msg = true;
    return count;
    }
    static ssize_t telem_read(struct file *filp, char __user *buf, size_t count,
    loff_t *pos)
    {
    struct telem_session_data *sess_data = filp.private_data;
    if (!sess_data.has_msg)
    return -ENODATA;
    if (count > sizeof(sess_data.response))
    return -EINVAL;
    if (copy_to_user(buf, sess_data.response, count))
    return -EFAULT;
    sess_data.has_msg = false;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn telem_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int telem_release(struct inode *inode, struct file *filp)
    {
    struct telem_session_data *sess_data = filp.private_data;
    atomic_set(&sess_data.dev_data.available, 1);
    put_device(&sess_data.dev_data.dev);
    kfree(sess_data);
    return 0;
    }
    static const struct file_operations telem_fops = {
    .open = telem_open,
    .write = telem_write,
    .read = telem_read,
    .release = telem_release,
    .owner = THIS_MODULE,
    };
//
// telem_device_free() - Callback to free the telem_device_data structure.
// @d: The device embedded in our device data, which we have been ref counting.
//
// Once all open file descriptors are closed and the device has been removed,
// the refcount of the device will fall to 0 and this will be called.
//
#[no_mangle]
unsafe extern "C" fn telem_device_free(d: *mut device) {
    static void telem_device_free(struct device *d)
    {
    struct telem_device_data *dev_data;
    dev_data = container_of(d, struct telem_device_data, dev);
    kfree(dev_data);
    }
//
// telem_device_probe() - Callback when creating a new device.
// @pdev: platform device that we will be receiving telems from.
//
// This finds a free minor number for the device, allocates and initializes
// some device data, and creates a new device and char dev node.
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
unsafe extern "C" fn telem_device_probe(pdev: *mut platform_device) -> c_int {
    static int telem_device_probe(struct platform_device *pdev)
    {
    struct telem_device_data *dev_data;
    int error, minor;
// Get the next available device number
    minor = ida_alloc_max(&telem_ida, TELEM_MAX_DEV-1, GFP_KERNEL);
    if (minor < 0) {
    error = minor;
    dev_err(&pdev.dev, "Failed to find minor number: %d\n", error);
    return error;
    }
    dev_data = kzalloc_obj(*dev_data);
    if (!dev_data) {
    ida_free(&telem_ida, minor);
    return -ENOMEM;
    }
// Initialize the device data
    dev_data.ec = dev_get_platdata(&pdev.dev);
    atomic_set(&dev_data.available, 1);
    platform_set_drvdata(pdev, dev_data);
// Initialize the device
    dev_data.dev.devt = MKDEV(telem_major, minor);
    dev_data.dev.class = &telem_class;
    dev_data.dev.release = telem_device_free;
    dev_set_name(&dev_data.dev, TELEM_DEV_NAME_FMT, minor);
    device_initialize(&dev_data.dev);
// Initialize the character device and add it to userspace
    cdev_init(&dev_data.cdev, &telem_fops);
    error = cdev_device_add(&dev_data.cdev, &dev_data.dev);
    if (error) {
    put_device(&dev_data.dev);
    ida_free(&telem_ida, minor);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn telem_device_remove(pdev: *mut platform_device) {
    static void telem_device_remove(struct platform_device *pdev)
    {
    struct telem_device_data *dev_data = platform_get_drvdata(pdev);
    cdev_device_del(&dev_data.cdev, &dev_data.dev);
    ida_free(&telem_ida, MINOR(dev_data.dev.devt));
    put_device(&dev_data.dev);
    }
    static const struct platform_device_id telem_id[] = {
    { .name = DRV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(platform, telem_id);
    static struct platform_driver telem_driver = {
    .probe = telem_device_probe,
    .remove = telem_device_remove,
    .driver = {
    .name = DRV_NAME,
    },
    .id_table = telem_id,
    };
#[no_mangle]
unsafe extern "C" fn telem_module_init() -> int __init {
    static int __init telem_module_init(void)
    {
    let mut dev_num: dev_t = 0;
    int ret;
    ret = class_register(&telem_class);
    if (ret) {
    pr_err(DRV_NAME ": Failed registering class: %d\n", ret);
    return ret;
    }
// Request the kernel for device numbers, starting with minor=0
    ret = alloc_chrdev_region(&dev_num, 0, TELEM_MAX_DEV, TELEM_DEV_NAME);
    if (ret) {
    pr_err(DRV_NAME ": Failed allocating dev numbers: %d\n", ret);
    goto destroy_class;
    }
    telem_major = MAJOR(dev_num);
    ret = platform_driver_register(&telem_driver);
    if (ret < 0) {
    pr_err(DRV_NAME ": Failed registering driver: %d\n", ret);
    goto unregister_region;
    }
    return 0;
    unregister_region:
    unregister_chrdev_region(MKDEV(telem_major, 0), TELEM_MAX_DEV);
    destroy_class:
    class_unregister(&telem_class);
    ida_destroy(&telem_ida);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn telem_module_exit() -> void __exit {
    static void __exit telem_module_exit(void)
    {
    platform_driver_unregister(&telem_driver);
    unregister_chrdev_region(MKDEV(telem_major, 0), TELEM_MAX_DEV);
    class_unregister(&telem_class);
    ida_destroy(&telem_ida);
    }
    module_init(telem_module_init);
    module_exit(telem_module_exit);
    MODULE_AUTHOR("Nick Crews <ncrews@chromium.org>");
    MODULE_DESCRIPTION("Wilco EC telemetry driver");
    MODULE_LICENSE("GPL");
