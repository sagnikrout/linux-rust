//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-acpi.h
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
// Copyright (C) 2013-15, Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_acpi_package_context {
    pub /: *mut *mut *mut char name; / package name,
    pub /: *mut *mut int length; / number of elements,
    pub format: *mut acpi_buffer,
    pub state: *mut acpi_buffer,
    pub data_valid: bool,
}

// codec name is used in DAIs is i2c-<HID>:00 with HID being 8 chars

// acpi match
// check all codecs

// acpi match
// check all codecs

//
// struct snd_soc_acpi_mach_params - interface for machine driver configuration
//
// @acpi_ipc_irq_index: used for BYT-CR detection
// @platform: string used for HDAudio codec support
// @codec_mask: used for HDAudio support
// @dmic_num: number of SoC- or chipset-attached PDM digital microphones
// @link_mask: SoundWire links enabled on the board
// @links: array of SoundWire link _ADR descriptors, null terminated
// @i2s_link_mask: I2S/TDM links enabled on the board
// @num_dai_drivers: number of elements in @dai_drivers
// @dai_drivers: pointer to dai_drivers, used e.g. in nocodec mode
// @subsystem_vendor: optional PCI SSID vendor value
// @subsystem_device: optional PCI SSID device value
// @subsystem_rev: optional PCI SSID revision value
// @subsystem_id_set: true if a value has been written to
// subsystem_vendor and subsystem_device.
// @bt_link_mask: BT offload link enabled on the board
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_acpi_mach_params {
    pub acpi_ipc_irq_index: u32,
    pub platform: *const c_char,
    pub codec_mask: u32,
    pub dmic_num: u32,
    pub link_mask: u32,
    pub links: *const snd_soc_acpi_link_adr,
    pub i2s_link_mask: u32,
    pub num_dai_drivers: u32,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub subsystem_vendor: c_ushort,
    pub subsystem_device: c_ushort,
    pub subsystem_rev: c_ushort,
    pub subsystem_id_set: bool,
    pub bt_link_mask: u32,
}

//
// struct snd_soc_acpi_endpoint - endpoint descriptor
// @num: endpoint number (mandatory, unique per device)
// @aggregated: 0 (independent) or 1 (logically grouped)
// @group_position: zero-based order (only when @aggregated is 1)
// @group_id: platform-unique group identifier (only when @aggregrated is 1)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_acpi_endpoint {
    pub num: u8,
    pub aggregated: u8,
    pub group_position: u8,
    pub group_id: u8,
}

//
// struct snd_soc_acpi_adr_device - descriptor for _ADR-enumerated device
// @adr: 64 bit ACPI _ADR value
// @num_endpoints: number of endpoints for this device
// @endpoints: array of endpoints
// @name_prefix: string used for codec controls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_acpi_adr_device {
    pub adr: u64,
    pub num_endpoints: u8,
    pub endpoints: *const snd_soc_acpi_endpoint,
    pub name_prefix: *const c_char,
}

//
// struct snd_soc_acpi_link_adr - ACPI-based list of _ADR enumerated devices
// @mask: one bit set indicates the link this list applies to
// @num_adr: ARRAY_SIZE of devices
// @adr_d: array of devices
//
// The number of devices per link can be more than 1, e.g. in SoundWire
// multi-drop configurations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_acpi_link_adr {
    pub mask: u32,
    pub num_adr: u32,
    pub adr_d: *const snd_soc_acpi_adr_device,
}

//
// when set the topology uses the -ssp<N> suffix, where N is determined based on
// BIOS or DMI information
//

//
// when more than one SSP is reported in the link mask, use the most significant.
// This choice was found to be valid on platforms with ES8336 codecs.
//

//
// when set the topology uses the -dmic<N>ch suffix, where N is determined based on
// BIOS or DMI information
//

//
// when set the speaker amplifier name suffix (i.e. "-max98360a") will be
// appended to topology file name
//

//
// when set the headphone codec name suffix (i.e. "-rt5682") will be appended to
// topology file name
//

//
// struct snd_soc_acpi_mach - ACPI-based machine descriptor. Most of the fields
// are related to the hardware, except for the firmware and topology file names.
// A platform supported by legacy and Sound Open Firmware (SOF) would expose
// all firmware/topology related fields.
//
// @id: ACPI ID (usually the codec's) used to find a matching machine driver.
// @uid: ACPI Unique ID, can be used to disambiguate matches.
// @comp_ids: list of compatible audio codecs using the same machine driver,
// firmware and topology
// @link_mask: describes required board layout, e.g. for SoundWire.
// @links: array of link _ADR descriptors, null terminated.
// @drv_name: machine driver name
// @fw_filename: firmware file name. Used when SOF is not enabled.
// @tplg_filename: topology file name. Used when SOF is not enabled.
// @board: board name
// @machine_quirk: pointer to quirk, usually based on DMI information when
// ACPI ID alone is not sufficient, wrong or misleading
// @quirk_data: data used to uniquely identify a machine, usually a list of
// audio codecs whose presence if checked with ACPI
// @machine_check: pointer to quirk function. The functionality is similar to
// the use of @machine_quirk, except that the return value is a boolean: the intent
// is to skip a machine if the additional hardware/firmware verification invalidates
// the initial selection in the snd_soc_acpi_mach table.
// @pdata: intended for platform data or machine specific-ops. This structure
// is not constant since this field may be updated at run-time
// @mach_params: machine driver configuration
// @sof_tplg_filename: Sound Open Firmware topology file name, if enabled
// @tplg_quirk_mask: quirks to select different topology files dynamically
// @get_function_tplg_files: This is an optional callback, if specified then instead of
// the single sof_tplg_filename the callback will return the list of function topology
// files to be loaded.
// Return value: The number of the files or negative ERRNO. 0 means that the single topology
// file should be used, no function topology split can be used on the machine.
// card: the pointer of the card
// mach: the pointer of the machine driver
// prefix: the prefix of the topology file name. Typically, it is the path.
// tplg_files: the pointer of the array of the topology file names.
// best_effort: ignore non supported links and try to build the card in best effort
// with supported links
//
// Descriptor for SST ASoC machine driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_acpi_mach {
    pub id: [u8; ACPI_ID_LEN],
    pub uid: *const c_char,
    pub comp_ids: *const snd_soc_acpi_codecs,
    pub link_mask: u32,
    pub links: *const snd_soc_acpi_link_adr,
    pub drv_name: *const c_char,
    pub fw_filename: *const c_char,
    pub tplg_filename: *const c_char,
    pub board: *const c_char,
    pub arg): *mut *mut *mut snd_soc_acpi_mach  (machine_quirk)(void,
    pub quirk_data: *const c_void,
    pub arg): *mut *mut bool (machine_check)(void,
    pub pdata: *mut c_void,
    pub mach_params: snd_soc_acpi_mach_params,
    pub sof_tplg_filename: *const c_char,
    pub tplg_quirk_mask: u32,
    pub best_effort): bool,
}

pub const SND_SOC_ACPI_MAX_CODECS: c_int = 3;
//
// struct snd_soc_acpi_codecs: Structure to hold secondary codec information
// apart from the matched one, this data will be passed to the quirk function
// to match with the ACPI detected devices
//
// @num_codecs: number of secondary codecs used in the platform
// @codecs: holds the codec IDs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: c_int,
    pub codecs: [u8; SND_SOC_ACPI_MAX_CODECS][ACPI_ID_LEN],
}
