//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/config.c
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
// usb/gadget/config.c -- simplify building config descriptors
//
// Copyright (C) 2003 David Brownell
//

//
// usb_descriptor_fillbuf - fill buffer with descriptors
// @buf: Buffer to be filled
// @buflen: Size of buf
// @src: Array of descriptor pointers, terminated by null pointer.
//
// Copies descriptors into the buffer, returning the length or a
// negative error code if they can't all be copied.  Useful when
// assembling descriptors for an associated set of interfaces used
// as part of configuring a composite device; or in other cases where
// sets of descriptors need to be marshaled.
//
    int
    usb_descriptor_fillbuf(void *buf, unsigned buflen,
    const struct usb_descriptor_header **src)
    {
    u8	*dest = buf;
    if (!src)
    return -EINVAL;
// fill buffer from src[] until null descriptor ptr
    for (; core::ptr::null_mut() != *src; src++) {
    let mut len: unsigned = (*src).bLength;
    if (len > buflen)
    return -EINVAL;
    memcpy(dest, *src, len);
    buflen -= len;
    dest += len;
    }
    return dest - (u8 *)buf;
    }
    EXPORT_SYMBOL_GPL(usb_descriptor_fillbuf);
//
// usb_copy_descriptors - copy a vector of USB descriptors
// @src: null-terminated vector to copy
// Context: initialization code, which may sleep
//
// This makes a copy of a vector of USB descriptors.  Its primary use
// is to support usb_function objects which can have multiple copies,
// each needing different descriptors.  Functions may have static
// tables of descriptors, which are used as templates and customized
// with identifiers (for interfaces, strings, endpoints, and more)
// as needed by a given function instance.
//
    struct usb_descriptor_header **
    usb_copy_descriptors(struct usb_descriptor_header **src)
    {
    struct usb_descriptor_header **tmp;
    unsigned bytes;
    unsigned n_desc;
    void *mem;
    struct usb_descriptor_header **ret;
// count descriptors and their sizes; then add vector size
    for (bytes = 0, n_desc = 0, tmp = src; *tmp; tmp++, n_desc++)
    bytes += (*tmp).bLength;
    bytes += (n_desc + 1) * sizeof(*tmp);
    mem = kmalloc(bytes, GFP_KERNEL);
    if (!mem)
    return core::ptr::null_mut();
// fill in pointers starting at "tmp",
// to descriptors copied starting at "mem";
// and return "ret"
//
    tmp = mem;
    ret = mem;
    mem += (n_desc + 1) * sizeof(*tmp);
    while (*src) {
    memcpy(mem, *src, (*src).bLength);
// tmp = mem;
    tmp++;
    mem += (*src).bLength;
    src++;
    }
// tmp = NULL;
    return ret;
    }
    EXPORT_SYMBOL_GPL(usb_copy_descriptors);
    int usb_assign_descriptors(struct usb_function *f,
    struct usb_descriptor_header **fs,
    struct usb_descriptor_header **hs,
    struct usb_descriptor_header **ss,
    struct usb_descriptor_header **ssp)
    {
// super-speed-plus descriptor falls back to super-speed one,
// if such a descriptor was provided, thus avoiding a NULL
// pointer dereference if a 5gbps capable gadget is used with
// a 10gbps capable config (device port + cable + host port)
//
    if (!ssp)
    ssp = ss;
    if (fs) {
    f.fs_descriptors = usb_copy_descriptors(fs);
    if (!f.fs_descriptors)
    goto err;
    }
    if (hs) {
    f.hs_descriptors = usb_copy_descriptors(hs);
    if (!f.hs_descriptors)
    goto err;
    }
    if (ss) {
    f.ss_descriptors = usb_copy_descriptors(ss);
    if (!f.ss_descriptors)
    goto err;
    }
    if (ssp) {
    f.ssp_descriptors = usb_copy_descriptors(ssp);
    if (!f.ssp_descriptors)
    goto err;
    }
    return 0;
    err:
    usb_free_all_descriptors(f);
    return -ENOMEM;
    }
    EXPORT_SYMBOL_GPL(usb_assign_descriptors);
#[no_mangle]
pub unsafe extern "C" fn usb_free_all_descriptors(f: *mut usb_function) {
    void usb_free_all_descriptors(struct usb_function *f)
    {
    usb_free_descriptors(f.fs_descriptors);
    f.fs_descriptors = core::ptr::null_mut();
    usb_free_descriptors(f.hs_descriptors);
    f.hs_descriptors = core::ptr::null_mut();
    usb_free_descriptors(f.ss_descriptors);
    f.ss_descriptors = core::ptr::null_mut();
    usb_free_descriptors(f.ssp_descriptors);
    f.ssp_descriptors = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(usb_free_all_descriptors);
    struct usb_descriptor_header *usb_otg_descriptor_alloc(
    struct usb_gadget *gadget)
    {
    struct usb_descriptor_header *otg_desc;
    let mut length: unsigned = 0;
    if (gadget.otg_caps && (gadget.otg_caps.otg_rev >= 0x0200))
    length = sizeof(struct usb_otg20_descriptor);
    else
    length = sizeof(struct usb_otg_descriptor);
    otg_desc = kzalloc(length, GFP_KERNEL);
    return otg_desc;
    }
    EXPORT_SYMBOL_GPL(usb_otg_descriptor_alloc);
    int usb_otg_descriptor_init(struct usb_gadget *gadget,
    struct usb_descriptor_header *otg_desc)
    {
    struct usb_otg_descriptor *otg1x_desc;
    struct usb_otg20_descriptor *otg20_desc;
    struct usb_otg_caps *otg_caps = gadget.otg_caps;
    let mut otg_attributes: u8 = 0;
    if (!otg_desc)
    return -EINVAL;
    if (otg_caps && otg_caps.otg_rev) {
    if (otg_caps.hnp_support)
    otg_attributes |= USB_OTG_HNP;
    if (otg_caps.srp_support)
    otg_attributes |= USB_OTG_SRP;
    if (otg_caps.adp_support && (otg_caps.otg_rev >= 0x0200))
    otg_attributes |= USB_OTG_ADP;
    } else {
    otg_attributes = USB_OTG_SRP | USB_OTG_HNP;
    }
    if (otg_caps && (otg_caps.otg_rev >= 0x0200)) {
    otg20_desc = (struct usb_otg20_descriptor *)otg_desc;
    otg20_desc.bLength = sizeof(struct usb_otg20_descriptor);
    otg20_desc.bDescriptorType = USB_DT_OTG;
    otg20_desc.bmAttributes = otg_attributes;
    otg20_desc.bcdOTG = cpu_to_le16(otg_caps.otg_rev);
    } else {
    otg1x_desc = (struct usb_otg_descriptor *)otg_desc;
    otg1x_desc.bLength = sizeof(struct usb_otg_descriptor);
    otg1x_desc.bDescriptorType = USB_DT_OTG;
    otg1x_desc.bmAttributes = otg_attributes;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(usb_otg_descriptor_init);
