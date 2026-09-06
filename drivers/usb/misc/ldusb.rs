//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/ldusb.c
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
// Generic USB driver for report based interrupt in/out devices
// like LD Didactic's USB devices. LD Didactic's USB devices are
// HID devices which do not use HID report definitons (they use
// raw interrupt in and our reports only for communication).
//
// This driver uses a ring buffer for time critical reading of
// interrupt in reports and provides read and write methods for
// raw interrupt reports (similar to the Windows HID driver).
// Devices based on the book USB COMPLETE by Jan Axelson may need
// such a compatibility to the Windows HID driver.
//
// Copyright (C) 2005 Michael Hund <mhund@ld-didactic.de>
//
// Derived from Lego USB Tower driver
// Copyright (C) 2003 David Glance <advidgsf@sourceforge.net>
// 2001-2004 Juergen Stuber <starblue@users.sourceforge.net>
//

// Define these values to match your devices
pub const USB_VENDOR_ID_LD: c_uint = 0x0f11	/* USB Vendor ID of LD Didactic GmbH */;
pub const USB_DEVICE_ID_LD_CASSY: c_uint = 0x1000	/* USB Product ID of CASSY-S modules with 8 bytes endpoint size */;
pub const USB_DEVICE_ID_LD_CASSY2: c_uint = 0x1001	/* USB Product ID of CASSY-S modules with 64 bytes endpoint size */;
pub const USB_DEVICE_ID_LD_POCKETCASSY: c_uint = 0x1010	/* USB Product ID of Pocket-CASSY */;
pub const USB_DEVICE_ID_LD_POCKETCASSY2: c_uint = 0x1011	/* USB Product ID of Pocket-CASSY 2 (reserved) */;
pub const USB_DEVICE_ID_LD_MOBILECASSY: c_uint = 0x1020	/* USB Product ID of Mobile-CASSY */;
pub const USB_DEVICE_ID_LD_MOBILECASSY2: c_uint = 0x1021	/* USB Product ID of Mobile-CASSY 2 (reserved) */;
pub const USB_DEVICE_ID_LD_MICROCASSYVOLTAGE: c_uint = 0x1031	/* USB Product ID of Micro-CASSY Voltage */;
pub const USB_DEVICE_ID_LD_MICROCASSYCURRENT: c_uint = 0x1032	/* USB Product ID of Micro-CASSY Current */;
pub const USB_DEVICE_ID_LD_MICROCASSYTIME: c_uint = 0x1033	/* USB Product ID of Micro-CASSY Time (reserved) */;
pub const USB_DEVICE_ID_LD_MICROCASSYTEMPERATURE: c_uint = 0x1035	/* USB Product ID of Micro-CASSY Temperature */;
pub const USB_DEVICE_ID_LD_MICROCASSYPH: c_uint = 0x1038	/* USB Product ID of Micro-CASSY pH */;
pub const USB_DEVICE_ID_LD_POWERANALYSERCASSY: c_uint = 0x1040	/* USB Product ID of Power Analyser CASSY */;
pub const USB_DEVICE_ID_LD_CONVERTERCONTROLLERCASSY: c_uint = 0x1042	/* USB Product ID of Converter Controller CASSY */;
pub const USB_DEVICE_ID_LD_MACHINETESTCASSY: c_uint = 0x1043	/* USB Product ID of Machine Test CASSY */;
pub const USB_DEVICE_ID_LD_JWM: c_uint = 0x1080	/* USB Product ID of Joule and Wattmeter */;
pub const USB_DEVICE_ID_LD_DMMP: c_uint = 0x1081	/* USB Product ID of Digital Multimeter P (reserved) */;
pub const USB_DEVICE_ID_LD_UMIP: c_uint = 0x1090	/* USB Product ID of UMI P */;
pub const USB_DEVICE_ID_LD_UMIC: c_uint = 0x10A0	/* USB Product ID of UMI C */;
pub const USB_DEVICE_ID_LD_UMIB: c_uint = 0x10B0	/* USB Product ID of UMI B */;
pub const USB_DEVICE_ID_LD_XRAY: c_uint = 0x1100	/* USB Product ID of X-Ray Apparatus 55481 */;
pub const USB_DEVICE_ID_LD_XRAY2: c_uint = 0x1101	/* USB Product ID of X-Ray Apparatus 554800 */;
pub const USB_DEVICE_ID_LD_XRAYCT: c_uint = 0x1110	/* USB Product ID of X-Ray Apparatus CT 554821*/;
pub const USB_DEVICE_ID_LD_VIDEOCOM: c_uint = 0x1200	/* USB Product ID of VideoCom */;
pub const USB_DEVICE_ID_LD_MOTOR: c_uint = 0x1210	/* USB Product ID of Motor (reserved) */;
pub const USB_DEVICE_ID_LD_COM3LAB: c_uint = 0x2000	/* USB Product ID of COM3LAB */;
pub const USB_DEVICE_ID_LD_TELEPORT: c_uint = 0x2010	/* USB Product ID of Terminal Adapter */;
pub const USB_DEVICE_ID_LD_NETWORKANALYSER: c_uint = 0x2020	/* USB Product ID of Network Analyser */;
pub const USB_DEVICE_ID_LD_POWERCONTROL: c_uint = 0x2030	/* USB Product ID of Converter Control Unit */;
pub const USB_DEVICE_ID_LD_MACHINETEST: c_uint = 0x2040	/* USB Product ID of Machine Test System */;
pub const USB_DEVICE_ID_LD_MOSTANALYSER: c_uint = 0x2050	/* USB Product ID of MOST Protocol Analyser */;
pub const USB_DEVICE_ID_LD_MOSTANALYSER2: c_uint = 0x2051	/* USB Product ID of MOST Protocol Analyser 2 */;
pub const USB_DEVICE_ID_LD_ABSESP: c_uint = 0x2060	/* USB Product ID of ABS ESP */;
pub const USB_DEVICE_ID_LD_AUTODATABUS: c_uint = 0x2070	/* USB Product ID of Automotive Data Buses */;
pub const USB_DEVICE_ID_LD_MCT: c_uint = 0x2080	/* USB Product ID of Microcontroller technique */;
pub const USB_DEVICE_ID_LD_HYBRID: c_uint = 0x2090	/* USB Product ID of Automotive Hybrid */;
pub const USB_DEVICE_ID_LD_HEATCONTROL: c_uint = 0x20A0	/* USB Product ID of Heat control */;

pub const USB_LD_MINOR_BASE: c_int = 0;

pub const USB_LD_MINOR_BASE: c_int = 176;

// table of devices that work with this driver
    static const struct usb_device_id ld_usb_table[] = {
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_CASSY) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_CASSY2) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_POCKETCASSY) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_POCKETCASSY2) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MOBILECASSY) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MOBILECASSY2) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MICROCASSYVOLTAGE) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MICROCASSYCURRENT) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MICROCASSYTIME) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MICROCASSYTEMPERATURE) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MICROCASSYPH) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_POWERANALYSERCASSY) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_CONVERTERCONTROLLERCASSY) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MACHINETESTCASSY) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_JWM) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_DMMP) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_UMIP) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_UMIC) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_UMIB) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_XRAY) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_XRAY2) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_VIDEOCOM) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MOTOR) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_COM3LAB) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_TELEPORT) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_NETWORKANALYSER) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_POWERCONTROL) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MACHINETEST) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MOSTANALYSER) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MOSTANALYSER2) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_ABSESP) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_AUTODATABUS) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_MCT) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_HYBRID) },
    { USB_DEVICE(USB_VENDOR_ID_LD, USB_DEVICE_ID_LD_HEATCONTROL) },
    { }					/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, ld_usb_table);
    MODULE_AUTHOR("Michael Hund <mhund@ld-didactic.de>");
    MODULE_DESCRIPTION("LD USB Driver");
    MODULE_LICENSE("GPL");
// All interrupt in transfers are collected in a ring buffer to
// avoid racing conditions and get better performance of the driver.
//
    let mut ring_buffer_size: static int = 128;
    module_param(ring_buffer_size, int, 0000);
    MODULE_PARM_DESC(ring_buffer_size, "Read ring buffer size in reports");
// The write_buffer can contain more than one interrupt out transfer.
//
    let mut write_buffer_size: static int = 10;
    module_param(write_buffer_size, int, 0000);
    MODULE_PARM_DESC(write_buffer_size, "Write buffer size in reports");
// As of kernel version 2.6.4 ehci-hcd uses an
// "only one interrupt transfer per frame" shortcut
// to simplify the scheduling of periodic transfers.
// This conflicts with our standard 1ms intervals for in and out URBs.
// We use default intervals of 2ms for in and 2ms for out transfers,
// which should be fast enough.
// Increase the interval to allow more devices that do interrupt transfers,
// or set to 1 to use the standard interval from the endpoint descriptors.
//
    let mut min_interrupt_in_interval: static int = 2;
    module_param(min_interrupt_in_interval, int, 0000);
    MODULE_PARM_DESC(min_interrupt_in_interval, "Minimum interrupt in interval in ms");
    let mut min_interrupt_out_interval: static int = 2;
    module_param(min_interrupt_out_interval, int, 0000);
    MODULE_PARM_DESC(min_interrupt_out_interval, "Minimum interrupt out interval in ms");
// Structure to hold all of our device specific stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ld_usb {
    pub kref: kref,
    pub /: *mut *mut mutex mutex; / locks this structure,
    pub /: *mut *mut *mut usb_interface intf; / save off the usb interface pointer,
    pub disconnected:1: c_ulong,
    pub /: *mut *mut int open_count; / number of times this port has been opened,
    pub ring_buffer: *mut c_char,
    pub ring_head: c_uint,
    pub ring_tail: c_uint,
    pub read_wait: wait_queue_head_t,
    pub write_wait: wait_queue_head_t,
    pub interrupt_in_buffer: *mut c_char,
    pub interrupt_in_endpoint: *mut usb_endpoint_descriptor,
    pub interrupt_in_urb: *mut urb,
    pub interrupt_in_interval: c_int,
    pub interrupt_in_endpoint_size: usize,
    pub interrupt_in_running: c_int,
    pub interrupt_in_done: c_int,
    pub buffer_overflow: c_int,
    pub rbsl: spinlock_t,
    pub interrupt_out_buffer: *mut c_char,
    pub interrupt_out_endpoint: *mut usb_endpoint_descriptor,
    pub interrupt_out_urb: *mut urb,
    pub interrupt_out_interval: c_int,
    pub interrupt_out_endpoint_size: usize,
    pub interrupt_out_busy: c_int,
}

    static struct usb_driver ld_usb_driver;
//
// ld_usb_abort_transfers
// aborts transfers and frees associated data structures
//
#[no_mangle]
unsafe extern "C" fn ld_usb_abort_transfers(dev: *mut ld_usb) {
    static void ld_usb_abort_transfers(struct ld_usb *dev)
    {
// shutdown transfer
    if (dev.interrupt_in_running) {
    dev.interrupt_in_running = 0;
    usb_kill_urb(dev.interrupt_in_urb);
    }
    if (dev.interrupt_out_busy)
    usb_kill_urb(dev.interrupt_out_urb);
    }
//
// ld_usb_delete
//
#[no_mangle]
unsafe extern "C" fn ld_usb_delete(kref: *mut kref) {
    static void ld_usb_delete(struct kref *kref)
    {
    struct ld_usb *dev = container_of(kref, struct ld_usb, kref);
// free data structures
    usb_free_urb(dev.interrupt_in_urb);
    usb_free_urb(dev.interrupt_out_urb);
    kfree(dev.ring_buffer);
    kfree(dev.interrupt_in_buffer);
    kfree(dev.interrupt_out_buffer);
    kfree(dev);
    }
//
// ld_usb_interrupt_in_callback
//
#[no_mangle]
unsafe extern "C" fn ld_usb_interrupt_in_callback(urb: *mut urb) {
    static void ld_usb_interrupt_in_callback(struct urb *urb)
    {
    struct ld_usb *dev = urb.context;
    size_t *actual_buffer;
    unsigned int next_ring_head;
    let mut status: c_int = urb.status;
    unsigned long flags;
    int retval;
    if (status) {
    if (status == -ENOENT ||
    status == -ECONNRESET ||
    status == -ESHUTDOWN) {
    goto exit;
    } else {
    dev_dbg(&dev.intf.dev,
    "%s: nonzero status received: %d\n", __func__,
    status);
    spin_lock_irqsave(&dev.rbsl, flags);
    goto resubmit; /* maybe we can recover */
    }
    }
    spin_lock_irqsave(&dev.rbsl, flags);
    if (urb.actual_length > 0) {
    next_ring_head = (dev.ring_head+1) % ring_buffer_size;
    if (next_ring_head != dev.ring_tail) {
    actual_buffer = (size_t *)(dev.ring_buffer + dev.ring_head * (sizeof(size_t)+dev.interrupt_in_endpoint_size));
// actual_buffer gets urb->actual_length + interrupt_in_buffer
// actual_buffer = urb->actual_length;
    memcpy(actual_buffer+1, dev.interrupt_in_buffer, urb.actual_length);
    dev.ring_head = next_ring_head;
    dev_dbg(&dev.intf.dev, "%s: received %d bytes\n",
    __func__, urb.actual_length);
    } else {
    dev_warn(&dev.intf.dev,
    "Ring buffer overflow, %d bytes dropped\n",
    urb.actual_length);
    dev.buffer_overflow = 1;
    }
    }
    resubmit:
// resubmit if we're still running
    if (dev.interrupt_in_running && !dev.buffer_overflow) {
    retval = usb_submit_urb(dev.interrupt_in_urb, GFP_ATOMIC);
    if (retval) {
    dev_err(&dev.intf.dev,
    "usb_submit_urb failed (%d)\n", retval);
    dev.buffer_overflow = 1;
    }
    }
    spin_unlock_irqrestore(&dev.rbsl, flags);
    exit:
    dev.interrupt_in_done = 1;
    wake_up_interruptible(&dev.read_wait);
    }
//
// ld_usb_interrupt_out_callback
//
#[no_mangle]
unsafe extern "C" fn ld_usb_interrupt_out_callback(urb: *mut urb) {
    static void ld_usb_interrupt_out_callback(struct urb *urb)
    {
    struct ld_usb *dev = urb.context;
    let mut status: c_int = urb.status;
// sync/async unlink faults aren't errors
    if (status && !(status == -ENOENT ||
    status == -ECONNRESET ||
    status == -ESHUTDOWN))
    dev_dbg(&dev.intf.dev,
    "%s - nonzero write interrupt status received: %d\n",
    __func__, status);
    dev.interrupt_out_busy = 0;
    wake_up_interruptible(&dev.write_wait);
    }
//
// ld_usb_open
//
#[no_mangle]
unsafe extern "C" fn ld_usb_open(inode: *mut inode, file: *mut file) -> c_int {
    static int ld_usb_open(struct inode *inode, struct file *file)
    {
    struct ld_usb *dev;
    int subminor;
    int retval;
    struct usb_interface *interface;
    stream_open(inode, file);
    subminor = iminor(inode);
    interface = usb_find_interface(&ld_usb_driver, subminor);
    if (!interface) {
    printk(KERN_ERR "%s - error, can't find device for minor %d\n",
    __func__, subminor);
    return -ENODEV;
    }
    dev = usb_get_intfdata(interface);
    if (!dev)
    return -ENODEV;
// lock this device
    if (mutex_lock_interruptible(&dev.mutex))
    return -ERESTARTSYS;
// allow opening only once
    if (dev.open_count) {
    retval = -EBUSY;
    goto unlock_exit;
    }
    dev.open_count = 1;
// initialize in direction
    dev.ring_head = 0;
    dev.ring_tail = 0;
    dev.buffer_overflow = 0;
    usb_fill_int_urb(dev.interrupt_in_urb,
    interface_to_usbdev(interface),
    usb_rcvintpipe(interface_to_usbdev(interface),
    dev.interrupt_in_endpoint.bEndpointAddress),
    dev.interrupt_in_buffer,
    dev.interrupt_in_endpoint_size,
    ld_usb_interrupt_in_callback,
    dev,
    dev.interrupt_in_interval);
    dev.interrupt_in_running = 1;
    dev.interrupt_in_done = 0;
    retval = usb_submit_urb(dev.interrupt_in_urb, GFP_KERNEL);
    if (retval) {
    dev_err(&interface.dev, "Couldn't submit interrupt_in_urb %d\n", retval);
    dev.interrupt_in_running = 0;
    dev.open_count = 0;
    goto unlock_exit;
    }
    kref_get(&dev.kref);
// save device in the file's private structure
    file.private_data = dev;
    unlock_exit:
    mutex_unlock(&dev.mutex);
    return retval;
    }
//
// ld_usb_release
//
#[no_mangle]
unsafe extern "C" fn ld_usb_release(inode: *mut inode, file: *mut file) -> c_int {
    static int ld_usb_release(struct inode *inode, struct file *file)
    {
    struct ld_usb *dev;
    let mut retval: c_int = 0;
    dev = file.private_data;
    if (dev == core::ptr::null_mut()) {
    retval = -ENODEV;
    goto exit;
    }
    mutex_lock(&dev.mutex);
    if (dev.disconnected)
    goto unlock_exit;
// wait until write transfer is finished
    if (dev.interrupt_out_busy)
    wait_event_interruptible_timeout(dev.write_wait, !dev.interrupt_out_busy, 2 * HZ);
    ld_usb_abort_transfers(dev);
    dev.open_count = 0;
    unlock_exit:
    mutex_unlock(&dev.mutex);
    kref_put(&dev.kref, ld_usb_delete);
    exit:
    return retval;
    }
//
// ld_usb_poll
//
#[no_mangle]
unsafe extern "C" fn ld_usb_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t ld_usb_poll(struct file *file, poll_table *wait)
    {
    struct ld_usb *dev;
    let mut mask: __poll_t = 0;
    dev = file.private_data;
    if (dev.disconnected)
    return EPOLLERR | EPOLLHUP;
    poll_wait(file, &dev.read_wait, wait);
    poll_wait(file, &dev.write_wait, wait);
    if (dev.ring_head != dev.ring_tail)
    mask |= EPOLLIN | EPOLLRDNORM;
    if (!dev.interrupt_out_busy)
    mask |= EPOLLOUT | EPOLLWRNORM;
    return mask;
    }
//
// ld_usb_read
//
    static ssize_t ld_usb_read(struct file *file, char __user *buffer, size_t count,
    loff_t *ppos)
    {
    struct ld_usb *dev;
    size_t *actual_buffer;
    size_t bytes_to_read;
    let mut retval: c_int = 0;
    int rv;
    dev = file.private_data;
// verify that we actually have some data to read
    if (count == 0)
    goto exit;
// lock this object
    if (mutex_lock_interruptible(&dev.mutex)) {
    retval = -ERESTARTSYS;
    goto exit;
    }
// verify that the device wasn't unplugged
    if (dev.disconnected) {
    retval = -ENODEV;
    printk(KERN_ERR "ldusb: No device or device unplugged %d\n", retval);
    goto unlock_exit;
    }
// wait for data
    spin_lock_irq(&dev.rbsl);
    while (dev.ring_head == dev.ring_tail) {
    dev.interrupt_in_done = 0;
    spin_unlock_irq(&dev.rbsl);
    if (file.f_flags & O_NONBLOCK) {
    retval = -EAGAIN;
    goto unlock_exit;
    }
    retval = wait_event_interruptible(dev.read_wait, dev.interrupt_in_done);
    if (retval < 0)
    goto unlock_exit;
    spin_lock_irq(&dev.rbsl);
    }
    spin_unlock_irq(&dev.rbsl);
// actual_buffer contains actual_length + interrupt_in_buffer
    actual_buffer = (size_t *)(dev.ring_buffer + dev.ring_tail * (sizeof(size_t)+dev.interrupt_in_endpoint_size));
    if (*actual_buffer > dev.interrupt_in_endpoint_size) {
    retval = -EIO;
    goto unlock_exit;
    }
    bytes_to_read = min(count, *actual_buffer);
    if (bytes_to_read < *actual_buffer)
    dev_warn(&dev.intf.dev, "Read buffer overflow, %zu bytes dropped\n",
// actual_buffer-bytes_to_read);
// copy one interrupt_in_buffer from ring_buffer into userspace
    if (copy_to_user(buffer, actual_buffer+1, bytes_to_read)) {
    retval = -EFAULT;
    goto unlock_exit;
    }
    retval = bytes_to_read;
    spin_lock_irq(&dev.rbsl);
    dev.ring_tail = (dev.ring_tail + 1) % ring_buffer_size;
    if (dev.buffer_overflow) {
    dev.buffer_overflow = 0;
    spin_unlock_irq(&dev.rbsl);
    rv = usb_submit_urb(dev.interrupt_in_urb, GFP_KERNEL);
    if (rv < 0)
    dev.buffer_overflow = 1;
    } else {
    spin_unlock_irq(&dev.rbsl);
    }
    unlock_exit:
// unlock the device
    mutex_unlock(&dev.mutex);
    exit:
    return retval;
    }
//
// ld_usb_write
//
    static ssize_t ld_usb_write(struct file *file, const char __user *buffer,
    size_t count, loff_t *ppos)
    {
    struct ld_usb *dev;
    size_t bytes_to_write;
    let mut retval: c_int = 0;
    dev = file.private_data;
// verify that we actually have some data to write
    if (count == 0)
    goto exit;
// lock this object
    if (mutex_lock_interruptible(&dev.mutex)) {
    retval = -ERESTARTSYS;
    goto exit;
    }
// verify that the device wasn't unplugged
    if (dev.disconnected) {
    retval = -ENODEV;
    printk(KERN_ERR "ldusb: No device or device unplugged %d\n", retval);
    goto unlock_exit;
    }
// wait until previous transfer is finished
    if (dev.interrupt_out_busy) {
    if (file.f_flags & O_NONBLOCK) {
    retval = -EAGAIN;
    goto unlock_exit;
    }
    retval = wait_event_interruptible(dev.write_wait, !dev.interrupt_out_busy);
    if (retval < 0) {
    goto unlock_exit;
    }
    }
// write the data into interrupt_out_buffer from userspace
    bytes_to_write = min(count, write_buffer_size*dev.interrupt_out_endpoint_size);
    if (bytes_to_write < count)
    dev_warn(&dev.intf.dev, "Write buffer overflow, %zu bytes dropped\n",
    count - bytes_to_write);
    dev_dbg(&dev.intf.dev, "%s: count = %zu, bytes_to_write = %zu\n",
    __func__, count, bytes_to_write);
    if (copy_from_user(dev.interrupt_out_buffer, buffer, bytes_to_write)) {
    retval = -EFAULT;
    goto unlock_exit;
    }
    if (dev.interrupt_out_endpoint == core::ptr::null_mut()) {
// try HID_REQ_SET_REPORT=9 on control_endpoint instead of interrupt_out_endpoint
    retval = usb_control_msg(interface_to_usbdev(dev.intf),
    usb_sndctrlpipe(interface_to_usbdev(dev.intf), 0),
    9,
    USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_OUT,
    1 << 8, 0,
    dev.interrupt_out_buffer,
    bytes_to_write,
    USB_CTRL_SET_TIMEOUT);
    if (retval < 0)
    dev_err(&dev.intf.dev,
    "Couldn't submit HID_REQ_SET_REPORT %d\n",
    retval);
    goto unlock_exit;
    }
// send off the urb
    usb_fill_int_urb(dev.interrupt_out_urb,
    interface_to_usbdev(dev.intf),
    usb_sndintpipe(interface_to_usbdev(dev.intf),
    dev.interrupt_out_endpoint.bEndpointAddress),
    dev.interrupt_out_buffer,
    bytes_to_write,
    ld_usb_interrupt_out_callback,
    dev,
    dev.interrupt_out_interval);
    dev.interrupt_out_busy = 1;
    wmb();
    retval = usb_submit_urb(dev.interrupt_out_urb, GFP_KERNEL);
    if (retval) {
    dev.interrupt_out_busy = 0;
    dev_err(&dev.intf.dev,
    "Couldn't submit interrupt_out_urb %d\n", retval);
    goto unlock_exit;
    }
    retval = bytes_to_write;
    unlock_exit:
// unlock the device
    mutex_unlock(&dev.mutex);
    exit:
    return retval;
    }
// file operations needed when we register this driver
    static const struct file_operations ld_usb_fops = {
    .owner =	THIS_MODULE,
    .read  =	ld_usb_read,
    .write =	ld_usb_write,
    .open =		ld_usb_open,
    .release =	ld_usb_release,
    .poll =		ld_usb_poll,
    };
//
// usb class driver info in order to get a minor number from the usb core,
// and to have the device registered with the driver core
//
    static struct usb_class_driver ld_usb_class = {
    .name =		"ldusb%d",
    .fops =		&ld_usb_fops,
    .minor_base =	USB_LD_MINOR_BASE,
    };
//
// ld_usb_probe
//
// Called by the usb core when a new device is connected that it thinks
// this driver might be interested in.
//
#[no_mangle]
unsafe extern "C" fn ld_usb_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int ld_usb_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    struct ld_usb *dev = core::ptr::null_mut();
    struct usb_host_interface *iface_desc;
    char *buffer;
    let mut retval: c_int = -ENOMEM;
    int res;
// allocate memory for our device state and initialize it
    dev = kzalloc_obj(*dev);
    if (!dev)
    goto exit;
    kref_init(&dev.kref);
    mutex_init(&dev.mutex);
    spin_lock_init(&dev.rbsl);
    dev.intf = intf;
    init_waitqueue_head(&dev.read_wait);
    init_waitqueue_head(&dev.write_wait);
// workaround for early firmware versions on fast computers
    if ((le16_to_cpu(udev.descriptor.idVendor) == USB_VENDOR_ID_LD) &&
    ((le16_to_cpu(udev.descriptor.idProduct) == USB_DEVICE_ID_LD_CASSY) ||
    (le16_to_cpu(udev.descriptor.idProduct) == USB_DEVICE_ID_LD_COM3LAB)) &&
    (le16_to_cpu(udev.descriptor.bcdDevice) <= 0x103)) {
    buffer = kmalloc(256, GFP_KERNEL);
    if (!buffer)
    goto error;
// usb_string makes SETUP+STALL to leave always ControlReadLoop
    usb_string(udev, 255, buffer, 256);
    kfree(buffer);
    }
    iface_desc = intf.cur_altsetting;
    res = usb_find_last_int_in_endpoint(iface_desc,
    &dev.interrupt_in_endpoint);
    if (res) {
    dev_err(&intf.dev, "Interrupt in endpoint not found\n");
    retval = res;
    goto error;
    }
    res = usb_find_last_int_out_endpoint(iface_desc,
    &dev.interrupt_out_endpoint);
    if (res)
    dev_warn(&intf.dev, "Interrupt out endpoint not found (using control endpoint instead)\n");
    dev.interrupt_in_endpoint_size = usb_endpoint_maxp(dev.interrupt_in_endpoint);
    dev.ring_buffer = kcalloc(ring_buffer_size,
    sizeof(size_t) + dev.interrupt_in_endpoint_size,
    GFP_KERNEL);
    if (!dev.ring_buffer)
    goto error;
    dev.interrupt_in_buffer = kmalloc(dev.interrupt_in_endpoint_size, GFP_KERNEL);
    if (!dev.interrupt_in_buffer)
    goto error;
    dev.interrupt_in_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!dev.interrupt_in_urb)
    goto error;
    dev.interrupt_out_endpoint_size = dev.interrupt_out_endpoint ? usb_endpoint_maxp(dev.interrupt_out_endpoint) :
    udev.descriptor.bMaxPacketSize0;
    dev.interrupt_out_buffer =
    kmalloc_array(write_buffer_size,
    dev.interrupt_out_endpoint_size, GFP_KERNEL);
    if (!dev.interrupt_out_buffer)
    goto error;
    dev.interrupt_out_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!dev.interrupt_out_urb)
    goto error;
    dev.interrupt_in_interval = max_t(int, min_interrupt_in_interval,
    dev.interrupt_in_endpoint.bInterval);
    if (dev.interrupt_out_endpoint)
    dev.interrupt_out_interval = max_t(int, min_interrupt_out_interval,
    dev.interrupt_out_endpoint.bInterval);
// we can register the device now, as it is ready
    usb_set_intfdata(intf, dev);
    retval = usb_register_dev(intf, &ld_usb_class);
    if (retval) {
// something prevented us from registering this driver
    dev_err(&intf.dev, "Not able to get a minor for this device.\n");
    usb_set_intfdata(intf, core::ptr::null_mut());
    goto error;
    }
// let the user know what node this device is now attached to
    dev_info(&intf.dev, "LD USB Device #%d now attached to major %d minor %d\n",
    (intf.minor - USB_LD_MINOR_BASE), USB_MAJOR, intf.minor);
    exit:
    return retval;
    error:
    kref_put(&dev.kref, ld_usb_delete);
    return retval;
    }
//
// ld_usb_disconnect
//
// Called by the usb core when the device is removed from the system.
//
#[no_mangle]
unsafe extern "C" fn ld_usb_disconnect(intf: *mut usb_interface) {
    static void ld_usb_disconnect(struct usb_interface *intf)
    {
    struct ld_usb *dev;
    int minor;
    dev = usb_get_intfdata(intf);
    usb_set_intfdata(intf, core::ptr::null_mut());
    minor = intf.minor;
// give back our minor
    usb_deregister_dev(intf, &ld_usb_class);
    usb_poison_urb(dev.interrupt_in_urb);
    usb_poison_urb(dev.interrupt_out_urb);
    mutex_lock(&dev.mutex);
    dev.disconnected = 1;
    if (dev.open_count) {
// wake up pollers
    wake_up_interruptible_all(&dev.read_wait);
    wake_up_interruptible_all(&dev.write_wait);
    }
    mutex_unlock(&dev.mutex);
    kref_put(&dev.kref, ld_usb_delete);
    dev_info(&intf.dev, "LD USB Device #%d now disconnected\n",
    (minor - USB_LD_MINOR_BASE));
    }
// usb specific object needed to register this driver with the usb subsystem
    static struct usb_driver ld_usb_driver = {
    .name =		"ldusb",
    .probe =	ld_usb_probe,
    .disconnect =	ld_usb_disconnect,
    .id_table =	ld_usb_table,
    };
    module_usb_driver(ld_usb_driver);
