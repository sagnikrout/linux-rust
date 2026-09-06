//! Automatically rewritten from C to Rust
//! Source: tools/gpio/lsgpio.c
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
// lsgpio - example on how to list the GPIO lines on a system
//
// Copyright (C) 2015 Linus Walleij
//
// Usage:
// lsgpio <-n device-name>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_flag {
    pub name: *mut c_char,
    pub mask: c_ulonglong,
}

    struct gpio_flag flagnames[] = {
    {
    .name = "used",
    .mask = GPIO_V2_LINE_FLAG_USED,
    },
    {
    .name = "input",
    .mask = GPIO_V2_LINE_FLAG_INPUT,
    },
    {
    .name = "output",
    .mask = GPIO_V2_LINE_FLAG_OUTPUT,
    },
    {
    .name = "active-low",
    .mask = GPIO_V2_LINE_FLAG_ACTIVE_LOW,
    },
    {
    .name = "open-drain",
    .mask = GPIO_V2_LINE_FLAG_OPEN_DRAIN,
    },
    {
    .name = "open-source",
    .mask = GPIO_V2_LINE_FLAG_OPEN_SOURCE,
    },
    {
    .name = "pull-up",
    .mask = GPIO_V2_LINE_FLAG_BIAS_PULL_UP,
    },
    {
    .name = "pull-down",
    .mask = GPIO_V2_LINE_FLAG_BIAS_PULL_DOWN,
    },
    {
    .name = "bias-disabled",
    .mask = GPIO_V2_LINE_FLAG_BIAS_DISABLED,
    },
    {
    .name = "clock-realtime",
    .mask = GPIO_V2_LINE_FLAG_EVENT_CLOCK_REALTIME,
    },
    };
#[no_mangle]
unsafe extern "C" fn print_attributes(info: *mut gpio_v2_line_info) {
    static void print_attributes(struct gpio_v2_line_info *info)
    {
    int i;
    const char *field_format = "%s";
    for (i = 0; i < ARRAY_SIZE(flagnames); i++) {
    if (info.flags & flagnames[i].mask) {
    fprintf(stdout, field_format, flagnames[i].name);
    field_format = ", %s";
    }
    }
    if ((info.flags & GPIO_V2_LINE_FLAG_EDGE_RISING) &&
    (info.flags & GPIO_V2_LINE_FLAG_EDGE_FALLING))
    fprintf(stdout, field_format, "both-edges");
#[no_mangle]
pub unsafe extern "C" fn if(GPIO_V2_LINE_FLAG_EDGE_RISING: info->flags &) -> else {
    else if (info.flags & GPIO_V2_LINE_FLAG_EDGE_RISING)
    fprintf(stdout, field_format, "rising-edge");
#[no_mangle]
pub unsafe extern "C" fn if(GPIO_V2_LINE_FLAG_EDGE_FALLING: info->flags &) -> else {
    else if (info.flags & GPIO_V2_LINE_FLAG_EDGE_FALLING)
    fprintf(stdout, field_format, "falling-edge");
    for (i = 0; i < info.num_attrs; i++) {
    if (info.attrs[i].id == GPIO_V2_LINE_ATTR_ID_DEBOUNCE)
    fprintf(stdout, ", debounce_period=%dusec",
    info.attrs[i].debounce_period_us);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn list_device(device_name: *const c_char) -> c_int {
    int list_device(const char *device_name)
    {
    struct gpiochip_info cinfo;
    char *chrdev_name;
    int fd;
    int ret;
    int i;
    ret = asprintf(&chrdev_name, "/dev/%s", device_name);
    if (ret < 0)
    return -ENOMEM;
    fd = open(chrdev_name, 0);
    if (fd == -1) {
    ret = -errno;
    fprintf(stderr, "Failed to open %s\n", chrdev_name);
    goto exit_free_name;
    }
// Inspect this GPIO chip
    ret = ioctl(fd, GPIO_GET_CHIPINFO_IOCTL, &cinfo);
    if (ret == -1) {
    ret = -errno;
    perror("Failed to issue CHIPINFO IOCTL\n");
    goto exit_close_error;
    }
    fprintf(stdout, "GPIO chip: %s, \"%s\", %u GPIO lines\n",
    cinfo.name, cinfo.label, cinfo.lines);
// Loop over the lines and print info
    for (i = 0; i < cinfo.lines; i++) {
    struct gpio_v2_line_info linfo;
    memset(&linfo, 0, sizeof(linfo));
    linfo.offset = i;
    ret = ioctl(fd, GPIO_V2_GET_LINEINFO_IOCTL, &linfo);
    if (ret == -1) {
    ret = -errno;
    perror("Failed to issue LINEINFO IOCTL\n");
    goto exit_close_error;
    }
    fprintf(stdout, "\tline %2d:", linfo.offset);
    if (linfo.name[0])
    fprintf(stdout, " \"%s\"", linfo.name);
    else
    fprintf(stdout, " unnamed");
    if (linfo.consumer[0])
    fprintf(stdout, " \"%s\"", linfo.consumer);
    else
    fprintf(stdout, " unused");
    if (linfo.flags) {
    fprintf(stdout, " [");
    print_attributes(&linfo);
    fprintf(stdout, "]");
    }
    fprintf(stdout, "\n");
    }
    exit_close_error:
    if (close(fd) == -1)
    perror("Failed to close GPIO character device file");
    exit_free_name:
    free(chrdev_name);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn print_usage() {
    void print_usage(void)
    {
    fprintf(stderr, "Usage: lsgpio [options]...\n"
    "List GPIO chips, lines and states\n"
    "  -n <name>  List GPIOs on a named device\n"
    "  -?         This helptext\n"
    );
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *device_name = core::ptr::null_mut();
    int ret;
    int c;
    while ((c = getopt(argc, argv, "n:")) != -1) {
    switch (c) {
    case 'n':
    device_name = optarg;
    break;
    case '?':
    print_usage();
    return -1;
    }
    }
    if (device_name)
    ret = list_device(device_name);
    else {
    const struct dirent *ent;
    DIR *dp;
// List all GPIO devices one at a time
    dp = opendir("/dev");
    if (!dp) {
    ret = -errno;
    goto error_out;
    }
    ret = -ENOENT;
    while (ent = readdir(dp), ent) {
    if (check_prefix(ent.d_name, "gpiochip")) {
    ret = list_device(ent.d_name);
    if (ret)
    break;
    }
    }
    ret = 0;
    if (closedir(dp) == -1) {
    perror("scanning devices: Failed to close directory");
    ret = -errno;
    }
    }
    error_out:
    return ret;
    }
