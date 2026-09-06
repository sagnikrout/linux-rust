//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/ite-it6263.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2024 NXP
//

// -----------------------------------------------------------------------------
// LVDS registers
//
// LVDS software reset registers
pub const LVDS_REG_05: c_uint = 0x05;

// LVDS system configuration registers
// 0x0b
pub const LVDS_REG_0B: c_uint = 0x0b;

// LVDS test pattern gen control registers
// 0x2c
pub const LVDS_REG_2C: c_uint = 0x2c;

pub const JEIDA: c_int = 0;

pub const SISO: c_int = 0;
pub const LVDS_REG_3C: c_uint = 0x3c;
pub const LVDS_REG_3F: c_uint = 0x3f;
pub const LVDS_REG_47: c_uint = 0x47;
pub const LVDS_REG_48: c_uint = 0x48;
pub const LVDS_REG_4F: c_uint = 0x4f;
pub const LVDS_REG_52: c_uint = 0x52;
// -----------------------------------------------------------------------------
// HDMI registers are separated into three banks:
// 1) HDMI register common bank: 0x00 ~ 0x2f
//
// HDMI genernal registers
pub const HDMI_REG_SW_RST: c_uint = 0x04;

    AUD_RST | HDCP_RST)
pub const HDMI_REG_SYS_STATUS: c_uint = 0x0e;

pub const HDMI_REG_BANK_CTRL: c_uint = 0x0f;

// HDMI System DDC control registers
pub const HDMI_REG_DDC_MASTER_CTRL: c_uint = 0x10;

pub const HDMI_REG_DDC_HEADER: c_uint = 0x11;
pub const HDMI_REG_DDC_REQOFF: c_uint = 0x12;
pub const HDMI_REG_DDC_REQCOUNT: c_uint = 0x13;
pub const HDMI_REG_DDC_EDIDSEG: c_uint = 0x14;
pub const HDMI_REG_DDC_CMD: c_uint = 0x15;
pub const DDC_CMD_EDID_READ: c_uint = 0x3;
pub const DDC_CMD_FIFO_CLR: c_uint = 0x9;
pub const HDMI_REG_DDC_STATUS: c_uint = 0x16;

pub const HDMI_DDC_FIFO_BYTES: c_int = 32;
pub const HDMI_REG_DDC_READFIFO: c_uint = 0x17;
pub const HDMI_REG_LVDS_PORT: c_uint = 0x1d /* LVDS input control I2C addr */;
pub const HDMI_REG_LVDS_PORT_EN: c_uint = 0x1e;
pub const LVDS_INPUT_CTRL_I2C_ADDR: c_uint = 0x33;
// -----------------------------------------------------------------------------
// 2) HDMI register bank0: 0x30 ~ 0xff
//
// HDMI AFE registers
pub const HDMI_REG_AFE_DRV_CTRL: c_uint = 0x61;

pub const HDMI_REG_AFE_XP_CTRL: c_uint = 0x62;

pub const HDMI_REG_AFE_ISW_CTRL: c_uint = 0x63;
pub const HDMI_REG_AFE_IP_CTRL: c_uint = 0x64;

// HDMI input data format registers
pub const HDMI_REG_INPUT_MODE: c_uint = 0x70;
pub const IN_RGB: c_uint = 0x00;
// HDMI general control registers
pub const HDMI_REG_HDMI_MODE: c_uint = 0xc0;

pub const HDMI_REG_GCP: c_uint = 0xc1;

pub const HDMI_REG_PKT_GENERAL_CTRL: c_uint = 0xc6;
pub const HDMI_REG_PKT_NULL_CTRL: c_uint = 0xc9;
pub const HDMI_REG_AVI_INFOFRM_CTRL: c_uint = 0xcd;

// -----------------------------------------------------------------------------
// 3) HDMI register bank1: 0x130 ~ 0x1ff (HDMI packet registers)
//
// NULL packet registers
// Header Byte(HB): n = 0 ~ 2

// Packet Byte(PB): n = 0 ~ 27(HDMI_MAX_INFOFRAME_SIZE), n = 0 for checksum

// AVI packet registers
pub const HDMI_REG_AVI_DB1: c_uint = 0x158;
pub const HDMI_REG_AVI_DB2: c_uint = 0x159;
pub const HDMI_REG_AVI_DB3: c_uint = 0x15a;
pub const HDMI_REG_AVI_DB4: c_uint = 0x15b;
pub const HDMI_REG_AVI_DB5: c_uint = 0x15c;
pub const HDMI_REG_AVI_CSUM: c_uint = 0x15d;
pub const HDMI_REG_AVI_DB6: c_uint = 0x15e;
pub const HDMI_REG_AVI_DB7: c_uint = 0x15f;
pub const HDMI_REG_AVI_DB8: c_uint = 0x160;
pub const HDMI_REG_AVI_DB9: c_uint = 0x161;
pub const HDMI_REG_AVI_DB10: c_uint = 0x162;
pub const HDMI_REG_AVI_DB11: c_uint = 0x163;
pub const HDMI_REG_AVI_DB12: c_uint = 0x164;
pub const HDMI_REG_AVI_DB13: c_uint = 0x165;

// IT6263 data sheet Rev0.8: LVDS RX supports input clock rate up to 150MHz.
pub const MAX_PIXEL_CLOCK_KHZ: c_int = 150000;
// IT6263 programming guide Ver0.90: PCLK_HIGH for TMDS clock over 80MHz.
pub const HIGH_PIXEL_CLOCK_KHZ: c_int = 80000;
//
// IT6263 data sheet Rev0.8: HDMI TX supports link speeds of up to 2.25Gbps
// (link clock rate of 225MHz).
//
pub const MAX_HDMI_TMDS_CHAR_RATE_HZ: c_int = 225000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct it6263 {
    pub dev: *mut device,
    pub hdmi_i2c: *mut i2c_client,
    pub lvds_i2c: *mut i2c_client,
    pub hdmi_regmap: *mut regmap,
    pub lvds_regmap: *mut regmap,
    pub bridge: drm_bridge,
    pub next_bridge: *mut drm_bridge,
    pub reset_gpio: *mut gpio_desc,
    pub lvds_data_mapping: c_int,
    pub lvds_dual_link: bool,
    pub lvds_link12_swap: bool,
}

    static inline struct it6263 *bridge_to_it6263(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct it6263, bridge);
    }
#[no_mangle]
unsafe extern "C" fn it6263_hdmi_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool it6263_hdmi_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case HDMI_REG_SW_RST:
    case HDMI_REG_BANK_CTRL:
    case HDMI_REG_DDC_MASTER_CTRL:
    case HDMI_REG_DDC_HEADER:
    case HDMI_REG_DDC_REQOFF:
    case HDMI_REG_DDC_REQCOUNT:
    case HDMI_REG_DDC_EDIDSEG:
    case HDMI_REG_DDC_CMD:
    case HDMI_REG_LVDS_PORT:
    case HDMI_REG_LVDS_PORT_EN:
    case HDMI_REG_AFE_DRV_CTRL:
    case HDMI_REG_AFE_XP_CTRL:
    case HDMI_REG_AFE_ISW_CTRL:
    case HDMI_REG_AFE_IP_CTRL:
    case HDMI_REG_INPUT_MODE:
    case HDMI_REG_HDMI_MODE:
    case HDMI_REG_GCP:
    case HDMI_REG_PKT_GENERAL_CTRL:
    case HDMI_REG_PKT_NULL_CTRL:
    case HDMI_REG_AVI_INFOFRM_CTRL:
    case HDMI_REG_PKT_HB(0) ... HDMI_REG_PKT_PB(HDMI_MAX_INFOFRAME_SIZE):
    case HDMI_REG_AVI_DB1:
    case HDMI_REG_AVI_DB2:
    case HDMI_REG_AVI_DB3:
    case HDMI_REG_AVI_DB4:
    case HDMI_REG_AVI_DB5:
    case HDMI_REG_AVI_CSUM:
    case HDMI_REG_AVI_DB6:
    case HDMI_REG_AVI_DB7:
    case HDMI_REG_AVI_DB8:
    case HDMI_REG_AVI_DB9:
    case HDMI_REG_AVI_DB10:
    case HDMI_REG_AVI_DB11:
    case HDMI_REG_AVI_DB12:
    case HDMI_REG_AVI_DB13:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn it6263_hdmi_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool it6263_hdmi_readable_reg(struct device *dev, unsigned int reg)
    {
    if (it6263_hdmi_writeable_reg(dev, reg))
    return true;
    switch (reg) {
    case HDMI_REG_SYS_STATUS:
    case HDMI_REG_DDC_STATUS:
    case HDMI_REG_DDC_READFIFO:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn it6263_hdmi_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool it6263_hdmi_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case HDMI_REG_SW_RST:
    case HDMI_REG_SYS_STATUS:
    case HDMI_REG_DDC_STATUS:
    case HDMI_REG_DDC_READFIFO:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_range_cfg it6263_hdmi_range_cfg = {
    .range_min = 0x00,
    .range_max = HDMI_REG_AVI_DB13,
    .selector_reg = HDMI_REG_BANK_CTRL,
    .selector_mask = REG_BANK_SEL,
    .selector_shift = 0,
    .window_start = 0x00,
    .window_len = 0x100,
    };
    static const struct regmap_config it6263_hdmi_regmap_config = {
    .name = "it6263-hdmi",
    .reg_bits = 8,
    .val_bits = 8,
    .writeable_reg = it6263_hdmi_writeable_reg,
    .readable_reg = it6263_hdmi_readable_reg,
    .volatile_reg = it6263_hdmi_volatile_reg,
    .max_register = HDMI_REG_AVI_DB13,
    .ranges = &it6263_hdmi_range_cfg,
    .num_ranges = 1,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn it6263_lvds_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool it6263_lvds_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case LVDS_REG_05:
    case LVDS_REG_0B:
    case LVDS_REG_2C:
    case LVDS_REG_3C:
    case LVDS_REG_3F:
    case LVDS_REG_47:
    case LVDS_REG_48:
    case LVDS_REG_4F:
    case LVDS_REG_52:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn it6263_lvds_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool it6263_lvds_readable_reg(struct device *dev, unsigned int reg)
    {
    return it6263_lvds_writeable_reg(dev, reg);
    }
#[no_mangle]
unsafe extern "C" fn it6263_lvds_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool it6263_lvds_volatile_reg(struct device *dev, unsigned int reg)
    {
    let mut reg: return = = LVDS_REG_05;
    }
    static const struct regmap_config it6263_lvds_regmap_config = {
    .name = "it6263-lvds",
    .reg_bits = 8,
    .val_bits = 8,
    .writeable_reg = it6263_lvds_writeable_reg,
    .readable_reg = it6263_lvds_readable_reg,
    .volatile_reg = it6263_lvds_volatile_reg,
    .max_register = LVDS_REG_52,
    .cache_type = REGCACHE_MAPLE,
    };
    static const char * const it6263_supplies[] = {
    "ivdd", "ovdd", "txavcc18", "txavcc33", "pvcc1", "pvcc2",
    "avcc", "anvdd", "apvdd"
    };
#[no_mangle]
unsafe extern "C" fn it6263_parse_dt(it: *mut it6263) -> c_int {
    static int it6263_parse_dt(struct it6263 *it)
    {
    struct device *dev = it.dev;
    struct device_node *port0, *port1;
    let mut ret: c_int = 0;
    it.lvds_data_mapping = drm_of_lvds_get_data_mapping(dev.of_node);
    if (it.lvds_data_mapping < 0) {
    dev_err(dev, "%pOF: invalid or missing %s DT property: %d\n",
    dev.of_node, "data-mapping", it.lvds_data_mapping);
    return it.lvds_data_mapping;
    }
    it.next_bridge = devm_drm_of_get_bridge(dev, dev.of_node, 2, 0);
    if (IS_ERR(it.next_bridge))
    return dev_err_probe(dev, PTR_ERR(it.next_bridge),
    "failed to get next bridge\n");
    port0 = of_graph_get_port_by_id(dev.of_node, 0);
    port1 = of_graph_get_port_by_id(dev.of_node, 1);
    if (port0 && port1) {
    int order;
    it.lvds_dual_link = true;
    order = drm_of_lvds_get_dual_link_pixel_order_sink(port0, port1);
    if (order < 0) {
    dev_err(dev,
    "failed to get dual link pixel order: %d\n",
    order);
    ret = order;
    } else if (order == DRM_LVDS_DUAL_LINK_EVEN_ODD_PIXELS) {
    it.lvds_link12_swap = true;
    }
    } else if (port1) {
    ret = -EINVAL;
    dev_err(dev, "single input LVDS port1 is not supported\n");
    } else if (!port0) {
    ret = -EINVAL;
    dev_err(dev, "no input LVDS port\n");
    }
    of_node_put(port0);
    of_node_put(port1);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn it6263_hw_reset(reset_gpio: *mut gpio_desc) {
    static inline void it6263_hw_reset(struct gpio_desc *reset_gpio)
    {
    if (!reset_gpio)
    return;
    gpiod_set_value_cansleep(reset_gpio, 0);
    fsleep(1000);
    gpiod_set_value_cansleep(reset_gpio, 1);
// The chip maker says the low pulse should be at least 40ms.
    fsleep(40000);
    gpiod_set_value_cansleep(reset_gpio, 0);
// addtional time to wait the high voltage to be stable
    fsleep(5000);
    }
#[no_mangle]
pub unsafe extern "C" fn it6263_lvds_set_i2c_addr(it: *mut it6263) -> c_int {
    static inline int it6263_lvds_set_i2c_addr(struct it6263 *it)
    {
    int ret;
    ret = regmap_write(it.hdmi_regmap, HDMI_REG_LVDS_PORT,
    LVDS_INPUT_CTRL_I2C_ADDR << 1);
    if (ret)
    return ret;
    return regmap_write(it.hdmi_regmap, HDMI_REG_LVDS_PORT_EN, BIT(0));
    }
#[no_mangle]
pub unsafe extern "C" fn it6263_lvds_reset(it: *mut it6263) {
    static inline void it6263_lvds_reset(struct it6263 *it)
    {
// AFE PLL reset
    regmap_write_bits(it.lvds_regmap, LVDS_REG_3C, BIT(0), 0x0);
    fsleep(1000);
    regmap_write_bits(it.lvds_regmap, LVDS_REG_3C, BIT(0), BIT(0));
// software pixel clock domain reset
    regmap_write_bits(it.lvds_regmap, LVDS_REG_05, REG_SOFT_P_RST,
    REG_SOFT_P_RST);
    fsleep(1000);
    regmap_write_bits(it.lvds_regmap, LVDS_REG_05, REG_SOFT_P_RST, 0x0);
    fsleep(10000);
    }
#[no_mangle]
pub unsafe extern "C" fn it6263_is_input_bus_fmt_valid(input_fmt: c_int) -> bool {
    static inline bool it6263_is_input_bus_fmt_valid(int input_fmt)
    {
    switch (input_fmt) {
    case MEDIA_BUS_FMT_RGB888_1X7X4_JEIDA:
    case MEDIA_BUS_FMT_RGB888_1X7X4_SPWG:
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn it6263_lvds_set_interface(it: *mut it6263) {
    static inline void it6263_lvds_set_interface(struct it6263 *it)
    {
    u8 fmt;
// color depth
    regmap_write_bits(it.lvds_regmap, LVDS_REG_2C, REG_COL_DEP, BIT8);
    if (it.lvds_data_mapping == MEDIA_BUS_FMT_RGB888_1X7X4_SPWG)
    fmt = VESA;
    else
    fmt = JEIDA;
// output mapping
    regmap_write_bits(it.lvds_regmap, LVDS_REG_2C, OUT_MAP, fmt);
    if (it.lvds_dual_link) {
    regmap_write_bits(it.lvds_regmap, LVDS_REG_2C, DMODE, DISO);
    regmap_write_bits(it.lvds_regmap, LVDS_REG_52, BIT(1), BIT(1));
    } else {
    regmap_write_bits(it.lvds_regmap, LVDS_REG_2C, DMODE, SISO);
    regmap_write_bits(it.lvds_regmap, LVDS_REG_52, BIT(1), 0);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn it6263_lvds_set_afe(it: *mut it6263) {
    static inline void it6263_lvds_set_afe(struct it6263 *it)
    {
    regmap_write(it.lvds_regmap, LVDS_REG_3C, 0xaa);
    regmap_write(it.lvds_regmap, LVDS_REG_3F, 0x02);
    regmap_write(it.lvds_regmap, LVDS_REG_47, 0xaa);
    regmap_write(it.lvds_regmap, LVDS_REG_48, 0x02);
    regmap_write(it.lvds_regmap, LVDS_REG_4F, 0x11);
    regmap_write_bits(it.lvds_regmap, LVDS_REG_0B, REG_SSC_PCLK_RF,
    REG_SSC_PCLK_RF);
    regmap_write_bits(it.lvds_regmap, LVDS_REG_3C, 0x07, 0);
    regmap_write_bits(it.lvds_regmap, LVDS_REG_2C, REG_DESSC_ENB,
    REG_DESSC_ENB);
    }
#[no_mangle]
pub unsafe extern "C" fn it6263_lvds_sys_cfg(it: *mut it6263) {
    static inline void it6263_lvds_sys_cfg(struct it6263 *it)
    {
    regmap_write_bits(it.lvds_regmap, LVDS_REG_0B, REG_LVDS_IN_SWAP,
    it.lvds_link12_swap ? REG_LVDS_IN_SWAP : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn it6263_lvds_config(it: *mut it6263) {
    static inline void it6263_lvds_config(struct it6263 *it)
    {
    it6263_lvds_reset(it);
    it6263_lvds_set_interface(it);
    it6263_lvds_set_afe(it);
    it6263_lvds_sys_cfg(it);
    }
#[no_mangle]
pub unsafe extern "C" fn it6263_hdmi_config(it: *mut it6263) {
    static inline void it6263_hdmi_config(struct it6263 *it)
    {
    regmap_write(it.hdmi_regmap, HDMI_REG_SW_RST, HDMI_RST_ALL);
    regmap_write(it.hdmi_regmap, HDMI_REG_INPUT_MODE, IN_RGB);
    regmap_write_bits(it.hdmi_regmap, HDMI_REG_GCP, HDMI_COLOR_DEPTH,
    HDMI_COLOR_DEPTH_24);
    }
#[no_mangle]
unsafe extern "C" fn it6263_detect(it: *mut it6263) -> enum drm_connector_status {
    static enum drm_connector_status it6263_detect(struct it6263 *it)
    {
    unsigned int val;
    regmap_read(it.hdmi_regmap, HDMI_REG_SYS_STATUS, &val);
    if (val & HPDETECT)
    return connector_status_connected;
    else
    return connector_status_disconnected;
    }
#[no_mangle]
unsafe extern "C" fn it6263_read_edid(data: *mut c_void, buf: *mut u8, block: c_uint, len: usize) -> c_int {
    static int it6263_read_edid(void *data, u8 *buf, unsigned int block, size_t len)
    {
    struct it6263 *it = data;
    struct regmap *regmap = it.hdmi_regmap;
    let mut start: c_uint = (block % 2) * EDID_LENGTH;
    let mut segment: c_uint = block >> 1;
    unsigned int count, val;
    int ret;
    regmap_write(regmap, HDMI_REG_DDC_MASTER_CTRL, MASTER_SEL_HOST);
    regmap_write(regmap, HDMI_REG_DDC_HEADER, DDC_ADDR << 1);
    regmap_write(regmap, HDMI_REG_DDC_EDIDSEG, segment);
    while (len) {
// clear DDC FIFO
    regmap_write(regmap, HDMI_REG_DDC_CMD, DDC_CMD_FIFO_CLR);
    ret = regmap_read_poll_timeout(regmap, HDMI_REG_DDC_STATUS,
    val, val & DDC_DONE,
    2000, 10000);
    if (ret) {
    dev_err(it.dev, "failed to clear DDC FIFO:%d\n", ret);
    return ret;
    }
    count = len > HDMI_DDC_FIFO_BYTES ? HDMI_DDC_FIFO_BYTES : len;
// fire the read command
    regmap_write(regmap, HDMI_REG_DDC_REQOFF, start);
    regmap_write(regmap, HDMI_REG_DDC_REQCOUNT, count);
    regmap_write(regmap, HDMI_REG_DDC_CMD, DDC_CMD_EDID_READ);
    start += count;
    len -= count;
    ret = regmap_read_poll_timeout(regmap, HDMI_REG_DDC_STATUS, val,
    val & (DDC_DONE | DDC_ERROR),
    20000, 250000);
    if (ret && !(val & DDC_ERROR)) {
    dev_err(it.dev, "failed to read EDID:%d\n", ret);
    return ret;
    }
    if (val & DDC_ERROR) {
    dev_err(it.dev, "DDC error\n");
    return -EIO;
    }
// cache to buffer
    for (; count > 0; count--) {
    regmap_read(regmap, HDMI_REG_DDC_READFIFO, &val);
// (buf++) = val;
    }
    }
    return 0;
    }
    static void it6263_bridge_atomic_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    regmap_write_bits(it.hdmi_regmap, HDMI_REG_GCP, AVMUTE, AVMUTE);
    regmap_write(it.hdmi_regmap, HDMI_REG_PKT_GENERAL_CTRL, 0);
    regmap_write(it.hdmi_regmap, HDMI_REG_AFE_DRV_CTRL,
    AFE_DRV_RST | AFE_DRV_PWD);
    }
    static void it6263_bridge_atomic_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    const struct drm_crtc_state *crtc_state;
    struct regmap *regmap = it.hdmi_regmap;
    const struct drm_display_mode *mode;
    struct drm_connector *connector;
    let mut is_stable: bool = false;
    struct drm_crtc *crtc;
    unsigned int val;
    bool pclk_high;
    int i, ret;
    it6263_hw_reset(it.reset_gpio);
    ret = it6263_lvds_set_i2c_addr(it);
    if (ret)
    dev_err(it.dev, "failed to set I2C addr\n");
    it6263_lvds_config(it);
    it6263_hdmi_config(it);
    connector = drm_atomic_get_new_connector_for_encoder(state,
    bridge.encoder);
    crtc = drm_atomic_get_new_connector_state(state, connector).crtc;
    crtc_state = drm_atomic_get_new_crtc_state(state, crtc);
    mode = &crtc_state.adjusted_mode;
    regmap_write(regmap, HDMI_REG_HDMI_MODE, TX_HDMI_MODE);
    drm_atomic_helper_connector_hdmi_update_infoframes(connector, state);
// HDMI AFE setup
    pclk_high = mode.clock > HIGH_PIXEL_CLOCK_KHZ;
    regmap_write(regmap, HDMI_REG_AFE_DRV_CTRL, AFE_DRV_RST);
    if (pclk_high)
    regmap_write(regmap, HDMI_REG_AFE_XP_CTRL,
    AFE_XP_GAINBIT | AFE_XP_RESETB);
    else
    regmap_write(regmap, HDMI_REG_AFE_XP_CTRL,
    AFE_XP_ER0 | AFE_XP_RESETB);
    regmap_write(regmap, HDMI_REG_AFE_ISW_CTRL, 0x10);
    if (pclk_high)
    regmap_write(regmap, HDMI_REG_AFE_IP_CTRL,
    AFE_IP_GAINBIT | AFE_IP_RESETB);
    else
    regmap_write(regmap, HDMI_REG_AFE_IP_CTRL,
    AFE_IP_ER0 | AFE_IP_RESETB);
// HDMI software video reset
    regmap_write_bits(regmap, HDMI_REG_SW_RST, SOFTV_RST, SOFTV_RST);
    fsleep(1000);
    regmap_write_bits(regmap, HDMI_REG_SW_RST, SOFTV_RST, 0);
// reconfigure LVDS and retry several times in case video is instable
    for (i = 0; i < 3; i++) {
    ret = regmap_read_poll_timeout(regmap, HDMI_REG_SYS_STATUS, val,
    val & TXVIDSTABLE,
    20000, 500000);
    if (!ret) {
    is_stable = true;
    break;
    }
    it6263_lvds_config(it);
    }
    if (!is_stable)
    dev_warn(it.dev, "failed to wait for video stable\n");
// HDMI AFE reset release and power up
    regmap_write(regmap, HDMI_REG_AFE_DRV_CTRL, 0);
    regmap_write_bits(regmap, HDMI_REG_GCP, AVMUTE, 0);
    regmap_write(regmap, HDMI_REG_PKT_GENERAL_CTRL, ENABLE_PKT | REPEAT_PKT);
    }
    static enum drm_mode_status
    it6263_bridge_mode_valid(struct drm_bridge *bridge,
    const struct drm_display_info *info,
    const struct drm_display_mode *mode)
    {
    unsigned long long rate;
    rate = drm_hdmi_compute_mode_clock(mode, 8, DRM_OUTPUT_COLOR_FORMAT_RGB444);
    if (rate == 0)
    return MODE_NOCLOCK;
    return bridge.funcs.hdmi_tmds_char_rate_valid(bridge, mode, rate);
    }
    static int it6263_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    struct drm_connector *connector;
    int ret;
    ret = drm_bridge_attach(encoder, it.next_bridge, bridge,
    flags | DRM_BRIDGE_ATTACH_NO_CONNECTOR);
    if (ret < 0)
    return ret;
    if (flags & DRM_BRIDGE_ATTACH_NO_CONNECTOR)
    return 0;
    connector = drm_bridge_connector_init(bridge.dev, encoder);
    if (IS_ERR(connector)) {
    ret = PTR_ERR(connector);
    dev_err(it.dev, "failed to initialize bridge connector: %d\n",
    ret);
    return ret;
    }
    return 0;
    }
    static enum drm_connector_status
    it6263_bridge_detect(struct drm_bridge *bridge, struct drm_connector *connector)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    return it6263_detect(it);
    }
    static const struct drm_edid *
    it6263_bridge_edid_read(struct drm_bridge *bridge,
    struct drm_connector *connector)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    return drm_edid_read_custom(connector, it6263_read_edid, it);
    }
    static u32 *
    it6263_bridge_atomic_get_input_bus_fmts(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state,
    u32 output_fmt,
    unsigned int *num_input_fmts)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    u32 *input_fmts;
// num_input_fmts = 0;
    if (!it6263_is_input_bus_fmt_valid(it.lvds_data_mapping))
    return core::ptr::null_mut();
    input_fmts = kmalloc_obj(*input_fmts);
    if (!input_fmts)
    return core::ptr::null_mut();
    input_fmts[0] = it.lvds_data_mapping;
// num_input_fmts = 1;
    return input_fmts;
    }
    static enum drm_mode_status
    it6263_hdmi_tmds_char_rate_valid(const struct drm_bridge *bridge,
    const struct drm_display_mode *mode,
    unsigned long long tmds_rate)
    {
    if (mode.clock > MAX_PIXEL_CLOCK_KHZ)
    return MODE_CLOCK_HIGH;
    if (tmds_rate > MAX_HDMI_TMDS_CHAR_RATE_HZ)
    return MODE_CLOCK_HIGH;
    return MODE_OK;
    }
#[no_mangle]
unsafe extern "C" fn it6263_hdmi_clear_avi_infoframe(bridge: *mut drm_bridge) -> c_int {
    static int it6263_hdmi_clear_avi_infoframe(struct drm_bridge *bridge)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    regmap_write(it.hdmi_regmap, HDMI_REG_AVI_INFOFRM_CTRL, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn it6263_hdmi_clear_hdmi_infoframe(bridge: *mut drm_bridge) -> c_int {
    static int it6263_hdmi_clear_hdmi_infoframe(struct drm_bridge *bridge)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    regmap_write(it.hdmi_regmap, HDMI_REG_PKT_NULL_CTRL, 0);
    return 0;
    }
    static int it6263_hdmi_write_avi_infoframe(struct drm_bridge *bridge,
    const u8 *buffer, size_t len)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    struct regmap *regmap = it.hdmi_regmap;
// write the first AVI infoframe data byte chunk(DB1-DB5)
    regmap_bulk_write(regmap, HDMI_REG_AVI_DB1,
    &buffer[HDMI_INFOFRAME_HEADER_SIZE],
    HDMI_AVI_DB_CHUNK1_SIZE);
// write the second AVI infoframe data byte chunk(DB6-DB13)
    regmap_bulk_write(regmap, HDMI_REG_AVI_DB6,
    &buffer[HDMI_INFOFRAME_HEADER_SIZE +
    HDMI_AVI_DB_CHUNK1_SIZE],
    HDMI_AVI_DB_CHUNK2_SIZE);
// write checksum
    regmap_write(regmap, HDMI_REG_AVI_CSUM, buffer[3]);
    regmap_write(regmap, HDMI_REG_AVI_INFOFRM_CTRL,
    ENABLE_PKT | REPEAT_PKT);
    return 0;
    }
    static int it6263_hdmi_write_hdmi_infoframe(struct drm_bridge *bridge,
    const u8 *buffer, size_t len)
    {
    struct it6263 *it = bridge_to_it6263(bridge);
    struct regmap *regmap = it.hdmi_regmap;
// write header and payload
    regmap_bulk_write(regmap, HDMI_REG_PKT_HB(0), buffer, len);
    regmap_write(regmap, HDMI_REG_PKT_NULL_CTRL,
    ENABLE_PKT | REPEAT_PKT);
    return 0;
    }
    static const struct drm_bridge_funcs it6263_bridge_funcs = {
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .attach = it6263_bridge_attach,
    .mode_valid = it6263_bridge_mode_valid,
    .atomic_disable = it6263_bridge_atomic_disable,
    .atomic_enable = it6263_bridge_atomic_enable,
    .detect = it6263_bridge_detect,
    .edid_read = it6263_bridge_edid_read,
    .atomic_get_input_bus_fmts = it6263_bridge_atomic_get_input_bus_fmts,
    .hdmi_tmds_char_rate_valid = it6263_hdmi_tmds_char_rate_valid,
    .hdmi_clear_avi_infoframe = it6263_hdmi_clear_avi_infoframe,
    .hdmi_write_avi_infoframe = it6263_hdmi_write_avi_infoframe,
    .hdmi_clear_hdmi_infoframe = it6263_hdmi_clear_hdmi_infoframe,
    .hdmi_write_hdmi_infoframe = it6263_hdmi_write_hdmi_infoframe,
    };
#[no_mangle]
unsafe extern "C" fn it6263_probe(client: *mut i2c_client) -> c_int {
    static int it6263_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct it6263 *it;
    int ret;
    it = devm_drm_bridge_alloc(dev, struct it6263, bridge,
    &it6263_bridge_funcs);
    if (IS_ERR(it))
    return PTR_ERR(it);
    it.dev = dev;
    it.hdmi_i2c = client;
    it.hdmi_regmap = devm_regmap_init_i2c(client,
    &it6263_hdmi_regmap_config);
    if (IS_ERR(it.hdmi_regmap))
    return dev_err_probe(dev, PTR_ERR(it.hdmi_regmap),
    "failed to init I2C regmap for HDMI\n");
    it.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(it.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(it.reset_gpio),
    "failed to get reset gpio\n");
    ret = devm_regulator_bulk_get_enable(dev, ARRAY_SIZE(it6263_supplies),
    it6263_supplies);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get power supplies\n");
    ret = it6263_parse_dt(it);
    if (ret)
    return ret;
    it.lvds_i2c = devm_i2c_new_dummy_device(dev, client.adapter,
    LVDS_INPUT_CTRL_I2C_ADDR);
    if (IS_ERR(it.lvds_i2c))
    return dev_err_probe(it.dev, PTR_ERR(it.lvds_i2c),
    "failed to allocate I2C device for LVDS\n");
    it.lvds_regmap = devm_regmap_init_i2c(it.lvds_i2c,
    &it6263_lvds_regmap_config);
    if (IS_ERR(it.lvds_regmap))
    return dev_err_probe(dev, PTR_ERR(it.lvds_regmap),
    "failed to init I2C regmap for LVDS\n");
    i2c_set_clientdata(client, it);
    it.bridge.of_node = dev.of_node;
// IT6263 chip doesn't support HPD interrupt.
    it.bridge.ops = DRM_BRIDGE_OP_DETECT | DRM_BRIDGE_OP_EDID |
    DRM_BRIDGE_OP_HDMI;
    it.bridge.type = DRM_MODE_CONNECTOR_HDMIA;
    it.bridge.vendor = "ITE";
    it.bridge.product = "IT6263";
    return devm_drm_bridge_add(dev, &it.bridge);
    }
    static const struct of_device_id it6263_of_match[] = {
    { .compatible = "ite,it6263", },
    { }
    };
    MODULE_DEVICE_TABLE(of, it6263_of_match);
    static const struct i2c_device_id it6263_i2c_ids[] = {
    { .name = "it6263" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, it6263_i2c_ids);
    static struct i2c_driver it6263_driver = {
    .probe = it6263_probe,
    .driver = {
    .name = "it6263",
    .of_match_table = it6263_of_match,
    },
    .id_table = it6263_i2c_ids,
    };
    module_i2c_driver(it6263_driver);
    MODULE_DESCRIPTION("ITE Tech. Inc. IT6263 LVDS/HDMI bridge");
    MODULE_AUTHOR("Liu Ying <victor.liu@nxp.com>");
    MODULE_LICENSE("GPL");
