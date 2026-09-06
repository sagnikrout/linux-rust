//! Automatically rewritten from C to Rust
//! Source: sound/soc/soc-link.c
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
// soc-link.c
//
// Copyright (C) 2019 Renesas Electronics Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

    static inline int _soc_link_ret(struct snd_soc_pcm_runtime *rtd,
    const char *func, int ret)
    {
    return snd_soc_ret(rtd.dev, ret,
    "at %s() on %s\n", func, rtd.dai_link.name);
    }
//
// We might want to check substream by using list.
// In such case, we can update these macros.
//

#[no_mangle]
pub unsafe extern "C" fn snd_soc_link_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    int snd_soc_link_init(struct snd_soc_pcm_runtime *rtd)
    {
    let mut ret: c_int = 0;
    if (rtd.dai_link.init)
    ret = rtd.dai_link.init(rtd);
    return soc_link_ret(rtd, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_soc_link_exit(rtd: *mut snd_soc_pcm_runtime) {
    void snd_soc_link_exit(struct snd_soc_pcm_runtime *rtd)
    {
    if (rtd.dai_link.exit)
    rtd.dai_link.exit(rtd);
    }
    int snd_soc_link_be_hw_params_fixup(struct snd_soc_pcm_runtime *rtd,
    struct snd_pcm_hw_params *params)
    {
    let mut ret: c_int = 0;
    if (rtd.dai_link.be_hw_params_fixup)
    ret = rtd.dai_link.be_hw_params_fixup(rtd, params);
    return soc_link_ret(rtd, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_soc_link_startup(substream: *mut snd_pcm_substream) -> c_int {
    int snd_soc_link_startup(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    let mut ret: c_int = 0;
    if (rtd.dai_link.ops &&
    rtd.dai_link.ops.startup)
    ret = rtd.dai_link.ops.startup(substream);
// mark substream if succeeded
    if (ret == 0)
    soc_link_mark_push(rtd, substream, startup);
    return soc_link_ret(rtd, ret);
    }
    void snd_soc_link_shutdown(struct snd_pcm_substream *substream,
    int rollback)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    if (rollback && !soc_link_mark_match(rtd, substream, startup))
    return;
    if (rtd.dai_link.ops &&
    rtd.dai_link.ops.shutdown)
    rtd.dai_link.ops.shutdown(substream);
// remove marked substream
    soc_link_mark_pop(rtd, startup);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_soc_link_prepare(substream: *mut snd_pcm_substream) -> c_int {
    int snd_soc_link_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    let mut ret: c_int = 0;
    if (rtd.dai_link.ops &&
    rtd.dai_link.ops.prepare)
    ret = rtd.dai_link.ops.prepare(substream);
    return soc_link_ret(rtd, ret);
    }
    int snd_soc_link_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    let mut ret: c_int = 0;
    if (rtd.dai_link.ops &&
    rtd.dai_link.ops.hw_params)
    ret = rtd.dai_link.ops.hw_params(substream, params);
// mark substream if succeeded
    if (ret == 0)
    soc_link_mark_push(rtd, substream, hw_params);
    return soc_link_ret(rtd, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_soc_link_hw_free(substream: *mut snd_pcm_substream, rollback: c_int) {
    void snd_soc_link_hw_free(struct snd_pcm_substream *substream, int rollback)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    if (rollback && !soc_link_mark_match(rtd, substream, hw_params))
    return;
    if (rtd.dai_link.ops &&
    rtd.dai_link.ops.hw_free)
    rtd.dai_link.ops.hw_free(substream);
// remove marked substream
    soc_link_mark_pop(rtd, hw_params);
    }
#[no_mangle]
unsafe extern "C" fn soc_link_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int soc_link_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    let mut ret: c_int = 0;
    if (rtd.dai_link.ops &&
    rtd.dai_link.ops.trigger)
    ret = rtd.dai_link.ops.trigger(substream, cmd);
    return soc_link_ret(rtd, ret);
    }
    int snd_soc_link_trigger(struct snd_pcm_substream *substream, int cmd,
    int rollback)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    ret = soc_link_trigger(substream, cmd);
    if (ret < 0)
    break;
    soc_link_mark_push(rtd, substream, trigger);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    if (rollback && !soc_link_mark_match(rtd, substream, trigger))
    break;
    ret = soc_link_trigger(substream, cmd);
    soc_link_mark_pop(rtd, startup);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_soc_link_compr_startup(cstream: *mut snd_compr_stream) -> c_int {
    int snd_soc_link_compr_startup(struct snd_compr_stream *cstream)
    {
    struct snd_soc_pcm_runtime *rtd = cstream.private_data;
    let mut ret: c_int = 0;
    if (rtd.dai_link.compr_ops &&
    rtd.dai_link.compr_ops.startup)
    ret = rtd.dai_link.compr_ops.startup(cstream);
    if (ret == 0)
    soc_link_mark_push(rtd, cstream, compr_startup);
    return soc_link_ret(rtd, ret);
    }
    EXPORT_SYMBOL_GPL(snd_soc_link_compr_startup);
    void snd_soc_link_compr_shutdown(struct snd_compr_stream *cstream,
    int rollback)
    {
    struct snd_soc_pcm_runtime *rtd = cstream.private_data;
    if (rollback && !soc_link_mark_match(rtd, cstream, compr_startup))
    return;
    if (rtd.dai_link.compr_ops &&
    rtd.dai_link.compr_ops.shutdown)
    rtd.dai_link.compr_ops.shutdown(cstream);
    soc_link_mark_pop(rtd, compr_startup);
    }
    EXPORT_SYMBOL_GPL(snd_soc_link_compr_shutdown);
#[no_mangle]
pub unsafe extern "C" fn snd_soc_link_compr_set_params(cstream: *mut snd_compr_stream) -> c_int {
    int snd_soc_link_compr_set_params(struct snd_compr_stream *cstream)
    {
    struct snd_soc_pcm_runtime *rtd = cstream.private_data;
    let mut ret: c_int = 0;
    if (rtd.dai_link.compr_ops &&
    rtd.dai_link.compr_ops.set_params)
    ret = rtd.dai_link.compr_ops.set_params(cstream);
    return soc_link_ret(rtd, ret);
    }
    EXPORT_SYMBOL_GPL(snd_soc_link_compr_set_params);
