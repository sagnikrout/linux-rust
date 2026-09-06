//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/brcmfmac.h
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


//
// Copyright (c) 2016 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const BRCMFMAC_COUNTRY_BUF_SZ: c_int = 4;
//
// Platform specific driver functions and data. Through the platform specific
// device data functions and data can be provided to help the brcmfmac driver to
// operate with the device in combination with the used platform.
//
// Note: the brcmfmac can be loaded as module or be statically built-in into
// the kernel. If built-in then do note that it uses module_init (and
// module_exit) routines which equal device_initcall. So if you intend to
// create a module with the platform specific data for the brcmfmac and have
// it built-in to the kernel then use a higher initcall then device_initcall
// (see init.h). If this is not done then brcmfmac will load without problems
// but will not pickup the platform data.
//
// When the driver does not "detect" platform driver data then it will continue
// without reporting anything and just assume there is no data needed. Which is
// probably true for most platforms.
//
// enum brcmf_bus_type - Bus type identifier. Currently SDIO, USB and PCIE are
// supported.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_bus_type {
    BRCMF_BUSTYPE_SDIO,
    BRCMF_BUSTYPE_USB,
    BRCMF_BUSTYPE_PCIE
}

//
// struct brcmfmac_sdio_pd - SDIO Device specific platform data.
//
// @txglomsz:		SDIO txglom size. Use 0 if default of driver is to be
// used.
// @drive_strength:	is the preferred drive_strength to be used for the SDIO
// pins. If 0 then a default value will be used. This is
// the target drive strength, the exact drive strength
// which will be used depends on the capabilities of the
// device.
// @oob_irq_supported:	does the board have support for OOB interrupts. SDIO
// in-band interrupts are relatively slow and for having
// less overhead on interrupt processing an out of band
// interrupt can be used. If the HW supports this then
// enable this by setting this field to true and configure
// the oob related fields.
// @oob_irq_nr,
// @oob_irq_flags:	the OOB interrupt information. The values are used for
// registering the irq using request_irq function.
// @broken_sg_support:	flag for broken sg list support of SDIO host controller.
// Set this to true if the SDIO host controller has higher
// align requirement than 32 bytes for each scatterlist
// item.
// @sd_head_align:	alignment requirement for start of data buffer.
// @sd_sgentry_align:	length alignment requirement for each sg entry.
// @reset:		This function can get called if the device communication
// broke down. This functionality is particularly useful in
// case of SDIO type devices. It is possible to reset a
// dongle via sdio data interface, but it requires that
// this is fully functional. This function is chip/module
// specific and this function should return only after the
// complete reset has completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmfmac_sdio_pd {
    pub txglomsz: c_int,
    pub drive_strength: c_uint,
    pub oob_irq_supported: bool,
    pub oob_irq_nr: c_uint,
    pub oob_irq_flags: c_ulong,
    pub broken_sg_support: bool,
    pub sd_head_align: c_ushort,
    pub sd_sgentry_align: c_ushort,
    pub (*reset)(void): *mut c_void,
}

//
// struct brcmfmac_pd_cc_entry - Struct for translating user space country code
// (iso3166) to firmware country code and
// revision.
//
// @iso3166:	iso3166 alpha 2 country code string.
// @cc:		firmware country code string.
// @rev:	firmware country code revision.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmfmac_pd_cc_entry {
    pub iso3166: [c_char; BRCMFMAC_COUNTRY_BUF_SZ],
    pub cc: [c_char; BRCMFMAC_COUNTRY_BUF_SZ],
    pub rev: i32,
}

//
// struct brcmfmac_pd_cc - Struct for translating country codes as set by user
// space to a country code and rev which can be used by
// firmware.
//
// @table_size:	number of entries in table (> 0)
// @table:	array of 1 or more elements with translation information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmfmac_pd_cc {
    pub table_size: c_int,
    pub table: [brcmfmac_pd_cc_entry; ],
}

//
// struct brcmfmac_pd_device - Device specific platform data. (id/rev/bus_type)
// is the unique identifier of the device.
//
// @id:			ID of the device for which this data is. In case of SDIO
// or PCIE this is the chipid as identified by chip.c In
// case of USB this is the chipid as identified by the
// device query.
// @rev:		chip revision, see id.
// @bus_type:		The type of bus. Some chipid/rev exist for different bus
// types. Each bus type has its own set of settings.
// @feature_disable:	Bitmask of features to disable (override), See feature.c
// in brcmfmac for details.
// @country_codes:	If available, pointer to struct for translating country
// codes.
// @bus:		Bus specific (union) device settings. Currently only
// SDIO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmfmac_pd_device {
    pub id: c_uint,
    pub rev: c_uint,
    pub bus_type: brcmf_bus_type,
    pub feature_disable: c_uint,
    pub country_codes: *mut brcmfmac_pd_cc,
    pub sdio: brcmfmac_sdio_pd,
    pub bus: },
}

//
// struct brcmfmac_platform_data - BRCMFMAC specific platform data.
//
// @power_on:	This function is called by the brcmfmac driver when the module
// gets loaded. This can be particularly useful for low power
// devices. The platform spcific routine may for example decide to
// power up the complete device. If there is no use-case for this
// function then provide NULL.
// @power_off:	This function is called by the brcmfmac when the module gets
// unloaded. At this point the devices can be powered down or
// otherwise be reset. So if an actual power_off is not supported
// but reset is supported by the devices then reset the devices
// when this function gets called. This can be particularly useful
// for low power devices. If there is no use-case for this
// function then provide NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmfmac_platform_data {
    pub (*power_on)(void): *mut c_void,
    pub (*power_off)(void): *mut c_void,
    pub fw_alternative_path: *mut c_char,
    pub device_count: c_int,
    pub devices: [brcmfmac_pd_device; ],
}
