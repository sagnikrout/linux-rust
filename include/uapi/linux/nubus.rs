//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nubus.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_category {
    NUBUS_CAT_BOARD          = 0x0001,
    NUBUS_CAT_DISPLAY        = 0x0003,
    NUBUS_CAT_NETWORK        = 0x0004,
    NUBUS_CAT_COMMUNICATIONS = 0x0006,
    NUBUS_CAT_FONT           = 0x0009,
    NUBUS_CAT_CPU            = 0x000A,
// For lack of a better name
    NUBUS_CAT_DUODOCK        = 0x0020
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_type_network {
    NUBUS_TYPE_ETHERNET      = 0x0001,
    NUBUS_TYPE_RS232         = 0x0002
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_type_display {
    NUBUS_TYPE_VIDEO         = 0x0001
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_type_cpu {
    NUBUS_TYPE_68020         = 0x0003,
    NUBUS_TYPE_68030         = 0x0004,
    NUBUS_TYPE_68040         = 0x0005
}

// Known <Cat,Type,SW,HW> tuples: (according to TattleTech and Slots)
// 68030 motherboards: <10,4,0,24>
// 68040 motherboards: <10,5,0,24>
// DuoDock Plus: <32,1,1,2>
//
// Toby Frame Buffer card: <3,1,1,1>
// RBV built-in video (IIci): <3,1,1,24>
// Valkyrie built-in video (Q630): <3,1,1,46>
// Macintosh Display Card: <3,1,1,25>
// Sonora built-in video (P460): <3,1,1,34>
// Jet framebuffer (DuoDock Plus): <3,1,1,41>
//
// SONIC comm-slot/on-board and DuoDock Ethernet: <4,1,1,272>
// SONIC LC-PDS Ethernet (Dayna, but like Apple 16-bit, sort of): <4,1,1,271>
// Apple SONIC LC-PDS Ethernet ("Apple Ethernet LC Twisted-Pair Card"): <4,1,0,281>
// Sonic Systems Ethernet A-Series Card: <4,1,268,256>
// Asante MacCon NuBus-A: <4,1,260,256> (alpha-1.0,1.1 revision)
// ROM on the above card: <2,1,0,0>
// Cabletron ethernet card: <4,1,1,265>
// Farallon ethernet card: <4,1,268,256> (identical to Sonic Systems card)
// Kinetics EtherPort IIN: <4,1,259,262>
// API Engineering EtherRun_LCa PDS enet card: <4,1,282,256>
//
// Add your devices to the list!  You can obtain the "Slots" utility
// from Apple's FTP site at:
// ftp://dev.apple.com/devworld/Tool_Chest/Devices_-_Hardware/NuBus_Slot_Manager
//
// Alternately, TattleTech can be found at any Info-Mac mirror site.
// or from its distribution site: ftp://ftp.decismkr.com/dms
//
// DrSW: Uniquely identifies the software interface to a board.  This
// Add known DrSW values here
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_drsw {
// NUBUS_CAT_DISPLAY
    NUBUS_DRSW_APPLE        = 0x0001,
    NUBUS_DRSW_APPLE_HIRES  = 0x0013, /* MacII HiRes card driver */

// NUBUS_CAT_NETWORK
    NUBUS_DRSW_3COM         = 0x0000,
    NUBUS_DRSW_CABLETRON    = 0x0001,
    NUBUS_DRSW_SONIC_LC     = 0x0001,
    NUBUS_DRSW_KINETICS     = 0x0103,
    NUBUS_DRSW_ASANTE       = 0x0104,
    NUBUS_DRSW_TECHWORKS    = 0x0109,
    NUBUS_DRSW_DAYNA        = 0x010b,
    NUBUS_DRSW_FARALLON     = 0x010c,
    NUBUS_DRSW_APPLE_SN     = 0x010f,
    NUBUS_DRSW_DAYNA2       = 0x0115,
    NUBUS_DRSW_FOCUS        = 0x011a,
    NUBUS_DRSW_ASANTE_CS    = 0x011d, /* use asante SMC9194 driver */
    NUBUS_DRSW_DAYNA_LC     = 0x011e,

// NUBUS_CAT_CPU
    NUBUS_DRSW_NONE         = 0x0000,
}

// DrHW: Uniquely identifies the hardware interface to a board (or at
// Add known DrHW values here
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_drhw {
// NUBUS_CAT_DISPLAY
    NUBUS_DRHW_APPLE_TFB      = 0x0001, /* Toby frame buffer card */
    NUBUS_DRHW_APPLE_WVC      = 0x0006, /* Apple Workstation Video Card */
    NUBUS_DRHW_SIGMA_CLRMAX   = 0x0007, /* Sigma Design ColorMax */
    NUBUS_DRHW_APPLE_SE30     = 0x0009, /* Apple SE/30 video */
    NUBUS_DRHW_APPLE_HRVC     = 0x0013, /* Mac II High-Res Video Card */
    NUBUS_DRHW_APPLE_MVC      = 0x0014, /* Mac II Monochrome Video Card */
    NUBUS_DRHW_APPLE_PVC      = 0x0017, /* Mac II Portrait Video Card */
    NUBUS_DRHW_APPLE_RBV1     = 0x0018, /* IIci RBV video */
    NUBUS_DRHW_APPLE_MDC      = 0x0019, /* Macintosh Display Card */
    NUBUS_DRHW_APPLE_VSC      = 0x0020, /* Duo MiniDock ViSC framebuffer */
    NUBUS_DRHW_APPLE_SONORA   = 0x0022, /* Sonora built-in video */
    NUBUS_DRHW_APPLE_JET      = 0x0029, /* Jet framebuffer (DuoDock) */
    NUBUS_DRHW_APPLE_24AC     = 0x002b, /* Mac 24AC Video Card */
    NUBUS_DRHW_APPLE_VALKYRIE = 0x002e,
    NUBUS_DRHW_SMAC_GFX       = 0x0105, /* SuperMac GFX */
    NUBUS_DRHW_RASTER_CB264   = 0x013B, /* RasterOps ColorBoard 264 */
    NUBUS_DRHW_MICRON_XCEED   = 0x0146, /* Micron Exceed color */
    NUBUS_DRHW_RDIUS_GSC      = 0x0153, /* Radius GS/C */
    NUBUS_DRHW_SMAC_SPEC8     = 0x017B, /* SuperMac Spectrum/8 */
    NUBUS_DRHW_SMAC_SPEC24    = 0x017C, /* SuperMac Spectrum/24 */
    NUBUS_DRHW_RASTER_CB364   = 0x026F, /* RasterOps ColorBoard 364 */
    NUBUS_DRHW_RDIUS_DCGX     = 0x027C, /* Radius DirectColor/GX */
    NUBUS_DRHW_RDIUS_PC8      = 0x0291, /* Radius PrecisionColor 8 */
    NUBUS_DRHW_LAPIS_PCS8     = 0x0292, /* Lapis ProColorServer 8 */
    NUBUS_DRHW_RASTER_24XLI   = 0x02A0, /* RasterOps 8/24 XLi */
    NUBUS_DRHW_RASTER_PBPGT   = 0x02A5, /* RasterOps PaintBoard Prism GT */
    NUBUS_DRHW_EMACH_FSX      = 0x02AE, /* E-Machines Futura SX */
    NUBUS_DRHW_RASTER_24XLTV  = 0x02B7, /* RasterOps 24XLTV */
    NUBUS_DRHW_SMAC_THUND24   = 0x02CB, /* SuperMac Thunder/24 */
    NUBUS_DRHW_SMAC_THUNDLGHT = 0x03D9, /* SuperMac ThunderLight */
    NUBUS_DRHW_RDIUS_PC24XP   = 0x0406, /* Radius PrecisionColor 24Xp */
    NUBUS_DRHW_RDIUS_PC24X    = 0x040A, /* Radius PrecisionColor 24X */
    NUBUS_DRHW_RDIUS_PC8XJ    = 0x040B, /* Radius PrecisionColor 8XJ */

// NUBUS_CAT_NETWORK
    NUBUS_DRHW_INTERLAN       = 0x0100,
    NUBUS_DRHW_SMC9194        = 0x0101,
    NUBUS_DRHW_KINETICS       = 0x0106,
    NUBUS_DRHW_CABLETRON      = 0x0109,
    NUBUS_DRHW_ASANTE_LC      = 0x010f,
    NUBUS_DRHW_SONIC          = 0x0110,
    NUBUS_DRHW_TECHWORKS      = 0x0112,
    NUBUS_DRHW_APPLE_SONIC_NB = 0x0118,
    NUBUS_DRHW_APPLE_SONIC_LC = 0x0119,
    NUBUS_DRHW_FOCUS          = 0x011c,
    NUBUS_DRHW_SONNET         = 0x011d,
}

// Resource IDs: These are the identifiers for the various weird and
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_res_id {
    NUBUS_RESID_TYPE         = 0x0001,
    NUBUS_RESID_NAME         = 0x0002,
    NUBUS_RESID_ICON         = 0x0003,
    NUBUS_RESID_DRVRDIR      = 0x0004,
    NUBUS_RESID_LOADREC      = 0x0005,
    NUBUS_RESID_BOOTREC      = 0x0006,
    NUBUS_RESID_FLAGS        = 0x0007,
    NUBUS_RESID_HWDEVID      = 0x0008,
    NUBUS_RESID_MINOR_BASEOS = 0x000a,
    NUBUS_RESID_MINOR_LENGTH = 0x000b,
    NUBUS_RESID_MAJOR_BASEOS = 0x000c,
    NUBUS_RESID_MAJOR_LENGTH = 0x000d,
    NUBUS_RESID_CICN         = 0x000f,
    NUBUS_RESID_ICL8         = 0x0010,
    NUBUS_RESID_ICL4         = 0x0011,
}

// Category-specific resources.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_board_res_id {
    NUBUS_RESID_BOARDID      = 0x0020,
    NUBUS_RESID_PRAMINITDATA = 0x0021,
    NUBUS_RESID_PRIMARYINIT  = 0x0022,
    NUBUS_RESID_TIMEOUTCONST = 0x0023,
    NUBUS_RESID_VENDORINFO   = 0x0024,
    NUBUS_RESID_BOARDFLAGS   = 0x0025,
    NUBUS_RESID_SECONDINIT   = 0x0026,

// Not sure why Apple put these next two in here
    NUBUS_RESID_VIDNAMES     = 0x0041,
    NUBUS_RESID_VIDMODES     = 0x007e
}

// Fields within the vendor info directory
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_vendor_res_id {
    NUBUS_RESID_VEND_ID     = 0x0001,
    NUBUS_RESID_VEND_SERIAL = 0x0002,
    NUBUS_RESID_VEND_REV    = 0x0003,
    NUBUS_RESID_VEND_PART   = 0x0004,
    NUBUS_RESID_VEND_DATE   = 0x0005
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_net_res_id {
    NUBUS_RESID_MAC_ADDRESS  = 0x0080
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_cpu_res_id {
    NUBUS_RESID_MEMINFO      = 0x0081,
    NUBUS_RESID_ROMINFO      = 0x0082
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nubus_display_res_id {
    NUBUS_RESID_GAMMADIR    = 0x0040,
    NUBUS_RESID_FIRSTMODE   = 0x0080,
    NUBUS_RESID_SECONDMODE  = 0x0081,
    NUBUS_RESID_THIRDMODE   = 0x0082,
    NUBUS_RESID_FOURTHMODE  = 0x0083,
    NUBUS_RESID_FIFTHMODE   = 0x0084,
    NUBUS_RESID_SIXTHMODE   = 0x0085
}
