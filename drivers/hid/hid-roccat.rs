//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-roccat.c
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
// Roccat driver for Linux
//
// Copyright (c) 2010 Stefan Achatz <erazor_de@users.sourceforge.net>
//
// Module roccat is a char device used to report special events of roccat
// hardware to userland. These events include requests for on-screen-display of
// profile or dpi settings or requests for execution of macro sequences that are
// not stored in device. The information in these events depends on hid device
// implementation and contains data that is not available in a single hid event
// or else hidraw could have been used.
// It is inspired by hidraw, but uses only one circular buffer for all readers.
//

pub const ROCCAT_FIRST_MINOR: c_int = 0;
pub const ROCCAT_MAX_DEVICES: c_int = 8;
// should be a power of 2 for performance reason
pub const ROCCAT_CBUF_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roccat_report {
    pub value: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct roccat_device {
    pub minor: c_uint,
    pub report_size: c_int,
    pub open: c_int,
    pub exist: c_int,
    pub wait: wait_queue_head_t,
    pub dev: *mut device,
    pub hid: *mut hid_device,
    pub readers: list_head,
// protects modifications of readers list
    pub readers_lock: mutex,
//
// circular_buffer has one writer and multiple readers with their own
// read pointers
//
    pub cbuf: [roccat_report; ROCCAT_CBUF_SIZE],
    pub cbuf_end: c_int,
    pub cbuf_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct roccat_reader {
    pub node: list_head,
    pub device: *mut roccat_device,
    pub cbuf_start: c_int,
}

    static int roccat_major;
    static struct cdev roccat_cdev;
    static struct roccat_device *devices[ROCCAT_MAX_DEVICES];
// protects modifications of devices array
    static DEFINE_MUTEX(devices_lock);
#[no_mangle]
unsafe extern "C" fn roccat_free_device(device: *mut roccat_device) {
    static void roccat_free_device(struct roccat_device *device)
    {
    int i;
    for (i = 0; i < ROCCAT_CBUF_SIZE; i++)
    kfree(device.cbuf[i].value);
    kfree(device);
    }
    static ssize_t roccat_read(struct file *file, char __user *buffer,
    size_t count, loff_t *ppos)
    {
    struct roccat_reader *reader = file.private_data;
    struct roccat_device *device = reader.device;
    struct roccat_report *report;
    let mut retval: isize = 0, len;
    DECLARE_WAITQUEUE(wait, current);
    mutex_lock(&device.cbuf_lock);
// no data?
    if (reader.cbuf_start == device.cbuf_end) {
    add_wait_queue(&device.wait, &wait);
    set_current_state(TASK_INTERRUPTIBLE);
// wait for data
    while (reader.cbuf_start == device.cbuf_end) {
    if (file.f_flags & O_NONBLOCK) {
    retval = -EAGAIN;
    break;
    }
    if (signal_pending(current)) {
    retval = -ERESTARTSYS;
    break;
    }
    if (!device.exist) {
    retval = -EIO;
    break;
    }
    mutex_unlock(&device.cbuf_lock);
    schedule();
    mutex_lock(&device.cbuf_lock);
    set_current_state(TASK_INTERRUPTIBLE);
    }
    set_current_state(TASK_RUNNING);
    remove_wait_queue(&device.wait, &wait);
    }
// here we either have data or a reason to return if retval is set
    if (retval)
    goto exit_unlock;
    report = &device.cbuf[reader.cbuf_start];
//
// If report is larger than requested amount of data, rest of report
// is lost!
//
    len = device.report_size > count ? count : device.report_size;
    if (copy_to_user(buffer, report.value, len)) {
    retval = -EFAULT;
    goto exit_unlock;
    }
    retval += len;
    reader.cbuf_start = (reader.cbuf_start + 1) % ROCCAT_CBUF_SIZE;
    exit_unlock:
    mutex_unlock(&device.cbuf_lock);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn roccat_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t roccat_poll(struct file *file, poll_table *wait)
    {
    struct roccat_reader *reader = file.private_data;
    poll_wait(file, &reader.device.wait, wait);
    if (reader.cbuf_start != reader.device.cbuf_end)
    return EPOLLIN | EPOLLRDNORM;
    if (!reader.device.exist)
    return EPOLLERR | EPOLLHUP;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn roccat_open(inode: *mut inode, file: *mut file) -> c_int {
    static int roccat_open(struct inode *inode, struct file *file)
    {
    let mut minor: c_uint = iminor(inode);
    struct roccat_reader *reader;
    struct roccat_device *device;
    let mut error: c_int = 0;
    reader = kzalloc_obj(struct roccat_reader);
    if (!reader)
    return -ENOMEM;
    mutex_lock(&devices_lock);
    device = devices[minor];
    if (!device) {
    pr_emerg("roccat device with minor %d doesn't exist\n", minor);
    error = -ENODEV;
    goto exit_err_devices;
    }
    mutex_lock(&device.readers_lock);
    if (!device.open++) {
// power on device on adding first reader
    error = hid_hw_power(device.hid, PM_HINT_FULLON);
    if (error < 0) {
    --device.open;
    goto exit_err_readers;
    }
    error = hid_hw_open(device.hid);
    if (error < 0) {
    hid_hw_power(device.hid, PM_HINT_NORMAL);
    --device.open;
    goto exit_err_readers;
    }
    }
    reader.device = device;
// new reader doesn't get old events
    reader.cbuf_start = device.cbuf_end;
    list_add_tail(&reader.node, &device.readers);
    file.private_data = reader;
    exit_err_readers:
    mutex_unlock(&device.readers_lock);
    exit_err_devices:
    mutex_unlock(&devices_lock);
    if (error)
    kfree(reader);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn roccat_release(inode: *mut inode, file: *mut file) -> c_int {
    static int roccat_release(struct inode *inode, struct file *file)
    {
    let mut minor: c_uint = iminor(inode);
    struct roccat_reader *reader = file.private_data;
    struct roccat_device *device;
    mutex_lock(&devices_lock);
    device = devices[minor];
    if (!device) {
    mutex_unlock(&devices_lock);
    pr_emerg("roccat device with minor %d doesn't exist\n", minor);
    return -ENODEV;
    }
    mutex_lock(&device.readers_lock);
    list_del(&reader.node);
    mutex_unlock(&device.readers_lock);
    kfree(reader);
    if (!--device.open) {
// removing last reader
    if (device.exist) {
    hid_hw_power(device.hid, PM_HINT_NORMAL);
    hid_hw_close(device.hid);
    } else {
    roccat_free_device(device);
    }
    }
    mutex_unlock(&devices_lock);
    return 0;
    }
//
// roccat_report_event() - output data to readers
// @minor: minor device number returned by roccat_connect()
// @data: pointer to data
//
// Return value is zero on success, a negative error code on failure.
//
// This is called from interrupt handler.
//
#[no_mangle]
pub unsafe extern "C" fn roccat_report_event(minor: c_int, data: *const u8) -> c_int {
    int roccat_report_event(int minor, u8 const *data)
    {
    struct roccat_device *device;
    struct roccat_reader *reader;
    struct roccat_report *report;
    uint8_t *new_value;
    device = devices[minor];
    new_value = kmemdup(data, device.report_size, GFP_ATOMIC);
    if (!new_value)
    return -ENOMEM;
    mutex_lock(&device.readers_lock);
    mutex_lock(&device.cbuf_lock);
    report = &device.cbuf[device.cbuf_end];
// passing NULL is safe
    kfree(report.value);
    report.value = new_value;
    device.cbuf_end = (device.cbuf_end + 1) % ROCCAT_CBUF_SIZE;
    list_for_each_entry(reader, &device.readers, node) {
//
// As we already inserted one element, the buffer can't be
// empty. If start and end are equal, buffer is full and we
// increase start, so that slow reader misses one event, but
// gets the newer ones in the right order.
//
    if (reader.cbuf_start == device.cbuf_end)
    reader.cbuf_start = (reader.cbuf_start + 1) % ROCCAT_CBUF_SIZE;
    }
    mutex_unlock(&device.cbuf_lock);
    mutex_unlock(&device.readers_lock);
    wake_up_interruptible(&device.wait);
    return 0;
    }
    EXPORT_SYMBOL_GPL(roccat_report_event);
//
// roccat_connect() - create a char device for special event output
// @class: the class thats used to create the device. Meant to hold device
// specific sysfs attributes.
// @hid: the hid device the char device should be connected to.
// @report_size: size of reports
//
// Return value is minor device number in Range [0, ROCCAT_MAX_DEVICES] on
// success, a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn roccat_connect(klass: *const class, hid: *mut hid_device, report_size: c_int) -> c_int {
    int roccat_connect(const struct class *klass, struct hid_device *hid, int report_size)
    {
    unsigned int minor;
    struct roccat_device *device;
    int temp;
    device = kzalloc_obj(struct roccat_device);
    if (!device)
    return -ENOMEM;
    mutex_lock(&devices_lock);
    for (minor = 0; minor < ROCCAT_MAX_DEVICES; ++minor) {
    if (devices[minor])
    continue;
    break;
    }
    if (minor < ROCCAT_MAX_DEVICES) {
    devices[minor] = device;
    } else {
    mutex_unlock(&devices_lock);
    kfree(device);
    return -EINVAL;
    }
    device.dev = device_create(klass, &hid.dev,
    MKDEV(roccat_major, minor), core::ptr::null_mut(),
    "%s%s%d", "roccat", hid.driver.name, minor);
    if (IS_ERR(device.dev)) {
    devices[minor] = core::ptr::null_mut();
    mutex_unlock(&devices_lock);
    temp = PTR_ERR(device.dev);
    kfree(device);
    return temp;
    }
    mutex_unlock(&devices_lock);
    init_waitqueue_head(&device.wait);
    INIT_LIST_HEAD(&device.readers);
    mutex_init(&device.readers_lock);
    mutex_init(&device.cbuf_lock);
    device.minor = minor;
    device.hid = hid;
    device.exist = 1;
    device.cbuf_end = 0;
    device.report_size = report_size;
    return minor;
    }
    EXPORT_SYMBOL_GPL(roccat_connect);
// roccat_disconnect() - remove char device from hid device
// @minor: the minor device number returned by roccat_connect()
//
#[no_mangle]
pub unsafe extern "C" fn roccat_disconnect(minor: c_int) {
    void roccat_disconnect(int minor)
    {
    struct roccat_device *device;
    mutex_lock(&devices_lock);
    device = devices[minor];
    mutex_unlock(&devices_lock);
    device.exist = 0; /* TODO exist maybe not needed */
    device_destroy(device.dev.class, MKDEV(roccat_major, minor));
    mutex_lock(&devices_lock);
    devices[minor] = core::ptr::null_mut();
    mutex_unlock(&devices_lock);
    if (device.open) {
    hid_hw_close(device.hid);
    wake_up_interruptible(&device.wait);
    } else {
    roccat_free_device(device);
    }
    }
    EXPORT_SYMBOL_GPL(roccat_disconnect);
#[no_mangle]
unsafe extern "C" fn roccat_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long roccat_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct inode *inode = file_inode(file);
    struct roccat_device *device;
    let mut minor: c_uint = iminor(inode);
    let mut retval: c_long = 0;
    mutex_lock(&devices_lock);
    device = devices[minor];
    if (!device) {
    retval = -ENODEV;
    goto out;
    }
    switch (cmd) {
    case ROCCATIOCGREPSIZE:
    if (put_user(device.report_size, (int __user *)arg))
    retval = -EFAULT;
    break;
    default:
    retval = -ENOTTY;
    }
    out:
    mutex_unlock(&devices_lock);
    return retval;
    }
    static const struct file_operations roccat_ops = {
    .owner = THIS_MODULE,
    .read = roccat_read,
    .poll = roccat_poll,
    .open = roccat_open,
    .release = roccat_release,
    .llseek = noop_llseek,
    .unlocked_ioctl = roccat_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn roccat_init() -> int __init {
    static int __init roccat_init(void)
    {
    int retval;
    dev_t dev_id;
    retval = alloc_chrdev_region(&dev_id, ROCCAT_FIRST_MINOR,
    ROCCAT_MAX_DEVICES, "roccat");
    if (retval < 0) {
    pr_warn("can't get major number\n");
    goto error;
    }
    roccat_major = MAJOR(dev_id);
    cdev_init(&roccat_cdev, &roccat_ops);
    retval = cdev_add(&roccat_cdev, dev_id, ROCCAT_MAX_DEVICES);
    if (retval < 0) {
    pr_warn("cannot add cdev\n");
    goto cleanup_alloc_chrdev_region;
    }
    return 0;
    cleanup_alloc_chrdev_region:
    unregister_chrdev_region(dev_id, ROCCAT_MAX_DEVICES);
    error:
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn roccat_exit() -> void __exit {
    static void __exit roccat_exit(void)
    {
    let mut dev_id: dev_t = MKDEV(roccat_major, 0);
    cdev_del(&roccat_cdev);
    unregister_chrdev_region(dev_id, ROCCAT_MAX_DEVICES);
    }
    module_init(roccat_init);
    module_exit(roccat_exit);
    MODULE_AUTHOR("Stefan Achatz");
    MODULE_DESCRIPTION("USB Roccat char device");
    MODULE_LICENSE("GPL v2");
