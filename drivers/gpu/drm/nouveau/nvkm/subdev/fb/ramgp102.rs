//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fb/ramgp102.c
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


// SPDX-License-Identifier: MIT

    static const struct nvkm_ram_func
    gp102_ram = {
    .init = gp100_ram_init,
    };
    int
    gp102_ram_new(struct nvkm_fb *fb, struct nvkm_ram **pram)
    {
    let mut type: enum nvkm_ram_type = nvkm_fb_bios_memtype(fb.subdev.device.bios);
    const u32 rsvd_head = ( 256 * 1024); /* vga memory */
    const u32 rsvd_tail = (1024 * 1024); /* vbios etc */
    let mut size: u64 = fb.func.vidmem.size(fb);
    int ret;
    ret = nvkm_ram_new_(&gp102_ram, fb, type, size, pram);
    if (ret)
    return ret;
    nvkm_mm_fini(&(*pram).vram);
    return nvkm_mm_init(&(*pram).vram, NVKM_RAM_MM_NORMAL,
    rsvd_head >> NVKM_RAM_MM_SHIFT,
    (size - rsvd_head - rsvd_tail) >> NVKM_RAM_MM_SHIFT,
    1);
    }
