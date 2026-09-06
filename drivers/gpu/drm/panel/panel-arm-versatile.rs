//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-arm-versatile.c
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
// Panel driver for the ARM Versatile family reference designs from
// ARM Limited.
//
// Author:
// Linus Walleij <linus.wallei@linaro.org>
//
// On the Versatile AB, these panels come mounted on daughterboards
// named "IB1" or "IB2" (Interface Board 1 & 2 respectively.) They
// are documented in ARM DUI 0225D Appendix C and D. These daughter
// boards support TFT display panels.
//
// - The IB1 is a passive board where the display connector defines a
// few wires for encoding the display type for autodetection,
// suitable display settings can then be looked up from this setting.
// The magic bits can be read out from the system controller.
//
// - The IB2 is a more complex board intended for GSM phone development
// with some logic and a control register, which needs to be accessed
// and the board display needs to be turned on explicitly.
//
// On the Versatile PB, a special CLCD adaptor board is available
// supporting the same displays as the Versatile AB, plus one more
// Epson QCIF display.
//

//
// This configuration register in the Versatile and RealView
// family is uniformly present but appears more and more
// unutilized starting with the RealView series.
//
pub const SYS_CLCD: c_uint = 0x50;
// The Versatile can detect the connected panel type

// IB2 control register for the Versatile daughterboard
pub const IB2_CTRL: c_uint = 0x00;

//
// struct versatile_panel_type - lookup struct for the supported panels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct versatile_panel_type {
//
// @name: the name of this panel
//
    pub name: *const c_char,
//
// @magic: the magic value from the detection register
//
    pub magic: u32,
//
// @mode: the DRM display mode for this panel
//
    pub mode: drm_display_mode,
//
// @bus_flags: the DRM bus flags for this panel e.g. inverted clock
//
    pub bus_flags: u32,
//
// @width_mm: the panel width in mm
//
    pub width_mm: u32,
//
// @height_mm: the panel height in mm
//
    pub height_mm: u32,
//
// @ib2: the panel may be connected on an IB2 daughterboard
//
    pub ib2: bool,
}

//
// struct versatile_panel - state container for the Versatile panels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct versatile_panel {
//
// @dev: the container device
//
    pub dev: *mut device,
//
// @panel: the DRM panel instance for this device
//
    pub panel: drm_panel,
//
// @panel_type: the Versatile panel type as detected
//
    pub panel_type: *const versatile_panel_type,
//
// @map: map to the parent syscon where the main register reside
//
    pub map: *mut regmap,
//
// @ib2_map: map to the IB2 syscon, if applicable
//
    pub ib2_map: *mut regmap,
}

    static const struct versatile_panel_type versatile_panels[] = {
//
// Sanyo TM38QV67A02A - 3.8 inch QVGA (320x240) Color TFT
// found on the Versatile AB IB1 connector or the Versatile
// PB adaptor board connector.
//
    {
    .name = "Sanyo TM38QV67A02A",
    .magic = SYS_CLCD_ID_SANYO_3_8,
    .width_mm = 79,
    .height_mm = 54,
    .mode = {
    .clock = 10000,
    .hdisplay = 320,
    .hsync_start = 320 + 6,
    .hsync_end = 320 + 6 + 6,
    .htotal = 320 + 6 + 6 + 6,
    .vdisplay = 240,
    .vsync_start = 240 + 5,
    .vsync_end = 240 + 5 + 6,
    .vtotal = 240 + 5 + 6 + 5,
    .flags = DRM_MODE_FLAG_PHSYNC | DRM_MODE_FLAG_PVSYNC,
    },
    },
//
// Sharp LQ084V1DG21 640x480 VGA Color TFT module
// found on the Versatile AB IB1 connector or the Versatile
// PB adaptor board connector.
//
    {
    .name = "Sharp LQ084V1DG21",
    .magic = SYS_CLCD_ID_SHARP_8_4,
    .width_mm = 171,
    .height_mm = 130,
    .mode = {
    .clock = 25000,
    .hdisplay = 640,
    .hsync_start = 640 + 24,
    .hsync_end = 640 + 24 + 96,
    .htotal = 640 + 24 + 96 + 24,
    .vdisplay = 480,
    .vsync_start = 480 + 11,
    .vsync_end = 480 + 11 + 2,
    .vtotal = 480 + 11 + 2 + 32,
    .flags = DRM_MODE_FLAG_PHSYNC | DRM_MODE_FLAG_PVSYNC,
    },
    },
//
// Epson L2F50113T00 - 2.2 inch QCIF 176x220 Color TFT
// found on the Versatile PB adaptor board connector.
//
    {
    .name = "Epson L2F50113T00",
    .magic = SYS_CLCD_ID_EPSON_2_2,
    .width_mm = 34,
    .height_mm = 45,
    .mode = {
    .clock = 62500,
    .hdisplay = 176,
    .hsync_start = 176 + 2,
    .hsync_end = 176 + 2 + 3,
    .htotal = 176 + 2 + 3 + 3,
    .vdisplay = 220,
    .vsync_start = 220 + 0,
    .vsync_end = 220 + 0 + 2,
    .vtotal = 220 + 0 + 2 + 1,
    .flags = DRM_MODE_FLAG_PHSYNC | DRM_MODE_FLAG_PVSYNC,
    },
    .bus_flags = DRM_BUS_FLAG_PIXDATA_DRIVE_NEGEDGE,
    },
//
// Sanyo ALR252RGT 240x320 portrait display found on the
// Versatile AB IB2 daughterboard for GSM prototyping.
//
    {
    .name = "Sanyo ALR252RGT",
    .magic = SYS_CLCD_ID_SANYO_2_5,
    .width_mm = 37,
    .height_mm = 50,
    .mode = {
    .clock = 5400,
    .hdisplay = 240,
    .hsync_start = 240 + 10,
    .hsync_end = 240 + 10 + 10,
    .htotal = 240 + 10 + 10 + 20,
    .vdisplay = 320,
    .vsync_start = 320 + 2,
    .vsync_end = 320 + 2 + 2,
    .vtotal = 320 + 2 + 2 + 2,
    .flags = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    },
    .bus_flags = DRM_BUS_FLAG_PIXDATA_DRIVE_NEGEDGE,
    .ib2 = true,
    },
    };
    static inline struct versatile_panel *
    to_versatile_panel(struct drm_panel *panel)
    {
    return container_of(panel, struct versatile_panel, panel);
    }
#[no_mangle]
unsafe extern "C" fn versatile_panel_disable(panel: *mut drm_panel) -> c_int {
    static int versatile_panel_disable(struct drm_panel *panel)
    {
    struct versatile_panel *vpanel = to_versatile_panel(panel);
// If we're on an IB2 daughterboard, turn off display
    if (vpanel.ib2_map) {
    dev_dbg(vpanel.dev, "disable IB2 display\n");
    regmap_update_bits(vpanel.ib2_map,
    IB2_CTRL,
    IB2_CTRL_LCD_MASK,
    IB2_CTRL_LCD_SD);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn versatile_panel_enable(panel: *mut drm_panel) -> c_int {
    static int versatile_panel_enable(struct drm_panel *panel)
    {
    struct versatile_panel *vpanel = to_versatile_panel(panel);
// If we're on an IB2 daughterboard, turn on display
    if (vpanel.ib2_map) {
    dev_dbg(vpanel.dev, "enable IB2 display\n");
    regmap_update_bits(vpanel.ib2_map,
    IB2_CTRL,
    IB2_CTRL_LCD_MASK,
    IB2_CTRL_LCD_BL_ON);
    }
    return 0;
    }
    static int versatile_panel_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct versatile_panel *vpanel = to_versatile_panel(panel);
    struct drm_display_mode *mode;
    connector.display_info.width_mm = vpanel.panel_type.width_mm;
    connector.display_info.height_mm = vpanel.panel_type.height_mm;
    connector.display_info.bus_flags = vpanel.panel_type.bus_flags;
    mode = drm_mode_duplicate(connector.dev, &vpanel.panel_type.mode);
    if (!mode)
    return -ENOMEM;
    drm_mode_set_name(mode);
    mode.type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED;
    mode.width_mm = vpanel.panel_type.width_mm;
    mode.height_mm = vpanel.panel_type.height_mm;
    drm_mode_probed_add(connector, mode);
    return 1;
    }
    static const struct drm_panel_funcs versatile_panel_drm_funcs = {
    .disable = versatile_panel_disable,
    .enable = versatile_panel_enable,
    .get_modes = versatile_panel_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn versatile_panel_probe(pdev: *mut platform_device) -> c_int {
    static int versatile_panel_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct versatile_panel *vpanel;
    struct device *parent;
    struct regmap *map;
    int ret;
    u32 val;
    int i;
    parent = dev.parent;
    if (!parent) {
    dev_err(dev, "no parent for versatile panel\n");
    return -ENODEV;
    }
    map = syscon_node_to_regmap(parent.of_node);
    if (IS_ERR(map)) {
    dev_err(dev, "no regmap for versatile panel parent\n");
    return PTR_ERR(map);
    }
    vpanel = devm_drm_panel_alloc(dev, struct versatile_panel, panel,
    &versatile_panel_drm_funcs,
    DRM_MODE_CONNECTOR_DPI);
    if (IS_ERR(vpanel))
    return PTR_ERR(vpanel);
    ret = regmap_read(map, SYS_CLCD, &val);
    if (ret) {
    dev_err(dev, "cannot access syscon regs\n");
    return ret;
    }
    val &= SYS_CLCD_CLCDID_MASK;
    for (i = 0; i < ARRAY_SIZE(versatile_panels); i++) {
    const struct versatile_panel_type *pt;
    pt = &versatile_panels[i];
    if (pt.magic == val) {
    vpanel.panel_type = pt;
    break;
    }
    }
// No panel detected or VGA, let's leave this show
    if (i == ARRAY_SIZE(versatile_panels)) {
    dev_info(dev, "no panel detected\n");
    return -ENODEV;
    }
    dev_info(dev, "detected: %s\n", vpanel.panel_type.name);
    vpanel.dev = dev;
    vpanel.map = map;
// Check if the panel is mounted on an IB2 daughterboard
    if (vpanel.panel_type.ib2) {
    vpanel.ib2_map = syscon_regmap_lookup_by_compatible(
    "arm,versatile-ib2-syscon");
    if (IS_ERR(vpanel.ib2_map))
    vpanel.ib2_map = core::ptr::null_mut();
    else
    dev_info(dev, "panel mounted on IB2 daughterboard\n");
    }
    drm_panel_add(&vpanel.panel);
    return 0;
    }
    static const struct of_device_id versatile_panel_match[] = {
    { .compatible = "arm,versatile-tft-panel", },
    {},
    };
    MODULE_DEVICE_TABLE(of, versatile_panel_match);
    static struct platform_driver versatile_panel_driver = {
    .probe		= versatile_panel_probe,
    .driver		= {
    .name	= "versatile-tft-panel",
    .of_match_table = versatile_panel_match,
    },
    };
    module_platform_driver(versatile_panel_driver);
    MODULE_AUTHOR("Linus Walleij <linus.walleij@linaro.org>");
    MODULE_DESCRIPTION("ARM Versatile panel driver");
    MODULE_LICENSE("GPL v2");
