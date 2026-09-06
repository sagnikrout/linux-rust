//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/sdio_ids.h
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
// SDIO Classes, Interface Types, Manufacturer IDs, etc.
//
// Standard SDIO Function Interfaces
//
pub const SDIO_CLASS_NONE: c_uint = 0x00	/* Not a SDIO standard interface */;
pub const SDIO_CLASS_UART: c_uint = 0x01	/* standard UART interface */;
pub const SDIO_CLASS_BT_A: c_uint = 0x02	/* Type-A BlueTooth std interface */;
pub const SDIO_CLASS_BT_B: c_uint = 0x03	/* Type-B BlueTooth std interface */;
pub const SDIO_CLASS_GPS: c_uint = 0x04	/* GPS standard interface */;
pub const SDIO_CLASS_CAMERA: c_uint = 0x05	/* Camera standard interface */;
pub const SDIO_CLASS_PHS: c_uint = 0x06	/* PHS standard interface */;
pub const SDIO_CLASS_WLAN: c_uint = 0x07	/* WLAN interface */;
pub const SDIO_CLASS_ATA: c_uint = 0x08	/* Embedded SDIO-ATA std interface */;
pub const SDIO_CLASS_BT_AMP: c_uint = 0x09	/* Type-A Bluetooth AMP interface */;
//
// Vendors and devices.  Sort key: vendor first, device next.
//
pub const SDIO_VENDOR_ID_STE: c_uint = 0x0020;
pub const SDIO_DEVICE_ID_STE_CW1200: c_uint = 0x2280;
pub const SDIO_VENDOR_ID_INTEL: c_uint = 0x0089;
pub const SDIO_DEVICE_ID_INTEL_IWMC3200WIMAX: c_uint = 0x1402;
pub const SDIO_DEVICE_ID_INTEL_IWMC3200WIFI: c_uint = 0x1403;
pub const SDIO_DEVICE_ID_INTEL_IWMC3200TOP: c_uint = 0x1404;
pub const SDIO_DEVICE_ID_INTEL_IWMC3200GPS: c_uint = 0x1405;
pub const SDIO_DEVICE_ID_INTEL_IWMC3200BT: c_uint = 0x1406;
pub const SDIO_DEVICE_ID_INTEL_IWMC3200WIMAX_2G5: c_uint = 0x1407;
pub const SDIO_VENDOR_ID_CGUYS: c_uint = 0x0092;
pub const SDIO_DEVICE_ID_CGUYS_EW_CG1102GC: c_uint = 0x0004;
pub const SDIO_VENDOR_ID_TI: c_uint = 0x0097;
pub const SDIO_DEVICE_ID_TI_WL1271: c_uint = 0x4076;
pub const SDIO_VENDOR_ID_ATHEROS: c_uint = 0x0271;
pub const SDIO_DEVICE_ID_ATHEROS_AR6003_00: c_uint = 0x0300;
pub const SDIO_DEVICE_ID_ATHEROS_AR6003_01: c_uint = 0x0301;
pub const SDIO_DEVICE_ID_ATHEROS_AR6004_00: c_uint = 0x0400;
pub const SDIO_DEVICE_ID_ATHEROS_AR6004_01: c_uint = 0x0401;
pub const SDIO_DEVICE_ID_ATHEROS_AR6004_02: c_uint = 0x0402;
pub const SDIO_DEVICE_ID_ATHEROS_AR6004_18: c_uint = 0x0418;
pub const SDIO_DEVICE_ID_ATHEROS_AR6004_19: c_uint = 0x0419;
pub const SDIO_DEVICE_ID_ATHEROS_AR6005: c_uint = 0x050A;
pub const SDIO_DEVICE_ID_ATHEROS_QCA9377: c_uint = 0x0701;
pub const SDIO_VENDOR_ID_BROADCOM: c_uint = 0x02d0;
pub const SDIO_DEVICE_ID_BROADCOM_NINTENDO_WII: c_uint = 0x044b;
pub const SDIO_DEVICE_ID_BROADCOM_43241: c_uint = 0x4324;
pub const SDIO_DEVICE_ID_BROADCOM_4329: c_uint = 0x4329;
pub const SDIO_DEVICE_ID_BROADCOM_4330: c_uint = 0x4330;
pub const SDIO_DEVICE_ID_BROADCOM_4334: c_uint = 0x4334;
pub const SDIO_DEVICE_ID_BROADCOM_4335_4339: c_uint = 0x4335;
pub const SDIO_DEVICE_ID_BROADCOM_4339: c_uint = 0x4339;
pub const SDIO_DEVICE_ID_BROADCOM_4345: c_uint = 0x4345;
pub const SDIO_DEVICE_ID_BROADCOM_4354: c_uint = 0x4354;
pub const SDIO_DEVICE_ID_BROADCOM_CYPRESS_89359: c_uint = 0x4355;
pub const SDIO_DEVICE_ID_BROADCOM_4356: c_uint = 0x4356;
pub const SDIO_DEVICE_ID_BROADCOM_4359: c_uint = 0x4359;
pub const SDIO_DEVICE_ID_BROADCOM_CYPRESS_4373: c_uint = 0x4373;
pub const SDIO_DEVICE_ID_BROADCOM_CYPRESS_43012: c_uint = 0xa804;
pub const SDIO_DEVICE_ID_BROADCOM_43143: c_uint = 0xa887;
pub const SDIO_DEVICE_ID_BROADCOM_43340: c_uint = 0xa94c;
pub const SDIO_DEVICE_ID_BROADCOM_43341: c_uint = 0xa94d;
pub const SDIO_DEVICE_ID_BROADCOM_43362: c_uint = 0xa962;
pub const SDIO_DEVICE_ID_BROADCOM_43364: c_uint = 0xa9a4;
pub const SDIO_DEVICE_ID_BROADCOM_43430: c_uint = 0xa9a6;
pub const SDIO_DEVICE_ID_BROADCOM_43439: c_uint = 0xa9af;
pub const SDIO_DEVICE_ID_BROADCOM_43455: c_uint = 0xa9bf;
pub const SDIO_DEVICE_ID_BROADCOM_43751: c_uint = 0xaae7;
pub const SDIO_DEVICE_ID_BROADCOM_43752: c_uint = 0xaae8;
pub const SDIO_VENDOR_ID_CYPRESS: c_uint = 0x04b4;
pub const SDIO_DEVICE_ID_BROADCOM_CYPRESS_43439: c_uint = 0xbd3d;
pub const SDIO_VENDOR_ID_MARVELL: c_uint = 0x02df;
pub const SDIO_DEVICE_ID_MARVELL_LIBERTAS: c_uint = 0x9103;
pub const SDIO_DEVICE_ID_MARVELL_8688_WLAN: c_uint = 0x9104;
pub const SDIO_DEVICE_ID_MARVELL_8688_BT: c_uint = 0x9105;
pub const SDIO_DEVICE_ID_MARVELL_8786_WLAN: c_uint = 0x9116;
pub const SDIO_DEVICE_ID_MARVELL_8787_WLAN: c_uint = 0x9119;
pub const SDIO_DEVICE_ID_MARVELL_8787_BT: c_uint = 0x911a;
pub const SDIO_DEVICE_ID_MARVELL_8787_BT_AMP: c_uint = 0x911b;
pub const SDIO_DEVICE_ID_MARVELL_8797_F0: c_uint = 0x9128;
pub const SDIO_DEVICE_ID_MARVELL_8797_WLAN: c_uint = 0x9129;
pub const SDIO_DEVICE_ID_MARVELL_8797_BT: c_uint = 0x912a;
pub const SDIO_DEVICE_ID_MARVELL_8897_WLAN: c_uint = 0x912d;
pub const SDIO_DEVICE_ID_MARVELL_8897_BT: c_uint = 0x912e;
pub const SDIO_DEVICE_ID_MARVELL_8887_F0: c_uint = 0x9134;
pub const SDIO_DEVICE_ID_MARVELL_8887_WLAN: c_uint = 0x9135;
pub const SDIO_DEVICE_ID_MARVELL_8887_BT: c_uint = 0x9136;
pub const SDIO_DEVICE_ID_MARVELL_8801_WLAN: c_uint = 0x9139;
pub const SDIO_DEVICE_ID_MARVELL_8997_F0: c_uint = 0x9140;
pub const SDIO_DEVICE_ID_MARVELL_8997_WLAN: c_uint = 0x9141;
pub const SDIO_DEVICE_ID_MARVELL_8997_BT: c_uint = 0x9142;
pub const SDIO_DEVICE_ID_MARVELL_8977_WLAN: c_uint = 0x9145;
pub const SDIO_DEVICE_ID_MARVELL_8977_BT: c_uint = 0x9146;
pub const SDIO_DEVICE_ID_MARVELL_8987_WLAN: c_uint = 0x9149;
pub const SDIO_DEVICE_ID_MARVELL_8987_BT: c_uint = 0x914a;
pub const SDIO_DEVICE_ID_MARVELL_8978_WLAN: c_uint = 0x9159;
pub const SDIO_VENDOR_ID_MEDIATEK: c_uint = 0x037a;
pub const SDIO_DEVICE_ID_MEDIATEK_MT7663: c_uint = 0x7663;
pub const SDIO_DEVICE_ID_MEDIATEK_MT7668: c_uint = 0x7668;
pub const SDIO_DEVICE_ID_MEDIATEK_MT7902: c_uint = 0x790a;
pub const SDIO_DEVICE_ID_MEDIATEK_MT7961: c_uint = 0x7961;
pub const SDIO_VENDOR_ID_MICROCHIP_WILC: c_uint = 0x0296;
pub const SDIO_DEVICE_ID_MICROCHIP_WILC1000: c_uint = 0x5347;
pub const SDIO_VENDOR_ID_MORSEMICRO: c_uint = 0x325b;
pub const SDIO_DEVICE_ID_MORSEMICRO_MM8108: c_uint = 0x0809;
pub const SDIO_VENDOR_ID_NXP: c_uint = 0x0471;
pub const SDIO_DEVICE_ID_NXP_IW61X_BASE: c_uint = 0x0204;
pub const SDIO_DEVICE_ID_NXP_IW61X: c_uint = 0x0205;
pub const SDIO_VENDOR_ID_REALTEK: c_uint = 0x024c;
pub const SDIO_DEVICE_ID_REALTEK_RTW8723BS: c_uint = 0xb723;
pub const SDIO_DEVICE_ID_REALTEK_RTW8821BS: c_uint = 0xb821;
pub const SDIO_DEVICE_ID_REALTEK_RTW8822BS: c_uint = 0xb822;
pub const SDIO_DEVICE_ID_REALTEK_RTW8821CS: c_uint = 0xc821;
pub const SDIO_DEVICE_ID_REALTEK_RTW8822CS: c_uint = 0xc822;
pub const SDIO_DEVICE_ID_REALTEK_RTW8723DS_2ANT: c_uint = 0xd723;
pub const SDIO_DEVICE_ID_REALTEK_RTW8723DS_1ANT: c_uint = 0xd724;
pub const SDIO_DEVICE_ID_REALTEK_RTW8821DS: c_uint = 0xd821;
pub const SDIO_DEVICE_ID_REALTEK_RTW8723CS: c_uint = 0xb703;
pub const SDIO_VENDOR_ID_SIANO: c_uint = 0x039a;
pub const SDIO_DEVICE_ID_SIANO_NOVA_B0: c_uint = 0x0201;
pub const SDIO_DEVICE_ID_SIANO_NICE: c_uint = 0x0202;
pub const SDIO_DEVICE_ID_SIANO_VEGA_A0: c_uint = 0x0300;
pub const SDIO_DEVICE_ID_SIANO_VENICE: c_uint = 0x0301;
pub const SDIO_DEVICE_ID_SIANO_MING: c_uint = 0x0302;
pub const SDIO_DEVICE_ID_SIANO_PELE: c_uint = 0x0500;
pub const SDIO_DEVICE_ID_SIANO_RIO: c_uint = 0x0600;
pub const SDIO_DEVICE_ID_SIANO_DENVER_2160: c_uint = 0x0700;
pub const SDIO_DEVICE_ID_SIANO_DENVER_1530: c_uint = 0x0800;
pub const SDIO_DEVICE_ID_SIANO_NOVA_A0: c_uint = 0x1100;
pub const SDIO_DEVICE_ID_SIANO_STELLAR: c_uint = 0x5347;
pub const SDIO_VENDOR_ID_RSI: c_uint = 0x041b;
pub const SDIO_DEVICE_ID_RSI_9113: c_uint = 0x9330;
pub const SDIO_DEVICE_ID_RSI_9116: c_uint = 0x9116;
pub const SDIO_VENDOR_ID_TI_WL1251: c_uint = 0x104c;
pub const SDIO_DEVICE_ID_TI_WL1251: c_uint = 0x9066;
