//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/emi26.c
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
// Emagic EMI 2|6 usb audio interface firmware loader.
// Copyright (C) 2002
// Tapio Laxström (tapio.laxstrom@iptime.fi)
//
// emi26.c,v 1.13 2002/03/08 13:10:26 tapio Exp
//

pub const EMI26_VENDOR_ID: c_uint = 0x086a  /* Emagic Soft-und Hardware GmBH */;
pub const EMI26_PRODUCT_ID: c_uint = 0x0100	/* EMI 2|6 without firmware */;
pub const EMI26B_PRODUCT_ID: c_uint = 0x0102	/* EMI 2|6 without firmware */;
pub const ANCHOR_LOAD_INTERNAL: c_uint = 0xA0	/* Vendor specific request code for Anchor Upload/Download (This one is implemented in the core) */;
pub const ANCHOR_LOAD_EXTERNAL: c_uint = 0xA3	/* This command is not implemented in the core. Requires firmware */;
pub const ANCHOR_LOAD_FPGA: c_uint = 0xA5	/* This command is not implemented in the core. Requires firmware. Emagic extension */;
pub const MAX_INTERNAL_ADDRESS: c_uint = 0x1B3F	/* This is the highest internal RAM address for the AN2131Q */;
pub const CPUCS_REG: c_uint = 0x7F92  /* EZ-USB Control and Status Register.  Bit 0 controls 8051 reset */;

    static int emi26_writememory( struct usb_device *dev, int address,
    const unsigned char *data, int length,
    __u8 bRequest);
    static int emi26_set_reset(struct usb_device *dev, unsigned char reset_bit);
    static int emi26_load_firmware (struct usb_device *dev);
    static int emi26_probe(struct usb_interface *intf, const struct usb_device_id *id);
    static void emi26_disconnect(struct usb_interface *intf);
// thanks to drivers/usb/serial/keyspan_pda.c code
    static int emi26_writememory (struct usb_device *dev, int address,
    const unsigned char *data, int length,
    __u8 request)
    {
    int result;
    unsigned char *buffer =  kmemdup(data, length, GFP_KERNEL);
    if (!buffer) {
    dev_err(&dev.dev, "kmalloc(%d) failed.\n", length);
    return -ENOMEM;
    }
// Note: usb_control_msg returns negative value on error or length of the
// data that was written!
    result = usb_control_msg (dev, usb_sndctrlpipe(dev, 0), request, 0x40, address, 0, buffer, length, 300);
    kfree (buffer);
    return result;
    }
// thanks to drivers/usb/serial/keyspan_pda.c code
#[no_mangle]
unsafe extern "C" fn emi26_set_reset(dev: *mut usb_device, reset_bit: c_uchar) -> c_int {
    static int emi26_set_reset (struct usb_device *dev, unsigned char reset_bit)
    {
    int response;
    dev_info(&dev.dev, "%s - %d\n", __func__, reset_bit);
// printk(KERN_DEBUG "%s - %d", __func__, reset_bit);
    response = emi26_writememory (dev, CPUCS_REG, &reset_bit, 1, 0xa0);
    if (response < 0) {
    dev_err(&dev.dev, "set_reset (%d) failed\n", reset_bit);
    }
    return response;
    }
pub const FW_LOAD_SIZE: c_int = 1023;
#[no_mangle]
unsafe extern "C" fn emi26_load_firmware(dev: *mut usb_device) -> c_int {
    static int emi26_load_firmware (struct usb_device *dev)
    {
    const struct firmware *loader_fw = core::ptr::null_mut();
    const struct firmware *bitstream_fw = core::ptr::null_mut();
    const struct firmware *firmware_fw = core::ptr::null_mut();
    const struct ihex_binrec *rec;
    let mut err: c_int = -ENOMEM;
    int i;
    __u32 addr;	/* Address to write */
    __u8 *buf;
    buf = kmalloc(FW_LOAD_SIZE, GFP_KERNEL);
    if (!buf)
    goto wraperr;
    err = request_ihex_firmware(&loader_fw, "emi26/loader.fw", &dev.dev);
    if (err)
    goto nofw;
    err = request_ihex_firmware(&bitstream_fw, "emi26/bitstream.fw",
    &dev.dev);
    if (err)
    goto nofw;
    err = request_ihex_firmware(&firmware_fw, "emi26/firmware.fw",
    &dev.dev);
    if (err) {
    nofw:
    dev_err(&dev.dev, "%s - request_firmware() failed\n",
    __func__);
    goto wraperr;
    }
// Assert reset (stop the CPU in the EMI)
    err = emi26_set_reset(dev,1);
    if (err < 0)
    goto wraperr;
    rec = (const struct ihex_binrec *)loader_fw.data;
// 1. We need to put the loader for the FPGA into the EZ-USB
    while (rec) {
    err = emi26_writememory(dev, be32_to_cpu(rec.addr),
    rec.data, be16_to_cpu(rec.len),
    ANCHOR_LOAD_INTERNAL);
    if (err < 0)
    goto wraperr;
    rec = ihex_next_binrec(rec);
    }
// De-assert reset (let the CPU run)
    err = emi26_set_reset(dev,0);
    if (err < 0)
    goto wraperr;
    msleep(250);	/* let device settle */
// 2. We upload the FPGA firmware into the EMI
// Note: collect up to 1023 (yes!) bytes and send them with
// a single request. This is _much_ faster!
    rec = (const struct ihex_binrec *)bitstream_fw.data;
    do {
    i = 0;
    addr = be32_to_cpu(rec.addr);
// intel hex records are terminated with type 0 element
    while (rec && (i + be16_to_cpu(rec.len) < FW_LOAD_SIZE)) {
    memcpy(buf + i, rec.data, be16_to_cpu(rec.len));
    i += be16_to_cpu(rec.len);
    rec = ihex_next_binrec(rec);
    }
    err = emi26_writememory(dev, addr, buf, i, ANCHOR_LOAD_FPGA);
    if (err < 0)
    goto wraperr;
    } while (rec);
// Assert reset (stop the CPU in the EMI)
    err = emi26_set_reset(dev,1);
    if (err < 0)
    goto wraperr;
// 3. We need to put the loader for the firmware into the EZ-USB (again...)
    for (rec = (const struct ihex_binrec *)loader_fw.data;
    rec; rec = ihex_next_binrec(rec)) {
    err = emi26_writememory(dev, be32_to_cpu(rec.addr),
    rec.data, be16_to_cpu(rec.len),
    ANCHOR_LOAD_INTERNAL);
    if (err < 0)
    goto wraperr;
    }
    msleep(250);	/* let device settle */
// De-assert reset (let the CPU run)
    err = emi26_set_reset(dev,0);
    if (err < 0)
    goto wraperr;
// 4. We put the part of the firmware that lies in the external RAM into the EZ-USB
    for (rec = (const struct ihex_binrec *)firmware_fw.data;
    rec; rec = ihex_next_binrec(rec)) {
    if (!INTERNAL_RAM(be32_to_cpu(rec.addr))) {
    err = emi26_writememory(dev, be32_to_cpu(rec.addr),
    rec.data, be16_to_cpu(rec.len),
    ANCHOR_LOAD_EXTERNAL);
    if (err < 0)
    goto wraperr;
    }
    }
// Assert reset (stop the CPU in the EMI)
    err = emi26_set_reset(dev,1);
    if (err < 0)
    goto wraperr;
    for (rec = (const struct ihex_binrec *)firmware_fw.data;
    rec; rec = ihex_next_binrec(rec)) {
    if (INTERNAL_RAM(be32_to_cpu(rec.addr))) {
    err = emi26_writememory(dev, be32_to_cpu(rec.addr),
    rec.data, be16_to_cpu(rec.len),
    ANCHOR_LOAD_INTERNAL);
    if (err < 0)
    goto wraperr;
    }
    }
// De-assert reset (let the CPU run)
    err = emi26_set_reset(dev,0);
    if (err < 0)
    goto wraperr;
    msleep(250);	/* let device settle */
// return 1 to fail the driver inialization
// and give real driver change to load
    err = 1;
    wraperr:
    if (err < 0)
    dev_err(&dev.dev,"%s - error loading firmware: error = %d\n",
    __func__, err);
    release_firmware(loader_fw);
    release_firmware(bitstream_fw);
    release_firmware(firmware_fw);
    kfree(buf);
    return err;
    }
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(EMI26_VENDOR_ID, EMI26_PRODUCT_ID) },
    { USB_DEVICE(EMI26_VENDOR_ID, EMI26B_PRODUCT_ID) },
    { }                                             /* Terminating entry */
    };
    MODULE_DEVICE_TABLE (usb, id_table);
#[no_mangle]
unsafe extern "C" fn emi26_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int emi26_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    struct usb_device *dev = interface_to_usbdev(intf);
    dev_info(&intf.dev, "%s start\n", __func__);
    emi26_load_firmware(dev);
// do not return the driver context, let real audio driver do that
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn emi26_disconnect(intf: *mut usb_interface) {
    static void emi26_disconnect(struct usb_interface *intf)
    {
    }
    static struct usb_driver emi26_driver = {
    .name		= "emi26 - firmware loader",
    .probe		= emi26_probe,
    .disconnect	= emi26_disconnect,
    .id_table	= id_table,
    };
    module_usb_driver(emi26_driver);
    MODULE_AUTHOR("Tapio Laxström");
    MODULE_DESCRIPTION("Emagic EMI 2|6 firmware loader.");
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE("emi26/loader.fw");
    MODULE_FIRMWARE("emi26/bitstream.fw");
    MODULE_FIRMWARE("emi26/firmware.fw");
// vi:ai:syntax=c:sw=8:ts=8:tw=80
//
