//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/display/dvo_ns2501.c
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
// Copyright (c) 2012 Gilles Dartiguelongue, Thomas Richter
//
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
// IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub const NS2501_VID: c_uint = 0x1305;
pub const NS2501_DID: c_uint = 0x6726;
pub const NS2501_VID_LO: c_uint = 0x00;
pub const NS2501_VID_HI: c_uint = 0x01;
pub const NS2501_DID_LO: c_uint = 0x02;
pub const NS2501_DID_HI: c_uint = 0x03;
pub const NS2501_REV: c_uint = 0x04;
pub const NS2501_RSVD: c_uint = 0x05;
pub const NS2501_FREQ_LO: c_uint = 0x06;
pub const NS2501_FREQ_HI: c_uint = 0x07;
pub const NS2501_REG8: c_uint = 0x08;

pub const NS2501_REG9: c_uint = 0x09;

pub const NS2501_REGC: c_uint = 0x0c;
//
// The following registers are not part of the official datasheet
// and are the result of reverse engineering.
//
// Register c0 controls how the DVO synchronizes with
// its input.
//
pub const NS2501_REGC0: c_uint = 0xc0;

//
// Register 41 is somehow related to the sync register and sync
// configuration. It should be 0x32 whenever regC0 is 0x05 (hsync off)
// and 0x00 otherwise.
//
pub const NS2501_REG41: c_uint = 0x41;
//
// this register controls the dithering of the DVO
// One bit enables it, the other define the dithering depth.
// The higher the value, the lower the dithering depth.
//
pub const NS2501_F9_REG: c_uint = 0xf9;

//
// PLL configuration register. This is a pair of registers,
// one single byte register at 1B, and a pair at 1C,1D.
// These registers are counters/dividers.
//
pub const NS2501_REG1B: c_uint = 0x1b /* one byte PLL control register */;
pub const NS2501_REG1C: c_uint = 0x1c /* low-part of the second register */;
pub const NS2501_REG1D: c_uint = 0x1d /* high-part of the second register */;
//
// Scaler control registers. Horizontal at b8,b9,
// vertical at 10,11. The scale factor is computed as
// 2^16/control-value. The low-byte comes first.
//
pub const NS2501_REG10: c_uint = 0x10 /* low-byte vertical scaler */;
pub const NS2501_REG11: c_uint = 0x11 /* high-byte vertical scaler */;
pub const NS2501_REGB8: c_uint = 0xb8 /* low-byte horizontal scaler */;
pub const NS2501_REGB9: c_uint = 0xb9 /* high-byte horizontal scaler */;
//
// Display window definition. This consists of four registers
// per dimension. One register pair defines the start of the
// display, one the end.
// As far as I understand, this defines the window within which
// the scaler samples the input.
//
pub const NS2501_REGC1: c_uint = 0xc1 /* low-byte horizontal display start */;
pub const NS2501_REGC2: c_uint = 0xc2 /* high-byte horizontal display start */;
pub const NS2501_REGC3: c_uint = 0xc3 /* low-byte horizontal display stop */;
pub const NS2501_REGC4: c_uint = 0xc4 /* high-byte horizontal display stop */;
pub const NS2501_REGC5: c_uint = 0xc5 /* low-byte vertical display start */;
pub const NS2501_REGC6: c_uint = 0xc6 /* high-byte vertical display start */;
pub const NS2501_REGC7: c_uint = 0xc7 /* low-byte vertical display stop */;
pub const NS2501_REGC8: c_uint = 0xc8 /* high-byte vertical display stop */;
//
// The following register pair seems to define the start of
// the vertical sync. If automatic syncing is enabled, and the
// register value defines a sync pulse that is later than the
// incoming sync, then the register value is ignored and the
// external hsync triggers the synchronization.
//
pub const NS2501_REG80: c_uint = 0x80 /* low-byte vsync-start */;
pub const NS2501_REG81: c_uint = 0x81 /* high-byte vsync-start */;
//
// The following register pair seems to define the total number
// of lines created at the output side of the scaler.
// This is again a low-high register pair.
//
pub const NS2501_REG82: c_uint = 0x82 /* output display height, low byte */;
pub const NS2501_REG83: c_uint = 0x83 /* output display height, high byte */;
//
// The following registers define the end of the front-porch
// in horizontal and vertical position and hence allow to shift
// the image left/right or up/down.
//
pub const NS2501_REG98: c_uint = 0x98 /* horizontal start of display + 256, low */;
pub const NS2501_REG99: c_uint = 0x99 /* horizontal start of display + 256, high */;
pub const NS2501_REG8E: c_uint = 0x8e /* vertical start of the display, low byte */;
pub const NS2501_REG8F: c_uint = 0x8f /* vertical start of the display, high byte */;
//
// The following register pair control the function of the
// backlight and the DVO output. To enable the corresponding
// function, the corresponding bit must be set in both registers.
//
pub const NS2501_REG34: c_uint = 0x34 /* DVO enable functions, first register */;
pub const NS2501_REG35: c_uint = 0x35 /* DVO enable functions, second register */;

//
// Registers 9C and 9D define the vertical output offset
// of the visible region.
//
pub const NS2501_REG9C: c_uint = 0x9c;
pub const NS2501_REG9D: c_uint = 0x9d;
//
// The register 9F defines the dithering. This requires the
// scaler to be ON. Bit 0 enables dithering, the remaining
// bits control the depth of the dither. The higher the value,
// the LOWER the dithering amplitude. A good value seems to be
// 15 (total register value).
//
pub const NS2501_REGF9: c_uint = 0xf9;

    enum {
    MODE_640x480,
    MODE_800x600,
    MODE_1024x768,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns2501_reg {
    pub offset: u8,
    pub value: u8,
}

//
// The following structure keeps the complete configuration of
// the DVO, given a specific output configuration.
// This is pretty much guess-work from reverse-engineering, so
// read all this with a grain of salt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns2501_configuration {
    pub /: *mut *mut u8 sync; / configuration of the C0 register,
    pub /: *mut *mut u8 conf; / configuration register 8,
    pub /: *mut *mut u8 syncb; / configuration register 41,
    pub /: *mut *mut u8 dither; / configuration of the dithering,
    pub /: *mut *mut u8 pll_a; / PLL configuration, register A, 1B,
    pub /: *mut *mut u16 pll_b; / PLL configuration, register B, 1C/1D,
    pub /: *mut *mut u16 hstart; / horizontal start, registers C1/C2,
    pub /: *mut *mut u16 hstop; / horizontal total, registers C3/C4,
    pub /: *mut *mut u16 vstart; / vertical start, registers C5/C6,
    pub /: *mut *mut u16 vstop; / vertical total, registers C7/C8,
    pub /: *mut *mut u16 vsync; / manual vertical sync start, 80/81,
    pub /: *mut *mut u16 vtotal; / number of lines generated, 82/83,
    pub /: *mut *mut u16 hpos; / horizontal position + 256, 98/99,
    pub /: *mut *mut u16 vpos; / vertical position, 8e/8f,
    pub /: *mut *mut u16 voffs; / vertical output offset, 9c/9d,
    pub /: *mut *mut u16 hscale; / horizontal scaling factor, b8/b9,
    pub /: *mut *mut u16 vscale; / vertical scaling factor, 10/11,
}

//
// DVO configuration values, partially based on what the BIOS
// of the Fujitsu Lifebook S6010 writes into registers,
// partially found by manual tweaking. These configurations assume
// a 1024x768 panel.
//
    static const struct ns2501_configuration ns2501_modes[] = {
    [MODE_640x480] = {
    .sync	= NS2501_C0_ENABLE | NS2501_C0_VSYNC,
    .conf	= NS2501_8_VEN | NS2501_8_HEN | NS2501_8_PD,
    .syncb	= 0x32,
    .dither	= 0x0f,
    .pll_a	= 17,
    .pll_b	= 852,
    .hstart	= 144,
    .hstop	= 783,
    .vstart	= 22,
    .vstop	= 514,
    .vsync	= 2047, /* actually, ignored with this config */
    .vtotal	= 1341,
    .hpos	= 0,
    .vpos	= 16,
    .voffs	= 36,
    .hscale	= 40960,
    .vscale	= 40960
    },
    [MODE_800x600] = {
    .sync	= NS2501_C0_ENABLE |
    NS2501_C0_HSYNC | NS2501_C0_VSYNC,
    .conf   = NS2501_8_VEN | NS2501_8_HEN | NS2501_8_PD,
    .syncb	= 0x00,
    .dither	= 0x0f,
    .pll_a	= 25,
    .pll_b	= 612,
    .hstart	= 215,
    .hstop	= 1016,
    .vstart	= 26,
    .vstop	= 627,
    .vsync	= 807,
    .vtotal	= 1341,
    .hpos	= 0,
    .vpos	= 4,
    .voffs	= 35,
    .hscale	= 51248,
    .vscale	= 51232
    },
    [MODE_1024x768] = {
    .sync	= NS2501_C0_ENABLE | NS2501_C0_VSYNC,
    .conf   = NS2501_8_VEN | NS2501_8_HEN | NS2501_8_PD,
    .syncb	= 0x32,
    .dither	= 0x0f,
    .pll_a	= 11,
    .pll_b	= 1350,
    .hstart	= 276,
    .hstop	= 1299,
    .vstart	= 15,
    .vstop	= 1056,
    .vsync	= 2047,
    .vtotal	= 1341,
    .hpos	= 0,
    .vpos	= 7,
    .voffs	= 27,
    .hscale	= 65535,
    .vscale	= 65535
    }
    };
//
// Other configuration values left by the BIOS of the
// Fujitsu S6010 in the DVO control registers. Their
// value does not depend on the BIOS and their meaning
// is unknown.
//
    static const struct ns2501_reg mode_agnostic_values[] = {
// 08 is mode specific
    [0] = { .offset = 0x0a, .value = 0x81, },
// 10,11 are part of the mode specific configuration
    [1] = { .offset = 0x12, .value = 0x02, },
    [2] = { .offset = 0x18, .value = 0x07, },
    [3] = { .offset = 0x19, .value = 0x00, },
    [4] = { .offset = 0x1a, .value = 0x00, }, /* PLL?, ignored */
// 1b,1c,1d are part of the mode specific configuration
    [5] = { .offset = 0x1e, .value = 0x02, },
    [6] = { .offset = 0x1f, .value = 0x40, },
    [7] = { .offset = 0x20, .value = 0x00, },
    [8] = { .offset = 0x21, .value = 0x00, },
    [9] = { .offset = 0x22, .value = 0x00, },
    [10] = { .offset = 0x23, .value = 0x00, },
    [11] = { .offset = 0x24, .value = 0x00, },
    [12] = { .offset = 0x25, .value = 0x00, },
    [13] = { .offset = 0x26, .value = 0x00, },
    [14] = { .offset = 0x27, .value = 0x00, },
    [15] = { .offset = 0x7e, .value = 0x18, },
// 80-84 are part of the mode-specific configuration
    [16] = { .offset = 0x84, .value = 0x00, },
    [17] = { .offset = 0x85, .value = 0x00, },
    [18] = { .offset = 0x86, .value = 0x00, },
    [19] = { .offset = 0x87, .value = 0x00, },
    [20] = { .offset = 0x88, .value = 0x00, },
    [21] = { .offset = 0x89, .value = 0x00, },
    [22] = { .offset = 0x8a, .value = 0x00, },
    [23] = { .offset = 0x8b, .value = 0x00, },
    [24] = { .offset = 0x8c, .value = 0x10, },
    [25] = { .offset = 0x8d, .value = 0x02, },
// 8e,8f are part of the mode-specific configuration
    [26] = { .offset = 0x90, .value = 0xff, },
    [27] = { .offset = 0x91, .value = 0x07, },
    [28] = { .offset = 0x92, .value = 0xa0, },
    [29] = { .offset = 0x93, .value = 0x02, },
    [30] = { .offset = 0x94, .value = 0x00, },
    [31] = { .offset = 0x95, .value = 0x00, },
    [32] = { .offset = 0x96, .value = 0x05, },
    [33] = { .offset = 0x97, .value = 0x00, },
// 98,99 are part of the mode-specific configuration
    [34] = { .offset = 0x9a, .value = 0x88, },
    [35] = { .offset = 0x9b, .value = 0x00, },
// 9c,9d are part of the mode-specific configuration
    [36] = { .offset = 0x9e, .value = 0x25, },
    [37] = { .offset = 0x9f, .value = 0x03, },
    [38] = { .offset = 0xa0, .value = 0x28, },
    [39] = { .offset = 0xa1, .value = 0x01, },
    [40] = { .offset = 0xa2, .value = 0x28, },
    [41] = { .offset = 0xa3, .value = 0x05, },
// register 0xa4 is mode specific, but 0x80..0x84 works always
    [42] = { .offset = 0xa4, .value = 0x84, },
    [43] = { .offset = 0xa5, .value = 0x00, },
    [44] = { .offset = 0xa6, .value = 0x00, },
    [45] = { .offset = 0xa7, .value = 0x00, },
    [46] = { .offset = 0xa8, .value = 0x00, },
// 0xa9 to 0xab are mode specific, but have no visible effect
    [47] = { .offset = 0xa9, .value = 0x04, },
    [48] = { .offset = 0xaa, .value = 0x70, },
    [49] = { .offset = 0xab, .value = 0x4f, },
    [50] = { .offset = 0xac, .value = 0x00, },
    [51] = { .offset = 0xad, .value = 0x00, },
    [52] = { .offset = 0xb6, .value = 0x09, },
    [53] = { .offset = 0xb7, .value = 0x03, },
// b8,b9 are part of the mode-specific configuration
    [54] = { .offset = 0xba, .value = 0x00, },
    [55] = { .offset = 0xbb, .value = 0x20, },
    [56] = { .offset = 0xf3, .value = 0x90, },
    [57] = { .offset = 0xf4, .value = 0x00, },
    [58] = { .offset = 0xf7, .value = 0x88, },
// f8 is mode specific, but the value does not matter
    [59] = { .offset = 0xf8, .value = 0x0a, },
    [60] = { .offset = 0xf9, .value = 0x00, }
    };
    static const struct ns2501_reg regs_init[] = {
    [0] = { .offset = 0x35, .value = 0xff, },
    [1] = { .offset = 0x34, .value = 0x00, },
    [2] = { .offset = 0x08, .value = 0x30, },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns2501_priv {
    pub quiet: bool,
    pub conf: *const ns2501_configuration,
}

//
// Read a register from the ns2501.
// Returns true if successful, false otherwise.
// If it returns false, it might be wise to enable the
// DVO with the above function.
//
#[no_mangle]
unsafe extern "C" fn ns2501_readb(dvo: *mut intel_dvo_device, addr: c_int, ch: *mut u8) -> bool {
    static bool ns2501_readb(struct intel_dvo_device *dvo, int addr, u8 *ch)
    {
    struct ns2501_priv *ns = dvo.dev_priv;
    struct i2c_adapter *adapter = dvo.i2c_bus;
    u8 out_buf[2];
    u8 in_buf[2];
    struct i2c_msg msgs[] = {
    {
    .addr = dvo.target_addr,
    .flags = 0,
    .len = 1,
    .buf = out_buf,
    },
    {
    .addr = dvo.target_addr,
    .flags = I2C_M_RD,
    .len = 1,
    .buf = in_buf,
    }
    };
    out_buf[0] = addr;
    out_buf[1] = 0;
    if (i2c_transfer(adapter, msgs, 2) == 2) {
// ch = in_buf[0];
    return true;
    }
    if (!ns.quiet) {
    DRM_DEBUG_KMS
    ("Unable to read register 0x%02x from %s:0x%02x.\n", addr,
    adapter.name, dvo.target_addr);
    }
    return false;
    }
//
// Write a register to the ns2501.
// Returns true if successful, false otherwise.
// If it returns false, it might be wise to enable the
// DVO with the above function.
//
#[no_mangle]
unsafe extern "C" fn ns2501_writeb(dvo: *mut intel_dvo_device, addr: c_int, ch: u8) -> bool {
    static bool ns2501_writeb(struct intel_dvo_device *dvo, int addr, u8 ch)
    {
    struct ns2501_priv *ns = dvo.dev_priv;
    struct i2c_adapter *adapter = dvo.i2c_bus;
    u8 out_buf[2];
    struct i2c_msg msg = {
    .addr = dvo.target_addr,
    .flags = 0,
    .len = 2,
    .buf = out_buf,
    };
    out_buf[0] = addr;
    out_buf[1] = ch;
    if (i2c_transfer(adapter, &msg, 1) == 1) {
    return true;
    }
    if (!ns.quiet) {
    DRM_DEBUG_KMS("Unable to write register 0x%02x to %s:%d\n",
    addr, adapter.name, dvo.target_addr);
    }
    return false;
    }
// National Semiconductor 2501 driver for chip on i2c bus
// scan for the chip on the bus.
// Hope the VBIOS initialized the PLL correctly so we can
// talk to it. If not, it will not be seen and not detected.
// Bummer!
//
    static bool ns2501_init(struct intel_dvo_device *dvo,
    struct i2c_adapter *adapter)
    {
// this will detect the NS2501 chip on the specified i2c bus
    struct ns2501_priv *ns;
    unsigned char ch;
    ns = kzalloc_obj(*ns);
    if (ns == core::ptr::null_mut())
    return false;
    dvo.i2c_bus = adapter;
    dvo.dev_priv = ns;
    ns.quiet = true;
    if (!ns2501_readb(dvo, NS2501_VID_LO, &ch))
    goto out;
    if (ch != (NS2501_VID & 0xff)) {
    DRM_DEBUG_KMS("ns2501 not detected got %d: from %s Target %d.\n",
    ch, adapter.name, dvo.target_addr);
    goto out;
    }
    if (!ns2501_readb(dvo, NS2501_DID_LO, &ch))
    goto out;
    if (ch != (NS2501_DID & 0xff)) {
    DRM_DEBUG_KMS("ns2501 not detected got %d: from %s Target %d.\n",
    ch, adapter.name, dvo.target_addr);
    goto out;
    }
    ns.quiet = false;
    DRM_DEBUG_KMS("init ns2501 dvo controller successfully!\n");
    return true;
    out:
    kfree(ns);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ns2501_detect(dvo: *mut intel_dvo_device) -> enum drm_connector_status {
    static enum drm_connector_status ns2501_detect(struct intel_dvo_device *dvo)
    {
//
// This is a Laptop display, it doesn't have hotplugging.
// Even if not, the detection bit of the 2501 is unreliable as
// it only works for some display types.
// It is even more unreliable as the PLL must be active for
// allowing reading from the chip.
//
    return connector_status_connected;
    }
    static enum drm_mode_status ns2501_mode_valid(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode)
    {
    DRM_DEBUG_KMS
    ("is mode valid (hdisplay=%d,htotal=%d,vdisplay=%d,vtotal=%d)\n",
    mode.hdisplay, mode.htotal, mode.vdisplay, mode.vtotal);
//
// Currently, these are all the modes I have data from.
// More might exist. Unclear how to find the native resolution
// of the panel in here so we could always accept it
// by disabling the scaler.
//
    if ((mode.hdisplay == 640 && mode.vdisplay == 480 && mode.clock == 25175) ||
    (mode.hdisplay == 800 && mode.vdisplay == 600 && mode.clock == 40000) ||
    (mode.hdisplay == 1024 && mode.vdisplay == 768 && mode.clock == 65000)) {
    return MODE_OK;
    } else {
    return MODE_ONE_SIZE;	/* Is this a reasonable error? */
    }
    }
    static void ns2501_mode_set(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
    const struct ns2501_configuration *conf;
    struct ns2501_priv *ns = dvo.dev_priv;
    int mode_idx, i;
    DRM_DEBUG_KMS
    ("set mode (hdisplay=%d,htotal=%d,vdisplay=%d,vtotal=%d).\n",
    mode.hdisplay, mode.htotal, mode.vdisplay, mode.vtotal);
    DRM_DEBUG_KMS("Detailed requested mode settings are:\n"
    "clock		: %d kHz\n"
    "hdisplay	: %d\n"
    "hblank start	: %d\n"
    "hblank end	: %d\n"
    "hsync start	: %d\n"
    "hsync end	: %d\n"
    "htotal		: %d\n"
    "hskew		: %d\n"
    "vdisplay	: %d\n"
    "vblank start	: %d\n"
    "hblank end	: %d\n"
    "vsync start	: %d\n"
    "vsync end	: %d\n"
    "vtotal		: %d\n",
    adjusted_mode.crtc_clock,
    adjusted_mode.crtc_hdisplay,
    adjusted_mode.crtc_hblank_start,
    adjusted_mode.crtc_hblank_end,
    adjusted_mode.crtc_hsync_start,
    adjusted_mode.crtc_hsync_end,
    adjusted_mode.crtc_htotal,
    adjusted_mode.crtc_hskew,
    adjusted_mode.crtc_vdisplay,
    adjusted_mode.crtc_vblank_start,
    adjusted_mode.crtc_vblank_end,
    adjusted_mode.crtc_vsync_start,
    adjusted_mode.crtc_vsync_end,
    adjusted_mode.crtc_vtotal);
    if (mode.hdisplay == 640 && mode.vdisplay == 480)
    mode_idx = MODE_640x480;
#[no_mangle]
pub unsafe extern "C" fn if(600: mode->hdisplay == 800 && mode->vdisplay ==) -> else {
    else if (mode.hdisplay == 800 && mode.vdisplay == 600)
    mode_idx = MODE_800x600;
#[no_mangle]
pub unsafe extern "C" fn if(768: mode->hdisplay == 1024 && mode->vdisplay ==) -> else {
    else if (mode.hdisplay == 1024 && mode.vdisplay == 768)
    mode_idx = MODE_1024x768;
    else
    return;
// Hopefully doing it every time won't hurt...
    for (i = 0; i < ARRAY_SIZE(regs_init); i++)
    ns2501_writeb(dvo, regs_init[i].offset, regs_init[i].value);
// Write the mode-agnostic values
    for (i = 0; i < ARRAY_SIZE(mode_agnostic_values); i++)
    ns2501_writeb(dvo, mode_agnostic_values[i].offset,
    mode_agnostic_values[i].value);
// Write now the mode-specific configuration
    conf = ns2501_modes + mode_idx;
    ns.conf = conf;
    ns2501_writeb(dvo, NS2501_REG8, conf.conf);
    ns2501_writeb(dvo, NS2501_REG1B, conf.pll_a);
    ns2501_writeb(dvo, NS2501_REG1C, conf.pll_b & 0xff);
    ns2501_writeb(dvo, NS2501_REG1D, conf.pll_b >> 8);
    ns2501_writeb(dvo, NS2501_REGC1, conf.hstart & 0xff);
    ns2501_writeb(dvo, NS2501_REGC2, conf.hstart >> 8);
    ns2501_writeb(dvo, NS2501_REGC3, conf.hstop & 0xff);
    ns2501_writeb(dvo, NS2501_REGC4, conf.hstop >> 8);
    ns2501_writeb(dvo, NS2501_REGC5, conf.vstart & 0xff);
    ns2501_writeb(dvo, NS2501_REGC6, conf.vstart >> 8);
    ns2501_writeb(dvo, NS2501_REGC7, conf.vstop & 0xff);
    ns2501_writeb(dvo, NS2501_REGC8, conf.vstop >> 8);
    ns2501_writeb(dvo, NS2501_REG80, conf.vsync & 0xff);
    ns2501_writeb(dvo, NS2501_REG81, conf.vsync >> 8);
    ns2501_writeb(dvo, NS2501_REG82, conf.vtotal & 0xff);
    ns2501_writeb(dvo, NS2501_REG83, conf.vtotal >> 8);
    ns2501_writeb(dvo, NS2501_REG98, conf.hpos & 0xff);
    ns2501_writeb(dvo, NS2501_REG99, conf.hpos >> 8);
    ns2501_writeb(dvo, NS2501_REG8E, conf.vpos & 0xff);
    ns2501_writeb(dvo, NS2501_REG8F, conf.vpos >> 8);
    ns2501_writeb(dvo, NS2501_REG9C, conf.voffs & 0xff);
    ns2501_writeb(dvo, NS2501_REG9D, conf.voffs >> 8);
    ns2501_writeb(dvo, NS2501_REGB8, conf.hscale & 0xff);
    ns2501_writeb(dvo, NS2501_REGB9, conf.hscale >> 8);
    ns2501_writeb(dvo, NS2501_REG10, conf.vscale & 0xff);
    ns2501_writeb(dvo, NS2501_REG11, conf.vscale >> 8);
    ns2501_writeb(dvo, NS2501_REGF9, conf.dither);
    ns2501_writeb(dvo, NS2501_REG41, conf.syncb);
    ns2501_writeb(dvo, NS2501_REGC0, conf.sync);
    }
// set the NS2501 power state
#[no_mangle]
unsafe extern "C" fn ns2501_get_hw_state(dvo: *mut intel_dvo_device) -> bool {
    static bool ns2501_get_hw_state(struct intel_dvo_device *dvo)
    {
    unsigned char ch;
    if (!ns2501_readb(dvo, NS2501_REG8, &ch))
    return false;
    return ch & NS2501_8_PD;
    }
// set the NS2501 power state
#[no_mangle]
unsafe extern "C" fn ns2501_dpms(dvo: *mut intel_dvo_device, enable: bool) {
    static void ns2501_dpms(struct intel_dvo_device *dvo, bool enable)
    {
    struct ns2501_priv *ns = dvo.dev_priv;
    DRM_DEBUG_KMS("Trying set the dpms of the DVO to %i\n", enable);
    if (enable) {
    ns2501_writeb(dvo, NS2501_REGC0, ns.conf.sync | 0x08);
    ns2501_writeb(dvo, NS2501_REG41, ns.conf.syncb);
    ns2501_writeb(dvo, NS2501_REG34, NS2501_34_ENABLE_OUTPUT);
    msleep(15);
    ns2501_writeb(dvo, NS2501_REG8,
    ns.conf.conf | NS2501_8_BPAS);
    if (!(ns.conf.conf & NS2501_8_BPAS))
    ns2501_writeb(dvo, NS2501_REG8, ns.conf.conf);
    msleep(200);
    ns2501_writeb(dvo, NS2501_REG34,
    NS2501_34_ENABLE_OUTPUT | NS2501_34_ENABLE_BACKLIGHT);
    ns2501_writeb(dvo, NS2501_REGC0, ns.conf.sync);
    } else {
    ns2501_writeb(dvo, NS2501_REG34, NS2501_34_ENABLE_OUTPUT);
    msleep(200);
    ns2501_writeb(dvo, NS2501_REG8, NS2501_8_VEN | NS2501_8_HEN |
    NS2501_8_BPAS);
    msleep(15);
    ns2501_writeb(dvo, NS2501_REG34, 0x00);
    }
    }
#[no_mangle]
unsafe extern "C" fn ns2501_destroy(dvo: *mut intel_dvo_device) {
    static void ns2501_destroy(struct intel_dvo_device *dvo)
    {
    struct ns2501_priv *ns = dvo.dev_priv;
    if (ns) {
    kfree(ns);
    dvo.dev_priv = core::ptr::null_mut();
    }
    }
    const struct intel_dvo_dev_ops ns2501_ops = {
    .init = ns2501_init,
    .detect = ns2501_detect,
    .mode_valid = ns2501_mode_valid,
    .mode_set = ns2501_mode_set,
    .dpms = ns2501_dpms,
    .get_hw_state = ns2501_get_hw_state,
    .destroy = ns2501_destroy,
    };
