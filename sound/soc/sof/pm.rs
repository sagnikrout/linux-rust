//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/pm.c
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

    let mut override_on_demand_boot: static int = -1;
    module_param_named(on_demand_boot, override_on_demand_boot, int, 0444);
    MODULE_PARM_DESC(on_demand_boot, "Force on-demand DSP boot: 0 - disabled, 1 - enabled");
//
// Helper function to determine the target DSP state during
// system suspend. This function only cares about the device
// D-states. Platform-specific substates, if any, should be
// handled by the platform-specific parts.
//
#[no_mangle]
unsafe extern "C" fn snd_sof_dsp_power_target(sdev: *mut snd_sof_dev) -> u32 {
    static u32 snd_sof_dsp_power_target(struct snd_sof_dev *sdev)
    {
    u32 target_dsp_state;
    switch (sdev.system_suspend_target) {
    case SOF_SUSPEND_S5:
    case SOF_SUSPEND_S4:
// DSP should be in D3 if the system is suspending to S3+
    case SOF_SUSPEND_S3:
// DSP should be in D3 if the system is suspending to S3
    target_dsp_state = SOF_DSP_PM_D3;
    break;
    case SOF_SUSPEND_S0IX:
//
// Currently, the only criterion for retaining the DSP in D0
// is that there are streams that ignored the suspend trigger.
// Additional criteria such Soundwire clock-stop mode and
// device suspend latency considerations will be added later.
//
    if (snd_sof_stream_suspend_ignored(sdev))
    target_dsp_state = SOF_DSP_PM_D0;
    else
    target_dsp_state = SOF_DSP_PM_D3;
    break;
    default:
// This case would be during runtime suspend
    target_dsp_state = SOF_DSP_PM_D3;
    break;
    }
    return target_dsp_state;
    }

#[no_mangle]
unsafe extern "C" fn sof_cache_debugfs(sdev: *mut snd_sof_dev) {
    static void sof_cache_debugfs(struct snd_sof_dev *sdev)
    {
    struct snd_sof_dfsentry *dfse;
    list_for_each_entry(dfse, &sdev.dfsentry_list, list) {
// nothing to do if debugfs buffer is not IO mem
    if (dfse.type == SOF_DFSENTRY_TYPE_BUF)
    continue;
// cache memory that is only accessible in D0
    if (dfse.access_type == SOF_DEBUGFS_ACCESS_D0_ONLY)
    memcpy_fromio(dfse.cache_buf, dfse.io_mem,
    dfse.size);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int {
    int snd_sof_boot_dsp_firmware(struct snd_sof_dev *sdev)
    {
    const struct sof_ipc_pm_ops *pm_ops = sof_ipc_get_ops(sdev, pm);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    int ret;
    guard(mutex)(&sdev.dsp_fw_boot_mutex);
    if (sdev.fw_state == SOF_FW_BOOT_COMPLETE) {
// Firmware already booted, just return
    return 0;
    }
    dev_dbg(sdev.dev, "Booting DSP firmware\n");
    sof_set_fw_state(sdev, SOF_FW_BOOT_PREPARE);
// load the firmware
    ret = snd_sof_load_firmware(sdev);
    if (ret < 0) {
    dev_err(sdev.dev, "%s: failed to load DSP firmware: %d\n",
    __func__, ret);
    sof_set_fw_state(sdev, SOF_FW_BOOT_FAILED);
    return ret;
    }
    sof_set_fw_state(sdev, SOF_FW_BOOT_IN_PROGRESS);
//
// Boot the firmware. The FW boot status will be modified
// in snd_sof_run_firmware() depending on the outcome.
//
    ret = snd_sof_run_firmware(sdev);
    if (ret < 0) {
    dev_err(sdev.dev, "%s: failed to boot DSP firmware: %d\n",
    __func__, ret);
    sof_set_fw_state(sdev, SOF_FW_BOOT_FAILED);
    return ret;
    }
// resume DMA trace
    ret = sof_fw_trace_resume(sdev);
    if (ret < 0) {
// non fatal
    dev_warn(sdev.dev, "%s: failed to resume trace: %d\n",
    __func__, ret);
    }
// restore pipelines
    if (tplg_ops && tplg_ops.set_up_all_pipelines) {
    ret = tplg_ops.set_up_all_pipelines(sdev, false);
    if (ret < 0) {
    dev_err(sdev.dev, "%s: failed to restore pipeline: %d\n",
    __func__, ret);
    goto setup_fail;
    }
    }
// Notify clients not managed by pm framework about core resume
    sof_resume_clients(sdev);
// notify DSP of system resume
    if (pm_ops && pm_ops.ctx_restore) {
    ret = pm_ops.ctx_restore(sdev);
    if (ret < 0)
    dev_err(sdev.dev, "%s: ctx_restore IPC failed: %d\n",
    __func__, ret);
    }
    setup_fail:

    if (ret < 0) {
//
// Debugfs cannot be read in runtime suspend, so cache
// the contents upon failure. This allows to capture
// possible DSP coredump information.
//
    sof_cache_debugfs(sdev);
    }

    return ret;
    }
    EXPORT_SYMBOL(snd_sof_boot_dsp_firmware);
#[no_mangle]
unsafe extern "C" fn sof_resume(dev: *mut device, runtime_resume: bool) -> c_int {
    static int sof_resume(struct device *dev, bool runtime_resume)
    {
    struct snd_sof_dev *sdev = dev_get_drvdata(dev);
    let mut old_state: u32 = sdev.dsp_power_state.state;
    bool on_demand_boot;
    int ret;
// do nothing if dsp resume callbacks are not set
    if (!runtime_resume && !sof_ops(sdev).resume)
    return 0;
    if (runtime_resume && !sof_ops(sdev).runtime_resume)
    return 0;
// DSP was never successfully started, nothing to resume
    if (sdev.first_boot)
    return 0;
//
// if the runtime_resume flag is set, call the runtime_resume routine
// or else call the system resume routine
//
    if (runtime_resume)
    ret = snd_sof_dsp_runtime_resume(sdev);
    else
    ret = snd_sof_dsp_resume(sdev);
    if (ret < 0) {
    dev_err(sdev.dev,
    "error: failed to power up DSP after resume\n");
    return ret;
    }
    if (sdev.dspless_mode_selected) {
    sof_set_fw_state(sdev, SOF_DSPLESS_MODE);
    return 0;
    }
//
// Nothing further to be done for platforms that support the low power
// D0 substate. Resume trace and return when resuming from
// low-power D0 substate
//
    if (!runtime_resume && sof_ops(sdev).set_power_state &&
    old_state == SOF_DSP_PM_D0) {
    ret = sof_fw_trace_resume(sdev);
    if (ret < 0)
// non fatal
    dev_warn(sdev.dev,
    "failed to enable trace after resume %d\n", ret);
    return 0;
    }
    if (override_on_demand_boot > -1)
    on_demand_boot = override_on_demand_boot ? true : false;
    else
    on_demand_boot = sdev.pdata.desc.on_demand_dsp_boot;
    if (on_demand_boot) {
// Only change the fw_state to PREPARE but skip booting
    sof_set_fw_state(sdev, SOF_FW_BOOT_PREPARE);
    return 0;
    }
    return snd_sof_boot_dsp_firmware(sdev);
    }
#[no_mangle]
unsafe extern "C" fn sof_suspend(dev: *mut device, runtime_suspend: bool) -> c_int {
    static int sof_suspend(struct device *dev, bool runtime_suspend)
    {
    struct snd_sof_dev *sdev = dev_get_drvdata(dev);
    const struct sof_ipc_pm_ops *pm_ops = sof_ipc_get_ops(sdev, pm);
    const struct sof_ipc_tplg_ops *tplg_ops = sof_ipc_get_ops(sdev, tplg);
    pm_message_t pm_state;
    let mut target_state: u32 = snd_sof_dsp_power_target(sdev);
    let mut old_state: u32 = sdev.dsp_power_state.state;
    int ret;
// do nothing if dsp suspend callback is not set
    if (!runtime_suspend && !sof_ops(sdev).suspend)
    return 0;
    if (runtime_suspend && !sof_ops(sdev).runtime_suspend)
    return 0;
// we need to tear down pipelines only if the DSP hardware is
// active, which happens for PCI devices. if the device is
// suspended, it is brought back to full power and then
// suspended again
//
    if (tplg_ops && tplg_ops.tear_down_all_pipelines && (old_state == SOF_DSP_PM_D0))
    tplg_ops.tear_down_all_pipelines(sdev, false);
    if (sdev.fw_state != SOF_FW_BOOT_COMPLETE)
    goto suspend;
// prepare for streams to be resumed properly upon resume
    if (!runtime_suspend) {
    ret = snd_sof_dsp_hw_params_upon_resume(sdev);
    if (ret < 0) {
    dev_err(sdev.dev,
    "error: setting hw_params flag during suspend %d\n",
    ret);
    return ret;
    }
    }
    pm_state.event = target_state;
// suspend DMA trace
    sof_fw_trace_suspend(sdev, pm_state);
// Notify clients not managed by pm framework about core suspend
    sof_suspend_clients(sdev, pm_state);
// Skip to platform-specific suspend if DSP is entering D0
    if (target_state == SOF_DSP_PM_D0)
    goto suspend;

// cache debugfs contents during runtime suspend
    if (runtime_suspend)
    sof_cache_debugfs(sdev);

// notify DSP of upcoming power down
    if (pm_ops && pm_ops.ctx_save) {
    ret = pm_ops.ctx_save(sdev);
    if (ret == -EBUSY || ret == -EAGAIN) {
//
// runtime PM has logic to handle -EBUSY/-EAGAIN so
// pass these errors up
//
    dev_err(sdev.dev, "ctx_save IPC error during suspend: %d\n", ret);
    return ret;
    } else if (ret < 0) {
// FW in unexpected state, continue to power down
    dev_warn(sdev.dev, "ctx_save IPC error: %d, proceeding with suspend\n",
    ret);
    }
    }
    suspend:
// return if the DSP was not probed successfully
    if (sdev.fw_state == SOF_FW_BOOT_NOT_STARTED)
    return 0;
// platform-specific suspend
    if (runtime_suspend)
    ret = snd_sof_dsp_runtime_suspend(sdev);
    else
    ret = snd_sof_dsp_suspend(sdev, target_state);
    if (ret < 0)
    dev_err(sdev.dev,
    "error: failed to power down DSP during suspend %d\n",
    ret);
// Do not reset FW state if DSP is in D0
    if (target_state == SOF_DSP_PM_D0)
    return ret;
// reset FW state
    sof_set_fw_state(sdev, SOF_FW_BOOT_NOT_STARTED);
    sdev.enabled_cores_mask = 0;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_sof_dsp_power_down_notify(sdev: *mut snd_sof_dev) -> c_int {
    int snd_sof_dsp_power_down_notify(struct snd_sof_dev *sdev)
    {
    const struct sof_ipc_pm_ops *pm_ops = sof_ipc_get_ops(sdev, pm);
//
// Notify DSP of upcoming power down only if the firmware has been
// booted up
//
    if (sdev.fw_state == SOF_FW_BOOT_COMPLETE && sof_ops(sdev).remove &&
    pm_ops && pm_ops.ctx_save)
    return pm_ops.ctx_save(sdev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_sof_runtime_suspend(dev: *mut device) -> c_int {
    int snd_sof_runtime_suspend(struct device *dev)
    {
    return sof_suspend(dev, true);
    }
    EXPORT_SYMBOL(snd_sof_runtime_suspend);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_runtime_idle(dev: *mut device) -> c_int {
    int snd_sof_runtime_idle(struct device *dev)
    {
    struct snd_sof_dev *sdev = dev_get_drvdata(dev);
    return snd_sof_dsp_runtime_idle(sdev);
    }
    EXPORT_SYMBOL(snd_sof_runtime_idle);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_runtime_resume(dev: *mut device) -> c_int {
    int snd_sof_runtime_resume(struct device *dev)
    {
    return sof_resume(dev, true);
    }
    EXPORT_SYMBOL(snd_sof_runtime_resume);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_resume(dev: *mut device) -> c_int {
    int snd_sof_resume(struct device *dev)
    {
    return sof_resume(dev, false);
    }
    EXPORT_SYMBOL(snd_sof_resume);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_suspend(dev: *mut device) -> c_int {
    int snd_sof_suspend(struct device *dev)
    {
    return sof_suspend(dev, false);
    }
    EXPORT_SYMBOL(snd_sof_suspend);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_prepare(dev: *mut device) -> c_int {
    int snd_sof_prepare(struct device *dev)
    {
    struct snd_sof_dev *sdev = dev_get_drvdata(dev);
    const struct sof_dev_desc *desc = sdev.pdata.desc;
// will suspend to S3 by default
    sdev.system_suspend_target = SOF_SUSPEND_S3;
//
// if the firmware is crashed or boot failed then we try to aim for S3
// to reboot the firmware
//
    if (sdev.fw_state == SOF_FW_CRASHED ||
    sdev.fw_state == SOF_FW_BOOT_FAILED)
    return 0;
    if (!desc.use_acpi_target_states)
    return 0;

    switch (acpi_target_system_state()) {
    case ACPI_STATE_S0:
    sdev.system_suspend_target = SOF_SUSPEND_S0IX;
    break;
    case ACPI_STATE_S1:
    case ACPI_STATE_S2:
    case ACPI_STATE_S3:
    sdev.system_suspend_target = SOF_SUSPEND_S3;
    break;
    case ACPI_STATE_S4:
    sdev.system_suspend_target = SOF_SUSPEND_S4;
    break;
    case ACPI_STATE_S5:
    sdev.system_suspend_target = SOF_SUSPEND_S5;
    break;
    default:
    break;
    }

    return 0;
    }
    EXPORT_SYMBOL(snd_sof_prepare);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_complete(dev: *mut device) {
    void snd_sof_complete(struct device *dev)
    {
    struct snd_sof_dev *sdev = dev_get_drvdata(dev);
    sdev.system_suspend_target = SOF_SUSPEND_NONE;
    }
    EXPORT_SYMBOL(snd_sof_complete);
