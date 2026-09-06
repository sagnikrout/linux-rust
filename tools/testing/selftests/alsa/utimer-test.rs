//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/alsa/utimer-test.c
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
// This test covers the functionality of userspace-driven ALSA timers. Such timers
// are purely virtual (so they don't directly depend on the hardware), and they could be
// created and triggered by userspace applications.
//
// Author: Ivan Orlov <ivan.orlov0322@gmail.com>
//

pub const FRAME_RATE: c_int = 8000;
pub const PERIOD_SIZE: c_int = 4410;

pub const TICKS_COUNT: c_int = 10;
pub const TICKS_RECORDING_DELTA: c_int = 5;
pub const TIMER_OUTPUT_BUF_LEN: c_int = 1024;
pub const TIMER_FREQ_SEC: c_int = 1;

    enum timer_app_event {
    TIMER_APP_STARTED,
    TIMER_APP_RESULT,
    TIMER_NO_EVENT,
    };
    FIXTURE(timer_f) {
    struct snd_timer_uinfo *utimer_info;
    };
    FIXTURE_SETUP(timer_f) {
    int timer_dev_fd;
    if (geteuid())
    SKIP(return, "This test needs root to run!");
    self.utimer_info = calloc(1, sizeof(*self.utimer_info));
    ASSERT_NE(core::ptr::null_mut(), self.utimer_info);
// Resolution is the time the period of frames takes in nanoseconds
    self.utimer_info.resolution = (NANO / FRAME_RATE * PERIOD_SIZE);
    timer_dev_fd = open("/dev/snd/timer", O_RDONLY);
    ASSERT_GE(timer_dev_fd, 0);
    if (ioctl(timer_dev_fd, SNDRV_TIMER_IOCTL_CREATE, self.utimer_info) < 0) {
    let mut err: c_int = errno;
    close(timer_dev_fd);
    if (err == ENOTTY || err == ENXIO)
    SKIP(return, "CONFIG_SND_UTIMER not enabled");
    ASSERT_EQ(err, 0);
    }
    ASSERT_GE(self.utimer_info.fd, 0);
    close(timer_dev_fd);
    }
    FIXTURE_TEARDOWN(timer_f) {
    close(self.utimer_info.fd);
    free(self.utimer_info);
    }
    static void *ticking_func(void *data)
    {
    int i;
    int *fd = (int *)data;
    for (i = 0; i < TICKS_COUNT; i++) {
// Well, trigger the timer!
    ioctl(*fd, SNDRV_TIMER_IOCTL_TRIGGER, core::ptr::null_mut());
    sleep(TIMER_FREQ_SEC);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn parse_timer_output(s: *const c_char) -> enum timer_app_event {
    static enum timer_app_event parse_timer_output(const char *s)
    {
    if (strstr(s, "Timer has started"))
    return TIMER_APP_STARTED;
    if (strstr(s, "Total ticks count"))
    return TIMER_APP_RESULT;
    return TIMER_NO_EVENT;
    }
#[no_mangle]
unsafe extern "C" fn parse_timer_result(s: *const c_char) -> c_int {
    static int parse_timer_result(const char *s)
    {
    char *end;
    long d;
    d = strtol(s + RESULT_PREFIX_LEN, &end, 10);
    if (end == s + RESULT_PREFIX_LEN)
    return -1;
    return d;
    }
//
// This test triggers the timer and counts ticks at the same time. The amount
// of the timer trigger calls should be equal to the amount of ticks received.
//
    TEST_F(timer_f, utimer) {
    char command[64];
    pthread_t ticking_thread;
    let mut total_ticks: c_int = 0;
    FILE *rfp;
    char *buf = malloc(TIMER_OUTPUT_BUF_LEN);
    ASSERT_NE(buf, core::ptr::null_mut());
// The timeout should be the ticks interval * count of ticks + some delta
    sprintf(command, "./global-timer %d %d %d", SNDRV_TIMER_GLOBAL_UDRIVEN,
    self.utimer_info.id, TICKS_COUNT * TIMER_FREQ_SEC + TICKS_RECORDING_DELTA);
    rfp = popen(command, "r");
    while (fgets(buf, TIMER_OUTPUT_BUF_LEN, rfp)) {
    buf[TIMER_OUTPUT_BUF_LEN - 1] = 0;
    switch (parse_timer_output(buf)) {
    case TIMER_APP_STARTED:
// global-timer waits for timer to trigger, so start the ticking thread
    pthread_create(&ticking_thread, core::ptr::null_mut(), ticking_func,
    &self.utimer_info.fd);
    break;
    case TIMER_APP_RESULT:
    total_ticks = parse_timer_result(buf);
    break;
    case TIMER_NO_EVENT:
    break;
    }
    }
    pthread_join(ticking_thread, core::ptr::null_mut());
    ASSERT_EQ(total_ticks, TICKS_COUNT);
    pclose(rfp);
    free(buf);
    }
    TEST(wrong_timers_test) {
    int timer_dev_fd;
    int utimer_fd;
    struct snd_timer_uinfo wrong_timer = {
    .resolution = 0,
    .id = UTIMER_DEFAULT_ID,
    .fd = UTIMER_DEFAULT_FD,
    };
    timer_dev_fd = open("/dev/snd/timer", O_RDONLY);
    ASSERT_GE(timer_dev_fd, 0);
    utimer_fd = ioctl(timer_dev_fd, SNDRV_TIMER_IOCTL_CREATE, &wrong_timer);
    ASSERT_LT(utimer_fd, 0);
// Check that id was not updated
    ASSERT_EQ(wrong_timer.id, UTIMER_DEFAULT_ID);
// Test the NULL as an argument is processed correctly
    ASSERT_LT(ioctl(timer_dev_fd, SNDRV_TIMER_IOCTL_CREATE, core::ptr::null_mut()), 0);
    close(timer_dev_fd);
    }
    TEST_HARNESS_MAIN
