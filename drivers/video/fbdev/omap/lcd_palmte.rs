//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap/lcd_palmte.c
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
// LCD panel support for the Palm Tungsten E
//
// Original version : Romain Goyet <r.goyet@gmail.com>
// Current version : Laurent Gonzalez <palmte.linux@free.fr>
//

    static struct lcd_panel palmte_panel = {
    .name		= "palmte",
    .config		= OMAP_LCDC_PANEL_TFT | OMAP_LCDC_INV_VSYNC |
    OMAP_LCDC_INV_HSYNC | OMAP_LCDC_HSVS_RISING_EDGE |
    OMAP_LCDC_HSVS_OPPOSITE,
    .data_lines	= 16,
    .bpp		= 8,
    .pixel_clock	= 12000,
    .x_res		= 320,
    .y_res		= 320,
    .hsw		= 4,
    .hfp		= 8,
    .hbp		= 28,
    .vsw		= 1,
    .vfp		= 8,
    .vbp		= 7,
    .pcd		= 0,
    };
#[no_mangle]
unsafe extern "C" fn palmte_panel_probe(pdev: *mut platform_device) -> c_int {
    static int palmte_panel_probe(struct platform_device *pdev)
    {
    omapfb_register_panel(&palmte_panel);
    return 0;
    }
    static struct platform_driver palmte_panel_driver = {
    .probe		= palmte_panel_probe,
    .driver		= {
    .name	= "lcd_palmte",
    },
    };
    module_platform_driver(palmte_panel_driver);
    MODULE_AUTHOR("Romain Goyet <r.goyet@gmail.com>, Laurent Gonzalez <palmte.linux@free.fr>");
    MODULE_DESCRIPTION("LCD panel support for the Palm Tungsten E");
    MODULE_LICENSE("GPL");
