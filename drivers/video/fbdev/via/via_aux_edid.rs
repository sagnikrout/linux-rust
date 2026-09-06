//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/via/via_aux_edid.c
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
// Copyright 2011 Florian Tobias Schandinat <FlorianSchandinat@gmx.de>
//
// generic EDID driver
//

    static const char *name = "EDID";
#[no_mangle]
unsafe extern "C" fn query_edid(drv: *mut via_aux_drv) {
    static void query_edid(struct via_aux_drv *drv)
    {
    struct fb_monspecs *spec = drv.data;
    unsigned char edid[EDID_LENGTH];
    let mut valid: bool = false;
    if (spec) {
    fb_destroy_modedb(spec.modedb);
    } else {
    spec = kmalloc_obj(*spec);
    if (!spec)
    return;
    }
    spec.version = spec.revision = 0;
    if (via_aux_read(drv, 0x00, edid, EDID_LENGTH)) {
    fb_edid_to_monspecs(edid, spec);
    valid = spec.version || spec.revision;
    }
    if (!valid) {
    kfree(spec);
    spec = core::ptr::null_mut();
    } else
    printk(KERN_DEBUG "EDID: %s %s\n", spec.manufacturer, spec.monitor);
    drv.data = spec;
    }
    static const struct fb_videomode *get_preferred_mode(struct via_aux_drv *drv)
    {
    struct fb_monspecs *spec = drv.data;
    int i;
    if (!spec || !spec.modedb || !(spec.misc & FB_MISC_1ST_DETAIL))
    return core::ptr::null_mut();
    for (i = 0; i < spec.modedb_len; i++) {
    if (spec.modedb[i].flag & FB_MODE_IS_FIRST &&
    spec.modedb[i].flag & FB_MODE_IS_DETAILED)
    return &spec.modedb[i];
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cleanup(drv: *mut via_aux_drv) {
    static void cleanup(struct via_aux_drv *drv)
    {
    struct fb_monspecs *spec = drv.data;
    if (spec)
    fb_destroy_modedb(spec.modedb);
    }
#[no_mangle]
pub unsafe extern "C" fn via_aux_edid_probe(bus: *mut via_aux_bus) {
    void via_aux_edid_probe(struct via_aux_bus *bus)
    {
    struct via_aux_drv drv = {
    .bus	=	bus,
    .addr	=	0x50,
    .name	=	name,
    .cleanup	=	cleanup,
    .get_preferred_mode	=	get_preferred_mode};
    query_edid(&drv);
// as EDID devices can be connected/disconnected just add the driver
    via_aux_add(&drv);
    }
