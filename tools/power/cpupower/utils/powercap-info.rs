//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/powercap-info.c
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
// (C) 2016 SUSE Software Solutions GmbH
// Thomas Renninger <trenn@suse.de>
//

    int powercap_show_all;
    static struct option info_opts[] = {
    { "all",		no_argument,		 core::ptr::null_mut(),	 'a'},
    { },
    };
#[no_mangle]
unsafe extern "C" fn powercap_print_one_zone(zone: *mut powercap_zone) -> c_int {
    static int powercap_print_one_zone(struct powercap_zone *zone)
    {
    int mode, i, ret = 0;
    char pr_prefix[1024] = "";
    for (i = 0; i < zone.tree_depth && i < POWERCAP_MAX_TREE_DEPTH; i++)
    strcat(pr_prefix, "\t");
    printf("%sZone: %s", pr_prefix, zone.name);
    ret = powercap_zone_get_enabled(zone, &mode);
    if (ret < 0)
    return ret;
    printf(" (%s)\n", mode ? "enabled" : "disabled");
    if (zone.has_power_uw)
    printf(_("%sPower can be monitored in micro Watts\n"),
    pr_prefix);
    if (zone.has_energy_uj)
    printf(_("%sPower can be monitored in micro Jules\n"),
    pr_prefix);
    printf("\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn powercap_show() -> c_int {
    static int powercap_show(void)
    {
    struct powercap_zone *root_zone;
    char line[MAX_LINE_LEN] = "";
    int ret, val;
    ret = powercap_get_driver(line, MAX_LINE_LEN);
    if (ret < 0) {
    printf(_("No powercapping driver loaded\n"));
    return ret;
    }
    printf("Driver: %s\n", line);
    ret = powercap_get_enabled(&val);
    if (ret < 0)
    return ret;
    if (!val) {
    printf(_("Powercapping is disabled\n"));
    return -1;
    }
    printf(_("Powercap domain hierarchy:\n\n"));
    root_zone = powercap_init_zones();
    if (root_zone == core::ptr::null_mut()) {
    printf(_("No powercap info found\n"));
    return 1;
    }
    powercap_walk_zones(root_zone, powercap_print_one_zone);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_cap_set(argc: c_int, argv: *mut c_char) -> c_int {
    int cmd_cap_set(int argc, char **argv)
    {
    return 0;
    };
#[no_mangle]
pub unsafe extern "C" fn cmd_cap_info(argc: c_int, argv: *mut c_char) -> c_int {
    int cmd_cap_info(int argc, char **argv)
    {
    let mut ret: c_int = 0, cont = 1;
    do {
    ret = getopt_long(argc, argv, "a", info_opts, core::ptr::null_mut());
    switch (ret) {
    case '?':
    cont = 0;
    break;
    case -1:
    cont = 0;
    break;
    case 'a':
    powercap_show_all = 1;
    break;
    default:
    fprintf(stderr, _("invalid or unknown argument\n"));
    return EXIT_FAILURE;
    }
    } while (cont);
    powercap_show();
    return 0;
    }
