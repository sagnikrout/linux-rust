//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/core/dc_surface.c
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
// Copyright 2015 Advanced Micro Devices, Inc.
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
// Authors: AMD
//
// DC interface (public)

// DC core (private)

//
// Private functions
//
#[no_mangle]
pub unsafe extern "C" fn dc_plane_construct(ctx: *mut dc_context, plane_state: *mut dc_plane_state) {
    void dc_plane_construct(struct dc_context *ctx, struct dc_plane_state *plane_state)
    {
    plane_state.ctx = ctx;
    plane_state.gamma_correction.is_identity = true;
    plane_state.in_transfer_func.type = TF_TYPE_BYPASS;
    plane_state.pre_multiplied_alpha = true;
// CM
    plane_state.cm.shaper_func.type = TF_TYPE_BYPASS;
    plane_state.cm.blend_func.type = TF_TYPE_BYPASS;
    plane_state.cm.lut3d_func.state.raw = 0;
    plane_state.cm.flags.all = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_plane_destruct(plane_state: *mut dc_plane_state) {
    void dc_plane_destruct(struct dc_plane_state *plane_state)
    {
    (void)plane_state;
// no more pointers to free within dc_plane_state
    }
// dc_state is passed in separately since it may differ from the current dc state accessible from plane_state e.g.
// if the driver is doing an update from an old context to a new one and the caller wants the pipe mask for the new
// context rather than the existing one
//
#[no_mangle]
pub unsafe extern "C" fn dc_plane_get_pipe_mask(dc_state: *mut dc_state, plane_state: *const dc_plane_state) -> u8 {
    uint8_t  dc_plane_get_pipe_mask(struct dc_state *dc_state, const struct dc_plane_state *plane_state)
    {
    let mut pipe_mask: u8 = 0;
    unsigned int i;
    for (i = 0; i < plane_state.ctx.dc.res_pool.pipe_count; i++) {
    struct pipe_ctx *pipe_ctx = &dc_state.res_ctx.pipe_ctx[i];
    if (pipe_ctx.plane_state == plane_state && pipe_ctx.plane_res.hubp)
    pipe_mask |= (uint8_t)(1 << pipe_ctx.plane_res.hubp.inst);
    }
    return pipe_mask;
    }
//
// Public functions
//
    struct dc_plane_state *dc_create_plane_state(const struct dc *dc)
    {
    struct dc_plane_state *plane_state = kvzalloc_obj(*plane_state,
    GFP_ATOMIC);
    if (core::ptr::null_mut() == plane_state)
    return core::ptr::null_mut();
    kref_init(&plane_state.refcount);
    dc_plane_construct(dc.ctx, plane_state);
    return plane_state;
    }
//
// Function: dc_plane_get_status
//
// @brief
// Looks up the pipe context of plane_state and updates the pending status
// of the pipe context. Then returns plane_state->status
//
// @param [in] plane_state: pointer to the plane_state to get the status of
//
    const struct dc_plane_status *dc_plane_get_status(
    const struct dc_plane_state *plane_state,
    union dc_plane_status_update_flags flags)
    {
    const struct dc_plane_status *plane_status;
    struct dc  *dc;
    unsigned int i;
    if (!plane_state ||
    !plane_state.ctx ||
    !plane_state.ctx.dc) {
    ASSERT(0);
    return core::ptr::null_mut(); /* remove this if above assert never hit */
    }
    plane_status = &plane_state.status;
    dc = plane_state.ctx.dc;
    if (dc.current_state == core::ptr::null_mut())
    return core::ptr::null_mut();
// Find the current plane state and set its pending bit to false
    for (i = 0; i < dc.res_pool.pipe_count; i++) {
    struct pipe_ctx *pipe_ctx =
    &dc.current_state.res_ctx.pipe_ctx[i];
    if (pipe_ctx.plane_state != plane_state)
    continue;
    if (pipe_ctx.plane_state && flags.bits.address)
    pipe_ctx.plane_state.status.is_flip_pending = false;
    if (pipe_ctx.plane_state && flags.bits.histogram)
    memset(&pipe_ctx.plane_state.status.cm_hist, 0,
    sizeof(pipe_ctx.plane_state.status.cm_hist));
    break;
    }
    dc_exit_ips_for_hw_access(dc);
    for (i = 0; i < dc.res_pool.pipe_count; i++) {
    struct pipe_ctx *pipe_ctx =
    &dc.current_state.res_ctx.pipe_ctx[i];
    if (pipe_ctx.plane_state != plane_state)
    continue;
    if (flags.bits.address)
    dc.hwss.update_pending_status(pipe_ctx);
    if (flags.bits.histogram) {
    struct dpp *dpp = pipe_ctx.plane_res.dpp;
    if (dpp && dpp.funcs.dpp_cm_hist_read)
    dpp.funcs.dpp_cm_hist_read(dpp, &pipe_ctx.plane_state.status.cm_hist);
    }
    }
    return plane_status;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_plane_state_retain(plane_state: *mut dc_plane_state) {
    void dc_plane_state_retain(struct dc_plane_state *plane_state)
    {
    kref_get(&plane_state.refcount);
    }
#[no_mangle]
unsafe extern "C" fn dc_plane_state_free(kref: *mut kref) {
    static void dc_plane_state_free(struct kref *kref)
    {
    struct dc_plane_state *plane_state = container_of(kref, struct dc_plane_state, refcount);
    dc_plane_destruct(plane_state);
    kvfree(plane_state);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_plane_state_release(plane_state: *mut dc_plane_state) {
    void dc_plane_state_release(struct dc_plane_state *plane_state)
    {
    kref_put(&plane_state.refcount, dc_plane_state_free);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_gamma_retain(gamma: *mut dc_gamma) {
    void dc_gamma_retain(struct dc_gamma *gamma)
    {
    kref_get(&gamma.refcount);
    }
#[no_mangle]
unsafe extern "C" fn dc_gamma_free(kref: *mut kref) {
    static void dc_gamma_free(struct kref *kref)
    {
    struct dc_gamma *gamma = container_of(kref, struct dc_gamma, refcount);
    kvfree(gamma);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_gamma_release(gamma: *mut dc_gamma) {
    void dc_gamma_release(struct dc_gamma **gamma)
    {
    kref_put(&(*gamma).refcount, dc_gamma_free);
// gamma = NULL;
    }
    struct dc_gamma *dc_create_gamma(void)
    {
    struct dc_gamma *gamma = kvzalloc_obj(*gamma);
    if (gamma == core::ptr::null_mut())
    goto alloc_fail;
    kref_init(&gamma.refcount);
    return gamma;
    alloc_fail:
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dc_transfer_func_retain(tf: *mut dc_transfer_func) {
    void dc_transfer_func_retain(struct dc_transfer_func *tf)
    {
    kref_get(&tf.refcount);
    }
#[no_mangle]
unsafe extern "C" fn dc_transfer_func_free(kref: *mut kref) {
    static void dc_transfer_func_free(struct kref *kref)
    {
    struct dc_transfer_func *tf = container_of(kref, struct dc_transfer_func, refcount);
    kvfree(tf);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_transfer_func_release(tf: *mut dc_transfer_func) {
    void dc_transfer_func_release(struct dc_transfer_func *tf)
    {
    kref_put(&tf.refcount, dc_transfer_func_free);
    }
    struct dc_transfer_func *dc_create_transfer_func(void)
    {
    struct dc_transfer_func *tf = kvzalloc_obj(*tf);
    if (tf == core::ptr::null_mut())
    goto alloc_fail;
    kref_init(&tf.refcount);
    return tf;
    alloc_fail:
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dc_3dlut_func_free(kref: *mut kref) {
    static void dc_3dlut_func_free(struct kref *kref)
    {
    struct dc_3dlut *lut = container_of(kref, struct dc_3dlut, refcount);
    kvfree(lut);
    }
    struct dc_3dlut *dc_create_3dlut_func(void)
    {
    struct dc_3dlut *lut = kvzalloc_obj(*lut);
    if (lut == core::ptr::null_mut())
    goto alloc_fail;
    kref_init(&lut.refcount);
    lut.state.raw = 0;
    return lut;
    alloc_fail:
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dc_3dlut_func_release(lut: *mut dc_3dlut) {
    void dc_3dlut_func_release(struct dc_3dlut *lut)
    {
    kref_put(&lut.refcount, dc_3dlut_func_free);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_3dlut_func_retain(lut: *mut dc_3dlut) {
    void dc_3dlut_func_retain(struct dc_3dlut *lut)
    {
    kref_get(&lut.refcount);
    }
#[no_mangle]
unsafe extern "C" fn dc_plane_cm_free(kref: *mut kref) {
    static void dc_plane_cm_free(struct kref *kref)
    {
    struct dc_plane_cm *cm = container_of(kref, struct dc_plane_cm, refcount);
    kvfree(cm);
    }
    struct dc_plane_cm *dc_plane_cm_create(void)
    {
    struct dc_plane_cm *cm = kvzalloc_obj(*cm);
    if (cm == core::ptr::null_mut())
    goto alloc_fail;
    kref_init(&cm.refcount);
    return cm;
    alloc_fail:
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dc_plane_cm_release(cm: *mut dc_plane_cm) {
    void dc_plane_cm_release(struct dc_plane_cm *cm)
    {
    kref_put(&cm.refcount, dc_plane_cm_free);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_plane_cm_retain(cm: *mut dc_plane_cm) {
    void dc_plane_cm_retain(struct dc_plane_cm *cm)
    {
    kref_get(&cm.refcount);
    }
    void dc_plane_force_dcc_and_tiling_disable(struct dc_plane_state *plane_state,
    bool clear_tiling)
    {
    struct dc *dc;
    unsigned int i;
    if (!plane_state)
    return;
    dc = plane_state.ctx.dc;
    if (!dc || !dc.current_state)
    return;
    for (i = 0; i < dc.res_pool.pipe_count; i++) {
    struct pipe_ctx *pipe_ctx = &dc.current_state.res_ctx.pipe_ctx[i];
    if (!pipe_ctx)
    continue;
    if (dc.hwss.clear_surface_dcc_and_tiling)
    dc.hwss.clear_surface_dcc_and_tiling(pipe_ctx, plane_state, clear_tiling);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dc_plane_copy_config(dst: *mut dc_plane_state, src: *const dc_plane_state) {
    void dc_plane_copy_config(struct dc_plane_state *dst, const struct dc_plane_state *src)
    {
    struct kref temp_refcount;
// backup persistent info
    memcpy(&temp_refcount, &dst.refcount, sizeof(struct kref));
// copy all configuration information
    memcpy(dst, src, sizeof(struct dc_plane_state));
// restore persistent info
    memcpy(&dst.refcount, &temp_refcount, sizeof(struct kref));
    }
