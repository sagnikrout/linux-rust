//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/bcm/raspberrypi-power.c
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
// (C) 2015 Pengutronix, Alexander Aring <aar@pengutronix.de>
//
// Authors:
// Alexander Aring <aar@pengutronix.de>
// Eric Anholt <eric@anholt.net>
//

//
// Firmware indices for the old power domains interface.  Only a few
// of them were actually implemented.
//
pub const RPI_OLD_POWER_DOMAIN_USB: c_int = 3;
pub const RPI_OLD_POWER_DOMAIN_V3D: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_power_domain {
    pub domain: u32,
    pub enabled: bool,
    pub old_interface: bool,
    pub base: generic_pm_domain,
    pub fw: *mut rpi_firmware,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_power_domains {
    pub has_new_interface: bool,
    pub xlate: genpd_onecell_data,
    pub fw: *mut rpi_firmware,
    pub domains: [rpi_power_domain; RPI_POWER_DOMAIN_COUNT],
}

//
// Packet definition used by RPI_FIRMWARE_SET_POWER_STATE and
// RPI_FIRMWARE_SET_DOMAIN_STATE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_power_domain_packet {
    pub domain: u32,
    pub state: u32,
}

//
// Asks the firmware to enable or disable power on a specific power
// domain.
//
#[no_mangle]
unsafe extern "C" fn rpi_firmware_set_power(domain: *mut generic_pm_domain, on: bool) -> c_int {
    static int rpi_firmware_set_power(struct generic_pm_domain *domain, bool on)
    {
    struct rpi_power_domain *rpi_domain =
    container_of(domain, struct rpi_power_domain, base);
    let mut old_interface: bool = rpi_domain.old_interface;
    struct rpi_power_domain_packet packet;
    int ret;
    packet.domain = rpi_domain.domain;
    packet.state = on;
    ret = rpi_firmware_property(rpi_domain.fw, old_interface ?
    RPI_FIRMWARE_SET_POWER_STATE :
    RPI_FIRMWARE_SET_DOMAIN_STATE,
    &packet, sizeof(packet));
    if (ret)
    dev_err(&domain.dev, "Failed to set %s to %u (%d)\n",
    old_interface ? "power" : "domain", on, ret);
    else
    dev_dbg(&domain.dev, "Set %s to %u\n",
    old_interface ? "power" : "domain", on);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rpi_domain_off(domain: *mut generic_pm_domain) -> c_int {
    static int rpi_domain_off(struct generic_pm_domain *domain)
    {
    return rpi_firmware_set_power(domain, false);
    }
#[no_mangle]
unsafe extern "C" fn rpi_domain_on(domain: *mut generic_pm_domain) -> c_int {
    static int rpi_domain_on(struct generic_pm_domain *domain)
    {
    return rpi_firmware_set_power(domain, true);
    }
    static void rpi_common_init_power_domain(struct rpi_power_domains *rpi_domains,
    int xlate_index, const char *name)
    {
    struct rpi_power_domain *dom = &rpi_domains.domains[xlate_index];
    dom.fw = rpi_domains.fw;
    dom.base.name = name;
    dom.base.flags = GENPD_FLAG_ACTIVE_WAKEUP;
    dom.base.power_on = rpi_domain_on;
    dom.base.power_off = rpi_domain_off;
//
// Treat all power domains as off at boot.
//
// The firmware itself may be keeping some domains on, but
// from Linux's perspective all we control is the refcounts
// that we give to the firmware, and we can't ask the firmware
// to turn off something that we haven't ourselves turned on.
//
    pm_genpd_init(&dom.base, core::ptr::null_mut(), true);
    rpi_domains.xlate.domains[xlate_index] = &dom.base;
    }
    static void rpi_init_power_domain(struct rpi_power_domains *rpi_domains,
    int xlate_index, const char *name)
    {
    struct rpi_power_domain *dom = &rpi_domains.domains[xlate_index];
    if (!rpi_domains.has_new_interface)
    return;
// The DT binding index is the firmware's domain index minus one.
    dom.domain = xlate_index + 1;
    rpi_common_init_power_domain(rpi_domains, xlate_index, name);
    }
    static void rpi_init_old_power_domain(struct rpi_power_domains *rpi_domains,
    int xlate_index, int domain,
    const char *name)
    {
    struct rpi_power_domain *dom = &rpi_domains.domains[xlate_index];
    dom.old_interface = true;
    dom.domain = domain;
    rpi_common_init_power_domain(rpi_domains, xlate_index, name);
    }
//
// Detects whether the firmware supports the new power domains interface.
//
// The firmware doesn't actually return an error on an unknown tag,
// and just skips over it, so we do the detection by putting an
// unexpected value in the return field and checking if it was
// unchanged.
//
    static bool
    rpi_has_new_domain_support(struct rpi_power_domains *rpi_domains)
    {
    struct rpi_power_domain_packet packet;
    int ret;
    packet.domain = RPI_POWER_DOMAIN_ARM;
    packet.state = ~0;
    ret = rpi_firmware_property(rpi_domains.fw,
    RPI_FIRMWARE_GET_DOMAIN_STATE,
    &packet, sizeof(packet));
    let mut ret: return = = 0 && packet.state != ~0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_power_probe(pdev: *mut platform_device) -> c_int {
    static int rpi_power_probe(struct platform_device *pdev)
    {
    struct device_node *fw_np;
    struct device *dev = &pdev.dev;
    struct rpi_power_domains *rpi_domains;
    rpi_domains = devm_kzalloc(dev, sizeof(*rpi_domains), GFP_KERNEL);
    if (!rpi_domains)
    return -ENOMEM;
    rpi_domains.xlate.domains =
    devm_kcalloc(dev,
    RPI_POWER_DOMAIN_COUNT,
    sizeof(*rpi_domains.xlate.domains),
    GFP_KERNEL);
    if (!rpi_domains.xlate.domains)
    return -ENOMEM;
    rpi_domains.xlate.num_domains = RPI_POWER_DOMAIN_COUNT;
    fw_np = of_parse_phandle(pdev.dev.of_node, "firmware", 0);
    if (!fw_np) {
    dev_err(&pdev.dev, "no firmware node\n");
    return -ENODEV;
    }
    rpi_domains.fw = devm_rpi_firmware_get(&pdev.dev, fw_np);
    of_node_put(fw_np);
    if (!rpi_domains.fw)
    return -EPROBE_DEFER;
    rpi_domains.has_new_interface =
    rpi_has_new_domain_support(rpi_domains);
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_I2C0, "I2C0");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_I2C1, "I2C1");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_I2C2, "I2C2");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_VIDEO_SCALER,
    "VIDEO_SCALER");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_VPU1, "VPU1");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_HDMI, "HDMI");
//
// Use the old firmware interface for USB power, so that we
// can turn it on even if the firmware hasn't been updated.
//
    rpi_init_old_power_domain(rpi_domains, RPI_POWER_DOMAIN_USB,
    RPI_OLD_POWER_DOMAIN_USB, "USB");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_VEC, "VEC");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_JPEG, "JPEG");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_H264, "H264");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_V3D, "V3D");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_ISP, "ISP");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_UNICAM0, "UNICAM0");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_UNICAM1, "UNICAM1");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_CCP2RX, "CCP2RX");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_CSI2, "CSI2");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_CPI, "CPI");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_DSI0, "DSI0");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_DSI1, "DSI1");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_TRANSPOSER,
    "TRANSPOSER");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_CCP2TX, "CCP2TX");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_CDP, "CDP");
    rpi_init_power_domain(rpi_domains, RPI_POWER_DOMAIN_ARM, "ARM");
    of_genpd_add_provider_onecell(dev.of_node, &rpi_domains.xlate);
    platform_set_drvdata(pdev, rpi_domains);
    return 0;
    }
    static const struct of_device_id rpi_power_of_match[] = {
    { .compatible = "raspberrypi,bcm2835-power", },
    {},
    };
    MODULE_DEVICE_TABLE(of, rpi_power_of_match);
    static struct platform_driver rpi_power_driver = {
    .driver = {
    .name = "raspberrypi-power",
    .of_match_table = rpi_power_of_match,
    },
    .probe		= rpi_power_probe,
    };
    builtin_platform_driver(rpi_power_driver);
    MODULE_AUTHOR("Alexander Aring <aar@pengutronix.de>");
    MODULE_AUTHOR("Eric Anholt <eric@anholt.net>");
    MODULE_DESCRIPTION("Raspberry Pi power domain driver");
