//! Automatically rewritten from C to Rust
//! Source: arch/x86/video/video-common.c
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
// Copyright (C) 2007 Antonino Daplas <adaplas@gmail.com>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

    pgprot_t pgprot_framebuffer(pgprot_t prot,
    unsigned long vm_start, unsigned long vm_end,
    unsigned long offset)
    {
    pgprot_val(prot) &= ~_PAGE_CACHE_MASK;
    if (boot_cpu_data.x86 > 3)
    pgprot_val(prot) |= cachemode2protval(_PAGE_CACHE_MODE_UC_MINUS);
    return prot;
    }
    EXPORT_SYMBOL(pgprot_framebuffer);
#[no_mangle]
pub unsafe extern "C" fn video_is_primary_device(dev: *mut device) -> bool {
    bool video_is_primary_device(struct device *dev)
    {

    struct screen_info *si = &sysfb_primary_display.screen;
    struct resource res[SCREEN_INFO_MAX_RESOURCES];
    ssize_t i, numres;

    struct pci_dev *pdev;
    if (!dev_is_pci(dev))
    return false;
    pdev = to_pci_dev(dev);
    if (!pci_is_display(pdev))
    return false;

    numres = screen_info_resources(si, res, ARRAY_SIZE(res));
    if (numres > 0) {
    for (i = 0; i < numres; ++i) {
    if (!(res[i].flags & IORESOURCE_MEM))
    continue;
    if (pci_find_resource(pdev, &res[i]))
    return true;
    }
    return false;
    }

//
// No framebuffer was set up by the firmware/bootloader, so fall back
// to the default VGA device.
//
    let mut pdev: return = = vga_default_device();
    }
    EXPORT_SYMBOL(video_is_primary_device);
    MODULE_LICENSE("GPL");
