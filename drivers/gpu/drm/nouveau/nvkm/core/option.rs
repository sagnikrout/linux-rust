//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/core/option.c
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


//
// Copyright 2012 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Ben Skeggs
//

    const char *
    nvkm_stropt(const char *optstr, const char *opt, int *arglen)
    {
    while (optstr && *optstr != '\0') {
    let mut len: c_int = strcspn(optstr, ",=");
    switch (optstr[len]) {
    case '=':
    if (!strncasecmpz(optstr, opt, len)) {
    optstr += len + 1;
// arglen = strcspn(optstr, ",=");
    return *arglen ? optstr : core::ptr::null_mut();
    }
    optstr++;
    break;
    case ',':
    optstr++;
    break;
    default:
    break;
    }
    optstr += len;
    }
    return core::ptr::null_mut();
    }
    bool
    nvkm_boolopt(const char *optstr, const char *opt, bool value)
    {
    int arglen;
    optstr = nvkm_stropt(optstr, opt, &arglen);
    if (optstr) {
    if (!strncasecmpz(optstr, "0", arglen) ||
    !strncasecmpz(optstr, "no", arglen) ||
    !strncasecmpz(optstr, "off", arglen) ||
    !strncasecmpz(optstr, "false", arglen))
    value = false;
    else
    if (!strncasecmpz(optstr, "1", arglen) ||
    !strncasecmpz(optstr, "yes", arglen) ||
    !strncasecmpz(optstr, "on", arglen) ||
    !strncasecmpz(optstr, "true", arglen))
    value = true;
    }
    return value;
    }
    long
    nvkm_longopt(const char *optstr, const char *opt, long value)
    {
    let mut result: c_long = value;
    int arglen;
    char *s;
    optstr = nvkm_stropt(optstr, opt, &arglen);
    if (optstr && (s = kstrndup(optstr, arglen, GFP_KERNEL))) {
    let mut ret: c_int = kstrtol(s, 0, &value);
    if (ret == 0)
    result = value;
    kfree(s);
    }
    return result;
    }
    int
    nvkm_dbgopt(const char *optstr, const char *sub)
    {
    let mut mode: c_int = 1, level = CONFIG_NOUVEAU_DEBUG_DEFAULT;
    while (optstr) {
    let mut len: c_int = strcspn(optstr, ",=");
    switch (optstr[len]) {
    case '=':
    if (strncasecmpz(optstr, sub, len))
    mode = 0;
    optstr++;
    break;
    default:
    if (mode) {
    if (!strncasecmpz(optstr, "fatal", len))
    level = NV_DBG_FATAL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncasecmpz(optstr, _arg: "error", _arg: len)) -> else {
    else if (!strncasecmpz(optstr, "error", len))
    level = NV_DBG_ERROR;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncasecmpz(optstr, _arg: "warn", _arg: len)) -> else {
    else if (!strncasecmpz(optstr, "warn", len))
    level = NV_DBG_WARN;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncasecmpz(optstr, _arg: "info", _arg: len)) -> else {
    else if (!strncasecmpz(optstr, "info", len))
    level = NV_DBG_INFO;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncasecmpz(optstr, _arg: "debug", _arg: len)) -> else {
    else if (!strncasecmpz(optstr, "debug", len))
    level = NV_DBG_DEBUG;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncasecmpz(optstr, _arg: "trace", _arg: len)) -> else {
    else if (!strncasecmpz(optstr, "trace", len))
    level = NV_DBG_TRACE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncasecmpz(optstr, _arg: "paranoia", _arg: len)) -> else {
    else if (!strncasecmpz(optstr, "paranoia", len))
    level = NV_DBG_PARANOIA;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncasecmpz(optstr, _arg: "spam", _arg: len)) -> else {
    else if (!strncasecmpz(optstr, "spam", len))
    level = NV_DBG_SPAM;
    }
    if (optstr[len] != '\0') {
    optstr++;
    mode = 1;
    break;
    }
    return level;
    }
    optstr += len;
    }
    return level;
    }
