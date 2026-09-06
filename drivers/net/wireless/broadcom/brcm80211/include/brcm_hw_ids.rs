//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/include/brcm_hw_ids.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

pub const BRCM_USB_VENDOR_ID_BROADCOM: c_uint = 0x0a5c;
pub const BRCM_USB_VENDOR_ID_LG: c_uint = 0x043e;
pub const BRCM_USB_VENDOR_ID_LINKSYS: c_uint = 0x13b1;
pub const CY_USB_VENDOR_ID_CYPRESS: c_uint = 0x04b4;

// Chipcommon Core Chip IDs
pub const BRCM_CC_43143_CHIP_ID: c_int = 43143;
pub const BRCM_CC_43235_CHIP_ID: c_int = 43235;
pub const BRCM_CC_43236_CHIP_ID: c_int = 43236;
pub const BRCM_CC_43238_CHIP_ID: c_int = 43238;
pub const BRCM_CC_43241_CHIP_ID: c_uint = 0x4324;
pub const BRCM_CC_43242_CHIP_ID: c_int = 43242;
pub const BRCM_CC_4329_CHIP_ID: c_uint = 0x4329;
pub const BRCM_CC_4330_CHIP_ID: c_uint = 0x4330;
pub const BRCM_CC_4334_CHIP_ID: c_uint = 0x4334;
pub const BRCM_CC_43340_CHIP_ID: c_int = 43340;
pub const BRCM_CC_43341_CHIP_ID: c_int = 43341;
pub const BRCM_CC_43362_CHIP_ID: c_int = 43362;
pub const BRCM_CC_4335_CHIP_ID: c_uint = 0x4335;
pub const BRCM_CC_4339_CHIP_ID: c_uint = 0x4339;
pub const BRCM_CC_43430_CHIP_ID: c_int = 43430;
pub const BRCM_CC_4345_CHIP_ID: c_uint = 0x4345;
pub const BRCM_CC_43454_CHIP_ID: c_int = 43454;
pub const BRCM_CC_43465_CHIP_ID: c_int = 43465;
pub const BRCM_CC_4350_CHIP_ID: c_uint = 0x4350;
pub const BRCM_CC_43525_CHIP_ID: c_int = 43525;
pub const BRCM_CC_4354_CHIP_ID: c_uint = 0x4354;
pub const BRCM_CC_4355_CHIP_ID: c_uint = 0x4355;
pub const BRCM_CC_4356_CHIP_ID: c_uint = 0x4356;
pub const BRCM_CC_43566_CHIP_ID: c_int = 43566;
pub const BRCM_CC_43567_CHIP_ID: c_int = 43567;
pub const BRCM_CC_43569_CHIP_ID: c_int = 43569;
pub const BRCM_CC_43570_CHIP_ID: c_int = 43570;
pub const BRCM_CC_4358_CHIP_ID: c_uint = 0x4358;
pub const BRCM_CC_4359_CHIP_ID: c_uint = 0x4359;
pub const BRCM_CC_43602_CHIP_ID: c_int = 43602;
pub const BRCM_CC_4364_CHIP_ID: c_uint = 0x4364;
pub const BRCM_CC_4365_CHIP_ID: c_uint = 0x4365;
pub const BRCM_CC_4366_CHIP_ID: c_uint = 0x4366;
pub const BRCM_CC_43664_CHIP_ID: c_int = 43664;
pub const BRCM_CC_43666_CHIP_ID: c_int = 43666;
pub const BRCM_CC_4371_CHIP_ID: c_uint = 0x4371;
pub const BRCM_CC_43751_CHIP_ID: c_int = 43751;
pub const BRCM_CC_43752_CHIP_ID: c_int = 43752;
pub const BRCM_CC_4377_CHIP_ID: c_uint = 0x4377;
pub const BRCM_CC_4378_CHIP_ID: c_uint = 0x4378;
pub const BRCM_CC_4387_CHIP_ID: c_uint = 0x4387;
pub const CY_CC_4373_CHIP_ID: c_uint = 0x4373;
pub const CY_CC_43012_CHIP_ID: c_int = 43012;
pub const CY_CC_43439_CHIP_ID: c_int = 43439;
// USB Device IDs
pub const BRCM_USB_43143_DEVICE_ID: c_uint = 0xbd1e;
pub const BRCM_USB_43235_LINKSYS_DEVICE_ID: c_uint = 0x0039;
pub const BRCM_USB_43236_DEVICE_ID: c_uint = 0xbd17;
pub const BRCM_USB_43242_DEVICE_ID: c_uint = 0xbd1f;
pub const BRCM_USB_43242_LG_DEVICE_ID: c_uint = 0x3101;
pub const BRCM_USB_43569_DEVICE_ID: c_uint = 0xbd27;
pub const BRCM_USB_BCMFW_DEVICE_ID: c_uint = 0x0bdc;
pub const CY_USB_4373_DEVICE_ID: c_uint = 0xbd29;
// PCIE Device IDs
pub const BRCM_PCIE_4350_DEVICE_ID: c_uint = 0x43a3;
pub const BRCM_PCIE_4354_DEVICE_ID: c_uint = 0x43df;
pub const BRCM_PCIE_4354_RAW_DEVICE_ID: c_uint = 0x4354;
pub const BRCM_PCIE_4355_DEVICE_ID: c_uint = 0x43dc;
pub const BRCM_PCIE_4356_DEVICE_ID: c_uint = 0x43ec;
pub const BRCM_PCIE_43567_DEVICE_ID: c_uint = 0x43d3;
pub const BRCM_PCIE_43570_DEVICE_ID: c_uint = 0x43d9;
pub const BRCM_PCIE_43570_RAW_DEVICE_ID: c_uint = 0xaa31;
pub const BRCM_PCIE_4358_DEVICE_ID: c_uint = 0x43e9;
pub const BRCM_PCIE_4359_DEVICE_ID: c_uint = 0x43ef;
pub const BRCM_PCIE_43602_DEVICE_ID: c_uint = 0x43ba;
pub const BRCM_PCIE_43602_2G_DEVICE_ID: c_uint = 0x43bb;
pub const BRCM_PCIE_43602_5G_DEVICE_ID: c_uint = 0x43bc;
pub const BRCM_PCIE_43602_RAW_DEVICE_ID: c_int = 43602;
pub const BRCM_PCIE_4364_DEVICE_ID: c_uint = 0x4464;
pub const BRCM_PCIE_4365_DEVICE_ID: c_uint = 0x43ca;
pub const BRCM_PCIE_4365_2G_DEVICE_ID: c_uint = 0x43cb;
pub const BRCM_PCIE_4365_5G_DEVICE_ID: c_uint = 0x43cc;
pub const BRCM_PCIE_4366_DEVICE_ID: c_uint = 0x43c3;
pub const BRCM_PCIE_4366_2G_DEVICE_ID: c_uint = 0x43c4;
pub const BRCM_PCIE_4366_5G_DEVICE_ID: c_uint = 0x43c5;
pub const BRCM_PCIE_4371_DEVICE_ID: c_uint = 0x440d;
pub const BRCM_PCIE_43596_DEVICE_ID: c_uint = 0x4415;
pub const BRCM_PCIE_43752_DEVICE_ID: c_uint = 0x449d;
pub const BRCM_PCIE_4377_DEVICE_ID: c_uint = 0x4488;
pub const BRCM_PCIE_4378_DEVICE_ID: c_uint = 0x4425;
pub const BRCM_PCIE_4387_DEVICE_ID: c_uint = 0x4433;
pub const CY_PCIE_54591_DEVICE_ID: c_uint = 0x4417;
// brcmsmac IDs
pub const BCM4313_D11N2G_ID: c_uint = 0x4727	/* 4313 802.11n 2.4G device */;
pub const BCM43224_D11N_ID: c_uint = 0x4353	/* 43224 802.11n dualband device */;
pub const BCM43224_D11N_ID_VEN1: c_uint = 0x0576	/* Vendor specific 43224 802.11n db */;
pub const BCM43225_D11N2G_ID: c_uint = 0x4357	/* 43225 802.11n 2.4GHz device */;
pub const BCM43236_D11N_ID: c_uint = 0x4346	/* 43236 802.11n dualband device */;
pub const BCM43236_D11N2G_ID: c_uint = 0x4347	/* 43236 802.11n 2.4GHz device */;
pub const BCM4313_CHIP_ID: c_uint = 0x4313;
pub const BCM43224_CHIP_ID: c_int = 43224;
