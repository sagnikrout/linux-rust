//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv50/crcca7d.c
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
    crcca7d_set_ctx(struct nv50_head *head, struct nv50_crc_notifier_ctx *ctx)
    {
    struct nvif_push *push = &head.disp.core.chan.push;
    let mut i: c_int = head.base.index;
    int ret;
    ret = PUSH_WAIT(push, ctx ? 3 : 2);
    if (ret)
    return ret;
    if (ctx) {
    let mut crc_hi: u32 = upper_32_bits(ctx.mem.addr);
    let mut crc_lo: u32 = lower_32_bits(ctx.mem.addr);
    PUSH_MTHD(push, NVCA7D, HEAD_SET_SURFACE_ADDRESS_HI_CRC(i), crc_hi,
    HEAD_SET_SURFACE_ADDRESS_LO_CRC(i),
    NVVAL(NVCA7D, HEAD_SET_SURFACE_ADDRESS_LO_CRC, ADDRESS_LO, crc_lo >> 4) |
    NVDEF(NVCA7D, HEAD_SET_SURFACE_ADDRESS_LO_CRC, TARGET, PHYSICAL_NVM) |
    NVDEF(NVCA7D, HEAD_SET_SURFACE_ADDRESS_LO_CRC, ENABLE, ENABLE));
    } else {
    PUSH_MTHD(push, NVCA7D, HEAD_SET_SURFACE_ADDRESS_LO_CRC(i),
    NVDEF(NVCA7D, HEAD_SET_SURFACE_ADDRESS_LO_CRC, ENABLE, DISABLE));
    }
    return 0;
    }
    static int
    crcca7d_set_src(struct nv50_head *head, int or, enum nv50_crc_source_type source,
    struct nv50_crc_notifier_ctx *ctx)
    {
    struct nvif_push *push = &head.disp.core.chan.push;
    let mut i: c_int = head.base.index;
    int primary_crc, ret;
    if (!source) {
    ret = PUSH_WAIT(push, 1);
    if (ret)
    return ret;
    PUSH_MTHD(push, NVCA7D, HEAD_SET_CRC_CONTROL(i), 0);
    return crcca7d_set_ctx(head, core::ptr::null_mut());
    }
    switch (source) {
    case NV50_CRC_SOURCE_TYPE_SOR:
    primary_crc = NVCA7D_HEAD_SET_CRC_CONTROL_PRIMARY_CRC_SOR(or);
    break;
    case NV50_CRC_SOURCE_TYPE_SF:
    primary_crc = NVCA7D_HEAD_SET_CRC_CONTROL_PRIMARY_CRC_SF;
    break;
    default:
    break;
    }
    ret = crcca7d_set_ctx(head, ctx);
    if (ret)
    return ret;
    ret = PUSH_WAIT(push, 2);
    if (ret)
    return ret;
    PUSH_MTHD(push, NVCA7D, HEAD_SET_CRC_CONTROL(i),
    NVDEF(NVCA7D, HEAD_SET_CRC_CONTROL, CONTROLLING_CHANNEL, CORE) |
    NVDEF(NVCA7D, HEAD_SET_CRC_CONTROL, EXPECT_BUFFER_COLLAPSE, FALSE) |
    NVVAL(NVCA7D, HEAD_SET_CRC_CONTROL, PRIMARY_CRC, primary_crc) |
    NVDEF(NVCA7D, HEAD_SET_CRC_CONTROL, SECONDARY_CRC, NONE) |
    NVDEF(NVCA7D, HEAD_SET_CRC_CONTROL, CRC_DURING_SNOOZE, DISABLE));
    return 0;
    }
    const struct nv50_crc_func
    crcca7d = {
    .set_src = crcca7d_set_src,
    .set_ctx = crcca7d_set_ctx,
    .get_entry = crcc37d_get_entry,
    .ctx_finished = crcc37d_ctx_finished,
    .flip_threshold = CRCC37D_FLIP_THRESHOLD,
    .num_entries = CRCC37D_MAX_ENTRIES,
    .notifier_len = sizeof(struct crcc37d_notifier),
    };
