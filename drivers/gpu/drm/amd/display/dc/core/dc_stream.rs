//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/core/dc_stream.c
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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

//
// Private functions
//
#[no_mangle]
pub unsafe extern "C" fn update_stream_signal(stream: *mut dc_stream_state, sink: *mut dc_sink) {
    void update_stream_signal(struct dc_stream_state *stream, struct dc_sink *sink)
    {
    unsigned int pix_clk;
    if (sink.sink_signal == SIGNAL_TYPE_NONE)
    stream.signal = stream.link.connector_signal;
    else
    stream.signal = sink.sink_signal;
    if (dc_is_dvi_signal(stream.signal)) {
    if (stream.ctx.dc.caps.dual_link_dvi &&
    (stream.timing.pix_clk_100hz / 10) > TMDS_MAX_PIXEL_CLOCK &&
    sink.sink_signal != SIGNAL_TYPE_DVI_SINGLE_LINK)
    stream.signal = SIGNAL_TYPE_DVI_DUAL_LINK;
    else
    stream.signal = SIGNAL_TYPE_DVI_SINGLE_LINK;
    }
    if (dc_is_hdmi_frl_signal(stream.signal)) {
    pix_clk = stream.timing.pix_clk_100hz / 10;
    if (stream.timing.pixel_encoding == PIXEL_ENCODING_YCBCR420)
    pix_clk /= 2;
// YCbCr422 to use assume 12-bit interface always, clock stays the same
    if (stream.timing.pixel_encoding != PIXEL_ENCODING_YCBCR422) {
    switch (stream.timing.display_color_depth) {
    case COLOR_DEPTH_666:
    case COLOR_DEPTH_888:
    break;
    case COLOR_DEPTH_101010:
    pix_clk = pix_clk * 10 / 8;
    break;
    case COLOR_DEPTH_121212:
    pix_clk = pix_clk * 12 / 8;
    break;
    default:
    break;
    }
    }
    if (pix_clk != 0 && pix_clk < HDMI2_TMDS_MAX_PIXEL_CLOCK)
    stream.signal = SIGNAL_TYPE_HDMI_TYPE_A;
    if (stream.timing.pixel_encoding == PIXEL_ENCODING_YCBCR420 &&
    stream.timing.h_addressable > 4096)
    stream.signal = SIGNAL_TYPE_HDMI_FRL;
    if (stream.timing.rid != 0)
    stream.signal = SIGNAL_TYPE_HDMI_FRL;
    if (stream.link.frl_flags.force_frl_always ||
    stream.link.frl_flags.force_frl_max
    || stream.link.frl_flags.force_frl_dsc
    || (stream.link.frl_flags.force_frl_rate != 0 &&
    stream.link.frl_flags.force_frl_rate != 0xF))
    stream.signal = SIGNAL_TYPE_HDMI_FRL;
    }
    }
    bool dc_stream_construct(struct dc_stream_state *stream,
    struct dc_sink *dc_sink_data)
    {
    let mut i: u32 = 0;
    stream.sink = dc_sink_data;
    dc_sink_retain(dc_sink_data);
    stream.ctx = dc_sink_data.ctx;
    stream.link = dc_sink_data.link;
    stream.sink_patches = dc_sink_data.edid_caps.panel_patch;
    stream.converter_disable_audio = dc_sink_data.converter_disable_audio;
    stream.qs_bit = dc_sink_data.edid_caps.qs_bit;
    stream.qy_bit = dc_sink_data.edid_caps.qy_bit;
// Copy audio modes
// TODO - Remove this translation
    for (i = 0; i < (dc_sink_data.edid_caps.audio_mode_count); i++) {
    stream.audio_info.modes[i].channel_count = dc_sink_data.edid_caps.audio_modes[i].channel_count;
    stream.audio_info.modes[i].format_code = dc_sink_data.edid_caps.audio_modes[i].format_code;
    stream.audio_info.modes[i].sample_rates.all = dc_sink_data.edid_caps.audio_modes[i].sample_rate;
    stream.audio_info.modes[i].sample_size = dc_sink_data.edid_caps.audio_modes[i].sample_size;
    }
    stream.audio_info.mode_count = dc_sink_data.edid_caps.audio_mode_count;
    stream.audio_info.audio_latency = dc_sink_data.edid_caps.audio_latency;
    stream.audio_info.video_latency = dc_sink_data.edid_caps.video_latency;
    memmove(
    stream.audio_info.display_name,
    dc_sink_data.edid_caps.display_name,
    AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS);
    stream.audio_info.manufacture_id = dc_sink_data.edid_caps.manufacturer_id;
    stream.audio_info.product_id = dc_sink_data.edid_caps.product_id;
    stream.audio_info.flags.all = dc_sink_data.edid_caps.speaker_flags;
    if (dc_sink_data.dc_container_id != core::ptr::null_mut()) {
    struct dc_container_id *dc_container_id = dc_sink_data.dc_container_id;
    stream.audio_info.port_id[0] = dc_container_id.portId[0];
    stream.audio_info.port_id[1] = dc_container_id.portId[1];
    } else {
// TODO - WindowDM has implemented,
    other DMs need Unhardcode port_id */
    stream.audio_info.port_id[0] = 0x5558859e;
    stream.audio_info.port_id[1] = 0xd989449;
    }
// EDID CAP translation for HDMI 2.0
    stream.timing.flags.LTE_340MCSC_SCRAMBLE = dc_sink_data.edid_caps.lte_340mcsc_scramble;
    memset(&stream.timing.dsc_cfg, 0, sizeof(stream.timing.dsc_cfg));
    stream.timing.dsc_cfg.num_slices_h = 0;
    stream.timing.dsc_cfg.num_slices_v = 0;
    stream.timing.dsc_cfg.bits_per_pixel = 128;
    stream.timing.dsc_cfg.block_pred_enable = 1;
    stream.timing.dsc_cfg.linebuf_depth = 9;
    stream.timing.dsc_cfg.version_minor = 2;
    stream.timing.dsc_cfg.ycbcr422_simple = 0;
    update_stream_signal(stream, dc_sink_data);
    stream.out_transfer_func.type = TF_TYPE_BYPASS;
    dc_stream_assign_stream_id(stream);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_destruct(stream: *mut dc_stream_state) {
    void dc_stream_destruct(struct dc_stream_state *stream)
    {
    dc_sink_release(stream.sink);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_assign_stream_id(stream: *mut dc_stream_state) {
    void dc_stream_assign_stream_id(struct dc_stream_state *stream)
    {
// MSB is reserved to indicate phantoms
    stream.stream_id = stream.ctx.dc_stream_id_count;
    stream.ctx.dc_stream_id_count++;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_retain(stream: *mut dc_stream_state) {
    void dc_stream_retain(struct dc_stream_state *stream)
    {
    kref_get(&stream.refcount);
    }
#[no_mangle]
unsafe extern "C" fn dc_stream_free(kref: *mut kref) {
    static void dc_stream_free(struct kref *kref)
    {
    struct dc_stream_state *stream = container_of(kref, struct dc_stream_state, refcount);
    dc_stream_destruct(stream);
    kfree(stream);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_release(stream: *mut dc_stream_state) {
    void dc_stream_release(struct dc_stream_state *stream)
    {
    if (stream != core::ptr::null_mut()) {
    kref_put(&stream.refcount, dc_stream_free);
    }
    }
    EXPORT_IF_KUNIT(dc_stream_release);
    struct dc_stream_state *dc_create_stream_for_sink(
    struct dc_sink *sink)
    {
    struct dc_stream_state *stream = core::ptr::null_mut();
    if (sink == core::ptr::null_mut())
    goto fail;
    DC_RUN_WITH_PREEMPTION_ENABLED(stream = kzalloc_obj(struct dc_stream_state, GFP_ATOMIC));
    if (stream == core::ptr::null_mut())
    goto fail;
    if (dc_stream_construct(stream, sink) == false)
    goto fail;
    kref_init(&stream.refcount);
    return stream;
    fail:
    if (stream)
    kfree(stream);
    return core::ptr::null_mut();
    }
    struct dc_stream_state *dc_copy_stream(const struct dc_stream_state *stream)
    {
    struct dc_stream_state *new_stream;
    new_stream = kmemdup(stream, sizeof(struct dc_stream_state), GFP_KERNEL);
    if (!new_stream)
    return core::ptr::null_mut();
    if (new_stream.sink)
    dc_sink_retain(new_stream.sink);
    dc_stream_assign_stream_id(new_stream);
// If using dynamic encoder assignment, wait till stream committed to assign encoder.
    if (new_stream.ctx.dc.res_pool.funcs.link_encs_assign &&
    !new_stream.ctx.dc.config.unify_link_enc_assignment)
    new_stream.link_enc = core::ptr::null_mut();
    kref_init(&new_stream.refcount);
    return new_stream;
    }
//
// dc_stream_get_status() - Get current stream status of the given stream state
// @stream: The stream to get the stream status for.
//
// The given stream is expected to exist in dc->current_state. Otherwise, NULL
// will be returned.
//
    struct dc_stream_status *dc_stream_get_status(
    struct dc_stream_state *stream)
    {
    struct dc *dc = stream.ctx.dc;
    return dc_state_get_stream_status(dc.current_state, stream);
    }
    const struct dc_stream_status *dc_stream_get_status_const(
    const struct dc_stream_state *stream)
    {
    struct dc *dc = stream.ctx.dc;
    return dc_state_get_stream_status(dc.current_state, stream);
    }
    struct dc_link *dc_stream_get_link(
    const struct dc_stream_state *stream)
    {
    return stream.link;
    }
    void program_cursor_attributes(
    struct dc *dc,
    struct dc_stream_state *stream)
    {
    uint8_t i;
    struct resource_context *res_ctx;
    struct pipe_ctx *pipe_to_program = core::ptr::null_mut();
    let mut enable_cursor_offload: bool = dc_dmub_srv_is_cursor_offload_enabled(dc);
    if (!stream)
    return;
    res_ctx = &dc.current_state.res_ctx;
    for (i = 0; i < MAX_PIPES; i++) {
    struct pipe_ctx *pipe_ctx = &res_ctx.pipe_ctx[i];
    if (pipe_ctx.stream != stream)
    continue;
    if (!pipe_to_program) {
    pipe_to_program = pipe_ctx;
    if (enable_cursor_offload && dc.hwss.begin_cursor_offload_update) {
    dc.hwss.begin_cursor_offload_update(dc, pipe_ctx);
    } else {
    dc.hwss.cursor_lock(dc, pipe_to_program, true);
    if (pipe_to_program.next_odm_pipe)
    dc.hwss.cursor_lock(dc, pipe_to_program.next_odm_pipe, true);
    }
    }
    dc.hwss.set_cursor_attribute(pipe_ctx);
    if (dc.ctx.dmub_srv)
    dc_send_update_cursor_info_to_dmu(pipe_ctx, i);
    if (dc.hwss.set_cursor_sdr_white_level)
    dc.hwss.set_cursor_sdr_white_level(pipe_ctx);
    if (enable_cursor_offload && dc.hwss.update_cursor_offload_pipe)
    dc.hwss.update_cursor_offload_pipe(dc, pipe_ctx);
    }
    if (pipe_to_program) {
    if (enable_cursor_offload && dc.hwss.commit_cursor_offload_update) {
    dc.hwss.commit_cursor_offload_update(dc, pipe_to_program);
    } else {
    dc.hwss.cursor_lock(dc, pipe_to_program, false);
    if (pipe_to_program.next_odm_pipe)
    dc.hwss.cursor_lock(dc, pipe_to_program.next_odm_pipe, false);
    }
    }
    }
//
// dc_stream_check_cursor_attributes() - Check validitity of cursor attributes and surface address
//
    bool dc_stream_check_cursor_attributes(
    const struct dc_stream_state *stream,
    struct dc_state *state,
    const struct dc_cursor_attributes *attributes)
    {
    const struct dc *dc;
    unsigned int max_cursor_size;
    if (core::ptr::null_mut() == stream) {
    dm_error("DC: dc_stream is core::ptr::null_mut()!\n");
    return false;
    }
    if (core::ptr::null_mut() == attributes) {
    dm_error("DC: attributes is core::ptr::null_mut()!\n");
    return false;
    }
    if (attributes.address.quad_part == 0) {
    dm_output_to_console("DC: Cursor address is 0!\n");
    return false;
    }
    dc = stream.ctx.dc;
// SubVP is not compatible with HW cursor larger than what can fit in cursor SRAM.
// Therefore, if cursor is greater than this, fallback to SW cursor.
//
    if (dc.debug.allow_sw_cursor_fallback && dc.res_pool.funcs.get_max_hw_cursor_size) {
    max_cursor_size = dc.res_pool.funcs.get_max_hw_cursor_size(dc, state, stream);
    max_cursor_size = max_cursor_size * max_cursor_size * 4;
    if (attributes.height * attributes.width * 4 > max_cursor_size) {
    return false;
    }
    }
    return true;
    }
//
// dc_stream_set_cursor_attributes() - Update cursor attributes and set cursor surface address
//
    bool dc_stream_set_cursor_attributes(
    struct dc_stream_state *stream,
    const struct dc_cursor_attributes *attributes)
    {
    let mut result: bool = false;
    if (!stream)
    return false;
    if (dc_stream_check_cursor_attributes(stream, stream.ctx.dc.current_state, attributes)) {
    stream.cursor_attributes = *attributes;
    result = true;
    }
    return result;
    }
    bool dc_stream_program_cursor_attributes(
    struct dc_stream_state *stream,
    const struct dc_cursor_attributes *attributes)
    {
    struct dc  *dc;
    let mut reset_idle_optimizations: bool = false;
    let mut should_release_dmub_hw_control_lock: bool = false;
    if (!stream)
    return false;
    dc = stream.ctx.dc;
    if (dc_stream_set_cursor_attributes(stream, attributes)) {
    dc_z10_restore(dc);
    if (dc.hwss.dmub_hw_control_lock) {
    if (dc_state_is_alt_in_use(dc, dc.current_state) &&
    !dc_dmub_srv_is_cursor_offload_enabled(dc)) {
    dc.hwss.dmub_hw_control_lock(dc, dc.current_state, true);
    should_release_dmub_hw_control_lock = true;
    }
    }
// disable idle optimizations while updating cursor
    if (dc.idle_optimizations_allowed) {
    dc_allow_idle_optimizations(dc, false);
    reset_idle_optimizations = true;
    }
    program_cursor_attributes(dc, stream);
// re-enable idle optimizations if necessary
    if (reset_idle_optimizations && !dc.debug.disable_dmub_reallow_idle)
    dc_allow_idle_optimizations(dc, true);
    if (dc.hwss.dmub_hw_control_lock) {
    if (should_release_dmub_hw_control_lock)
    dc.hwss.dmub_hw_control_lock(dc, dc.current_state, false);
    }
    return true;
    }
    return false;
    }
    void program_cursor_position(
    struct dc *dc,
    struct dc_stream_state *stream)
    {
    uint8_t i;
    struct resource_context *res_ctx;
    struct pipe_ctx *pipe_to_program = core::ptr::null_mut();
    let mut enable_cursor_offload: bool = dc_dmub_srv_is_cursor_offload_enabled(dc);
    if (!stream)
    return;
    res_ctx = &dc.current_state.res_ctx;
    for (i = 0; i < MAX_PIPES; i++) {
    struct pipe_ctx *pipe_ctx = &res_ctx.pipe_ctx[i];
    if (pipe_ctx.stream != stream ||
    (!pipe_ctx.plane_res.mi  && !pipe_ctx.plane_res.hubp) ||
    !pipe_ctx.plane_state ||
    (!pipe_ctx.plane_res.xfm && !pipe_ctx.plane_res.dpp) ||
    (!pipe_ctx.plane_res.ipp && !pipe_ctx.plane_res.dpp))
    continue;
    if (!pipe_to_program) {
    pipe_to_program = pipe_ctx;
    if (enable_cursor_offload && dc.hwss.begin_cursor_offload_update)
    dc.hwss.begin_cursor_offload_update(dc, pipe_ctx);
    else
    dc.hwss.cursor_lock(dc, pipe_to_program, true);
    }
    dc.hwss.set_cursor_position(pipe_ctx);
    if (enable_cursor_offload && dc.hwss.update_cursor_offload_pipe)
    dc.hwss.update_cursor_offload_pipe(dc, pipe_ctx);
    if (dc.ctx.dmub_srv)
    dc_send_update_cursor_info_to_dmu(pipe_ctx, i);
    }
    if (pipe_to_program) {
    if (enable_cursor_offload && dc.hwss.commit_cursor_offload_update)
    dc.hwss.commit_cursor_offload_update(dc, pipe_to_program);
    else
    dc.hwss.cursor_lock(dc, pipe_to_program, false);
    }
    }
    bool dc_stream_set_cursor_position(
    struct dc_stream_state *stream,
    const struct dc_cursor_position *position)
    {
    if (core::ptr::null_mut() == stream) {
    dm_error("DC: dc_stream is core::ptr::null_mut()!\n");
    return false;
    }
    if (core::ptr::null_mut() == position) {
    dm_error("DC: cursor position is core::ptr::null_mut()!\n");
    return false;
    }
    stream.cursor_position = *position;
    return true;
    }
    bool dc_stream_program_cursor_position(
    struct dc_stream_state *stream,
    const struct dc_cursor_position *position)
    {
    struct dc *dc;
    let mut reset_idle_optimizations: bool = false;
    const struct dc_cursor_position *old_position;
    let mut should_release_dmub_hw_control_lock: bool = false;
    if (!stream)
    return false;
    old_position = &stream.cursor_position;
    dc = stream.ctx.dc;
    if (dc_stream_set_cursor_position(stream, position)) {
    dc_z10_restore(dc);
    if (dc.hwss.dmub_hw_control_lock) {
    if (dc_state_is_alt_in_use(dc, dc.current_state) &&
    !dc_dmub_srv_is_cursor_offload_enabled(dc)) {
    dc.hwss.dmub_hw_control_lock(dc, dc.current_state, true);
    should_release_dmub_hw_control_lock = true;
    }
    }
// disable idle optimizations if enabling cursor
    if (dc.idle_optimizations_allowed &&
    (!old_position.enable || dc.debug.exit_idle_opt_for_cursor_updates) &&
    position.enable) {
    dc_allow_idle_optimizations(dc, false);
    reset_idle_optimizations = true;
    }
    program_cursor_position(dc, stream);
// re-enable idle optimizations if necessary
    if (reset_idle_optimizations && !dc.debug.disable_dmub_reallow_idle)
    dc_allow_idle_optimizations(dc, true);
// apply/update visual confirm
    if (dc.debug.visual_confirm == VISUAL_CONFIRM_HW_CURSOR) {
// update software state
    unsigned int i;
    for (i = 0; i < dc.res_pool.pipe_count; i++) {
    struct pipe_ctx *pipe_ctx = &dc.current_state.res_ctx.pipe_ctx[i];
// adjust visual confirm color for all pipes with current stream
    if (stream == pipe_ctx.stream) {
    get_cursor_visual_confirm_color(pipe_ctx, &(pipe_ctx.visual_confirm_color));
// programming hardware
    if (pipe_ctx.plane_state)
    dc.hwss.update_visual_confirm_color(dc, pipe_ctx,
    pipe_ctx.plane_res.hubp.mpcc_id);
    }
    }
    }
    if (stream.drr_trigger_mode == DRR_TRIGGER_ON_FLIP_AND_CURSOR) {
// apply manual trigger
    unsigned int i;
    for (i = 0; i < dc.res_pool.pipe_count; i++) {
    struct pipe_ctx *pipe_ctx = &dc.current_state.res_ctx.pipe_ctx[i];
// trigger event on first pipe with current stream
    if (stream == pipe_ctx.stream &&
    pipe_ctx.stream_res.tg.funcs.program_manual_trigger) {
    pipe_ctx.stream_res.tg.funcs.program_manual_trigger(
    pipe_ctx.stream_res.tg);
    break;
    }
    }
    }
    if (dc.hwss.dmub_hw_control_lock) {
    if (should_release_dmub_hw_control_lock)
    dc.hwss.dmub_hw_control_lock(dc, dc.current_state, false);
    }
    return true;
    }
    return false;
    }
    bool dc_stream_add_writeback(struct dc *dc,
    struct dc_stream_state *stream,
    struct dc_writeback_info *wb_info)
    {
    let mut isDrc: bool = false;
    let mut i: c_uint = 0;
    struct dwbc *dwb;
    if (stream == core::ptr::null_mut()) {
    dm_error("DC: dc_stream is core::ptr::null_mut()!\n");
    return false;
    }
    if (wb_info == core::ptr::null_mut()) {
    dm_error("DC: dc_writeback_info is core::ptr::null_mut()!\n");
    return false;
    }
    if (wb_info.dwb_pipe_inst >= MAX_DWB_PIPES) {
    dm_error("DC: writeback pipe is invalid!\n");
    return false;
    }
    dc_exit_ips_for_hw_access(dc);
    wb_info.dwb_params.out_transfer_func = &stream.out_transfer_func;
    dwb = dc.res_pool.dwbc[wb_info.dwb_pipe_inst];
    dwb.dwb_is_drc = false;
// recalculate and apply DML parameters
    for (i = 0; i < stream.num_wb_info; i++) {
// dynamic update
    if (stream.writeback_info[i].wb_enabled &&
    stream.writeback_info[i].dwb_pipe_inst == wb_info.dwb_pipe_inst) {
    stream.writeback_info[i] = *wb_info;
    isDrc = true;
    }
    }
    if (!isDrc) {
    ASSERT(stream.num_wb_info + 1 <= MAX_DWB_PIPES);
    stream.writeback_info[stream.num_wb_info++] = *wb_info;
    }
    if (dc.hwss.enable_writeback) {
    struct dc_stream_status *stream_status = dc_stream_get_status(stream);
    dwb = dc.res_pool.dwbc[wb_info.dwb_pipe_inst];
    if (stream_status)
    dwb.otg_inst = stream_status.primary_otg_inst;
    }
    if (!dc.hwss.update_bandwidth(dc, dc.current_state)) {
    dm_error("DC: update_bandwidth failed!\n");
    return false;
    }
// enable writeback
    if (dc.hwss.enable_writeback) {
    dwb = dc.res_pool.dwbc[wb_info.dwb_pipe_inst];
    if (dwb.funcs.is_enabled(dwb)) {
// writeback pipe already enabled, only need to update
    dc.hwss.update_writeback(dc, wb_info, dc.current_state);
    } else {
// Enable writeback pipe from scratch
    dc.hwss.enable_writeback(dc, wb_info, dc.current_state);
    }
    }
    return true;
    }
    bool dc_stream_fc_disable_writeback(struct dc *dc,
    struct dc_stream_state *stream,
    uint32_t dwb_pipe_inst)
    {
    struct dwbc *dwb = dc.res_pool.dwbc[dwb_pipe_inst];
    if (stream == core::ptr::null_mut()) {
    dm_error("DC: dc_stream is core::ptr::null_mut()!\n");
    return false;
    }
    if (dwb_pipe_inst >= MAX_DWB_PIPES) {
    dm_error("DC: writeback pipe is invalid!\n");
    return false;
    }
    if (stream.num_wb_info > MAX_DWB_PIPES) {
    dm_error("DC: num_wb_info is invalid!\n");
    return false;
    }
    dc_exit_ips_for_hw_access(dc);
    if (dwb.funcs.set_fc_enable)
    dwb.funcs.set_fc_enable(dwb, DWB_FRAME_CAPTURE_DISABLE);
    return true;
    }
//
// dc_stream_remove_writeback() - Disables writeback and removes writeback info.
// @dc: Display core control structure.
// @stream: Display core stream state.
// @dwb_pipe_inst: Display writeback pipe.
//
// Return: returns true on success, false otherwise.
//
    bool dc_stream_remove_writeback(struct dc *dc,
    struct dc_stream_state *stream,
    uint32_t dwb_pipe_inst)
    {
    unsigned int i, j;
    if (stream == core::ptr::null_mut()) {
    dm_error("DC: dc_stream is core::ptr::null_mut()!\n");
    return false;
    }
    if (dwb_pipe_inst >= MAX_DWB_PIPES) {
    dm_error("DC: writeback pipe is invalid!\n");
    return false;
    }
    if (stream.num_wb_info > MAX_DWB_PIPES) {
    dm_error("DC: num_wb_info is invalid!\n");
    return false;
    }
// remove writeback info for the requested writeback pipe from stream
    for (i = 0, j = 0; i < stream.num_wb_info; i++) {
// drop every entry that targets the pipe being removed
    if (stream.writeback_info[i].dwb_pipe_inst == dwb_pipe_inst)
    continue;
// keep this entry, compacting it down when earlier entries were removed
    if (j != i)
    memcpy(&stream.writeback_info[j], &stream.writeback_info[i],
    sizeof(struct dc_writeback_info));
    j++;
    }
    stream.num_wb_info = j;
// recalculate and apply DML parameters
    if (!dc.hwss.update_bandwidth(dc, dc.current_state)) {
    dm_error("DC: update_bandwidth failed!\n");
    return false;
    }
    dc_exit_ips_for_hw_access(dc);
// disable writeback
    if (dc.hwss.disable_writeback) {
    struct dwbc *dwb = dc.res_pool.dwbc[dwb_pipe_inst];
    if (dwb.funcs.is_enabled(dwb))
    dc.hwss.disable_writeback(dc, dwb_pipe_inst);
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_get_vblank_counter(stream: *const dc_stream_state) -> u32 {
    uint32_t dc_stream_get_vblank_counter(const struct dc_stream_state *stream)
    {
    uint8_t i;
    struct dc  *dc = stream.ctx.dc;
    struct resource_context *res_ctx =
    &dc.current_state.res_ctx;
    dc_exit_ips_for_hw_access(dc);
    for (i = 0; i < MAX_PIPES; i++) {
    struct timing_generator *tg = res_ctx.pipe_ctx[i].stream_res.tg;
    if (res_ctx.pipe_ctx[i].stream != stream || !tg)
    continue;
    return tg.funcs.get_frame_count(tg);
    }
    return 0;
    }
    bool dc_stream_send_dp_sdp(const struct dc_stream_state *stream,
    const uint8_t *custom_sdp_message,
    unsigned int sdp_message_size)
    {
    int i;
    struct dc  *dc;
    struct resource_context *res_ctx;
    if (stream == core::ptr::null_mut()) {
    dm_error("DC: dc_stream is core::ptr::null_mut()!\n");
    return false;
    }
    dc = stream.ctx.dc;
    res_ctx = &dc.current_state.res_ctx;
    dc_exit_ips_for_hw_access(dc);
    for (i = 0; i < MAX_PIPES; i++) {
    struct pipe_ctx *pipe_ctx = &res_ctx.pipe_ctx[i];
    if (pipe_ctx.stream != stream)
    continue;
    if (dc.hwss.send_immediate_sdp_message != core::ptr::null_mut())
    dc.hwss.send_immediate_sdp_message(pipe_ctx,
    custom_sdp_message,
    sdp_message_size);
    else
    DC_LOG_WARNING("%s:send_immediate_sdp_message not implemented on this ASIC\n",
    __func__);
    }
    return true;
    }
    bool dc_stream_get_scanoutpos(const struct dc_stream_state *stream,
    uint32_t *v_blank_start,
    uint32_t *v_blank_end,
    uint32_t *h_position,
    uint32_t *v_position)
    {
    uint8_t i;
    let mut ret: bool = false;
    struct dc  *dc;
    struct resource_context *res_ctx;
    if (!stream.ctx)
    return false;
    dc = stream.ctx.dc;
    res_ctx = &dc.current_state.res_ctx;
    dc_exit_ips_for_hw_access(dc);
    for (i = 0; i < MAX_PIPES; i++) {
    struct timing_generator *tg = res_ctx.pipe_ctx[i].stream_res.tg;
    if (res_ctx.pipe_ctx[i].stream != stream || !tg)
    continue;
    tg.funcs.get_scanoutpos(tg,
    v_blank_start,
    v_blank_end,
    h_position,
    v_position);
    ret = true;
    break;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_dmdata_status_done(dc: *mut dc, stream: *mut dc_stream_state) -> bool {
    bool dc_stream_dmdata_status_done(struct dc *dc, struct dc_stream_state *stream)
    {
    struct pipe_ctx *pipe = core::ptr::null_mut();
    int i;
    if (!dc.hwss.dmdata_status_done)
    return false;
    for (i = 0; i < MAX_PIPES; i++) {
    pipe = &dc.current_state.res_ctx.pipe_ctx[i];
    if (pipe.stream == stream)
    break;
    }
// Stream not found, by default we'll assume HUBP fetched dm data
    if (i == MAX_PIPES)
    return true;
    dc_exit_ips_for_hw_access(dc);
    return dc.hwss.dmdata_status_done(pipe);
    }
    bool dc_stream_set_dynamic_metadata(struct dc *dc,
    struct dc_stream_state *stream,
    struct dc_dmdata_attributes *attr)
    {
    struct pipe_ctx *pipe_ctx = core::ptr::null_mut();
    struct hubp *hubp;
    int i;
// Dynamic metadata is only supported on HDMI or DP
    if (!dc_is_hdmi_signal(stream.signal) && !dc_is_dp_signal(stream.signal))
    return false;
// Check hardware support
    if (!dc.hwss.program_dmdata_engine)
    return false;
    for (i = 0; i < MAX_PIPES; i++) {
    pipe_ctx = &dc.current_state.res_ctx.pipe_ctx[i];
    if (pipe_ctx.stream == stream)
    break;
    }
    if (i == MAX_PIPES)
    return false;
    hubp = pipe_ctx.plane_res.hubp;
    if (hubp == core::ptr::null_mut())
    return false;
    pipe_ctx.stream.dmdata_address = attr.address;
    dc_exit_ips_for_hw_access(dc);
    dc.hwss.program_dmdata_engine(pipe_ctx);
    if (hubp.funcs.dmdata_set_attributes != core::ptr::null_mut() &&
    pipe_ctx.stream.dmdata_address.quad_part != 0) {
    hubp.funcs.dmdata_set_attributes(hubp, attr);
    }
    return true;
    }
    enum dc_status dc_stream_add_dsc_to_resource(struct dc *dc,
    struct dc_state *state,
    struct dc_stream_state *stream)
    {
    if (dc.res_pool.funcs.add_dsc_to_stream_resource) {
    return dc.res_pool.funcs.add_dsc_to_stream_resource(dc, state, stream);
    } else {
    return DC_NO_DSC_RESOURCE;
    }
    }
    struct pipe_ctx *dc_stream_get_pipe_ctx(struct dc_stream_state *stream)
    {
    let mut i: c_int = 0;
    for (i = 0; i < MAX_PIPES; i++) {
    struct pipe_ctx *pipe = &stream.ctx.dc.current_state.res_ctx.pipe_ctx[i];
    if (pipe.stream == stream)
    return pipe;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_log(dc: *const dc, stream: *const dc_stream_state) {
    void dc_stream_log(const struct dc *dc, const struct dc_stream_state *stream)
    {
    DC_LOG_DC(
    "core_stream 0x%p: src: %d, %d, %d, %d; dst: %d, %d, %d, %d, colorSpace:%d\n",
    stream,
    stream.src.x,
    stream.src.y,
    stream.src.width,
    stream.src.height,
    stream.dst.x,
    stream.dst.y,
    stream.dst.width,
    stream.dst.height,
    stream.output_color_space);
    DC_LOG_DC(
    "\tpix_clk_khz: %d, h_total: %d, v_total: %d, pixel_encoding:%s, color_depth:%s\n",
    stream.timing.pix_clk_100hz / 10,
    stream.timing.h_total,
    stream.timing.v_total,
    dc_pixel_encoding_to_str(stream.timing.pixel_encoding),
    dc_color_depth_to_str(stream.timing.display_color_depth));
    DC_LOG_DC(
    "\tlink: %d\n",
    stream.link.link_index);
    DC_LOG_DC(
    "\tdsc: %d, mst_pbn: %d\n",
    stream.timing.flags.DSC,
    stream.timing.dsc_cfg.mst_pbn);
    if (stream.sink) {
    if (stream.sink.sink_signal != SIGNAL_TYPE_VIRTUAL &&
    stream.sink.sink_signal != SIGNAL_TYPE_NONE) {
    DC_LOG_DC(
    "\tsignal: %x dispname: %s manufacturer_id: 0x%x product_id: 0x%x\n",
    stream.signal,
    stream.sink.edid_caps.display_name,
    stream.sink.edid_caps.manufacturer_id,
    stream.sink.edid_caps.product_id);
    }
    }
    }
// TODO - move to per plane ownership?
//
// dc_stream_get_3dlut()
// Requirements:
// 1. Is stream already owns an RMCM instance, return it.
// 2. If it doesn't and we don't need to allocate, return NULL.
// 3. If there's a free RMCM instance, assign to stream and return it.
// 4. If no free RMCM instances, return NULL.
//
    struct dc_rmcm_3dlut *dc_stream_get_3dlut_for_stream(
    const struct dc *dc,
    const struct dc_stream_state *stream,
    bool allocate_one)
    {
    let mut num_rmcm: c_uint = dc.caps.color.mpc.num_rmcm_3dluts;
// see if one is allocated for this stream
    for (unsigned int i = 0; i < num_rmcm; i++) {
    if (dc.res_pool.rmcm_3dlut[i].isInUse &&
    dc.res_pool.rmcm_3dlut[i].stream == stream)
    return &dc.res_pool.rmcm_3dlut[i];
    }
// case: not found one, and dont need to allocate
    if (!allocate_one)
    return core::ptr::null_mut();
// see if there is an unused 3dlut, allocate
    for (unsigned int i = 0; i < num_rmcm; i++) {
    if (!dc.res_pool.rmcm_3dlut[i].isInUse) {
    dc.res_pool.rmcm_3dlut[i].isInUse = true;
    dc.res_pool.rmcm_3dlut[i].stream = stream;
    return &dc.res_pool.rmcm_3dlut[i];
    }
    }
// dont have a 3dlut
    return core::ptr::null_mut();
    }
    void dc_stream_release_3dlut_for_stream(
    const struct dc *dc,
    const struct dc_stream_state *stream)
    {
    struct dc_rmcm_3dlut *rmcm_3dlut =
    dc_stream_get_3dlut_for_stream(dc, stream, false);
    if (rmcm_3dlut) {
    rmcm_3dlut.isInUse = false;
    rmcm_3dlut.stream  = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_init_rmcm_3dlut(dc: *mut dc) {
    void dc_stream_init_rmcm_3dlut(struct dc *dc)
    {
    let mut num_rmcm: c_uint = dc.caps.color.mpc.num_rmcm_3dluts;
    for (unsigned int i = 0; i < num_rmcm; i++) {
    dc.res_pool.rmcm_3dlut[i].isInUse = false;
    dc.res_pool.rmcm_3dlut[i].stream = core::ptr::null_mut();
    }
    }
//
// Finds the greatest index in refresh_rate_hz that contains a value <= refresh
//
#[no_mangle]
unsafe extern "C" fn dc_stream_get_nearest_smallest_index(stream: *mut dc_stream_state, refresh: c_int) -> c_int {
    static int dc_stream_get_nearest_smallest_index(struct dc_stream_state *stream, int refresh)
    {
    for (int i = 0; i < (LUMINANCE_DATA_TABLE_SIZE - 1); ++i) {
    if ((stream.lumin_data.refresh_rate_hz[i] <= refresh) && (refresh < stream.lumin_data.refresh_rate_hz[i + 1])) {
    return i;
    }
    }
    return 9;
    }
//
// Finds a corresponding brightness for a given refresh rate between 2 given indices, where index1 < index2
//
    static int dc_stream_get_brightness_millinits_linear_interpolation (struct dc_stream_state *stream,
    int index1,
    int index2,
    int refresh_hz)
    {
    let mut slope: c_longlong = 0;
    let mut y_intercept: c_longlong = 0;
    let mut brightness_millinits: c_longlong = 0;
    if (stream.lumin_data.refresh_rate_hz[index2] != stream.lumin_data.refresh_rate_hz[index1]) {
    slope = (stream.lumin_data.luminance_millinits[index2] - stream.lumin_data.luminance_millinits[index1]) /
    (stream.lumin_data.refresh_rate_hz[index2] - stream.lumin_data.refresh_rate_hz[index1]);
    }
    y_intercept = stream.lumin_data.luminance_millinits[index2] - slope * stream.lumin_data.refresh_rate_hz[index2];
    brightness_millinits = y_intercept + (long long)refresh_hz * slope;
    return (int)brightness_millinits;
    }
//
// Finds a corresponding refresh rate for a given brightness between 2 given indices, where index1 < index2
//
    static int dc_stream_get_refresh_hz_linear_interpolation (struct dc_stream_state *stream,
    int index1,
    int index2,
    int brightness_millinits)
    {
    let mut slope: c_longlong = 1;
    let mut y_intercept: c_longlong = 0;
    let mut refresh_hz: c_longlong = 0;
    if (stream.lumin_data.refresh_rate_hz[index2] != stream.lumin_data.refresh_rate_hz[index1]) {
    slope = (stream.lumin_data.luminance_millinits[index2] - stream.lumin_data.luminance_millinits[index1]) /
    (stream.lumin_data.refresh_rate_hz[index2] - stream.lumin_data.refresh_rate_hz[index1]);
    }
    y_intercept = stream.lumin_data.luminance_millinits[index2] - slope * stream.lumin_data.refresh_rate_hz[index2];
    refresh_hz = div64_s64((brightness_millinits - y_intercept), slope);
    return (int)refresh_hz;
    }
//
// Finds the current brightness in millinits given a refresh rate
//
#[no_mangle]
unsafe extern "C" fn dc_stream_get_brightness_millinits_from_refresh(stream: *mut dc_stream_state, refresh_hz: c_int) -> c_int {
    static int dc_stream_get_brightness_millinits_from_refresh (struct dc_stream_state *stream, int refresh_hz)
    {
    let mut nearest_smallest_index: c_int = dc_stream_get_nearest_smallest_index(stream, refresh_hz);
    let mut nearest_smallest_value: c_int = stream.lumin_data.refresh_rate_hz[nearest_smallest_index];
    if (nearest_smallest_value == refresh_hz)
    return stream.lumin_data.luminance_millinits[nearest_smallest_index];
    if (nearest_smallest_index >= 9)
    return dc_stream_get_brightness_millinits_linear_interpolation(stream, nearest_smallest_index - 1, nearest_smallest_index, refresh_hz);
    if (nearest_smallest_value == stream.lumin_data.refresh_rate_hz[nearest_smallest_index + 1])
    return stream.lumin_data.luminance_millinits[nearest_smallest_index];
    return dc_stream_get_brightness_millinits_linear_interpolation(stream, nearest_smallest_index, nearest_smallest_index + 1, refresh_hz);
    }
//
// Finds the lowest/highest refresh rate (depending on search_for_max_increase)
// that can be achieved from starting_refresh_hz while staying
// within flicker criteria
//
    static int dc_stream_calculate_flickerless_refresh_rate(struct dc_stream_state *stream,
    int current_brightness,
    int starting_refresh_hz,
    bool is_gaming,
    bool search_for_max_increase)
    {
    let mut nearest_smallest_index: c_int = dc_stream_get_nearest_smallest_index(stream, starting_refresh_hz);
    int flicker_criteria_millinits = is_gaming ?
    stream.lumin_data.flicker_criteria_milli_nits_GAMING :
    stream.lumin_data.flicker_criteria_milli_nits_STATIC;
    let mut safe_upper_bound: c_int = current_brightness + flicker_criteria_millinits;
    let mut safe_lower_bound: c_int = current_brightness - flicker_criteria_millinits;
    let mut lumin_millinits_temp: c_int = 0;
    let mut offset: c_int = -1;
    if (search_for_max_increase) {
    offset = 1;
    }
//
// Increments up or down by 1 depending on search_for_max_increase
//
    for (int i = nearest_smallest_index; (i > 0 && !search_for_max_increase) || (i < (LUMINANCE_DATA_TABLE_SIZE - 1) && search_for_max_increase); i += offset) {
    lumin_millinits_temp = stream.lumin_data.luminance_millinits[i + offset];
    if ((lumin_millinits_temp >= safe_upper_bound) || (lumin_millinits_temp <= safe_lower_bound)) {
    if (stream.lumin_data.refresh_rate_hz[i + offset] == stream.lumin_data.refresh_rate_hz[i])
    return stream.lumin_data.refresh_rate_hz[i];
    int target_brightness = (stream.lumin_data.luminance_millinits[i + offset] >= (current_brightness + flicker_criteria_millinits)) ?
    current_brightness + flicker_criteria_millinits :
    current_brightness - flicker_criteria_millinits;
    let mut refresh: c_int = 0;
//
// Need the second input to be < third input for dc_stream_get_refresh_hz_linear_interpolation
//
    if (search_for_max_increase)
    refresh = dc_stream_get_refresh_hz_linear_interpolation(stream, i, i + offset, target_brightness);
    else
    refresh = dc_stream_get_refresh_hz_linear_interpolation(stream, i + offset, i, target_brightness);
    if (refresh == stream.lumin_data.refresh_rate_hz[i + offset])
    return stream.lumin_data.refresh_rate_hz[i + offset];
    return refresh;
    }
    }
    if (search_for_max_increase)
    return (int)div64_s64((long long)stream.timing.pix_clk_100hz*100, stream.timing.v_total*(long long)stream.timing.h_total);
    else
    return stream.lumin_data.refresh_rate_hz[0];
    }
//
// Gets the max delta luminance within a specified refresh range
//
#[no_mangle]
unsafe extern "C" fn dc_stream_get_max_delta_lumin_millinits(stream: *mut dc_stream_state, hz1: c_int, hz2: c_int, isGaming: bool) -> c_int {
    static int dc_stream_get_max_delta_lumin_millinits(struct dc_stream_state *stream, int hz1, int hz2, bool isGaming)
    {
    let mut lower_refresh_brightness: c_int = dc_stream_get_brightness_millinits_from_refresh (stream, hz1);
    let mut higher_refresh_brightness: c_int = dc_stream_get_brightness_millinits_from_refresh (stream, hz2);
    let mut min: c_int = lower_refresh_brightness;
    let mut max: c_int = higher_refresh_brightness;
//
// Static screen, therefore no need to scan through array
//
    if (!isGaming) {
    if (lower_refresh_brightness >= higher_refresh_brightness) {
    return lower_refresh_brightness - higher_refresh_brightness;
    }
    return higher_refresh_brightness - lower_refresh_brightness;
    }
    min = MIN(lower_refresh_brightness, higher_refresh_brightness);
    max = MAX(lower_refresh_brightness, higher_refresh_brightness);
    let mut nearest_smallest_index: c_int = dc_stream_get_nearest_smallest_index(stream, hz1);
    for (; nearest_smallest_index < (LUMINANCE_DATA_TABLE_SIZE - 1) &&
    stream.lumin_data.refresh_rate_hz[nearest_smallest_index + 1] <= hz2 ; nearest_smallest_index++) {
    min = MIN(min, stream.lumin_data.luminance_millinits[nearest_smallest_index + 1]);
    max = MAX(max, stream.lumin_data.luminance_millinits[nearest_smallest_index + 1]);
    }
    return (max - min);
    }
//
// Determines the max flickerless instant vtotal delta for a stream.
// Determines vtotal increase/decrease based on the bool "increase"
//
#[no_mangle]
unsafe extern "C" fn dc_stream_get_max_flickerless_instant_vtotal_delta(stream: *mut dc_stream_state, is_gaming: bool, increase: bool) -> c_uint {
    static unsigned int dc_stream_get_max_flickerless_instant_vtotal_delta(struct dc_stream_state *stream, bool is_gaming, bool increase)
    {
    if (stream.timing.v_total * stream.timing.h_total == 0)
    return 0;
    let mut current_refresh_hz: c_int = (int)div64_s64((long long)stream.timing.pix_clk_100hz*100, stream.timing.v_total*(long long)stream.timing.h_total);
    int safe_refresh_hz = dc_stream_calculate_flickerless_refresh_rate(stream,
    dc_stream_get_brightness_millinits_from_refresh(stream, current_refresh_hz),
    current_refresh_hz,
    is_gaming,
    increase);
    let mut safe_refresh_v_total: c_int = (int)div64_s64((long long)stream.timing.pix_clk_100hz*100, safe_refresh_hz*(long long)stream.timing.h_total);
    if (increase)
    return (((int) stream.timing.v_total - safe_refresh_v_total) >= 0) ? (stream.timing.v_total - safe_refresh_v_total) : 0;
    return ((safe_refresh_v_total - (int) stream.timing.v_total) >= 0) ? (safe_refresh_v_total - stream.timing.v_total) : 0;
    }
//
// Finds the highest refresh rate that can be achieved
// from starting_refresh_hz while staying within flicker criteria
//
#[no_mangle]
pub unsafe extern "C" fn dc_stream_calculate_max_flickerless_refresh_rate(stream: *mut dc_stream_state, starting_refresh_hz: c_int, is_gaming: bool) -> c_int {
    int dc_stream_calculate_max_flickerless_refresh_rate(struct dc_stream_state *stream, int starting_refresh_hz, bool is_gaming)
    {
    if (!stream.lumin_data.is_valid)
    return 0;
    let mut current_brightness: c_int = dc_stream_get_brightness_millinits_from_refresh(stream, starting_refresh_hz);
    return dc_stream_calculate_flickerless_refresh_rate(stream,
    current_brightness,
    starting_refresh_hz,
    is_gaming,
    true);
    }
//
// Finds the lowest refresh rate that can be achieved
// from starting_refresh_hz while staying within flicker criteria
//
#[no_mangle]
pub unsafe extern "C" fn dc_stream_calculate_min_flickerless_refresh_rate(stream: *mut dc_stream_state, starting_refresh_hz: c_int, is_gaming: bool) -> c_int {
    int dc_stream_calculate_min_flickerless_refresh_rate(struct dc_stream_state *stream, int starting_refresh_hz, bool is_gaming)
    {
    if (!stream.lumin_data.is_valid)
    return 0;
    let mut current_brightness: c_int = dc_stream_get_brightness_millinits_from_refresh(stream, starting_refresh_hz);
    return dc_stream_calculate_flickerless_refresh_rate(stream,
    current_brightness,
    starting_refresh_hz,
    is_gaming,
    false);
    }
//
// Determines if there will be a flicker when moving between 2 refresh rates
//
#[no_mangle]
pub unsafe extern "C" fn dc_stream_is_refresh_rate_range_flickerless(stream: *mut dc_stream_state, hz1: c_int, hz2: c_int, is_gaming: bool) -> bool {
    bool dc_stream_is_refresh_rate_range_flickerless(struct dc_stream_state *stream, int hz1, int hz2, bool is_gaming)
    {
//
// Assume that we wont flicker if there is invalid data
//
    if (!stream.lumin_data.is_valid)
    return false;
    let mut dl: c_int = dc_stream_get_max_delta_lumin_millinits(stream, hz1, hz2, is_gaming);
    int flicker_criteria_millinits = (is_gaming) ?
    stream.lumin_data.flicker_criteria_milli_nits_GAMING :
    stream.lumin_data.flicker_criteria_milli_nits_STATIC;
    return (dl <= flicker_criteria_millinits);
    }
//
// Determines the max instant vtotal delta increase that can be applied without
// flickering for a given stream
//
    unsigned int dc_stream_get_max_flickerless_instant_vtotal_decrease(struct dc_stream_state *stream,
    bool is_gaming)
    {
    if (!stream.lumin_data.is_valid)
    return 0;
    return dc_stream_get_max_flickerless_instant_vtotal_delta(stream, is_gaming, true);
    }
//
// Determines the max instant vtotal delta decrease that can be applied without
// flickering for a given stream
//
    unsigned int dc_stream_get_max_flickerless_instant_vtotal_increase(struct dc_stream_state *stream,
    bool is_gaming)
    {
    if (!stream.lumin_data.is_valid)
    return 0;
    return dc_stream_get_max_flickerless_instant_vtotal_delta(stream, is_gaming, false);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_is_cursor_limit_pending(dc: *mut dc, stream: *mut dc_stream_state) -> bool {
    bool dc_stream_is_cursor_limit_pending(struct dc *dc, struct dc_stream_state *stream)
    {
    let mut is_limit_pending: bool = false;
    if (dc.current_state)
    is_limit_pending = dc_state_get_stream_cursor_subvp_limit(stream, dc.current_state);
    return is_limit_pending;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_stream_can_clear_cursor_limit(dc: *mut dc, stream: *mut dc_stream_state) -> bool {
    bool dc_stream_can_clear_cursor_limit(struct dc *dc, struct dc_stream_state *stream)
    {
    let mut can_clear_limit: bool = false;
    if (dc.current_state)
    can_clear_limit = dc_state_get_stream_cursor_subvp_limit(stream, dc.current_state) &&
    (stream.hw_cursor_req ||
    !stream.cursor_position.enable ||
    dc_stream_check_cursor_attributes(stream, dc.current_state, &stream.cursor_attributes));
    return can_clear_limit;
    }
