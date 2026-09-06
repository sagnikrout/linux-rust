//! Automatically rewritten from C to Rust
//! Source: drivers/media/test-drivers/vimc/vimc-debayer.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// vimc-debayer.c Virtual Media Controller Driver
//
// Copyright (C) 2015-2017 Helen Koike <helen.fornazier@gmail.com>
//

// TODO: Add support for more output formats, we only support RGB888 for now.

    enum vimc_debayer_rgb_colors {
    VIMC_DEBAYER_RED = 0,
    VIMC_DEBAYER_GREEN = 1,
    VIMC_DEBAYER_BLUE = 2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_debayer_pix_map {
    pub code: u32,
    pub order: [enum vimc_debayer_rgb_colors; 2][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_debayer_device {
    pub ved: vimc_ent_device,
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub pads: [media_pad; 2],
    pub src_frame: *mut u8,
    void (*set_rgb_src)(struct vimc_debayer_device *vdebayer,
    unsigned int lin, unsigned int col,
    pub rgb[3]): c_uint,
//
// Virtual "hardware" configuration, filled when the stream starts or
// when controls are set.
//
    struct {
    pub sink_pix_map: *const vimc_debayer_pix_map,
    pub sink_bpp: c_uint,
    pub size: v4l2_area,
    pub mean_win_size: c_uint,
    pub src_code: u32,
    pub hw: },
}

    static const struct v4l2_mbus_framefmt sink_fmt_default = {
    .width = 640,
    .height = 480,
    .code = MEDIA_BUS_FMT_SRGGB8_1X8,
    .field = V4L2_FIELD_NONE,
    .colorspace = V4L2_COLORSPACE_SRGB,
    };
    static const u32 vimc_debayer_src_mbus_codes[] = {
    MEDIA_BUS_FMT_GBR888_1X24,
    MEDIA_BUS_FMT_BGR888_1X24,
    MEDIA_BUS_FMT_BGR888_3X8,
    MEDIA_BUS_FMT_RGB888_1X24,
    MEDIA_BUS_FMT_RGB888_2X12_BE,
    MEDIA_BUS_FMT_RGB888_2X12_LE,
    MEDIA_BUS_FMT_RGB888_3X8,
    MEDIA_BUS_FMT_RGB888_1X7X4_SPWG,
    MEDIA_BUS_FMT_RGB888_1X7X4_JEIDA,
    MEDIA_BUS_FMT_RGB888_1X32_PADHI,
    };
    static const struct vimc_debayer_pix_map vimc_debayer_pix_map_list[] = {
    {
    .code = MEDIA_BUS_FMT_SBGGR8_1X8,
    .order = { { VIMC_DEBAYER_BLUE, VIMC_DEBAYER_GREEN },
    { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_RED } }
    },
    {
    .code = MEDIA_BUS_FMT_SGBRG8_1X8,
    .order = { { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_BLUE },
    { VIMC_DEBAYER_RED, VIMC_DEBAYER_GREEN } }
    },
    {
    .code = MEDIA_BUS_FMT_SGRBG8_1X8,
    .order = { { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_RED },
    { VIMC_DEBAYER_BLUE, VIMC_DEBAYER_GREEN } }
    },
    {
    .code = MEDIA_BUS_FMT_SRGGB8_1X8,
    .order = { { VIMC_DEBAYER_RED, VIMC_DEBAYER_GREEN },
    { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_BLUE } }
    },
    {
    .code = MEDIA_BUS_FMT_SBGGR10_1X10,
    .order = { { VIMC_DEBAYER_BLUE, VIMC_DEBAYER_GREEN },
    { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_RED } }
    },
    {
    .code = MEDIA_BUS_FMT_SGBRG10_1X10,
    .order = { { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_BLUE },
    { VIMC_DEBAYER_RED, VIMC_DEBAYER_GREEN } }
    },
    {
    .code = MEDIA_BUS_FMT_SGRBG10_1X10,
    .order = { { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_RED },
    { VIMC_DEBAYER_BLUE, VIMC_DEBAYER_GREEN } }
    },
    {
    .code = MEDIA_BUS_FMT_SRGGB10_1X10,
    .order = { { VIMC_DEBAYER_RED, VIMC_DEBAYER_GREEN },
    { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_BLUE } }
    },
    {
    .code = MEDIA_BUS_FMT_SBGGR12_1X12,
    .order = { { VIMC_DEBAYER_BLUE, VIMC_DEBAYER_GREEN },
    { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_RED } }
    },
    {
    .code = MEDIA_BUS_FMT_SGBRG12_1X12,
    .order = { { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_BLUE },
    { VIMC_DEBAYER_RED, VIMC_DEBAYER_GREEN } }
    },
    {
    .code = MEDIA_BUS_FMT_SGRBG12_1X12,
    .order = { { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_RED },
    { VIMC_DEBAYER_BLUE, VIMC_DEBAYER_GREEN } }
    },
    {
    .code = MEDIA_BUS_FMT_SRGGB12_1X12,
    .order = { { VIMC_DEBAYER_RED, VIMC_DEBAYER_GREEN },
    { VIMC_DEBAYER_GREEN, VIMC_DEBAYER_BLUE } }
    },
    };
    static const struct vimc_debayer_pix_map *vimc_debayer_pix_map_by_code(u32 code)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(vimc_debayer_pix_map_list); i++)
    if (vimc_debayer_pix_map_list[i].code == code)
    return &vimc_debayer_pix_map_list[i];
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn vimc_debayer_src_code_is_valid(code: u32) -> bool {
    static bool vimc_debayer_src_code_is_valid(u32 code)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(vimc_debayer_src_mbus_codes); i++)
    if (vimc_debayer_src_mbus_codes[i] == code)
    return true;
    return false;
    }
    static int vimc_debayer_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state)
    {
    struct v4l2_mbus_framefmt *mf;
    mf = v4l2_subdev_state_get_format(sd_state, 0);
// mf = sink_fmt_default;
    mf = v4l2_subdev_state_get_format(sd_state, 1);
// mf = sink_fmt_default;
    mf.code = VIMC_DEBAYER_SOURCE_MBUS_FMT;
    return 0;
    }
    static int vimc_debayer_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (VIMC_IS_SRC(code.pad)) {
    if (code.index >= ARRAY_SIZE(vimc_debayer_src_mbus_codes))
    return -EINVAL;
    code.code = vimc_debayer_src_mbus_codes[code.index];
    } else {
    if (code.index >= ARRAY_SIZE(vimc_debayer_pix_map_list))
    return -EINVAL;
    code.code = vimc_debayer_pix_map_list[code.index].code;
    }
    return 0;
    }
    static int vimc_debayer_enum_frame_size(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    if (fse.index)
    return -EINVAL;
    if (VIMC_IS_SINK(fse.pad)) {
    const struct vimc_debayer_pix_map *vpix =
    vimc_debayer_pix_map_by_code(fse.code);
    if (!vpix)
    return -EINVAL;
    } else if (!vimc_debayer_src_code_is_valid(fse.code)) {
    return -EINVAL;
    }
    fse.min_width = VIMC_FRAME_MIN_WIDTH;
    fse.max_width = VIMC_FRAME_MAX_WIDTH;
    fse.min_height = VIMC_FRAME_MIN_HEIGHT;
    fse.max_height = VIMC_FRAME_MAX_HEIGHT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vimc_debayer_adjust_sink_fmt(fmt: *mut v4l2_mbus_framefmt) {
    static void vimc_debayer_adjust_sink_fmt(struct v4l2_mbus_framefmt *fmt)
    {
    const struct vimc_debayer_pix_map *vpix;
// Don't accept a code that is not on the debayer table
    vpix = vimc_debayer_pix_map_by_code(fmt.code);
    if (!vpix)
    fmt.code = sink_fmt_default.code;
    fmt.width = clamp_t(u32, fmt.width, VIMC_FRAME_MIN_WIDTH,
    VIMC_FRAME_MAX_WIDTH) & ~1;
    fmt.height = clamp_t(u32, fmt.height, VIMC_FRAME_MIN_HEIGHT,
    VIMC_FRAME_MAX_HEIGHT) & ~1;
    if (fmt.field == V4L2_FIELD_ANY)
    fmt.field = sink_fmt_default.field;
    vimc_colorimetry_clamp(fmt);
    }
    static int vimc_debayer_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct vimc_debayer_device *vdebayer = v4l2_get_subdevdata(sd);
    struct v4l2_mbus_framefmt *format;
// Do not change the format while stream is on.
    if (fmt.which == V4L2_SUBDEV_FORMAT_ACTIVE && vdebayer.src_frame)
    return -EBUSY;
//
// Do not change the format of the source pad, it is propagated from
// the sink.
//
    if (VIMC_IS_SRC(fmt.pad))
    return v4l2_subdev_get_fmt(sd, sd_state, fmt);
// Set the new format in the sink pad.
    vimc_debayer_adjust_sink_fmt(&fmt.format);
    format = v4l2_subdev_state_get_format(sd_state, 0);
    dev_dbg(vdebayer.ved.dev, "%s: sink format update: "
    "old:%dx%d (0x%x, %d, %d, %d, %d) "
    "new:%dx%d (0x%x, %d, %d, %d, %d)\n", vdebayer.sd.name,
// old
    format.width, format.height, format.code,
    format.colorspace, format.quantization,
    format.xfer_func, format.ycbcr_enc,
// new
    fmt.format.width, fmt.format.height, fmt.format.code,
    fmt.format.colorspace,	fmt.format.quantization,
    fmt.format.xfer_func, fmt.format.ycbcr_enc);
// format = fmt->format;
// Propagate the format to the source pad.
    format = v4l2_subdev_state_get_format(sd_state, 1);
// format = fmt->format;
    format.code = VIMC_DEBAYER_SOURCE_MBUS_FMT;
    return 0;
    }
    static const struct v4l2_subdev_pad_ops vimc_debayer_pad_ops = {
    .enum_mbus_code		= vimc_debayer_enum_mbus_code,
    .enum_frame_size	= vimc_debayer_enum_frame_size,
    .get_fmt		= v4l2_subdev_get_fmt,
    .set_fmt		= vimc_debayer_set_fmt,
    };
    static void vimc_debayer_process_rgb_frame(struct vimc_debayer_device *vdebayer,
    unsigned int lin,
    unsigned int col,
    unsigned int rgb[3])
    {
    const struct vimc_pix_map *vpix;
    unsigned int i, index;
    vpix = vimc_pix_map_by_code(vdebayer.hw.src_code);
    index = VIMC_FRAME_INDEX(lin, col, vdebayer.hw.size.width, 3);
    for (i = 0; i < 3; i++) {
    switch (vpix.pixelformat) {
    case V4L2_PIX_FMT_RGB24:
    vdebayer.src_frame[index + i] = rgb[i];
    break;
    case V4L2_PIX_FMT_BGR24:
    vdebayer.src_frame[index + i] = rgb[2 - i];
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn vimc_debayer_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int vimc_debayer_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct vimc_debayer_device *vdebayer = v4l2_get_subdevdata(sd);
    if (enable) {
    const struct v4l2_mbus_framefmt *sink_fmt;
    const struct v4l2_mbus_framefmt *src_fmt;
    struct v4l2_subdev_state *state;
    const struct vimc_pix_map *vpix;
    unsigned int frame_size;
    if (vdebayer.src_frame)
    return 0;
    state = v4l2_subdev_lock_and_get_active_state(sd);
    sink_fmt = v4l2_subdev_state_get_format(state, 0);
    src_fmt = v4l2_subdev_state_get_format(state, 1);
// Calculate the frame size of the source pad
    vpix = vimc_pix_map_by_code(src_fmt.code);
    frame_size = src_fmt.width * src_fmt.height * vpix.bpp;
// Save the bytes per pixel of the sink
    vpix = vimc_pix_map_by_code(sink_fmt.code);
    vdebayer.hw.sink_bpp = vpix.bpp;
// Get the corresponding pixel map from the table
    vdebayer.hw.sink_pix_map =
    vimc_debayer_pix_map_by_code(sink_fmt.code);
    vdebayer.hw.size.width = sink_fmt.width;
    vdebayer.hw.size.height = sink_fmt.height;
    vdebayer.hw.src_code = src_fmt.code;
    v4l2_subdev_unlock_state(state);
//
// Allocate the frame buffer. Use vmalloc to be able to
// allocate a large amount of memory
//
    vdebayer.src_frame = vmalloc(frame_size);
    if (!vdebayer.src_frame)
    return -ENOMEM;
    } else {
    if (!vdebayer.src_frame)
    return 0;
    vfree(vdebayer.src_frame);
    vdebayer.src_frame = core::ptr::null_mut();
    }
    return 0;
    }
    static const struct v4l2_subdev_core_ops vimc_debayer_core_ops = {
    .log_status = v4l2_ctrl_subdev_log_status,
    .subscribe_event = v4l2_ctrl_subdev_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,
    };
    static const struct v4l2_subdev_video_ops vimc_debayer_video_ops = {
    .s_stream = vimc_debayer_s_stream,
    };
    static const struct v4l2_subdev_ops vimc_debayer_ops = {
    .core = &vimc_debayer_core_ops,
    .pad = &vimc_debayer_pad_ops,
    .video = &vimc_debayer_video_ops,
    };
    static const struct v4l2_subdev_internal_ops vimc_debayer_internal_ops = {
    .init_state = vimc_debayer_init_state,
    };
    static unsigned int vimc_debayer_get_val(const u8 *bytes,
    const unsigned int n_bytes)
    {
    unsigned int i;
    let mut acc: c_uint = 0;
    for (i = 0; i < n_bytes; i++)
    acc = acc + (bytes[i] << (8 * i));
    return acc;
    }
    static void vimc_debayer_calc_rgb_sink(struct vimc_debayer_device *vdebayer,
    const u8 *frame,
    const unsigned int lin,
    const unsigned int col,
    unsigned int rgb[3])
    {
    unsigned int i, seek, wlin, wcol;
    unsigned int n_rgb[3] = {0, 0, 0};
    for (i = 0; i < 3; i++)
    rgb[i] = 0;
//
// Calculate how many we need to subtract to get to the pixel in
// the top left corner of the mean window (considering the current
// pixel as the center)
//
    seek = vdebayer.hw.mean_win_size / 2;
// Sum the values of the colors in the mean window
    dev_dbg(vdebayer.ved.dev,
    "deb: %s: --- Calc pixel %dx%d, window mean %d, seek %d ---\n",
    vdebayer.sd.name, lin, col, vdebayer.hw.size.height, seek);
//
// Iterate through all the lines in the mean window, start
// with zero if the pixel is outside the frame and don't pass
// the height when the pixel is in the bottom border of the
// frame
//
    for (wlin = seek > lin ? 0 : lin - seek;
    wlin < lin + seek + 1 && wlin < vdebayer.hw.size.height;
    wlin++) {
//
// Iterate through all the columns in the mean window, start
// with zero if the pixel is outside the frame and don't pass
// the width when the pixel is in the right border of the
// frame
//
    for (wcol = seek > col ? 0 : col - seek;
    wcol < col + seek + 1 && wcol < vdebayer.hw.size.width;
    wcol++) {
    enum vimc_debayer_rgb_colors color;
    unsigned int index;
// Check which color this pixel is
    color = vdebayer.hw.sink_pix_map.order[wlin % 2][wcol % 2];
    index = VIMC_FRAME_INDEX(wlin, wcol,
    vdebayer.hw.size.width,
    vdebayer.hw.sink_bpp);
    dev_dbg(vdebayer.ved.dev,
    "deb: %s: RGB CALC: frame index %d, win pos %dx%d, color %d\n",
    vdebayer.sd.name, index, wlin, wcol, color);
// Get its value
    rgb[color] = rgb[color] +
    vimc_debayer_get_val(&frame[index],
    vdebayer.hw.sink_bpp);
// Save how many values we already added
    n_rgb[color]++;
    dev_dbg(vdebayer.ved.dev, "deb: %s: RGB CALC: val %d, n %d\n",
    vdebayer.sd.name, rgb[color], n_rgb[color]);
    }
    }
// Calculate the mean
    for (i = 0; i < 3; i++) {
    dev_dbg(vdebayer.ved.dev,
    "deb: %s: PRE CALC: %dx%d Color %d, val %d, n %d\n",
    vdebayer.sd.name, lin, col, i, rgb[i], n_rgb[i]);
    if (n_rgb[i])
    rgb[i] = rgb[i] / n_rgb[i];
    dev_dbg(vdebayer.ved.dev,
    "deb: %s: FINAL CALC: %dx%d Color %d, val %d\n",
    vdebayer.sd.name, lin, col, i, rgb[i]);
    }
    }
    static void *vimc_debayer_process_frame(struct vimc_ent_device *ved,
    const void *sink_frame)
    {
    struct vimc_debayer_device *vdebayer =
    container_of(ved, struct vimc_debayer_device, ved);
    unsigned int rgb[3];
    unsigned int i, j;
// If the stream in this node is not active, just return
    if (!vdebayer.src_frame)
    return ERR_PTR(-EINVAL);
    for (i = 0; i < vdebayer.hw.size.height; i++)
    for (j = 0; j < vdebayer.hw.size.width; j++) {
    vimc_debayer_calc_rgb_sink(vdebayer, sink_frame, i, j, rgb);
    vdebayer.set_rgb_src(vdebayer, i, j, rgb);
    }
    return vdebayer.src_frame;
    }
#[no_mangle]
unsafe extern "C" fn vimc_debayer_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int vimc_debayer_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct vimc_debayer_device *vdebayer =
    container_of(ctrl.handler, struct vimc_debayer_device, hdl);
    switch (ctrl.id) {
    case VIMC_CID_MEAN_WIN_SIZE:
    vdebayer.hw.mean_win_size = ctrl.val;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct v4l2_ctrl_ops vimc_debayer_ctrl_ops = {
    .s_ctrl = vimc_debayer_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn vimc_debayer_release(ved: *mut vimc_ent_device) {
    static void vimc_debayer_release(struct vimc_ent_device *ved)
    {
    struct vimc_debayer_device *vdebayer =
    container_of(ved, struct vimc_debayer_device, ved);
    v4l2_ctrl_handler_free(&vdebayer.hdl);
    v4l2_subdev_cleanup(&vdebayer.sd);
    media_entity_cleanup(vdebayer.ved.ent);
    kfree(vdebayer);
    }
    static const struct v4l2_ctrl_config vimc_debayer_ctrl_class = {
    .flags = V4L2_CTRL_FLAG_READ_ONLY | V4L2_CTRL_FLAG_WRITE_ONLY,
    .id = VIMC_CID_VIMC_CLASS,
    .name = "VIMC Controls",
    .type = V4L2_CTRL_TYPE_CTRL_CLASS,
    };
    static const struct v4l2_ctrl_config vimc_debayer_ctrl_mean_win_size = {
    .ops = &vimc_debayer_ctrl_ops,
    .id = VIMC_CID_MEAN_WIN_SIZE,
    .name = "Debayer Mean Window Size",
    .type = V4L2_CTRL_TYPE_INTEGER,
    .min = 1,
    .max = 25,
    .step = 2,
    .def = 3,
    };
    static struct vimc_ent_device *vimc_debayer_add(struct vimc_device *vimc,
    const char *vcfg_name)
    {
    struct v4l2_device *v4l2_dev = &vimc.v4l2_dev;
    struct vimc_debayer_device *vdebayer;
    int ret;
// Allocate the vdebayer struct
    vdebayer = kzalloc_obj(*vdebayer);
    if (!vdebayer)
    return ERR_PTR(-ENOMEM);
// Create controls:
    v4l2_ctrl_handler_init(&vdebayer.hdl, 2);
    v4l2_ctrl_new_custom(&vdebayer.hdl, &vimc_debayer_ctrl_class, core::ptr::null_mut());
    v4l2_ctrl_new_custom(&vdebayer.hdl, &vimc_debayer_ctrl_mean_win_size, core::ptr::null_mut());
    vdebayer.sd.ctrl_handler = &vdebayer.hdl;
    if (vdebayer.hdl.error) {
    ret = vdebayer.hdl.error;
    goto err_free_vdebayer;
    }
// Initialize ved and sd
    vdebayer.pads[0].flags = MEDIA_PAD_FL_SINK;
    vdebayer.pads[1].flags = MEDIA_PAD_FL_SOURCE;
    ret = vimc_ent_sd_register(&vdebayer.ved, &vdebayer.sd, v4l2_dev,
    vcfg_name,
    MEDIA_ENT_F_PROC_VIDEO_PIXEL_ENC_CONV, 2,
    vdebayer.pads, &vimc_debayer_internal_ops,
    &vimc_debayer_ops);
    if (ret)
    goto err_free_hdl;
    vdebayer.ved.process_frame = vimc_debayer_process_frame;
    vdebayer.ved.dev = vimc.mdev.dev;
    vdebayer.hw.mean_win_size = vimc_debayer_ctrl_mean_win_size.def;
    vdebayer.set_rgb_src = vimc_debayer_process_rgb_frame;
    return &vdebayer.ved;
    err_free_hdl:
    v4l2_ctrl_handler_free(&vdebayer.hdl);
    err_free_vdebayer:
    kfree(vdebayer);
    return ERR_PTR(ret);
    }
    const struct vimc_ent_type vimc_debayer_type = {
    .add = vimc_debayer_add,
    .release = vimc_debayer_release
    };
