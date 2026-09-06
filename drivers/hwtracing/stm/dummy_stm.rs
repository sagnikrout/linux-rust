//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/stm/dummy_stm.c
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
// A dummy STM device for stm/stm_source class testing.
// Copyright (c) 2014, Intel Corporation.
//
// STM class implements generic infrastructure for  System Trace Module devices
// as defined in MIPI STPv2 specification.
//

    static ssize_t notrace
    dummy_stm_packet(struct stm_data *stm_data, unsigned int master,
    unsigned int channel, unsigned int packet, unsigned int flags,
    unsigned int size, const unsigned char *payload)
    {

    let mut pl: u64 = 0;
    if (payload)
    pl = *(u64 *)payload;
    if (size < 8)
    pl &= (1ull << (size * 8)) - 1;
    trace_printk("[%u:%u] [pkt: %x/%x] (%llx)\n", master, channel,
    packet, size, pl);

    return size;
    }
pub const DUMMY_STM_MAX: c_int = 32;
    static struct stm_data dummy_stm[DUMMY_STM_MAX];
    let mut nr_dummies: static int = 4;
    module_param(nr_dummies, int, 0400);
    static unsigned int fail_mode;
    module_param(fail_mode, int, 0600);
    static unsigned int master_min;
    module_param(master_min, int, 0400);
    let mut master_max: static unsigned int = STP_MASTER_MAX;
    module_param(master_max, int, 0400);
    let mut nr_channels: static unsigned int = STP_CHANNEL_MAX;
    module_param(nr_channels, int, 0400);
    static int dummy_stm_link(struct stm_data *data, unsigned int master,
    unsigned int channel)
    {
    if (fail_mode && (channel & fail_mode))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dummy_stm_init() -> c_int {
    static int dummy_stm_init(void)
    {
    int i, ret = -ENOMEM;
    if (nr_dummies < 0 || nr_dummies > DUMMY_STM_MAX)
    return -EINVAL;
    if (master_min > master_max ||
    master_max > STP_MASTER_MAX ||
    nr_channels > STP_CHANNEL_MAX)
    return -EINVAL;
    for (i = 0; i < nr_dummies; i++) {
    dummy_stm[i].name = kasprintf(GFP_KERNEL, "dummy_stm.%d", i);
    if (!dummy_stm[i].name)
    goto fail_unregister;
    dummy_stm[i].sw_start		= master_min;
    dummy_stm[i].sw_end		= master_max;
    dummy_stm[i].sw_nchannels	= nr_channels;
    dummy_stm[i].packet		= dummy_stm_packet;
    dummy_stm[i].link		= dummy_stm_link;
    ret = stm_register_device(core::ptr::null_mut(), &dummy_stm[i], THIS_MODULE);
    if (ret)
    goto fail_free;
    }
    return 0;
    fail_unregister:
    for (i--; i >= 0; i--) {
    stm_unregister_device(&dummy_stm[i]);
    fail_free:
    kfree(dummy_stm[i].name);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dummy_stm_exit() {
    static void dummy_stm_exit(void)
    {
    int i;
    for (i = 0; i < nr_dummies; i++) {
    stm_unregister_device(&dummy_stm[i]);
    kfree(dummy_stm[i].name);
    }
    }
    module_init(dummy_stm_init);
    module_exit(dummy_stm_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("dummy_stm device");
    MODULE_AUTHOR("Alexander Shishkin <alexander.shishkin@linux.intel.com>");
