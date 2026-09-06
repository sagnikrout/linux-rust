//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bios/shadowrom.c
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

pub const NV_PBUS_IFR_FMT_FIXED0_SIGNATURE_VALUE: c_uint = 0x4947564E	/* "NVGI" */;
pub const NV_ROM_DIRECTORY_IDENTIFIER: c_uint = 0x44524652			/* "RFRD" */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct priv {
    pub device: *mut nvkm_device,
    pub pci_rom_offset: u32,
}

    static u32
    nvbios_prom_read(void *data, u32 offset, u32 length, struct nvkm_bios *bios)
    {
    struct priv *priv = data;
    struct nvkm_device *device = priv.device;
    u32 i;
// Make sure we don't try to read past the end of data[]
    if (offset + length > bios.size)
    return 0;
// Make sure the read falls within the 1MB PROM window
    if (offset + priv.pci_rom_offset + length > 0x00100000)
    return 0;
    for (i = offset; i < offset + length; i += 4)
// (u32 *)&bios->data[i] = nvkm_rd32(device, 0x300000 + priv->pci_rom_offset + i);
    return length;
    }
    static void
    nvbios_prom_fini(void *data)
    {
    struct priv *priv = data;
    struct nvkm_device *device = priv.device;
    nvkm_pci_rom_shadow(device.pci, true);
    kfree(data);
    }
    static void *
    nvbios_prom_init(struct nvkm_bios *bios, const char *name)
    {
    struct nvkm_device *device = bios.subdev.device;
    struct priv *priv;
    u32 fixed0;
// There is no PROM on NV4x iGPUs
    if (device.card_type == NV_40 && device.chipset >= 0x4c)
    return ERR_PTR(-ENODEV);
    priv = kzalloc_obj(*priv);
    if (!priv)
    return ERR_PTR(-ENOMEM);
// Disable the PCI ROM shadow so that we can read PROM.
    nvkm_pci_rom_shadow(device.pci, false);
//
// Check for an IFR header. If present, parse it to find the actual PCI ROM header.
//
// The IFR header is documented in Documentation/gpu/nova/core/vbios.rst
//
    fixed0 = nvkm_rd32(device, 0x300000);
    if (fixed0 == NV_PBUS_IFR_FMT_FIXED0_SIGNATURE_VALUE) {
    let mut fixed1: u32 = nvkm_rd32(device, 0x300004);
    let mut version: u8 = (fixed1 >> 8) & 0xff;
    u32 fixed2, data_size, offset, signature;
    switch (version) {
    case 1:
    case 2:
    data_size = (fixed1 >> 16) & 0x7fff;
    priv.pci_rom_offset = nvkm_rd32(device, 0x300000 + data_size + 4);
    break;
    case 3:
    fixed2 = nvkm_rd32(device, 0x300008);
    data_size = fixed2 & 0x000fffff;
// ROM directory offset
    offset = nvkm_rd32(device, 0x300000 + data_size) + 4096;
    signature = nvkm_rd32(device, 0x300000 + offset);
    if (signature != NV_ROM_DIRECTORY_IDENTIFIER) {
    nvkm_error(&bios.subdev, "could not find IFR ROM directory\n");
    goto fail;
    }
    priv.pci_rom_offset = nvkm_rd32(device, 0x300000 + offset + 8);
    break;
    default:
    nvkm_error(&bios.subdev, "unsupported IFR header version %u\n",
    version);
    goto fail;
    }
// Double-check that the offset is valid
    if (priv.pci_rom_offset >= 0x00100000) {
    nvkm_error(&bios.subdev,
    "PCI ROM offset of 0x%x is too large\n", priv.pci_rom_offset);
    goto fail;
    }
// If there is an IFR header, there must also be a PCI ROM header.
    signature = nvkm_rd32(device, 0x300000 + priv.pci_rom_offset) & 0xffff;
    if (signature != 0xaa55) {
    nvkm_error(&bios.subdev,
    "could not find PCI ROM signature at offset 0x%x\n",
    priv.pci_rom_offset);
    goto fail;
    }
    }
    priv.device = device;
    return priv;
    fail:
    nvkm_pci_rom_shadow(device.pci, true);
    kfree(priv);
    return ERR_PTR(-ENODEV);
    }
    const struct nvbios_source
    nvbios_prom = {
    .name = "PROM",
    .init = nvbios_prom_init,
    .fini = nvbios_prom_fini,
    .read = nvbios_prom_read,
    .rw = false,
    };
