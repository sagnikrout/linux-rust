//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb/dvb-usb-firmware.c
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
// dvb-usb-firmware.c is part of the DVB USB library.
//
// Copyright (C) 2004-6 Patrick Boettcher (patrick.boettcher@posteo.de)
// see dvb-usb-init.c for copyright information.
//
// This file contains functions for downloading the firmware to Cypress FX 1 and 2 based devices.
//
// FIXME: This part does actually not belong to dvb-usb, but to the usb-subsystem.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cypress_controller {
    pub id: c_int,
    pub /: *const *const *const char name; / name of the usb controller,
    pub /: *mut *mut u16 cpu_cs_register; / needs to be restarted, when the firmware has been downloaded.,
}

    static struct usb_cypress_controller cypress[] = {
    { .id = DEVICE_SPECIFIC, .name = "Device specific", .cpu_cs_register = 0 },
    { .id = CYPRESS_AN2135,  .name = "Cypress AN2135",  .cpu_cs_register = 0x7f92 },
    { .id = CYPRESS_AN2235,  .name = "Cypress AN2235",  .cpu_cs_register = 0x7f92 },
    { .id = CYPRESS_FX2,     .name = "Cypress FX2",     .cpu_cs_register = 0xe600 },
    };
//
// load a firmware packet to the device
//
#[no_mangle]
unsafe extern "C" fn usb_cypress_writemem(udev: *mut usb_device, addr: u16, data: *mut u8, len: u8) -> c_int {
    static int usb_cypress_writemem(struct usb_device *udev,u16 addr,u8 *data, u8 len)
    {
    return usb_control_msg(udev, usb_sndctrlpipe(udev,0),
    0xa0, USB_TYPE_VENDOR, addr, 0x00, data, len, 5000);
    }
#[no_mangle]
pub unsafe extern "C" fn usb_cypress_load_firmware(udev: *mut usb_device, fw: *const firmware, type: c_int) -> c_int {
    int usb_cypress_load_firmware(struct usb_device *udev, const struct firmware *fw, int type)
    {
    struct hexline *hx;
    u8 *buf;
    int ret, pos = 0;
    let mut cpu_cs_register: u16 = cypress[type].cpu_cs_register;
    buf = kmalloc(sizeof(*hx), GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    hx = (struct hexline *)buf;
// stop the CPU
    buf[0] = 1;
    if (usb_cypress_writemem(udev, cpu_cs_register, buf, 1) != 1)
    err("could not stop the USB controller CPU.");
    while ((ret = dvb_usb_get_hexline(fw, hx, &pos)) > 0) {
    deb_fw("writing to address 0x%04x (buffer: 0x%02x %02x)\n", hx.addr, hx.len, hx.chk);
    ret = usb_cypress_writemem(udev, hx.addr, hx.data, hx.len);
    if (ret != hx.len) {
    err("error while transferring firmware (transferred size: %d, block size: %d)",
    ret, hx.len);
    ret = -EINVAL;
    break;
    }
    }
    if (ret < 0) {
    err("firmware download failed at %d with %d",pos,ret);
    kfree(buf);
    return ret;
    }
    if (ret == 0) {
// restart the CPU
    buf[0] = 0;
    if (usb_cypress_writemem(udev, cpu_cs_register, buf, 1) != 1) {
    err("could not restart the USB controller CPU.");
    ret = -EINVAL;
    }
    } else
    ret = -EIO;
    kfree(buf);
    return ret;
    }
    EXPORT_SYMBOL(usb_cypress_load_firmware);
    int dvb_usb_download_firmware(struct usb_device *udev,
    const struct dvb_usb_device_properties *props)
    {
    int ret;
    const struct firmware *fw = core::ptr::null_mut();
    if ((ret = request_firmware(&fw, props.firmware, &udev.dev)) != 0) {
    err("did not find the firmware file '%s' (status %d). You can use <kernel_dir>/scripts/get_dvb_firmware to get the firmware",
    props.firmware,ret);
    return ret;
    }
    info("downloading firmware from file '%s'",props.firmware);
    switch (props.usb_ctrl) {
    case CYPRESS_AN2135:
    case CYPRESS_AN2235:
    case CYPRESS_FX2:
    ret = usb_cypress_load_firmware(udev, fw, props.usb_ctrl);
    break;
    case DEVICE_SPECIFIC:
    if (props.download_firmware)
    ret = props.download_firmware(udev,fw);
    else {
    err("BUG: driver didn't specified a download_firmware-callback, although it claims to have a DEVICE_SPECIFIC one.");
    ret = -EINVAL;
    }
    break;
    default:
    ret = -EINVAL;
    break;
    }
    release_firmware(fw);
    return ret;
    }
    int dvb_usb_get_hexline(const struct firmware *fw, struct hexline *hx,
    int *pos)
    {
    u8 *b = (u8 *) &fw.data[*pos];
    let mut data_offs: c_int = 4;
    if (*pos >= fw.size)
    return 0;
    memset(hx,0,sizeof(struct hexline));
    hx.len  = b[0];
    if ((*pos + hx.len + 4) >= fw.size)
    return -EINVAL;
    hx.addr = b[1] | (b[2] << 8);
    hx.type = b[3];
    if (hx.type == 0x04) {
// b[4] and b[5] are the Extended linear address record data field
    hx.addr |= (b[4] << 24) | (b[5] << 16);
// hx->len -= 2;
    data_offs += 2; */
    }
    memcpy(hx.data,&b[data_offs],hx.len);
    hx.chk = b[hx.len + data_offs];
// pos += hx->len + 5;
    return *pos;
    }
    EXPORT_SYMBOL(dvb_usb_get_hexline);
