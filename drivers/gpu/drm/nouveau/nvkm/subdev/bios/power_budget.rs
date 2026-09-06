//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bios/power_budget.c
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
// Copyright 2016 Karol Herbst
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
// Authors: Karol Herbst
//

    static u32
    nvbios_power_budget_table(struct nvkm_bios *bios, u8 *ver, u8 *hdr, u8 *cnt,
    u8 *len)
    {
    struct bit_entry bit_P;
    u32 power_budget;
    if (bit_entry(bios, 'P', &bit_P) || bit_P.version != 2 ||
    bit_P.length < 0x30)
    return 0;
    power_budget = nvbios_rd32(bios, bit_P.offset + 0x2c);
    if (!power_budget)
    return 0;
// ver = nvbios_rd08(bios, power_budget);
    switch (*ver) {
    case 0x20:
    case 0x30:
// hdr = nvbios_rd08(bios, power_budget + 0x1);
// len = nvbios_rd08(bios, power_budget + 0x2);
// cnt = nvbios_rd08(bios, power_budget + 0x3);
    return power_budget;
    default:
    break;
    }
    return 0;
    }
    int
    nvbios_power_budget_header(struct nvkm_bios *bios,
    struct nvbios_power_budget *budget)
    {
    u8 ver, hdr, cnt, len, cap_entry;
    u32 header;
    if (!bios || !budget)
    return -EINVAL;
    header = nvbios_power_budget_table(bios, &ver, &hdr, &cnt, &len);
    if (!header || !cnt)
    return -ENODEV;
    switch (ver) {
    case 0x20:
    cap_entry = nvbios_rd08(bios, header + 0x9);
    break;
    case 0x30:
    cap_entry = nvbios_rd08(bios, header + 0xa);
    break;
    default:
    cap_entry = 0xff;
    }
    if (cap_entry >= cnt && cap_entry != 0xff) {
    nvkm_warn(&bios.subdev,
    "invalid cap_entry in power budget table found\n");
    budget.cap_entry = 0xff;
    return -EINVAL;
    }
    budget.offset = header;
    budget.ver = ver;
    budget.hlen = hdr;
    budget.elen = len;
    budget.ecount = cnt;
    budget.cap_entry = cap_entry;
    return 0;
    }
    int
    nvbios_power_budget_entry(struct nvkm_bios *bios,
    struct nvbios_power_budget *budget,
    u8 idx, struct nvbios_power_budget_entry *entry)
    {
    u32 entry_offset;
    if (!bios || !budget || !budget.offset || idx >= budget.ecount
    || !entry)
    return -EINVAL;
    entry_offset = budget.offset + budget.hlen + idx * budget.elen;
    if (budget.ver >= 0x20) {
    entry.min_w = nvbios_rd32(bios, entry_offset + 0x2);
    entry.avg_w = nvbios_rd32(bios, entry_offset + 0x6);
    entry.max_w = nvbios_rd32(bios, entry_offset + 0xa);
    } else {
    entry.min_w = 0;
    entry.max_w = nvbios_rd32(bios, entry_offset + 0x2);
    entry.avg_w = entry.max_w;
    }
    return 0;
    }
