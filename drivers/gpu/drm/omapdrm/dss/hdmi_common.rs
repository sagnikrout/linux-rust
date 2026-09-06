//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/omapdrm/dss/hdmi_common.c
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

    int hdmi_parse_lanes_of(struct platform_device *pdev, struct device_node *ep,
    struct hdmi_phy_data *phy)
    {
    struct property *prop;
    int r, len;
    prop = of_find_property(ep, "lanes", &len);
    if (prop) {
    u32 lanes[8];
    if (len / sizeof(u32) != ARRAY_SIZE(lanes)) {
    dev_err(&pdev.dev, "bad number of lanes\n");
    return -EINVAL;
    }
    r = of_property_read_u32_array(ep, "lanes", lanes,
    ARRAY_SIZE(lanes));
    if (r) {
    dev_err(&pdev.dev, "failed to read lane data\n");
    return r;
    }
    r = hdmi_phy_parse_lanes(phy, lanes);
    if (r) {
    dev_err(&pdev.dev, "failed to parse lane data\n");
    return r;
    }
    } else {
    static const u32 default_lanes[] = { 0, 1, 2, 3, 4, 5, 6, 7 };
    r = hdmi_phy_parse_lanes(phy, default_lanes);
    if (WARN_ON(r)) {
    dev_err(&pdev.dev, "failed to parse lane data\n");
    return r;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hdmi_compute_acr(pclk: u32, sample_freq: u32, n: *mut u32, cts: *mut u32) -> c_int {
    int hdmi_compute_acr(u32 pclk, u32 sample_freq, u32 *n, u32 *cts)
    {
    u32 deep_color;
    let mut deep_color_correct: bool = false;
    if (n == core::ptr::null_mut() || cts == core::ptr::null_mut())
    return -EINVAL;
// TODO: When implemented, query deep color mode here.
    deep_color = 100;
//
// When using deep color, the default N value (as in the HDMI
// specification) yields to an non-integer CTS. Hence, we
// modify it while keeping the restrictions described in
// section 7.2.1 of the HDMI 1.4a specification.
//
    switch (sample_freq) {
    case 32000:
    case 48000:
    case 96000:
    case 192000:
    if (deep_color == 125)
    if (pclk == 27027000 || pclk == 74250000)
    deep_color_correct = true;
    if (deep_color == 150)
    if (pclk == 27027000)
    deep_color_correct = true;
    break;
    case 44100:
    case 88200:
    case 176400:
    if (deep_color == 125)
    if (pclk == 27027000)
    deep_color_correct = true;
    break;
    default:
    return -EINVAL;
    }
    if (deep_color_correct) {
    switch (sample_freq) {
    case 32000:
// n = 8192;
    break;
    case 44100:
// n = 12544;
    break;
    case 48000:
// n = 8192;
    break;
    case 88200:
// n = 25088;
    break;
    case 96000:
// n = 16384;
    break;
    case 176400:
// n = 50176;
    break;
    case 192000:
// n = 32768;
    break;
    default:
    return -EINVAL;
    }
    } else {
    switch (sample_freq) {
    case 32000:
// n = 4096;
    break;
    case 44100:
// n = 6272;
    break;
    case 48000:
// n = 6144;
    break;
    case 88200:
// n = 12544;
    break;
    case 96000:
// n = 12288;
    break;
    case 176400:
// n = 25088;
    break;
    case 192000:
// n = 24576;
    break;
    default:
    return -EINVAL;
    }
    }
// Calculate CTS. See HDMI 1.3a or 1.4a specifications
// cts = (pclk/1000) * (*n / 128) * deep_color / (sample_freq / 10);
    return 0;
    }
