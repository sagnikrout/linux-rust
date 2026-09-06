//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/macsmc-reboot.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple SMC Reboot/Poweroff Handler
// Copyright The Asahi Linux Contributors
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsmc_reboot_nvmem {
    pub shutdown_flag: *mut nvmem_cell,
    pub boot_stage: *mut nvmem_cell,
    pub boot_error_count: *mut nvmem_cell,
    pub panic_count: *mut nvmem_cell,
}

    static const char * const nvmem_names[] = {
    "shutdown_flag",
    "boot_stage",
    "boot_error_count",
    "panic_count",
    };
    enum boot_stage {
    BOOT_STAGE_SHUTDOWN		= 0x00, /* Clean shutdown */
    BOOT_STAGE_IBOOT_DONE		= 0x2f, /* Last stage of bootloader */
    BOOT_STAGE_KERNEL_STARTED	= 0x30, /* Normal OS booting */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsmc_reboot {
    pub dev: *mut device,
    pub smc: *mut apple_smc,
    pub reboot_notify: notifier_block,
    union {
    pub nvm: macsmc_reboot_nvmem,
    pub nvm_cells: [*mut nvmem_cell; ARRAY_SIZE(nvmem_names)],
}

    };
// Helpers to read/write a u8 given a struct nvmem_cell
#[no_mangle]
unsafe extern "C" fn nvmem_cell_get_u8(cell: *mut nvmem_cell) -> c_int {
    static int nvmem_cell_get_u8(struct nvmem_cell *cell)
    {
    size_t len;
    void *bfr;
    u8 val;
    bfr = nvmem_cell_read(cell, &len);
    if (IS_ERR(bfr))
    return PTR_ERR(bfr);
    if (len < 1) {
    kfree(bfr);
    return -EINVAL;
    }
    val = *(u8 *)bfr;
    kfree(bfr);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn nvmem_cell_set_u8(cell: *mut nvmem_cell, val: u8) -> c_int {
    static int nvmem_cell_set_u8(struct nvmem_cell *cell, u8 val)
    {
    return nvmem_cell_write(cell, &val, sizeof(val));
    }
//
// SMC 'MBSE' key actions:
//
// 'offw' - shutdown warning
// 'slpw' - sleep warning
// 'rest' - restart warning
// 'off1' - shutdown (needs PMU bit set to stay on)
// 'susp' - suspend
// 'phra' - restart ("PE Halt Restart Action"?)
// 'panb' - panic beginning
// 'pane' - panic end
//
#[no_mangle]
unsafe extern "C" fn macsmc_prepare_atomic(data: *mut sys_off_data) -> c_int {
    static int macsmc_prepare_atomic(struct sys_off_data *data)
    {
    struct macsmc_reboot *reboot = data.cb_data;
    dev_info(reboot.dev, "Preparing SMC for atomic mode\n");
    apple_smc_enter_atomic(reboot.smc);
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn macsmc_power_off(data: *mut sys_off_data) -> c_int {
    static int macsmc_power_off(struct sys_off_data *data)
    {
    struct macsmc_reboot *reboot = data.cb_data;
    dev_info(reboot.dev, "Issuing power off (off1)\n");
    if (apple_smc_write_u32_atomic(reboot.smc, SMC_KEY(MBSE), SMC_KEY(off1)) < 0) {
    dev_err(reboot.dev, "Failed to issue MBSE = off1 (power_off)\n");
    } else {
    mdelay(100);
    WARN_ONCE(1, "Unable to power off system\n");
    }
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn macsmc_restart(data: *mut sys_off_data) -> c_int {
    static int macsmc_restart(struct sys_off_data *data)
    {
    struct macsmc_reboot *reboot = data.cb_data;
    dev_info(reboot.dev, "Issuing restart (phra)\n");
    if (apple_smc_write_u32_atomic(reboot.smc, SMC_KEY(MBSE), SMC_KEY(phra)) < 0) {
    dev_err(reboot.dev, "Failed to issue MBSE = phra (restart)\n");
    } else {
    mdelay(100);
    WARN_ONCE(1, "Unable to restart system\n");
    }
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn macsmc_reboot_notify(this: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
    static int macsmc_reboot_notify(struct notifier_block *this, unsigned long action, void *data)
    {
    struct macsmc_reboot *reboot = container_of(this, struct macsmc_reboot, reboot_notify);
    u8 shutdown_flag;
    u32 val;
    switch (action) {
    case SYS_RESTART:
    val = SMC_KEY(rest);
    shutdown_flag = 0;
    break;
    case SYS_POWER_OFF:
    val = SMC_KEY(offw);
    shutdown_flag = 1;
    break;
    default:
    return NOTIFY_DONE;
    }
    dev_info(reboot.dev, "Preparing for reboot (%p4ch)\n", &val);
// On the Mac Mini, this will turn off the LED for power off
    if (apple_smc_write_u32(reboot.smc, SMC_KEY(MBSE), val) < 0)
    dev_err(reboot.dev, "Failed to issue MBSE = %p4ch (reboot_prepare)\n", &val);
// Set the boot_stage to 0, which means we're doing a clean shutdown/reboot.
    if (reboot.nvm.boot_stage &&
    nvmem_cell_set_u8(reboot.nvm.boot_stage, BOOT_STAGE_SHUTDOWN) < 0)
    dev_err(reboot.dev, "Failed to write boot_stage\n");
//
// Set the PMU flag to actually reboot into the off state.
// Without this, the device will just reboot. We make it optional in case it is no longer
// necessary on newer hardware.
//
    if (reboot.nvm.shutdown_flag &&
    nvmem_cell_set_u8(reboot.nvm.shutdown_flag, shutdown_flag) < 0)
    dev_err(reboot.dev, "Failed to write shutdown_flag\n");
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn macsmc_power_init_error_counts(reboot: *mut macsmc_reboot) {
    static void macsmc_power_init_error_counts(struct macsmc_reboot *reboot)
    {
    int boot_error_count, panic_count;
    if (!reboot.nvm.boot_error_count || !reboot.nvm.panic_count)
    return;
    boot_error_count = nvmem_cell_get_u8(reboot.nvm.boot_error_count);
    if (boot_error_count < 0) {
    dev_err(reboot.dev, "Failed to read boot_error_count (%d)\n", boot_error_count);
    return;
    }
    panic_count = nvmem_cell_get_u8(reboot.nvm.panic_count);
    if (panic_count < 0) {
    dev_err(reboot.dev, "Failed to read panic_count (%d)\n", panic_count);
    return;
    }
    if (!boot_error_count && !panic_count)
    return;
    dev_warn(reboot.dev, "PMU logged %d boot error(s) and %d panic(s)\n",
    boot_error_count, panic_count);
    if (nvmem_cell_set_u8(reboot.nvm.panic_count, 0) < 0)
    dev_err(reboot.dev, "Failed to reset panic_count\n");
    if (nvmem_cell_set_u8(reboot.nvm.boot_error_count, 0) < 0)
    dev_err(reboot.dev, "Failed to reset boot_error_count\n");
    }
#[no_mangle]
unsafe extern "C" fn macsmc_reboot_probe(pdev: *mut platform_device) -> c_int {
    static int macsmc_reboot_probe(struct platform_device *pdev)
    {
    struct apple_smc *smc = dev_get_drvdata(pdev.dev.parent);
    struct macsmc_reboot *reboot;
    int ret, i;
    reboot = devm_kzalloc(&pdev.dev, sizeof(*reboot), GFP_KERNEL);
    if (!reboot)
    return -ENOMEM;
    reboot.dev = &pdev.dev;
    reboot.smc = smc;
    platform_set_drvdata(pdev, reboot);
    for (i = 0; i < ARRAY_SIZE(nvmem_names); i++) {
    struct nvmem_cell *cell;
    cell = devm_nvmem_cell_get(&pdev.dev,
    nvmem_names[i]);
    if (IS_ERR(cell)) {
    if (PTR_ERR(cell) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    dev_warn(&pdev.dev, "Missing NVMEM cell %s (%ld)\n",
    nvmem_names[i], PTR_ERR(cell));
// Non fatal, we'll deal with it
    cell = core::ptr::null_mut();
    }
    reboot.nvm_cells[i] = cell;
    }
// Set the boot_stage to indicate we're running the OS kernel
    if (reboot.nvm.boot_stage &&
    nvmem_cell_set_u8(reboot.nvm.boot_stage, BOOT_STAGE_KERNEL_STARTED) < 0)
    dev_err(reboot.dev, "Failed to write boot_stage\n");
// Display and clear the error counts
    macsmc_power_init_error_counts(reboot);
    reboot.reboot_notify.notifier_call = macsmc_reboot_notify;
    ret = devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_POWER_OFF_PREPARE,
    SYS_OFF_PRIO_HIGH, macsmc_prepare_atomic, reboot);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to register power-off prepare handler\n");
    ret = devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_POWER_OFF, SYS_OFF_PRIO_HIGH,
    macsmc_power_off, reboot);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to register power-off handler\n");
    ret = devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_RESTART_PREPARE,
    SYS_OFF_PRIO_HIGH, macsmc_prepare_atomic, reboot);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to register restart prepare handler\n");
    ret = devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_RESTART, SYS_OFF_PRIO_HIGH,
    macsmc_restart, reboot);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to register restart handler\n");
    ret = devm_register_reboot_notifier(&pdev.dev, &reboot.reboot_notify);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to register reboot notifier\n");
    dev_info(&pdev.dev, "Handling reboot and poweroff requests via SMC\n");
    return 0;
    }
    static const struct of_device_id macsmc_reboot_of_table[] = {
    { .compatible = "apple,smc-reboot", },
    {}
    };
    MODULE_DEVICE_TABLE(of, macsmc_reboot_of_table);
    static struct platform_driver macsmc_reboot_driver = {
    .driver = {
    .name = "macsmc-reboot",
    .of_match_table = macsmc_reboot_of_table,
    },
    .probe = macsmc_reboot_probe,
    };
    module_platform_driver(macsmc_reboot_driver);
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_DESCRIPTION("Apple SMC reboot/poweroff driver");
    MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
