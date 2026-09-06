//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pmac_feature.h
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
// Definition of platform feature hooks for PowerMacs
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 1998 Paul Mackerras &
// Ben. Herrenschmidt.
//
// Note: I removed media-bay details from the feature stuff, I believe it's
// not worth it, the media-bay driver can directly use the mac-io
// ASIC registers.
//
// Implementation note: Currently, none of these functions will block.
// However, they may internally protect themselves with a spinlock
// for way too long. Be prepared for at least some of these to block
// in the future.
//
// Unless specifically defined, the result code is assumed to be an
// error when negative, 0 is the default success result. Some functions
// may return additional positive result values.
//
// To keep implementation simple, all feature calls are assumed to have
// the prototype parameters (struct device_node* node, int value).
// When either is not used, pass 0.
//

//
// Known Mac motherboard models
//
// Please, report any error here to benh@kernel.crashing.org, thanks !
//
// Note that I don't fully maintain this list for Core99 & MacRISC2
// and I'm considering removing all NewWorld entries from it and
// entirely rely on the model string.
//
// PowerSurge are the first generation of PCI Pmacs. This include
// all of the Grand-Central based machines. We currently don't
// differentiate most of them.
//
pub const PMAC_TYPE_PSURGE: c_uint = 0x10	/* PowerSurge */;
pub const PMAC_TYPE_ANS: c_uint = 0x11	/* Apple Network Server */;
// Here is the infamous serie of OHare based machines
//
pub const PMAC_TYPE_COMET: c_uint = 0x20	/* Believed to be PowerBook 2400 */;
pub const PMAC_TYPE_HOOPER: c_uint = 0x21	/* Believed to be PowerBook 3400 */;
pub const PMAC_TYPE_KANGA: c_uint = 0x22	/* PowerBook 3500 (first G3) */;
pub const PMAC_TYPE_ALCHEMY: c_uint = 0x23	/* Alchemy motherboard base */;
pub const PMAC_TYPE_GAZELLE: c_uint = 0x24	/* Spartacus, some 5xxx/6xxx */;
pub const PMAC_TYPE_UNKNOWN_OHARE: c_uint = 0x2f	/* Unknown, but OHare based */;
// Here are the Heathrow based machines
// FIXME: Differenciate wallstreet,mainstreet,wallstreetII
//
pub const PMAC_TYPE_GOSSAMER: c_uint = 0x30	/* Gossamer motherboard */;
pub const PMAC_TYPE_SILK: c_uint = 0x31	/* Desktop PowerMac G3 */;
pub const PMAC_TYPE_WALLSTREET: c_uint = 0x32	/* Wallstreet/Mainstreet PowerBook*/;
pub const PMAC_TYPE_UNKNOWN_HEATHROW: c_uint = 0x3f	/* Unknown but heathrow based */;
// Here are newworld machines based on Paddington (heathrow derivative)
//
pub const PMAC_TYPE_101_PBOOK: c_uint = 0x40	/* 101 PowerBook (aka Lombard) */;
pub const PMAC_TYPE_ORIG_IMAC: c_uint = 0x41	/* First generation iMac */;
pub const PMAC_TYPE_YOSEMITE: c_uint = 0x42	/* B&W G3 */;
pub const PMAC_TYPE_YIKES: c_uint = 0x43	/* Yikes G4 (PCI graphics) */;
pub const PMAC_TYPE_UNKNOWN_PADDINGTON: c_uint = 0x4f	/* Unknown but paddington based */;
// Core99 machines based on UniNorth 1.0 and 1.5
//
// Note: A single entry here may cover several actual models according
// to the device-tree. (Sawtooth is most tower G4s, FW_IMAC is most
// FireWire based iMacs, etc...). Those machines are too similar to be
// distinguished here, when they need to be differencied, use the
// device-tree "model" or "compatible" property.
//
pub const PMAC_TYPE_ORIG_IBOOK: c_uint = 0x40	/* First iBook model (no firewire) */;
pub const PMAC_TYPE_SAWTOOTH: c_uint = 0x41	/* Desktop G4s */;
pub const PMAC_TYPE_FW_IMAC: c_uint = 0x42	/* FireWire iMacs (except Pangea based) */;
pub const PMAC_TYPE_FW_IBOOK: c_uint = 0x43	/* FireWire iBooks (except iBook2) */;
pub const PMAC_TYPE_CUBE: c_uint = 0x44	/* Cube PowerMac */;
pub const PMAC_TYPE_QUICKSILVER: c_uint = 0x45	/* QuickSilver G4s */;
pub const PMAC_TYPE_PISMO: c_uint = 0x46	/* Pismo PowerBook */;
pub const PMAC_TYPE_TITANIUM: c_uint = 0x47	/* Titanium PowerBook */;
pub const PMAC_TYPE_TITANIUM2: c_uint = 0x48	/* Titanium II PowerBook (no L3, M6) */;
pub const PMAC_TYPE_TITANIUM3: c_uint = 0x49	/* Titanium III PowerBook (with L3 & M7) */;
pub const PMAC_TYPE_TITANIUM4: c_uint = 0x50	/* Titanium IV PowerBook (with L3 & M9) */;
pub const PMAC_TYPE_EMAC: c_uint = 0x50	/* eMac */;
pub const PMAC_TYPE_UNKNOWN_CORE99: c_uint = 0x5f;
// MacRisc2 with UniNorth 2.0
pub const PMAC_TYPE_RACKMAC: c_uint = 0x80	/* XServe */;
pub const PMAC_TYPE_WINDTUNNEL: c_uint = 0x81;
// MacRISC2 machines based on the Pangea chipset
//
pub const PMAC_TYPE_PANGEA_IMAC: c_uint = 0x100	/* Flower Power iMac */;
pub const PMAC_TYPE_IBOOK2: c_uint = 0x101	/* iBook2 (polycarbonate) */;
pub const PMAC_TYPE_FLAT_PANEL_IMAC: c_uint = 0x102	/* Flat panel iMac */;
pub const PMAC_TYPE_UNKNOWN_PANGEA: c_uint = 0x10f;
// MacRISC2 machines based on the Intrepid chipset
//
pub const PMAC_TYPE_UNKNOWN_INTREPID: c_uint = 0x11f	/* Generic */;
// MacRISC4 / G5 machines. We don't have per-machine selection here anymore,
// but rather machine families
//
pub const PMAC_TYPE_POWERMAC_G5: c_uint = 0x150	/* U3 & U3H based */;
pub const PMAC_TYPE_POWERMAC_G5_U3L: c_uint = 0x151	/* U3L based desktop */;
pub const PMAC_TYPE_IMAC_G5: c_uint = 0x152	/* iMac G5 */;
pub const PMAC_TYPE_XSERVE_G5: c_uint = 0x153	/* Xserve G5 */;
pub const PMAC_TYPE_UNKNOWN_K2: c_uint = 0x19f	/* Any other K2 based */;
pub const PMAC_TYPE_UNKNOWN_SHASTA: c_uint = 0x19e	/* Any other Shasta based */;
//
// Motherboard flags
//
pub const PMAC_MB_CAN_SLEEP: c_uint = 0x00000001;
pub const PMAC_MB_HAS_FW_POWER: c_uint = 0x00000002;
pub const PMAC_MB_OLD_CORE99: c_uint = 0x00000004;
pub const PMAC_MB_MOBILE: c_uint = 0x00000008;
pub const PMAC_MB_MAY_SLEEP: c_uint = 0x00000010;
//
// Feature calls supported on pmac
//
// Use this inline wrapper
//
// PMAC_FTR_SERIAL_ENABLE	(struct device_node* node, int param, int value)
// enable/disable an SCC side. Pass the node corresponding to the
// channel side as a parameter.
// param is the type of port
// if param is ored with PMAC_SCC_FLAG_XMON, then the SCC is locked enabled
// for use by xmon.
//

pub const PMAC_SCC_ASYNC: c_int = 0;
pub const PMAC_SCC_IRDA: c_int = 1;
pub const PMAC_SCC_I2S1: c_int = 2;
pub const PMAC_SCC_FLAG_XMON: c_uint = 0x00001000;
// PMAC_FTR_MODEM_ENABLE	(struct device_node* node, 0, int value)
// enable/disable the internal modem.
//

// PMAC_FTR_SWIM3_ENABLE	(struct device_node* node, 0,int value)
// enable/disable the swim3 (floppy) cell of a mac-io ASIC
//

// PMAC_FTR_MESH_ENABLE		(struct device_node* node, 0, int value)
// enable/disable the mesh (scsi) cell of a mac-io ASIC
//

// PMAC_FTR_IDE_ENABLE		(struct device_node* node, int busID, int value)
// enable/disable an IDE port of a mac-io ASIC
// pass the busID parameter
//

// PMAC_FTR_IDE_RESET		(struct device_node* node, int busID, int value)
// assert(1)/release(0) an IDE reset line (mac-io IDE only)
//

// PMAC_FTR_BMAC_ENABLE		(struct device_node* node, 0, int value)
// enable/disable the bmac (ethernet) cell of a mac-io ASIC, also drive
// its reset line
//

// PMAC_FTR_GMAC_ENABLE		(struct device_node* node, 0, int value)
// enable/disable the gmac (ethernet) cell of an uninorth ASIC. This
// control the cell's clock.
//

// PMAC_FTR_GMAC_PHY_RESET	(struct device_node* node, 0, 0)
// Perform a HW reset of the PHY connected to a gmac controller.
// Pass the gmac device node, not the PHY node.
//

// PMAC_FTR_SOUND_CHIP_ENABLE	(struct device_node* node, 0, int value)
// enable/disable the sound chip, whatever it is and provided it can
// actually be controlled
//

// -- add various tweaks related to sound routing --
// PMAC_FTR_AIRPORT_ENABLE	(struct device_node* node, 0, int value)
// enable/disable the airport card
//

// PMAC_FTR_RESET_CPU		(NULL, int cpu_nr, 0)
// toggle the reset line of a CPU on an uninorth-based SMP machine
//

// PMAC_FTR_USB_ENABLE		(struct device_node* node, 0, int value)
// enable/disable an USB cell, along with the power of the USB "pad"
// on keylargo based machines
//

// PMAC_FTR_1394_ENABLE		(struct device_node* node, 0, int value)
// enable/disable the firewire cell of an uninorth ASIC.
//

// PMAC_FTR_1394_CABLE_POWER	(struct device_node* node, 0, int value)
// enable/disable the firewire cable power supply of the uninorth
// firewire cell
//

// PMAC_FTR_SLEEP_STATE		(struct device_node* node, 0, int value)
// set the sleep state of the motherboard.
//
// Pass -1 as value to query for sleep capability
// Pass 1 to set IOs to sleep
// Pass 0 to set IOs to wake
//

// PMAC_FTR_GET_MB_INFO		(NULL, selector, 0)
//
// returns some motherboard infos.
// selector: 0  - model id
// 1  - model flags (capabilities)
// 2  - model name (cast to const char *)
//

pub const PMAC_MB_INFO_MODEL: c_int = 0;
pub const PMAC_MB_INFO_FLAGS: c_int = 1;
pub const PMAC_MB_INFO_NAME: c_int = 2;
// PMAC_FTR_READ_GPIO		(NULL, int index, 0)
//
// read a GPIO from a mac-io controller of type KeyLargo or Pangea.
// the value returned is a byte (positive), or a negative error code
//

// PMAC_FTR_WRITE_GPIO		(NULL, int index, int value)
//
// write a GPIO of a mac-io controller of type KeyLargo or Pangea.
//

// PMAC_FTR_ENABLE_MPIC
//
// Enable the MPIC cell
//

// PMAC_FTR_AACK_DELAY_ENABLE	(NULL, int enable, 0)
//
// Enable/disable the AACK delay on the northbridge for systems using DFS
//

// PMAC_FTR_DEVICE_CAN_WAKE
//
// Used by video drivers to inform system that they can actually perform
// wakeup from sleep
//

// Don't use those directly, they are for the sake of pmac_setup.c
extern "C" {
    pub fn pmac_do_feature_call(selector: c_uint, ...) -> c_long;
}
extern "C" {
    pub fn pmac_feature_init();
}
// Video suspend tweak
extern "C" {
    pub fn pmac_set_early_video_resume(data): *mut *mut void (proc)(void, data: *mut c_void);
}
extern "C" {
    pub fn pmac_call_early_video_resume();
}

// The AGP driver registers itself here
// Those are meant to be used by video drivers to deal with AGP
// suspend resume properly
//
extern "C" {
    pub fn pmac_suspend_agp_for_card(dev: *mut pci_dev);
}
extern "C" {
    pub fn pmac_resume_agp_for_card(dev: *mut pci_dev);
}
//
// The part below is for use by macio_asic.c only, do not rely
// on the data structures or constants below in a normal driver
//
pub const MAX_MACIO_CHIPS: c_int = 2;
// For use by macio_asic PCI driver
pub const MACIO_FLAG_SCCA_ON: c_uint = 0x00000001;
pub const MACIO_FLAG_SCCB_ON: c_uint = 0x00000002;
pub const MACIO_FLAG_SCC_LOCKED: c_uint = 0x00000004;
pub const MACIO_FLAG_AIRPORT_ON: c_uint = 0x00000010;
pub const MACIO_FLAG_FW_SUPPORTED: c_uint = 0x00000020;
extern "C" {
    pub fn macio_find(child: *mut *mut device_node, type: c_int) -> *mut macio_chip;
}

//
// Those are exported by pmac feature for internal use by arch code
// only like the platform function callbacks, do not use directly in drivers
//
// Uninorth reg. access. Note that Uni-N regs are big endian
//

// Uninorth variant:
//
// 0 = not uninorth
// 1 = U1.x or U2.x
// 3 = U3
// 4 = U4
//
extern "C" {
    pub fn pmac_get_uninorth_variant() -> c_int;
}
//
// Power macintoshes have either a CUDA, PMU or SMU controlling
// system reset, power, NVRAM, RTC.
//

