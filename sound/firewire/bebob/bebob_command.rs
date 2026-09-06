//! Automatically rewritten from C to Rust
//! Source: sound/firewire/bebob/bebob_command.c
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
// bebob_command.c - driver for BeBoB based devices
//
// Copyright (c) 2013-2014 Takashi Sakamoto
//

    int avc_audio_set_selector(struct fw_unit *unit, unsigned int subunit_id,
    unsigned int fb_id, unsigned int num)
    {
    u8 *buf;
    int err;
    buf = kzalloc(12, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
    buf[0]  = 0x00;		/* AV/C CONTROL */
    buf[1]  = 0x08 | (0x07 & subunit_id);	/* AUDIO SUBUNIT ID */
    buf[2]  = 0xb8;		/* FUNCTION BLOCK  */
    buf[3]  = 0x80;		/* type is 'selector'*/
    buf[4]  = 0xff & fb_id;	/* function block id */
    buf[5]  = 0x10;		/* control attribute is CURRENT */
    buf[6]  = 0x02;		/* selector length is 2 */
    buf[7]  = 0xff & num;	/* input function block plug number */
    buf[8]  = 0x01;		/* control selector is SELECTOR_CONTROL */
    err = fcp_avc_transaction(unit, buf, 12, buf, 12,
    BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) |
    BIT(6) | BIT(7) | BIT(8));
    if (err < 0)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(9: err <) -> else {
    else if (err < 9)
    err = -EIO;
    else if (buf[0] == 0x08) /* NOT IMPLEMENTED */
    err = -ENOSYS;
    else if (buf[0] == 0x0a) /* REJECTED */
    err = -EINVAL;
    else
    err = 0;
    kfree(buf);
    return err;
    }
    int avc_audio_get_selector(struct fw_unit *unit, unsigned int subunit_id,
    unsigned int fb_id, unsigned int *num)
    {
    u8 *buf;
    int err;
    buf = kzalloc(12, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
    buf[0]  = 0x01;		/* AV/C STATUS */
    buf[1]  = 0x08 | (0x07 & subunit_id);	/* AUDIO SUBUNIT ID */
    buf[2]  = 0xb8;		/* FUNCTION BLOCK */
    buf[3]  = 0x80;		/* type is 'selector'*/
    buf[4]  = 0xff & fb_id;	/* function block id */
    buf[5]  = 0x10;		/* control attribute is CURRENT */
    buf[6]  = 0x02;		/* selector length is 2 */
    buf[7]  = 0xff;		/* input function block plug number */
    buf[8]  = 0x01;		/* control selector is SELECTOR_CONTROL */
    err = fcp_avc_transaction(unit, buf, 12, buf, 12,
    BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) |
    BIT(6) | BIT(8));
    if (err < 0)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(9: err <) -> else {
    else if (err < 9)
    err = -EIO;
    else if (buf[0] == 0x08) /* NOT IMPLEMENTED */
    err = -ENOSYS;
    else if (buf[0] == 0x0a) /* REJECTED */
    err = -EINVAL;
    else if (buf[0] == 0x0b) /* IN TRANSITION */
    err = -EAGAIN;
    if (err < 0)
    goto end;
// num = buf[7];
    err = 0;
    end:
    kfree(buf);
    return err;
    }
    static inline void
    avc_bridgeco_fill_extension_addr(u8 *buf, u8 *addr)
    {
    buf[1] = addr[0];
    memcpy(buf + 4, addr + 1, 5);
    }
    static inline void
    avc_bridgeco_fill_plug_info_extension_command(u8 *buf, u8 *addr,
    unsigned int itype)
    {
    buf[0] = 0x01;	/* AV/C STATUS */
    buf[2] = 0x02;	/* AV/C GENERAL PLUG INFO */
    buf[3] = 0xc0;	/* BridgeCo extension */
    avc_bridgeco_fill_extension_addr(buf, addr);
    buf[9] = itype;	/* info type */
    }
    int avc_bridgeco_get_plug_type(struct fw_unit *unit,
    u8 addr[AVC_BRIDGECO_ADDR_BYTES],
    enum avc_bridgeco_plug_type *type)
    {
    u8 *buf;
    int err;
    buf = kzalloc(12, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
// Info type is 'plug type'.
    avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x00);
    err = fcp_avc_transaction(unit, buf, 12, buf, 12,
    BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) |
    BIT(6) | BIT(7) | BIT(9));
    if (err < 0)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(11: err <) -> else {
    else if (err < 11)
    err = -EIO;
    else if (buf[0] == 0x08) /* NOT IMPLEMENTED */
    err = -ENOSYS;
    else if (buf[0] == 0x0a) /* REJECTED */
    err = -EINVAL;
    else if (buf[0] == 0x0b) /* IN TRANSITION */
    err = -EAGAIN;
    if (err < 0)
    goto end;
// type = buf[10];
    err = 0;
    end:
    kfree(buf);
    return err;
    }
    int avc_bridgeco_get_plug_ch_count(struct fw_unit *unit, u8 addr[AVC_BRIDGECO_ADDR_BYTES],
    unsigned int *ch_count)
    {
    u8 *buf;
    int err;
    buf = kzalloc(12, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
// Info type is 'plug type'.
    avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x02);
    err = fcp_avc_transaction(unit, buf, 12, buf, 12,
    BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) |
    BIT(6) | BIT(7) | BIT(9));
    if (err < 0)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(11: err <) -> else {
    else if (err < 11)
    err = -EIO;
    else if (buf[0] == 0x08) // NOT IMPLEMENTED
    err = -ENOSYS;
    else if (buf[0] == 0x0a) // REJECTED
    err = -EINVAL;
    else if (buf[0] == 0x0b) // IN TRANSITION
    err = -EAGAIN;
    if (err < 0)
    goto end;
// ch_count = buf[10];
    err = 0;
    end:
    kfree(buf);
    return err;
    }
    int avc_bridgeco_get_plug_ch_pos(struct fw_unit *unit,
    u8 addr[AVC_BRIDGECO_ADDR_BYTES],
    u8 *buf, unsigned int len)
    {
    int err;
// Info type is 'channel position'.
    avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x03);
    err = fcp_avc_transaction(unit, buf, 12, buf, 256,
    BIT(1) | BIT(2) | BIT(3) | BIT(4) |
    BIT(5) | BIT(6) | BIT(7) | BIT(9));
    if (err < 0)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(11: err <) -> else {
    else if (err < 11)
    err = -EIO;
    else if (buf[0] == 0x08) /* NOT IMPLEMENTED */
    err = -ENOSYS;
    else if (buf[0] == 0x0a) /* REJECTED */
    err = -EINVAL;
    else if (buf[0] == 0x0b) /* IN TRANSITION */
    err = -EAGAIN;
    if (err < 0)
    goto end;
// Pick up specific data.
    memmove(buf, buf + 10, err - 10);
    err = 0;
    end:
    return err;
    }
    int avc_bridgeco_get_plug_section_type(struct fw_unit *unit,
    u8 addr[AVC_BRIDGECO_ADDR_BYTES],
    unsigned int id, u8 *type)
    {
    u8 *buf;
    int err;
// section info includes charactors but this module don't need it
    buf = kzalloc(12, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
// Info type is 'section info'.
    avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x07);
    buf[10] = 0xff & ++id;	/* section id */
    err = fcp_avc_transaction(unit, buf, 12, buf, 12,
    BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) |
    BIT(6) | BIT(7) | BIT(9) | BIT(10));
    if (err < 0)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(12: err <) -> else {
    else if (err < 12)
    err = -EIO;
    else if (buf[0] == 0x08) /* NOT IMPLEMENTED */
    err = -ENOSYS;
    else if (buf[0] == 0x0a) /* REJECTED */
    err = -EINVAL;
    else if (buf[0] == 0x0b) /* IN TRANSITION */
    err = -EAGAIN;
    if (err < 0)
    goto end;
// type = buf[11];
    err = 0;
    end:
    kfree(buf);
    return err;
    }
    int avc_bridgeco_get_plug_input(struct fw_unit *unit,
    u8 addr[AVC_BRIDGECO_ADDR_BYTES], u8 input[7])
    {
    int err;
    u8 *buf;
    buf = kzalloc(18, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
// Info type is 'plug input'.
    avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x05);
    err = fcp_avc_transaction(unit, buf, 16, buf, 16,
    BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) |
    BIT(6) | BIT(7));
    if (err < 0)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(16: err <) -> else {
    else if (err < 16)
    err = -EIO;
    else if (buf[0] == 0x08) /* NOT IMPLEMENTED */
    err = -ENOSYS;
    else if (buf[0] == 0x0a) /* REJECTED */
    err = -EINVAL;
    else if (buf[0] == 0x0b) /* IN TRANSITION */
    err = -EAGAIN;
    if (err < 0)
    goto end;
    memcpy(input, buf + 10, 5);
    err = 0;
    end:
    kfree(buf);
    return err;
    }
    int avc_bridgeco_get_plug_strm_fmt(struct fw_unit *unit,
    u8 addr[AVC_BRIDGECO_ADDR_BYTES], u8 *buf,
    unsigned int *len, unsigned int eid)
    {
    int err;
// check given buffer
    if ((buf == core::ptr::null_mut()) || (*len < 12)) {
    err = -EINVAL;
    goto end;
    }
    buf[0] = 0x01;	/* AV/C STATUS */
    buf[2] = 0x2f;	/* AV/C STREAM FORMAT SUPPORT */
    buf[3] = 0xc1;	/* Bridgeco extension - List Request */
    avc_bridgeco_fill_extension_addr(buf, addr);
    buf[10] = 0xff & eid;	/* Entry ID */
    err = fcp_avc_transaction(unit, buf, 12, buf, *len,
    BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) |
    BIT(6) | BIT(7) | BIT(10));
    if (err < 0)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(12: err <) -> else {
    else if (err < 12)
    err = -EIO;
    else if (buf[0] == 0x08)        /* NOT IMPLEMENTED */
    err = -ENOSYS;
    else if (buf[0] == 0x0a)        /* REJECTED */
    err = -EINVAL;
    else if (buf[0] == 0x0b)        /* IN TRANSITION */
    err = -EAGAIN;
#[no_mangle]
pub unsafe extern "C" fn if(eid: buf[10] !=) -> else {
    else if (buf[10] != eid)
    err = -EIO;
    if (err < 0)
    goto end;
// Pick up 'stream format info'.
    memmove(buf, buf + 11, err - 11);
// len = err - 11;
    err = 0;
    end:
    return err;
    }
