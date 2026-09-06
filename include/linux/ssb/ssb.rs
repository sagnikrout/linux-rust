//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ssb/ssb.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_sprom_core_pwr_info {
    pub itssi_5g: u8 itssi_2g,,
    pub maxpwr_5gh: u8 maxpwr_2g, maxpwr_5gl, maxpwr_5g,,
    pub pa_5gh: [u16 pa_2g[4], pa_5gl[4], pa_5g[4],; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_sprom {
    pub revision: u8,
    pub /: *mut *mut u8 il0mac[6] __aligned(sizeof(u16)); / MAC address for 802.11b/g,
    pub /: *mut *mut u8 et0mac[6] __aligned(sizeof(u16)); / MAC address for Ethernet,
    pub /: *mut *mut u8 et1mac[6] __aligned(sizeof(u16)); / MAC address for 802.11a,
    pub /: *mut *mut u8 et2mac[6] __aligned(sizeof(u16)); / MAC address for extra Ethernet,
    pub /: *mut *mut u8 et0phyaddr; / MII address for enet0,
    pub /: *mut *mut u8 et1phyaddr; / MII address for enet1,
    pub /: *mut *mut u8 et2phyaddr; / MII address for enet2,
    pub /: *mut *mut u8 et0mdcport; / MDIO for enet0,
    pub /: *mut *mut u8 et1mdcport; / MDIO for enet1,
    pub /: *mut *mut u8 et2mdcport; / MDIO for enet2,
    pub /: *mut *mut u16 dev_id; / Device ID overriding e.g. PCI ID,
    pub /: *mut *mut u16 board_rev; / Board revision number from SPROM.,
    pub /: *mut *mut u16 board_num; / Board number from SPROM.,
    pub /: *mut *mut u16 board_type; / Board type from SPROM.,
    pub /: *mut *mut u8 country_code; / Country Code,
    pub /: *mut *mut char alpha2[2]; / Country Code as two chars like EU or US,
    pub /: *mut *mut u8 leddc_on_time; / LED Powersave Duty Cycle On Count,
    pub /: *mut *mut u8 leddc_off_time; / LED Powersave Duty Cycle Off Count,
    pub /: *mut *mut u8 ant_available_a; / 2GHz antenna available bits (up to 4),
    pub /: *mut *mut u8 ant_available_bg; / 5GHz antenna available bits (up to 4),
    pub pa0b0: u16,
    pub pa0b1: u16,
    pub pa0b2: u16,
    pub pa1b0: u16,
    pub pa1b1: u16,
    pub pa1b2: u16,
    pub pa1lob0: u16,
    pub pa1lob1: u16,
    pub pa1lob2: u16,
    pub pa1hib0: u16,
    pub pa1hib1: u16,
    pub pa1hib2: u16,
    pub /: *mut *mut u8 gpio0; / GPIO pin 0,
    pub /: *mut *mut u8 gpio1; / GPIO pin 1,
    pub /: *mut *mut u8 gpio2; / GPIO pin 2,
    pub /: *mut *mut u8 gpio3; / GPIO pin 3,
    pub /: *mut *mut u8 maxpwr_bg; / 2.4GHz Amplifier Max Power (in dBm Q5.2),
    pub /: *mut *mut u8 maxpwr_al; / 5.2GHz Amplifier Max Power (in dBm Q5.2),
    pub /: *mut *mut u8 maxpwr_a; / 5.3GHz Amplifier Max Power (in dBm Q5.2),
    pub /: *mut *mut u8 maxpwr_ah; / 5.8GHz Amplifier Max Power (in dBm Q5.2),
    pub /: *mut *mut u8 itssi_a; / Idle TSSI Target for A-PHY,
    pub /: *mut *mut u8 itssi_bg; / Idle TSSI Target for B/G-PHY,
    pub /: *mut *mut u8 tri2g; / 2.4GHz TX isolation,
    pub /: *mut *mut u8 tri5gl; / 5.2GHz TX isolation,
    pub /: *mut *mut u8 tri5g; / 5.3GHz TX isolation,
    pub /: *mut *mut u8 tri5gh; / 5.8GHz TX isolation,
    pub /: *mut *mut u8 txpid2g[4]; / 2GHz TX power index,
    pub /: *mut *mut u8 txpid5gl[4]; / 4.9 - 5.1GHz TX power index,
    pub /: *mut *mut u8 txpid5g[4]; / 5.1 - 5.5GHz TX power index,
    pub /: *mut *mut u8 txpid5gh[4]; / 5.5 - ...GHz TX power index,
    pub /: *mut *mut s8 rxpo2g; / 2GHz RX power offset,
    pub /: *mut *mut s8 rxpo5g; / 5GHz RX power offset,
    pub /: *mut *mut u8 rssisav2g; / 2GHz RSSI params,
    pub rssismc2g: u8,
    pub rssismf2g: u8,
    pub /: *mut *mut u8 bxa2g; / 2GHz BX arch,
    pub /: *mut *mut u8 rssisav5g; / 5GHz RSSI params,
    pub rssismc5g: u8,
    pub rssismf5g: u8,
    pub /: *mut *mut u8 bxa5g; / 5GHz BX arch,
    pub /: *mut *mut u16 cck2gpo; / CCK power offset,
    pub /: *mut *mut u32 ofdm2gpo; / 2.4GHz OFDM power offset,
    pub /: *mut *mut u32 ofdm5glpo; / 5.2GHz OFDM power offset,
    pub /: *mut *mut u32 ofdm5gpo; / 5.3GHz OFDM power offset,
    pub /: *mut *mut u32 ofdm5ghpo; / 5.8GHz OFDM power offset,
    pub boardflags: u32,
    pub boardflags2: u32,
    pub boardflags3: u32,
// TODO: Switch all drivers to new u32 fields and drop below ones
    pub /: *mut *mut u16 boardflags_lo; / Board flags (bits 0-15),
    pub /: *mut *mut u16 boardflags_hi; / Board flags (bits 16-31),
    pub /: *mut *mut u16 boardflags2_lo; / Board flags (bits 32-47),
    pub /: *mut *mut u16 boardflags2_hi; / Board flags (bits 48-63),
    pub core_pwr_info: [ssb_sprom_core_pwr_info; 4],
// Antenna gain values for up to 4 antennas
// on each band. Values in dBm/4 (Q5.2). Negative gain means the
// loss in the connectors is bigger than the gain.
    pub a3: s8 a0, a1, a2,,
    pub antenna_gain: },
    pub antswlut: u8 tssipos, extpa_gain, pdet_range, tr_iso,,
    pub ghz2: },
    pub antswlut: u8 tssipos, extpa_gain, pdet_range, tr_iso,,
    pub ghz5: },
    pub fem: },
    pub mcs2gpo: [u16; 8],
    pub mcs5gpo: [u16; 8],
    pub mcs5glpo: [u16; 8],
    pub mcs5ghpo: [u16; 8],
    pub opo: u8,
    pub rxgainerr2ga: [u8; 3],
    pub rxgainerr5gla: [u8; 3],
    pub rxgainerr5gma: [u8; 3],
    pub rxgainerr5gha: [u8; 3],
    pub rxgainerr5gua: [u8; 3],
    pub noiselvl2ga: [u8; 3],
    pub noiselvl5gla: [u8; 3],
    pub noiselvl5gma: [u8; 3],
    pub noiselvl5gha: [u8; 3],
    pub noiselvl5gua: [u8; 3],
    pub regrev: u8,
    pub txchain: u8,
    pub rxchain: u8,
    pub antswitch: u8,
    pub cddpo: u16,
    pub stbcpo: u16,
    pub bw40po: u16,
    pub bwduppo: u16,
    pub tempthresh: u8,
    pub tempoffset: u8,
    pub rawtempsense: u16,
    pub measpower: u8,
    pub tempsense_slope: u8,
    pub tempcorrx: u8,
    pub tempsense_option: u8,
    pub freqoffset_corr: u8,
    pub iqcal_swp_dis: u8,
    pub hw_iqcal_en: u8,
    pub elna2g: u8,
    pub elna5g: u8,
    pub phycal_tempdelta: u8,
    pub temps_period: u8,
    pub temps_hysteresis: u8,
    pub measpower1: u8,
    pub measpower2: u8,
    pub pcieingress_war: u8,
// power per rate from sromrev 9
    pub cckbw202gpo: u16,
    pub cckbw20ul2gpo: u16,
    pub legofdmbw202gpo: u32,
    pub legofdmbw20ul2gpo: u32,
    pub legofdmbw205glpo: u32,
    pub legofdmbw20ul5glpo: u32,
    pub legofdmbw205gmpo: u32,
    pub legofdmbw20ul5gmpo: u32,
    pub legofdmbw205ghpo: u32,
    pub legofdmbw20ul5ghpo: u32,
    pub mcsbw202gpo: u32,
    pub mcsbw20ul2gpo: u32,
    pub mcsbw402gpo: u32,
    pub mcsbw205glpo: u32,
    pub mcsbw20ul5glpo: u32,
    pub mcsbw405glpo: u32,
    pub mcsbw205gmpo: u32,
    pub mcsbw20ul5gmpo: u32,
    pub mcsbw405gmpo: u32,
    pub mcsbw205ghpo: u32,
    pub mcsbw20ul5ghpo: u32,
    pub mcsbw405ghpo: u32,
    pub mcs32po: u16,
    pub legofdm40duppo: u16,
    pub sar2g: u8,
    pub sar5g: u8,
}

// Information about the PCB the circuitry is soldered on.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_boardinfo {
    pub vendor: u16,
    pub type: u16,
}

// Lowlevel read/write operations on the device MMIO.
// Internal, don't use that outside of ssb.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_bus_ops {
    pub offset): *mut *mut *mut u8 (read8)(struct ssb_device dev, u16,
    pub offset): *mut *mut *mut u16 (read16)(struct ssb_device dev, u16,
    pub offset): *mut *mut *mut u32 (read32)(struct ssb_device dev, u16,
    pub value): *mut *mut *mut void (write8)(struct ssb_device dev, u16 offset, u8,
    pub value): *mut *mut *mut void (write16)(struct ssb_device dev, u16 offset, u16,
    pub value): *mut *mut *mut void (write32)(struct ssb_device dev, u16 offset, u32,

    pub reg_width): size_t count, u16 offset, u8,
    pub reg_width): size_t count, u16 offset, u8,

}

// Core-ID values.
pub const SSB_DEV_CHIPCOMMON: c_uint = 0x800;
pub const SSB_DEV_ILINE20: c_uint = 0x801;
pub const SSB_DEV_SDRAM: c_uint = 0x803;
pub const SSB_DEV_PCI: c_uint = 0x804;
pub const SSB_DEV_MIPS: c_uint = 0x805;
pub const SSB_DEV_ETHERNET: c_uint = 0x806;
pub const SSB_DEV_V90: c_uint = 0x807;
pub const SSB_DEV_USB11_HOSTDEV: c_uint = 0x808;
pub const SSB_DEV_ADSL: c_uint = 0x809;
pub const SSB_DEV_ILINE100: c_uint = 0x80A;
pub const SSB_DEV_IPSEC: c_uint = 0x80B;
pub const SSB_DEV_PCMCIA: c_uint = 0x80D;
pub const SSB_DEV_INTERNAL_MEM: c_uint = 0x80E;
pub const SSB_DEV_MEMC_SDRAM: c_uint = 0x80F;
pub const SSB_DEV_EXTIF: c_uint = 0x811;
pub const SSB_DEV_80211: c_uint = 0x812;
pub const SSB_DEV_MIPS_3302: c_uint = 0x816;
pub const SSB_DEV_USB11_HOST: c_uint = 0x817;
pub const SSB_DEV_USB11_DEV: c_uint = 0x818;
pub const SSB_DEV_USB20_HOST: c_uint = 0x819;
pub const SSB_DEV_USB20_DEV: c_uint = 0x81A;
pub const SSB_DEV_SDIO_HOST: c_uint = 0x81B;
pub const SSB_DEV_ROBOSWITCH: c_uint = 0x81C;
pub const SSB_DEV_PARA_ATA: c_uint = 0x81D;
pub const SSB_DEV_SATA_XORDMA: c_uint = 0x81E;
pub const SSB_DEV_ETHERNET_GBIT: c_uint = 0x81F;
pub const SSB_DEV_PCIE: c_uint = 0x820;
pub const SSB_DEV_MIMO_PHY: c_uint = 0x821;
pub const SSB_DEV_SRAM_CTRLR: c_uint = 0x822;
pub const SSB_DEV_MINI_MACPHY: c_uint = 0x823;
pub const SSB_DEV_ARM_1176: c_uint = 0x824;
pub const SSB_DEV_ARM_7TDMI: c_uint = 0x825;
pub const SSB_DEV_ARM_CM3: c_uint = 0x82A;
// Vendor-ID values
pub const SSB_VENDOR_BROADCOM: c_uint = 0x4243;
// Some kernel subsystems poke with dev->drvdata, so we must use the
// following ugly workaround to get from struct device to struct ssb_device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ssb_dev_wrapper {
    pub dev: device,
    pub sdev: *mut ssb_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_device {
// Having a copy of the ops pointer in each dev struct
// is an optimization.
    pub ops: *const ssb_bus_ops,
    pub dma_dev: *mut *mut device dev,,
    pub bus: *mut ssb_bus,
    pub id: ssb_device_id,
    pub core_index: u8,
    pub irq: c_uint,
// Internal-only stuff follows.
    pub /: *mut *mut *mut void drvdata; / Per-device data,
    pub /: *mut *mut *mut void devtypedata; / Per-devicetype (eg 802.11) data,
}

// Go from struct device to struct ssb_device.
// Device specific user data
// Devicetype specific user data. This is per device-type (not per device)
extern "C" {
    pub fn ssb_set_devtypedata(dev: *mut ssb_device, data: *mut c_void);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_driver {
    pub name: *const c_char,
    pub id_table: *const ssb_device_id,
    pub id): *const *const *const int (probe)(struct ssb_device dev, struct ssb_device_id,
    pub dev): *mut *mut void (remove)(struct ssb_device,
    pub state): *mut *mut *mut int (suspend)(struct ssb_device dev, pm_message_t,
    pub dev): *mut *mut int (resume)(struct ssb_device,
    pub dev): *mut *mut void (shutdown)(struct ssb_device,
    pub drv: device_driver,
}

extern "C" {
    pub fn __ssb_driver_register(drv: *mut ssb_driver, owner: *mut module) -> c_int;
}

extern "C" {
    pub fn ssb_driver_unregister(drv: *mut ssb_driver);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssb_bustype {
    SSB_BUSTYPE_SSB,	/* This SSB bus is the system bus */
    SSB_BUSTYPE_PCI,	/* SSB is connected to PCI bus */
    SSB_BUSTYPE_PCMCIA,	/* SSB is connected to PCMCIA bus */
    SSB_BUSTYPE_SDIO,	/* SSB is connected to SDIO bus */
}

// board_vendor
pub const SSB_BOARDVENDOR_BCM: c_uint = 0x14E4	/* Broadcom */;
pub const SSB_BOARDVENDOR_DELL: c_uint = 0x1028	/* Dell */;
pub const SSB_BOARDVENDOR_HP: c_uint = 0x0E11	/* HP */;
// board_type
pub const SSB_BOARD_BCM94301CB: c_uint = 0x0406;
pub const SSB_BOARD_BCM94301MP: c_uint = 0x0407;
pub const SSB_BOARD_BU4309: c_uint = 0x040A;
pub const SSB_BOARD_BCM94309CB: c_uint = 0x040B;
pub const SSB_BOARD_BCM4309MP: c_uint = 0x040C;
pub const SSB_BOARD_BU4306: c_uint = 0x0416;
pub const SSB_BOARD_BCM94306MP: c_uint = 0x0418;
pub const SSB_BOARD_BCM4309G: c_uint = 0x0421;
pub const SSB_BOARD_BCM4306CB: c_uint = 0x0417;
pub const SSB_BOARD_BCM94306PC: c_uint = 0x0425	/* pcmcia 3.3v 4306 card */;
pub const SSB_BOARD_BCM94306CBSG: c_uint = 0x042B	/* with SiGe PA */;
pub const SSB_BOARD_PCSG94306: c_uint = 0x042D	/* with SiGe PA */;
pub const SSB_BOARD_BU4704SD: c_uint = 0x042E	/* with sdram */;
pub const SSB_BOARD_BCM94704AGR: c_uint = 0x042F	/* dual 11a/11g Router */;
pub const SSB_BOARD_BCM94308MP: c_uint = 0x0430	/* 11a-only minipci */;
pub const SSB_BOARD_BU4318: c_uint = 0x0447;
pub const SSB_BOARD_CB4318: c_uint = 0x0448;
pub const SSB_BOARD_MPG4318: c_uint = 0x0449;
pub const SSB_BOARD_MP4318: c_uint = 0x044A;
pub const SSB_BOARD_SD4318: c_uint = 0x044B;
pub const SSB_BOARD_BCM94306P: c_uint = 0x044C	/* with SiGe */;
pub const SSB_BOARD_BCM94303MP: c_uint = 0x044E;
pub const SSB_BOARD_BCM94306MPM: c_uint = 0x0450;
pub const SSB_BOARD_BCM94306MPL: c_uint = 0x0453;
pub const SSB_BOARD_PC4303: c_uint = 0x0454	/* pcmcia */;
pub const SSB_BOARD_BCM94306MPLNA: c_uint = 0x0457;
pub const SSB_BOARD_BCM94306MPH: c_uint = 0x045B;
pub const SSB_BOARD_BCM94306PCIV: c_uint = 0x045C;
pub const SSB_BOARD_BCM94318MPGH: c_uint = 0x0463;
pub const SSB_BOARD_BU4311: c_uint = 0x0464;
pub const SSB_BOARD_BCM94311MC: c_uint = 0x0465;
pub const SSB_BOARD_BCM94311MCAG: c_uint = 0x0466;
// 4321 boards
pub const SSB_BOARD_BU4321: c_uint = 0x046B;
pub const SSB_BOARD_BU4321E: c_uint = 0x047C;
pub const SSB_BOARD_MP4321: c_uint = 0x046C;
pub const SSB_BOARD_CB2_4321: c_uint = 0x046D;
pub const SSB_BOARD_CB2_4321_AG: c_uint = 0x0066;
pub const SSB_BOARD_MC4321: c_uint = 0x046E;
// 4325 boards
pub const SSB_BOARD_BCM94325DEVBU: c_uint = 0x0490;
pub const SSB_BOARD_BCM94325BGABU: c_uint = 0x0491;
pub const SSB_BOARD_BCM94325SDGWB: c_uint = 0x0492;
pub const SSB_BOARD_BCM94325SDGMDL: c_uint = 0x04AA;
pub const SSB_BOARD_BCM94325SDGMDL2: c_uint = 0x04C6;
pub const SSB_BOARD_BCM94325SDGMDL3: c_uint = 0x04C9;
pub const SSB_BOARD_BCM94325SDABGWBA: c_uint = 0x04E1;
// 4322 boards
pub const SSB_BOARD_BCM94322MC: c_uint = 0x04A4;
pub const SSB_BOARD_BCM94322USB: c_uint = 0x04A8	/* dualband */;
pub const SSB_BOARD_BCM94322HM: c_uint = 0x04B0;
pub const SSB_BOARD_BCM94322USB2D: c_uint = 0x04Bf	/* single band discrete front end */;
// 4312 boards
pub const SSB_BOARD_BU4312: c_uint = 0x048A;
pub const SSB_BOARD_BCM4312MCGSG: c_uint = 0x04B5;
// chip_package

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_bus {
// The MMIO area.
    pub mmio: *mut void __iomem,
    pub ops: *const ssb_bus_ops,
// The core currently mapped into the MMIO window.
// Not valid on all host-buses. So don't use outside of SSB.
    pub mapped_device: *mut ssb_device,
// Currently mapped PCMCIA segment. (bustype == SSB_BUSTYPE_PCMCIA only)
    pub mapped_pcmcia_seg: u8,
// Current SSB base address window for SDIO.
    pub sdio_sbaddr: u32,
}

// Lock for core and segment switching.
// On PCMCIA-host busses this is used to protect the whole MMIO access.
// The host-bus this backplane is running on.
// Pointers to the host-bus. Check bustype before using any of these pointers.
// Pointer to the PCI bus (only valid if bustype == SSB_BUSTYPE_PCI).
// Pointer to the PCMCIA device (only if bustype == SSB_BUSTYPE_PCMCIA).
// Pointer to the SDIO device (only if bustype == SSB_BUSTYPE_SDIO).
// See enum ssb_quirks

// Mutex to protect the SPROM writing.

// ID information about the Chip.
// List of devices (cores) on the backplane.
// Software ID number for this bus.
// The ChipCommon device (if available).
// The PCI-core device (if available).
// The MIPS-core device (if available).
// The EXTif-core device (if available).
// The following structure elements are not available in early
// SSB initialization. Though, they are available for regular
// registered drivers at any stage. So be careful when
// using them in the ssb core code.
// ID information about the PCB.
// Contents of the SPROM.
// If the board has a cardbus slot, this is set to true.

// Lock for GPIO register access.

// Internal-only stuff follows. Do not touch.
// Is the bus already powered up?
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssb_quirks {
// SDIO connected card requires performing a read after writing a 32-bit value
    SSB_QUIRK_SDIO_READ_AFTER_WRITE32	= (1 << 0),
}

// The initialization-invariants.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_init_invariants {
// Versioning information about the PCB.
    pub boardinfo: ssb_boardinfo,
// The SPROM information. That's either stored in an
// EEPROM or NVRAM on the board.
    pub sprom: ssb_sprom,
// If the board has a cardbus slot, this is set to true.
    pub has_cardbus_slot: bool,
}

// Type of function to fetch the invariants.
// Register SoC bus.

extern "C" {
    pub fn ssb_bus_unregister(bus: *mut ssb_bus);
}
// Does the device have an SPROM?
extern "C" {
    pub fn ssb_is_sprom_available(bus: *mut ssb_bus) -> bool;
}
// Set a fallback SPROM.
// See kdoc at the function definition for complete documentation.
// Suspend a SSB bus.
// Call this from the parent bus suspend routine.
extern "C" {
    pub fn ssb_bus_suspend(bus: *mut ssb_bus) -> c_int;
}
// Resume a SSB bus.
// Call this from the parent bus resume routine.
extern "C" {
    pub fn ssb_bus_resume(bus: *mut ssb_bus) -> c_int;
}
extern "C" {
    pub fn ssb_clockspeed(bus: *mut ssb_bus) -> u32;
}
// Is the device enabled in hardware?
extern "C" {
    pub fn ssb_device_is_enabled(dev: *mut ssb_device) -> c_int;
}
// Enable a device and pass device-specific SSB_TMSLOW flags.
// If no device-specific flags are available, use 0.
extern "C" {
    pub fn ssb_device_enable(dev: *mut ssb_device, core_specific_flags: u32);
}
// Disable a device in hardware and pass SSB_TMSLOW flags (if any).
extern "C" {
    pub fn ssb_device_disable(dev: *mut ssb_device, core_specific_flags: u32);
}
// Device MMIO register read/write functions.

// The SSB DMA API. Use this API for any DMA operation on the device.
// This API basically is a wrapper that calls the correct DMA API for
// the host device type the SSB device is attached to.
// Translation (routing) bits that need to be ORed to DMA
// addresses before they are given to a device.
extern "C" {
    pub fn ssb_dma_translation(dev: *mut ssb_device) -> u32;
}
pub const SSB_DMA_TRANSLATION_MASK: c_uint = 0xC0000000;
pub const SSB_DMA_TRANSLATION_SHIFT: c_int = 30;

// PCI-host wrapper driver
extern "C" {
    pub fn ssb_pcihost_register(driver: *mut pci_driver) -> c_int;
}

// If a driver is shutdown or suspended, call this to signal
// that the bus may be completely powered down. SSB will decide,
// if it's really time to power down the bus, based on if there
// are other devices that want to run.
extern "C" {
    pub fn ssb_bus_may_powerdown(bus: *mut ssb_bus) -> c_int;
}
// Before initializing and enabling a device, call this to power-up the bus.
// If you want to allow use of dynamic-power-control, pass the flag.
// Otherwise static always-on powercontrol will be used.
extern "C" {
    pub fn ssb_bus_powerup(bus: *mut ssb_bus, dynamic_pctl: bool) -> c_int;
}
extern "C" {
    pub fn ssb_commit_settings(bus: *mut ssb_bus);
}
// Various helper functions
extern "C" {
    pub fn ssb_admatch_base(adm: u32) -> u32;
}
extern "C" {
    pub fn ssb_admatch_size(adm: u32) -> u32;
}
// PCI device mapping and fixup routines.
// Called from the architecture pcibios init code.
// These are only available on SSB_EMBEDDED configurations.

extern "C" {
    pub fn ssb_pcibios_plat_dev_init(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn ssb_pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> c_int;
}

