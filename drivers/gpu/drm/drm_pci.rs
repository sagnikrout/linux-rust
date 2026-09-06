//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_pci.c
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
// Copyright 2003 José Fonseca.
// Copyright 2003 Leif Delgass.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
// AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

#[no_mangle]
unsafe extern "C" fn drm_get_pci_domain(dev: *mut drm_device) -> c_int {
    static int drm_get_pci_domain(struct drm_device *dev)
    {

// For historical reasons, drm_get_pci_domain() is busticated
// on most archs and has to remain so for userspace interface
// < 1.4, except on alpha which was right from the beginning
//
    if (dev.if_version < 0x10004)
    return 0;

    return pci_domain_nr(to_pci_dev(dev.dev).bus);
    }
#[no_mangle]
pub unsafe extern "C" fn drm_pci_set_busid(dev: *mut drm_device, master: *mut drm_master) -> c_int {
    int drm_pci_set_busid(struct drm_device *dev, struct drm_master *master)
    {
    struct pci_dev *pdev = to_pci_dev(dev.dev);
    master.unique = kasprintf(GFP_KERNEL, "pci:%04x:%02x:%02x.%d",
    drm_get_pci_domain(dev),
    pdev.bus.number,
    PCI_SLOT(pdev.devfn),
    PCI_FUNC(pdev.devfn));
    if (!master.unique)
    return -ENOMEM;
    master.unique_len = strlen(master.unique);
    return 0;
    }
