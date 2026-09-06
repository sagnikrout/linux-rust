//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/mt9m111.c
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
// Driver for MT9M111/MT9M112/MT9M131 CMOS Image Sensor from Micron/Aptina
//
// Copyright (C) 2008, Robert Jarzmik <robert.jarzmik@free.fr>
//

//
// MT9M111, MT9M112 and MT9M131:
// i2c address is 0x48 or 0x5d (depending on SADDR pin)
// The platform has to define struct i2c_board_info objects and link to them
// from struct soc_camera_host_desc
//
// Sensor core register addresses (0x000..0x0ff)
//
pub const MT9M111_CHIP_VERSION: c_uint = 0x000;
pub const MT9M111_ROW_START: c_uint = 0x001;
pub const MT9M111_COLUMN_START: c_uint = 0x002;
pub const MT9M111_WINDOW_HEIGHT: c_uint = 0x003;
pub const MT9M111_WINDOW_WIDTH: c_uint = 0x004;
pub const MT9M111_HORIZONTAL_BLANKING_B: c_uint = 0x005;
pub const MT9M111_VERTICAL_BLANKING_B: c_uint = 0x006;
pub const MT9M111_HORIZONTAL_BLANKING_A: c_uint = 0x007;
pub const MT9M111_VERTICAL_BLANKING_A: c_uint = 0x008;
pub const MT9M111_SHUTTER_WIDTH: c_uint = 0x009;
pub const MT9M111_ROW_SPEED: c_uint = 0x00a;
pub const MT9M111_EXTRA_DELAY: c_uint = 0x00b;
pub const MT9M111_SHUTTER_DELAY: c_uint = 0x00c;
pub const MT9M111_RESET: c_uint = 0x00d;
pub const MT9M111_READ_MODE_B: c_uint = 0x020;
pub const MT9M111_READ_MODE_A: c_uint = 0x021;
pub const MT9M111_FLASH_CONTROL: c_uint = 0x023;
pub const MT9M111_GREEN1_GAIN: c_uint = 0x02b;
pub const MT9M111_BLUE_GAIN: c_uint = 0x02c;
pub const MT9M111_RED_GAIN: c_uint = 0x02d;
pub const MT9M111_GREEN2_GAIN: c_uint = 0x02e;
pub const MT9M111_GLOBAL_GAIN: c_uint = 0x02f;
pub const MT9M111_CONTEXT_CONTROL: c_uint = 0x0c8;
pub const MT9M111_PAGE_MAP: c_uint = 0x0f0;
pub const MT9M111_BYTE_WISE_ADDR: c_uint = 0x0f1;

//
// Colorpipe register addresses (0x100..0x1ff)
//
pub const MT9M111_OPER_MODE_CTRL: c_uint = 0x106;
pub const MT9M111_OUTPUT_FORMAT_CTRL: c_uint = 0x108;
pub const MT9M111_TPG_CTRL: c_uint = 0x148;
pub const MT9M111_REDUCER_XZOOM_B: c_uint = 0x1a0;
pub const MT9M111_REDUCER_XSIZE_B: c_uint = 0x1a1;
pub const MT9M111_REDUCER_YZOOM_B: c_uint = 0x1a3;
pub const MT9M111_REDUCER_YSIZE_B: c_uint = 0x1a4;
pub const MT9M111_REDUCER_XZOOM_A: c_uint = 0x1a6;
pub const MT9M111_REDUCER_XSIZE_A: c_uint = 0x1a7;
pub const MT9M111_REDUCER_YZOOM_A: c_uint = 0x1a9;
pub const MT9M111_REDUCER_YSIZE_A: c_uint = 0x1aa;
pub const MT9M111_EFFECTS_MODE: c_uint = 0x1e2;
pub const MT9M111_OUTPUT_FORMAT_CTRL2_A: c_uint = 0x13a;
pub const MT9M111_OUTPUT_FORMAT_CTRL2_B: c_uint = 0x19b;

//
// Camera control register addresses (0x200..0x2ff not implemented)
//

    (val), (mask))
pub const MT9M111_MIN_DARK_ROWS: c_int = 8;
pub const MT9M111_MIN_DARK_COLS: c_int = 26;
pub const MT9M111_MAX_HEIGHT: c_int = 1024;
pub const MT9M111_MAX_WIDTH: c_int = 1280;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9m111_context {
    pub read_mode: u16,
    pub blanking_h: u16,
    pub blanking_v: u16,
    pub reducer_xzoom: u16,
    pub reducer_yzoom: u16,
    pub reducer_xsize: u16,
    pub reducer_ysize: u16,
    pub output_fmt_ctrl2: u16,
    pub control: u16,
}

    static struct mt9m111_context context_a = {
    .read_mode		= MT9M111_READ_MODE_A,
    .blanking_h		= MT9M111_HORIZONTAL_BLANKING_A,
    .blanking_v		= MT9M111_VERTICAL_BLANKING_A,
    .reducer_xzoom		= MT9M111_REDUCER_XZOOM_A,
    .reducer_yzoom		= MT9M111_REDUCER_YZOOM_A,
    .reducer_xsize		= MT9M111_REDUCER_XSIZE_A,
    .reducer_ysize		= MT9M111_REDUCER_YSIZE_A,
    .output_fmt_ctrl2	= MT9M111_OUTPUT_FORMAT_CTRL2_A,
    .control		= MT9M111_CTXT_CTRL_RESTART,
    };
    static struct mt9m111_context context_b = {
    .read_mode		= MT9M111_READ_MODE_B,
    .blanking_h		= MT9M111_HORIZONTAL_BLANKING_B,
    .blanking_v		= MT9M111_VERTICAL_BLANKING_B,
    .reducer_xzoom		= MT9M111_REDUCER_XZOOM_B,
    .reducer_yzoom		= MT9M111_REDUCER_YZOOM_B,
    .reducer_xsize		= MT9M111_REDUCER_XSIZE_B,
    .reducer_ysize		= MT9M111_REDUCER_YSIZE_B,
    .output_fmt_ctrl2	= MT9M111_OUTPUT_FORMAT_CTRL2_B,
    .control		= MT9M111_CTXT_CTRL_RESTART |
    MT9M111_CTXT_CTRL_DEFECTCOR_B | MT9M111_CTXT_CTRL_RESIZE_B |
    MT9M111_CTXT_CTRL_CTRL2_B | MT9M111_CTXT_CTRL_GAMMA_B |
    MT9M111_CTXT_CTRL_READ_MODE_B | MT9M111_CTXT_CTRL_VBLANK_SEL_B |
    MT9M111_CTXT_CTRL_HBLANK_SEL_B,
    };
// MT9M111 has only one fixed colorspace per pixelcode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9m111_datafmt {
    pub code: u32,
    pub colorspace: enum v4l2_colorspace,
}

    static const struct mt9m111_datafmt mt9m111_colour_fmts[] = {
    {MEDIA_BUS_FMT_YUYV8_2X8, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_YVYU8_2X8, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_UYVY8_2X8, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_VYUY8_2X8, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_RGB555_2X8_PADHI_LE, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_RGB555_2X8_PADHI_BE, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_RGB565_2X8_LE, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_RGB565_2X8_BE, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_BGR565_2X8_LE, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_BGR565_2X8_BE, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_SBGGR8_1X8, V4L2_COLORSPACE_SRGB},
    {MEDIA_BUS_FMT_SBGGR10_2X8_PADHI_LE, V4L2_COLORSPACE_SRGB},
    };
    enum mt9m111_mode_id {
    MT9M111_MODE_SXGA_8FPS,
    MT9M111_MODE_SXGA_15FPS,
    MT9M111_MODE_QSXGA_30FPS,
    MT9M111_NUM_MODES,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9m111_mode_info {
    pub sensor_w: c_uint,
    pub sensor_h: c_uint,
    pub max_image_w: c_uint,
    pub max_image_h: c_uint,
    pub max_fps: c_uint,
    pub reg_val: c_uint,
    pub reg_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9m111 {
    pub subdev: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub gain: *mut v4l2_ctrl,
    pub ctx: *mut mt9m111_context,
    pub /: *mut *mut v4l2_rect rect; / cropping rectangle,
    pub clk: *mut clk,
    pub /: *mut *mut unsigned int width; / output,
    pub /: *mut *mut unsigned int height; / sizes,
    pub frame_interval: v4l2_fract,
    pub current_mode: *const mt9m111_mode_info,
    pub /: *mut *mut mutex power_lock; / lock to protect power_count,
    pub power_count: c_int,
    pub fmt: *const mt9m111_datafmt,
    pub /: *mut *mut int lastpage; / PageMap cache value,
    pub regulator: *mut regulator,
    pub is_streaming: bool,
// user point of view - 0: falling 1: rising edge
    pub pclk_sample:1: c_uint,
    pub pad: media_pad,
}

    static const struct mt9m111_mode_info mt9m111_mode_data[MT9M111_NUM_MODES] = {
    [MT9M111_MODE_SXGA_8FPS] = {
    .sensor_w = 1280,
    .sensor_h = 1024,
    .max_image_w = 1280,
    .max_image_h = 1024,
    .max_fps = 8,
    .reg_val = MT9M111_RM_LOW_POWER_RD,
    .reg_mask = MT9M111_RM_PWR_MASK | MT9M111_RM_SKIP2_MASK,
    },
    [MT9M111_MODE_SXGA_15FPS] = {
    .sensor_w = 1280,
    .sensor_h = 1024,
    .max_image_w = 1280,
    .max_image_h = 1024,
    .max_fps = 15,
    .reg_val = MT9M111_RM_FULL_POWER_RD,
    .reg_mask = MT9M111_RM_PWR_MASK | MT9M111_RM_SKIP2_MASK,
    },
    [MT9M111_MODE_QSXGA_30FPS] = {
    .sensor_w = 1280,
    .sensor_h = 1024,
    .max_image_w = 640,
    .max_image_h = 512,
    .max_fps = 30,
    .reg_val = MT9M111_RM_LOW_POWER_RD | MT9M111_RM_COL_SKIP_2X |
    MT9M111_RM_ROW_SKIP_2X,
    .reg_mask = MT9M111_RM_PWR_MASK | MT9M111_RM_SKIP2_MASK,
    },
    };
// Find a data format by a pixel code
    static const struct mt9m111_datafmt *mt9m111_find_datafmt(struct mt9m111 *mt9m111,
    u32 code)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(mt9m111_colour_fmts); i++)
    if (mt9m111_colour_fmts[i].code == code)
    return mt9m111_colour_fmts + i;
    return mt9m111.fmt;
    }
    static struct mt9m111 *to_mt9m111(const struct i2c_client *client)
    {
    return container_of(i2c_get_clientdata(client), struct mt9m111, subdev);
    }
#[no_mangle]
unsafe extern "C" fn reg_page_map_set(client: *mut i2c_client, reg: u16) -> c_int {
    static int reg_page_map_set(struct i2c_client *client, const u16 reg)
    {
    int ret;
    u16 page;
    struct mt9m111 *mt9m111 = to_mt9m111(client);
    page = (reg >> 8);
    if (page == mt9m111.lastpage)
    return 0;
    if (page > 2)
    return -EINVAL;
    ret = i2c_smbus_write_word_swapped(client, MT9M111_PAGE_MAP, page);
    if (!ret)
    mt9m111.lastpage = page;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_reg_read(client: *mut i2c_client, reg: u16) -> c_int {
    static int mt9m111_reg_read(struct i2c_client *client, const u16 reg)
    {
    int ret;
    ret = reg_page_map_set(client, reg);
    if (!ret)
    ret = i2c_smbus_read_word_swapped(client, reg & 0xff);
    dev_dbg(&client.dev, "read  reg.%03x . %04x\n", reg, ret);
    return ret;
    }
    static int mt9m111_reg_write(struct i2c_client *client, const u16 reg,
    const u16 data)
    {
    int ret;
    ret = reg_page_map_set(client, reg);
    if (!ret)
    ret = i2c_smbus_write_word_swapped(client, reg & 0xff, data);
    dev_dbg(&client.dev, "write reg.%03x = %04x . %d\n", reg, data, ret);
    return ret;
    }
    static int mt9m111_reg_set(struct i2c_client *client, const u16 reg,
    const u16 data)
    {
    int ret;
    ret = mt9m111_reg_read(client, reg);
    if (ret >= 0)
    ret = mt9m111_reg_write(client, reg, ret | data);
    return ret;
    }
    static int mt9m111_reg_clear(struct i2c_client *client, const u16 reg,
    const u16 data)
    {
    int ret;
    ret = mt9m111_reg_read(client, reg);
    if (ret >= 0)
    ret = mt9m111_reg_write(client, reg, ret & ~data);
    return ret;
    }
    static int mt9m111_reg_mask(struct i2c_client *client, const u16 reg,
    const u16 data, const u16 mask)
    {
    int ret;
    ret = mt9m111_reg_read(client, reg);
    if (ret >= 0)
    ret = mt9m111_reg_write(client, reg, (ret & ~mask) | data);
    return ret;
    }
    static int mt9m111_set_context(struct mt9m111 *mt9m111,
    struct mt9m111_context *ctx)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    return reg_write(CONTEXT_CONTROL, ctx.control);
    }
    static int mt9m111_setup_rect_ctx(struct mt9m111 *mt9m111,
    struct mt9m111_context *ctx, struct v4l2_rect *rect,
    unsigned int width, unsigned int height)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    let mut ret: c_int = mt9m111_reg_write(client, ctx.reducer_xzoom, rect.width);
    if (!ret)
    ret = mt9m111_reg_write(client, ctx.reducer_yzoom, rect.height);
    if (!ret)
    ret = mt9m111_reg_write(client, ctx.reducer_xsize, width);
    if (!ret)
    ret = mt9m111_reg_write(client, ctx.reducer_ysize, height);
    return ret;
    }
    static int mt9m111_setup_geometry(struct mt9m111 *mt9m111, struct v4l2_rect *rect,
    int width, int height, u32 code)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    int ret;
    ret = reg_write(COLUMN_START, rect.left);
    if (!ret)
    ret = reg_write(ROW_START, rect.top);
    if (!ret)
    ret = reg_write(WINDOW_WIDTH, rect.width);
    if (!ret)
    ret = reg_write(WINDOW_HEIGHT, rect.height);
    if (code != MEDIA_BUS_FMT_SBGGR10_2X8_PADHI_LE) {
// IFP in use, down-scaling possible
    if (!ret)
    ret = mt9m111_setup_rect_ctx(mt9m111, &context_b,
    rect, width, height);
    if (!ret)
    ret = mt9m111_setup_rect_ctx(mt9m111, &context_a,
    rect, width, height);
    }
    dev_dbg(&client.dev, "%s(%x): %ux%u@%u:%u . %ux%u = %d\n",
    __func__, code, rect.width, rect.height, rect.left, rect.top,
    width, height, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_enable(mt9m111: *mut mt9m111) -> c_int {
    static int mt9m111_enable(struct mt9m111 *mt9m111)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    return reg_write(RESET, MT9M111_RESET_CHIP_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_reset(mt9m111: *mut mt9m111) -> c_int {
    static int mt9m111_reset(struct mt9m111 *mt9m111)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    int ret;
    ret = reg_set(RESET, MT9M111_RESET_RESET_MODE);
    if (!ret)
    ret = reg_set(RESET, MT9M111_RESET_RESET_SOC);
    if (!ret)
    ret = reg_clear(RESET, MT9M111_RESET_RESET_MODE
    | MT9M111_RESET_RESET_SOC);
    return ret;
    }
    static int mt9m111_set_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct mt9m111 *mt9m111 = to_mt9m111(client);
    let mut rect: v4l2_rect = sel.r;
    int width, height;
    int ret, align = 0;
    if (sel.which != V4L2_SUBDEV_FORMAT_ACTIVE ||
    sel.target != V4L2_SEL_TGT_CROP)
    return -EINVAL;
    if (mt9m111.fmt.code == MEDIA_BUS_FMT_SBGGR8_1X8 ||
    mt9m111.fmt.code == MEDIA_BUS_FMT_SBGGR10_2X8_PADHI_LE) {
// Bayer format - even size lengths
    align = 1;
// Let the user play with the starting pixel
    }
// FIXME: the datasheet doesn't specify minimum sizes
    v4l_bound_align_image(&rect.width, 2, MT9M111_MAX_WIDTH, align,
    &rect.height, 2, MT9M111_MAX_HEIGHT, align, 0);
    rect.left = clamp(rect.left, MT9M111_MIN_DARK_COLS,
    MT9M111_MIN_DARK_COLS + MT9M111_MAX_WIDTH -
    (__s32)rect.width);
    rect.top = clamp(rect.top, MT9M111_MIN_DARK_ROWS,
    MT9M111_MIN_DARK_ROWS + MT9M111_MAX_HEIGHT -
    (__s32)rect.height);
    width = min(mt9m111.width, rect.width);
    height = min(mt9m111.height, rect.height);
    ret = mt9m111_setup_geometry(mt9m111, &rect, width, height, mt9m111.fmt.code);
    if (!ret) {
    mt9m111.rect = rect;
    mt9m111.width = width;
    mt9m111.height = height;
    }
    return ret;
    }
    static int mt9m111_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct mt9m111 *mt9m111 = to_mt9m111(client);
    if (sel.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r.left = MT9M111_MIN_DARK_COLS;
    sel.r.top = MT9M111_MIN_DARK_ROWS;
    sel.r.width = MT9M111_MAX_WIDTH;
    sel.r.height = MT9M111_MAX_HEIGHT;
    return 0;
    case V4L2_SEL_TGT_CROP:
    sel.r = mt9m111.rect;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int mt9m111_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *mf = &format.format;
    struct mt9m111 *mt9m111 = container_of(sd, struct mt9m111, subdev);
    if (format.pad)
    return -EINVAL;
    if (format.which == V4L2_SUBDEV_FORMAT_TRY) {
    mf = v4l2_subdev_state_get_format(sd_state, format.pad);
    format.format = *mf;
    return 0;
    }
    mf.width	= mt9m111.width;
    mf.height	= mt9m111.height;
    mf.code	= mt9m111.fmt.code;
    mf.colorspace	= mt9m111.fmt.colorspace;
    mf.field	= V4L2_FIELD_NONE;
    mf.ycbcr_enc	= V4L2_YCBCR_ENC_DEFAULT;
    mf.quantization	= V4L2_QUANTIZATION_DEFAULT;
    mf.xfer_func	= V4L2_XFER_FUNC_DEFAULT;
    return 0;
    }
    static int mt9m111_set_pixfmt(struct mt9m111 *mt9m111,
    u32 code)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    u16 data_outfmt2, mask_outfmt2 = MT9M111_OUTFMT_PROCESSED_BAYER |
    MT9M111_OUTFMT_BYPASS_IFP | MT9M111_OUTFMT_RGB |
    MT9M111_OUTFMT_RGB565 | MT9M111_OUTFMT_RGB555 |
    MT9M111_OUTFMT_RGB444x | MT9M111_OUTFMT_RGBx444 |
    MT9M111_OUTFMT_SWAP_YCbCr_C_Y_RGB_EVEN |
    MT9M111_OUTFMT_SWAP_YCbCr_Cb_Cr_RGB_R_B;
    int ret;
    switch (code) {
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    data_outfmt2 = MT9M111_OUTFMT_PROCESSED_BAYER |
    MT9M111_OUTFMT_RGB;
    break;
    case MEDIA_BUS_FMT_SBGGR10_2X8_PADHI_LE:
    data_outfmt2 = MT9M111_OUTFMT_BYPASS_IFP | MT9M111_OUTFMT_RGB;
    break;
    case MEDIA_BUS_FMT_RGB555_2X8_PADHI_LE:
    data_outfmt2 = MT9M111_OUTFMT_RGB | MT9M111_OUTFMT_RGB555 |
    MT9M111_OUTFMT_SWAP_YCbCr_C_Y_RGB_EVEN;
    break;
    case MEDIA_BUS_FMT_RGB555_2X8_PADHI_BE:
    data_outfmt2 = MT9M111_OUTFMT_RGB | MT9M111_OUTFMT_RGB555;
    break;
    case MEDIA_BUS_FMT_RGB565_2X8_LE:
    data_outfmt2 = MT9M111_OUTFMT_RGB | MT9M111_OUTFMT_RGB565 |
    MT9M111_OUTFMT_SWAP_YCbCr_C_Y_RGB_EVEN;
    break;
    case MEDIA_BUS_FMT_RGB565_2X8_BE:
    data_outfmt2 = MT9M111_OUTFMT_RGB | MT9M111_OUTFMT_RGB565;
    break;
    case MEDIA_BUS_FMT_BGR565_2X8_BE:
    data_outfmt2 = MT9M111_OUTFMT_RGB | MT9M111_OUTFMT_RGB565 |
    MT9M111_OUTFMT_SWAP_YCbCr_Cb_Cr_RGB_R_B;
    break;
    case MEDIA_BUS_FMT_BGR565_2X8_LE:
    data_outfmt2 = MT9M111_OUTFMT_RGB | MT9M111_OUTFMT_RGB565 |
    MT9M111_OUTFMT_SWAP_YCbCr_C_Y_RGB_EVEN |
    MT9M111_OUTFMT_SWAP_YCbCr_Cb_Cr_RGB_R_B;
    break;
    case MEDIA_BUS_FMT_UYVY8_2X8:
    data_outfmt2 = 0;
    break;
    case MEDIA_BUS_FMT_VYUY8_2X8:
    data_outfmt2 = MT9M111_OUTFMT_SWAP_YCbCr_Cb_Cr_RGB_R_B;
    break;
    case MEDIA_BUS_FMT_YUYV8_2X8:
    data_outfmt2 = MT9M111_OUTFMT_SWAP_YCbCr_C_Y_RGB_EVEN;
    break;
    case MEDIA_BUS_FMT_YVYU8_2X8:
    data_outfmt2 = MT9M111_OUTFMT_SWAP_YCbCr_C_Y_RGB_EVEN |
    MT9M111_OUTFMT_SWAP_YCbCr_Cb_Cr_RGB_R_B;
    break;
    default:
    dev_err(&client.dev, "Pixel format not handled: %x\n", code);
    return -EINVAL;
    }
// receiver samples on falling edge, chip-hw default is rising
    if (mt9m111.pclk_sample == 0)
    mask_outfmt2 |= MT9M111_OUTFMT_INV_PIX_CLOCK;
    ret = mt9m111_reg_mask(client, context_a.output_fmt_ctrl2,
    data_outfmt2, mask_outfmt2);
    if (!ret)
    ret = mt9m111_reg_mask(client, context_b.output_fmt_ctrl2,
    data_outfmt2, mask_outfmt2);
    return ret;
    }
    static int mt9m111_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *mf = &format.format;
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct mt9m111 *mt9m111 = container_of(sd, struct mt9m111, subdev);
    const struct mt9m111_datafmt *fmt;
    struct v4l2_rect *rect = &mt9m111.rect;
    bool bayer;
    int ret;
    if (mt9m111.is_streaming)
    return -EBUSY;
    if (format.pad)
    return -EINVAL;
    fmt = mt9m111_find_datafmt(mt9m111, mf.code);
    bayer = fmt.code == MEDIA_BUS_FMT_SBGGR8_1X8 ||
    fmt.code == MEDIA_BUS_FMT_SBGGR10_2X8_PADHI_LE;
//
// With Bayer format enforce even side lengths, but let the user play
// with the starting pixel
//
    if (bayer) {
    rect.width = ALIGN(rect.width, 2);
    rect.height = ALIGN(rect.height, 2);
    }
    if (fmt.code == MEDIA_BUS_FMT_SBGGR10_2X8_PADHI_LE) {
// IFP bypass mode, no scaling
    mf.width = rect.width;
    mf.height = rect.height;
    } else {
// No upscaling
    if (mf.width > rect.width)
    mf.width = rect.width;
    if (mf.height > rect.height)
    mf.height = rect.height;
    }
    dev_dbg(&client.dev, "%s(): %ux%u, code=%x\n", __func__,
    mf.width, mf.height, fmt.code);
    mf.code = fmt.code;
    mf.colorspace = fmt.colorspace;
    mf.field	= V4L2_FIELD_NONE;
    mf.ycbcr_enc	= V4L2_YCBCR_ENC_DEFAULT;
    mf.quantization	= V4L2_QUANTIZATION_DEFAULT;
    mf.xfer_func	= V4L2_XFER_FUNC_DEFAULT;
    if (format.which == V4L2_SUBDEV_FORMAT_TRY) {
// v4l2_subdev_state_get_format(sd_state, 0) = *mf;
    return 0;
    }
    ret = mt9m111_setup_geometry(mt9m111, rect, mf.width, mf.height, mf.code);
    if (!ret)
    ret = mt9m111_set_pixfmt(mt9m111, mf.code);
    if (!ret) {
    mt9m111.width	= mf.width;
    mt9m111.height	= mf.height;
    mt9m111.fmt	= fmt;
    }
    return ret;
    }
    static const struct mt9m111_mode_info *
    mt9m111_find_mode(struct mt9m111 *mt9m111, unsigned int req_fps,
    unsigned int width, unsigned int height)
    {
    const struct mt9m111_mode_info *mode;
    struct v4l2_rect *sensor_rect = &mt9m111.rect;
    unsigned int gap, gap_best = (unsigned int) -1;
    int i, best_gap_idx = MT9M111_MODE_SXGA_15FPS;
    let mut skip_30fps: bool = false;
//
// The fps selection is based on the row, column skipping mechanism.
// So ensure that the sensor window is set to default else the fps
// aren't calculated correctly within the sensor hw.
//
    if (sensor_rect.width != MT9M111_MAX_WIDTH ||
    sensor_rect.height != MT9M111_MAX_HEIGHT) {
    dev_info(mt9m111.subdev.dev,
    "Framerate selection is not supported for cropped "
    "images\n");
    return core::ptr::null_mut();
    }
// 30fps only supported for images not exceeding 640x512
    if (width > MT9M111_MAX_WIDTH / 2 || height > MT9M111_MAX_HEIGHT / 2) {
    dev_dbg(mt9m111.subdev.dev,
    "Framerates > 15fps are supported only for images "
    "not exceeding 640x512\n");
    skip_30fps = true;
    }
// find best matched fps
    for (i = 0; i < MT9M111_NUM_MODES; i++) {
    let mut fps: c_uint = mt9m111_mode_data[i].max_fps;
    if (fps == 30 && skip_30fps)
    continue;
    gap = abs(fps - req_fps);
    if (gap < gap_best) {
    best_gap_idx = i;
    gap_best = gap;
    }
    }
//
// Use context a/b default timing values instead of calculate blanking
// timing values.
//
    mode = &mt9m111_mode_data[best_gap_idx];
    mt9m111.ctx = (best_gap_idx == MT9M111_MODE_QSXGA_30FPS) ? &context_a :
    &context_b;
    return mode;
    }

    static int mt9m111_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    int val;
    if (reg.reg > 0x2ff)
    return -EINVAL;
    val = mt9m111_reg_read(client, reg.reg);
    reg.size = 2;
    reg.val = (u64)val;
    if (reg.val > 0xffff)
    return -EIO;
    return 0;
    }
    static int mt9m111_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    if (reg.reg > 0x2ff)
    return -EINVAL;
    if (mt9m111_reg_write(client, reg.reg, reg.val) < 0)
    return -EIO;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn mt9m111_set_flip(mt9m111: *mut mt9m111, flip: c_int, mask: c_int) -> c_int {
    static int mt9m111_set_flip(struct mt9m111 *mt9m111, int flip, int mask)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    int ret;
    if (flip)
    ret = mt9m111_reg_set(client, mt9m111.ctx.read_mode, mask);
    else
    ret = mt9m111_reg_clear(client, mt9m111.ctx.read_mode, mask);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_get_global_gain(mt9m111: *mut mt9m111) -> c_int {
    static int mt9m111_get_global_gain(struct mt9m111 *mt9m111)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    int data;
    data = reg_read(GLOBAL_GAIN);
    if (data >= 0)
    return (data & 0x2f) * (1 << ((data >> 10) & 1)) *
    (1 << ((data >> 9) & 1));
    return data;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_set_global_gain(mt9m111: *mut mt9m111, gain: c_int) -> c_int {
    static int mt9m111_set_global_gain(struct mt9m111 *mt9m111, int gain)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    u16 val;
    if (gain > 63 * 2 * 2)
    return -EINVAL;
    if ((gain >= 64 * 2) && (gain < 63 * 2 * 2))
    val = (1 << 10) | (1 << 9) | (gain / 4);
#[no_mangle]
pub unsafe extern "C" fn if(2): *mut *mut (gain >= 64) && (gain < 64) -> else {
    else if ((gain >= 64) && (gain < 64 * 2))
    val = (1 << 9) | (gain / 2);
    else
    val = gain;
    return reg_write(GLOBAL_GAIN, val);
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_set_autoexposure(mt9m111: *mut mt9m111, val: c_int) -> c_int {
    static int mt9m111_set_autoexposure(struct mt9m111 *mt9m111, int val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    if (val == V4L2_EXPOSURE_AUTO)
    return reg_set(OPER_MODE_CTRL, MT9M111_OPMODE_AUTOEXPO_EN);
    return reg_clear(OPER_MODE_CTRL, MT9M111_OPMODE_AUTOEXPO_EN);
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_set_autowhitebalance(mt9m111: *mut mt9m111, on: c_int) -> c_int {
    static int mt9m111_set_autowhitebalance(struct mt9m111 *mt9m111, int on)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    if (on)
    return reg_set(OPER_MODE_CTRL, MT9M111_OPMODE_AUTOWHITEBAL_EN);
    return reg_clear(OPER_MODE_CTRL, MT9M111_OPMODE_AUTOWHITEBAL_EN);
    }
    static const char * const mt9m111_test_pattern_menu[] = {
    "Disabled",
    "Vertical monochrome gradient",
    "Flat color type 1",
    "Flat color type 2",
    "Flat color type 3",
    "Flat color type 4",
    "Flat color type 5",
    "Color bar",
    };
#[no_mangle]
unsafe extern "C" fn mt9m111_set_test_pattern(mt9m111: *mut mt9m111, val: c_int) -> c_int {
    static int mt9m111_set_test_pattern(struct mt9m111 *mt9m111, int val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    return mt9m111_reg_mask(client, MT9M111_TPG_CTRL, val,
    MT9M111_TPG_SEL_MASK);
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_set_colorfx(mt9m111: *mut mt9m111, val: c_int) -> c_int {
    static int mt9m111_set_colorfx(struct mt9m111 *mt9m111, int val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    static const struct v4l2_control colorfx[] = {
    { V4L2_COLORFX_NONE,		0 },
    { V4L2_COLORFX_BW,		1 },
    { V4L2_COLORFX_SEPIA,		2 },
    { V4L2_COLORFX_NEGATIVE,	3 },
    { V4L2_COLORFX_SOLARIZATION,	4 },
    };
    int i;
    for (i = 0; i < ARRAY_SIZE(colorfx); i++) {
    if (colorfx[i].id == val) {
    return mt9m111_reg_mask(client, MT9M111_EFFECTS_MODE,
    colorfx[i].value,
    MT9M111_EFFECTS_MODE_MASK);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int mt9m111_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct mt9m111 *mt9m111 = container_of(ctrl.handler,
    struct mt9m111, hdl);
    switch (ctrl.id) {
    case V4L2_CID_VFLIP:
    return mt9m111_set_flip(mt9m111, ctrl.val,
    MT9M111_RMB_MIRROR_ROWS);
    case V4L2_CID_HFLIP:
    return mt9m111_set_flip(mt9m111, ctrl.val,
    MT9M111_RMB_MIRROR_COLS);
    case V4L2_CID_GAIN:
    return mt9m111_set_global_gain(mt9m111, ctrl.val);
    case V4L2_CID_EXPOSURE_AUTO:
    return mt9m111_set_autoexposure(mt9m111, ctrl.val);
    case V4L2_CID_AUTO_WHITE_BALANCE:
    return mt9m111_set_autowhitebalance(mt9m111, ctrl.val);
    case V4L2_CID_TEST_PATTERN:
    return mt9m111_set_test_pattern(mt9m111, ctrl.val);
    case V4L2_CID_COLORFX:
    return mt9m111_set_colorfx(mt9m111, ctrl.val);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_suspend(mt9m111: *mut mt9m111) -> c_int {
    static int mt9m111_suspend(struct mt9m111 *mt9m111)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    int ret;
    v4l2_ctrl_s_ctrl(mt9m111.gain, mt9m111_get_global_gain(mt9m111));
    ret = reg_set(RESET, MT9M111_RESET_RESET_MODE);
    if (!ret)
    ret = reg_set(RESET, MT9M111_RESET_RESET_SOC |
    MT9M111_RESET_OUTPUT_DISABLE |
    MT9M111_RESET_ANALOG_STANDBY);
    if (!ret)
    ret = reg_clear(RESET, MT9M111_RESET_CHIP_ENABLE);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_restore_state(mt9m111: *mut mt9m111) {
    static void mt9m111_restore_state(struct mt9m111 *mt9m111)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    mt9m111_set_context(mt9m111, mt9m111.ctx);
    mt9m111_set_pixfmt(mt9m111, mt9m111.fmt.code);
    mt9m111_setup_geometry(mt9m111, &mt9m111.rect,
    mt9m111.width, mt9m111.height, mt9m111.fmt.code);
    v4l2_ctrl_handler_setup(&mt9m111.hdl);
    mt9m111_reg_mask(client, mt9m111.ctx.read_mode,
    mt9m111.current_mode.reg_val,
    mt9m111.current_mode.reg_mask);
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_resume(mt9m111: *mut mt9m111) -> c_int {
    static int mt9m111_resume(struct mt9m111 *mt9m111)
    {
    let mut ret: c_int = mt9m111_enable(mt9m111);
    if (!ret)
    ret = mt9m111_reset(mt9m111);
    if (!ret)
    mt9m111_restore_state(mt9m111);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_init(mt9m111: *mut mt9m111) -> c_int {
    static int mt9m111_init(struct mt9m111 *mt9m111)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    int ret;
    ret = mt9m111_enable(mt9m111);
    if (!ret)
    ret = mt9m111_reset(mt9m111);
    if (!ret)
    ret = mt9m111_set_context(mt9m111, mt9m111.ctx);
    if (ret)
    dev_err(&client.dev, "mt9m111 init failed: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_power_on(mt9m111: *mut mt9m111) -> c_int {
    static int mt9m111_power_on(struct mt9m111 *mt9m111)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9m111.subdev);
    int ret;
    ret = clk_prepare_enable(mt9m111.clk);
    if (ret < 0)
    return ret;
    ret = regulator_enable(mt9m111.regulator);
    if (ret < 0)
    goto out_clk_disable;
    ret = mt9m111_resume(mt9m111);
    if (ret < 0)
    goto out_regulator_disable;
    return 0;
    out_regulator_disable:
    regulator_disable(mt9m111.regulator);
    out_clk_disable:
    clk_disable_unprepare(mt9m111.clk);
    dev_err(&client.dev, "Failed to resume the sensor: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_power_off(mt9m111: *mut mt9m111) {
    static void mt9m111_power_off(struct mt9m111 *mt9m111)
    {
    mt9m111_suspend(mt9m111);
    regulator_disable(mt9m111.regulator);
    clk_disable_unprepare(mt9m111.clk);
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_s_power(sd: *mut v4l2_subdev, on: c_int) -> c_int {
    static int mt9m111_s_power(struct v4l2_subdev *sd, int on)
    {
    struct mt9m111 *mt9m111 = container_of(sd, struct mt9m111, subdev);
    let mut ret: c_int = 0;
    mutex_lock(&mt9m111.power_lock);
//
// If the power count is modified from 0 to != 0 or from != 0 to 0,
// update the power state.
//
    if (mt9m111.power_count == !on) {
    if (on)
    ret = mt9m111_power_on(mt9m111);
    else
    mt9m111_power_off(mt9m111);
    }
    if (!ret) {
// Update the power count.
    mt9m111.power_count += on ? 1 : -1;
    WARN_ON(mt9m111.power_count < 0);
    }
    mutex_unlock(&mt9m111.power_lock);
    return ret;
    }
    static const struct v4l2_ctrl_ops mt9m111_ctrl_ops = {
    .s_ctrl = mt9m111_s_ctrl,
    };
    static const struct v4l2_subdev_core_ops mt9m111_subdev_core_ops = {
    .s_power	= mt9m111_s_power,
    .log_status = v4l2_ctrl_subdev_log_status,
    .subscribe_event = v4l2_ctrl_subdev_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,

    .g_register	= mt9m111_g_register,
    .s_register	= mt9m111_s_register,

    };
    static int mt9m111_get_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *fi)
    {
    struct mt9m111 *mt9m111 = container_of(sd, struct mt9m111, subdev);
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (fi.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    fi.interval = mt9m111.frame_interval;
    return 0;
    }
    static int mt9m111_set_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *fi)
    {
    struct mt9m111 *mt9m111 = container_of(sd, struct mt9m111, subdev);
    const struct mt9m111_mode_info *mode;
    struct v4l2_fract *fract = &fi.interval;
    int fps;
    if (mt9m111.is_streaming)
    return -EBUSY;
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (fi.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    if (fi.pad != 0)
    return -EINVAL;
    if (fract.numerator == 0) {
    fract.denominator = 30;
    fract.numerator = 1;
    }
    fps = DIV_ROUND_CLOSEST(fract.denominator, fract.numerator);
// Find best fitting mode. Do not update the mode if no one was found.
    mode = mt9m111_find_mode(mt9m111, fps, mt9m111.width, mt9m111.height);
    if (!mode)
    return 0;
    if (mode.max_fps != fps) {
    fract.denominator = mode.max_fps;
    fract.numerator = 1;
    }
    mt9m111.current_mode = mode;
    mt9m111.frame_interval = fi.interval;
    return 0;
    }
    static int mt9m111_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index >= ARRAY_SIZE(mt9m111_colour_fmts))
    return -EINVAL;
    code.code = mt9m111_colour_fmts[code.index].code;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int mt9m111_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct mt9m111 *mt9m111 = container_of(sd, struct mt9m111, subdev);
    mt9m111.is_streaming = !!enable;
    return 0;
    }
    static int mt9m111_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state)
    {
    struct v4l2_mbus_framefmt *format =
    v4l2_subdev_state_get_format(sd_state, 0);
    format.width	= MT9M111_MAX_WIDTH;
    format.height	= MT9M111_MAX_HEIGHT;
    format.code	= mt9m111_colour_fmts[0].code;
    format.colorspace	= mt9m111_colour_fmts[0].colorspace;
    format.field	= V4L2_FIELD_NONE;
    format.ycbcr_enc	= V4L2_YCBCR_ENC_DEFAULT;
    format.quantization	= V4L2_QUANTIZATION_DEFAULT;
    format.xfer_func	= V4L2_XFER_FUNC_DEFAULT;
    return 0;
    }
    static int mt9m111_get_mbus_config(struct v4l2_subdev *sd,
    unsigned int pad,
    struct v4l2_mbus_config *cfg)
    {
    struct mt9m111 *mt9m111 = container_of(sd, struct mt9m111, subdev);
    cfg.type = V4L2_MBUS_PARALLEL;
    cfg.bus.parallel.flags = V4L2_MBUS_MASTER |
    V4L2_MBUS_HSYNC_ACTIVE_HIGH |
    V4L2_MBUS_VSYNC_ACTIVE_HIGH |
    V4L2_MBUS_DATA_ACTIVE_HIGH;
    cfg.bus.parallel.flags |= mt9m111.pclk_sample ?
    V4L2_MBUS_PCLK_SAMPLE_RISING :
    V4L2_MBUS_PCLK_SAMPLE_FALLING;
    return 0;
    }
    static const struct v4l2_subdev_video_ops mt9m111_subdev_video_ops = {
    .s_stream	= mt9m111_s_stream,
    };
    static const struct v4l2_subdev_pad_ops mt9m111_subdev_pad_ops = {
    .enum_mbus_code = mt9m111_enum_mbus_code,
    .get_selection	= mt9m111_get_selection,
    .set_selection	= mt9m111_set_selection,
    .get_fmt	= mt9m111_get_fmt,
    .set_fmt	= mt9m111_set_fmt,
    .get_frame_interval = mt9m111_get_frame_interval,
    .set_frame_interval = mt9m111_set_frame_interval,
    .get_mbus_config = mt9m111_get_mbus_config,
    };
    static const struct v4l2_subdev_ops mt9m111_subdev_ops = {
    .core	= &mt9m111_subdev_core_ops,
    .video	= &mt9m111_subdev_video_ops,
    .pad	= &mt9m111_subdev_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops mt9m111_internal_ops = {
    .init_state	= mt9m111_init_state,
    };
//
// Interface active, can use i2c. If it fails, it can indeed mean, that
// this wasn't our capture interface, so, we wait for the right one
//
#[no_mangle]
unsafe extern "C" fn mt9m111_video_probe(client: *mut i2c_client) -> c_int {
    static int mt9m111_video_probe(struct i2c_client *client)
    {
    struct mt9m111 *mt9m111 = to_mt9m111(client);
    s32 data;
    int ret;
    ret = mt9m111_s_power(&mt9m111.subdev, 1);
    if (ret < 0)
    return ret;
    data = reg_read(CHIP_VERSION);
    switch (data) {
    case 0x143a: /* MT9M111 or MT9M131 */
    dev_info(&client.dev,
    "Detected a MT9M111/MT9M131 chip ID %x\n", data);
    break;
    case 0x148c: /* MT9M112 */
    dev_info(&client.dev, "Detected a MT9M112 chip ID %x\n", data);
    break;
    default:
    dev_err(&client.dev,
    "No MT9M111/MT9M112/MT9M131 chip detected register read %x\n",
    data);
    ret = -ENODEV;
    goto done;
    }
    ret = mt9m111_init(mt9m111);
    if (ret)
    goto done;
    ret = v4l2_ctrl_handler_setup(&mt9m111.hdl);
    done:
    mt9m111_s_power(&mt9m111.subdev, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_probe_fw(client: *mut i2c_client, mt9m111: *mut mt9m111) -> c_int {
    static int mt9m111_probe_fw(struct i2c_client *client, struct mt9m111 *mt9m111)
    {
    struct v4l2_fwnode_endpoint bus_cfg = {
    .bus_type = V4L2_MBUS_PARALLEL
    };
    struct fwnode_handle *np;
    int ret;
    np = fwnode_graph_get_next_endpoint(dev_fwnode(&client.dev), core::ptr::null_mut());
    if (!np)
    return -EINVAL;
    ret = v4l2_fwnode_endpoint_parse(np, &bus_cfg);
    if (ret)
    goto out_put_fw;
    mt9m111.pclk_sample = !!(bus_cfg.bus.parallel.flags &
    V4L2_MBUS_PCLK_SAMPLE_RISING);
    out_put_fw:
    fwnode_handle_put(np);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_probe(client: *mut i2c_client) -> c_int {
    static int mt9m111_probe(struct i2c_client *client)
    {
    struct mt9m111 *mt9m111;
    struct i2c_adapter *adapter = client.adapter;
    int ret;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_WORD_DATA)) {
    dev_warn(&adapter.dev,
    "I2C-Adapter doesn't support I2C_FUNC_SMBUS_WORD\n");
    return -EIO;
    }
    mt9m111 = devm_kzalloc(&client.dev, sizeof(struct mt9m111), GFP_KERNEL);
    if (!mt9m111)
    return -ENOMEM;
    if (dev_fwnode(&client.dev)) {
    ret = mt9m111_probe_fw(client, mt9m111);
    if (ret)
    return ret;
    }
    mt9m111.clk = devm_v4l2_sensor_clk_get(&client.dev, "mclk");
    if (IS_ERR(mt9m111.clk))
    return dev_err_probe(&client.dev, PTR_ERR(mt9m111.clk),
    "failed to get mclk\n");
    mt9m111.regulator = devm_regulator_get(&client.dev, "vdd");
    if (IS_ERR(mt9m111.regulator)) {
    dev_err(&client.dev, "regulator not found: %pe\n",
    mt9m111.regulator);
    return PTR_ERR(mt9m111.regulator);
    }
// Default HIGHPOWER context
    mt9m111.ctx = &context_b;
    v4l2_i2c_subdev_init(&mt9m111.subdev, client, &mt9m111_subdev_ops);
    mt9m111.subdev.internal_ops = &mt9m111_internal_ops;
    mt9m111.subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE |
    V4L2_SUBDEV_FL_HAS_EVENTS;
    v4l2_ctrl_handler_init(&mt9m111.hdl, 7);
    v4l2_ctrl_new_std(&mt9m111.hdl, &mt9m111_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std(&mt9m111.hdl, &mt9m111_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std(&mt9m111.hdl, &mt9m111_ctrl_ops,
    V4L2_CID_AUTO_WHITE_BALANCE, 0, 1, 1, 1);
    mt9m111.gain = v4l2_ctrl_new_std(&mt9m111.hdl, &mt9m111_ctrl_ops,
    V4L2_CID_GAIN, 0, 63 * 2 * 2, 1, 32);
    v4l2_ctrl_new_std_menu(&mt9m111.hdl,
    &mt9m111_ctrl_ops, V4L2_CID_EXPOSURE_AUTO, 1, 0,
    V4L2_EXPOSURE_AUTO);
    v4l2_ctrl_new_std_menu_items(&mt9m111.hdl,
    &mt9m111_ctrl_ops, V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(mt9m111_test_pattern_menu) - 1, 0, 0,
    mt9m111_test_pattern_menu);
    v4l2_ctrl_new_std_menu(&mt9m111.hdl, &mt9m111_ctrl_ops,
    V4L2_CID_COLORFX, V4L2_COLORFX_SOLARIZATION,
    ~(BIT(V4L2_COLORFX_NONE) |
    BIT(V4L2_COLORFX_BW) |
    BIT(V4L2_COLORFX_SEPIA) |
    BIT(V4L2_COLORFX_NEGATIVE) |
    BIT(V4L2_COLORFX_SOLARIZATION)),
    V4L2_COLORFX_NONE);
    mt9m111.subdev.ctrl_handler = &mt9m111.hdl;
    if (mt9m111.hdl.error) {
    ret = mt9m111.hdl.error;
    return ret;
    }
    mt9m111.pad.flags = MEDIA_PAD_FL_SOURCE;
    mt9m111.subdev.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&mt9m111.subdev.entity, 1, &mt9m111.pad);
    if (ret < 0)
    goto out_hdlfree;
    mt9m111.current_mode = &mt9m111_mode_data[MT9M111_MODE_SXGA_15FPS];
    mt9m111.frame_interval.numerator = 1;
    mt9m111.frame_interval.denominator = mt9m111.current_mode.max_fps;
// Second stage probe - when a capture adapter is there
    mt9m111.rect.left	= MT9M111_MIN_DARK_COLS;
    mt9m111.rect.top	= MT9M111_MIN_DARK_ROWS;
    mt9m111.rect.width	= MT9M111_MAX_WIDTH;
    mt9m111.rect.height	= MT9M111_MAX_HEIGHT;
    mt9m111.width		= mt9m111.rect.width;
    mt9m111.height		= mt9m111.rect.height;
    mt9m111.fmt		= &mt9m111_colour_fmts[0];
    mt9m111.lastpage	= -1;
    mutex_init(&mt9m111.power_lock);
    ret = mt9m111_video_probe(client);
    if (ret < 0)
    goto out_entityclean;
    mt9m111.subdev.dev = &client.dev;
    ret = v4l2_async_register_subdev(&mt9m111.subdev);
    if (ret < 0)
    goto out_entityclean;
    return 0;
    out_entityclean:
    media_entity_cleanup(&mt9m111.subdev.entity);
    out_hdlfree:
    v4l2_ctrl_handler_free(&mt9m111.hdl);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m111_remove(client: *mut i2c_client) {
    static void mt9m111_remove(struct i2c_client *client)
    {
    struct mt9m111 *mt9m111 = to_mt9m111(client);
    v4l2_async_unregister_subdev(&mt9m111.subdev);
    media_entity_cleanup(&mt9m111.subdev.entity);
    v4l2_ctrl_handler_free(&mt9m111.hdl);
    }
    static const struct of_device_id mt9m111_of_match[] = {
    { .compatible = "micron,mt9m111", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mt9m111_of_match);
    static const struct i2c_device_id mt9m111_id[] = {
    { .name = "mt9m111" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mt9m111_id);
    static struct i2c_driver mt9m111_i2c_driver = {
    .driver = {
    .name = "mt9m111",
    .of_match_table = mt9m111_of_match,
    },
    .probe		= mt9m111_probe,
    .remove		= mt9m111_remove,
    .id_table	= mt9m111_id,
    };
    module_i2c_driver(mt9m111_i2c_driver);
    MODULE_DESCRIPTION("Micron/Aptina MT9M111/MT9M112/MT9M131 Camera driver");
    MODULE_AUTHOR("Robert Jarzmik");
    MODULE_LICENSE("GPL");
