//! Automatically rewritten from C to Rust
//! Source: tools/platform/x86/amd/test-pmf.c
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
// AMD Platform Management Framework Test Tool
//
// Copyright (c) 2026, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Authors: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
// Sanket Goswami <Sanket.Goswami@amd.com>
//

// Feature flag names
    static const char * const feature_names[] = {
    "Auto Mode",
    "Static Power Slider",
    "Policy Builder (Smart PC)",
    "Dynamic Power Slider AC",
    "Dynamic Power Slider DC",
    };
    static const char *banner =
    "====================================================\n"
    "      AMD PMF Metrics info and Feature Status\n"
    "====================================================\n\n";
// Print feature flags
#[no_mangle]
unsafe extern "C" fn pmf_print_features(flags: u32) {
    static void pmf_print_features(uint32_t flags)
    {
    size_t i;
    for (i = 0; i < ARRAY_SIZE(feature_names); i++)
    printf("  [%c] %s\n", (flags & (1U << i)) ? 'x' : ' ', feature_names[i]);
    }
// Print BIOS parameters
#[no_mangle]
unsafe extern "C" fn pmf_print_bios_params(type: *const c_char, params: *const __u32) {
    static void pmf_print_bios_params(const char *type, const __u32 *params)
    {
    int i;
    for (i = 0; i < AMD_PMF_BIOS_PARAMS_MAX; i++)
    printf("  Custom BIOS %s%d: %u\n", type, i + 1, params[i]);
    }
// Open the PMF device
#[no_mangle]
unsafe extern "C" fn pmf_open_device() -> c_int {
    static int pmf_open_device(void)
    {
    int fd;
    fd = open(DEVICE_NODE, O_RDONLY);
    if (fd < 0)
    fprintf(stderr, "Error: Cannot open %s: %s\n", DEVICE_NODE, strerror(errno));
    return fd;
    }
// Query PMF info using the single IOCTL
#[no_mangle]
unsafe extern "C" fn pmf_get_info(fd: c_int, info: *mut amd_pmf_info) -> c_int {
    static int pmf_get_info(int fd, struct amd_pmf_info *info)
    {
    int ret;
// Zero-initialize and set size for versioning
    memset(info, 0, sizeof(*info));
    info.size = sizeof(*info);
    ret = ioctl(fd, IOCTL_AMD_PMF_POPULATE_DATA, info);
    if (ret < 0) {
    fprintf(stderr, "Error: IOCTL_AMD_PMF_POPULATE_DATA failed: %s\n", strerror(errno));
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmf_print_info(info: *const amd_pmf_info) {
    static void pmf_print_info(const struct amd_pmf_info *info)
    {
    printf("%s", banner);
// Feature status
    printf("Feature Status:\n");
    pmf_print_features(info.features_supported);
// Device states
    printf("\nDevice States:\n");
    printf("  Platform Type:     %s\n", amd_pmf_get_platform_type(info.platform_type));
    printf("  Laptop Placement:  %s\n", amd_pmf_get_laptop_placement(info.laptop_placement));
    printf("  Lid State:         %s\n", info.lid_state ? "Closed" : "Open");
    printf("  User Presence:     %s\n", info.user_presence ? "Present" : "Away");
    printf("  Slider Position:   %s\n", amd_pmf_get_slider_position(info.slider_position));
// Thermal and power metrics
    printf("\nThermal/Power Metrics:\n");
    printf("  Skin Temperature:  %d\n", info.skin_temp / 100);
    printf("  GFX Busy:          %u\n", info.gfx_busy);
    printf("  Ambient Light:     %d\n", info.ambient_light);
    printf("  Avg C0 Residency:  %u\n", info.avg_c0_residency);
    printf("  Max C0 Residency:  %u\n", info.max_c0_residency);
    printf("  Socket Power:      %u\n", info.socket_power);
// BIOS parameters
    printf("\nCustom BIOS Input Parameters:\n");
    pmf_print_bios_params("Input", info.bios_input);
    printf("\nCustom BIOS Output Parameters:\n");
    pmf_print_bios_params("Output", info.bios_output);
    printf("\n=================================================\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct amd_pmf_info info;
    int fd, ret;
    fd = pmf_open_device();
    if (fd < 0)
    return -1;
// Query all info with single IOCTL
    ret = pmf_get_info(fd, &info);
    close(fd);
    if (ret < 0)
    return -1;
    pmf_print_info(&info);
    return 0;
    }
