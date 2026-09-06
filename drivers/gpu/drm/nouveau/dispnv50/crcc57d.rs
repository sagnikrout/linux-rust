//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv50/crcc57d.c
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

    static int crcc57d_set_src(struct nv50_head *head, int or, enum nv50_crc_source_type source,
    struct nv50_crc_notifier_ctx *ctx)
    {
    struct nvif_push *push = &nv50_disp(head.base.base.dev).core.chan.push;
    let mut i: c_int = head.base.index;
    u32 crc_args = NVDEF(NVC57D, HEAD_SET_CRC_CONTROL, CONTROLLING_CHANNEL, CORE) |
    NVDEF(NVC57D, HEAD_SET_CRC_CONTROL, EXPECT_BUFFER_COLLAPSE, FALSE) |
    NVDEF(NVC57D, HEAD_SET_CRC_CONTROL, SECONDARY_CRC, NONE) |
    NVDEF(NVC57D, HEAD_SET_CRC_CONTROL, CRC_DURING_SNOOZE, DISABLE);
    int ret;
    switch (source) {
    case NV50_CRC_SOURCE_TYPE_SOR:
    crc_args |= NVDEF(NVC57D, HEAD_SET_CRC_CONTROL, PRIMARY_CRC, SOR(or));
    break;
    case NV50_CRC_SOURCE_TYPE_SF:
    crc_args |= NVDEF(NVC57D, HEAD_SET_CRC_CONTROL, PRIMARY_CRC, SF);
    break;
    default:
    break;
    }
    ret = PUSH_WAIT(push, 4);
    if (ret)
    return ret;
    if (source) {
    PUSH_MTHD(push, NVC57D, HEAD_SET_CONTEXT_DMA_CRC(i), ctx.ntfy.handle);
    PUSH_MTHD(push, NVC57D, HEAD_SET_CRC_CONTROL(i), crc_args);
    } else {
    PUSH_MTHD(push, NVC57D, HEAD_SET_CRC_CONTROL(i), 0);
    PUSH_MTHD(push, NVC57D, HEAD_SET_CONTEXT_DMA_CRC(i), 0);
    }
    return 0;
    }
    const struct nv50_crc_func crcc57d = {
    .set_src = crcc57d_set_src,
    .set_ctx = crcc37d_set_ctx,
    .get_entry = crcc37d_get_entry,
    .ctx_finished = crcc37d_ctx_finished,
    .flip_threshold = CRCC37D_FLIP_THRESHOLD,
    .num_entries = CRCC37D_MAX_ENTRIES,
    .notifier_len = sizeof(struct crcc37d_notifier),
    };
