//! Automatically rewritten from C to Rust
//! Source: tools/thermal/thermal-engine/thermal-engine.c
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
// Thermal monitoring tool based on the thermal netlink events.
//
// Copyright (C) 2022 Linaro Ltd.
//
// Author: Daniel Lezcano <daniel.lezcano@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct options {
    pub loglevel: c_int,
    pub logopt: c_int,
    pub interactive: c_int,
    pub daemonize: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_data {
    pub tz: *mut thermal_zone,
    pub th: *mut thermal_handler,
}

#[no_mangle]
unsafe extern "C" fn show_threshold(th: *mut thermal_threshold, arg: *mut __maybe_unused void) -> c_int {
    static int show_threshold(struct thermal_threshold *th, __maybe_unused void *arg)
    {
    INFO("threshold temp=%d, direction=%d\n",
    th.temperature, th.direction);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn show_trip(tt: *mut thermal_trip, arg: *mut __maybe_unused void) -> c_int {
    static int show_trip(struct thermal_trip *tt, __maybe_unused void *arg)
    {
    INFO("trip id=%d, type=%d, temp=%d, hyst=%d\n",
    tt.id, tt.type, tt.temp, tt.hyst);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn show_temp(tz: *mut thermal_zone, arg: *mut __maybe_unused void) -> c_int {
    static int show_temp(struct thermal_zone *tz, __maybe_unused void *arg)
    {
    thermal_cmd_get_temp(arg, tz);
    INFO("temperature: %d\n", tz.temp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn show_governor(tz: *mut thermal_zone, arg: *mut __maybe_unused void) -> c_int {
    static int show_governor(struct thermal_zone *tz, __maybe_unused void *arg)
    {
    thermal_cmd_get_governor(arg, tz);
    INFO("governor: '%s'\n", tz.governor);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn show_tz(tz: *mut thermal_zone, arg: *mut __maybe_unused void) -> c_int {
    static int show_tz(struct thermal_zone *tz, __maybe_unused void *arg)
    {
    INFO("thermal zone '%s', id=%d\n", tz.name, tz.id);
    for_each_thermal_trip(tz.trip, show_trip, core::ptr::null_mut());
    for_each_thermal_threshold(tz.thresholds, show_threshold, core::ptr::null_mut());
    show_temp(tz, arg);
    show_governor(tz, arg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_threshold(tz: *mut thermal_zone, arg: *mut __maybe_unused void) -> c_int {
    static int set_threshold(struct thermal_zone *tz, __maybe_unused void *arg)
    {
    struct thermal_handler *th = arg;
    int thresholds[] = { 43000, 65000, 49000, 55000, 57000 };
    size_t i;
    INFO("Setting threshold for thermal zone '%s', id=%d\n", tz.name, tz.id);
    if (thermal_cmd_threshold_flush(th, tz)) {
    ERROR("Failed to flush all previous thresholds\n");
    return -1;
    }
    for (i = 0; i < sizeof(thresholds) / sizeof(thresholds[0]); i++)
    if (thermal_cmd_threshold_add(th, tz, thresholds[i],
    THERMAL_THRESHOLD_WAY_UP |
    THERMAL_THRESHOLD_WAY_DOWN)) {
    ERROR("Failed to set threshold\n");
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tz_create(name: *const c_char, tz_id: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int tz_create(const char *name, int tz_id, __maybe_unused void *arg)
    {
    INFO("Thermal zone '%s'/%d created\n", name, tz_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tz_delete(tz_id: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int tz_delete(int tz_id, __maybe_unused void *arg)
    {
    INFO("Thermal zone %d deleted\n", tz_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tz_disable(tz_id: c_int, arg: *mut c_void) -> c_int {
    static int tz_disable(int tz_id, void *arg)
    {
    struct thermal_data *td = arg;
    struct thermal_zone *tz = thermal_zone_find_by_id(td.tz, tz_id);
    INFO("Thermal zone %d ('%s') disabled\n", tz_id, tz.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tz_enable(tz_id: c_int, arg: *mut c_void) -> c_int {
    static int tz_enable(int tz_id, void *arg)
    {
    struct thermal_data *td = arg;
    struct thermal_zone *tz = thermal_zone_find_by_id(td.tz, tz_id);
    INFO("Thermal zone %d ('%s') enabled\n", tz_id, tz.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trip_high(tz_id: c_int, trip_id: c_int, temp: c_int, arg: *mut c_void) -> c_int {
    static int trip_high(int tz_id, int trip_id, int temp, void *arg)
    {
    struct thermal_data *td = arg;
    struct thermal_zone *tz = thermal_zone_find_by_id(td.tz, tz_id);
    INFO("Thermal zone %d ('%s'): trip point %d crossed way up with %d °C\n",
    tz_id, tz.name, trip_id, temp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trip_low(tz_id: c_int, trip_id: c_int, temp: c_int, arg: *mut c_void) -> c_int {
    static int trip_low(int tz_id, int trip_id, int temp, void *arg)
    {
    struct thermal_data *td = arg;
    struct thermal_zone *tz = thermal_zone_find_by_id(td.tz, tz_id);
    INFO("Thermal zone %d ('%s'): trip point %d crossed way down with %d °C\n",
    tz_id, tz.name, trip_id, temp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trip_add(tz_id: c_int, trip_id: c_int, type: c_int, temp: c_int, hyst: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int trip_add(int tz_id, int trip_id, int type, int temp, int hyst, __maybe_unused void *arg)
    {
    INFO("Trip point added %d: id=%d, type=%d, temp=%d, hyst=%d\n",
    tz_id, trip_id, type, temp, hyst);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trip_delete(tz_id: c_int, trip_id: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int trip_delete(int tz_id, int trip_id, __maybe_unused void *arg)
    {
    INFO("Trip point deleted %d: id=%d\n", tz_id, trip_id);
    return 0;
    }
    static int trip_change(int tz_id, int trip_id, int type, int temp,
    int hyst, __maybe_unused void *arg)
    {
    struct thermal_data *td = arg;
    struct thermal_zone *tz = thermal_zone_find_by_id(td.tz, tz_id);
    INFO("Trip point changed %d: id=%d, type=%d, temp=%d, hyst=%d\n",
    tz_id, trip_id, type, temp, hyst);
    tz.trip[trip_id].type = type;
    tz.trip[trip_id].temp = temp;
    tz.trip[trip_id].hyst = hyst;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdev_add(name: *const c_char, cdev_id: c_int, max_state: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int cdev_add(const char *name, int cdev_id, int max_state, __maybe_unused void *arg)
    {
    INFO("Cooling device '%s'/%d (max state=%d) added\n", name, cdev_id, max_state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdev_delete(cdev_id: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int cdev_delete(int cdev_id, __maybe_unused void *arg)
    {
    INFO("Cooling device %d deleted", cdev_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdev_update(cdev_id: c_int, cur_state: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int cdev_update(int cdev_id, int cur_state, __maybe_unused void *arg)
    {
    INFO("cdev:%d state:%d\n", cdev_id, cur_state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gov_change(tz_id: c_int, name: *const c_char, arg: *mut __maybe_unused void) -> c_int {
    static int gov_change(int tz_id, const char *name, __maybe_unused void *arg)
    {
    struct thermal_data *td = arg;
    struct thermal_zone *tz = thermal_zone_find_by_id(td.tz, tz_id);
    INFO("%s: governor changed %s . %s\n", tz.name, tz.governor, name);
    strcpy(tz.governor, name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn threshold_add(tz_id: c_int, temp: c_int, direction: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int threshold_add(int tz_id, int temp, int direction, __maybe_unused void *arg)
    {
    INFO("Threshold added tz_id=%d: temp=%d, direction=%d\n", tz_id, temp, direction);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn threshold_delete(tz_id: c_int, temp: c_int, direction: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int threshold_delete(int tz_id, int temp, int direction, __maybe_unused void *arg)
    {
    INFO("Threshold deleted tz_id=%d: temp=%d, direction=%d\n", tz_id, temp, direction);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn threshold_flush(tz_id: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int threshold_flush(int tz_id, __maybe_unused void *arg)
    {
    INFO("Thresholds flushed tz_id=%d\n", tz_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn threshold_up(tz_id: c_int, temp: c_int, prev_temp: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int threshold_up(int tz_id, int temp, int prev_temp, __maybe_unused void *arg)
    {
    INFO("Threshold crossed way up tz_id=%d: temp=%d, prev_temp=%d\n",
    tz_id, temp, prev_temp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn threshold_down(tz_id: c_int, temp: c_int, prev_temp: c_int, arg: *mut __maybe_unused void) -> c_int {
    static int threshold_down(int tz_id, int temp, int prev_temp, __maybe_unused void *arg)
    {
    INFO("Threshold crossed way down tz_id=%d: temp=%d, prev_temp=%d\n",
    tz_id, temp, prev_temp);
    return 0;
    }
    static struct thermal_ops ops = {
    .events.tz_create		= tz_create,
    .events.tz_delete		= tz_delete,
    .events.tz_disable		= tz_disable,
    .events.tz_enable		= tz_enable,
    .events.trip_high		= trip_high,
    .events.trip_low		= trip_low,
    .events.trip_add		= trip_add,
    .events.trip_delete		= trip_delete,
    .events.trip_change		= trip_change,
    .events.cdev_add		= cdev_add,
    .events.cdev_delete		= cdev_delete,
    .events.cdev_update		= cdev_update,
    .events.gov_change		= gov_change,
    .events.threshold_add		= threshold_add,
    .events.threshold_delete	= threshold_delete,
    .events.threshold_flush		= threshold_flush,
    .events.threshold_up		= threshold_up,
    .events.threshold_down		= threshold_down,
    };
#[no_mangle]
unsafe extern "C" fn thermal_event(fd: __maybe_unused int, arg: *mut __maybe_unused void) -> c_int {
    static int thermal_event(__maybe_unused int fd, __maybe_unused void *arg)
    {
    struct thermal_data *td = arg;
    return thermal_events_handle(td.th, td);
    }
#[no_mangle]
unsafe extern "C" fn usage(cmd: *const c_char) {
    static void usage(const char *cmd)
    {
    printf("%s : A thermal monitoring engine based on notifications\n", cmd);
    printf("Usage: %s [options]\n", cmd);
    printf("\t-h, --help\t\tthis help\n");
    printf("\t-d, --daemonize\n");
    printf("\t-l <level>, --loglevel <level>\tlog level: ");
    printf("DEBUG, INFO, NOTICE, WARN, ERROR\n");
    printf("\t-s, --syslog\t\toutput to syslog\n");
    printf("\n");
    exit(0);
    }
#[no_mangle]
unsafe extern "C" fn options_init(argc: c_int, argv[]: *mut c_char, options: *mut options) -> c_int {
    static int options_init(int argc, char *argv[], struct options *options)
    {
    int opt;
    struct option long_options[] = {
    { "help",	no_argument, core::ptr::null_mut(), 'h' },
    { "daemonize",	no_argument, core::ptr::null_mut(), 'd' },
    { "syslog",	no_argument, core::ptr::null_mut(), 's' },
    { "loglevel",	required_argument, core::ptr::null_mut(), 'l' },
    { 0, 0, 0, 0 }
    };
    while (1) {
    let mut optindex: c_int = 0;
    opt = getopt_long(argc, argv, "l:dhs", long_options, &optindex);
    if (opt == -1)
    break;
    switch (opt) {
    case 'l':
    options.loglevel = log_str2level(optarg);
    break;
    case 'd':
    options.daemonize = 1;
    break;
    case 's':
    options.logopt = TO_SYSLOG;
    break;
    case 'h':
    usage(basename(argv[0]));
    break;
    default: /* '?' */
    return -1;
    }
    }
    return 0;
    }
    enum {
    THERMAL_ENGINE_SUCCESS = 0,
    THERMAL_ENGINE_OPTION_ERROR,
    THERMAL_ENGINE_DAEMON_ERROR,
    THERMAL_ENGINE_LOG_ERROR,
    THERMAL_ENGINE_THERMAL_ERROR,
    THERMAL_ENGINE_THRESHOLD_ERROR,
    THERMAL_ENGINE_MAINLOOP_ERROR,
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct thermal_data td;
    struct options options = {
    .loglevel = LOG_INFO,
    .logopt = TO_STDOUT,
    };
    if (options_init(argc, argv, &options)) {
    ERROR("Usage: %s --help\n", argv[0]);
    return THERMAL_ENGINE_OPTION_ERROR;
    }
    if (options.daemonize && daemon(0, 0)) {
    ERROR("Failed to daemonize: %m\n");
    return THERMAL_ENGINE_DAEMON_ERROR;
    }
    if (log_init(options.loglevel, basename(argv[0]), options.logopt)) {
    ERROR("Failed to initialize logging facility\n");
    return THERMAL_ENGINE_LOG_ERROR;
    }
    td.th = thermal_init(&ops);
    if (!td.th) {
    ERROR("Failed to initialize the thermal library\n");
    return THERMAL_ENGINE_THERMAL_ERROR;
    }
    td.tz = thermal_zone_discover(td.th);
    if (!td.tz) {
    ERROR("No thermal zone available\n");
    return THERMAL_ENGINE_THERMAL_ERROR;
    }
    for_each_thermal_zone(td.tz, set_threshold, td.th);
    for_each_thermal_zone(td.tz, show_tz, td.th);
    if (mainloop_init()) {
    ERROR("Failed to initialize the mainloop\n");
    return THERMAL_ENGINE_MAINLOOP_ERROR;
    }
    if (mainloop_add(thermal_events_fd(td.th), thermal_event, &td)) {
    ERROR("Failed to setup the mainloop\n");
    return THERMAL_ENGINE_MAINLOOP_ERROR;
    }
    INFO("Waiting for thermal events ...\n");
    if (mainloop(-1)) {
    ERROR("Mainloop failed\n");
    return THERMAL_ENGINE_MAINLOOP_ERROR;
    }
    return THERMAL_ENGINE_SUCCESS;
    }
