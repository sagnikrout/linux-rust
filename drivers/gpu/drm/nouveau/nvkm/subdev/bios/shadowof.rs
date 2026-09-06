//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bios/shadowof.c
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
// Copyright 2012 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct priv {
    pub data: *const void __iomem,
    pub size: c_int,
}

    static u32
    of_read(void *data, u32 offset, u32 length, struct nvkm_bios *bios)
    {
    struct priv *priv = data;
    if (offset < priv.size) {
    length = min_t(u32, length, priv.size - offset);
    memcpy_fromio(bios.data + offset, priv.data + offset, length);
    return length;
    }
    return 0;
    }
    static u32
    of_size(void *data)
    {
    struct priv *priv = data;
    return priv.size;
    }
    static void *
    of_init(struct nvkm_bios *bios, const char *name)
    {
    struct nvkm_device *device = bios.subdev.device;
    struct pci_dev *pdev = device.func.pci(device).pdev;
    struct device_node *dn;
    struct priv *priv;
    if (!(dn = pci_device_to_OF_node(pdev)))
    return ERR_PTR(-ENODEV);
    if (!(priv = kzalloc_obj(*priv)))
    return ERR_PTR(-ENOMEM);
    if ((priv.data = of_get_property(dn, "NVDA,BMP", &priv.size)))
    return priv;
    kfree(priv);
    return ERR_PTR(-EINVAL);
    }
#[no_mangle]
unsafe extern "C" fn of_fini(p: *mut c_void) {
    static void of_fini(void *p)
    {
    kfree(p);
    }
    const struct nvbios_source
    nvbios_of = {
    .name = "OpenFirmware",
    .init = of_init,
    .fini = of_fini,
    .read = of_read,
    .size = of_size,
    .rw = false,
    .ignore_checksum = true,
    .no_pcir = true,
    };

    const struct nvbios_source
    nvbios_of = {
    };
