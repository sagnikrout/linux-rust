//! Automatically rewritten from C to Rust
//! Source: drivers/usb/storage/freecom.c
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
// Driver for Freecom USB/IDE adaptor
//
// Freecom v0.1:
//
// First release
//
// Current development and maintenance by:
// (C) 2000 David Brown <usb-storage@davidb.org>
//
// This driver was developed with information provided in FREECOM's USB
// Programmers Reference Guide.  For further information contact Freecom
// (https://www.freecom.de/)
//

    MODULE_DESCRIPTION("Driver for Freecom USB/IDE adaptor");
    MODULE_AUTHOR("David Brown <usb-storage@davidb.org>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("USB_STORAGE");

    static void pdump(struct us_data *us, void *ibuffer, int length);

// Bits of HD_STATUS
pub const ERR_STAT: c_uint = 0x01;
pub const DRQ_STAT: c_uint = 0x08;
// All of the outgoing packets are 64 bytes long.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct freecom_cb_wrap {
    pub /: *mut *mut u8 Type; / Command type.,
    pub /: *mut *mut u8 Timeout; / Timeout in seconds.,
    pub /: *mut *mut u8 Atapi[12]; / An ATAPI packet.,
    pub /: *mut *mut u8 Filler[50]; / Padding Data.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freecom_xfer_wrap {
    pub /: *mut *mut u8 Type; / Command type.,
    pub /: *mut *mut u8 Timeout; / Timeout in seconds.,
    pub /: *mut *mut __le32 Count; / Number of bytes to transfer.,
    pub Pad: [u8; 58],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct freecom_ide_out {
    pub /: *mut *mut u8 Type; / Type + IDE register.,
    pub Pad: u8,
    pub /: *mut *mut __le16 Value; / Value to write.,
    pub Pad2: [u8; 60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freecom_ide_in {
    pub /: *mut *mut u8 Type; / Type | IDE register.,
    pub Pad: [u8; 63],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freecom_status {
    pub Status: u8,
    pub Reason: u8,
    pub Count: __le16,
    pub Pad: [u8; 60],
}

//
// Freecom stuffs the interrupt status in the INDEX_STAT bit of the ide
// register.
//
pub const FCM_INT_STATUS: c_uint = 0x02 /* INDEX_STAT */;
pub const FCM_STATUS_BUSY: c_uint = 0x80;
//
// These are the packet types.  The low bit indicates that this command
// should wait for an interrupt.
//
pub const FCM_PACKET_ATAPI: c_uint = 0x21;
pub const FCM_PACKET_STATUS: c_uint = 0x20;
//
// Receive data from the IDE interface.  The ATAPI packet has already
// waited, so the data should be immediately available.
//
pub const FCM_PACKET_INPUT: c_uint = 0x81;
// Send data to the IDE interface.
pub const FCM_PACKET_OUTPUT: c_uint = 0x01;
//
// Write a value to an ide register.  Or the ide register to write after
// munging the address a bit.
//
pub const FCM_PACKET_IDE_WRITE: c_uint = 0x40;
pub const FCM_PACKET_IDE_READ: c_uint = 0xC0;
// All packets (except for status) are 64 bytes long.
pub const FCM_PACKET_LENGTH: c_int = 64;
pub const FCM_STATUS_PACKET_LENGTH: c_int = 4;
    static int init_freecom(struct us_data *us);
//
// The table of devices
//

    vendorName, productName, useProtocol, useTransport, \
    initFunction, flags) \
    { USB_DEVICE_VER(id_vendor, id_product, bcdDeviceMin, bcdDeviceMax), \
    .driver_info = (flags) }
    static const struct usb_device_id freecom_usb_ids[] = {

    { }		/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, freecom_usb_ids);

//
// The flags table
//

    vendor_name, product_name, use_protocol, use_transport, \
    init_function, Flags) \
    { \
    .vendorName = vendor_name,	\
    .productName = product_name,	\
    .useProtocol = use_protocol,	\
    .useTransport = use_transport,	\
    .initFunction = init_function,	\
    }
    static const struct us_unusual_dev freecom_unusual_dev_list[] = {

    { }		/* Terminating entry */
    };

    static int
    freecom_readdata (struct scsi_cmnd *srb, struct us_data *us,
    unsigned int ipipe, unsigned int opipe, int count)
    {
    struct freecom_xfer_wrap *fxfr =
    (struct freecom_xfer_wrap *) us.iobuf;
    int result;
    fxfr.Type = FCM_PACKET_INPUT | 0x00;
    fxfr.Timeout = 0;    /* Short timeout for debugging. */
    fxfr.Count = cpu_to_le32 (count);
    memset (fxfr.Pad, 0, sizeof (fxfr.Pad));
    usb_stor_dbg(us, "Read data Freecom! (c=%d)\n", count);
// Issue the transfer command.
    result = usb_stor_bulk_transfer_buf (us, opipe, fxfr,
    FCM_PACKET_LENGTH, core::ptr::null_mut());
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "Freecom readdata transport error\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
// Now transfer all of our blocks.
    usb_stor_dbg(us, "Start of read\n");
    result = usb_stor_bulk_srb(us, ipipe, srb);
    usb_stor_dbg(us, "freecom_readdata done!\n");
    if (result > USB_STOR_XFER_SHORT)
    return USB_STOR_TRANSPORT_ERROR;
    return USB_STOR_TRANSPORT_GOOD;
    }
    static int
    freecom_writedata (struct scsi_cmnd *srb, struct us_data *us,
    int unsigned ipipe, unsigned int opipe, int count)
    {
    struct freecom_xfer_wrap *fxfr =
    (struct freecom_xfer_wrap *) us.iobuf;
    int result;
    fxfr.Type = FCM_PACKET_OUTPUT | 0x00;
    fxfr.Timeout = 0;    /* Short timeout for debugging. */
    fxfr.Count = cpu_to_le32 (count);
    memset (fxfr.Pad, 0, sizeof (fxfr.Pad));
    usb_stor_dbg(us, "Write data Freecom! (c=%d)\n", count);
// Issue the transfer command.
    result = usb_stor_bulk_transfer_buf (us, opipe, fxfr,
    FCM_PACKET_LENGTH, core::ptr::null_mut());
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "Freecom writedata transport error\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
// Now transfer all of our blocks.
    usb_stor_dbg(us, "Start of write\n");
    result = usb_stor_bulk_srb(us, opipe, srb);
    usb_stor_dbg(us, "freecom_writedata done!\n");
    if (result > USB_STOR_XFER_SHORT)
    return USB_STOR_TRANSPORT_ERROR;
    return USB_STOR_TRANSPORT_GOOD;
    }
//
// Transport for the Freecom USB/IDE adaptor.
//
#[no_mangle]
unsafe extern "C" fn freecom_transport(srb: *mut scsi_cmnd, us: *mut us_data) -> c_int {
    static int freecom_transport(struct scsi_cmnd *srb, struct us_data *us)
    {
    struct freecom_cb_wrap *fcb;
    struct freecom_status  *fst;
    unsigned int ipipe, opipe;		/* We need both pipes. */
    int result;
    unsigned int partial;
    int length;
    fcb = (struct freecom_cb_wrap *) us.iobuf;
    fst = (struct freecom_status *) us.iobuf;
    usb_stor_dbg(us, "Freecom TRANSPORT STARTED\n");
// Get handles for both transports.
    opipe = us.send_bulk_pipe;
    ipipe = us.recv_bulk_pipe;
// The ATAPI Command always goes out first.
    fcb.Type = FCM_PACKET_ATAPI | 0x00;
    fcb.Timeout = 0;
    memcpy (fcb.Atapi, srb.cmnd, 12);
    memset (fcb.Filler, 0, sizeof (fcb.Filler));
    US_DEBUG(pdump(us, srb.cmnd, 12));
// Send it out.
    result = usb_stor_bulk_transfer_buf (us, opipe, fcb,
    FCM_PACKET_LENGTH, core::ptr::null_mut());
//
// The Freecom device will only fail if there is something wrong in
// USB land.  It returns the status in its own registers, which
// come back in the bulk pipe.
//
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "freecom transport error\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
//
// There are times we can optimize out this status read, but it
// doesn't hurt us to always do it now.
//
    result = usb_stor_bulk_transfer_buf (us, ipipe, fst,
    FCM_STATUS_PACKET_LENGTH, &partial);
    usb_stor_dbg(us, "foo Status result %d %u\n", result, partial);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    US_DEBUG(pdump(us, (void *)fst, partial));
//
// The firmware will time-out commands after 20 seconds. Some commands
// can legitimately take longer than this, so we use a different
// command that only waits for the interrupt and then sends status,
// without having to send a new ATAPI command to the device.
//
// NOTE: There is some indication that a data transfer after a timeout
// may not work, but that is a condition that should never happen.
//
    while (fst.Status & FCM_STATUS_BUSY) {
    usb_stor_dbg(us, "20 second USB/ATAPI bridge TIMEOUT occurred!\n");
    usb_stor_dbg(us, "fst.Status is %x\n", fst.Status);
// Get the status again
    fcb.Type = FCM_PACKET_STATUS;
    fcb.Timeout = 0;
    memset (fcb.Atapi, 0, sizeof(fcb.Atapi));
    memset (fcb.Filler, 0, sizeof (fcb.Filler));
// Send it out.
    result = usb_stor_bulk_transfer_buf (us, opipe, fcb,
    FCM_PACKET_LENGTH, core::ptr::null_mut());
//
// The Freecom device will only fail if there is something
// wrong in USB land.  It returns the status in its own
// registers, which come back in the bulk pipe.
//
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "freecom transport error\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
// get the data
    result = usb_stor_bulk_transfer_buf (us, ipipe, fst,
    FCM_STATUS_PACKET_LENGTH, &partial);
    usb_stor_dbg(us, "bar Status result %d %u\n", result, partial);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    US_DEBUG(pdump(us, (void *)fst, partial));
    }
    if (partial != 4)
    return USB_STOR_TRANSPORT_ERROR;
    if ((fst.Status & 1) != 0) {
    usb_stor_dbg(us, "operation failed\n");
    return USB_STOR_TRANSPORT_FAILED;
    }
//
// The device might not have as much data available as we
// requested.  If you ask for more than the device has, this reads
// and such will hang.
//
    usb_stor_dbg(us, "Device indicates that it has %d bytes available\n",
    le16_to_cpu(fst.Count));
    usb_stor_dbg(us, "SCSI requested %d\n", scsi_bufflen(srb));
// Find the length we desire to read.
    switch (srb.cmnd[0]) {
    case INQUIRY:
    case REQUEST_SENSE:	/* 16 or 18 bytes? spec says 18, lots of devices only have 16 */
    case MODE_SENSE:
    case MODE_SENSE_10:
    length = le16_to_cpu(fst.Count);
    break;
    default:
    length = scsi_bufflen(srb);
    }
// verify that this amount is legal
    if (length > scsi_bufflen(srb)) {
    length = scsi_bufflen(srb);
    usb_stor_dbg(us, "Truncating request to match buffer length: %d\n",
    length);
    }
//
// What we do now depends on what direction the data is supposed to
// move in.
//
    switch (us.srb.sc_data_direction) {
    case DMA_FROM_DEVICE:
// catch bogus "read 0 length" case
    if (!length)
    break;
//
// Make sure that the status indicates that the device
// wants data as well.
//
    if ((fst.Status & DRQ_STAT) == 0 || (fst.Reason & 3) != 2) {
    usb_stor_dbg(us, "SCSI wants data, drive doesn't have any\n");
    return USB_STOR_TRANSPORT_FAILED;
    }
    result = freecom_readdata (srb, us, ipipe, opipe, length);
    if (result != USB_STOR_TRANSPORT_GOOD)
    return result;
    usb_stor_dbg(us, "Waiting for status\n");
    result = usb_stor_bulk_transfer_buf (us, ipipe, fst,
    FCM_PACKET_LENGTH, &partial);
    US_DEBUG(pdump(us, (void *)fst, partial));
    if (partial != 4 || result > USB_STOR_XFER_SHORT)
    return USB_STOR_TRANSPORT_ERROR;
    if ((fst.Status & ERR_STAT) != 0) {
    usb_stor_dbg(us, "operation failed\n");
    return USB_STOR_TRANSPORT_FAILED;
    }
    if ((fst.Reason & 3) != 3) {
    usb_stor_dbg(us, "Drive seems still hungry\n");
    return USB_STOR_TRANSPORT_FAILED;
    }
    usb_stor_dbg(us, "Transfer happy\n");
    break;
    case DMA_TO_DEVICE:
// catch bogus "write 0 length" case
    if (!length)
    break;
//
// Make sure the status indicates that the device wants to
// send us data.
//
// !!IMPLEMENT!!
    result = freecom_writedata (srb, us, ipipe, opipe, length);
    if (result != USB_STOR_TRANSPORT_GOOD)
    return result;
    usb_stor_dbg(us, "Waiting for status\n");
    result = usb_stor_bulk_transfer_buf (us, ipipe, fst,
    FCM_PACKET_LENGTH, &partial);
    if (partial != 4 || result > USB_STOR_XFER_SHORT)
    return USB_STOR_TRANSPORT_ERROR;
    if ((fst.Status & ERR_STAT) != 0) {
    usb_stor_dbg(us, "operation failed\n");
    return USB_STOR_TRANSPORT_FAILED;
    }
    if ((fst.Reason & 3) != 3) {
    usb_stor_dbg(us, "Drive seems still hungry\n");
    return USB_STOR_TRANSPORT_FAILED;
    }
    usb_stor_dbg(us, "Transfer happy\n");
    break;
    case DMA_NONE:
// Easy, do nothing.
    break;
    default:
// should never hit here -- filtered in usb.c
    usb_stor_dbg(us, "freecom unimplemented direction: %d\n",
    us.srb.sc_data_direction);
// Return fail, SCSI seems to handle this better.
    return USB_STOR_TRANSPORT_FAILED;
    }
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn init_freecom(us: *mut us_data) -> c_int {
    static int init_freecom(struct us_data *us)
    {
    int result;
    char *buffer = us.iobuf;
//
// The DMA-mapped I/O buffer is 64 bytes long, just right for
// all our packets.  No need to allocate any extra buffer space.
//
    result = usb_stor_control_msg(us, us.recv_ctrl_pipe,
    0x4c, 0xc0, 0x4346, 0x0, buffer, 0x20, 3*HZ);
    buffer[32] = '\0';
    usb_stor_dbg(us, "String returned from FC init is: %s\n", buffer);
//
// Special thanks to the people at Freecom for providing me with
// this "magic sequence", which they use in their Windows and MacOS
// drivers to make sure that all the attached perhiperals are
// properly reset.
//
// send reset
    result = usb_stor_control_msg(us, us.send_ctrl_pipe,
    0x4d, 0x40, 0x24d8, 0x0, core::ptr::null_mut(), 0x0, 3*HZ);
    usb_stor_dbg(us, "result from activate reset is %d\n", result);
// wait 250ms
    msleep(250);
// clear reset
    result = usb_stor_control_msg(us, us.send_ctrl_pipe,
    0x4d, 0x40, 0x24f8, 0x0, core::ptr::null_mut(), 0x0, 3*HZ);
    usb_stor_dbg(us, "result from clear reset is %d\n", result);
// wait 3 seconds
    msleep(3 * 1000);
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn usb_stor_freecom_reset(us: *mut us_data) -> c_int {
    static int usb_stor_freecom_reset(struct us_data *us)
    {
    printk (KERN_CRIT "freecom reset called\n");
// We don't really have this feature.
    return FAILED;
    }

#[no_mangle]
unsafe extern "C" fn pdump(us: *mut us_data, ibuffer: *mut c_void, length: c_int) {
    static void pdump(struct us_data *us, void *ibuffer, int length)
    {
    static char line[80];
    let mut offset: c_int = 0;
    unsigned char *buffer = (unsigned char *) ibuffer;
    int i, j;
    int from, base;
    offset = 0;
    for (i = 0; i < length; i++) {
    if ((i & 15) == 0) {
    if (i > 0) {
    offset += sprintf (line+offset, " - ");
    for (j = i - 16; j < i; j++) {
    if (buffer[j] >= 32 && buffer[j] <= 126)
    line[offset++] = buffer[j];
    else
    line[offset++] = '.';
    }
    line[offset] = 0;
    usb_stor_dbg(us, "%s\n", line);
    offset = 0;
    }
    offset += sprintf (line+offset, "%08x:", i);
    } else if ((i & 7) == 0) {
    offset += sprintf (line+offset, " -");
    }
    offset += sprintf (line+offset, " %02x", buffer[i] & 0xff);
    }
// Add the last "chunk" of data.
    from = (length - 1) % 16;
    base = ((length - 1) / 16) * 16;
    for (i = from + 1; i < 16; i++)
    offset += sprintf (line+offset, "   ");
    if (from < 8)
    offset += sprintf (line+offset, "  ");
    offset += sprintf (line+offset, " - ");
    for (i = 0; i <= from; i++) {
    if (buffer[base+i] >= 32 && buffer[base+i] <= 126)
    line[offset++] = buffer[base+i];
    else
    line[offset++] = '.';
    }
    line[offset] = 0;
    usb_stor_dbg(us, "%s\n", line);
    }

    static struct scsi_host_template freecom_host_template;
    static int freecom_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct us_data *us;
    int result;
    result = usb_stor_probe1(&us, intf, id,
    (id - freecom_usb_ids) + freecom_unusual_dev_list,
    &freecom_host_template);
    if (result)
    return result;
    us.transport_name = "Freecom";
    us.transport = freecom_transport;
    us.transport_reset = usb_stor_freecom_reset;
    us.max_lun = 0;
    result = usb_stor_probe2(us);
    return result;
    }
    static struct usb_driver freecom_driver = {
    .name =		DRV_NAME,
    .probe =	freecom_probe,
    .disconnect =	usb_stor_disconnect,
    .suspend =	usb_stor_suspend,
    .resume =	usb_stor_resume,
    .reset_resume =	usb_stor_reset_resume,
    .pre_reset =	usb_stor_pre_reset,
    .post_reset =	usb_stor_post_reset,
    .id_table =	freecom_usb_ids,
    .soft_unbind =	1,
    .no_dynamic_id = 1,
    };
    module_usb_stor_driver(freecom_driver, freecom_host_template, DRV_NAME);
