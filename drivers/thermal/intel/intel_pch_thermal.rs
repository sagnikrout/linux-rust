//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/intel_pch_thermal.c
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


// SPDX-License-Identifier: GPL-2.0-only
// intel_pch_thermal.c - Intel PCH Thermal driver
//
// Copyright (c) 2015, Intel Corporation.
//
// Authors:
// Tushar Dave <tushar.n.dave@intel.com>
//

// Intel PCH thermal Device IDs
pub const PCH_THERMAL_DID_HSW_1: c_uint = 0x9C24 /* Haswell PCH */;
pub const PCH_THERMAL_DID_HSW_2: c_uint = 0x8C24 /* Haswell PCH */;
pub const PCH_THERMAL_DID_WPT: c_uint = 0x9CA4 /* Wildcat Point */;
pub const PCH_THERMAL_DID_SKL: c_uint = 0x9D31 /* Skylake PCH */;
pub const PCH_THERMAL_DID_SKL_H: c_uint = 0xA131 /* Skylake PCH 100 series */;
pub const PCH_THERMAL_DID_CNL: c_uint = 0x9Df9 /* CNL PCH */;
pub const PCH_THERMAL_DID_CNL_H: c_uint = 0xA379 /* CNL-H PCH */;
pub const PCH_THERMAL_DID_CNL_LP: c_uint = 0x02F9 /* CNL-LP PCH */;

pub const PCH_THERMAL_DID_LWB: c_uint = 0xA1B1 /* Lewisburg PCH */;
pub const PCH_THERMAL_DID_WBG: c_uint = 0x8D24 /* Wellsburg PCH */;
// Wildcat Point-LP  PCH Thermal registers
pub const WPT_TEMP: c_uint = 0x0000	/* Temperature */;
pub const WPT_TSC: c_uint = 0x04	/* Thermal Sensor Control */;
pub const WPT_TSS: c_uint = 0x06	/* Thermal Sensor Status */;
pub const WPT_TSEL: c_uint = 0x08	/* Thermal Sensor Enable and Lock */;
pub const WPT_TSREL: c_uint = 0x0A	/* Thermal Sensor Report Enable and Lock */;
pub const WPT_TSMIC: c_uint = 0x0C	/* Thermal Sensor SMI Control */;
pub const WPT_CTT: c_uint = 0x0010	/* Catastrophic Trip Point */;
pub const WPT_TSPM: c_uint = 0x001C	/* Thermal Sensor Power Management */;
pub const WPT_TAHV: c_uint = 0x0014	/* Thermal Alert High Value */;
pub const WPT_TALV: c_uint = 0x0018	/* Thermal Alert Low Value */;
pub const WPT_TL: c_uint = 0x00000040	/* Throttle Value */;
pub const WPT_PHL: c_uint = 0x0060	/* PCH Hot Level */;
pub const WPT_PHLC: c_uint = 0x62	/* PHL Control */;
pub const WPT_TAS: c_uint = 0x80	/* Thermal Alert Status */;
pub const WPT_TSPIEN: c_uint = 0x82	/* PCI Interrupt Event Enables */;
pub const WPT_TSGPEN: c_uint = 0x84	/* General Purpose Event Enables */;
// Wildcat Point-LP  PCH Thermal Register bit definitions
pub const WPT_TEMP_TSR: c_uint = 0x01ff	/* Temp TS Reading */;
pub const WPT_TSC_CPDE: c_uint = 0x01	/* Catastrophic Power-Down Enable */;
pub const WPT_TSS_TSDSS: c_uint = 0x10	/* Thermal Sensor Dynamic Shutdown Status */;
pub const WPT_TSS_GPES: c_uint = 0x08	/* GPE status */;
pub const WPT_TSEL_ETS: c_uint = 0x01    /* Enable TS */;
pub const WPT_TSEL_PLDB: c_uint = 0x80	/* TSEL Policy Lock-Down Bit */;
pub const WPT_TL_TOL: c_uint = 0x000001FF	/* T0 Level */;
pub const WPT_TL_T1L: c_uint = 0x1ff00000	/* T1 Level */;
pub const WPT_TL_TTEN: c_uint = 0x20000000	/* TT Enable */;
// Resolution of 1/2 degree C and an offset of -50C

// Amount of time for each cooling delay, 100ms by default for now
    let mut delay_timeout: static unsigned int = 100;
    module_param(delay_timeout, int, 0644);
    MODULE_PARM_DESC(delay_timeout, "amount of time delay for each iteration.");
// Number of iterations for cooling delay, 600 counts by default for now
    let mut delay_cnt: static unsigned int = 600;
    module_param(delay_cnt, int, 0644);
    MODULE_PARM_DESC(delay_cnt, "total number of iterations for time delay.");
    static char driver_name[] = "Intel PCH thermal driver";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_thermal_device {
    pub hw_base: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub tzd: *mut thermal_zone_device,
    pub bios_enabled: bool,
}

//
// On some platforms, there is a companion ACPI device, which adds
// passive trip temperature using _PSV method. There is no specific
// passive temperature setting in MMIO interface of this PCI device.
//
    static int pch_wpt_add_acpi_psv_trip(struct pch_thermal_device *ptd,
    struct thermal_trip *trip)
    {
    struct acpi_device *adev;
    int temp;
    adev = ACPI_COMPANION(&ptd.pdev.dev);
    if (!adev)
    return 0;
    if (thermal_acpi_passive_trip_temp(adev, &temp) || temp <= 0)
    return 0;
    trip.type = THERMAL_TRIP_PASSIVE;
    trip.temperature = temp;
    return 1;
    }

    static int pch_wpt_add_acpi_psv_trip(struct pch_thermal_device *ptd,
    struct thermal_trip *trip)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pch_thermal_get_temp(tzd: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int pch_thermal_get_temp(struct thermal_zone_device *tzd, int *temp)
    {
    struct pch_thermal_device *ptd = thermal_zone_device_priv(tzd);
// temp = GET_WPT_TEMP(WPT_TEMP_TSR & readw(ptd->hw_base + WPT_TEMP));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pch_critical(tzd: *mut thermal_zone_device) {
    static void pch_critical(struct thermal_zone_device *tzd)
    {
    dev_dbg(thermal_zone_device(tzd), "%s: critical temperature reached\n",
    thermal_zone_device_type(tzd));
    }
    static const struct thermal_zone_device_ops tzd_ops = {
    .get_temp = pch_thermal_get_temp,
    .critical = pch_critical,
    };
    enum pch_board_ids {
    PCH_BOARD_HSW = 0,
    PCH_BOARD_WPT,
    PCH_BOARD_SKL,
    PCH_BOARD_CNL,
    PCH_BOARD_CML,
    PCH_BOARD_LWB,
    PCH_BOARD_WBG,
    };
    static const char *board_names[] = {
    [PCH_BOARD_HSW] = "pch_haswell",
    [PCH_BOARD_WPT] = "pch_wildcat_point",
    [PCH_BOARD_SKL] = "pch_skylake",
    [PCH_BOARD_CNL] = "pch_cannonlake",
    [PCH_BOARD_CML] = "pch_cometlake",
    [PCH_BOARD_LWB] = "pch_lewisburg",
    [PCH_BOARD_WBG] = "pch_wellsburg",
    };
    static int intel_pch_thermal_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct thermal_trip ptd_trips[PCH_MAX_TRIPS] = { 0 };
    let mut board_id: enum pch_board_ids = id.driver_data;
    struct pch_thermal_device *ptd;
    let mut nr_trips: c_int = 0;
    u16 trip_temp;
    u8 tsel;
    int err;
    ptd = devm_kzalloc(&pdev.dev, sizeof(*ptd), GFP_KERNEL);
    if (!ptd)
    return -ENOMEM;
    pci_set_drvdata(pdev, ptd);
    ptd.pdev = pdev;
    err = pci_enable_device(pdev);
    if (err) {
    dev_err(&pdev.dev, "failed to enable pci device\n");
    return err;
    }
    err = pci_request_regions(pdev, driver_name);
    if (err) {
    dev_err(&pdev.dev, "failed to request pci region\n");
    goto error_disable;
    }
    ptd.hw_base = pci_ioremap_bar(pdev, 0);
    if (!ptd.hw_base) {
    err = -ENOMEM;
    dev_err(&pdev.dev, "failed to map mem base\n");
    goto error_release;
    }
// Check if BIOS has already enabled thermal sensor
    if (WPT_TSEL_ETS & readb(ptd.hw_base + WPT_TSEL)) {
    ptd.bios_enabled = true;
    goto read_trips;
    }
    tsel = readb(ptd.hw_base + WPT_TSEL);
//
// When TSEL's Policy Lock-Down bit is 1, TSEL become RO.
// If so, thermal sensor cannot enable. Bail out.
//
    if (tsel & WPT_TSEL_PLDB) {
    dev_err(&ptd.pdev.dev, "Sensor can't be enabled\n");
    err = -ENODEV;
    goto error_cleanup;
    }
    writeb(tsel|WPT_TSEL_ETS, ptd.hw_base + WPT_TSEL);
    if (!(WPT_TSEL_ETS & readb(ptd.hw_base + WPT_TSEL))) {
    dev_err(&ptd.pdev.dev, "Sensor can't be enabled\n");
    err = -ENODEV;
    goto error_cleanup;
    }
    read_trips:
    trip_temp = readw(ptd.hw_base + WPT_CTT);
    trip_temp &= 0x1FF;
    if (trip_temp) {
    ptd_trips[nr_trips].temperature = GET_WPT_TEMP(trip_temp);
    ptd_trips[nr_trips++].type = THERMAL_TRIP_CRITICAL;
    }
    trip_temp = readw(ptd.hw_base + WPT_PHL);
    trip_temp &= 0x1FF;
    if (trip_temp) {
    ptd_trips[nr_trips].temperature = GET_WPT_TEMP(trip_temp);
    ptd_trips[nr_trips++].type = THERMAL_TRIP_HOT;
    }
    nr_trips += pch_wpt_add_acpi_psv_trip(ptd, &ptd_trips[nr_trips]);
    ptd.tzd = thermal_zone_device_register_with_trips(board_names[board_id],
    ptd_trips, nr_trips,
    ptd, &tzd_ops,
    core::ptr::null_mut(), 0, 0);
    if (IS_ERR(ptd.tzd)) {
    dev_err(&pdev.dev, "Failed to register thermal zone %s\n",
    board_names[board_id]);
    err = PTR_ERR(ptd.tzd);
    goto error_cleanup;
    }
    err = thermal_zone_device_enable(ptd.tzd);
    if (err)
    goto err_unregister;
    return 0;
    err_unregister:
    thermal_zone_device_unregister(ptd.tzd);
    error_cleanup:
    iounmap(ptd.hw_base);
    error_release:
    pci_release_regions(pdev);
    error_disable:
    pci_disable_device(pdev);
    dev_err(&pdev.dev, "pci device failed to probe\n");
    return err;
    }
#[no_mangle]
unsafe extern "C" fn intel_pch_thermal_remove(pdev: *mut pci_dev) {
    static void intel_pch_thermal_remove(struct pci_dev *pdev)
    {
    struct pch_thermal_device *ptd = pci_get_drvdata(pdev);
    thermal_zone_device_unregister(ptd.tzd);
    iounmap(ptd.hw_base);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
#[no_mangle]
unsafe extern "C" fn intel_pch_thermal_suspend_noirq(device: *mut device) -> c_int {
    static int intel_pch_thermal_suspend_noirq(struct device *device)
    {
    struct pch_thermal_device *ptd = dev_get_drvdata(device);
    u16 pch_thr_temp, pch_cur_temp;
    let mut pch_delay_cnt: c_int = 0;
    u8 tsel;
// Shutdown the thermal sensor if it is not enabled by BIOS
    if (!ptd.bios_enabled) {
    tsel = readb(ptd.hw_base + WPT_TSEL);
    writeb(tsel & 0xFE, ptd.hw_base + WPT_TSEL);
    return 0;
    }
// Do not check temperature if it is not s2idle
    if (pm_suspend_via_firmware())
    return 0;
// Get the PCH temperature threshold value
    pch_thr_temp = GET_PCH_TEMP(WPT_TEMP_TSR & readw(ptd.hw_base + WPT_TSPM));
// Get the PCH current temperature value
    pch_cur_temp = GET_PCH_TEMP(WPT_TEMP_TSR & readw(ptd.hw_base + WPT_TEMP));
    if (pch_cur_temp >= pch_thr_temp)
    dev_warn(&ptd.pdev.dev,
    "CPU-PCH current temp [%dC] higher than the threshold temp [%dC], S0ix might fail. Start cooling...\n",
    pch_cur_temp, pch_thr_temp);
//
// If current PCH temperature is higher than configured PCH threshold
// value, run some delay loop with sleep to let the current temperature
// go down below the threshold value which helps to allow system enter
// lower power S0ix suspend state. Even after delay loop if PCH current
// temperature stays above threshold, notify the warning message
// which helps to indentify the reason why S0ix entry was rejected.
//
    while (pch_delay_cnt < delay_cnt) {
    if (pch_cur_temp < pch_thr_temp)
    break;
    if (pm_wakeup_pending()) {
    dev_warn(&ptd.pdev.dev, "Wakeup event detected, abort cooling\n");
    return 0;
    }
    pch_delay_cnt++;
    dev_dbg(&ptd.pdev.dev,
    "CPU-PCH current temp [%dC] higher than the threshold temp [%dC], sleep %d times for %d ms duration\n",
    pch_cur_temp, pch_thr_temp, pch_delay_cnt, delay_timeout);
    msleep(delay_timeout);
// Read the PCH current temperature for next cycle.
    pch_cur_temp = GET_PCH_TEMP(WPT_TEMP_TSR & readw(ptd.hw_base + WPT_TEMP));
    }
    if (pch_cur_temp >= pch_thr_temp)
    dev_warn(&ptd.pdev.dev,
    "CPU-PCH is hot [%dC] after %d ms delay. S0ix might fail\n",
    pch_cur_temp, pch_delay_cnt * delay_timeout);
    else {
    if (pch_delay_cnt)
    dev_info(&ptd.pdev.dev,
    "CPU-PCH is cool [%dC] after %d ms delay\n",
    pch_cur_temp, pch_delay_cnt * delay_timeout);
    else
    dev_info(&ptd.pdev.dev,
    "CPU-PCH is cool [%dC]\n",
    pch_cur_temp);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_pch_thermal_resume(device: *mut device) -> c_int {
    static int intel_pch_thermal_resume(struct device *device)
    {
    struct pch_thermal_device *ptd = dev_get_drvdata(device);
    u8 tsel;
    if (ptd.bios_enabled)
    return 0;
    tsel = readb(ptd.hw_base + WPT_TSEL);
    writeb(tsel | WPT_TSEL_ETS, ptd.hw_base + WPT_TSEL);
    return 0;
    }
    static const struct pci_device_id intel_pch_thermal_id[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_HSW_1),
    .driver_data = PCH_BOARD_HSW, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_HSW_2),
    .driver_data = PCH_BOARD_HSW, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_WPT),
    .driver_data = PCH_BOARD_WPT, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_SKL),
    .driver_data = PCH_BOARD_SKL, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_SKL_H),
    .driver_data = PCH_BOARD_SKL, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_CNL),
    .driver_data = PCH_BOARD_CNL, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_CNL_H),
    .driver_data = PCH_BOARD_CNL, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_CNL_LP),
    .driver_data = PCH_BOARD_CNL, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_CML_H),
    .driver_data = PCH_BOARD_CML, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_LWB),
    .driver_data = PCH_BOARD_LWB, },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCH_THERMAL_DID_WBG),
    .driver_data = PCH_BOARD_WBG, },
    { 0, },
    };
    MODULE_DEVICE_TABLE(pci, intel_pch_thermal_id);
    static const struct dev_pm_ops intel_pch_pm_ops = {
    .suspend_noirq = intel_pch_thermal_suspend_noirq,
    .resume = intel_pch_thermal_resume,
    };
    static struct pci_driver intel_pch_thermal_driver = {
    .name		= "intel_pch_thermal",
    .id_table	= intel_pch_thermal_id,
    .probe		= intel_pch_thermal_probe,
    .remove		= intel_pch_thermal_remove,
    .driver.pm	= &intel_pch_pm_ops,
    };
    module_pci_driver(intel_pch_thermal_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Intel PCH Thermal driver");
