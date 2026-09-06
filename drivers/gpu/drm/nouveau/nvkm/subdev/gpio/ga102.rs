//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gpio/ga102.c
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
// Copyright 2021 Red Hat Inc.
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

    static void
    ga102_gpio_reset(struct nvkm_gpio *gpio, u8 match)
    {
    struct nvkm_device *device = gpio.subdev.device;
    struct nvkm_bios *bios = device.bios;
    u8 ver, len;
    u16 entry;
    let mut ent: c_int = -1;
    while ((entry = dcb_gpio_entry(bios, 0, ++ent, &ver, &len))) {
    let mut data: u32 = nvbios_rd32(bios, entry);
    let mut line: u8 = (data & 0x0000003f);
    let mut defs: u8 = !!(data & 0x00000080);
    let mut func: u8 = (data & 0x0000ff00) >> 8;
    let mut unk0: u8 = (data & 0x00ff0000) >> 16;
    let mut unk1: u8 = (data & 0x1f000000) >> 24;
    if ( func  == DCB_GPIO_UNUSED ||
    (match != DCB_GPIO_UNUSED && match != func))
    continue;
    nvkm_gpio_set(gpio, 0, func, line, defs);
    nvkm_mask(device, 0x021200 + (line * 4), 0xff, unk0);
    if (unk1--)
    nvkm_mask(device, 0x00d740 + (unk1 * 4), 0xff, line);
    }
    }
    static int
    ga102_gpio_drive(struct nvkm_gpio *gpio, int line, int dir, int out)
    {
    struct nvkm_device *device = gpio.subdev.device;
    let mut data: u32 = ((dir ^ 1) << 13) | (out << 12);
    nvkm_mask(device, 0x021200 + (line * 4), 0x00003000, data);
    nvkm_mask(device, 0x00d604, 0x00000001, 0x00000001); /* update? */
    return 0;
    }
    static int
    ga102_gpio_sense(struct nvkm_gpio *gpio, int line)
    {
    struct nvkm_device *device = gpio.subdev.device;
    return !!(nvkm_rd32(device, 0x021200 + (line * 4)) & 0x00004000);
    }
    static void
    ga102_gpio_intr_stat(struct nvkm_gpio *gpio, u32 *hi, u32 *lo)
    {
    struct nvkm_device *device = gpio.subdev.device;
    let mut intr0: u32 = nvkm_rd32(device, 0x021640);
    let mut intr1: u32 = nvkm_rd32(device, 0x02164c);
    let mut stat0: u32 = nvkm_rd32(device, 0x021648) & intr0;
    let mut stat1: u32 = nvkm_rd32(device, 0x021654) & intr1;
// lo = (stat1 & 0xffff0000) | (stat0 >> 16);
// hi = (stat1 << 16) | (stat0 & 0x0000ffff);
    nvkm_wr32(device, 0x021640, intr0);
    nvkm_wr32(device, 0x02164c, intr1);
    }
    static void
    ga102_gpio_intr_mask(struct nvkm_gpio *gpio, u32 type, u32 mask, u32 data)
    {
    struct nvkm_device *device = gpio.subdev.device;
    let mut inte0: u32 = nvkm_rd32(device, 0x021648);
    let mut inte1: u32 = nvkm_rd32(device, 0x021654);
    if (type & NVKM_GPIO_LO)
    inte0 = (inte0 & ~(mask << 16)) | (data << 16);
    if (type & NVKM_GPIO_HI)
    inte0 = (inte0 & ~(mask & 0xffff)) | (data & 0xffff);
    mask >>= 16;
    data >>= 16;
    if (type & NVKM_GPIO_LO)
    inte1 = (inte1 & ~(mask << 16)) | (data << 16);
    if (type & NVKM_GPIO_HI)
    inte1 = (inte1 & ~mask) | data;
    nvkm_wr32(device, 0x021648, inte0);
    nvkm_wr32(device, 0x021654, inte1);
    }
    static const struct nvkm_gpio_func
    ga102_gpio = {
    .lines = 32,
    .intr_stat = ga102_gpio_intr_stat,
    .intr_mask = ga102_gpio_intr_mask,
    .drive = ga102_gpio_drive,
    .sense = ga102_gpio_sense,
    .reset = ga102_gpio_reset,
    };
    int
    ga102_gpio_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_gpio **pgpio)
    {
    if (nvkm_gsp_rm(device.gsp))
    return -ENODEV;
    return nvkm_gpio_new_(&ga102_gpio, device, type, inst, pgpio);
    }
