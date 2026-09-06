//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcma/bcma.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcma_hosttype {
    BCMA_HOSTTYPE_PCI,
    BCMA_HOSTTYPE_SDIO,
    BCMA_HOSTTYPE_SOC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_chipinfo {
    pub id: u16,
    pub rev: u8,
    pub pkg: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_boardinfo {
    pub vendor: u16,
    pub type: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcma_clkmode {
    BCMA_CLKMODE_FAST,
    BCMA_CLKMODE_DYNAMIC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_host_ops {
    pub offset): *mut *mut *mut u8 (read8)(struct bcma_device core, u16,
    pub offset): *mut *mut *mut u16 (read16)(struct bcma_device core, u16,
    pub offset): *mut *mut *mut u32 (read32)(struct bcma_device core, u16,
    pub value): *mut *mut *mut void (write8)(struct bcma_device core, u16 offset, u8,
    pub value): *mut *mut *mut void (write16)(struct bcma_device core, u16 offset, u16,
    pub value): *mut *mut *mut void (write32)(struct bcma_device core, u16 offset, u32,

    pub reg_width): size_t count, u16 offset, u8,
    pub reg_width): size_t count, u16 offset, u8,

// Agent ops
    pub offset): *mut *mut *mut u32 (aread32)(struct bcma_device core, u16,
    pub value): *mut *mut *mut void (awrite32)(struct bcma_device core, u16 offset, u32,
}

// Core manufacturers
pub const BCMA_MANUF_ARM: c_uint = 0x43B;
pub const BCMA_MANUF_MIPS: c_uint = 0x4A7;
pub const BCMA_MANUF_BCM: c_uint = 0x4BF;
// Core class values.
pub const BCMA_CL_SIM: c_uint = 0x0;
pub const BCMA_CL_EROM: c_uint = 0x1;
pub const BCMA_CL_CORESIGHT: c_uint = 0x9;
pub const BCMA_CL_VERIF: c_uint = 0xB;
pub const BCMA_CL_OPTIMO: c_uint = 0xD;
pub const BCMA_CL_GEN: c_uint = 0xE;
pub const BCMA_CL_PRIMECELL: c_uint = 0xF;
// Core-ID values.
pub const BCMA_CORE_OOB_ROUTER: c_uint = 0x367	/* Out of band */;
pub const BCMA_CORE_4706_CHIPCOMMON: c_uint = 0x500;
pub const BCMA_CORE_NS_PCIEG2: c_uint = 0x501;
pub const BCMA_CORE_NS_DMA: c_uint = 0x502;
pub const BCMA_CORE_NS_SDIO3: c_uint = 0x503;
pub const BCMA_CORE_NS_USB20: c_uint = 0x504;
pub const BCMA_CORE_NS_USB30: c_uint = 0x505;
pub const BCMA_CORE_NS_A9JTAG: c_uint = 0x506;
pub const BCMA_CORE_NS_DDR23: c_uint = 0x507;
pub const BCMA_CORE_NS_ROM: c_uint = 0x508;
pub const BCMA_CORE_NS_NAND: c_uint = 0x509;
pub const BCMA_CORE_NS_QSPI: c_uint = 0x50A;
pub const BCMA_CORE_NS_CHIPCOMMON_B: c_uint = 0x50B;
pub const BCMA_CORE_4706_SOC_RAM: c_uint = 0x50E;
pub const BCMA_CORE_ARMCA9: c_uint = 0x510;
pub const BCMA_CORE_4706_MAC_GBIT: c_uint = 0x52D;
pub const BCMA_CORE_AMEMC: c_uint = 0x52E	/* DDR1/2 memory controller core */;
pub const BCMA_CORE_ALTA: c_uint = 0x534	/* I2S core */;
pub const BCMA_CORE_4706_MAC_GBIT_COMMON: c_uint = 0x5DC;
pub const BCMA_CORE_DDR23_PHY: c_uint = 0x5DD;
pub const BCMA_CORE_INVALID: c_uint = 0x700;
pub const BCMA_CORE_CHIPCOMMON: c_uint = 0x800;
pub const BCMA_CORE_ILINE20: c_uint = 0x801;
pub const BCMA_CORE_SRAM: c_uint = 0x802;
pub const BCMA_CORE_SDRAM: c_uint = 0x803;
pub const BCMA_CORE_PCI: c_uint = 0x804;
pub const BCMA_CORE_MIPS: c_uint = 0x805;
pub const BCMA_CORE_ETHERNET: c_uint = 0x806;
pub const BCMA_CORE_V90: c_uint = 0x807;
pub const BCMA_CORE_USB11_HOSTDEV: c_uint = 0x808;
pub const BCMA_CORE_ADSL: c_uint = 0x809;
pub const BCMA_CORE_ILINE100: c_uint = 0x80A;
pub const BCMA_CORE_IPSEC: c_uint = 0x80B;
pub const BCMA_CORE_UTOPIA: c_uint = 0x80C;
pub const BCMA_CORE_PCMCIA: c_uint = 0x80D;
pub const BCMA_CORE_INTERNAL_MEM: c_uint = 0x80E;
pub const BCMA_CORE_MEMC_SDRAM: c_uint = 0x80F;
pub const BCMA_CORE_OFDM: c_uint = 0x810;
pub const BCMA_CORE_EXTIF: c_uint = 0x811;
pub const BCMA_CORE_80211: c_uint = 0x812;
pub const BCMA_CORE_PHY_A: c_uint = 0x813;
pub const BCMA_CORE_PHY_B: c_uint = 0x814;
pub const BCMA_CORE_PHY_G: c_uint = 0x815;
pub const BCMA_CORE_MIPS_3302: c_uint = 0x816;
pub const BCMA_CORE_USB11_HOST: c_uint = 0x817;
pub const BCMA_CORE_USB11_DEV: c_uint = 0x818;
pub const BCMA_CORE_USB20_HOST: c_uint = 0x819;
pub const BCMA_CORE_USB20_DEV: c_uint = 0x81A;
pub const BCMA_CORE_SDIO_HOST: c_uint = 0x81B;
pub const BCMA_CORE_ROBOSWITCH: c_uint = 0x81C;
pub const BCMA_CORE_PARA_ATA: c_uint = 0x81D;
pub const BCMA_CORE_SATA_XORDMA: c_uint = 0x81E;
pub const BCMA_CORE_ETHERNET_GBIT: c_uint = 0x81F;
pub const BCMA_CORE_PCIE: c_uint = 0x820;
pub const BCMA_CORE_PHY_N: c_uint = 0x821;
pub const BCMA_CORE_SRAM_CTL: c_uint = 0x822;
pub const BCMA_CORE_MINI_MACPHY: c_uint = 0x823;
pub const BCMA_CORE_ARM_1176: c_uint = 0x824;
pub const BCMA_CORE_ARM_7TDMI: c_uint = 0x825;
pub const BCMA_CORE_PHY_LP: c_uint = 0x826;
pub const BCMA_CORE_PMU: c_uint = 0x827;
pub const BCMA_CORE_PHY_SSN: c_uint = 0x828;
pub const BCMA_CORE_SDIO_DEV: c_uint = 0x829;
pub const BCMA_CORE_ARM_CM3: c_uint = 0x82A;
pub const BCMA_CORE_PHY_HT: c_uint = 0x82B;
pub const BCMA_CORE_MIPS_74K: c_uint = 0x82C;
pub const BCMA_CORE_MAC_GBIT: c_uint = 0x82D;
pub const BCMA_CORE_DDR12_MEM_CTL: c_uint = 0x82E;
pub const BCMA_CORE_PCIE_RC: c_uint = 0x82F	/* PCIe Root Complex */;
pub const BCMA_CORE_OCP_OCP_BRIDGE: c_uint = 0x830;
pub const BCMA_CORE_SHARED_COMMON: c_uint = 0x831;
pub const BCMA_CORE_OCP_AHB_BRIDGE: c_uint = 0x832;
pub const BCMA_CORE_SPI_HOST: c_uint = 0x833;
pub const BCMA_CORE_I2S: c_uint = 0x834;
pub const BCMA_CORE_SDR_DDR1_MEM_CTL: c_uint = 0x835	/* SDR/DDR1 memory controller core */;
pub const BCMA_CORE_SHIM: c_uint = 0x837	/* SHIM component in ubus/6362 */;
pub const BCMA_CORE_PHY_AC: c_uint = 0x83B;
pub const BCMA_CORE_PCIE2: c_uint = 0x83C	/* PCI Express Gen2 */;
pub const BCMA_CORE_USB30_DEV: c_uint = 0x83D;
pub const BCMA_CORE_ARM_CR4: c_uint = 0x83E;
pub const BCMA_CORE_GCI: c_uint = 0x840;
pub const BCMA_CORE_CMEM: c_uint = 0x846	/* CNDS DDR2/3 memory controller */;
pub const BCMA_CORE_ARM_CA7: c_uint = 0x847;
pub const BCMA_CORE_SYS_MEM: c_uint = 0x849;
pub const BCMA_CORE_DEFAULT: c_uint = 0xFFF;
pub const BCMA_MAX_NR_CORES: c_int = 16;
pub const BCMA_CORE_SIZE: c_uint = 0x1000;
// Chip IDs of PCIe devices
pub const BCMA_CHIP_ID_BCM4313: c_uint = 0x4313;
pub const BCMA_CHIP_ID_BCM43142: c_int = 43142;
pub const BCMA_CHIP_ID_BCM43131: c_int = 43131;
pub const BCMA_CHIP_ID_BCM43217: c_int = 43217;
pub const BCMA_CHIP_ID_BCM43222: c_int = 43222;
pub const BCMA_CHIP_ID_BCM43224: c_int = 43224;
pub const BCMA_PKG_ID_BCM43224_FAB_CSM: c_uint = 0x8;
pub const BCMA_PKG_ID_BCM43224_FAB_SMIC: c_uint = 0xa;
pub const BCMA_CHIP_ID_BCM43225: c_int = 43225;
pub const BCMA_CHIP_ID_BCM43227: c_int = 43227;
pub const BCMA_CHIP_ID_BCM43228: c_int = 43228;
pub const BCMA_CHIP_ID_BCM43421: c_int = 43421;
pub const BCMA_CHIP_ID_BCM43428: c_int = 43428;
pub const BCMA_CHIP_ID_BCM43431: c_int = 43431;
pub const BCMA_CHIP_ID_BCM43460: c_int = 43460;
pub const BCMA_CHIP_ID_BCM4331: c_uint = 0x4331;
pub const BCMA_CHIP_ID_BCM6362: c_uint = 0x6362;
pub const BCMA_CHIP_ID_BCM4360: c_uint = 0x4360;
pub const BCMA_CHIP_ID_BCM4352: c_uint = 0x4352;
// Chip IDs of SoCs
pub const BCMA_CHIP_ID_BCM4706: c_uint = 0x5300;
pub const BCMA_PKG_ID_BCM4706L: c_int = 1;
pub const BCMA_CHIP_ID_BCM4716: c_uint = 0x4716;
pub const BCMA_PKG_ID_BCM4716: c_int = 8;
pub const BCMA_PKG_ID_BCM4717: c_int = 9;
pub const BCMA_PKG_ID_BCM4718: c_int = 10;
pub const BCMA_CHIP_ID_BCM47162: c_int = 47162;
pub const BCMA_CHIP_ID_BCM4748: c_uint = 0x4748;
pub const BCMA_CHIP_ID_BCM4749: c_uint = 0x4749;
pub const BCMA_CHIP_ID_BCM5356: c_uint = 0x5356;
pub const BCMA_CHIP_ID_BCM5357: c_uint = 0x5357;
pub const BCMA_PKG_ID_BCM5358: c_int = 9;
pub const BCMA_PKG_ID_BCM47186: c_int = 10;
pub const BCMA_PKG_ID_BCM5357: c_int = 11;
pub const BCMA_CHIP_ID_BCM53572: c_int = 53572;
pub const BCMA_PKG_ID_BCM47188: c_int = 9;
pub const BCMA_CHIP_ID_BCM4707: c_int = 53010;
pub const BCMA_PKG_ID_BCM4707: c_int = 1;
pub const BCMA_PKG_ID_BCM4708: c_int = 2;
pub const BCMA_PKG_ID_BCM4709: c_int = 0;
pub const BCMA_CHIP_ID_BCM47094: c_int = 53030;
pub const BCMA_CHIP_ID_BCM53018: c_int = 53018;
pub const BCMA_CHIP_ID_BCM53573: c_int = 53573;
pub const BCMA_PKG_ID_BCM53573: c_int = 0;
pub const BCMA_PKG_ID_BCM47189: c_int = 1;
// Board types (on PCI usually equals to the subsystem dev id)
// BCM4313

// BCM4716

// BCM43224

// BCM43228

// BCM4331

// BCM53572

// BCM43142

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_device {
    pub bus: *mut bcma_bus,
    pub id: bcma_device_id,
    pub dev: device,
    pub dma_dev: *mut device,
    pub irq: c_uint,
    pub dev_registered: bool,
    pub core_index: u8,
    pub core_unit: u8,
    pub addr: u32,
    pub addr_s: [u32; 8],
    pub wrap: u32,
    pub io_addr: *mut void __iomem,
    pub io_wrap: *mut void __iomem,
    pub drvdata: *mut c_void,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_driver {
    pub name: *const c_char,
    pub id_table: *const bcma_device_id,
    pub dev): *mut *mut int (probe)(struct bcma_device,
    pub dev): *mut *mut void (remove)(struct bcma_device,
    pub dev): *mut *mut int (suspend)(struct bcma_device,
    pub dev): *mut *mut int (resume)(struct bcma_device,
    pub dev): *mut *mut void (shutdown)(struct bcma_device,
    pub drv: device_driver,
}

extern "C" {
    pub fn __bcma_driver_register(drv: *mut bcma_driver, owner: *mut module) -> c_int;
}

extern "C" {
    pub fn bcma_driver_unregister(drv: *mut bcma_driver);
}
// module_bcma_driver() - Helper macro for drivers that don't do
// anything special in module init/exit.  This eliminates a lot of
// boilerplate.  Each module may only use this macro once, and
// calling it replaces module_init() and module_exit()
//

// Set a fallback SPROM.
// See kdoc at the function definition for complete documentation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_bus {
    pub dev: *mut device,
// The MMIO area.
    pub mmio: *mut void __iomem,
    pub ops: *const bcma_host_ops,
    pub hosttype: bcma_hosttype,
    pub /: *mut *mut bool host_is_pcie2; / Used for BCMA_HOSTTYPE_PCI only,
    pub /: *mut *mut *mut pci_dev host_pci; / PCI bus pointer (BCMA_HOSTTYPE_PCI only),
    pub chipinfo: bcma_chipinfo,
    pub boardinfo: bcma_boardinfo,
    pub mapped_core: *mut bcma_device,
    pub cores: list_head,
    pub nr_cores: u8,
    pub num: u8,
    pub drv_cc: bcma_drv_cc,
    pub drv_cc_b: bcma_drv_cc_b,
    pub drv_pci: [bcma_drv_pci; 2],
    pub drv_pcie2: bcma_drv_pcie2,
    pub drv_mips: bcma_drv_mips,
    pub drv_gmac_cmn: bcma_drv_gmac_cmn,
// We decided to share SPROM struct with SSB as long as we do not need
// any hacks for BCMA. This simplifies drivers code.
    pub sprom: ssb_sprom,
}

extern "C" {
    pub fn bcma_find_core_unit(_arg: bus, _arg: coreid, _arg: 0) -> return;
}

extern "C" {
    pub fn bcma_host_pci_up(bus: *mut bcma_bus);
}
extern "C" {
    pub fn bcma_host_pci_down(bus: *mut bcma_bus);
}

extern "C" {
    pub fn bcma_core_is_enabled(core: *mut bcma_device) -> bool;
}
extern "C" {
    pub fn bcma_core_disable(core: *mut bcma_device, flags: u32);
}
extern "C" {
    pub fn bcma_core_enable(core: *mut bcma_device, flags: u32) -> c_int;
}
extern "C" {
    pub fn bcma_chipco_pll_read(cc: *mut bcma_drv_cc, offset: u32) -> u32;
}
pub const BCMA_DMA_TRANSLATION_MASK: c_uint = 0xC0000000;
pub const BCMA_DMA_TRANSLATION_NONE: c_uint = 0x00000000;
pub const BCMA_DMA_TRANSLATION_DMA32_CMT: c_uint = 0x40000000 /* Client Mode Translation for 32-bit DMA */;
pub const BCMA_DMA_TRANSLATION_DMA64_CMT: c_uint = 0x80000000 /* Client Mode Translation for 64-bit DMA */;
extern "C" {
    pub fn bcma_core_dma_translation(core: *mut bcma_device) -> u32;
}
extern "C" {
    pub fn bcma_core_irq(core: *mut bcma_device, num: c_int) -> c_uint;
}
