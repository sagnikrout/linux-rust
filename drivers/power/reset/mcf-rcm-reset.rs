//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/mcf-rcm-reset.c
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
// Freescale ColdFire MCF5441x RCM power-on reason driver
//
// Copyright (C) 2026 Jean-Michel Hautbois <jeanmichel.hautbois@yoseli.org>
//

//
// Reset Status Register (RSR) layout, MCF54418 Reference Manual chapter
// 12.3.2. The register is 8-bit read-only and latches the cause (or
// causes) of the most recent reset until the next one occurs.
//
pub const MCF_RSR_SOFT: c_uint = 0x20	/* Last reset caused by software */;
pub const MCF_RSR_LOC: c_uint = 0x10	/* Last reset caused by PLL loss of clock */;
pub const MCF_RSR_POR: c_uint = 0x08	/* Last reset caused by power-on */;
pub const MCF_RSR_EXT: c_uint = 0x04	/* Last reset caused by external pin */;
pub const MCF_RSR_WDRCORE: c_uint = 0x02	/* Last reset caused by core watchdog */;
pub const MCF_RSR_LOL: c_uint = 0x01	/* Last reset caused by PLL loss of lock */;

    MCF_RSR_LOC | MCF_RSR_LOL | MCF_RSR_SOFT)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcf_rcm {
    pub reason: *const c_char,
}

//
// Decode RSR into a power_on_reason string.
//
// The MCF5441x Reset Status Register can latch several cause bits at the
// same time (Reference Manual chapter 12.3.2: "one or more status bits
// may be set at the same time"). A power-on, for example, also resets
// the PLL and may co-flag LOC and LOL during the boot sequence. The
// power_on_reason ABI carries a single string, so this routine picks
// one cause; the chosen priority surfaces the most explanatory one for
// diagnostics:
//
// POR              cold boot dominates any spurious co-flagged cause
// EXT              operator action via the RESET pin
// WDRCORE          core watchdog timeout, a fault to investigate
// LOC, LOL         PLL clock or lock failure, hardware fault
// SOFT             explicit software-requested reset
//
    static const char *mcf_rcm_decode(u8 rsr)
    {
    if (rsr & MCF_RSR_POR)
    return POWER_ON_REASON_REGULAR;
    if (rsr & MCF_RSR_EXT)
    return POWER_ON_REASON_RST_BTN;
    if (rsr & MCF_RSR_WDRCORE)
    return POWER_ON_REASON_WATCHDOG;
    if (rsr & (MCF_RSR_LOC | MCF_RSR_LOL))
    return POWER_ON_REASON_CPU_CLK_FAIL;
    if (rsr & MCF_RSR_SOFT)
    return POWER_ON_REASON_SOFTWARE;
    return POWER_ON_REASON_UNKNOWN;
    }
    static ssize_t power_on_reason_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct mcf_rcm *rcm = platform_get_drvdata(to_platform_device(dev));
    return sysfs_emit(buf, "%s\n", rcm.reason);
    }
    static DEVICE_ATTR_RO(power_on_reason);
    static struct attribute *mcf_rcm_attrs[] = {
    &dev_attr_power_on_reason.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(mcf_rcm);
#[no_mangle]
unsafe extern "C" fn mcf_rcm_probe(pdev: *mut platform_device) -> c_int {
    static int mcf_rcm_probe(struct platform_device *pdev)
    {
    struct mcf_rcm *rcm;
    void __iomem *base;
    u8 rsr;
    rcm = devm_kzalloc(&pdev.dev, sizeof(*rcm), GFP_KERNEL);
    if (!rcm)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    rsr = readb_relaxed(base);
    rcm.reason = mcf_rcm_decode(rsr);
    platform_set_drvdata(pdev, rcm);
    if (!(rsr & MCF_RSR_KNOWN_CAUSES))
    dev_warn(&pdev.dev, "Unknown reset cause (RSR=0x%02x)\n", rsr);
    else
    dev_info(&pdev.dev, "Starting after %s (RSR=0x%02x)\n",
    rcm.reason, rsr);
    return 0;
    }
    static const struct platform_device_id mcf_rcm_id[] = {
    { "mcf-rcm-reset" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, mcf_rcm_id);
    static struct platform_driver mcf_rcm_driver = {
    .probe = mcf_rcm_probe,
    .id_table = mcf_rcm_id,
    .driver = {
    .name = "mcf-rcm-reset",
    .dev_groups = mcf_rcm_groups,
    },
    };
    module_platform_driver(mcf_rcm_driver);
    MODULE_AUTHOR("Jean-Michel Hautbois <jeanmichel.hautbois@yoseli.org>");
    MODULE_DESCRIPTION("Freescale ColdFire RCM power-on reason driver");
    MODULE_LICENSE("GPL");
