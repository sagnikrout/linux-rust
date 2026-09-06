//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/nvidia/nv_of.c
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


//
// linux/drivers/video/nvidia/nv_of.c
//
// Copyright 2004 Antonino A. Daplas <adaplas @pol.net>
//
// Based on rivafb-i2c.c
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

#[no_mangle]
pub unsafe extern "C" fn nvidia_probe_of_connector(info: *mut fb_info, conn: c_int, out_edid: *mut u8) -> c_int {
    int nvidia_probe_of_connector(struct fb_info *info, int conn, u8 **out_edid)
    {
    struct nvidia_par *par = info.par;
    struct device_node *parent, *dp;
    const unsigned char *pedid = core::ptr::null_mut();
    static char *propnames[] = {
    "DFP,EDID", "LCD,EDID", "EDID", "EDID1",
    "EDID,B", "EDID,A", core::ptr::null_mut() };
    int i;
    parent = pci_device_to_OF_node(par.pci_dev);
    if (parent == core::ptr::null_mut())
    return -1;
    if (par.twoHeads) {
    const char *pname;
    int len;
    for_each_child_of_node(parent, dp) {
    pname = of_get_property(dp, "name", core::ptr::null_mut());
    if (!pname)
    continue;
    len = strlen(pname);
    if ((pname[len-1] == 'A' && conn == 1) ||
    (pname[len-1] == 'B' && conn == 2)) {
    for (i = 0; propnames[i] != core::ptr::null_mut(); ++i) {
    pedid = of_get_property(dp,
    propnames[i], core::ptr::null_mut());
    if (pedid != core::ptr::null_mut())
    break;
    }
    of_node_put(dp);
    break;
    }
    }
    }
    if (pedid == core::ptr::null_mut()) {
    for (i = 0; propnames[i] != core::ptr::null_mut(); ++i) {
    pedid = of_get_property(parent, propnames[i], core::ptr::null_mut());
    if (pedid != core::ptr::null_mut())
    break;
    }
    }
    if (pedid) {
// out_edid = kmemdup(pedid, EDID_LENGTH, GFP_KERNEL);
    if (*out_edid == core::ptr::null_mut())
    return -1;
    printk(KERN_DEBUG "nvidiafb: Found OF EDID for head %d\n", conn);
    return 0;
    }
    return -1;
    }
