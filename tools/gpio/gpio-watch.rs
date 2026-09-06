//! Automatically rewritten from C to Rust
//! Source: tools/gpio/gpio-watch.c
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
// gpio-watch - monitor unrequested lines for property changes using the
// character device
//
// Copyright (C) 2019 BayLibre SAS
// Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct gpio_v2_line_info_changed chg;
    struct gpio_v2_line_info req;
    struct pollfd pfd;
    int fd, i, j, ret;
    char *event, *end;
    ssize_t rd;
    if (argc < 3)
    goto err_usage;
    fd = open(argv[1], O_RDWR | O_CLOEXEC);
    if (fd < 0) {
    perror("unable to open gpiochip");
    return EXIT_FAILURE;
    }
    for (i = 0, j = 2; i < argc - 2; i++, j++) {
    memset(&req, 0, sizeof(req));
    req.offset = strtoul(argv[j], &end, 0);
    if (*end != '\0')
    goto err_usage;
    ret = ioctl(fd, GPIO_V2_GET_LINEINFO_WATCH_IOCTL, &req);
    if (ret) {
    perror("unable to set up line watch");
    return EXIT_FAILURE;
    }
    }
    pfd.fd = fd;
    pfd.events = POLLIN | POLLPRI;
    for (;;) {
    ret = poll(&pfd, 1, 5000);
    if (ret < 0) {
    perror("error polling the linechanged fd");
    return EXIT_FAILURE;
    } else if (ret > 0) {
    memset(&chg, 0, sizeof(chg));
    rd = read(pfd.fd, &chg, sizeof(chg));
    if (rd < 0 || rd != sizeof(chg)) {
    if (rd != sizeof(chg))
    errno = EIO;
    perror("error reading line change event");
    return EXIT_FAILURE;
    }
    switch (chg.event_type) {
    case GPIO_V2_LINE_CHANGED_REQUESTED:
    event = "requested";
    break;
    case GPIO_V2_LINE_CHANGED_RELEASED:
    event = "released";
    break;
    case GPIO_V2_LINE_CHANGED_CONFIG:
    event = "config changed";
    break;
    default:
    fprintf(stderr,
    "invalid event type received from the kernel\n");
    return EXIT_FAILURE;
    }
    printf("line %u: %s at %" PRIu64 "\n",
    chg.info.offset, event, (uint64_t)chg.timestamp_ns);
    }
    }
    return 0;
    err_usage:
    printf("%s: <gpiochip> <line0> <line1> ...\n", argv[0]);
    return EXIT_FAILURE;
    }
