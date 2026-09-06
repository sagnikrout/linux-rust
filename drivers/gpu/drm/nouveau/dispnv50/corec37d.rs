//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv50/corec37d.c
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
// Copyright 2018 Red Hat Inc.
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

    int
    corec37d_wndw_owner(struct nv50_core *core)
    {
    struct nvif_push *push = &core.chan.push;
    const u32 windows = 8; /*XXX*/
    int ret, i;
    if ((ret = PUSH_WAIT(push, windows * 2)))
    return ret;
    for (i = 0; i < windows; i++) {
    PUSH_MTHD(push, NVC37D, WINDOW_SET_CONTROL(i),
    NVDEF(NVC37D, WINDOW_SET_CONTROL, OWNER, HEAD(i >> 1)));
    }
    return 0;
    }
    int
    corec37d_update(struct nv50_core *core, u32 *interlock, bool ntfy)
    {
    struct nvif_push *push = &core.chan.push;
    int ret;
    if ((ret = PUSH_WAIT(push, (ntfy ? 2 * 2 : 0) + 5)))
    return ret;
    if (ntfy) {
    PUSH_MTHD(push, NVC37D, SET_NOTIFIER_CONTROL,
    NVDEF(NVC37D, SET_NOTIFIER_CONTROL, MODE, WRITE) |
    NVVAL(NVC37D, SET_NOTIFIER_CONTROL, OFFSET, NV50_DISP_CORE_NTFY >> 4) |
    NVDEF(NVC37D, SET_NOTIFIER_CONTROL, NOTIFY, ENABLE));
    }
    PUSH_MTHD(push, NVC37D, SET_INTERLOCK_FLAGS, interlock[NV50_DISP_INTERLOCK_CURS],
    SET_WINDOW_INTERLOCK_FLAGS, interlock[NV50_DISP_INTERLOCK_WNDW]);
    PUSH_MTHD(push, NVC37D, UPDATE, 0x00000001 |
    NVDEF(NVC37D, UPDATE, SPECIAL_HANDLING, NONE) |
    NVDEF(NVC37D, UPDATE, INHIBIT_INTERRUPTS, FALSE));
    if (ntfy) {
    PUSH_MTHD(push, NVC37D, SET_NOTIFIER_CONTROL,
    NVDEF(NVC37D, SET_NOTIFIER_CONTROL, NOTIFY, DISABLE));
    }
    return PUSH_KICK(push);
    }
    int
    corec37d_ntfy_wait_done(struct nouveau_bo *bo, u32 offset,
    struct nvif_device *device)
    {
    s64 time = nvif_msec(device, 2000ULL,
    if (NVBO_TD32(bo, offset, NV_DISP_NOTIFIER, _0, STATUS, ==, FINISHED))
    break;
    usleep_range(1, 2);
    );
    return time < 0 ? time : 0;
    }
    void
    corec37d_ntfy_init(struct nouveau_bo *bo, u32 offset)
    {
    NVBO_WR32(bo, offset, NV_DISP_NOTIFIER, _0,
    NVDEF(NV_DISP_NOTIFIER, _0, STATUS, NOT_BEGUN));
    NVBO_WR32(bo, offset, NV_DISP_NOTIFIER, _1, 0);
    NVBO_WR32(bo, offset, NV_DISP_NOTIFIER, _2, 0);
    NVBO_WR32(bo, offset, NV_DISP_NOTIFIER, _3, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn corec37d_caps_init(drm: *mut nouveau_drm, disp: *mut nv50_disp) -> c_int {
    int corec37d_caps_init(struct nouveau_drm *drm, struct nv50_disp *disp)
    {
    int ret;
    ret = nvif_object_ctor(&disp.disp.object, "dispCaps", 0,
    disp.core.func.caps_class, core::ptr::null_mut(), 0, &disp.caps);
    if (ret) {
    NV_ERROR(drm,
    "Failed to init notifier caps region: %d\n",
    ret);
    return ret;
    }
    ret = nvif_object_map(&disp.caps, core::ptr::null_mut(), 0);
    if (ret) {
    NV_ERROR(drm,
    "Failed to map notifier caps region: %d\n",
    ret);
    return ret;
    }
    return 0;
    }
    static int
    corec37d_init(struct nv50_core *core)
    {
    struct nvif_push *push = &core.chan.push;
    const u32 windows = 8; /*XXX*/
    int ret, i;
    if ((ret = PUSH_WAIT(push, 2 + windows * 5)))
    return ret;
    PUSH_MTHD(push, NVC37D, SET_CONTEXT_DMA_NOTIFIER, core.chan.sync.handle);
    for (i = 0; i < windows; i++) {
    PUSH_MTHD(push, NVC37D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS(i),
    NVDEF(NVC37D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS, RGB_PACKED1BPP, TRUE) |
    NVDEF(NVC37D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS, RGB_PACKED2BPP, TRUE) |
    NVDEF(NVC37D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS, RGB_PACKED4BPP, TRUE) |
    NVDEF(NVC37D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS, RGB_PACKED8BPP, TRUE) |
    NVDEF(NVC37D, WINDOW_SET_WINDOW_FORMAT_USAGE_BOUNDS, YUV_PACKED422, TRUE),
    WINDOW_SET_WINDOW_ROTATED_FORMAT_USAGE_BOUNDS(i), 0x00000000);
    PUSH_MTHD(push, NVC37D, WINDOW_SET_WINDOW_USAGE_BOUNDS(i),
    NVVAL(NVC37D, WINDOW_SET_WINDOW_USAGE_BOUNDS, MAX_PIXELS_FETCHED_PER_LINE, 0x7fff) |
    NVDEF(NVC37D, WINDOW_SET_WINDOW_USAGE_BOUNDS, INPUT_LUT, USAGE_1025) |
    NVDEF(NVC37D, WINDOW_SET_WINDOW_USAGE_BOUNDS, INPUT_SCALER_TAPS, TAPS_2) |
    NVDEF(NVC37D, WINDOW_SET_WINDOW_USAGE_BOUNDS, UPSCALING_ALLOWED, FALSE));
    }
    core.assign_windows = true;
    return PUSH_KICK(push);
    }
    static const struct nv50_core_func
    corec37d = {
    .init = corec37d_init,
    .ntfy_init = corec37d_ntfy_init,
    .caps_init = corec37d_caps_init,
    .caps_class = GV100_DISP_CAPS,
    .ntfy_wait_done = corec37d_ntfy_wait_done,
    .update = corec37d_update,
    .wndw.owner = corec37d_wndw_owner,
    .head = &headc37d,
    .sor = &sorc37d,

    .crc = &crcc37d,

    };
    int
    corec37d_new(struct nouveau_drm *drm, s32 oclass, struct nv50_core **pcore)
    {
    return core507d_new_(&corec37d, drm, oclass, pcore);
    }
