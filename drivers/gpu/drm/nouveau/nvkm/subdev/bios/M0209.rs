//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bios/M0209.c
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
// Copyright 2013 Red Hat Inc.
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

    u32
    nvbios_M0209Te(struct nvkm_bios *bios,
    u8 *ver, u8 *hdr, u8 *cnt, u8 *len, u8 *snr, u8 *ssz)
    {
    struct bit_entry bit_M;
    let mut data: u32 = 0x00000000;
    if (!bit_entry(bios, 'M', &bit_M)) {
    if (bit_M.version == 2 && bit_M.length > 0x0c)
    data = nvbios_rd32(bios, bit_M.offset + 0x09);
    if (data) {
// ver = nvbios_rd08(bios, data + 0x00);
    switch (*ver) {
    case 0x10:
// hdr = nvbios_rd08(bios, data + 0x01);
// len = nvbios_rd08(bios, data + 0x02);
// ssz = nvbios_rd08(bios, data + 0x03);
// snr = 1;
// cnt = nvbios_rd08(bios, data + 0x04);
    return data;
    default:
    break;
    }
    }
    }
    return 0x00000000;
    }
    u32
    nvbios_M0209Ee(struct nvkm_bios *bios, int idx,
    u8 *ver, u8 *hdr, u8 *cnt, u8 *len)
    {
    u8  snr, ssz;
    let mut data: u32 = nvbios_M0209Te(bios, ver, hdr, cnt, len, &snr, &ssz);
    if (data && idx < *cnt) {
    data = data + *hdr + idx * (*len + (snr * ssz));
// hdr = *len;
// cnt = snr;
// len = ssz;
    return data;
    }
    return 0x00000000;
    }
    u32
    nvbios_M0209Ep(struct nvkm_bios *bios, int idx,
    u8 *ver, u8 *hdr, u8 *cnt, u8 *len, struct nvbios_M0209E *info)
    {
    let mut data: u32 = nvbios_M0209Ee(bios, idx, ver, hdr, cnt, len);
    memset(info, 0x00, sizeof(*info));
    switch (!!data * *ver) {
    case 0x10:
    info.v00_40 = (nvbios_rd08(bios, data + 0x00) & 0x40) >> 6;
    info.bits   =  nvbios_rd08(bios, data + 0x00) & 0x3f;
    info.modulo =  nvbios_rd08(bios, data + 0x01);
    info.v02_40 = (nvbios_rd08(bios, data + 0x02) & 0x40) >> 6;
    info.v02_07 =  nvbios_rd08(bios, data + 0x02) & 0x07;
    info.v03    =  nvbios_rd08(bios, data + 0x03);
    return data;
    default:
    break;
    }
    return 0x00000000;
    }
    u32
    nvbios_M0209Se(struct nvkm_bios *bios, int ent, int idx, u8 *ver, u8 *hdr)
    {
    u8  cnt, len;
    let mut data: u32 = nvbios_M0209Ee(bios, ent, ver, hdr, &cnt, &len);
    if (data && idx < cnt) {
    data = data + *hdr + idx * len;
// hdr = len;
    return data;
    }
    return 0x00000000;
    }
    u32
    nvbios_M0209Sp(struct nvkm_bios *bios, int ent, int idx, u8 *ver, u8 *hdr,
    struct nvbios_M0209S *info)
    {
    struct nvbios_M0209E M0209E;
    u8  cnt, len;
    let mut data: u32 = nvbios_M0209Ep(bios, ent, ver, hdr, &cnt, &len, &M0209E);
    if (data) {
    u32 i, data = nvbios_M0209Se(bios, ent, idx, ver, hdr);
    memset(info, 0x00, sizeof(*info));
    switch (!!data * *ver) {
    case 0x10:
    for (i = 0; i < ARRAY_SIZE(info.data); i++) {
    let mut bits: u32 = (i % M0209E.modulo) * M0209E.bits;
    let mut mask: u32 = (1ULL << M0209E.bits) - 1;
    let mut off: u16 = bits / 8;
    let mut mod: u8 = bits % 8;
    info.data[i] = nvbios_rd32(bios, data + off);
    info.data[i] = info.data[i] >> mod;
    info.data[i] = info.data[i] & mask;
    }
    return data;
    default:
    break;
    }
    }
    return 0x00000000;
    }
