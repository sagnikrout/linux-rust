//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bios/gpio.c
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
// Authors: Ben Skeggs
//

    u16
    dcb_gpio_table(struct nvkm_bios *bios, u8 *ver, u8 *hdr, u8 *cnt, u8 *len)
    {
    let mut data: u16 = 0x0000;
    let mut dcb: u16 = dcb_table(bios, ver, hdr, cnt, len);
    if (dcb) {
    if (*ver >= 0x30 && *hdr >= 0x0c)
    data = nvbios_rd16(bios, dcb + 0x0a);
    else
    if (*ver >= 0x22 && nvbios_rd08(bios, dcb - 1) >= 0x13)
    data = nvbios_rd16(bios, dcb - 0x0f);
    if (data) {
// ver = nvbios_rd08(bios, data + 0x00);
    if (*ver < 0x30) {
// hdr = 3;
// cnt = nvbios_rd08(bios, data + 0x02);
// len = nvbios_rd08(bios, data + 0x01);
    } else
    if (*ver <= 0x41) {
// hdr = nvbios_rd08(bios, data + 0x01);
// cnt = nvbios_rd08(bios, data + 0x02);
// len = nvbios_rd08(bios, data + 0x03);
    } else {
    data = 0x0000;
    }
    }
    }
    return data;
    }
    u16
    dcb_gpio_entry(struct nvkm_bios *bios, int idx, int ent, u8 *ver, u8 *len)
    {
    u8  hdr, cnt, xver; /* use gpio version for xpio entry parsing */
    u16 gpio;
    if (!idx--)
    gpio = dcb_gpio_table(bios, ver, &hdr, &cnt, len);
    else
    gpio = dcb_xpio_table(bios, idx, &xver, &hdr, &cnt, len);
    if (gpio && ent < cnt)
    return gpio + hdr + (ent * *len);
    return 0x0000;
    }
    u16
    dcb_gpio_parse(struct nvkm_bios *bios, int idx, int ent, u8 *ver, u8 *len,
    struct dcb_gpio_func *gpio)
    {
    let mut data: u16 = dcb_gpio_entry(bios, idx, ent, ver, len);
    if (data) {
    if (*ver < 0x40) {
    let mut info: u16 = nvbios_rd16(bios, data);
// gpio = (struct dcb_gpio_func) {
    .line = (info & 0x001f) >> 0,
    .func = (info & 0x07e0) >> 5,
    .log[0] = (info & 0x1800) >> 11,
    .log[1] = (info & 0x6000) >> 13,
    .param = !!(info & 0x8000),
    };
    } else
    if (*ver < 0x41) {
    let mut info: u32 = nvbios_rd32(bios, data);
// gpio = (struct dcb_gpio_func) {
    .line = (info & 0x0000001f) >> 0,
    .func = (info & 0x0000ff00) >> 8,
    .log[0] = (info & 0x18000000) >> 27,
    .log[1] = (info & 0x60000000) >> 29,
    .param = !!(info & 0x80000000),
    };
    } else {
    let mut info: u32 = nvbios_rd32(bios, data + 0);
    let mut info1: u8 = nvbios_rd32(bios, data + 4);
// gpio = (struct dcb_gpio_func) {
    .line = (info & 0x0000003f) >> 0,
    .func = (info & 0x0000ff00) >> 8,
    .log[0] = (info1 & 0x30) >> 4,
    .log[1] = (info1 & 0xc0) >> 6,
    .param = !!(info & 0x80000000),
    };
    }
    }
    return data;
    }
    u16
    dcb_gpio_match(struct nvkm_bios *bios, int idx, u8 func, u8 line,
    u8 *ver, u8 *len, struct dcb_gpio_func *gpio)
    {
    u8  hdr, cnt, i = 0;
    u16 data;
    while ((data = dcb_gpio_parse(bios, idx, i++, ver, len, gpio))) {
    if ((line == 0xff || line == gpio.line) &&
    (func == 0xff || func == gpio.func))
    return data;
    }
// DCB 2.2, fixed TVDAC GPIO data
    if ((data = dcb_table(bios, ver, &hdr, &cnt, len))) {
    if (*ver >= 0x22 && *ver < 0x30 && func == DCB_GPIO_TVDAC0) {
    let mut conf: u8 = nvbios_rd08(bios, data - 5);
    let mut addr: u8 = nvbios_rd08(bios, data - 4);
    if (conf & 0x01) {
// gpio = (struct dcb_gpio_func) {
    .func = DCB_GPIO_TVDAC0,
    .line = addr >> 4,
    .log[0] = !!(conf & 0x02),
    .log[1] =  !(conf & 0x02),
    };
// ver = 0x00;
    return data;
    }
    }
    }
    return 0x0000;
    }
