//! Automatically rewritten from C to Rust
//! Source: drivers/usb/image/mdc800.c
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
// copyright (C) 1999/2000 by Henning Zabel <henning@uni-paderborn.de>
//
// USB-Kernel Driver for the Mustek MDC800 Digital Camera
// (c) 1999/2000 Henning Zabel <henning@uni-paderborn.de>
//
// The driver brings the USB functions of the MDC800 to Linux.
// To use the Camera you must support the USB Protocol of the camera
// to the Kernel Node.
// The Driver uses a misc device Node. Create it with :
// mknod /dev/mustek c 180 32
//
// The driver supports only one camera.
//
// Fix: mdc800 used sleep_on and slept with io_lock held.
// Converted sleep_on to waitqueues with schedule_timeout and made io_lock
// a semaphore from a spinlock.
// by Oliver Neukum <oliver@neukum.name>
// (02/12/2001)
//
// Identify version on module load.
// (08/04/2001) gb
//
// version 0.7.5
// Fixed potential SMP races with Spinlocks.
// Thanks to Oliver Neukum <oliver@neukum.name> who
// noticed the race conditions.
// (30/10/2000)
//
// Fixed: Setting urb->dev before submitting urb.
// by Greg KH <greg@kroah.com>
// (13/10/2000)
//
// version 0.7.3
// bugfix : The mdc800->state field gets set to READY after the
// disconnect function sets it to NOT_CONNECTED. This makes the
// driver running like the camera is connected and causes some
// hang ups.
//
// version 0.7.1
// MOD_INC and MOD_DEC are changed in usb_probe to prevent load/unload
// problems when compiled as Module.
// (04/04/2000)
//
// The mdc800 driver gets assigned the USB Minor 32-47. The Registration
// was updated to use these values.
// (26/03/2000)
//
// The Init und Exit Module Function are updated.
// (01/03/2000)
//
// version 0.7.0
// Rewrite of the driver : The driver now uses URB's. The old stuff
// has been removed.
//
// version 0.6.0
// Rewrite of this driver: The Emulation of the rs232 protocoll
// has been removed from the driver. A special executeCommand function
// for this driver is included to gphoto.
// The driver supports two kind of communication to bulk endpoints.
// Either with the dev->bus->ops->bulk... or with callback function.
// (09/11/1999)
//
// version 0.5.0:
// first Version that gets a version number. Most of the needed
// functions work.
// (20/10/1999)
//

//
// Version Information
//

// Vendor and Product Information
pub const MDC800_VENDOR_ID: c_uint = 0x055f;
pub const MDC800_PRODUCT_ID: c_uint = 0xa800;
// Timeouts (msec)
pub const TO_DOWNLOAD_GET_READY: c_int = 1500;
pub const TO_DOWNLOAD_GET_BUSY: c_int = 1500;
pub const TO_WRITE_GET_READY: c_int = 1000;
pub const TO_DEFAULT_COMMAND: c_int = 5000;

// Minor Number of the device (create with mknod /dev/mustek c 180 32)
pub const MDC800_DEVICE_MINOR_BASE: c_int = 32;
//
    Data and structs
//
    typedef enum {
    NOT_CONNECTED, READY, WORKING, DOWNLOAD
    } mdc800_state;
// Data for the driver
    struct mdc800_data
    {
    struct usb_device *	dev;			// Device Data
    mdc800_state 		state;
    unsigned int		endpoint [4];
    struct urb *		irq_urb;
    wait_queue_head_t	irq_wait;
    int			irq_woken;
    char*			irq_urb_buffer;
    int			camera_busy;          // is camera busy ?
    int 			camera_request_ready; // Status to synchronize with irq
    char 			camera_response [8];  // last Bytes send after busy
    struct urb *   		write_urb;
    char*			write_urb_buffer;
    wait_queue_head_t	write_wait;
    int			written;
    struct urb *   		download_urb;
    char*			download_urb_buffer;
    wait_queue_head_t	download_wait;
    int			downloaded;
    int			download_left;		// Bytes left to download ?
// Device Data
    char			out [64];	// Answer Buffer
    int 			out_ptr;	// Index to the first not readen byte
    int			out_count;	// Bytes in the buffer
    int			open;		// Camera device open ?
    struct mutex		io_lock;	// IO -lock
    char 			in [8];		// Command Input Buffer
    int  			in_count;
    int			pic_index;	// Cache for the Imagesize (-1 for nothing cached )
    int			pic_len;
    int			minor;
    };
// Specification of the Endpoints
    static struct usb_endpoint_descriptor mdc800_ed [4] =
    {
    {
    .bLength = 		0,
    .bDescriptorType =	0,
    .bEndpointAddress =	0x01,
    .bmAttributes = 	0x02,
    .wMaxPacketSize =	cpu_to_le16(8),
    .bInterval = 		0,
    .bRefresh = 		0,
    .bSynchAddress = 	0,
    },
    {
    .bLength = 		0,
    .bDescriptorType = 	0,
    .bEndpointAddress = 	0x82,
    .bmAttributes = 	0x03,
    .wMaxPacketSize = 	cpu_to_le16(8),
    .bInterval = 		0,
    .bRefresh = 		0,
    .bSynchAddress = 	0,
    },
    {
    .bLength = 		0,
    .bDescriptorType = 	0,
    .bEndpointAddress = 	0x03,
    .bmAttributes = 	0x02,
    .wMaxPacketSize = 	cpu_to_le16(64),
    .bInterval = 		0,
    .bRefresh = 		0,
    .bSynchAddress = 	0,
    },
    {
    .bLength = 		0,
    .bDescriptorType = 	0,
    .bEndpointAddress = 	0x84,
    .bmAttributes = 	0x02,
    .wMaxPacketSize = 	cpu_to_le16(64),
    .bInterval = 		0,
    .bRefresh = 		0,
    .bSynchAddress = 	0,
    },
    };
// The Variable used by the driver
    static struct mdc800_data* mdc800;
//
    The USB Part of the driver
//
#[no_mangle]
unsafe extern "C" fn mdc800_endpoint_equals(a: *mut usb_endpoint_descriptor, b: *mut usb_endpoint_descriptor) -> c_int {
    static int mdc800_endpoint_equals (struct usb_endpoint_descriptor *a,struct usb_endpoint_descriptor *b)
    {
    return (
    ( a.bEndpointAddress == b.bEndpointAddress )
    && ( a.bmAttributes     == b.bmAttributes     )
    && ( a.wMaxPacketSize   == b.wMaxPacketSize   )
    );
    }
//
// Checks whether the camera responds busy
//
#[no_mangle]
unsafe extern "C" fn mdc800_isBusy(ch: *mut *mut c_char) -> c_int {
    static int mdc800_isBusy (char* ch)
    {
    let mut i: c_int = 0;
    while (i<8)
    {
    if (ch [i] != (char)0x99)
    return 0;
    i++;
    }
    return 1;
    }
//
// Checks whether the Camera is ready
//
#[no_mangle]
unsafe extern "C" fn mdc800_isReady(ch: *mut c_char) -> c_int {
    static int mdc800_isReady (char *ch)
    {
    let mut i: c_int = 0;
    while (i<8)
    {
    if (ch [i] != (char)0xbb)
    return 0;
    i++;
    }
    return 1;
    }
//
// USB IRQ Handler for InputLine
//
#[no_mangle]
unsafe extern "C" fn mdc800_usb_irq(urb: *mut urb) {
    static void mdc800_usb_irq (struct urb *urb)
    {
    let mut data_received: c_int = 0, wake_up;
    let mut b: *mut c_uchar = urb.transfer_buffer;
    let mut mdc800: *mut mdc800_data = urb.context;
    struct device *dev = &mdc800.dev.dev;
    let mut status: c_int = urb.status;
    if (status >= 0) {
    if (mdc800_isBusy (b))
    {
    if (!mdc800.camera_busy)
    {
    mdc800.camera_busy=1;
    dev_dbg(dev, "gets busy\n");
    }
    }
    else
    {
    if (mdc800.camera_busy && mdc800_isReady (b))
    {
    mdc800.camera_busy=0;
    dev_dbg(dev, "gets ready\n");
    }
    }
    if (!(mdc800_isBusy (b) || mdc800_isReady (b)))
    {
// Store Data in camera_answer field
    dev_dbg(dev, "%i %i %i %i %i %i %i %i \n",b[0],b[1],b[2],b[3],b[4],b[5],b[6],b[7]);
    memcpy (mdc800.camera_response,b,8);
    data_received=1;
    }
    }
    wake_up= ( mdc800.camera_request_ready > 0 )
    &&
    (
    ((mdc800.camera_request_ready == 1) && (!mdc800.camera_busy))
    ||
    ((mdc800.camera_request_ready == 2) && data_received)
    ||
    ((mdc800.camera_request_ready == 3) && (mdc800.camera_busy))
    ||
    (status < 0)
    );
    if (wake_up)
    {
    mdc800.camera_request_ready=0;
    mdc800.irq_woken=1;
    wake_up (&mdc800.irq_wait);
    }
    }
//
// Waits a while until the irq responds that camera is ready
//
// mode : 0: Wait for camera gets ready
// 1: Wait for receiving data
// 2: Wait for camera gets busy
//
// msec: Time to wait
//
#[no_mangle]
unsafe extern "C" fn mdc800_usb_waitForIRQ(mode: c_int, msec: c_int) -> c_int {
    static int mdc800_usb_waitForIRQ (int mode, int msec)
    {
    mdc800.camera_request_ready=1+mode;
    wait_event_timeout(mdc800.irq_wait, mdc800.irq_woken,
    msecs_to_jiffies(msec));
    mdc800.irq_woken = 0;
    if (mdc800.camera_request_ready>0)
    {
    mdc800.camera_request_ready=0;
    dev_err(&mdc800.dev.dev, "timeout waiting for camera.\n");
    return -1;
    }
    if (mdc800.state == NOT_CONNECTED)
    {
    printk(KERN_WARNING "mdc800: Camera gets disconnected "
    "during waiting for irq.\n");
    mdc800.camera_request_ready=0;
    return -2;
    }
    return 0;
    }
//
// The write_urb callback function
//
#[no_mangle]
unsafe extern "C" fn mdc800_usb_write_notify(urb: *mut urb) {
    static void mdc800_usb_write_notify (struct urb *urb)
    {
    let mut mdc800: *mut mdc800_data = urb.context;
    let mut status: c_int = urb.status;
    if (status != 0)
    dev_err(&mdc800.dev.dev,
    "writing command fails (status=%i)\n", status);
    else
    mdc800.state=READY;
    mdc800.written = 1;
    wake_up (&mdc800.write_wait);
    }
//
// The download_urb callback function
//
#[no_mangle]
unsafe extern "C" fn mdc800_usb_download_notify(urb: *mut urb) {
    static void mdc800_usb_download_notify (struct urb *urb)
    {
    let mut mdc800: *mut mdc800_data = urb.context;
    let mut status: c_int = urb.status;
    if (status == 0) {
// Fill output buffer with these data
    memcpy (mdc800.out,  urb.transfer_buffer, 64);
    mdc800.out_count=64;
    mdc800.out_ptr=0;
    mdc800.download_left-=64;
    if (mdc800.download_left == 0)
    {
    mdc800.state=READY;
    }
    } else {
    dev_err(&mdc800.dev.dev,
    "request bytes fails (status:%i)\n", status);
    }
    mdc800.downloaded = 1;
    wake_up (&mdc800.download_wait);
    }
//
    Probing for the Camera
//
    static struct usb_driver mdc800_usb_driver;
    static const struct file_operations mdc800_device_ops;
    static struct usb_class_driver mdc800_class = {
    .name =		"mdc800%d",
    .fops =		&mdc800_device_ops,
    .minor_base =	MDC800_DEVICE_MINOR_BASE,
    };
//
// Callback to search the Mustek MDC800 on the USB Bus
//
    static int mdc800_usb_probe (struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    int i,j;
    struct usb_host_interface *intf_desc;
    struct usb_device *dev = interface_to_usbdev (intf);
    let mut irq_interval: c_int = 0;
    int retval;
    dev_dbg(&intf.dev, "(%s) called.\n", __func__);
    if (mdc800.dev != core::ptr::null_mut())
    {
    dev_warn(&intf.dev, "only one Mustek MDC800 is supported.\n");
    return -ENODEV;
    }
    if (dev.descriptor.bNumConfigurations != 1)
    {
    dev_err(&intf.dev,
    "probe fails . wrong Number of Configuration\n");
    return -ENODEV;
    }
    intf_desc = intf.cur_altsetting;
    if (
    ( intf_desc.desc.bInterfaceClass != 0xff )
    ||	( intf_desc.desc.bInterfaceSubClass != 0 )
    || ( intf_desc.desc.bInterfaceProtocol != 0 )
    || ( intf_desc.desc.bNumEndpoints != 4)
    )
    {
    dev_err(&intf.dev, "probe fails . wrong Interface\n");
    return -ENODEV;
    }
// Check the Endpoints
    for (i=0; i<4; i++)
    {
    mdc800.endpoint[i]=-1;
    for (j=0; j<4; j++)
    {
    if (mdc800_endpoint_equals (&intf_desc.endpoint [j].desc,&mdc800_ed [i]))
    {
    mdc800.endpoint[i]=intf_desc.endpoint [j].desc.bEndpointAddress ;
    if (i==1)
    {
    irq_interval=intf_desc.endpoint [j].desc.bInterval;
    }
    }
    }
    if (mdc800.endpoint[i] == -1)
    {
    dev_err(&intf.dev, "probe fails . Wrong Endpoints.\n");
    return -ENODEV;
    }
    }
    dev_info(&intf.dev, "Found Mustek MDC800 on USB.\n");
    mutex_lock(&mdc800.io_lock);
    retval = usb_register_dev(intf, &mdc800_class);
    if (retval) {
    dev_err(&intf.dev, "Not able to get a minor for this device.\n");
    mutex_unlock(&mdc800.io_lock);
    return -ENODEV;
    }
    mdc800.dev=dev;
    mdc800.open=0;
// Setup URB Structs
    usb_fill_int_urb (
    mdc800.irq_urb,
    mdc800.dev,
    usb_rcvintpipe (mdc800.dev,mdc800.endpoint [1]),
    mdc800.irq_urb_buffer,
    8,
    mdc800_usb_irq,
    mdc800,
    irq_interval
    );
    usb_fill_bulk_urb (
    mdc800.write_urb,
    mdc800.dev,
    usb_sndbulkpipe (mdc800.dev, mdc800.endpoint[0]),
    mdc800.write_urb_buffer,
    8,
    mdc800_usb_write_notify,
    mdc800
    );
    usb_fill_bulk_urb (
    mdc800.download_urb,
    mdc800.dev,
    usb_rcvbulkpipe (mdc800.dev, mdc800.endpoint [3]),
    mdc800.download_urb_buffer,
    64,
    mdc800_usb_download_notify,
    mdc800
    );
    mdc800.state=READY;
    mutex_unlock(&mdc800.io_lock);
    usb_set_intfdata(intf, mdc800);
    return 0;
    }
//
// Disconnect USB device (maybe the MDC800)
//
#[no_mangle]
unsafe extern "C" fn mdc800_usb_disconnect(intf: *mut usb_interface) {
    static void mdc800_usb_disconnect (struct usb_interface *intf)
    {
    let mut mdc800: *mut mdc800_data = usb_get_intfdata(intf);
    dev_dbg(&intf.dev, "(%s) called\n", __func__);
    if (mdc800) {
    if (mdc800.state == NOT_CONNECTED)
    return;
    usb_deregister_dev(intf, &mdc800_class);
// must be under lock to make sure no URB
    is submitted after usb_kill_urb() */
    mutex_lock(&mdc800.io_lock);
    mdc800.state=NOT_CONNECTED;
    usb_kill_urb(mdc800.irq_urb);
    usb_kill_urb(mdc800.write_urb);
    usb_kill_urb(mdc800.download_urb);
    mutex_unlock(&mdc800.io_lock);
    mdc800.dev = core::ptr::null_mut();
    usb_set_intfdata(intf, core::ptr::null_mut());
    }
    dev_info(&intf.dev, "Mustek MDC800 disconnected from USB.\n");
    }
//
#[no_mangle]
pub unsafe extern "C" fn Part(_arg: file_operations) -> The Misc device {
    The Misc device Part (file_operations)
//
// This Function calc the Answersize for a command.
//
#[no_mangle]
unsafe extern "C" fn mdc800_getAnswerSize(command: c_char) -> c_int {
    static int mdc800_getAnswerSize (char command)
    {
    switch ((unsigned char) command)
    {
    case 0x2a:
    case 0x49:
    case 0x51:
    case 0x0d:
    case 0x20:
    case 0x07:
    case 0x01:
    case 0x25:
    case 0x00:
    return 8;
    case 0x05:
    case 0x3e:
    return mdc800.pic_len;
    case 0x09:
    return 4096;
    default:
    return 0;
    }
    }
//
// Init the device: (1) alloc mem (2) Increase MOD Count ..
//
#[no_mangle]
unsafe extern "C" fn mdc800_device_open(inode: *mut *mut inode, file: *mut file) -> c_int {
    static int mdc800_device_open (struct inode* inode, struct file *file)
    {
    let mut retval: c_int = 0;
    let mut errn: c_int = 0;
    mutex_lock(&mdc800.io_lock);
    if (mdc800.state == NOT_CONNECTED)
    {
    errn=-EBUSY;
    goto error_out;
    }
    if (mdc800.open)
    {
    errn=-EBUSY;
    goto error_out;
    }
    mdc800.in_count=0;
    mdc800.out_count=0;
    mdc800.out_ptr=0;
    mdc800.pic_index=0;
    mdc800.pic_len=-1;
    mdc800.download_left=0;
    mdc800.camera_busy=0;
    mdc800.camera_request_ready=0;
    mdc800.irq_urb.dev = mdc800.dev;
    retval = usb_submit_urb (mdc800.irq_urb, GFP_KERNEL);
    if (retval) {
    dev_err(&mdc800.dev.dev,
    "request USB irq fails (submit_retval=%i).\n", retval);
    errn = -EIO;
    goto error_out;
    }
    mdc800.open=1;
    dev_dbg(&mdc800.dev.dev, "Mustek MDC800 device opened.\n");
    error_out:
    mutex_unlock(&mdc800.io_lock);
    return errn;
    }
//
// Close the Camera and release Memory
//
#[no_mangle]
unsafe extern "C" fn mdc800_device_release(inode: *mut *mut inode, file: *mut file) -> c_int {
    static int mdc800_device_release (struct inode* inode, struct file *file)
    {
    let mut retval: c_int = 0;
    mutex_lock(&mdc800.io_lock);
    if (mdc800.open && (mdc800.state != NOT_CONNECTED))
    {
    usb_kill_urb(mdc800.irq_urb);
    usb_kill_urb(mdc800.write_urb);
    usb_kill_urb(mdc800.download_urb);
    mdc800.open=0;
    }
    else
    {
    retval=-EIO;
    }
    mutex_unlock(&mdc800.io_lock);
    return retval;
    }
//
// The Device read callback Function
//
#[no_mangle]
unsafe extern "C" fn mdc800_device_read(file: *mut file, buf: *mut char __user, len: usize, pos: *mut loff_t) -> isize {
    static ssize_t mdc800_device_read (struct file *file, char __user *buf, size_t len, loff_t *pos)
    {
    size_t left=len, sts=len; /* single transfer size */
    char __user *ptr = buf;
    int retval;
    mutex_lock(&mdc800.io_lock);
    if (mdc800.state == NOT_CONNECTED)
    {
    mutex_unlock(&mdc800.io_lock);
    return -EBUSY;
    }
    if (mdc800.state == WORKING)
    {
    printk(KERN_WARNING "mdc800: Illegal State \"working\""
    "reached during read ?!\n");
    mutex_unlock(&mdc800.io_lock);
    return -EBUSY;
    }
    if (!mdc800.open)
    {
    mutex_unlock(&mdc800.io_lock);
    return -EBUSY;
    }
    while (left)
    {
    if (signal_pending (current))
    {
    mutex_unlock(&mdc800.io_lock);
    let mut len: return = = left ? -EINTR : len-left;
    }
    sts=left > (mdc800.out_count-mdc800.out_ptr)?mdc800.out_count-mdc800.out_ptr:left;
    if (sts <= 0)
    {
// Too less Data in buffer
    if (mdc800.state == DOWNLOAD)
    {
    mdc800.out_count=0;
    mdc800.out_ptr=0;
// Download -> Request new bytes
    mdc800.download_urb.dev = mdc800.dev;
    retval = usb_submit_urb (mdc800.download_urb, GFP_KERNEL);
    if (retval) {
    dev_err(&mdc800.dev.dev,
    "Can't submit download urb "
    "(retval=%i)\n", retval);
    mutex_unlock(&mdc800.io_lock);
    return len-left;
    }
    retval = wait_event_timeout(mdc800.download_wait,
    mdc800.downloaded,
    msecs_to_jiffies(TO_DOWNLOAD_GET_READY));
    if (!retval)
    usb_kill_urb(mdc800.download_urb);
    mdc800.downloaded = 0;
    if (mdc800.download_urb.status != 0)
    {
    dev_err(&mdc800.dev.dev,
    "request download-bytes fails "
    "(status=%i)\n",
    mdc800.download_urb.status);
    mutex_unlock(&mdc800.io_lock);
    return len-left;
    }
    }
    else
    {
// No more bytes -> that's an error
    mutex_unlock(&mdc800.io_lock);
    return -EIO;
    }
    }
    else
    {
// Copy Bytes
    if (copy_to_user(ptr, &mdc800.out [mdc800.out_ptr],
    sts)) {
    mutex_unlock(&mdc800.io_lock);
    return -EFAULT;
    }
    ptr+=sts;
    left-=sts;
    mdc800.out_ptr+=sts;
    }
    }
    mutex_unlock(&mdc800.io_lock);
    return len-left;
    }
//
// The Device write callback Function
// If a 8Byte Command is received, it will be send to the camera.
// After this the driver initiates the request for the answer or
// just waits until the camera becomes ready.
//
#[no_mangle]
unsafe extern "C" fn mdc800_device_write(file: *mut file, buf: *const char __user, len: usize, pos: *mut loff_t) -> isize {
    static ssize_t mdc800_device_write (struct file *file, const char __user *buf, size_t len, loff_t *pos)
    {
    let mut i: usize = 0;
    int retval;
    mutex_lock(&mdc800.io_lock);
    if (mdc800.state != READY)
    {
    mutex_unlock(&mdc800.io_lock);
    return -EBUSY;
    }
    if (!mdc800.open )
    {
    mutex_unlock(&mdc800.io_lock);
    return -EBUSY;
    }
    while (i<len)
    {
    unsigned char c;
    if (signal_pending (current))
    {
    mutex_unlock(&mdc800.io_lock);
    return -EINTR;
    }
    if(get_user(c, buf+i))
    {
    mutex_unlock(&mdc800.io_lock);
    return -EFAULT;
    }
// check for command start
    if (c == 0x55)
    {
    mdc800.in_count=0;
    mdc800.out_count=0;
    mdc800.out_ptr=0;
    mdc800.download_left=0;
    }
// save command byte
    if (mdc800.in_count < 8)
    {
    mdc800.in[mdc800.in_count] = c;
    mdc800.in_count++;
    }
    else
    {
    mutex_unlock(&mdc800.io_lock);
    return -EIO;
    }
// Command Buffer full ? -> send it to camera
    if (mdc800.in_count == 8)
    {
    int answersize;
    if (mdc800_usb_waitForIRQ (0,TO_GET_READY))
    {
    dev_err(&mdc800.dev.dev,
    "Camera didn't get ready.\n");
    mutex_unlock(&mdc800.io_lock);
    return -EIO;
    }
    answersize=mdc800_getAnswerSize (mdc800.in[1]);
    mdc800.state=WORKING;
    memcpy (mdc800.write_urb.transfer_buffer, mdc800.in,8);
    mdc800.write_urb.dev = mdc800.dev;
    retval = usb_submit_urb (mdc800.write_urb, GFP_KERNEL);
    if (retval) {
    dev_err(&mdc800.dev.dev,
    "submitting write urb fails "
    "(retval=%i)\n", retval);
    mutex_unlock(&mdc800.io_lock);
    return -EIO;
    }
    wait_event_timeout(mdc800.write_wait, mdc800.written,
    msecs_to_jiffies(TO_WRITE_GET_READY));
    mdc800.written = 0;
    if (mdc800.state == WORKING)
    {
    usb_kill_urb(mdc800.write_urb);
    mutex_unlock(&mdc800.io_lock);
    return -EIO;
    }
    switch ((unsigned char) mdc800.in[1])
    {
    case 0x05: /* Download Image */
    case 0x3e: /* Take shot in Fine Mode (WCam Mode) */
    if (mdc800.pic_len < 0)
    {
    dev_err(&mdc800.dev.dev,
    "call 0x07 before "
    "0x05,0x3e\n");
    mdc800.state=READY;
    mutex_unlock(&mdc800.io_lock);
    return -EIO;
    }
    mdc800.pic_len=-1;
    fallthrough;
    case 0x09: /* Download Thumbnail */
    mdc800.download_left=answersize+64;
    mdc800.state=DOWNLOAD;
    mdc800_usb_waitForIRQ (0,TO_DOWNLOAD_GET_BUSY);
    break;
    default:
    if (answersize)
    {
    if (mdc800_usb_waitForIRQ (1,TO_READ_FROM_IRQ))
    {
    dev_err(&mdc800.dev.dev, "requesting answer from irq fails\n");
    mutex_unlock(&mdc800.io_lock);
    return -EIO;
    }
// Write dummy data, (this is ugly but part of the USB Protocol
// if you use endpoint 1 as bulk and not as irq)
    memcpy (mdc800.out, mdc800.camera_response,8);
// This is the interpreted answer
    memcpy (&mdc800.out[8], mdc800.camera_response,8);
    mdc800.out_ptr=0;
    mdc800.out_count=16;
// Cache the Imagesize, if command was getImageSize
    if (mdc800.in [1] == (char) 0x07)
    {
    mdc800.pic_len=(int) 65536*(unsigned char) mdc800.camera_response[0]+256*(unsigned char) mdc800.camera_response[1]+(unsigned char) mdc800.camera_response[2];
    dev_dbg(&mdc800.dev.dev, "cached imagesize = %i\n", mdc800.pic_len);
    }
    }
    else
    {
    if (mdc800_usb_waitForIRQ (0,TO_DEFAULT_COMMAND))
    {
    dev_err(&mdc800.dev.dev, "Command Timeout.\n");
    mutex_unlock(&mdc800.io_lock);
    return -EIO;
    }
    }
    mdc800.state=READY;
    break;
    }
    }
    i++;
    }
    mutex_unlock(&mdc800.io_lock);
    return i;
    }
//
#[no_mangle]
pub unsafe extern "C" fn driver(types: Structs and) -> Init and Cleanup this {
    Init and Cleanup this driver (Structs and types)
//
// File Operations of this drivers
    static const struct file_operations mdc800_device_ops =
    {
    .owner =	THIS_MODULE,
    .read =		mdc800_device_read,
    .write =	mdc800_device_write,
    .open =		mdc800_device_open,
    .release =	mdc800_device_release,
    .llseek =	noop_llseek,
    };
    static const struct usb_device_id mdc800_table[] = {
    { USB_DEVICE(MDC800_VENDOR_ID, MDC800_PRODUCT_ID) },
    { }						/* Terminating entry */
    };
    MODULE_DEVICE_TABLE (usb, mdc800_table);
//
// USB Driver Struct for this device
//
    static struct usb_driver mdc800_usb_driver =
    {
    .name =		"mdc800",
    .probe =	mdc800_usb_probe,
    .disconnect =	mdc800_usb_disconnect,
    .id_table =	mdc800_table
    };
//
#[no_mangle]
pub unsafe extern "C" fn driver(Functions: Main) -> Init and Cleanup this {
    Init and Cleanup this driver (Main Functions)
//
#[no_mangle]
unsafe extern "C" fn usb_mdc800_init() -> int __init {
    static int __init usb_mdc800_init (void)
    {
    let mut retval: c_int = -ENODEV;
// Allocate Memory
    mdc800=kzalloc_obj(struct mdc800_data);
    if (!mdc800)
    goto cleanup_on_fail;
    mdc800.dev = core::ptr::null_mut();
    mdc800.state=NOT_CONNECTED;
    mutex_init (&mdc800.io_lock);
    init_waitqueue_head (&mdc800.irq_wait);
    init_waitqueue_head (&mdc800.write_wait);
    init_waitqueue_head (&mdc800.download_wait);
    mdc800.irq_woken = 0;
    mdc800.downloaded = 0;
    mdc800.written = 0;
    mdc800.irq_urb_buffer=kmalloc (8, GFP_KERNEL);
    if (!mdc800.irq_urb_buffer)
    goto cleanup_on_fail;
    mdc800.write_urb_buffer=kmalloc (8, GFP_KERNEL);
    if (!mdc800.write_urb_buffer)
    goto cleanup_on_fail;
    mdc800.download_urb_buffer=kmalloc (64, GFP_KERNEL);
    if (!mdc800.download_urb_buffer)
    goto cleanup_on_fail;
    mdc800.irq_urb=usb_alloc_urb (0, GFP_KERNEL);
    if (!mdc800.irq_urb)
    goto cleanup_on_fail;
    mdc800.download_urb=usb_alloc_urb (0, GFP_KERNEL);
    if (!mdc800.download_urb)
    goto cleanup_on_fail;
    mdc800.write_urb=usb_alloc_urb (0, GFP_KERNEL);
    if (!mdc800.write_urb)
    goto cleanup_on_fail;
// Register the driver
    retval = usb_register(&mdc800_usb_driver);
    if (retval)
    goto cleanup_on_fail;
    printk(KERN_INFO KBUILD_MODNAME ": " DRIVER_VERSION ":"
    DRIVER_DESC "\n");
    return 0;
// Clean driver up, when something fails
    cleanup_on_fail:
    if (mdc800 != core::ptr::null_mut())
    {
    printk(KERN_ERR "mdc800: can't alloc memory!\n");
    kfree(mdc800.download_urb_buffer);
    kfree(mdc800.write_urb_buffer);
    kfree(mdc800.irq_urb_buffer);
    usb_free_urb(mdc800.write_urb);
    usb_free_urb(mdc800.download_urb);
    usb_free_urb(mdc800.irq_urb);
    kfree (mdc800);
    }
    mdc800 = core::ptr::null_mut();
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn usb_mdc800_cleanup() -> void __exit {
    static void __exit usb_mdc800_cleanup (void)
    {
    usb_deregister (&mdc800_usb_driver);
    usb_free_urb (mdc800.irq_urb);
    usb_free_urb (mdc800.download_urb);
    usb_free_urb (mdc800.write_urb);
    kfree (mdc800.irq_urb_buffer);
    kfree (mdc800.write_urb_buffer);
    kfree (mdc800.download_urb_buffer);
    kfree (mdc800);
    mdc800 = core::ptr::null_mut();
    }
    module_init (usb_mdc800_init);
    module_exit (usb_mdc800_cleanup);
    MODULE_AUTHOR( DRIVER_AUTHOR );
    MODULE_DESCRIPTION( DRIVER_DESC );
    MODULE_LICENSE("GPL");
