//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv50/coreca7d.c
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
//
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.
//

    static int
    coreca7d_update(struct nv50_core *core, u32 *interlock, bool ntfy)
    {
    let mut ntfy_addr: u64 = core.disp.sync.offset + NV50_DISP_CORE_NTFY;
    let mut ntfy_hi: u32 = upper_32_bits(ntfy_addr);
    let mut ntfy_lo: u32 = lower_32_bits(ntfy_addr);
    struct nvif_push *push = &core.chan.push;
    int ret;
    ret = PUSH_WAIT(push, 5 + (ntfy ? 5 + 2 : 0));
    if (ret)
    return ret;
    if (ntfy) {
    PUSH_MTHD(push, NVCA7D, SET_SURFACE_ADDRESS_HI_NOTIFIER, ntfy_hi,
    SET_SURFACE_ADDRESS_LO_NOTIFIER,
    NVVAL(NVCA7D, SET_SURFACE_ADDRESS_LO_NOTIFIER, ADDRESS_LO, ntfy_lo >> 4) |
    NVDEF(NVCA7D, SET_SURFACE_ADDRESS_LO_NOTIFIER, TARGET, PHYSICAL_NVM) |
    NVDEF(NVCA7D, SET_SURFACE_ADDRESS_LO_NOTIFIER, ENABLE, ENABLE));
    PUSH_MTHD(push, NVCA7D, SET_NOTIFIER_CONTROL,
    NVDEF(NVCA7D, SET_NOTIFIER_CONTROL, MODE, WRITE) |
    NVDEF(NVCA7D, SET_NOTIFIER_CONTROL, NOTIFY, ENABLE));
    }
    PUSH_MTHD(push, NVCA7D, SET_INTERLOCK_FLAGS, interlock[NV50_DISP_INTERLOCK_CURS],
    SET_WINDOW_INTERLOCK_FLAGS, interlock[NV50_DISP_INTERLOCK_WNDW]);
    PUSH_MTHD(push, NVCA7D, UPDATE,
    NVDEF(NVCA7D, UPDATE, RELEASE_ELV, TRUE) |
    NVDEF(NVCA7D, UPDATE, SPECIAL_HANDLING, NONE) |
    NVDEF(NVCA7D, UPDATE, INHIBIT_INTERRUPTS, FALSE));
    if (ntfy) {
    PUSH_MTHD(push, NVCA7D, SET_NOTIFIER_CONTROL,
    NVDEF(NVCA7D, SET_NOTIFIER_CONTROL, NOTIFY, DISABLE));
    }
    return PUSH_KICK(push);
    }
    static int
    coreca7d_init(struct nv50_core *core)
    {
    struct nvif_push *push = &core.chan.push;
    let mut windows: u32 = 8, heads = 4;
    int ret, i;
    ret = PUSH_WAIT(push, windows * 6 + heads * 6);
    if (ret)
    return ret;
    for (i = 0; i < windows; i++) {
    PUSH_MTHD(push, NVCA7D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS(i),
    NVDEF(NVCA7D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS, RGB_PACKED1BPP, TRUE) |
    NVDEF(NVCA7D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS, RGB_PACKED2BPP, TRUE) |
    NVDEF(NVCA7D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS, RGB_PACKED4BPP, TRUE) |
    NVDEF(NVCA7D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS, RGB_PACKED8BPP, TRUE),
    WINDOW_SET_WINDOW_ROTATED_FORMAT_USAGE_BOUNDS(i), 0x00000000);
    PUSH_MTHD(push, NVCA7D, WINDOW_SET_WINDOW_USAGE_BOUNDS(i),
    NVVAL(NVCA7D, WINDOW_SET_WINDOW_USAGE_BOUNDS, MAX_PIXELS_FETCHED_PER_LINE, 0x7fff) |
    NVDEF(NVCA7D, WINDOW_SET_WINDOW_USAGE_BOUNDS, ILUT_ALLOWED, TRUE) |
    NVDEF(NVCA7D, WINDOW_SET_WINDOW_USAGE_BOUNDS, INPUT_SCALER_TAPS, TAPS_2) |
    NVDEF(NVCA7D, WINDOW_SET_WINDOW_USAGE_BOUNDS, UPSCALING_ALLOWED, FALSE),
    WINDOW_SET_PHYSICAL(i), BIT(i));
    }
    for (i = 0; i < heads; i++) {
    PUSH_MTHD(push, NVCA7D, HEAD_SET_HEAD_USAGE_BOUNDS(i),
    NVDEF(NVCA7D, HEAD_SET_HEAD_USAGE_BOUNDS, CURSOR, USAGE_W256_H256) |
    NVDEF(NVCA7D, HEAD_SET_HEAD_USAGE_BOUNDS, OLUT_ALLOWED, TRUE) |
    NVDEF(NVCA7D, HEAD_SET_HEAD_USAGE_BOUNDS, OUTPUT_SCALER_TAPS, TAPS_2) |
    NVDEF(NVCA7D, HEAD_SET_HEAD_USAGE_BOUNDS, UPSCALING_ALLOWED, TRUE));
    PUSH_MTHD(push, NVCA7D, HEAD_SET_TILE_MASK(i), BIT(i));
    PUSH_MTHD(push, NVCA7D, TILE_SET_TILE_SIZE(i), 0);
    }
    core.assign_windows = true;
    return PUSH_KICK(push);
    }
    static const struct nv50_core_func
    coreca7d = {
    .init = coreca7d_init,
    .ntfy_init = corec37d_ntfy_init,
    .caps_init = corec37d_caps_init,
    .caps_class = GB202_DISP_CAPS,
    .ntfy_wait_done = corec37d_ntfy_wait_done,
    .update = coreca7d_update,
    .wndw.owner = corec37d_wndw_owner,
    .head = &headca7d,
    .sor = &sorc37d,

    .crc = &crcca7d,

    };
    int
    coreca7d_new(struct nouveau_drm *drm, s32 oclass, struct nv50_core **pcore)
    {
    return core507d_new_(&coreca7d, drm, oclass, pcore);
    }
