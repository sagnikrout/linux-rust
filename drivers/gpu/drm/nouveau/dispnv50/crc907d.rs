//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv50/crc907d.c
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

pub const CRC907D_MAX_ENTRIES: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc907d_notifier {
    pub status: u32,
    pub /: *mut *mut u32 :32; / reserved,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc907d_entry {
    pub status: u32,
    pub compositor_crc: u32,
    pub output_crc: [u32; 2],
    pub entries: [}; CRC907D_MAX_ENTRIES],
    pub __packed: },
    static int
    crc907d_set_src(struct nv50_head *head, int or, enum nv50_crc_source_type source,
    struct nv50_crc_notifier_ctx *ctx)
    {
    pub &nv50_disp(head->base.base.dev)->core->chan.push: *mut *mut nvif_push push =,
    pub head->base.index: int i =,
    u32 crc_args = NVDEF(NV907D, HEAD_SET_CRC_CONTROL, CONTROLLING_CHANNEL, CORE) |
    NVDEF(NV907D, HEAD_SET_CRC_CONTROL, EXPECT_BUFFER_COLLAPSE, FALSE) |
    NVDEF(NV907D, HEAD_SET_CRC_CONTROL, TIMESTAMP_MODE, FALSE) |
    NVDEF(NV907D, HEAD_SET_CRC_CONTROL, SECONDARY_OUTPUT, NONE) |
    NVDEF(NV907D, HEAD_SET_CRC_CONTROL, CRC_DURING_SNOOZE, DISABLE) |
    pub ENABLE): NVDEF(NV907D, HEAD_SET_CRC_CONTROL, WIDE_PIPE_CRC,,
    pub ret: c_int,
    switch (source) {
    case NV50_CRC_SOURCE_TYPE_SOR:
    pub SOR(or)): crc_args |= NVDEF(NV907D, HEAD_SET_CRC_CONTROL, PRIMARY_OUTPUT,,
    case NV50_CRC_SOURCE_TYPE_PIOR:
    pub PIOR(or)): crc_args |= NVDEF(NV907D, HEAD_SET_CRC_CONTROL, PRIMARY_OUTPUT,,
    case NV50_CRC_SOURCE_TYPE_DAC:
    pub DAC(or)): crc_args |= NVDEF(NV907D, HEAD_SET_CRC_CONTROL, PRIMARY_OUTPUT,,
    case NV50_CRC_SOURCE_TYPE_RG:
    pub RG(i)): crc_args |= NVDEF(NV907D, HEAD_SET_CRC_CONTROL, PRIMARY_OUTPUT,,
    case NV50_CRC_SOURCE_TYPE_SF:
    pub SF(i)): crc_args |= NVDEF(NV907D, HEAD_SET_CRC_CONTROL, PRIMARY_OUTPUT,,
    case NV50_CRC_SOURCE_NONE:
    pub NONE): crc_args |= NVDEF(NV907D, HEAD_SET_CRC_CONTROL, PRIMARY_OUTPUT,,
    }
    if ((ret = PUSH_WAIT(push, 4)))
    pub ret: return,
    if (source) {
    pub ctx->ntfy.handle): PUSH_MTHD(push, NV907D, HEAD_SET_CONTEXT_DMA_CRC(i),,
    pub crc_args): PUSH_MTHD(push, NV907D, HEAD_SET_CRC_CONTROL(i),,
    } else {
    pub crc_args): PUSH_MTHD(push, NV907D, HEAD_SET_CRC_CONTROL(i),,
    pub 0): PUSH_MTHD(push, NV907D, HEAD_SET_CONTEXT_DMA_CRC(i),,
    }
    pub 0: return,
    }
    static int
    crc907d_set_ctx(struct nv50_head *head, struct nv50_crc_notifier_ctx *ctx)
    {
    pub &nv50_disp(head->base.base.dev)->core->chan.push: *mut *mut nvif_push push =,
    pub head->base.index: int i =,
    pub ret: c_int,
    if ((ret = PUSH_WAIT(push, 2)))
    pub ret: return,
    pub 0): PUSH_MTHD(push, NV907D, HEAD_SET_CONTEXT_DMA_CRC(i), ctx ? ctx->ntfy.handle :,
    pub 0: return,
    }
    static u32 crc907d_get_entry(struct nv50_head *head,
    struct nv50_crc_notifier_ctx *ctx,
    enum nv50_crc_source source, int idx)
    {
    pub ctx->mem.object.map.ptr: *mut *mut crc907d_notifier __iomem notifier =,
    pub ioread32_native(&notifier->entries[idx].output_crc[0]): return,
    }
    static bool crc907d_ctx_finished(struct nv50_head *head,
    struct nv50_crc_notifier_ctx *ctx)
    {
    pub nouveau_drm(head->base.base.dev): *mut *mut nouveau_drm drm =,
    pub ctx->mem.object.map.ptr: *mut *mut crc907d_notifier __iomem notifier =,
    pub ioread32_native(&notifier->status): u32 status =,
    pub 0x0000003e: u32 overflow = status &,
    if (!(status & 0x00000001))
    pub false: return,
    if (overflow) {
    pub NULL: *const *const char engine =,
    switch (overflow) {
    pub break: case 0x00000004: engine = "DSI";,
    pub break: case 0x00000008: engine = "Compositor";,
    pub break: case 0x00000010: engine = "CRC output 1";,
    pub break: case 0x00000020: engine = "CRC output 2";,
    }
    if (engine)
    NV_ERROR(drm,
    "CRC notifier context for head %d overflowed on %s: %x\n",
    pub status): head->base.index, engine,,
    else
    NV_ERROR(drm,
    "CRC notifier context for head %d overflowed: %x\n",
    pub status): head->base.index,,
    }
    NV_DEBUG(drm, "Head %d CRC context status: %x\n",
    pub status): head->base.index,,
    pub true: return,
    }
    const struct nv50_crc_func crc907d = {
    .set_src = crc907d_set_src,
    .set_ctx = crc907d_set_ctx,
    .get_entry = crc907d_get_entry,
    .ctx_finished = crc907d_ctx_finished,
    .flip_threshold = CRC907D_MAX_ENTRIES - 10,
    .num_entries = CRC907D_MAX_ENTRIES,
    .notifier_len = sizeof(struct crc907d_notifier),
}
