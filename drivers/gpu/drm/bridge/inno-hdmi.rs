//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/inno-hdmi.c
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Zheng Yang <zhengyang@rock-chips.com>
// Yakir Yang <ykk@rock-chips.com>
// Andy Yan <andyshrk@163.com>
//

pub const DDC_SEGMENT_ADDR: c_uint = 0x30;

pub const DDC_BUS_FREQ_L: c_uint = 0x4b;
pub const DDC_BUS_FREQ_H: c_uint = 0x4c;
pub const HDMI_SYS_CTRL: c_uint = 0x00;

pub const v_INT_POL_HIGH: c_int = 1;
pub const v_INT_POL_LOW: c_int = 0;
pub const HDMI_VIDEO_CONTRL1: c_uint = 0x01;

pub const v_DE_EXTERNAL: c_int = 1;
pub const v_DE_INTERNAL: c_int = 0;
    enum {
    VIDEO_INPUT_SDR_RGB444 = 0,
    VIDEO_INPUT_DDR_RGB444 = 5,
    VIDEO_INPUT_DDR_YCBCR422 = 6
    };
pub const HDMI_VIDEO_CONTRL2: c_uint = 0x02;

    enum {
    VIDEO_INPUT_12BITS = 0,
    VIDEO_INPUT_10BITS = 1,
    VIDEO_INPUT_REVERT = 2,
    VIDEO_INPUT_8BITS = 3,
    };
pub const HDMI_VIDEO_CONTRL: c_uint = 0x03;

    enum {
    C0_C2_CHANGE_ENABLE = 0,
    C0_C2_CHANGE_DISABLE = 1,
    AUTO_CSC_DISABLE = 0,
    AUTO_CSC_ENABLE = 1,
    };
pub const HDMI_VIDEO_CONTRL3: c_uint = 0x04;

pub const v_CSC_ENABLE: c_int = 1;
pub const v_CSC_DISABLE: c_int = 0;
pub const HDMI_AV_MUTE: c_uint = 0x05;

pub const HDMI_VIDEO_TIMING_CTL: c_uint = 0x08;

pub const HDMI_VIDEO_EXT_HTOTAL_L: c_uint = 0x09;
pub const HDMI_VIDEO_EXT_HTOTAL_H: c_uint = 0x0a;
pub const HDMI_VIDEO_EXT_HBLANK_L: c_uint = 0x0b;
pub const HDMI_VIDEO_EXT_HBLANK_H: c_uint = 0x0c;
pub const HDMI_VIDEO_EXT_HDELAY_L: c_uint = 0x0d;
pub const HDMI_VIDEO_EXT_HDELAY_H: c_uint = 0x0e;
pub const HDMI_VIDEO_EXT_HDURATION_L: c_uint = 0x0f;
pub const HDMI_VIDEO_EXT_HDURATION_H: c_uint = 0x10;
pub const HDMI_VIDEO_EXT_VTOTAL_L: c_uint = 0x11;
pub const HDMI_VIDEO_EXT_VTOTAL_H: c_uint = 0x12;
pub const HDMI_VIDEO_EXT_VBLANK: c_uint = 0x13;
pub const HDMI_VIDEO_EXT_VDELAY: c_uint = 0x14;
pub const HDMI_VIDEO_EXT_VDURATION: c_uint = 0x15;
pub const HDMI_VIDEO_CSC_COEF: c_uint = 0x18;
pub const HDMI_AUDIO_CTRL1: c_uint = 0x35;
    enum {
    CTS_SOURCE_INTERNAL = 0,
    CTS_SOURCE_EXTERNAL = 1,
    };

    enum {
    DOWNSAMPLE_DISABLE = 0,
    DOWNSAMPLE_1_2 = 1,
    DOWNSAMPLE_1_4 = 2,
    };

    enum {
    AUDIO_SOURCE_IIS = 0,
    AUDIO_SOURCE_SPDIF = 1,
    };

    enum {
    MCLK_128FS = 0,
    MCLK_256FS = 1,
    MCLK_384FS = 2,
    MCLK_512FS = 3,
    };

pub const AUDIO_SAMPLE_RATE: c_uint = 0x37;
    enum {
    AUDIO_32K = 0x3,
    AUDIO_441K = 0x0,
    AUDIO_48K = 0x2,
    AUDIO_882K = 0x8,
    AUDIO_96K = 0xa,
    AUDIO_1764K = 0xc,
    AUDIO_192K = 0xe,
    };
pub const AUDIO_I2S_MODE: c_uint = 0x38;
    enum {
    I2S_CHANNEL_1_2 = 1,
    I2S_CHANNEL_3_4 = 3,
    I2S_CHANNEL_5_6 = 7,
    I2S_CHANNEL_7_8 = 0xf
    };

    enum {
    I2S_STANDARD = 0,
    I2S_LEFT_JUSTIFIED = 1,
    I2S_RIGHT_JUSTIFIED = 2,
    };

pub const AUDIO_I2S_MAP: c_uint = 0x39;
pub const AUDIO_I2S_SWAPS_SPDIF: c_uint = 0x3a;

pub const N_32K: c_uint = 0x1000;
pub const N_441K: c_uint = 0x1880;
pub const N_882K: c_uint = 0x3100;
pub const N_1764K: c_uint = 0x6200;
pub const N_48K: c_uint = 0x1800;
pub const N_96K: c_uint = 0x3000;
pub const N_192K: c_uint = 0x6000;
pub const HDMI_AUDIO_CHANNEL_STATUS: c_uint = 0x3e;

pub const AUDIO_N_H: c_uint = 0x3f;
pub const AUDIO_N_M: c_uint = 0x40;
pub const AUDIO_N_L: c_uint = 0x41;
pub const HDMI_AUDIO_CTS_H: c_uint = 0x45;
pub const HDMI_AUDIO_CTS_M: c_uint = 0x46;
pub const HDMI_AUDIO_CTS_L: c_uint = 0x47;
pub const HDMI_DDC_CLK_L: c_uint = 0x4b;
pub const HDMI_DDC_CLK_H: c_uint = 0x4c;
pub const HDMI_EDID_SEGMENT_POINTER: c_uint = 0x4d;
pub const HDMI_EDID_WORD_ADDR: c_uint = 0x4e;
pub const HDMI_EDID_FIFO_OFFSET: c_uint = 0x4f;
pub const HDMI_EDID_FIFO_ADDR: c_uint = 0x50;
pub const HDMI_PACKET_SEND_MANUAL: c_uint = 0x9c;
pub const HDMI_PACKET_SEND_AUTO: c_uint = 0x9d;

pub const HDMI_CONTROL_PACKET_BUF_INDEX: c_uint = 0x9f;
    enum {
    INFOFRAME_VSI = 0x05,
    INFOFRAME_AVI = 0x06,
    INFOFRAME_AAI = 0x08,
    };
pub const HDMI_CONTROL_PACKET_ADDR: c_uint = 0xa0;
pub const HDMI_MAXIMUM_INFO_FRAME_SIZE: c_uint = 0x11;
    enum {
    AVI_COLOR_MODE_RGB = 0,
    AVI_COLOR_MODE_YCBCR422 = 1,
    AVI_COLOR_MODE_YCBCR444 = 2,
    AVI_COLORIMETRY_NO_DATA = 0,
    AVI_COLORIMETRY_SMPTE_170M = 1,
    AVI_COLORIMETRY_ITU709 = 2,
    AVI_COLORIMETRY_EXTENDED = 3,
    AVI_CODED_FRAME_ASPECT_NO_DATA = 0,
    AVI_CODED_FRAME_ASPECT_4_3 = 1,
    AVI_CODED_FRAME_ASPECT_16_9 = 2,
    ACTIVE_ASPECT_RATE_SAME_AS_CODED_FRAME = 0x08,
    ACTIVE_ASPECT_RATE_4_3 = 0x09,
    ACTIVE_ASPECT_RATE_16_9 = 0x0A,
    ACTIVE_ASPECT_RATE_14_9 = 0x0B,
    };
pub const HDMI_HDCP_CTRL: c_uint = 0x52;

pub const HDMI_INTERRUPT_MASK1: c_uint = 0xc0;
pub const HDMI_INTERRUPT_STATUS1: c_uint = 0xc1;

pub const HDMI_INTERRUPT_MASK2: c_uint = 0xc2;
pub const HDMI_INTERRUPT_STATUS2: c_uint = 0xc3;

pub const HDMI_STATUS: c_uint = 0xc8;

pub const HDMI_COLORBAR: c_uint = 0xc9;
pub const HDMI_PHY_SYNC: c_uint = 0xce;
pub const HDMI_PHY_SYS_CTL: c_uint = 0xe0;

pub const HDMI_PHY_CHG_PWR: c_uint = 0xe1;

pub const HDMI_PHY_DRIVER: c_uint = 0xe2;

pub const HDMI_PHY_PRE_EMPHASIS: c_uint = 0xe3;

pub const HDMI_PHY_FEEDBACK_DIV_RATIO_LOW: c_uint = 0xe7;

pub const HDMI_PHY_FEEDBACK_DIV_RATIO_HIGH: c_uint = 0xe8;

pub const HDMI_PHY_PRE_DIV_RATIO: c_uint = 0xed;

pub const HDMI_CEC_CTRL: c_uint = 0xd0;

pub const HDMI_CEC_DATA: c_uint = 0xd1;
pub const HDMI_CEC_TX_OFFSET: c_uint = 0xd2;
pub const HDMI_CEC_RX_OFFSET: c_uint = 0xd3;
pub const HDMI_CEC_CLK_H: c_uint = 0xd4;
pub const HDMI_CEC_CLK_L: c_uint = 0xd5;
pub const HDMI_CEC_TX_LENGTH: c_uint = 0xd6;
pub const HDMI_CEC_RX_LENGTH: c_uint = 0xd7;
pub const HDMI_CEC_TX_INT_MASK: c_uint = 0xd8;

pub const HDMI_CEC_RX_INT_MASK: c_uint = 0xd9;

pub const HDMI_CEC_TX_INT: c_uint = 0xda;
pub const HDMI_CEC_RX_INT: c_uint = 0xdb;
pub const HDMI_CEC_BUSFREETIME_L: c_uint = 0xdc;
pub const HDMI_CEC_BUSFREETIME_H: c_uint = 0xdd;
pub const HDMI_CEC_LOGICADDR: c_uint = 0xde;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inno_hdmi_i2c {
    pub adap: i2c_adapter,
    pub ddc_addr: u8,
    pub segment_addr: u8,
    pub lock: mutex,
    pub cmp: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inno_hdmi {
    pub dev: *mut device,
    pub bridge: drm_bridge,
    pub pclk: *mut clk,
    pub refclk: *mut clk,
    pub regs: *mut void __iomem,
    pub grf: *mut regmap,
    pub i2c: *mut inno_hdmi_i2c,
    pub ddc: *mut i2c_adapter,
    pub plat_data: *const inno_hdmi_plat_data,
}

    enum {
    CSC_RGB_0_255_TO_ITU601_16_235_8BIT,
    CSC_RGB_0_255_TO_ITU709_16_235_8BIT,
    CSC_RGB_0_255_TO_RGB_16_235_8BIT,
    };
    static const char coeff_csc[][24] = {
//
// RGB2YUV:601 SD mode:
// Cb = -0.291G - 0.148R + 0.439B + 128
// Y  = 0.504G  + 0.257R + 0.098B + 16
// Cr = -0.368G + 0.439R - 0.071B + 128
//
    {
    0x11, 0x5f, 0x01, 0x82, 0x10, 0x23, 0x00, 0x80,
    0x02, 0x1c, 0x00, 0xa1, 0x00, 0x36, 0x00, 0x1e,
    0x11, 0x29, 0x10, 0x59, 0x01, 0x82, 0x00, 0x80
    },
//
// RGB2YUV:709 HD mode:
// Cb = - 0.338G - 0.101R + 0.439B + 128
// Y  = 0.614G   + 0.183R + 0.062B + 16
// Cr = - 0.399G + 0.439R - 0.040B + 128
//
    {
    0x11, 0x98, 0x01, 0xc1, 0x10, 0x28, 0x00, 0x80,
    0x02, 0x74, 0x00, 0xbb, 0x00, 0x3f, 0x00, 0x10,
    0x11, 0x5a, 0x10, 0x67, 0x01, 0xc1, 0x00, 0x80
    },
//
// RGB[0:255]2RGB[16:235]:
// R' = R x (235-16)/255 + 16;
// G' = G x (235-16)/255 + 16;
// B' = B x (235-16)/255 + 16;
//
    {
    0x00, 0x00, 0x03, 0x6F, 0x00, 0x00, 0x00, 0x10,
    0x03, 0x6F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
    0x00, 0x00, 0x00, 0x00, 0x03, 0x6F, 0x00, 0x10
    },
    };
    static struct inno_hdmi *bridge_to_inno_hdmi(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct inno_hdmi, bridge);
    }
    static int inno_hdmi_find_phy_config(struct inno_hdmi *hdmi,
    unsigned long pixelclk)
    {
    const struct inno_hdmi_phy_config *phy_configs = hdmi.plat_data.phy_configs;
    int i;
    for (i = 0; phy_configs[i].pixelclock != ~0UL; i++) {
    if (pixelclk <= phy_configs[i].pixelclock)
    return i;
    }
    DRM_DEV_DEBUG(hdmi.dev, "No phy configuration for pixelclock %lu\n",
    pixelclk);
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn hdmi_readb(hdmi: *mut inno_hdmi, offset: u16) -> u8 {
    static inline u8 hdmi_readb(struct inno_hdmi *hdmi, u16 offset)
    {
    return readl_relaxed(hdmi.regs + (offset) * 0x04);
    }
#[no_mangle]
pub unsafe extern "C" fn hdmi_writeb(hdmi: *mut inno_hdmi, offset: u16, val: u32) {
    static inline void hdmi_writeb(struct inno_hdmi *hdmi, u16 offset, u32 val)
    {
    writel_relaxed(val, hdmi.regs + (offset) * 0x04);
    }
    static inline void hdmi_modb(struct inno_hdmi *hdmi, u16 offset,
    u32 msk, u32 val)
    {
    let mut temp: u8 = hdmi_readb(hdmi, offset) & ~msk;
    temp |= val & msk;
    hdmi_writeb(hdmi, offset, temp);
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_i2c_init(hdmi: *mut inno_hdmi, rate: c_ulonglong) {
    static void inno_hdmi_i2c_init(struct inno_hdmi *hdmi, unsigned long long rate)
    {
    let mut ddc_bus_freq: c_ulonglong = rate >> 2;
    do_div(ddc_bus_freq, HDMI_SCL_RATE);
    hdmi_writeb(hdmi, DDC_BUS_FREQ_L, ddc_bus_freq & 0xFF);
    hdmi_writeb(hdmi, DDC_BUS_FREQ_H, (ddc_bus_freq >> 8) & 0xFF);
// Clear the EDID interrupt flag and mute the interrupt
    hdmi_writeb(hdmi, HDMI_INTERRUPT_MASK1, 0);
    hdmi_writeb(hdmi, HDMI_INTERRUPT_STATUS1, m_INT_EDID_READY);
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_sys_power(hdmi: *mut inno_hdmi, enable: bool) {
    static void inno_hdmi_sys_power(struct inno_hdmi *hdmi, bool enable)
    {
    if (enable)
    hdmi_modb(hdmi, HDMI_SYS_CTRL, m_POWER, v_PWR_ON);
    else
    hdmi_modb(hdmi, HDMI_SYS_CTRL, m_POWER, v_PWR_OFF);
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_standby(hdmi: *mut inno_hdmi) {
    static void inno_hdmi_standby(struct inno_hdmi *hdmi)
    {
    inno_hdmi_sys_power(hdmi, false);
    hdmi_writeb(hdmi, HDMI_PHY_DRIVER, 0x00);
    hdmi_writeb(hdmi, HDMI_PHY_PRE_EMPHASIS, 0x00);
    hdmi_writeb(hdmi, HDMI_PHY_CHG_PWR, 0x00);
    hdmi_writeb(hdmi, HDMI_PHY_SYS_CTL, 0x15);
    };
    static void inno_hdmi_power_up(struct inno_hdmi *hdmi,
    unsigned long mpixelclock)
    {
    struct inno_hdmi_phy_config *phy_config;
    let mut ret: c_int = inno_hdmi_find_phy_config(hdmi, mpixelclock);
    if (ret < 0) {
    phy_config = hdmi.plat_data.default_phy_config;
    DRM_DEV_ERROR(hdmi.dev,
    "Using default phy configuration for TMDS rate %lu",
    mpixelclock);
    } else {
    phy_config = &hdmi.plat_data.phy_configs[ret];
    }
    inno_hdmi_sys_power(hdmi, false);
    hdmi_writeb(hdmi, HDMI_PHY_PRE_EMPHASIS, phy_config.pre_emphasis);
    hdmi_writeb(hdmi, HDMI_PHY_DRIVER, phy_config.voltage_level_control);
    hdmi_writeb(hdmi, HDMI_PHY_SYS_CTL, 0x15);
    hdmi_writeb(hdmi, HDMI_PHY_SYS_CTL, 0x14);
    hdmi_writeb(hdmi, HDMI_PHY_SYS_CTL, 0x10);
    hdmi_writeb(hdmi, HDMI_PHY_CHG_PWR, 0x0f);
    hdmi_writeb(hdmi, HDMI_PHY_SYNC, 0x00);
    hdmi_writeb(hdmi, HDMI_PHY_SYNC, 0x01);
    inno_hdmi_sys_power(hdmi, true);
    };
#[no_mangle]
unsafe extern "C" fn inno_hdmi_init_hw(hdmi: *mut inno_hdmi) {
    static void inno_hdmi_init_hw(struct inno_hdmi *hdmi)
    {
    u32 val;
    u32 msk;
    hdmi_modb(hdmi, HDMI_SYS_CTRL, m_RST_DIGITAL, v_NOT_RST_DIGITAL);
    usleep_range(100, 150);
    hdmi_modb(hdmi, HDMI_SYS_CTRL, m_RST_ANALOG, v_NOT_RST_ANALOG);
    usleep_range(100, 150);
    msk = m_REG_CLK_INV | m_REG_CLK_SOURCE | m_POWER | m_INT_POL;
    val = v_REG_CLK_INV | v_REG_CLK_SOURCE_SYS | v_PWR_ON | v_INT_POL_HIGH;
    hdmi_modb(hdmi, HDMI_SYS_CTRL, msk, val);
    inno_hdmi_standby(hdmi);
//
// When the controller isn't configured to an accurate
// video timing and there is no reference clock available,
// then the TMDS clock source would be switched to PCLK_HDMI,
// so we need to init the TMDS rate to PCLK rate, and
// reconfigure the DDC clock.
//
    if (hdmi.refclk)
    inno_hdmi_i2c_init(hdmi, clk_get_rate(hdmi.refclk));
    else
    inno_hdmi_i2c_init(hdmi, clk_get_rate(hdmi.pclk));
// Unmute hotplug interrupt
    hdmi_modb(hdmi, HDMI_STATUS, m_MASK_INT_HOTPLUG, v_MASK_INT_HOTPLUG(1));
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_bridge_clear_avi_infoframe(bridge: *mut drm_bridge) -> c_int {
    static int inno_hdmi_bridge_clear_avi_infoframe(struct drm_bridge *bridge)
    {
    struct inno_hdmi *hdmi = bridge_to_inno_hdmi(bridge);
    hdmi_writeb(hdmi, HDMI_CONTROL_PACKET_BUF_INDEX, INFOFRAME_AVI);
    return 0;
    }
    static int inno_hdmi_bridge_write_avi_infoframe(struct drm_bridge *bridge,
    const u8 *buffer, size_t len)
    {
    struct inno_hdmi *hdmi = bridge_to_inno_hdmi(bridge);
    ssize_t i;
    inno_hdmi_bridge_clear_avi_infoframe(bridge);
    for (i = 0; i < len; i++)
    hdmi_writeb(hdmi, HDMI_CONTROL_PACKET_ADDR + i, buffer[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_bridge_clear_hdmi_infoframe(bridge: *mut drm_bridge) -> c_int {
    static int inno_hdmi_bridge_clear_hdmi_infoframe(struct drm_bridge *bridge)
    {
    drm_warn_once(bridge.encoder.dev, "HDMI VSI not implemented\n");
    return 0;
    }
    static int inno_hdmi_bridge_write_hdmi_infoframe(struct drm_bridge *bridge,
    const u8 *buffer, size_t len)
    {
    drm_warn_once(bridge.encoder.dev, "HDMI VSI not implemented\n");
    return 0;
    }
    static int inno_hdmi_config_video_csc(struct inno_hdmi *hdmi,
    struct drm_connector *connector,
    struct drm_display_mode *mode)
    {
    struct drm_connector_state *conn_state = connector.state;
    let mut c0_c2_change: c_int = 0;
    let mut csc_enable: c_int = 0;
    let mut csc_mode: c_int = 0;
    let mut auto_csc: c_int = 0;
    int value;
    int i;
    int colorimetry;
    let mut vic: u8 = drm_match_cea_mode(mode);
    if (vic == 6 || vic == 7 || vic == 21 || vic == 22 ||
    vic == 2 || vic == 3 || vic == 17 || vic == 18)
    colorimetry = HDMI_COLORIMETRY_ITU_601;
    else
    colorimetry = HDMI_COLORIMETRY_ITU_709;
// Input video mode is SDR RGB24bit, data enable signal from external
    hdmi_writeb(hdmi, HDMI_VIDEO_CONTRL1, v_DE_EXTERNAL |
    v_VIDEO_INPUT_FORMAT(VIDEO_INPUT_SDR_RGB444));
// Input color hardcode to RGB, and output color hardcode to RGB888
    value = v_VIDEO_INPUT_BITS(VIDEO_INPUT_8BITS) |
    v_VIDEO_OUTPUT_COLOR(0) |
    v_VIDEO_INPUT_CSP(0);
    hdmi_writeb(hdmi, HDMI_VIDEO_CONTRL2, value);
    if (conn_state.hdmi.output_format == DRM_OUTPUT_COLOR_FORMAT_RGB444) {
    if (conn_state.hdmi.is_limited_range) {
    csc_mode = CSC_RGB_0_255_TO_RGB_16_235_8BIT;
    auto_csc = AUTO_CSC_DISABLE;
    c0_c2_change = C0_C2_CHANGE_DISABLE;
    csc_enable = v_CSC_ENABLE;
    } else {
    value = v_SOF_DISABLE | v_COLOR_DEPTH_NOT_INDICATED(1);
    hdmi_writeb(hdmi, HDMI_VIDEO_CONTRL3, value);
    hdmi_modb(hdmi, HDMI_VIDEO_CONTRL,
    m_VIDEO_AUTO_CSC | m_VIDEO_C0_C2_SWAP,
    v_VIDEO_AUTO_CSC(AUTO_CSC_DISABLE) |
    v_VIDEO_C0_C2_SWAP(C0_C2_CHANGE_DISABLE));
    return 0;
    }
    } else {
    if (colorimetry == HDMI_COLORIMETRY_ITU_601) {
    if (conn_state.hdmi.output_format == DRM_OUTPUT_COLOR_FORMAT_YCBCR444) {
    csc_mode = CSC_RGB_0_255_TO_ITU601_16_235_8BIT;
    auto_csc = AUTO_CSC_DISABLE;
    c0_c2_change = C0_C2_CHANGE_DISABLE;
    csc_enable = v_CSC_ENABLE;
    }
    } else {
    if (conn_state.hdmi.output_format == DRM_OUTPUT_COLOR_FORMAT_YCBCR444) {
    csc_mode = CSC_RGB_0_255_TO_ITU709_16_235_8BIT;
    auto_csc = AUTO_CSC_DISABLE;
    c0_c2_change = C0_C2_CHANGE_DISABLE;
    csc_enable = v_CSC_ENABLE;
    }
    }
    }
    for (i = 0; i < 24; i++)
    hdmi_writeb(hdmi, HDMI_VIDEO_CSC_COEF + i, coeff_csc[csc_mode][i]);
    value = v_SOF_DISABLE | csc_enable | v_COLOR_DEPTH_NOT_INDICATED(1);
    hdmi_writeb(hdmi, HDMI_VIDEO_CONTRL3, value);
    hdmi_modb(hdmi, HDMI_VIDEO_CONTRL, m_VIDEO_AUTO_CSC |
    m_VIDEO_C0_C2_SWAP, v_VIDEO_AUTO_CSC(auto_csc) |
    v_VIDEO_C0_C2_SWAP(c0_c2_change));
    return 0;
    }
    static int inno_hdmi_config_video_timing(struct inno_hdmi *hdmi,
    struct drm_display_mode *mode)
    {
    const struct inno_hdmi_plat_ops *plat_ops = hdmi.plat_data.ops;
    u32 value;
    if (plat_ops && plat_ops.enable)
    plat_ops.enable(hdmi.dev, mode);
// Set detail external video timing polarity and interlace mode
    value = v_EXTERANL_VIDEO(1);
    value |= mode.flags & DRM_MODE_FLAG_PHSYNC ?
    v_HSYNC_POLARITY(1) : v_HSYNC_POLARITY(0);
    value |= mode.flags & DRM_MODE_FLAG_PVSYNC ?
    v_VSYNC_POLARITY(1) : v_VSYNC_POLARITY(0);
    value |= mode.flags & DRM_MODE_FLAG_INTERLACE ?
    v_INETLACE(1) : v_INETLACE(0);
    hdmi_writeb(hdmi, HDMI_VIDEO_TIMING_CTL, value);
// Set detail external video timing
    value = mode.htotal;
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_HTOTAL_L, value & 0xFF);
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_HTOTAL_H, (value >> 8) & 0xFF);
    value = mode.htotal - mode.hdisplay;
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_HBLANK_L, value & 0xFF);
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_HBLANK_H, (value >> 8) & 0xFF);
    value = mode.htotal - mode.hsync_start;
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_HDELAY_L, value & 0xFF);
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_HDELAY_H, (value >> 8) & 0xFF);
    value = mode.hsync_end - mode.hsync_start;
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_HDURATION_L, value & 0xFF);
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_HDURATION_H, (value >> 8) & 0xFF);
    value = mode.vtotal;
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_VTOTAL_L, value & 0xFF);
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_VTOTAL_H, (value >> 8) & 0xFF);
    value = mode.vtotal - mode.vdisplay;
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_VBLANK, value & 0xFF);
    value = mode.vtotal - mode.vsync_start;
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_VDELAY, value & 0xFF);
    value = mode.vsync_end - mode.vsync_start;
    hdmi_writeb(hdmi, HDMI_VIDEO_EXT_VDURATION, value & 0xFF);
    hdmi_writeb(hdmi, HDMI_PHY_PRE_DIV_RATIO, 0x1e);
    hdmi_writeb(hdmi, HDMI_PHY_FEEDBACK_DIV_RATIO_LOW, 0x2c);
    hdmi_writeb(hdmi, HDMI_PHY_FEEDBACK_DIV_RATIO_HIGH, 0x01);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_setup(hdmi: *mut inno_hdmi, state: *mut drm_atomic_commit) -> c_int {
    static int inno_hdmi_setup(struct inno_hdmi *hdmi, struct drm_atomic_commit *state)
    {
    struct drm_bridge *bridge = &hdmi.bridge;
    struct drm_connector *connector;
    struct drm_display_info *info;
    struct drm_connector_state *new_conn_state;
    struct drm_crtc_state *new_crtc_state;
    connector = drm_atomic_get_new_connector_for_encoder(state, bridge.encoder);
    new_conn_state = drm_atomic_get_new_connector_state(state, connector);
    if (WARN_ON(!new_conn_state))
    return -EINVAL;
    new_crtc_state = drm_atomic_get_new_crtc_state(state, new_conn_state.crtc);
    if (WARN_ON(!new_crtc_state))
    return -EINVAL;
    info = &connector.display_info;
// Mute video and audio output
    hdmi_modb(hdmi, HDMI_AV_MUTE, m_AUDIO_MUTE | m_VIDEO_BLACK,
    v_AUDIO_MUTE(1) | v_VIDEO_MUTE(1));
// Set HDMI Mode
    hdmi_writeb(hdmi, HDMI_HDCP_CTRL, v_HDMI_DVI(info.is_hdmi));
    inno_hdmi_config_video_timing(hdmi, &new_crtc_state.adjusted_mode);
    inno_hdmi_config_video_csc(hdmi, connector, &new_crtc_state.adjusted_mode);
    drm_atomic_helper_connector_hdmi_update_infoframes(connector, state);
//
// When IP controller have configured to an accurate video
// timing, then the TMDS clock source would be switched to
// DCLK_LCDC, so we need to init the TMDS rate to mode pixel
// clock rate, and reconfigure the DDC clock.
//
    inno_hdmi_i2c_init(hdmi, new_conn_state.hdmi.tmds_char_rate);
// Unmute video and audio output
    hdmi_modb(hdmi, HDMI_AV_MUTE, m_AUDIO_MUTE | m_VIDEO_BLACK,
    v_AUDIO_MUTE(0) | v_VIDEO_MUTE(0));
    inno_hdmi_power_up(hdmi, new_conn_state.hdmi.tmds_char_rate);
    return 0;
    }
    static enum drm_mode_status inno_hdmi_bridge_mode_valid(struct drm_bridge *bridge,
    const struct drm_display_info *info,
    const struct drm_display_mode *mode)
    {
    struct inno_hdmi *hdmi = bridge_to_inno_hdmi(bridge);
    unsigned long mpixelclk, max_tolerance;
    long rounded_refclk;
// No support for double-clock modes
    if (mode.flags & DRM_MODE_FLAG_DBLCLK)
    return MODE_BAD;
    mpixelclk = mode.clock * 1000;
    if (mpixelclk < HDMI_TMDS_CHAR_RATE_MIN_HZ)
    return MODE_CLOCK_LOW;
    if (inno_hdmi_find_phy_config(hdmi, mpixelclk) < 0)
    return MODE_CLOCK_HIGH;
    if (hdmi.refclk) {
    rounded_refclk = clk_round_rate(hdmi.refclk, mpixelclk);
    if (rounded_refclk < 0)
    return MODE_BAD;
// Vesa DMT standard mentions +/- 0.5% max tolerance
    max_tolerance = mpixelclk / 200;
    if (abs_diff((unsigned long)rounded_refclk, mpixelclk) > max_tolerance)
    return MODE_NOCLOCK;
    }
    return MODE_OK;
    }
    static enum drm_connector_status
    inno_hdmi_bridge_detect(struct drm_bridge *bridge, struct drm_connector *connector)
    {
    struct inno_hdmi *hdmi = bridge_to_inno_hdmi(bridge);
    return (hdmi_readb(hdmi, HDMI_STATUS) & m_HOTPLUG) ?
    connector_status_connected : connector_status_disconnected;
    }
    static const struct drm_edid *
    inno_hdmi_bridge_edid_read(struct drm_bridge *bridge, struct drm_connector *connector)
    {
    struct inno_hdmi *hdmi = bridge_to_inno_hdmi(bridge);
    const struct drm_edid *drm_edid;
    drm_edid = drm_edid_read_ddc(connector, bridge.ddc);
    if (!drm_edid)
    dev_dbg(hdmi.dev, "failed to get edid\n");
    return drm_edid;
    }
    static void inno_hdmi_bridge_atomic_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct inno_hdmi *hdmi = bridge_to_inno_hdmi(bridge);
    inno_hdmi_setup(hdmi, state);
    }
    static void inno_hdmi_bridge_atomic_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct inno_hdmi *hdmi = bridge_to_inno_hdmi(bridge);
    inno_hdmi_standby(hdmi);
    }
    static const struct drm_bridge_funcs inno_hdmi_bridge_funcs = {
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_enable = inno_hdmi_bridge_atomic_enable,
    .atomic_disable = inno_hdmi_bridge_atomic_disable,
    .detect = inno_hdmi_bridge_detect,
    .edid_read = inno_hdmi_bridge_edid_read,
    .hdmi_clear_avi_infoframe = inno_hdmi_bridge_clear_avi_infoframe,
    .hdmi_write_avi_infoframe = inno_hdmi_bridge_write_avi_infoframe,
    .hdmi_clear_hdmi_infoframe = inno_hdmi_bridge_clear_hdmi_infoframe,
    .hdmi_write_hdmi_infoframe = inno_hdmi_bridge_write_hdmi_infoframe,
    .mode_valid = inno_hdmi_bridge_mode_valid,
    };
#[no_mangle]
unsafe extern "C" fn inno_hdmi_i2c_irq(hdmi: *mut inno_hdmi) -> irqreturn_t {
    static irqreturn_t inno_hdmi_i2c_irq(struct inno_hdmi *hdmi)
    {
    struct inno_hdmi_i2c *i2c = hdmi.i2c;
    u8 stat;
    stat = hdmi_readb(hdmi, HDMI_INTERRUPT_STATUS1);
    if (!(stat & m_INT_EDID_READY))
    return IRQ_NONE;
// Clear HDMI EDID interrupt flag
    hdmi_writeb(hdmi, HDMI_INTERRUPT_STATUS1, m_INT_EDID_READY);
    complete(&i2c.cmp);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_hardirq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t inno_hdmi_hardirq(int irq, void *dev_id)
    {
    struct inno_hdmi *hdmi = dev_id;
    let mut ret: irqreturn_t = IRQ_NONE;
    u8 interrupt;
    if (hdmi.i2c)
    ret = inno_hdmi_i2c_irq(hdmi);
    interrupt = hdmi_readb(hdmi, HDMI_STATUS);
    if (interrupt & m_INT_HOTPLUG) {
    hdmi_modb(hdmi, HDMI_STATUS, m_INT_HOTPLUG, m_INT_HOTPLUG);
    ret = IRQ_WAKE_THREAD;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t inno_hdmi_irq(int irq, void *dev_id)
    {
    struct inno_hdmi *hdmi = dev_id;
    drm_helper_hpd_irq_event(hdmi.bridge.dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_i2c_read(hdmi: *mut inno_hdmi, msgs: *mut i2c_msg) -> c_int {
    static int inno_hdmi_i2c_read(struct inno_hdmi *hdmi, struct i2c_msg *msgs)
    {
    let mut length: c_int = msgs.len;
    u8 *buf = msgs.buf;
    int ret;
    ret = wait_for_completion_timeout(&hdmi.i2c.cmp, HZ / 10);
    if (!ret)
    return -EAGAIN;
    while (length--)
// buf++ = hdmi_readb(hdmi, HDMI_EDID_FIFO_ADDR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_i2c_write(hdmi: *mut inno_hdmi, msgs: *mut i2c_msg) -> c_int {
    static int inno_hdmi_i2c_write(struct inno_hdmi *hdmi, struct i2c_msg *msgs)
    {
//
// The DDC module only support read EDID message, so
// we assume that each word write to this i2c adapter
// should be the offset of EDID word address.
//
    if (msgs.len != 1 || (msgs.addr != DDC_ADDR && msgs.addr != DDC_SEGMENT_ADDR))
    return -EINVAL;
    reinit_completion(&hdmi.i2c.cmp);
    if (msgs.addr == DDC_SEGMENT_ADDR)
    hdmi.i2c.segment_addr = msgs.buf[0];
    if (msgs.addr == DDC_ADDR)
    hdmi.i2c.ddc_addr = msgs.buf[0];
// Set edid fifo first addr
    hdmi_writeb(hdmi, HDMI_EDID_FIFO_OFFSET, 0x00);
// Set edid word address 0x00/0x80
    hdmi_writeb(hdmi, HDMI_EDID_WORD_ADDR, hdmi.i2c.ddc_addr);
// Set edid segment pointer
    hdmi_writeb(hdmi, HDMI_EDID_SEGMENT_POINTER, hdmi.i2c.segment_addr);
    return 0;
    }
    static int inno_hdmi_i2c_xfer(struct i2c_adapter *adap,
    struct i2c_msg *msgs, int num)
    {
    struct inno_hdmi *hdmi = i2c_get_adapdata(adap);
    struct inno_hdmi_i2c *i2c = hdmi.i2c;
    int i, ret = 0;
    mutex_lock(&i2c.lock);
// Clear the EDID interrupt flag and unmute the interrupt
    hdmi_writeb(hdmi, HDMI_INTERRUPT_MASK1, m_INT_EDID_READY);
    hdmi_writeb(hdmi, HDMI_INTERRUPT_STATUS1, m_INT_EDID_READY);
    for (i = 0; i < num; i++) {
    DRM_DEV_DEBUG(hdmi.dev,
    "xfer: num: %d/%d, len: %d, flags: %#x\n",
    i + 1, num, msgs[i].len, msgs[i].flags);
    if (msgs[i].flags & I2C_M_RD)
    ret = inno_hdmi_i2c_read(hdmi, &msgs[i]);
    else
    ret = inno_hdmi_i2c_write(hdmi, &msgs[i]);
    if (ret < 0)
    break;
    }
    if (!ret)
    ret = num;
// Mute HDMI EDID interrupt
    hdmi_writeb(hdmi, HDMI_INTERRUPT_MASK1, 0);
    mutex_unlock(&i2c.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_i2c_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 inno_hdmi_i2c_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm inno_hdmi_algorithm = {
    .master_xfer	= inno_hdmi_i2c_xfer,
    .functionality	= inno_hdmi_i2c_func,
    };
    static struct i2c_adapter *inno_hdmi_i2c_adapter(struct inno_hdmi *hdmi)
    {
    struct i2c_adapter *adap;
    struct inno_hdmi_i2c *i2c;
    int ret;
    i2c = devm_kzalloc(hdmi.dev, sizeof(*i2c), GFP_KERNEL);
    if (!i2c)
    return ERR_PTR(-ENOMEM);
    mutex_init(&i2c.lock);
    init_completion(&i2c.cmp);
    adap = &i2c.adap;
    adap.owner = THIS_MODULE;
    adap.dev.parent = hdmi.dev;
    adap.dev.of_node = hdmi.dev.of_node;
    adap.algo = &inno_hdmi_algorithm;
    strscpy(adap.name, "Inno HDMI", sizeof(adap.name));
    i2c_set_adapdata(adap, hdmi);
    ret = devm_i2c_add_adapter(hdmi.dev, adap);
    if (ret) {
    dev_warn(hdmi.dev, "cannot add %s I2C adapter\n", adap.name);
    return ERR_PTR(ret);
    }
    hdmi.i2c = i2c;
    DRM_DEV_INFO(hdmi.dev, "registered %s I2C bus driver\n", adap.name);
    return adap;
    }
    struct inno_hdmi *inno_hdmi_bind(struct device *dev,
    struct drm_encoder *encoder,
    const struct inno_hdmi_plat_data *plat_data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct inno_hdmi *hdmi;
    int irq;
    int ret;
    if (!plat_data.phy_configs || !plat_data.default_phy_config) {
    dev_err(dev, "Missing platform PHY ops\n");
    return ERR_PTR(-ENODEV);
    }
    hdmi = devm_drm_bridge_alloc(dev, struct inno_hdmi, bridge, &inno_hdmi_bridge_funcs);
    if (IS_ERR(hdmi))
    return ERR_CAST(hdmi);
    hdmi.dev = dev;
    hdmi.plat_data = plat_data;
    hdmi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hdmi.regs))
    return ERR_CAST(hdmi.regs);
    hdmi.pclk = devm_clk_get_enabled(hdmi.dev, "pclk");
    if (IS_ERR(hdmi.pclk)) {
    dev_err_probe(dev, PTR_ERR(hdmi.pclk), "Unable to get HDMI pclk\n");
    return ERR_CAST(hdmi.pclk);
    }
    hdmi.refclk = devm_clk_get_optional_enabled(hdmi.dev, "ref");
    if (IS_ERR(hdmi.refclk)) {
    dev_err_probe(dev, PTR_ERR(hdmi.refclk), "Unable to get HDMI refclk\n");
    return ERR_CAST(hdmi.refclk);
    }
    inno_hdmi_init_hw(hdmi);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return ERR_PTR(irq);
    ret = devm_request_threaded_irq(dev, irq, inno_hdmi_hardirq,
    inno_hdmi_irq, IRQF_SHARED,
    dev_name(dev), hdmi);
    if (ret)
    return ERR_PTR(ret);
    hdmi.bridge.driver_private = hdmi;
    hdmi.bridge.ops = DRM_BRIDGE_OP_DETECT |
    DRM_BRIDGE_OP_EDID |
    DRM_BRIDGE_OP_HDMI |
    DRM_BRIDGE_OP_HPD;
    hdmi.bridge.of_node = pdev.dev.of_node;
    hdmi.bridge.type = DRM_MODE_CONNECTOR_HDMIA;
    hdmi.bridge.vendor = "Inno";
    hdmi.bridge.product = "Inno HDMI";
    hdmi.bridge.ddc = inno_hdmi_i2c_adapter(hdmi);
    if (IS_ERR(hdmi.bridge.ddc))
    return ERR_CAST(hdmi.bridge.ddc);
    ret = devm_drm_bridge_add(dev, &hdmi.bridge);
    if (ret)
    return ERR_PTR(ret);
    ret = drm_bridge_attach(encoder, &hdmi.bridge, core::ptr::null_mut(), DRM_BRIDGE_ATTACH_NO_CONNECTOR);
    if (ret)
    return ERR_PTR(ret);
    return hdmi;
    }
    EXPORT_SYMBOL_GPL(inno_hdmi_bind);
    MODULE_AUTHOR("Andy Yan <andyshrk@163.com>");
    MODULE_DESCRIPTION("INNOSILICON HDMI transmitter library");
    MODULE_LICENSE("GPL");
