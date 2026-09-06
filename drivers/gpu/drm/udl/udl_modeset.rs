//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/udl/udl_modeset.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 Red Hat
//
// based in parts on udlfb.c:
// Copyright (C) 2009 Roberto De Ioris <roberto@unbit.it>
// Copyright (C) 2009 Jaya Kumar <jayakumar.lkml@gmail.com>
// Copyright (C) 2009 Bernie Thompson <bernie@plugable.com>
//

//
// All DisplayLink bulk operations start with 0xaf (UDL_MSG_BULK), followed by
// a specific command code. All operations are written to a command buffer, which
// the driver sends to the device.
//
    static char *udl_set_register(char *buf, u8 reg, u8 val)
    {
// buf++ = UDL_MSG_BULK;
// buf++ = UDL_CMD_WRITEREG;
// buf++ = reg;
// buf++ = val;
    return buf;
    }
    static char *udl_vidreg_lock(char *buf)
    {
    return udl_set_register(buf, UDL_REG_VIDREG, UDL_VIDREG_LOCK);
    }
    static char *udl_vidreg_unlock(char *buf)
    {
    return udl_set_register(buf, UDL_REG_VIDREG, UDL_VIDREG_UNLOCK);
    }
    static char *udl_set_blank_mode(char *buf, u8 mode)
    {
    return udl_set_register(buf, UDL_REG_BLANKMODE, mode);
    }
    static char *udl_set_color_depth(char *buf, u8 selection)
    {
    return udl_set_register(buf, UDL_REG_COLORDEPTH, selection);
    }
    static char *udl_set_base16bpp(char *buf, u32 base)
    {
// the base pointer is 24 bits wide, 0x20 is hi byte.
    let mut reg20: u8 = FIELD_GET(UDL_BASE_ADDR2_MASK, base);
    let mut reg21: u8 = FIELD_GET(UDL_BASE_ADDR1_MASK, base);
    let mut reg22: u8 = FIELD_GET(UDL_BASE_ADDR0_MASK, base);
    buf = udl_set_register(buf, UDL_REG_BASE16BPP_ADDR2, reg20);
    buf = udl_set_register(buf, UDL_REG_BASE16BPP_ADDR1, reg21);
    buf = udl_set_register(buf, UDL_REG_BASE16BPP_ADDR0, reg22);
    return buf;
    }
//
// DisplayLink HW has separate 16bpp and 8bpp framebuffers.
// In 24bpp modes, the low 323 RGB bits go in the 8bpp framebuffer
//
    static char *udl_set_base8bpp(char *buf, u32 base)
    {
// the base pointer is 24 bits wide, 0x26 is hi byte.
    let mut reg26: u8 = FIELD_GET(UDL_BASE_ADDR2_MASK, base);
    let mut reg27: u8 = FIELD_GET(UDL_BASE_ADDR1_MASK, base);
    let mut reg28: u8 = FIELD_GET(UDL_BASE_ADDR0_MASK, base);
    buf = udl_set_register(buf, UDL_REG_BASE8BPP_ADDR2, reg26);
    buf = udl_set_register(buf, UDL_REG_BASE8BPP_ADDR1, reg27);
    buf = udl_set_register(buf, UDL_REG_BASE8BPP_ADDR0, reg28);
    return buf;
    }
    static char *udl_set_register_16(char *wrptr, u8 reg, u16 value)
    {
    wrptr = udl_set_register(wrptr, reg, value >> 8);
    return udl_set_register(wrptr, reg+1, value);
    }
//
// This is kind of weird because the controller takes some
// register values in a different byte order than other registers.
//
    static char *udl_set_register_16be(char *wrptr, u8 reg, u16 value)
    {
    wrptr = udl_set_register(wrptr, reg, value);
    return udl_set_register(wrptr, reg+1, value >> 8);
    }
//
// LFSR is linear feedback shift register. The reason we have this is
// because the display controller needs to minimize the clock depth of
// various counters used in the display path. So this code reverses the
// provided value into the lfsr16 value by counting backwards to get
// the value that needs to be set in the hardware comparator to get the
// same actual count. This makes sense once you read above a couple of
// times and think about it from a hardware perspective.
//
#[no_mangle]
unsafe extern "C" fn udl_lfsr16(actual_count: u16) -> u16 {
    static u16 udl_lfsr16(u16 actual_count)
    {
    u32 lv = 0xFFFF; /* This is the lfsr value that the hw starts with */
    while (actual_count--) {
    lv =	 ((lv << 1) |
    (((lv >> 15) ^ (lv >> 4) ^ (lv >> 2) ^ (lv >> 1)) & 1))
    & 0xFFFF;
    }
    return (u16) lv;
    }
//
// This does LFSR conversion on the value that is to be written.
// See LFSR explanation above for more detail.
//
    static char *udl_set_register_lfsr16(char *wrptr, u8 reg, u16 value)
    {
    return udl_set_register_16(wrptr, reg, udl_lfsr16(value));
    }
//
// Takes a DRM display mode and converts it into the DisplayLink
// equivalent register commands.
//
    static char *udl_set_display_mode(char *buf, struct drm_display_mode *mode)
    {
    let mut reg01: u16 = mode.crtc_htotal - mode.crtc_hsync_start;
    let mut reg03: u16 = reg01 + mode.crtc_hdisplay;
    let mut reg05: u16 = mode.crtc_vtotal - mode.crtc_vsync_start;
    let mut reg07: u16 = reg05 + mode.crtc_vdisplay;
    let mut reg09: u16 = mode.crtc_htotal - 1;
    u16 reg0b = 1; /* libdlo hardcodes hsync start to 1 */
    let mut reg0d: u16 = mode.crtc_hsync_end - mode.crtc_hsync_start + 1;
    let mut reg0f: u16 = mode.hdisplay;
    let mut reg11: u16 = mode.crtc_vtotal;
    u16 reg13 = 0; /* libdlo hardcodes vsync start to 0 */
    let mut reg15: u16 = mode.crtc_vsync_end - mode.crtc_vsync_start;
    let mut reg17: u16 = mode.crtc_vdisplay;
    let mut reg1b: u16 = mode.clock / 5;
    buf = udl_set_register_lfsr16(buf, UDL_REG_XDISPLAYSTART, reg01);
    buf = udl_set_register_lfsr16(buf, UDL_REG_XDISPLAYEND, reg03);
    buf = udl_set_register_lfsr16(buf, UDL_REG_YDISPLAYSTART, reg05);
    buf = udl_set_register_lfsr16(buf, UDL_REG_YDISPLAYEND, reg07);
    buf = udl_set_register_lfsr16(buf, UDL_REG_XENDCOUNT, reg09);
    buf = udl_set_register_lfsr16(buf, UDL_REG_HSYNCSTART, reg0b);
    buf = udl_set_register_lfsr16(buf, UDL_REG_HSYNCEND, reg0d);
    buf = udl_set_register_16(buf, UDL_REG_HPIXELS, reg0f);
    buf = udl_set_register_lfsr16(buf, UDL_REG_YENDCOUNT, reg11);
    buf = udl_set_register_lfsr16(buf, UDL_REG_VSYNCSTART, reg13);
    buf = udl_set_register_lfsr16(buf, UDL_REG_VSYNCEND, reg15);
    buf = udl_set_register_16(buf, UDL_REG_VPIXELS, reg17);
    buf = udl_set_register_16be(buf, UDL_REG_PIXELCLOCK5KHZ, reg1b);
    return buf;
    }
    static char *udl_dummy_render(char *wrptr)
    {
// wrptr++ = UDL_MSG_BULK;
// wrptr++ = UDL_CMD_WRITECOPY16;
// wrptr++ = 0x00; /* from addr
// wrptr++ = 0x00;
// wrptr++ = 0x01; /* one pixel
// wrptr++ = 0x00; /* to address
// wrptr++ = 0x00;
    return wrptr;
    }
#[no_mangle]
unsafe extern "C" fn udl_log_cpp(cpp: c_uint) -> c_long {
    static long udl_log_cpp(unsigned int cpp)
    {
    if (WARN_ON(!is_power_of_2(cpp)))
    return -EINVAL;
    return __ffs(cpp);
    }
    static int udl_handle_damage(struct drm_framebuffer *fb,
    const struct iosys_map *map,
    const struct drm_rect *clip)
    {
    struct drm_device *dev = fb.dev;
    struct udl_device *udl = to_udl(dev);
    void *vaddr = map.vaddr; /* TODO: Use mapping abstraction properly */
    int i, ret;
    char *cmd;
    struct urb *urb;
    int log_bpp;
    ret = udl_log_cpp(fb.format.cpp[0]);
    if (ret < 0)
    return ret;
    log_bpp = ret;
    urb = udl_get_urb(udl);
    if (!urb)
    return -ENOMEM;
    cmd = urb.transfer_buffer;
    for (i = clip.y1; i < clip.y2; i++) {
    let mut line_offset: c_int = fb.pitches[0] * i;
    let mut byte_offset: c_int = line_offset + (clip.x1 << log_bpp);
    let mut dev_byte_offset: c_int = (fb.width * i + clip.x1) << log_bpp;
    let mut byte_width: c_int = drm_rect_width(clip) << log_bpp;
    ret = udl_render_hline(udl, log_bpp, &urb, (char *)vaddr,
    &cmd, byte_offset, dev_byte_offset,
    byte_width);
    if (ret)
    return ret;
    }
    if (cmd > (char *)urb.transfer_buffer) {
// Send partial buffer remaining before exiting
    int len;
    if (cmd < (char *)urb.transfer_buffer + urb.transfer_buffer_length)
// cmd++ = UDL_MSG_BULK;
    len = cmd - (char *)urb.transfer_buffer;
    ret = udl_submit_urb(udl, urb, len);
    } else {
    udl_urb_completion(urb);
    }
    return 0;
    }
//
// Primary plane
//
    static const uint32_t udl_primary_plane_formats[] = {
    DRM_FORMAT_RGB565,
    DRM_FORMAT_XRGB8888,
    };
    static const uint64_t udl_primary_plane_fmtmods[] = {
    DRM_FORMAT_MOD_LINEAR,
    DRM_FORMAT_MOD_INVALID
    };
    static int udl_primary_plane_helper_atomic_check(struct drm_plane *plane,
    struct drm_atomic_commit *state)
    {
    struct drm_plane_state *new_plane_state = drm_atomic_get_new_plane_state(state, plane);
    struct drm_crtc *new_crtc = new_plane_state.crtc;
    struct drm_crtc_state *new_crtc_state = core::ptr::null_mut();
    if (new_crtc)
    new_crtc_state = drm_atomic_get_new_crtc_state(state, new_crtc);
    return drm_atomic_helper_check_plane_state(new_plane_state, new_crtc_state,
    DRM_PLANE_NO_SCALING,
    DRM_PLANE_NO_SCALING,
    false, false);
    }
    static void udl_primary_plane_helper_atomic_update(struct drm_plane *plane,
    struct drm_atomic_commit *state)
    {
    struct drm_device *dev = plane.dev;
    struct drm_plane_state *plane_state = drm_atomic_get_new_plane_state(state, plane);
    struct drm_shadow_plane_state *shadow_plane_state = to_drm_shadow_plane_state(plane_state);
    struct drm_framebuffer *fb = plane_state.fb;
    struct drm_plane_state *old_plane_state = drm_atomic_get_old_plane_state(state, plane);
    struct drm_atomic_helper_damage_iter iter;
    struct drm_rect damage;
    int ret, idx;
    if (!fb)
    return; /* no framebuffer; plane is disabled */
    ret = drm_gem_fb_begin_cpu_access(fb, DMA_FROM_DEVICE);
    if (ret)
    return;
    if (!drm_dev_enter(dev, &idx))
    goto out_drm_gem_fb_end_cpu_access;
    drm_atomic_helper_damage_iter_init(&iter, old_plane_state, plane_state);
    drm_atomic_for_each_plane_damage(&iter, &damage) {
    udl_handle_damage(fb, &shadow_plane_state.data[0], &damage);
    }
    drm_dev_exit(idx);
    out_drm_gem_fb_end_cpu_access:
    drm_gem_fb_end_cpu_access(fb, DMA_FROM_DEVICE);
    }
    static const struct drm_plane_helper_funcs udl_primary_plane_helper_funcs = {
    DRM_GEM_SHADOW_PLANE_HELPER_FUNCS,
    .atomic_check = udl_primary_plane_helper_atomic_check,
    .atomic_update = udl_primary_plane_helper_atomic_update,
    };
    static const struct drm_plane_funcs udl_primary_plane_funcs = {
    .update_plane = drm_atomic_helper_update_plane,
    .disable_plane = drm_atomic_helper_disable_plane,
    .destroy = drm_plane_cleanup,
    DRM_GEM_SHADOW_PLANE_FUNCS,
    };
//
// CRTC
//
#[no_mangle]
unsafe extern "C" fn udl_crtc_helper_atomic_enable(crtc: *mut drm_crtc, state: *mut drm_atomic_commit) {
    static void udl_crtc_helper_atomic_enable(struct drm_crtc *crtc, struct drm_atomic_commit *state)
    {
    struct drm_device *dev = crtc.dev;
    struct udl_device *udl = to_udl(dev);
    struct drm_crtc_state *crtc_state = drm_atomic_get_new_crtc_state(state, crtc);
    struct drm_display_mode *mode = &crtc_state.mode;
    struct urb *urb;
    char *buf;
    int idx;
    if (!drm_dev_enter(dev, &idx))
    return;
    urb = udl_get_urb(udl);
    if (!urb) {
    drm_err_ratelimited(dev, "get urb failed when enabling crtc\n");
    goto out;
    }
    buf = (char *)urb.transfer_buffer;
    buf = udl_vidreg_lock(buf);
    buf = udl_set_color_depth(buf, UDL_COLORDEPTH_16BPP);
// set base for 16bpp segment to 0
    buf = udl_set_base16bpp(buf, 0);
// set base for 8bpp segment to end of fb
    buf = udl_set_base8bpp(buf, 2 * mode.vdisplay * mode.hdisplay);
    buf = udl_set_display_mode(buf, mode);
    buf = udl_set_blank_mode(buf, UDL_BLANKMODE_ON);
    buf = udl_vidreg_unlock(buf);
    buf = udl_dummy_render(buf);
    udl_submit_urb(udl, urb, buf - (char *)urb.transfer_buffer);
    out:
    drm_dev_exit(idx);
    }
#[no_mangle]
unsafe extern "C" fn udl_crtc_helper_atomic_disable(crtc: *mut drm_crtc, state: *mut drm_atomic_commit) {
    static void udl_crtc_helper_atomic_disable(struct drm_crtc *crtc, struct drm_atomic_commit *state)
    {
    struct drm_device *dev = crtc.dev;
    struct udl_device *udl = to_udl(dev);
    struct urb *urb;
    char *buf;
    int idx;
    if (!drm_dev_enter(dev, &idx))
    return;
    urb = udl_get_urb(udl);
    if (!urb)
    goto out;
    buf = (char *)urb.transfer_buffer;
    buf = udl_vidreg_lock(buf);
    buf = udl_set_blank_mode(buf, UDL_BLANKMODE_POWERDOWN);
    buf = udl_vidreg_unlock(buf);
    buf = udl_dummy_render(buf);
    udl_submit_urb(udl, urb, buf - (char *)urb.transfer_buffer);
    out:
    drm_dev_exit(idx);
    }
    static const struct drm_crtc_helper_funcs udl_crtc_helper_funcs = {
    .atomic_check = drm_crtc_helper_atomic_check,
    .atomic_enable = udl_crtc_helper_atomic_enable,
    .atomic_disable = udl_crtc_helper_atomic_disable,
    };
    static const struct drm_crtc_funcs udl_crtc_funcs = {
    .reset = drm_atomic_helper_crtc_reset,
    .destroy = drm_crtc_cleanup,
    .set_config = drm_atomic_helper_set_config,
    .page_flip = drm_atomic_helper_page_flip,
    .atomic_duplicate_state = drm_atomic_helper_crtc_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_crtc_destroy_state,
    };
//
// Encoder
//
    static const struct drm_encoder_funcs udl_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
//
// Connector
//
#[no_mangle]
unsafe extern "C" fn udl_connector_helper_get_modes(connector: *mut drm_connector) -> c_int {
    static int udl_connector_helper_get_modes(struct drm_connector *connector)
    {
    const struct drm_edid *drm_edid;
    int count;
    drm_edid = udl_edid_read(connector);
    drm_edid_connector_update(connector, drm_edid);
    count = drm_edid_connector_add_modes(connector);
    drm_edid_free(drm_edid);
    return count;
    }
    static int udl_connector_helper_detect_ctx(struct drm_connector *connector,
    struct drm_modeset_acquire_ctx *ctx,
    bool force)
    {
    struct udl_device *udl = to_udl(connector.dev);
    if (udl_probe_edid(udl))
    return connector_status_connected;
    return connector_status_disconnected;
    }
    static const struct drm_connector_helper_funcs udl_connector_helper_funcs = {
    .get_modes = udl_connector_helper_get_modes,
    .detect_ctx = udl_connector_helper_detect_ctx,
    };
    static const struct drm_connector_funcs udl_connector_funcs = {
    .reset = drm_atomic_helper_connector_reset,
    .fill_modes = drm_helper_probe_single_connector_modes,
    .destroy = drm_connector_cleanup,
    .atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_connector_destroy_state,
    };
//
// Modesetting
//
    static enum drm_mode_status udl_mode_config_mode_valid(struct drm_device *dev,
    const struct drm_display_mode *mode)
    {
    struct udl_device *udl = to_udl(dev);
    if (udl.sku_pixel_limit) {
    if (mode.vdisplay * mode.hdisplay > udl.sku_pixel_limit)
    return MODE_MEM;
    }
    return MODE_OK;
    }
    static const struct drm_mode_config_funcs udl_mode_config_funcs = {
    .fb_create = drm_gem_fb_create_with_dirty,
    .mode_valid = udl_mode_config_mode_valid,
    .atomic_check  = drm_atomic_helper_check,
    .atomic_commit = drm_atomic_helper_commit,
    };
#[no_mangle]
pub unsafe extern "C" fn udl_modeset_init(udl: *mut udl_device) -> c_int {
    int udl_modeset_init(struct udl_device *udl)
    {
    struct drm_device *dev = &udl.drm;
    struct drm_plane *primary_plane;
    struct drm_crtc *crtc;
    struct drm_encoder *encoder;
    struct drm_connector *connector;
    int ret;
    ret = drmm_mode_config_init(dev);
    if (ret)
    return ret;
    dev.mode_config.min_width = 640;
    dev.mode_config.min_height = 480;
    dev.mode_config.max_width = 2048;
    dev.mode_config.max_height = 2048;
    dev.mode_config.preferred_depth = 16;
    dev.mode_config.funcs = &udl_mode_config_funcs;
    primary_plane = &udl.primary_plane;
    ret = drm_universal_plane_init(dev, primary_plane, 0,
    &udl_primary_plane_funcs,
    udl_primary_plane_formats,
    ARRAY_SIZE(udl_primary_plane_formats),
    udl_primary_plane_fmtmods,
    DRM_PLANE_TYPE_PRIMARY, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_plane_helper_add(primary_plane, &udl_primary_plane_helper_funcs);
    drm_plane_enable_fb_damage_clips(primary_plane);
    crtc = &udl.crtc;
    ret = drm_crtc_init_with_planes(dev, crtc, primary_plane, core::ptr::null_mut(),
    &udl_crtc_funcs, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_crtc_helper_add(crtc, &udl_crtc_helper_funcs);
    encoder = &udl.encoder;
    ret = drm_encoder_init(dev, encoder, &udl_encoder_funcs, DRM_MODE_ENCODER_DAC, core::ptr::null_mut());
    if (ret)
    return ret;
    encoder.possible_crtcs = drm_crtc_mask(crtc);
    connector = &udl.connector;
    ret = drm_connector_init(dev, connector, &udl_connector_funcs, DRM_MODE_CONNECTOR_VGA);
    if (ret)
    return ret;
    drm_connector_helper_add(connector, &udl_connector_helper_funcs);
    connector.polled = DRM_CONNECTOR_POLL_CONNECT |
    DRM_CONNECTOR_POLL_DISCONNECT;
    ret = drm_connector_attach_encoder(connector, encoder);
    if (ret)
    return ret;
    drm_mode_config_reset(dev);
    drmm_kms_helper_poll_init(dev);
    return 0;
    }
