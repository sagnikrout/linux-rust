//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tas2781-comlib.c
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
// TAS2563/TAS2781 Common functions for HDA and ASoC Audio drivers
//
// Copyright 2023 - 2025 Texas Instruments, Inc.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>

    int tasdevice_dev_read(struct tasdevice_priv *tas_priv,
    unsigned short chn, unsigned int reg, unsigned int *val)
    {
    let mut ret: c_int = 0;
    if (chn < tas_priv.ndev) {
    struct regmap *map = tas_priv.regmap;
    ret = tas_priv.change_chn_book(tas_priv, chn,
    TASDEVICE_BOOK_ID(reg));
    if (ret < 0)
    goto out;
    ret = regmap_read(map, TASDEVICE_PGRG(reg), val);
    if (ret < 0)
    dev_err(tas_priv.dev, "%s, E=%d\n", __func__, ret);
    } else {
    ret = -EINVAL;
    dev_err(tas_priv.dev, "%s, no such channel(%d)\n", __func__,
    chn);
    }
    out:
    return ret;
    }
    EXPORT_SYMBOL_GPL(tasdevice_dev_read);
    int tasdevice_dev_bulk_read(struct tasdevice_priv *tas_priv,
    unsigned short chn, unsigned int reg, unsigned char *data,
    unsigned int len)
    {
    let mut ret: c_int = 0;
    if (chn < tas_priv.ndev) {
    struct regmap *map = tas_priv.regmap;
    ret = tas_priv.change_chn_book(tas_priv, chn,
    TASDEVICE_BOOK_ID(reg));
    if (ret < 0)
    goto out;
    ret = regmap_bulk_read(map, TASDEVICE_PGRG(reg), data, len);
    if (ret < 0)
    dev_err(tas_priv.dev, "%s, E=%d\n", __func__, ret);
    } else
    dev_err(tas_priv.dev, "%s, no such channel(%d)\n", __func__,
    chn);
    out:
    return ret;
    }
    EXPORT_SYMBOL_GPL(tasdevice_dev_bulk_read);
    int tasdevice_dev_write(struct tasdevice_priv *tas_priv,
    unsigned short chn, unsigned int reg, unsigned int value)
    {
    let mut ret: c_int = 0;
    if (chn < tas_priv.ndev) {
    struct regmap *map = tas_priv.regmap;
    ret = tas_priv.change_chn_book(tas_priv, chn,
    TASDEVICE_BOOK_ID(reg));
    if (ret < 0)
    goto out;
    ret = regmap_write(map, TASDEVICE_PGRG(reg),
    value);
    if (ret < 0)
    dev_err(tas_priv.dev, "%s, E=%d\n", __func__, ret);
    } else {
    ret = -EINVAL;
    dev_err(tas_priv.dev, "%s, no such channel(%d)\n", __func__,
    chn);
    }
    out:
    return ret;
    }
    EXPORT_SYMBOL_GPL(tasdevice_dev_write);
    int tasdevice_dev_bulk_write(
    struct tasdevice_priv *tas_priv, unsigned short chn,
    unsigned int reg, unsigned char *data,
    unsigned int len)
    {
    let mut ret: c_int = 0;
    if (chn < tas_priv.ndev) {
    struct regmap *map = tas_priv.regmap;
    ret = tas_priv.change_chn_book(tas_priv, chn,
    TASDEVICE_BOOK_ID(reg));
    if (ret < 0)
    goto out;
    ret = regmap_bulk_write(map, TASDEVICE_PGRG(reg),
    data, len);
    if (ret < 0)
    dev_err(tas_priv.dev, "%s, E=%d\n", __func__, ret);
    } else {
    ret = -EINVAL;
    dev_err(tas_priv.dev, "%s, no such channel(%d)\n", __func__,
    chn);
    }
    out:
    return ret;
    }
    EXPORT_SYMBOL_GPL(tasdevice_dev_bulk_write);
#[no_mangle]
unsafe extern "C" fn tasdev_dsp_prog_blk_remove(prog: *mut tasdevice_prog) {
    static void tasdev_dsp_prog_blk_remove(struct tasdevice_prog *prog)
    {
    struct tasdevice_data *tas_dt;
    struct tasdev_blk *blk;
    unsigned int i;
    if (!prog)
    return;
    tas_dt = &(prog.dev_data);
    if (!tas_dt.dev_blks)
    return;
    for (i = 0; i < tas_dt.nr_blk; i++) {
    blk = &(tas_dt.dev_blks[i]);
    kfree(blk.data);
    }
    kfree(tas_dt.dev_blks);
    }
    static void tasdev_dsp_prog_remove(struct tasdevice_prog *prog,
    unsigned short nr)
    {
    int i;
    for (i = 0; i < nr; i++)
    tasdev_dsp_prog_blk_remove(&prog[i]);
    kfree(prog);
    }
#[no_mangle]
unsafe extern "C" fn tasdev_dsp_cfg_blk_remove(cfg: *mut tasdevice_config) {
    static void tasdev_dsp_cfg_blk_remove(struct tasdevice_config *cfg)
    {
    struct tasdevice_data *tas_dt;
    struct tasdev_blk *blk;
    unsigned int i;
    if (cfg) {
    tas_dt = &(cfg.dev_data);
    if (!tas_dt.dev_blks)
    return;
    for (i = 0; i < tas_dt.nr_blk; i++) {
    blk = &(tas_dt.dev_blks[i]);
    kfree(blk.data);
    }
    kfree(tas_dt.dev_blks);
    }
    }
    static void tasdev_dsp_cfg_remove(struct tasdevice_config *config,
    unsigned short nr)
    {
    int i;
    for (i = 0; i < nr; i++)
    tasdev_dsp_cfg_blk_remove(&config[i]);
    kfree(config);
    }
#[no_mangle]
pub unsafe extern "C" fn tasdevice_dsp_remove(context: *mut c_void) {
    void tasdevice_dsp_remove(void *context)
    {
    struct tasdevice_priv *tas_dev = (struct tasdevice_priv *) context;
    struct tasdevice_fw *tas_fmw = tas_dev.fmw;
    if (!tas_dev.fmw)
    return;
    if (tas_fmw.programs)
    tasdev_dsp_prog_remove(tas_fmw.programs,
    tas_fmw.nr_programs);
    if (tas_fmw.configs)
    tasdev_dsp_cfg_remove(tas_fmw.configs,
    tas_fmw.nr_configurations);
    kfree(tas_fmw);
    tas_dev.fmw = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(tasdevice_dsp_remove);
#[no_mangle]
pub unsafe extern "C" fn tasdevice_remove(tas_priv: *mut tasdevice_priv) {
    void tasdevice_remove(struct tasdevice_priv *tas_priv)
    {
    mutex_destroy(&tas_priv.codec_lock);
    }
    EXPORT_SYMBOL_GPL(tasdevice_remove);
    MODULE_DESCRIPTION("TAS2781 common library");
    MODULE_AUTHOR("Shenghao Ding, TI, <shenghao-ding@ti.com>");
    MODULE_LICENSE("GPL");
