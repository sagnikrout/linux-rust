//! Automatically rewritten from C to Rust
//! Source: sound/drivers/vx/vx_hwdep.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Digigram VX soundcards
//
// DSP firmware management
//
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
//

    MODULE_FIRMWARE("vx/bx_1_vxp.b56");
    MODULE_FIRMWARE("vx/bx_1_vp4.b56");
    MODULE_FIRMWARE("vx/x1_1_vx2.xlx");
    MODULE_FIRMWARE("vx/x1_2_v22.xlx");
    MODULE_FIRMWARE("vx/x1_1_vxp.xlx");
    MODULE_FIRMWARE("vx/x1_1_vp4.xlx");
    MODULE_FIRMWARE("vx/bd56002.boot");
    MODULE_FIRMWARE("vx/bd563v2.boot");
    MODULE_FIRMWARE("vx/bd563s3.boot");
    MODULE_FIRMWARE("vx/l_1_vx2.d56");
    MODULE_FIRMWARE("vx/l_1_v22.d56");
    MODULE_FIRMWARE("vx/l_1_vxp.d56");
    MODULE_FIRMWARE("vx/l_1_vp4.d56");
#[no_mangle]
pub unsafe extern "C" fn snd_vx_setup_firmware(chip: *mut vx_core) -> c_int {
    int snd_vx_setup_firmware(struct vx_core *chip)
    {
    static const char * const fw_files[VX_TYPE_NUMS][4] = {
    [VX_TYPE_BOARD] = {
    core::ptr::null_mut(), "x1_1_vx2.xlx", "bd56002.boot", "l_1_vx2.d56",
    },
    [VX_TYPE_V2] = {
    core::ptr::null_mut(), "x1_2_v22.xlx", "bd563v2.boot", "l_1_v22.d56",
    },
    [VX_TYPE_MIC] = {
    core::ptr::null_mut(), "x1_2_v22.xlx", "bd563v2.boot", "l_1_v22.d56",
    },
    [VX_TYPE_VXPOCKET] = {
    "bx_1_vxp.b56", "x1_1_vxp.xlx", "bd563s3.boot", "l_1_vxp.d56"
    },
    [VX_TYPE_VXP440] = {
    "bx_1_vp4.b56", "x1_1_vp4.xlx", "bd563s3.boot", "l_1_vp4.d56"
    },
    };
    int i, err;
    for (i = 0; i < 4; i++) {
    char path[32];
    const struct firmware *fw;
    if (! fw_files[chip.type][i])
    continue;
    sprintf(path, "vx/%s", fw_files[chip.type][i]);
    if (request_firmware(&fw, path, chip.card.dev)) {
    dev_err(chip.card.dev, "vx: can't load firmware %s\n", path);
    return -ENOENT;
    }
    err = chip.ops.load_dsp(chip, i, fw);
    if (err < 0) {
    release_firmware(fw);
    return err;
    }
    if (i == 1)
    chip.chip_status |= VX_STAT_XILINX_LOADED;

    chip.firmware[i] = fw;

    release_firmware(fw);

    }
// ok, we reached to the last one
// create the devices if not built yet
    err = snd_vx_pcm_new(chip);
    if (err < 0)
    return err;
    err = snd_vx_mixer_new(chip);
    if (err < 0)
    return err;
    if (chip.ops.add_controls) {
    err = chip.ops.add_controls(chip);
    if (err < 0)
    return err;
    }
    chip.chip_status |= VX_STAT_DEVICE_INIT;
    chip.chip_status |= VX_STAT_CHIP_INIT;
    return snd_card_register(chip.card);
    }
// exported
#[no_mangle]
pub unsafe extern "C" fn snd_vx_free_firmware(chip: *mut vx_core) {
    void snd_vx_free_firmware(struct vx_core *chip)
    {

    int i;
    for (i = 0; i < 4; i++)
    release_firmware(chip.firmware[i]);

    }
    EXPORT_SYMBOL(snd_vx_setup_firmware);
    EXPORT_SYMBOL(snd_vx_free_firmware);
