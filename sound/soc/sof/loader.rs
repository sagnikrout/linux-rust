//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/loader.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//
// Generic firmware loader.
//

#[no_mangle]
pub unsafe extern "C" fn snd_sof_load_firmware_raw(sdev: *mut snd_sof_dev) -> c_int {
    int snd_sof_load_firmware_raw(struct snd_sof_dev *sdev)
    {
    struct snd_sof_pdata *plat_data = sdev.pdata;
    const char *fw_filename;
    ssize_t ext_man_size;
    int ret;
// Don't request firmware again if firmware is already requested
    if (sdev.basefw.fw)
    return 0;
    fw_filename = kasprintf(GFP_KERNEL, "%s/%s",
    plat_data.fw_filename_prefix,
    plat_data.fw_filename);
    if (!fw_filename)
    return -ENOMEM;
    ret = request_firmware(&sdev.basefw.fw, fw_filename, sdev.dev);
    if (ret < 0) {
    dev_err(sdev.dev,
    "error: sof firmware file is missing, you might need to\n");
    dev_err(sdev.dev,
    "       download it from https://github.com/thesofproject/sof-bin/\n");
    goto err;
    } else {
    dev_dbg(sdev.dev, "request_firmware %s successful\n",
    fw_filename);
    }
// check for extended manifest
    ext_man_size = sdev.ipc.ops.fw_loader.parse_ext_manifest(sdev);
    if (ext_man_size > 0) {
// when no error occurred, drop extended manifest
    sdev.basefw.payload_offset = ext_man_size;
    } else if (!ext_man_size) {
// No extended manifest, so nothing to skip during FW load
    dev_dbg(sdev.dev, "firmware doesn't contain extended manifest\n");
    } else {
    ret = ext_man_size;
    dev_err(sdev.dev, "error: firmware %s contains unsupported or invalid extended manifest: %d\n",
    fw_filename, ret);
    }
    err:
    kfree(fw_filename);
    return ret;
    }
    EXPORT_SYMBOL(snd_sof_load_firmware_raw);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_load_firmware_memcpy(sdev: *mut snd_sof_dev) -> c_int {
    int snd_sof_load_firmware_memcpy(struct snd_sof_dev *sdev)
    {
    int ret;
    ret = snd_sof_load_firmware_raw(sdev);
    if (ret < 0)
    return ret;
// make sure the FW header and file is valid
    ret = sdev.ipc.ops.fw_loader.validate(sdev);
    if (ret < 0) {
    dev_err(sdev.dev, "error: invalid FW header\n");
    goto error;
    }
// prepare the DSP for FW loading
    ret = snd_sof_dsp_reset(sdev);
    if (ret < 0) {
    dev_err(sdev.dev, "error: failed to reset DSP\n");
    goto error;
    }
// parse and load firmware modules to DSP
    if (sdev.ipc.ops.fw_loader.load_fw_to_dsp) {
    ret = sdev.ipc.ops.fw_loader.load_fw_to_dsp(sdev);
    if (ret < 0) {
    dev_err(sdev.dev, "Firmware loading failed\n");
    goto error;
    }
    }
    return 0;
    error:
    release_firmware(sdev.basefw.fw);
    sdev.basefw.fw = core::ptr::null_mut();
    return ret;
    }
    EXPORT_SYMBOL(snd_sof_load_firmware_memcpy);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_run_firmware(sdev: *mut snd_sof_dev) -> c_int {
    int snd_sof_run_firmware(struct snd_sof_dev *sdev)
    {
    int ret;
    init_waitqueue_head(&sdev.boot_wait);
// (re-)enable dsp dump
    sdev.dbg_dump_printed = false;
    sdev.ipc_dump_printed = false;
// create read-only fw_version debugfs to store boot version info
    if (sdev.first_boot) {
    ret = snd_sof_debugfs_buf_item(sdev, &sdev.fw_version,
    sizeof(sdev.fw_version),
    "fw_version", 0444);
// errors are only due to memory allocation, not debugfs
    if (ret < 0) {
    dev_err(sdev.dev, "snd_sof_debugfs_buf_item failed\n");
    return ret;
    }
    }
// perform pre fw run operations
    ret = snd_sof_dsp_pre_fw_run(sdev);
    if (ret < 0) {
    dev_err(sdev.dev, "failed pre fw run op\n");
    return ret;
    }
    dev_dbg(sdev.dev, "booting DSP firmware\n");
// boot the firmware on the DSP
    ret = snd_sof_dsp_run(sdev);
    if (ret < 0) {
    snd_sof_dsp_dbg_dump(sdev, "Failed to start DSP",
    SOF_DBG_DUMP_MBOX | SOF_DBG_DUMP_PCI);
    return ret;
    }
//
// now wait for the DSP to boot. There are 3 possible outcomes:
// 1. Boot wait times out indicating FW boot failure.
// 2. FW boots successfully and fw_ready op succeeds.
// 3. FW boots but fw_ready op fails.
//
    ret = wait_event_timeout(sdev.boot_wait,
    sdev.fw_state > SOF_FW_BOOT_IN_PROGRESS,
    msecs_to_jiffies(sdev.boot_timeout));
    if (ret == 0) {
    snd_sof_dsp_dbg_dump(sdev, "Firmware boot failure due to timeout",
    SOF_DBG_DUMP_REGS | SOF_DBG_DUMP_MBOX |
    SOF_DBG_DUMP_TEXT | SOF_DBG_DUMP_PCI);
    return -EIO;
    }
    if (sdev.fw_state == SOF_FW_BOOT_READY_FAILED)
    return -EIO; /* FW boots but fw_ready op failed */
    dev_dbg(sdev.dev, "firmware boot complete\n");
    sof_set_fw_state(sdev, SOF_FW_BOOT_COMPLETE);
// perform post fw run operations
    ret = snd_sof_dsp_post_fw_run(sdev);
    if (ret < 0) {
    dev_err(sdev.dev, "error: failed post fw run op\n");
    return ret;
    }
    if (sdev.ipc.ops.post_fw_boot)
    return sdev.ipc.ops.post_fw_boot(sdev);
    return 0;
    }
    EXPORT_SYMBOL(snd_sof_run_firmware);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_fw_unload(sdev: *mut snd_sof_dev) {
    void snd_sof_fw_unload(struct snd_sof_dev *sdev)
    {
// TODO: support module unloading at runtime
    release_firmware(sdev.basefw.fw);
    sdev.basefw.fw = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(snd_sof_fw_unload);
