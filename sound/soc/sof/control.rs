//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/control.c
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
// Mixer Controls

    int snd_sof_volume_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_mixer_control *sm = (struct soc_mixer_control *)kcontrol.private_value;
    struct snd_sof_control *scontrol = sm.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    if (tplg_ops && tplg_ops.control && tplg_ops.control.volume_get)
    return tplg_ops.control.volume_get(scontrol, ucontrol);
    return 0;
    }
    int snd_sof_volume_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_mixer_control *sm = (struct soc_mixer_control *)kcontrol.private_value;
    struct snd_sof_control *scontrol = sm.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    if (tplg_ops && tplg_ops.control && tplg_ops.control.volume_put)
    return tplg_ops.control.volume_put(scontrol, ucontrol);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_sof_volume_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    int snd_sof_volume_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
    {
    struct soc_mixer_control *sm = (struct soc_mixer_control *)kcontrol.private_value;
    struct snd_sof_control *scontrol = sm.dobj.private;
    let mut channels: c_uint = scontrol.num_channels;
    int platform_max;
    if (!sm.platform_max)
    sm.platform_max = sm.max;
    platform_max = sm.platform_max;
    if (platform_max == 1 && !strstr(kcontrol.id.name, " Volume"))
    uinfo.type = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    else
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = channels;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = platform_max - sm.min;
    return 0;
    }
    int snd_sof_switch_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_mixer_control *sm = (struct soc_mixer_control *)kcontrol.private_value;
    struct snd_sof_control *scontrol = sm.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    if (tplg_ops && tplg_ops.control && tplg_ops.control.switch_get)
    return tplg_ops.control.switch_get(scontrol, ucontrol);
    return 0;
    }
    int snd_sof_switch_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_mixer_control *sm = (struct soc_mixer_control *)kcontrol.private_value;
    struct snd_sof_control *scontrol = sm.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    if (tplg_ops && tplg_ops.control && tplg_ops.control.switch_put)
    return tplg_ops.control.switch_put(scontrol, ucontrol);
    return false;
    }
    int snd_sof_enum_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_enum *se = (struct soc_enum *)kcontrol.private_value;
    struct snd_sof_control *scontrol = se.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    if (tplg_ops && tplg_ops.control && tplg_ops.control.enum_get)
    return tplg_ops.control.enum_get(scontrol, ucontrol);
    return 0;
    }
    int snd_sof_enum_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_enum *se = (struct soc_enum *)kcontrol.private_value;
    struct snd_sof_control *scontrol = se.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    if (tplg_ops && tplg_ops.control && tplg_ops.control.enum_put)
    return tplg_ops.control.enum_put(scontrol, ucontrol);
    return false;
    }
    int snd_sof_bytes_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_bytes_ext *be = (struct soc_bytes_ext *)kcontrol.private_value;
    struct snd_sof_control *scontrol = be.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    if (tplg_ops && tplg_ops.control && tplg_ops.control.bytes_get)
    return tplg_ops.control.bytes_get(scontrol, ucontrol);
    return 0;
    }
    int snd_sof_bytes_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_bytes_ext *be = (struct soc_bytes_ext *)kcontrol.private_value;
    struct snd_sof_control *scontrol = be.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    if (tplg_ops && tplg_ops.control && tplg_ops.control.bytes_put)
    return tplg_ops.control.bytes_put(scontrol, ucontrol);
    return 0;
    }
    int snd_sof_bytes_ext_put(struct snd_kcontrol *kcontrol,
    const unsigned int __user *binary_data,
    unsigned int size)
    {
    struct soc_bytes_ext *be = (struct soc_bytes_ext *)kcontrol.private_value;
    struct snd_sof_control *scontrol = be.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
// make sure we have at least a header
    if (size < sizeof(struct snd_ctl_tlv))
    return -EINVAL;
    if (tplg_ops && tplg_ops.control && tplg_ops.control.bytes_ext_put)
    return tplg_ops.control.bytes_ext_put(scontrol, binary_data, size);
    return 0;
    }
    int snd_sof_bytes_ext_volatile_get(struct snd_kcontrol *kcontrol, unsigned int __user *binary_data,
    unsigned int size)
    {
    struct soc_bytes_ext *be = (struct soc_bytes_ext *)kcontrol.private_value;
    struct snd_sof_control *scontrol = be.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    int ret, err;
// ignore the ext_volatile_get call if the callbacks are not provided
    if (!tplg_ops || !tplg_ops.control ||
    !tplg_ops.control.bytes_ext_volatile_get)
    return 0;
    ret = pm_runtime_resume_and_get(scomp.dev);
    if (ret < 0 && ret != -EACCES) {
    dev_err_ratelimited(scomp.dev, "%s: failed to resume %d\n", __func__, ret);
    return ret;
    }
// Make sure the DSP/firmware is booted up
    ret = snd_sof_boot_dsp_firmware(sdev);
    if (!ret)
    ret = tplg_ops.control.bytes_ext_volatile_get(scontrol,
    binary_data,
    size);
    err = pm_runtime_put_autosuspend(scomp.dev);
    if (err < 0)
    dev_err_ratelimited(scomp.dev, "%s: failed to idle %d\n", __func__, err);
    return ret;
    }
    int snd_sof_bytes_ext_get(struct snd_kcontrol *kcontrol,
    unsigned int __user *binary_data,
    unsigned int size)
    {
    struct soc_bytes_ext *be = (struct soc_bytes_ext *)kcontrol.private_value;
    struct snd_sof_control *scontrol = be.dobj.private;
    struct snd_soc_component *scomp = scontrol.scomp;
    struct snd_sof_dev *sdev = snd_soc_component_get_drvdata(scomp);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    if (tplg_ops && tplg_ops.control && tplg_ops.control.bytes_ext_get)
    return tplg_ops.control.bytes_ext_get(scontrol, binary_data, size);
    return 0;
    }
