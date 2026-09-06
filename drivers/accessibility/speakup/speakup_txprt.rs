//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/speakup_txprt.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// originally written by: Kirk Reiser <kirk@braille.uwo.ca>
// this version considerably modified by David Borowski, david575@rogers.com
//
// Copyright (C) 1998-99  Kirk Reiser.
// Copyright (C) 2003 David Borowski.
//
// specifically written as a driver for the speakup screenreview
// s not a general device driver.
//

pub const SYNTH_CLEAR: c_uint = 0x18;

    enum default_vars_id {
    CAPS_START_ID = 0, CAPS_STOP_ID,
    RATE_ID, PITCH_ID,
    VOL_ID, TONE_ID,
    DIRECT_ID, V_LAST_VAR_ID,
    NB_ID
    };
    static struct var_t vars[NB_ID] = {
    [CAPS_START_ID] = { CAPS_START, .u.s = {"\x05P8" } },
    [CAPS_STOP_ID] = { CAPS_STOP, .u.s = {"\x05P5" } },
    [RATE_ID] = { RATE, .u.n = {"\x05R%d", 5, 0, 9, 0, 0, core::ptr::null_mut() } },
    [PITCH_ID] = { PITCH, .u.n = {"\x05P%d", 5, 0, 9, 0, 0, core::ptr::null_mut() } },
    [VOL_ID] = { VOL, .u.n = {"\x05V%d", 5, 0, 9, 0, 0, core::ptr::null_mut() } },
    [TONE_ID] = { TONE, .u.n = {"\x05T%c", 12, 0, 25, 61, 0, core::ptr::null_mut() } },
    [DIRECT_ID] = { DIRECT, .u.n = {core::ptr::null_mut(), 0, 0, 1, 0, 0, core::ptr::null_mut() } },
    V_LAST_VAR
    };
// These attributes will appear in /sys/accessibility/speakup/txprt.
    static struct kobj_attribute caps_start_attribute =
    __ATTR(caps_start, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute caps_stop_attribute =
    __ATTR(caps_stop, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute pitch_attribute =
    __ATTR(pitch, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute rate_attribute =
    __ATTR(rate, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute tone_attribute =
    __ATTR(tone, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute vol_attribute =
    __ATTR(vol, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute delay_time_attribute =
    __ATTR(delay_time, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute direct_attribute =
    __ATTR(direct, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute full_time_attribute =
    __ATTR(full_time, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute jiffy_delta_attribute =
    __ATTR(jiffy_delta, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute trigger_time_attribute =
    __ATTR(trigger_time, 0644, spk_var_show, spk_var_store);
//
// Create a group of attributes so that we can create and destroy them all
// at once.
//
    static struct attribute *synth_attrs[] = {
    &caps_start_attribute.attr,
    &caps_stop_attribute.attr,
    &pitch_attribute.attr,
    &rate_attribute.attr,
    &tone_attribute.attr,
    &vol_attribute.attr,
    &delay_time_attribute.attr,
    &direct_attribute.attr,
    &full_time_attribute.attr,
    &jiffy_delta_attribute.attr,
    &trigger_time_attribute.attr,
    core::ptr::null_mut(),	/* need to core::ptr::null_mut() terminate the list of attributes */
    };
    static struct spk_synth synth_txprt = {
    .name = "txprt",
    .version = DRV_VERSION,
    .long_name = "Transport",
    .init = "\x05N1",
    .procspeech = PROCSPEECH,
    .clear = SYNTH_CLEAR,
    .delay = 500,
    .trigger = 50,
    .jiffies = 50,
    .full = 40000,
    .dev_name = SYNTH_DEFAULT_DEV,
    .startup = SYNTH_START,
    .checkval = SYNTH_CHECK,
    .vars = vars,
    .io_ops = &spk_ttyio_ops,
    .probe = spk_ttyio_synth_probe,
    .release = spk_ttyio_release,
    .synth_immediate = spk_ttyio_synth_immediate,
    .catch_up = spk_do_catch_up,
    .flush = spk_synth_flush,
    .is_alive = spk_synth_is_alive_restart,
    .synth_adjust = core::ptr::null_mut(),
    .read_buff_add = core::ptr::null_mut(),
    .get_index = core::ptr::null_mut(),
    .indexing = {
    .command = core::ptr::null_mut(),
    .lowindex = 0,
    .highindex = 0,
    .currindex = 0,
    },
    .attributes = {
    .attrs = synth_attrs,
    .name = "txprt",
    },
    };
    module_param_named(ser, synth_txprt.ser, int, 0444);
    module_param_named(dev, synth_txprt.dev_name, charp, 0444);
    module_param_named(start, synth_txprt.startup, short, 0444);
    module_param_named(rate, vars[RATE_ID].u.n.default_val, int, 0444);
    module_param_named(pitch, vars[PITCH_ID].u.n.default_val, int, 0444);
    module_param_named(vol, vars[VOL_ID].u.n.default_val, int, 0444);
    module_param_named(tone, vars[TONE_ID].u.n.default_val, int, 0444);
    module_param_named(direct, vars[DIRECT_ID].u.n.default_val, int, 0444);
    MODULE_PARM_DESC(ser, "Set the serial port for the synthesizer (0-based).");
    MODULE_PARM_DESC(dev, "Set the device e.g. ttyUSB0, for the synthesizer.");
    MODULE_PARM_DESC(start, "Start the synthesizer once it is loaded.");
    MODULE_PARM_DESC(rate, "Set the rate variable on load.");
    MODULE_PARM_DESC(pitch, "Set the pitch variable on load.");
    MODULE_PARM_DESC(vol, "Set the vol variable on load.");
    MODULE_PARM_DESC(tone, "Set the tone variable on load.");
    MODULE_PARM_DESC(direct, "Set the direct variable on load.");
    module_spk_synth(synth_txprt);
    MODULE_AUTHOR("Kirk Reiser <kirk@braille.uwo.ca>");
    MODULE_AUTHOR("David Borowski");
    MODULE_DESCRIPTION("Speakup support for Transport synthesizers");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(DRV_VERSION);
