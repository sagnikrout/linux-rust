//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fbcvt.c
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
// linux/drivers/video/fbcvt.c - VESA(TM) Coordinated Video Timings
//
// Copyright (C) 2005 Antonino Daplas <adaplas@pol.net>
//
// Based from the VESA(TM) Coordinated Video Timing Generator by
// Graham Loveridge April 9, 2003 available at
// http://www.elo.utfsm.cl/~elo212/docs/CVTd6r1.xls
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

pub const FB_CVT_CELLSIZE: c_int = 8;
pub const FB_CVT_GTF_C: c_int = 40;
pub const FB_CVT_GTF_J: c_int = 20;
pub const FB_CVT_GTF_K: c_int = 128;
pub const FB_CVT_GTF_M: c_int = 600;
pub const FB_CVT_MIN_VSYNC_BP: c_int = 550;
pub const FB_CVT_MIN_VPORCH: c_int = 3;
pub const FB_CVT_MIN_BPORCH: c_int = 6;
pub const FB_CVT_RB_MIN_VBLANK: c_int = 460;
pub const FB_CVT_RB_HBLANK: c_int = 160;
pub const FB_CVT_RB_V_FPORCH: c_int = 3;
pub const FB_CVT_FLAG_REDUCED_BLANK: c_int = 1;
pub const FB_CVT_FLAG_MARGINS: c_int = 2;
pub const FB_CVT_FLAG_INTERLACED: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_cvt_data {
    pub xres: u32,
    pub yres: u32,
    pub refresh: u32,
    pub f_refresh: u32,
    pub pixclock: u32,
    pub hperiod: u32,
    pub hblank: u32,
    pub hfreq: u32,
    pub htotal: u32,
    pub vtotal: u32,
    pub vsync: u32,
    pub hsync: u32,
    pub h_front_porch: u32,
    pub h_back_porch: u32,
    pub v_front_porch: u32,
    pub v_back_porch: u32,
    pub h_margin: u32,
    pub v_margin: u32,
    pub interlace: u32,
    pub aspect_ratio: u32,
    pub active_pixels: u32,
    pub flags: u32,
    pub status: u32,
}

    static const unsigned char fb_cvt_vbi_tab[] = {
    4,        /* 4:3      */
    5,        /* 16:9     */
    6,        /* 16:10    */
    7,        /* 5:4      */
    7,        /* 15:9     */
    8,        /* reserved */
    9,        /* reserved */
    10        /* custom   */
    };
// returns hperiod * 1000
#[no_mangle]
unsafe extern "C" fn fb_cvt_hperiod(cvt: *mut fb_cvt_data) -> u32 {
    static u32 fb_cvt_hperiod(struct fb_cvt_data *cvt)
    {
    let mut num: u32 = 1000000000/cvt.f_refresh;
    u32 den;
    if (cvt.flags & FB_CVT_FLAG_REDUCED_BLANK) {
    num -= FB_CVT_RB_MIN_VBLANK * 1000;
    den = 2 * (cvt.yres/cvt.interlace + 2 * cvt.v_margin);
    } else {
    num -= FB_CVT_MIN_VSYNC_BP * 1000;
    den = 2 * (cvt.yres/cvt.interlace + cvt.v_margin * 2
    + FB_CVT_MIN_VPORCH + cvt.interlace/2);
    }
    return 2 * (num/den);
    }
// returns ideal duty cycle * 1000
#[no_mangle]
unsafe extern "C" fn fb_cvt_ideal_duty_cycle(cvt: *mut fb_cvt_data) -> u32 {
    static u32 fb_cvt_ideal_duty_cycle(struct fb_cvt_data *cvt)
    {
    u32 c_prime = (FB_CVT_GTF_C - FB_CVT_GTF_J) *
    (FB_CVT_GTF_K) + 256 * FB_CVT_GTF_J;
    let mut m_prime: u32 = (FB_CVT_GTF_K * FB_CVT_GTF_M);
    let mut h_period_est: u32 = cvt.hperiod;
    return (1000 * c_prime  - ((m_prime * h_period_est)/1000))/256;
    }
#[no_mangle]
unsafe extern "C" fn fb_cvt_hblank(cvt: *mut fb_cvt_data) -> u32 {
    static u32 fb_cvt_hblank(struct fb_cvt_data *cvt)
    {
    let mut hblank: u32 = 0;
    if (cvt.flags & FB_CVT_FLAG_REDUCED_BLANK)
    hblank = FB_CVT_RB_HBLANK;
    else {
    let mut ideal_duty_cycle: u32 = fb_cvt_ideal_duty_cycle(cvt);
    let mut active_pixels: u32 = cvt.active_pixels;
    if (ideal_duty_cycle < 20000)
    hblank = (active_pixels * 20000)/
    (100000 - 20000);
    else {
    hblank = (active_pixels * ideal_duty_cycle)/
    (100000 - ideal_duty_cycle);
    }
    }
    hblank &= ~((2 * FB_CVT_CELLSIZE) - 1);
    return hblank;
    }
#[no_mangle]
unsafe extern "C" fn fb_cvt_hsync(cvt: *mut fb_cvt_data) -> u32 {
    static u32 fb_cvt_hsync(struct fb_cvt_data *cvt)
    {
    u32 hsync;
    if (cvt.flags & FB_CVT_FLAG_REDUCED_BLANK)
    hsync = 32;
    else
    hsync = (FB_CVT_CELLSIZE * cvt.htotal)/100;
    hsync &= ~(FB_CVT_CELLSIZE - 1);
    return hsync;
    }
#[no_mangle]
unsafe extern "C" fn fb_cvt_vbi_lines(cvt: *mut fb_cvt_data) -> u32 {
    static u32 fb_cvt_vbi_lines(struct fb_cvt_data *cvt)
    {
    u32 vbi_lines, min_vbi_lines, act_vbi_lines;
    if (cvt.flags & FB_CVT_FLAG_REDUCED_BLANK) {
    vbi_lines = (1000 * FB_CVT_RB_MIN_VBLANK)/cvt.hperiod + 1;
    min_vbi_lines =  FB_CVT_RB_V_FPORCH + cvt.vsync +
    FB_CVT_MIN_BPORCH;
    } else {
    vbi_lines = (FB_CVT_MIN_VSYNC_BP * 1000)/cvt.hperiod + 1 +
    FB_CVT_MIN_VPORCH;
    min_vbi_lines = cvt.vsync + FB_CVT_MIN_BPORCH +
    FB_CVT_MIN_VPORCH;
    }
    if (vbi_lines < min_vbi_lines)
    act_vbi_lines = min_vbi_lines;
    else
    act_vbi_lines = vbi_lines;
    return act_vbi_lines;
    }
#[no_mangle]
unsafe extern "C" fn fb_cvt_vtotal(cvt: *mut fb_cvt_data) -> u32 {
    static u32 fb_cvt_vtotal(struct fb_cvt_data *cvt)
    {
    let mut vtotal: u32 = cvt.yres/cvt.interlace;
    vtotal += 2 * cvt.v_margin + cvt.interlace/2 + fb_cvt_vbi_lines(cvt);
    vtotal |= cvt.interlace/2;
    return vtotal;
    }
#[no_mangle]
unsafe extern "C" fn fb_cvt_pixclock(cvt: *mut fb_cvt_data) -> u32 {
    static u32 fb_cvt_pixclock(struct fb_cvt_data *cvt)
    {
    u32 pixclock;
    if (cvt.flags & FB_CVT_FLAG_REDUCED_BLANK)
    pixclock = (cvt.f_refresh * cvt.vtotal * cvt.htotal)/1000;
    else
    pixclock = (cvt.htotal * 1000000)/cvt.hperiod;
    pixclock /= 250;
    pixclock *= 250;
    pixclock *= 1000;
    return pixclock;
    }
#[no_mangle]
unsafe extern "C" fn fb_cvt_aspect_ratio(cvt: *mut fb_cvt_data) -> u32 {
    static u32 fb_cvt_aspect_ratio(struct fb_cvt_data *cvt)
    {
    let mut xres: u32 = cvt.xres;
    let mut yres: u32 = cvt.yres;
    let mut aspect: u32 = -1;
    if (xres == (yres * 4)/3 && !((yres * 4) % 3))
    aspect = 0;
#[no_mangle]
pub unsafe extern "C" fn if(9): *mut *mut *mut xres == (yres  16)/9 && !((yres  16) %) -> else {
    else if (xres == (yres * 16)/9 && !((yres * 16) % 9))
    aspect = 1;
#[no_mangle]
pub unsafe extern "C" fn if(10): *mut *mut *mut xres == (yres  16)/10 && !((yres  16) %) -> else {
    else if (xres == (yres * 16)/10 && !((yres * 16) % 10))
    aspect = 2;
#[no_mangle]
pub unsafe extern "C" fn if(4): *mut *mut *mut xres == (yres  5)/4 && !((yres  5) %) -> else {
    else if (xres == (yres * 5)/4 && !((yres * 5) % 4))
    aspect = 3;
#[no_mangle]
pub unsafe extern "C" fn if(9): *mut *mut *mut xres == (yres  15)/9 && !((yres  15) %) -> else {
    else if (xres == (yres * 15)/9 && !((yres * 15) % 9))
    aspect = 4;
    else {
    printk(KERN_INFO "fbcvt: Aspect ratio not CVT "
    "standard\n");
    aspect = 7;
    cvt.status = 1;
    }
    return aspect;
    }
#[no_mangle]
unsafe extern "C" fn fb_cvt_print_name(cvt: *mut fb_cvt_data) {
    static void fb_cvt_print_name(struct fb_cvt_data *cvt)
    {
    u32 pixcount, pixcount_mod;
    let mut size: c_int = 256;
    let mut off: c_int = 0;
    u8 *buf;
    buf = kzalloc(size, GFP_KERNEL);
    if (!buf)
    return;
    pixcount = (cvt.xres * (cvt.yres/cvt.interlace))/1000000;
    pixcount_mod = (cvt.xres * (cvt.yres/cvt.interlace)) % 1000000;
    pixcount_mod /= 1000;
    off += scnprintf(buf + off, size - off, "fbcvt: %dx%d@%d: CVT Name - ",
    cvt.xres, cvt.yres, cvt.refresh);
    if (cvt.status) {
    off += scnprintf(buf + off, size - off,
    "Not a CVT standard - %d.%03d Mega Pixel Image\n",
    pixcount, pixcount_mod);
    } else {
    if (pixcount)
    off += scnprintf(buf + off, size - off, "%d", pixcount);
    off += scnprintf(buf + off, size - off, ".%03dM", pixcount_mod);
    if (cvt.aspect_ratio == 0)
    off += scnprintf(buf + off, size - off, "3");
#[no_mangle]
pub unsafe extern "C" fn if(3: cvt->aspect_ratio ==) -> else {
    else if (cvt.aspect_ratio == 3)
    off += scnprintf(buf + off, size - off, "4");
#[no_mangle]
pub unsafe extern "C" fn if(4: cvt->aspect_ratio == 1 || cvt->aspect_ratio ==) -> else {
    else if (cvt.aspect_ratio == 1 || cvt.aspect_ratio == 4)
    off += scnprintf(buf + off, size - off, "9");
#[no_mangle]
pub unsafe extern "C" fn if(2: cvt->aspect_ratio ==) -> else {
    else if (cvt.aspect_ratio == 2)
    off += scnprintf(buf + off, size - off, "A");
    if (cvt.flags & FB_CVT_FLAG_REDUCED_BLANK)
    off += scnprintf(buf + off, size - off, "-R");
    }
    printk(KERN_INFO "%s\n", buf);
    kfree(buf);
    }
    static void fb_cvt_convert_to_mode(struct fb_cvt_data *cvt,
    struct fb_videomode *mode)
    {
    mode.refresh = cvt.f_refresh;
    mode.pixclock = KHZ2PICOS(cvt.pixclock/1000);
    mode.left_margin = cvt.h_back_porch;
    mode.right_margin = cvt.h_front_porch;
    mode.hsync_len = cvt.hsync;
    mode.upper_margin = cvt.v_back_porch;
    mode.lower_margin = cvt.v_front_porch;
    mode.vsync_len = cvt.vsync;
    mode.sync &= ~(FB_SYNC_HOR_HIGH_ACT | FB_SYNC_VERT_HIGH_ACT);
    if (cvt.flags & FB_CVT_FLAG_REDUCED_BLANK)
    mode.sync |= FB_SYNC_HOR_HIGH_ACT;
    else
    mode.sync |= FB_SYNC_VERT_HIGH_ACT;
    }
//
// fb_find_mode_cvt - calculate mode using VESA(TM) CVT
// @mode: pointer to fb_videomode; xres, yres, refresh and vmode must be
// pre-filled with the desired values
// @margins: add margin to calculation (1.8% of xres and yres)
// @rb: compute with reduced blanking (for flatpanels)
//
// RETURNS:
// 0 for success
// @mode is filled with computed values.  If interlaced, the refresh field
// will be filled with the field rate (2x the frame rate)
//
// DESCRIPTION:
// Computes video timings using VESA(TM) Coordinated Video Timings
//
#[no_mangle]
pub unsafe extern "C" fn fb_find_mode_cvt(mode: *mut fb_videomode, margins: c_int, rb: c_int) -> c_int {
    int fb_find_mode_cvt(struct fb_videomode *mode, int margins, int rb)
    {
    struct fb_cvt_data cvt;
    memset(&cvt, 0, sizeof(cvt));
    if (margins)
    cvt.flags |= FB_CVT_FLAG_MARGINS;
    if (rb)
    cvt.flags |= FB_CVT_FLAG_REDUCED_BLANK;
    if (mode.vmode & FB_VMODE_INTERLACED)
    cvt.flags |= FB_CVT_FLAG_INTERLACED;
    cvt.xres = mode.xres;
    cvt.yres = mode.yres;
    cvt.refresh = mode.refresh;
    cvt.f_refresh = cvt.refresh;
    cvt.interlace = 1;
    if (!cvt.xres || !cvt.yres || !cvt.refresh || cvt.f_refresh > INT_MAX) {
    printk(KERN_INFO "fbcvt: Invalid input parameters\n");
    return 1;
    }
    if (!(cvt.refresh == 50 || cvt.refresh == 60 || cvt.refresh == 70 ||
    cvt.refresh == 85)) {
    printk(KERN_INFO "fbcvt: Refresh rate not CVT "
    "standard\n");
    cvt.status = 1;
    }
    cvt.xres &= ~(FB_CVT_CELLSIZE - 1);
    if (cvt.flags & FB_CVT_FLAG_INTERLACED) {
    cvt.interlace = 2;
    cvt.f_refresh *= 2;
    }
    if (cvt.flags & FB_CVT_FLAG_REDUCED_BLANK) {
    if (cvt.refresh != 60) {
    printk(KERN_INFO "fbcvt: 60Hz refresh rate "
    "advised for reduced blanking\n");
    cvt.status = 1;
    }
    }
    if (cvt.flags & FB_CVT_FLAG_MARGINS) {
    cvt.h_margin = (cvt.xres * 18)/1000;
    cvt.h_margin &= ~(FB_CVT_CELLSIZE - 1);
    cvt.v_margin = ((cvt.yres/cvt.interlace)* 18)/1000;
    }
    cvt.aspect_ratio = fb_cvt_aspect_ratio(&cvt);
    cvt.active_pixels = cvt.xres + 2 * cvt.h_margin;
    cvt.hperiod = fb_cvt_hperiod(&cvt);
    cvt.vsync = fb_cvt_vbi_tab[cvt.aspect_ratio];
    cvt.vtotal = fb_cvt_vtotal(&cvt);
    cvt.hblank = fb_cvt_hblank(&cvt);
    cvt.htotal = cvt.active_pixels + cvt.hblank;
    cvt.hsync = fb_cvt_hsync(&cvt);
    cvt.pixclock = fb_cvt_pixclock(&cvt);
    cvt.hfreq = cvt.pixclock/cvt.htotal;
    cvt.h_back_porch = cvt.hblank/2 + cvt.h_margin;
    cvt.h_front_porch = cvt.hblank - cvt.hsync - cvt.h_back_porch +
    2 * cvt.h_margin;
    cvt.v_front_porch = 3 + cvt.v_margin;
    cvt.v_back_porch = cvt.vtotal - cvt.yres/cvt.interlace -
    cvt.v_front_porch - cvt.vsync;
    fb_cvt_print_name(&cvt);
    fb_cvt_convert_to_mode(&cvt, mode);
    return 0;
    }
