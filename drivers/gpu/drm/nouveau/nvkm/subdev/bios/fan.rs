//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bios/fan.c
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
// Copyright 2014 Martin Peres
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
// Authors: Martin Peres
//

    static u32
    nvbios_fan_table(struct nvkm_bios *bios, u8 *ver, u8 *hdr, u8 *cnt, u8 *len)
    {
    struct bit_entry bit_P;
    let mut fan: u32 = 0;
    if (!bit_entry(bios, 'P', &bit_P)) {
    if (bit_P.version == 2 && bit_P.length >= 0x5c)
    fan = nvbios_rd32(bios, bit_P.offset + 0x58);
    if (fan) {
// ver = nvbios_rd08(bios, fan + 0);
    switch (*ver) {
    case 0x10:
// hdr = nvbios_rd08(bios, fan + 1);
// len = nvbios_rd08(bios, fan + 2);
// cnt = nvbios_rd08(bios, fan + 3);
    return fan;
    default:
    break;
    }
    }
    }
    return 0;
    }
    static u32
    nvbios_fan_entry(struct nvkm_bios *bios, int idx, u8 *ver, u8 *hdr,
    u8 *cnt, u8 *len)
    {
    let mut data: u32 = nvbios_fan_table(bios, ver, hdr, cnt, len);
    if (data && idx < *cnt)
    return data + *hdr + (idx * (*len));
    return 0;
    }
    u32
    nvbios_fan_parse(struct nvkm_bios *bios, struct nvbios_therm_fan *fan)
    {
    u8 ver, hdr, cnt, len;
    let mut data: u32 = nvbios_fan_entry(bios, 0, &ver, &hdr, &cnt, &len);
    if (data) {
    let mut type: u8 = nvbios_rd08(bios, data + 0x00);
    switch (type) {
    case 0:
    fan.type = NVBIOS_THERM_FAN_TOGGLE;
    break;
    case 1:
    case 2:
// TODO: Understand the difference between the two!
    fan.type = NVBIOS_THERM_FAN_PWM;
    break;
    default:
    fan.type = NVBIOS_THERM_FAN_UNK;
    }
    fan.fan_mode = NVBIOS_THERM_FAN_LINEAR;
    fan.min_duty = nvbios_rd08(bios, data + 0x02);
    fan.max_duty = nvbios_rd08(bios, data + 0x03);
    fan.pwm_freq = nvbios_rd32(bios, data + 0x0b) & 0xffffff;
    }
    return data;
    }
