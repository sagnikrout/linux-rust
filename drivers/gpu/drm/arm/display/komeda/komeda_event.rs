//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/arm/display/komeda/komeda_event.c
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
// (C) COPYRIGHT 2019 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_str {
    pub str: *mut c_char,
    pub sz: u32,
    pub len: u32,
}

// return 0 on success,  < 0 on no space.
//
    __printf(2, 3)
#[no_mangle]
unsafe extern "C" fn komeda_sprintf(str: *mut komeda_str, fmt: *const c_char, ...) -> c_int {
    static int komeda_sprintf(struct komeda_str *str, const char *fmt, ...)
    {
    va_list args;
    int num, free_sz;
    int err;
    free_sz = str.sz - str.len - 1;
    if (free_sz <= 0)
    return -ENOSPC;
    va_start(args, fmt);
    num = vsnprintf(str.str + str.len, free_sz, fmt, args);
    va_end(args);
    if (num < free_sz) {
    str.len += num;
    err = 0;
    } else {
    str.len = str.sz - 1;
    err = -ENOSPC;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn evt_sprintf(str: *mut komeda_str, evt: u64, msg: *const c_char) {
    static void evt_sprintf(struct komeda_str *str, u64 evt, const char *msg)
    {
    if (evt)
    komeda_sprintf(str, msg);
    }
#[no_mangle]
unsafe extern "C" fn evt_str(str: *mut komeda_str, events: u64) {
    static void evt_str(struct komeda_str *str, u64 events)
    {
    if (events == 0ULL) {
    komeda_sprintf(str, "None");
    return;
    }
    evt_sprintf(str, events & KOMEDA_EVENT_VSYNC, "VSYNC|");
    evt_sprintf(str, events & KOMEDA_EVENT_FLIP, "FLIP|");
    evt_sprintf(str, events & KOMEDA_EVENT_EOW, "EOW|");
    evt_sprintf(str, events & KOMEDA_EVENT_MODE, "OP-MODE|");
    evt_sprintf(str, events & KOMEDA_EVENT_URUN, "UNDERRUN|");
    evt_sprintf(str, events & KOMEDA_EVENT_OVR, "OVERRUN|");
// GLB error
    evt_sprintf(str, events & KOMEDA_ERR_MERR, "MERR|");
    evt_sprintf(str, events & KOMEDA_ERR_FRAMETO, "FRAMETO|");
// DOU error
    evt_sprintf(str, events & KOMEDA_ERR_DRIFTTO, "DRIFTTO|");
    evt_sprintf(str, events & KOMEDA_ERR_FRAMETO, "FRAMETO|");
    evt_sprintf(str, events & KOMEDA_ERR_TETO, "TETO|");
    evt_sprintf(str, events & KOMEDA_ERR_CSCE, "CSCE|");
// LPU errors or events
    evt_sprintf(str, events & KOMEDA_EVENT_IBSY, "IBSY|");
    evt_sprintf(str, events & KOMEDA_EVENT_EMPTY, "EMPTY|");
    evt_sprintf(str, events & KOMEDA_EVENT_FULL, "FULL|");
    evt_sprintf(str, events & KOMEDA_ERR_AXIE, "AXIE|");
    evt_sprintf(str, events & KOMEDA_ERR_ACE0, "ACE0|");
    evt_sprintf(str, events & KOMEDA_ERR_ACE1, "ACE1|");
    evt_sprintf(str, events & KOMEDA_ERR_ACE2, "ACE2|");
    evt_sprintf(str, events & KOMEDA_ERR_ACE3, "ACE3|");
// LPU TBU errors
    evt_sprintf(str, events & KOMEDA_ERR_TCF, "TCF|");
    evt_sprintf(str, events & KOMEDA_ERR_TTNG, "TTNG|");
    evt_sprintf(str, events & KOMEDA_ERR_TITR, "TITR|");
    evt_sprintf(str, events & KOMEDA_ERR_TEMR, "TEMR|");
    evt_sprintf(str, events & KOMEDA_ERR_TTF, "TTF|");
// CU errors
    evt_sprintf(str, events & KOMEDA_ERR_CPE, "COPROC|");
    evt_sprintf(str, events & KOMEDA_ERR_ZME, "ZME|");
    evt_sprintf(str, events & KOMEDA_ERR_CFGE, "CFGE|");
    evt_sprintf(str, events & KOMEDA_ERR_TEMR, "TEMR|");
    if (str.len > 0 && (str.str[str.len - 1] == '|')) {
    str.str[str.len - 1] = 0;
    str.len--;
    }
    }
#[no_mangle]
unsafe extern "C" fn is_new_frame(a: *mut komeda_events) -> bool {
    static bool is_new_frame(struct komeda_events *a)
    {
    return (a.pipes[0] | a.pipes[1]) &
    (KOMEDA_EVENT_FLIP | KOMEDA_EVENT_EOW);
    }
#[no_mangle]
pub unsafe extern "C" fn komeda_print_events(evts: *mut komeda_events, dev: *mut drm_device) {
    void komeda_print_events(struct komeda_events *evts, struct drm_device *dev)
    {
    let mut print_evts: u64 = 0;
    let mut en_print: static bool = true;
    struct komeda_dev *mdev = dev.dev_private;
    let mut err_verbosity: u16 const = mdev.err_verbosity;
    let mut evts_mask: u64 = evts.global | evts.pipes[0] | evts.pipes[1];
// reduce the same msg print, only print the first evt for one frame
    if (evts.global || is_new_frame(evts))
    en_print = true;
    if (!(err_verbosity & KOMEDA_DEV_PRINT_DISABLE_RATELIMIT) && !en_print)
    return;
    if (err_verbosity & KOMEDA_DEV_PRINT_ERR_EVENTS)
    print_evts |= KOMEDA_ERR_EVENTS;
    if (err_verbosity & KOMEDA_DEV_PRINT_WARN_EVENTS)
    print_evts |= KOMEDA_WARN_EVENTS;
    if (err_verbosity & KOMEDA_DEV_PRINT_INFO_EVENTS)
    print_evts |= KOMEDA_INFO_EVENTS;
    if (evts_mask & print_evts) {
    char msg[256];
    struct komeda_str str;
    let mut p: drm_printer = drm_info_printer(dev.dev);
    str.str = msg;
    str.sz  = sizeof(msg);
    str.len = 0;
    komeda_sprintf(&str, "gcu: ");
    evt_str(&str, evts.global);
    komeda_sprintf(&str, ", pipes[0]: ");
    evt_str(&str, evts.pipes[0]);
    komeda_sprintf(&str, ", pipes[1]: ");
    evt_str(&str, evts.pipes[1]);
    DRM_ERROR("err detect: %s\n", msg);
    if ((err_verbosity & KOMEDA_DEV_PRINT_DUMP_STATE_ON_EVENT) &&
    (evts_mask & (KOMEDA_ERR_EVENTS | KOMEDA_WARN_EVENTS)))
    drm_state_dump(dev, &p);
    en_print = false;
    }
    }
