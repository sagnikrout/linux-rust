//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/atom/punit_atom_debug.c
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
//
// Intel SOC Punit device state debug driver
// Punit controls power management for North Complex devices (Graphics
// blocks, Image Signal Processing, video processing, display, DSP etc.)
//
// Copyright (c) 2015, Intel Corporation.
//

// Subsystem config/status Video processor
pub const VED_SS_PM0: c_uint = 0x32;
// Subsystem config/status ISP (Image Signal Processor)
pub const ISP_SS_PM0: c_uint = 0x39;
// Subsystem config/status Input/output controller
pub const MIO_SS_PM: c_uint = 0x3B;
// Shift bits for getting status for video, isp and i/o
pub const SSS_SHIFT: c_int = 24;
// Power gate status reg
pub const PWRGT_STATUS: c_uint = 0x61;
// Shift bits for getting status for graphics rendering
pub const RENDER_POS: c_int = 0;
// Shift bits for getting status for media control
pub const MEDIA_POS: c_int = 2;
// Shift bits for getting status for Valley View/Baytrail display
pub const VLV_DISPLAY_POS: c_int = 6;
// Subsystem config/status display for Cherry Trail SOC
pub const CHT_DSP_SSS: c_uint = 0x36;
// Shift bits for getting status for display
pub const CHT_DSP_SSS_POS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct punit_device {
    pub name: *mut c_char,
    pub reg: c_int,
    pub sss_pos: c_int,
}

    static const struct punit_device punit_device_tng[] = {
    { "DISPLAY",	CHT_DSP_SSS,	SSS_SHIFT },
    { "VED",	VED_SS_PM0,	SSS_SHIFT },
    { "ISP",	ISP_SS_PM0,	SSS_SHIFT },
    { "MIO",	MIO_SS_PM,	SSS_SHIFT },
    { core::ptr::null_mut() }
    };
    static const struct punit_device punit_device_byt[] = {
    { "GFX RENDER",	PWRGT_STATUS,	RENDER_POS },
    { "GFX MEDIA",	PWRGT_STATUS,	MEDIA_POS },
    { "DISPLAY",	PWRGT_STATUS,	VLV_DISPLAY_POS },
    { "VED",	VED_SS_PM0,	SSS_SHIFT },
    { "ISP",	ISP_SS_PM0,	SSS_SHIFT },
    { "MIO",	MIO_SS_PM,	SSS_SHIFT },
    { core::ptr::null_mut() }
    };
    static const struct punit_device punit_device_cht[] = {
    { "GFX RENDER",	PWRGT_STATUS,	RENDER_POS },
    { "GFX MEDIA",	PWRGT_STATUS,	MEDIA_POS },
    { "DISPLAY",	CHT_DSP_SSS,	CHT_DSP_SSS_POS },
    { "VED",	VED_SS_PM0,	SSS_SHIFT },
    { "ISP",	ISP_SS_PM0,	SSS_SHIFT },
    { "MIO",	MIO_SS_PM,	SSS_SHIFT },
    { core::ptr::null_mut() }
    };
    static const char * const dstates[] = {"D0", "D0i1", "D0i2", "D0i3"};
#[no_mangle]
unsafe extern "C" fn punit_dev_state_show(seq_file: *mut seq_file, unused: *mut c_void) -> c_int {
    static int punit_dev_state_show(struct seq_file *seq_file, void *unused)
    {
    u32 punit_pwr_status;
    struct punit_device *punit_devp = seq_file.private;
    int index;
    int status;
    seq_puts(seq_file, "\n\nPUNIT NORTH COMPLEX DEVICES :\n");
    while (punit_devp.name) {
    status = iosf_mbi_read(BT_MBI_UNIT_PMC, MBI_REG_READ,
    punit_devp.reg, &punit_pwr_status);
    if (status) {
    seq_printf(seq_file, "%9s : Read Failed\n",
    punit_devp.name);
    } else  {
    index = (punit_pwr_status >> punit_devp.sss_pos) & 3;
    seq_printf(seq_file, "%9s : %s\n", punit_devp.name,
    dstates[index]);
    }
    punit_devp++;
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(punit_dev_state);
    static struct dentry *punit_dbg_file;
#[no_mangle]
unsafe extern "C" fn punit_dbgfs_register(punit_device: *mut punit_device) {
    static void punit_dbgfs_register(struct punit_device *punit_device)
    {
    punit_dbg_file = debugfs_create_dir("punit_atom", core::ptr::null_mut());
    debugfs_create_file("dev_power_state", 0444, punit_dbg_file,
    punit_device, &punit_dev_state_fops);
    }
#[no_mangle]
unsafe extern "C" fn punit_dbgfs_unregister() {
    static void punit_dbgfs_unregister(void)
    {
    debugfs_remove_recursive(punit_dbg_file);
    }

    static const struct punit_device *punit_dev;
#[no_mangle]
unsafe extern "C" fn punit_s2idle_check() {
    static void punit_s2idle_check(void)
    {
    const struct punit_device *punit_devp;
    u32 punit_pwr_status, dstate;
    int status;
    for (punit_devp = punit_dev; punit_devp.name; punit_devp++) {
// Skip MIO, it is on till the very last moment
    if (punit_devp.reg == MIO_SS_PM)
    continue;
    status = iosf_mbi_read(BT_MBI_UNIT_PMC, MBI_REG_READ,
    punit_devp.reg, &punit_pwr_status);
    if (status) {
    pr_err("%s read failed\n", punit_devp.name);
    } else  {
    dstate = (punit_pwr_status >> punit_devp.sss_pos) & 3;
    if (!dstate)
    pr_err("%s is in D0 prior to s2idle\n", punit_devp.name);
    }
    }
    }
    static struct acpi_s2idle_dev_ops punit_s2idle_ops = {
    .check = punit_s2idle_check,
    };
#[no_mangle]
unsafe extern "C" fn punit_s2idle_check_register(punit_device: *mut punit_device) {
    static void punit_s2idle_check_register(struct punit_device *punit_device)
    {
    punit_dev = punit_device;
    acpi_register_lps0_dev(&punit_s2idle_ops);
    }
#[no_mangle]
unsafe extern "C" fn punit_s2idle_check_unregister() {
    static void punit_s2idle_check_unregister(void)
    {
    acpi_unregister_lps0_dev(&punit_s2idle_ops);
    }

    static void punit_s2idle_check_register(struct punit_device *punit_device) {}
    static void punit_s2idle_check_unregister(void) {}

    X86_MATCH_VFM_FEATURE(vfm, X86_FEATURE_MWAIT, data)
    static const struct x86_cpu_id intel_punit_cpu_ids[] = {
    X86_MATCH(INTEL_ATOM_SILVERMONT,	&punit_device_byt),
    X86_MATCH(INTEL_ATOM_SILVERMONT_MID,	&punit_device_tng),
    X86_MATCH(INTEL_ATOM_AIRMONT,		&punit_device_cht),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, intel_punit_cpu_ids);
#[no_mangle]
unsafe extern "C" fn punit_atom_debug_init() -> int __init {
    static int __init punit_atom_debug_init(void)
    {
    struct punit_device *punit_device;
    const struct x86_cpu_id *id;
    id = x86_match_cpu(intel_punit_cpu_ids);
    if (!id)
    return -ENODEV;
    punit_device = (struct punit_device *)id.driver_data;
    punit_dbgfs_register(punit_device);
    punit_s2idle_check_register(punit_device);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn punit_atom_debug_exit() -> void __exit {
    static void __exit punit_atom_debug_exit(void)
    {
    punit_s2idle_check_unregister();
    punit_dbgfs_unregister();
    }
    module_init(punit_atom_debug_init);
    module_exit(punit_atom_debug_exit);
    MODULE_AUTHOR("Kumar P, Mahesh <mahesh.kumar.p@intel.com>");
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_DESCRIPTION("Driver for Punit devices states debugging");
    MODULE_LICENSE("GPL v2");
