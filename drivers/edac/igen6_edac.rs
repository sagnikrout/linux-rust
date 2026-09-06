//! Automatically rewritten from C to Rust
//! Source: drivers/edac/igen6_edac.c
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
// Driver for Intel client SoC with integrated memory controller using IBECC
//
// Copyright (C) 2020 Intel Corporation
//
// The In-Band ECC (IBECC) IP provides ECC protection to all or specific
// regions of the physical memory space. It's used for memory controllers
// that don't support the out-of-band ECC which often needs an additional
// storage device to each channel for storing ECC data.
//

// Debug macros

    edac_printk(level, "igen6", fmt, ##arg)

    edac_mc_chipset_printk(mci, level, "igen6", fmt, ##arg)

// Probing upper bound, not a hardware capability limit.
pub const MAX_IMC_TO_PROBE: c_int = 8;

// Size of physical memory
pub const TOM_OFFSET: c_uint = 0xa0;
// Top of low usable DRAM
pub const TOLUD_OFFSET: c_uint = 0xbc;
// Capability register C
pub const CAPID_C_OFFSET: c_uint = 0xec;

// Capability register E
pub const CAPID_E_OFFSET: c_uint = 0xf0;

// Error Status
pub const ERRSTS_OFFSET: c_uint = 0xc8;

// Error Command
pub const ERRCMD_OFFSET: c_uint = 0xca;

// IBECC MMIO base address

// IBECC error log

// Host MMIO base address
pub const MCHBAR_OFFSET: c_uint = 0x48;

pub const MCHBAR_SIZE: c_uint = 0x10000;
// Parameters for the channel decode stage

// Parameters for DRAM decode stage

// DIMM characteristics

// Hash for memory controller selection

// Hash for channel selection

// Hash for enhanced channel selection

// Parameters for memory slice decode stage

//
// A slice represents a portion of memory space participating in an
// interleave relationship within the memory hierarchy.
//
// It can represent in different levels such as:
//
// - a pair of memory controllers
// - a memory controller
// - a memory channel
// - a memory sub-channel / DIMM
//
// +--------+
// |        |
// | Zone 1 |
// |        |
// +--------+  +--------+
// |        |  |        |
// | Zone 0 |  | Zone 0 |
// |        |  |        |
// +--------+  +--------+
//
// Slice L     Slice S
//
// Memory space is divided into:
//
// - Zone 0 : Interleaved region
// - Zone 1 : Non-interleaved region (upper part of the large slice).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slice {
// Slice address.
    pub addr: u64,
// Slice that @addr belongs to.
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igen6_imc {
    pub mc: c_int,
    pub mci: *mut mem_ctl_info,
    pub pdev: *mut pci_dev,
    pub dev: device,
    pub window: *mut void __iomem,
    pub size: u64,
    pub ch_s_size: u64,
    pub ch_l_map: c_int,
    pub dimm_s_size: [u64; NUM_CHANNELS],
    pub dimm_l_size: [u64; NUM_CHANNELS],
    pub dimm_l_map: [c_int; NUM_CHANNELS],
}

    static struct res_config {
    bool machine_check;
// The number of present memory controllers.
    int num_imc;
// Host MMIO configuration
    u64 reg_mchbar_mask;
// Top of memory
    u64 reg_tom_mask;
// Top of upper usable DRAM
    u64 reg_touud_mask;
// IBECC error log
    u64 reg_eccerrlog_addr_mask;
// MEMSS_PMA_CR registers.
    u32 reg_mem_config_offset;
    u32 reg_mem_config_ddr_type_mask;
    u32 reg_mem_config_ibecc_en_mask;
    u32 reg_capabilities_misc_offset;
    u32 reg_capabilities_misc_ibecc_dis;
// Memory controller registers.
    u32 reg_mad_inter_size_mask[NUM_CHANNELS];
    u64 reg_mad_inter_size_granularity;
    u32 reg_mad_intra_rank_mask[NUM_DIMMS];
    u32 reg_mad_intra_width_mask[NUM_DIMMS];
    u32 reg_mad_intra_density_mask[NUM_DIMMS];
    u32 imc_base;
    u32 cmf_base;
    u32 cmf_size;
    u32 ms_hash_offset;
    u32 ibecc_base;
    u32 ibecc_error_log_offset;
// Get memory type.
    enum mem_type (*get_mem_type)(struct igen6_imc *imc);
// Get DRAM chip type.
    enum dev_type (*get_dev_type)(struct igen6_imc *imc, int chan, int dimm_l);
// Set imc->ch_{s_size,l_map}.
    void (*set_chan_params)(struct igen6_imc *imc);
// Set imc->dimm_{l_size,s_size,l_map}[chan].
    void (*set_dimm_params)(struct igen6_imc *imc, int chan);
    bool (*ibecc_available)(struct pci_dev *pdev);
// Convert error address logged in IBECC to system physical address
    u64 (*err_addr_to_sys_addr)(u64 eaddr, int mc);
// Convert error address logged in IBECC to integrated memory controller address
    u64 (*err_addr_to_imc_addr)(u64 eaddr, int mc);
    } *res_cfg;
    static struct igen6_pvt {
    void __iomem *memss_pma_cr;
    u64 ms_hash;
    u64 ms_s_size;
    int ms_l_map;
    struct igen6_imc imc[];
    } *igen6_pvt;
// The top of low usable DRAM
    static u32 igen6_tolud;
// The size of physical memory
    static u64 igen6_tom;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct decoded_addr {
    pub mc: c_int,
    pub imc_addr: u64,
    pub sys_addr: u64,
    pub channel_idx: c_int,
    pub channel_addr: u64,
    pub sub_channel_idx: c_int,
    pub sub_channel_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecclog_node {
    pub llnode: llist_node,
    pub mc: c_int,
    pub ecclog: u64,
}

//
// In the NMI handler, the driver uses the lock-less memory allocator
// to allocate memory to store the IBECC error logs and links the logs
// to the lock-less list. Delay printk() and the work of error reporting
// to EDAC core in a worker.
//

    static LLIST_HEAD(ecclog_llist);
    static struct gen_pool *ecclog_pool;
    static char ecclog_buf[ECCLOG_POOL_SIZE];
    static struct irq_work ecclog_irq_work;
    static struct work_struct ecclog_work;
// SoC compute die IDs with IBECC capability.
// Elkhart Lake
pub const DID_EHL_SKU5: c_uint = 0x4514;
pub const DID_EHL_SKU6: c_uint = 0x4528;
pub const DID_EHL_SKU7: c_uint = 0x452a;
pub const DID_EHL_SKU8: c_uint = 0x4516;
pub const DID_EHL_SKU9: c_uint = 0x452c;
pub const DID_EHL_SKU10: c_uint = 0x452e;
pub const DID_EHL_SKU11: c_uint = 0x4532;
pub const DID_EHL_SKU12: c_uint = 0x4518;
pub const DID_EHL_SKU13: c_uint = 0x451a;
pub const DID_EHL_SKU14: c_uint = 0x4534;
pub const DID_EHL_SKU15: c_uint = 0x4536;
// ICL-NNPI
pub const DID_ICL_SKU8: c_uint = 0x4581;
pub const DID_ICL_SKU10: c_uint = 0x4585;
pub const DID_ICL_SKU11: c_uint = 0x4589;
pub const DID_ICL_SKU12: c_uint = 0x458d;
// Tiger Lake
pub const DID_TGL_SKU: c_uint = 0x9a14;
// Alder Lake
pub const DID_ADL_SKU1: c_uint = 0x4601;
pub const DID_ADL_SKU2: c_uint = 0x4602;
pub const DID_ADL_SKU3: c_uint = 0x4621;
pub const DID_ADL_SKU4: c_uint = 0x4641;
// Alder Lake-N
pub const DID_ADL_N_SKU1: c_uint = 0x4614;
pub const DID_ADL_N_SKU2: c_uint = 0x4617;
pub const DID_ADL_N_SKU3: c_uint = 0x461b;
pub const DID_ADL_N_SKU4: c_uint = 0x461c;
pub const DID_ADL_N_SKU5: c_uint = 0x4673;
pub const DID_ADL_N_SKU6: c_uint = 0x4674;
pub const DID_ADL_N_SKU7: c_uint = 0x4675;
pub const DID_ADL_N_SKU8: c_uint = 0x4677;
pub const DID_ADL_N_SKU9: c_uint = 0x4678;
pub const DID_ADL_N_SKU10: c_uint = 0x4679;
pub const DID_ADL_N_SKU11: c_uint = 0x467c;
pub const DID_ADL_N_SKU12: c_uint = 0x4632;
// Arizona Beach
pub const DID_AZB_SKU1: c_uint = 0x4676;
// Amston Lake
pub const DID_ASL_SKU1: c_uint = 0x464a;
pub const DID_ASL_SKU2: c_uint = 0x4646;
pub const DID_ASL_SKU3: c_uint = 0x4652;
// Raptor Lake-P
pub const DID_RPL_P_SKU1: c_uint = 0xa706;
pub const DID_RPL_P_SKU2: c_uint = 0xa707;
pub const DID_RPL_P_SKU3: c_uint = 0xa708;
pub const DID_RPL_P_SKU4: c_uint = 0xa716;
pub const DID_RPL_P_SKU5: c_uint = 0xa718;
// Meteor Lake-PS
pub const DID_MTL_PS_SKU1: c_uint = 0x7d21;
pub const DID_MTL_PS_SKU2: c_uint = 0x7d22;
pub const DID_MTL_PS_SKU3: c_uint = 0x7d23;
pub const DID_MTL_PS_SKU4: c_uint = 0x7d24;
// Meteor Lake-P
pub const DID_MTL_P_SKU1: c_uint = 0x7d01;
pub const DID_MTL_P_SKU2: c_uint = 0x7d02;
pub const DID_MTL_P_SKU3: c_uint = 0x7d14;
// Arrow Lake-UH
pub const DID_ARL_UH_SKU1: c_uint = 0x7d06;
pub const DID_ARL_UH_SKU2: c_uint = 0x7d20;
pub const DID_ARL_UH_SKU3: c_uint = 0x7d30;
// Panther Lake-H
pub const DID_PTL_H_SKU1: c_uint = 0xb000;
pub const DID_PTL_H_SKU2: c_uint = 0xb001;
pub const DID_PTL_H_SKU3: c_uint = 0xb002;
pub const DID_PTL_H_SKU4: c_uint = 0xb003;
pub const DID_PTL_H_SKU5: c_uint = 0xb004;
pub const DID_PTL_H_SKU6: c_uint = 0xb005;
pub const DID_PTL_H_SKU7: c_uint = 0xb008;
pub const DID_PTL_H_SKU8: c_uint = 0xb011;
pub const DID_PTL_H_SKU9: c_uint = 0xb014;
pub const DID_PTL_H_SKU10: c_uint = 0xb015;
pub const DID_PTL_H_SKU11: c_uint = 0xb028;
pub const DID_PTL_H_SKU12: c_uint = 0xb029;
pub const DID_PTL_H_SKU13: c_uint = 0xb02a;
pub const DID_PTL_H_SKU14: c_uint = 0xb00a;
// Starfire
pub const DID_STF_SKU1: c_uint = 0xb02b;
// Wildcat Lake
pub const DID_WCL_SKU1: c_uint = 0xfd00;
// Nova Lake-H/HX
pub const DID_NVL_H_SKU1: c_uint = 0xd701;
pub const DID_NVL_H_SKU2: c_uint = 0xd702;
pub const DID_NVL_H_SKU3: c_uint = 0xd704;
pub const DID_NVL_H_SKU4: c_uint = 0xd705;
// Remove the interleave bit and shift upper part down to fill gap.
#[no_mangle]
unsafe extern "C" fn squeeze_addr(addr: u64, intlv_bit: c_int) -> u64 {
    static u64 squeeze_addr(u64 addr, int intlv_bit)
    {
    u64 slice_addr;
    slice_addr  = GET_BITFIELD(addr, intlv_bit + 1, 63) << intlv_bit;
    slice_addr |= GET_BITFIELD(addr, 0, intlv_bit - 1);
    return slice_addr;
    }
// Shift the upper bits up and insert a zero at the @intlv_bit bit position.
#[no_mangle]
unsafe extern "C" fn inflate_addr(addr: u64, intlv_bit: c_int) -> u64 {
    static u64 inflate_addr(u64 addr, int intlv_bit)
    {
    u64 inflated_addr;
// Insert a zero at @intlv_bit position.
    inflated_addr  = GET_BITFIELD(addr, intlv_bit, 63) << (intlv_bit + 1);
    inflated_addr |= GET_BITFIELD(addr, 0, intlv_bit - 1);
    return inflated_addr;
    }
#[no_mangle]
unsafe extern "C" fn compute_hash(addr: u64, hash_mask: u64, hash_base: u64, intlv_bit: c_int) -> u64 {
    static u64 compute_hash(u64 addr, u64 hash_mask, u64 hash_base, int intlv_bit)
    {
    u64 hash_addr;
    int i;
//
// In hash mode, @intlv_bit is the lowest selected bit of @addr
// to be XORed. While @mask may or may not include this @intlv_bit,
// we enforce that @mask includes @intlv_bit to ensure @intlv_bit is
// XORed exactly once.
//
    hash_mask |= BIT_ULL(intlv_bit);
    hash_addr  = addr & hash_mask;
    for (i = 6; i < 20; i++)
    hash_base ^= (hash_addr >> i) & 1;
    return hash_base;
    }
//
// Converts a higher-level address (system / IMC / channel) into a lower-level
// slice address and identifier.
//
    static void translate_to_lower_level(u64 addr, u64 hash_mask, u64 hash_base,
    int intlv_bit, u64 s_size, int l_map,
    struct slice *slice)
    {
// In non-interleave zone.
    if (addr >= 2 * s_size) {
    slice.addr = addr - s_size;
    slice.id  = l_map;
    return;
    }
// In interleave zone.
    slice.addr = squeeze_addr(addr, intlv_bit);
// Non-hash mode.
    if (!hash_mask) {
    slice.id = GET_BITFIELD(addr, intlv_bit, intlv_bit);
    return;
    }
// Hash mode.
    slice.id = compute_hash(addr, hash_mask, hash_base, intlv_bit);
    }
// Reconstruct address for upper memory hierarchy level.
    static u64 translate_to_upper_level(u64 addr, u64 hash_mask, u64 hash_base,
    int intlv_bit, u64 s_size)
    {
    u64 inflated_addr, hash_val;
// In non-interleave zone.
    if (addr >= s_size)
    return addr + s_size;
//
// In interleave zone.
//
// Insert a zero at @intlv_bit position.
//
    inflated_addr = inflate_addr(addr, intlv_bit);
//
// Reconstruct the removed interleave bit and use it to replace
// the zero at @intlv_bit position.
//
    hash_val = compute_hash(inflated_addr, hash_mask, hash_base, intlv_bit);
    return inflated_addr | (hash_val << intlv_bit);
    }
#[no_mangle]
unsafe extern "C" fn get_mchbar(pdev: *mut pci_dev, mchbar: *mut u64) -> c_int {
    static int get_mchbar(struct pci_dev *pdev, u64 *mchbar)
    {
    union  {
    u64 v;
    struct {
    u32 v_lo;
    u32 v_hi;
    };
    } u;
    if (pci_read_config_dword(pdev, MCHBAR_OFFSET, &u.v_lo)) {
    igen6_printk(KERN_ERR, "Failed to read lower MCHBAR\n");
    return -ENODEV;
    }
    if (pci_read_config_dword(pdev, MCHBAR_OFFSET + 4, &u.v_hi)) {
    igen6_printk(KERN_ERR, "Failed to read upper MCHBAR\n");
    return -ENODEV;
    }
    if (!(u.v & MCHBAR_EN)) {
    igen6_printk(KERN_ERR, "MCHBAR is disabled\n");
    return -ENODEV;
    }
// mchbar = u.v & res_cfg->reg_mchbar_mask;
    edac_dbg(2, "MCHBAR 0x%llx (reg 0x%llx)\n", *mchbar, u.v);
    return 0;
    }
// Check whether the memory controller is absent.
#[no_mangle]
unsafe extern "C" fn imc_absent(window: *mut void __iomem) -> bool {
    static bool imc_absent(void __iomem *window)
    {
    return readl(window + MAD_INTER_CHANNEL_OFFSET) == ~0;
    }
// Return MMIO base address of the memory controller if it's present, otherwise return NULL.
    static void __iomem *map_imc_window(u64 mchbar, int pmc)
    {
    void __iomem *window;
    window = ioremap(mchbar + pmc * MCHBAR_SIZE, MCHBAR_SIZE);
    if (!window)
    return core::ptr::null_mut();
    if (imc_absent(window)) {
    iounmap(window);
    return core::ptr::null_mut();
    }
    return window;
    }
// Return the number of present memory controllers.
#[no_mangle]
unsafe extern "C" fn get_imc_num(mchbar: u64) -> c_int {
    static int get_imc_num(u64 mchbar)
    {
    void __iomem *window;
    int lmc, pmc;
    for (lmc = 0, pmc = 0; pmc < MAX_IMC_TO_PROBE; pmc++) {
    window = map_imc_window(mchbar, pmc);
    if (window) {
    iounmap(window);
    lmc++;
    }
    }
    return lmc;
    }
#[no_mangle]
unsafe extern "C" fn ehl_ibecc_available(pdev: *mut pci_dev) -> bool {
    static bool ehl_ibecc_available(struct pci_dev *pdev)
    {
    u32 v;
    if (pci_read_config_dword(pdev, CAPID_C_OFFSET, &v))
    return false;
    return !!(CAPID_C_IBECC & v);
    }
#[no_mangle]
unsafe extern "C" fn ehl_err_addr_to_sys_addr(eaddr: u64, mc: c_int) -> u64 {
    static u64 ehl_err_addr_to_sys_addr(u64 eaddr, int mc)
    {
    return eaddr;
    }
#[no_mangle]
unsafe extern "C" fn ehl_err_addr_to_imc_addr(eaddr: u64, mc: c_int) -> u64 {
    static u64 ehl_err_addr_to_imc_addr(u64 eaddr, int mc)
    {
    if (eaddr < igen6_tolud)
    return eaddr;
    if (igen6_tom <= _4GB)
    return eaddr + igen6_tolud - _4GB;
    if (eaddr >= igen6_tom)
    return eaddr + igen6_tolud - igen6_tom;
    return eaddr;
    }
#[no_mangle]
unsafe extern "C" fn icl_ibecc_available(pdev: *mut pci_dev) -> bool {
    static bool icl_ibecc_available(struct pci_dev *pdev)
    {
    u32 v;
    if (pci_read_config_dword(pdev, CAPID_C_OFFSET, &v))
    return false;
    return !(CAPID_C_IBECC & v) &&
    (boot_cpu_data.x86_stepping >= 1);
    }
#[no_mangle]
unsafe extern "C" fn tgl_ibecc_available(pdev: *mut pci_dev) -> bool {
    static bool tgl_ibecc_available(struct pci_dev *pdev)
    {
    u32 v;
    if (pci_read_config_dword(pdev, CAPID_E_OFFSET, &v))
    return false;
    return !(CAPID_E_IBECC & v);
    }
#[no_mangle]
unsafe extern "C" fn mtl_p_ibecc_available(pdev: *mut pci_dev) -> bool {
    static bool mtl_p_ibecc_available(struct pci_dev *pdev)
    {
    u32 v;
    if (pci_read_config_dword(pdev, CAPID_E_OFFSET, &v))
    return false;
    return !(CAPID_E_IBECC_BIT18 & v);
    }
#[no_mangle]
unsafe extern "C" fn generic_ibecc_available(pdev: *mut pci_dev) -> bool {
    static bool generic_ibecc_available(struct pci_dev *pdev)
    {
    void __iomem *base = igen6_pvt.memss_pma_cr;
    bool present;
    u32 val;
    if (res_cfg.reg_capabilities_misc_offset) {
    val = readl(base + res_cfg.reg_capabilities_misc_offset);
    present = !(val & res_cfg.reg_capabilities_misc_ibecc_dis);
    edac_dbg(2, "capabilities misc reg 0x%x\n", val);
    } else if (res_cfg.reg_mem_config_offset) {
    val = readl(base + res_cfg.reg_mem_config_offset);
    present = !!(val & res_cfg.reg_mem_config_ibecc_en_mask);
    edac_dbg(2, "mem config reg 0x%x\n", val);
    } else {
    igen6_printk(KERN_ERR, "No register for detecting IBECC presence.\n");
    present = false;
    }
    return present;
    }
#[no_mangle]
unsafe extern "C" fn mem_addr_to_sys_addr(maddr: u64) -> u64 {
    static u64 mem_addr_to_sys_addr(u64 maddr)
    {
    if (maddr < igen6_tolud)
    return maddr;
    if (igen6_tom <= _4GB)
    return maddr - igen6_tolud + _4GB;
    if (maddr < _4GB)
    return maddr - igen6_tolud + igen6_tom;
    return maddr;
    }
#[no_mangle]
unsafe extern "C" fn tgl_err_addr_to_mem_addr(eaddr: u64, mc: c_int) -> u64 {
    static u64 tgl_err_addr_to_mem_addr(u64 eaddr, int mc)
    {
    u64 mask, ms_s_size;
    int intlv_bit;
    u32 ms_hash;
    ms_s_size = igen6_pvt.ms_s_size;
    if (eaddr >= ms_s_size)
    return eaddr + ms_s_size;
    ms_hash = igen6_pvt.ms_hash;
    mask = MEM_SLICE_HASH_MASK(ms_hash);
    intlv_bit = MEM_SLICE_HASH_LSB_MASK_BIT(ms_hash) + 6;
    return translate_to_upper_level(eaddr, mask, mc, intlv_bit, ms_s_size);
    }
#[no_mangle]
unsafe extern "C" fn tgl_err_addr_to_sys_addr(eaddr: u64, mc: c_int) -> u64 {
    static u64 tgl_err_addr_to_sys_addr(u64 eaddr, int mc)
    {
    let mut maddr: u64 = tgl_err_addr_to_mem_addr(eaddr, mc);
    return mem_addr_to_sys_addr(maddr);
    }
#[no_mangle]
unsafe extern "C" fn tgl_err_addr_to_imc_addr(eaddr: u64, mc: c_int) -> u64 {
    static u64 tgl_err_addr_to_imc_addr(u64 eaddr, int mc)
    {
    return eaddr;
    }
#[no_mangle]
unsafe extern "C" fn adl_err_addr_to_sys_addr(eaddr: u64, mc: c_int) -> u64 {
    static u64 adl_err_addr_to_sys_addr(u64 eaddr, int mc)
    {
    return mem_addr_to_sys_addr(eaddr);
    }
#[no_mangle]
unsafe extern "C" fn adl_err_addr_to_imc_addr(eaddr: u64, mc: c_int) -> u64 {
    static u64 adl_err_addr_to_imc_addr(u64 eaddr, int mc)
    {
    let mut ms_s_size: u64 = igen6_pvt.ms_s_size;
    struct igen6_imc *imc = &igen6_pvt.imc[mc];
    struct slice slice;
    int intlv_bit;
    u32 mc_hash;
    if (eaddr >= 2 * ms_s_size)
    return eaddr - ms_s_size;
    mc_hash = readl(imc.window + MAD_MC_HASH_OFFSET);
    intlv_bit = MAC_MC_HASH_LSB(mc_hash) + 6;
    translate_to_lower_level(eaddr, 0, 0, intlv_bit, ms_s_size, 0, &slice);
    return slice.addr;
    }
#[no_mangle]
unsafe extern "C" fn ptl_h_get_mem_type(imc: *mut igen6_imc) -> enum mem_type {
    static enum mem_type ptl_h_get_mem_type(struct igen6_imc *imc)
    {
    u32 mtype, val;
    val = readl(igen6_pvt.memss_pma_cr + res_cfg.reg_mem_config_offset);
    mtype = field_get(res_cfg.reg_mem_config_ddr_type_mask, val);
    edac_dbg(2, "mtype %u (reg 0x%x)\n", mtype, val);
    switch (mtype) {
    case 1:
    return MEM_DDR5;
    case 2:
    return MEM_LPDDR5;
    case 3:
    return MEM_LPDDR4;
    default:
    return MEM_UNKNOWN;
    }
    }
#[no_mangle]
unsafe extern "C" fn ptl_h_get_dev_type(imc: *mut igen6_imc, chan: c_int, dimm: c_int) -> enum dev_type {
    static enum dev_type ptl_h_get_dev_type(struct igen6_imc *imc, int chan, int dimm)
    {
    u32 width, val;
    val = readl(imc.window + MAD_INTRA_CH0_OFFSET + chan * 4);
    width = field_get(res_cfg.reg_mad_intra_width_mask[dimm], val);
    switch (width) {
    case 1:
    return DEV_X8;
    default:
    return DEV_X16;
    }
    }
#[no_mangle]
unsafe extern "C" fn ptl_h_get_chan_size(imc: *mut igen6_imc, chan: c_int) -> u64 {
    static u64 ptl_h_get_chan_size(struct igen6_imc *imc, int chan)
    {
    let mut val: u32 = readl(imc.window + MAD_INTER_CHANNEL_OFFSET);
    return field_get(res_cfg.reg_mad_inter_size_mask[chan], val) *
    res_cfg.reg_mad_inter_size_granularity;
    }
#[no_mangle]
unsafe extern "C" fn ptl_h_get_dimm_size(imc: *mut igen6_imc, chan: c_int, dimm: c_int) -> u64 {
    static u64 ptl_h_get_dimm_size(struct igen6_imc *imc, int chan, int dimm)
    {
    let mut val: u32 = readl(imc.window + MAD_INTRA_CH0_OFFSET + chan * 4);
    let mut ranks: u32 = 1 << field_get(res_cfg.reg_mad_intra_rank_mask[dimm], val);
// DRAM device density in Gb
    let mut density: u64 = field_get(res_cfg.reg_mad_intra_density_mask[dimm], val) * 4;
    let mut mtype: enum mem_type = ptl_h_get_mem_type(imc);
    let mut dtype: enum dev_type = ptl_h_get_dev_type(imc, chan, dimm);
    u64 sub_ch_width, dev_num;
    switch (mtype) {
    case MEM_DDR5:
    sub_ch_width = 32;
    break;
    case MEM_LPDDR5:
    case MEM_LPDDR4:
    sub_ch_width = 16;
    break;
    default:
    sub_ch_width = 0;
    }
    switch (dtype) {
    case DEV_X8:
    dev_num = sub_ch_width / 8;
    break;
    case DEV_X16:
    dev_num = sub_ch_width / 16;
    break;
    default:
    dev_num = 0;
    }
    edac_dbg(2, "ranks %d, density %lluGb, sub_ch_width %llu, dev_num %llu (reg 0x%x)\n", ranks, density, sub_ch_width, dev_num, val);
    return ((dev_num * density / 8) * ranks) << 30;
    }
#[no_mangle]
unsafe extern "C" fn ptl_h_set_chan_params(imc: *mut igen6_imc) {
    static void ptl_h_set_chan_params(struct igen6_imc *imc)
    {
    let mut ch0_size: u64 = ptl_h_get_chan_size(imc, 0);
    let mut ch1_size: u64 = ptl_h_get_chan_size(imc, 1);
    if (ch0_size <= ch1_size) {
    imc.ch_s_size = ch0_size;
    imc.ch_l_map = 1;
    } else {
    imc.ch_s_size = ch1_size;
    imc.ch_l_map = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn ptl_h_set_dimm_params(imc: *mut igen6_imc, chan: c_int) {
    static void ptl_h_set_dimm_params(struct igen6_imc *imc, int chan)
    {
    let mut dimm0_size: u64 = ptl_h_get_dimm_size(imc, chan, 0);
    let mut dimm1_size: u64 = ptl_h_get_dimm_size(imc, chan, 1);
    if (dimm0_size <= dimm1_size) {
    imc.dimm_s_size[chan] = dimm0_size;
    imc.dimm_l_size[chan] = dimm1_size;
    imc.dimm_l_map[chan]  = 1;
    } else {
    imc.dimm_s_size[chan] = dimm1_size;
    imc.dimm_l_size[chan] = dimm0_size;
    imc.dimm_l_map[chan]  = 0;
    }
    }
    static struct res_config ehl_cfg = {
    .num_imc		= 1,
    .reg_mchbar_mask	= GENMASK_ULL(38, 16),
    .reg_tom_mask		= GENMASK_ULL(38, 20),
    .reg_touud_mask		= GENMASK_ULL(38, 20),
    .reg_eccerrlog_addr_mask = GENMASK_ULL(38, 5),
    .imc_base		= 0x5000,
    .ibecc_base		= 0xdc00,
    .ibecc_available	= ehl_ibecc_available,
    .ibecc_error_log_offset	= 0x170,
    .err_addr_to_sys_addr	= ehl_err_addr_to_sys_addr,
    .err_addr_to_imc_addr	= ehl_err_addr_to_imc_addr,
    };
    static struct res_config icl_cfg = {
    .num_imc		= 1,
    .reg_mchbar_mask	= GENMASK_ULL(38, 16),
    .reg_tom_mask		= GENMASK_ULL(38, 20),
    .reg_touud_mask		= GENMASK_ULL(38, 20),
    .reg_eccerrlog_addr_mask = GENMASK_ULL(38, 5),
    .imc_base		= 0x5000,
    .ibecc_base		= 0xd800,
    .ibecc_error_log_offset	= 0x170,
    .ibecc_available	= icl_ibecc_available,
    .err_addr_to_sys_addr	= ehl_err_addr_to_sys_addr,
    .err_addr_to_imc_addr	= ehl_err_addr_to_imc_addr,
    };
    static struct res_config tgl_cfg = {
    .machine_check		= true,
    .num_imc		= 2,
    .reg_mchbar_mask	= GENMASK_ULL(38, 17),
    .reg_tom_mask		= GENMASK_ULL(38, 20),
    .reg_touud_mask		= GENMASK_ULL(38, 20),
    .reg_eccerrlog_addr_mask = GENMASK_ULL(38, 5),
    .imc_base		= 0x5000,
    .cmf_base		= 0x11000,
    .cmf_size		= 0x800,
    .ms_hash_offset		= 0xac,
    .ibecc_base		= 0xd400,
    .ibecc_error_log_offset	= 0x170,
    .ibecc_available	= tgl_ibecc_available,
    .err_addr_to_sys_addr	= tgl_err_addr_to_sys_addr,
    .err_addr_to_imc_addr	= tgl_err_addr_to_imc_addr,
    };
// Shared by Alder Lake, Alder Lake-N, Arizona Beach, Amston Lake, and Raptor Lake-P
    static struct res_config adl_cfg = {
    .machine_check		= true,
    .num_imc		= 2,
    .reg_mchbar_mask	= GENMASK_ULL(41, 17),
    .reg_tom_mask		= GENMASK_ULL(41, 20),
    .reg_touud_mask		= GENMASK_ULL(41, 20),
    .reg_eccerrlog_addr_mask = GENMASK_ULL(45, 5),
    .imc_base		= 0xd800,
    .ibecc_base		= 0xd400,
    .ibecc_error_log_offset	= 0x68,
    .ibecc_available	= tgl_ibecc_available,
    .err_addr_to_sys_addr	= adl_err_addr_to_sys_addr,
    .err_addr_to_imc_addr	= adl_err_addr_to_imc_addr,
    };
    static struct res_config mtl_ps_cfg = {
    .machine_check				= true,
    .num_imc				= 2,
    .reg_mchbar_mask			= GENMASK_ULL(41, 17),
    .reg_tom_mask				= GENMASK_ULL(41, 20),
    .reg_touud_mask				= GENMASK_ULL(41, 20),
    .reg_eccerrlog_addr_mask		= GENMASK_ULL(38, 5),
    .reg_capabilities_misc_offset		= 0x13c00,
    .reg_capabilities_misc_ibecc_dis	= BIT(6),
    .imc_base				= 0xd800,
    .ibecc_base				= 0xd400,
    .ibecc_error_log_offset			= 0x170,
    .ibecc_available			= generic_ibecc_available,
    .err_addr_to_sys_addr			= adl_err_addr_to_sys_addr,
    .err_addr_to_imc_addr			= adl_err_addr_to_imc_addr,
    };
// Shared by Meteor Lake-P, Arrow Lake-UH, and Wildcat Lake
    static struct res_config mtl_p_cfg = {
    .machine_check		= true,
    .num_imc		= 2,
    .reg_mchbar_mask	= GENMASK_ULL(41, 17),
    .reg_tom_mask		= GENMASK_ULL(41, 20),
    .reg_touud_mask		= GENMASK_ULL(41, 20),
    .reg_eccerrlog_addr_mask = GENMASK_ULL(38, 5),
    .imc_base		= 0xd800,
    .ibecc_base		= 0xd400,
    .ibecc_error_log_offset	= 0x170,
    .ibecc_available	= mtl_p_ibecc_available,
    .err_addr_to_sys_addr	= adl_err_addr_to_sys_addr,
    .err_addr_to_imc_addr	= adl_err_addr_to_imc_addr,
    };
// Shared by Panther Lake-H and Starfire
    static struct res_config ptl_h_cfg = {
    .machine_check			= true,
    .num_imc			= 2,
    .reg_mchbar_mask		= GENMASK_ULL(41, 17),
    .reg_tom_mask			= GENMASK_ULL(41, 20),
    .reg_touud_mask			= GENMASK_ULL(41, 20),
    .reg_eccerrlog_addr_mask	= GENMASK_ULL(38, 5),
    .reg_mem_config_offset		= 0x13d04,
    .reg_mem_config_ddr_type_mask	= GENMASK(8, 6),
    .reg_mad_inter_size_mask[0]	= GENMASK(15, 8),
    .reg_mad_inter_size_mask[1]	= GENMASK(23, 16),
    .reg_mad_inter_size_granularity	= BIT_ULL(29),
    .reg_mad_intra_rank_mask[0]	= BIT(7),
    .reg_mad_intra_rank_mask[1]	= BIT(15),
    .reg_mad_intra_width_mask[0]	= BIT(6),
    .reg_mad_intra_width_mask[1]	= BIT(14),
    .reg_mad_intra_density_mask[0]	= GENMASK(3, 0),
    .reg_mad_intra_density_mask[1]	= GENMASK(11, 8),
    .imc_base			= 0xd800,
    .ibecc_base			= 0xd400,
    .ibecc_error_log_offset		= 0x170,
    .get_mem_type			= ptl_h_get_mem_type,
    .get_dev_type			= ptl_h_get_dev_type,
    .set_chan_params		= ptl_h_set_chan_params,
    .set_dimm_params		= ptl_h_set_dimm_params,
    .ibecc_available		= mtl_p_ibecc_available,
    .err_addr_to_sys_addr		= adl_err_addr_to_sys_addr,
    .err_addr_to_imc_addr		= adl_err_addr_to_imc_addr,
    };
    static struct res_config nvl_h_cfg = {
    .machine_check			= true,
    .num_imc			= 2,
    .reg_mchbar_mask		= GENMASK_ULL(41, 17),
    .reg_tom_mask			= GENMASK_ULL(41, 20),
    .reg_touud_mask			= GENMASK_ULL(41, 20),
    .reg_eccerrlog_addr_mask	= GENMASK_ULL(38, 5),
    .reg_mem_config_offset		= 0x12904,
    .reg_mem_config_ddr_type_mask	= GENMASK(8, 6),
    .reg_mem_config_ibecc_en_mask	= GENMASK(3, 2),
    .reg_mad_inter_size_mask[0]	= GENMASK(15, 8),
    .reg_mad_inter_size_mask[1]	= GENMASK(23, 16),
    .reg_mad_inter_size_granularity	= BIT_ULL(29),
    .reg_mad_intra_rank_mask[0]	= BIT(7),
    .reg_mad_intra_rank_mask[1]	= BIT(15),
    .reg_mad_intra_width_mask[0]	= BIT(6),
    .reg_mad_intra_width_mask[1]	= BIT(14),
    .reg_mad_intra_density_mask[0]	= GENMASK(3, 0),
    .reg_mad_intra_density_mask[1]	= GENMASK(11, 8),
    .imc_base			= 0xd800,
    .ibecc_base			= 0xd400,
    .ibecc_error_log_offset		= 0x170,
    .get_mem_type			= ptl_h_get_mem_type,
    .get_dev_type			= ptl_h_get_dev_type,
    .set_chan_params		= ptl_h_set_chan_params,
    .set_dimm_params		= ptl_h_set_dimm_params,
    .ibecc_available		= generic_ibecc_available,
    .err_addr_to_sys_addr		= adl_err_addr_to_sys_addr,
    .err_addr_to_imc_addr		= adl_err_addr_to_imc_addr,
    };
    static struct pci_device_id igen6_pci_tbl[] = {
    { PCI_VDEVICE(INTEL, DID_EHL_SKU5), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU6), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU7), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU8), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU9), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU10), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU11), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU12), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU13), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU14), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_EHL_SKU15), .driver_data = (kernel_ulong_t)&ehl_cfg },
    { PCI_VDEVICE(INTEL, DID_ICL_SKU8), .driver_data = (kernel_ulong_t)&icl_cfg },
    { PCI_VDEVICE(INTEL, DID_ICL_SKU10), .driver_data = (kernel_ulong_t)&icl_cfg },
    { PCI_VDEVICE(INTEL, DID_ICL_SKU11), .driver_data = (kernel_ulong_t)&icl_cfg },
    { PCI_VDEVICE(INTEL, DID_ICL_SKU12), .driver_data = (kernel_ulong_t)&icl_cfg },
    { PCI_VDEVICE(INTEL, DID_TGL_SKU), .driver_data = (kernel_ulong_t)&tgl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_SKU1), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_SKU2), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_SKU3), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_SKU4), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU1), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU2), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU3), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU4), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU5), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU6), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU7), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU8), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU9), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU10), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU11), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ADL_N_SKU12), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_AZB_SKU1), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ASL_SKU1), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ASL_SKU2), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_ASL_SKU3), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_RPL_P_SKU1), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_RPL_P_SKU2), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_RPL_P_SKU3), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_RPL_P_SKU4), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_RPL_P_SKU5), .driver_data = (kernel_ulong_t)&adl_cfg },
    { PCI_VDEVICE(INTEL, DID_MTL_PS_SKU1), .driver_data = (kernel_ulong_t)&mtl_ps_cfg },
    { PCI_VDEVICE(INTEL, DID_MTL_PS_SKU2), .driver_data = (kernel_ulong_t)&mtl_ps_cfg },
    { PCI_VDEVICE(INTEL, DID_MTL_PS_SKU3), .driver_data = (kernel_ulong_t)&mtl_ps_cfg },
    { PCI_VDEVICE(INTEL, DID_MTL_PS_SKU4), .driver_data = (kernel_ulong_t)&mtl_ps_cfg },
    { PCI_VDEVICE(INTEL, DID_MTL_P_SKU1), .driver_data = (kernel_ulong_t)&mtl_p_cfg },
    { PCI_VDEVICE(INTEL, DID_MTL_P_SKU2), .driver_data = (kernel_ulong_t)&mtl_p_cfg },
    { PCI_VDEVICE(INTEL, DID_MTL_P_SKU3), .driver_data = (kernel_ulong_t)&mtl_p_cfg },
    { PCI_VDEVICE(INTEL, DID_ARL_UH_SKU1), .driver_data = (kernel_ulong_t)&mtl_p_cfg },
    { PCI_VDEVICE(INTEL, DID_ARL_UH_SKU2), .driver_data = (kernel_ulong_t)&mtl_p_cfg },
    { PCI_VDEVICE(INTEL, DID_ARL_UH_SKU3), .driver_data = (kernel_ulong_t)&mtl_p_cfg },
    { PCI_VDEVICE(INTEL, DID_WCL_SKU1), .driver_data = (kernel_ulong_t)&mtl_p_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU1), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU2), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU3), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU4), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU5), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU6), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU7), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU8), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU9), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU10), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU11), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU12), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU13), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_PTL_H_SKU14), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_STF_SKU1), .driver_data = (kernel_ulong_t)&ptl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_NVL_H_SKU1), .driver_data = (kernel_ulong_t)&nvl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_NVL_H_SKU2), .driver_data = (kernel_ulong_t)&nvl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_NVL_H_SKU3), .driver_data = (kernel_ulong_t)&nvl_h_cfg },
    { PCI_VDEVICE(INTEL, DID_NVL_H_SKU4), .driver_data = (kernel_ulong_t)&nvl_h_cfg },
    { },
    };
    MODULE_DEVICE_TABLE(pci, igen6_pci_tbl);
#[no_mangle]
unsafe extern "C" fn get_mem_type(imc: *mut igen6_imc) -> enum mem_type {
    static enum mem_type get_mem_type(struct igen6_imc *imc)
    {
    u32 val;
    if (res_cfg.get_mem_type)
    return res_cfg.get_mem_type(imc);
    val = readl(imc.window + MAD_INTER_CHANNEL_OFFSET);
    switch (MAD_INTER_CHANNEL_DDR_TYPE(val)) {
    case 0:
    return MEM_DDR4;
    case 1:
    return MEM_DDR3;
    case 2:
    return MEM_LPDDR3;
    case 3:
    return MEM_LPDDR4;
    case 4:
    return MEM_WIO2;
    default:
    return MEM_UNKNOWN;
    }
    }
#[no_mangle]
unsafe extern "C" fn large_dimm(imc: *mut igen6_imc, chan: c_int, dimm: c_int) -> bool {
    static bool large_dimm(struct igen6_imc *imc, int chan, int dimm)
    {
    let mut dimm: return = = imc.dimm_l_map[chan];
    }
#[no_mangle]
unsafe extern "C" fn get_dev_type(imc: *mut igen6_imc, chan: c_int, dimm: c_int) -> enum dev_type {
    static enum dev_type get_dev_type(struct igen6_imc *imc, int chan, int dimm)
    {
    u32 width, val;
    if (res_cfg.get_dev_type)
    return res_cfg.get_dev_type(imc, chan, dimm);
    val = readl(imc.window + MAD_DIMM_CH0_OFFSET + chan * 4);
    width = large_dimm(imc, chan, dimm) ? MAD_DIMM_CH_DLW(val) :
    MAD_DIMM_CH_DSW(val);
    switch (width) {
    case 0:
    return DEV_X8;
    case 1:
    return DEV_X16;
    case 2:
    return DEV_X32;
    default:
    return DEV_UNKNOWN;
    }
    }
#[no_mangle]
unsafe extern "C" fn get_dimm_size(imc: *mut igen6_imc, chan: c_int, dimm: c_int) -> u64 {
    static u64 get_dimm_size(struct igen6_imc *imc, int chan, int dimm)
    {
    if (large_dimm(imc, chan, dimm))
    return imc.dimm_l_size[chan];
    return imc.dimm_s_size[chan];
    }
#[no_mangle]
unsafe extern "C" fn set_chan_params(imc: *mut igen6_imc) {
    static void set_chan_params(struct igen6_imc *imc)
    {
    u32 val;
    if (res_cfg.set_chan_params) {
    res_cfg.set_chan_params(imc);
    return;
    }
    val = readl(imc.window + MAD_INTER_CHANNEL_OFFSET);
    imc.ch_s_size = MAD_INTER_CHANNEL_CH_S_SIZE(val);
    imc.ch_l_map = MAD_INTER_CHANNEL_CH_L_MAP(val);
    }
#[no_mangle]
unsafe extern "C" fn set_dimm_params(imc: *mut igen6_imc, chan: c_int) {
    static void set_dimm_params(struct igen6_imc *imc, int chan)
    {
    u32 val;
    if (res_cfg.set_dimm_params) {
    res_cfg.set_dimm_params(imc, chan);
    return;
    }
    val = readl(imc.window + MAD_INTRA_CH0_OFFSET + chan * 4);
    imc.dimm_l_map[chan]  = MAD_INTRA_CH_DIMM_L_MAP(val);
    val = readl(imc.window + MAD_DIMM_CH0_OFFSET + chan * 4);
    imc.dimm_l_size[chan] = MAD_DIMM_CH_DIMM_L_SIZE(val);
    imc.dimm_s_size[chan] = MAD_DIMM_CH_DIMM_S_SIZE(val);
    }
#[no_mangle]
unsafe extern "C" fn igen6_decode(res: *mut decoded_addr) -> c_int {
    static int igen6_decode(struct decoded_addr *res)
    {
    struct igen6_imc *imc = &igen6_pvt.imc[res.mc];
    let mut addr: u64 = res.imc_addr, s_size;
    int intlv_bit, l_map;
    u32 hash, hash_mask;
    struct slice slice;
    if (addr >= igen6_tom) {
    edac_dbg(0, "Address 0x%llx out of range\n", addr);
    return -EINVAL;
    }
// Decode channel
    hash   = readl(imc.window + CHANNEL_HASH_OFFSET);
    s_size = imc.ch_s_size;
    l_map  = imc.ch_l_map;
    hash_mask = CHANNEL_HASH_MODE(hash) ? CHANNEL_HASH_MASK(hash) : 0;
    intlv_bit = CHANNEL_HASH_LSB_MASK_BIT(hash) + 6;
    translate_to_lower_level(addr, hash_mask, 0, intlv_bit, s_size, l_map, &slice);
    res.channel_idx  = slice.id;
    res.channel_addr = slice.addr;
// Decode sub-channel/DIMM
    hash   = readl(imc.window + CHANNEL_EHASH_OFFSET);
    s_size = imc.dimm_s_size[res.channel_idx];
    l_map  = imc.dimm_l_map[res.channel_idx];
    hash_mask = CHANNEL_HASH_MODE(hash) ? CHANNEL_HASH_MASK(hash) : 0;
    intlv_bit = CHANNEL_HASH_LSB_MASK_BIT(hash) + 6;
    translate_to_lower_level(res.channel_addr, hash_mask, 0, intlv_bit, s_size, l_map, &slice);
    res.sub_channel_idx  = slice.id;
    res.sub_channel_addr = slice.addr;
    return 0;
    }
    static void igen6_output_error(struct decoded_addr *res,
    struct mem_ctl_info *mci, u64 ecclog)
    {
    enum hw_event_mc_err_type type = ecclog & ECC_ERROR_LOG_UE ?
    HW_EVENT_ERR_UNCORRECTED :
    HW_EVENT_ERR_CORRECTED;
    edac_mc_handle_error(type, mci, 1,
    res.sys_addr >> PAGE_SHIFT,
    res.sys_addr & ~PAGE_MASK,
    ECC_ERROR_LOG_SYND(ecclog),
    res.channel_idx, res.sub_channel_idx,
    -1, "", "");
    }
    static struct gen_pool *ecclog_gen_pool_create(void)
    {
    struct gen_pool *pool;
    pool = gen_pool_create(ilog2(sizeof(struct ecclog_node)), -1);
    if (!pool)
    return core::ptr::null_mut();
    if (gen_pool_add(pool, (unsigned long)ecclog_buf, ECCLOG_POOL_SIZE, -1)) {
    gen_pool_destroy(pool);
    return core::ptr::null_mut();
    }
    return pool;
    }
#[no_mangle]
unsafe extern "C" fn ecclog_gen_pool_add(mc: c_int, ecclog: u64) -> c_int {
    static int ecclog_gen_pool_add(int mc, u64 ecclog)
    {
    struct ecclog_node *node;
    node = (void *)gen_pool_alloc(ecclog_pool, sizeof(*node));
    if (!node)
    return -ENOMEM;
    node.mc = mc;
    node.ecclog = ecclog;
    llist_add(&node.llnode, &ecclog_llist);
    return 0;
    }
//
// Either the memory-mapped I/O status register ECC_ERROR_LOG or the PCI
// configuration space status register ERRSTS can indicate whether a
// correctable error or an uncorrectable error occurred. We only use the
// ECC_ERROR_LOG register to check error type, but need to clear both
// registers to enable future error events.
//
#[no_mangle]
unsafe extern "C" fn ecclog_read_and_clear(imc: *mut igen6_imc) -> u64 {
    static u64 ecclog_read_and_clear(struct igen6_imc *imc)
    {
    let mut ecclog: u64 = readq(imc.window + ECC_ERROR_LOG_OFFSET);
//
// Quirk: The ECC_ERROR_LOG register of certain SoCs may contain
// the invalid value ~0. This will result in a flood of invalid
// error reports in polling mode. Skip it.
//
    if (ecclog == ~0)
    return 0;
// Neither a CE nor a UE. Skip it.
    if (!(ecclog & (ECC_ERROR_LOG_CE | ECC_ERROR_LOG_UE)))
    return 0;
// Clear CE/UE bits by writing 1s
    writeq(ecclog, imc.window + ECC_ERROR_LOG_OFFSET);
    return ecclog;
    }
#[no_mangle]
unsafe extern "C" fn errsts_clear(imc: *mut igen6_imc) {
    static void errsts_clear(struct igen6_imc *imc)
    {
    u16 errsts;
    if (pci_read_config_word(imc.pdev, ERRSTS_OFFSET, &errsts)) {
    igen6_printk(KERN_ERR, "Failed to read ERRSTS\n");
    return;
    }
// Clear CE/UE bits by writing 1s
    if (errsts & (ERRSTS_CE | ERRSTS_UE))
    pci_write_config_word(imc.pdev, ERRSTS_OFFSET, errsts);
    }
#[no_mangle]
unsafe extern "C" fn errcmd_enable_error_reporting(enable: bool) -> c_int {
    static int errcmd_enable_error_reporting(bool enable)
    {
    struct igen6_imc *imc = &igen6_pvt.imc[0];
    u16 errcmd;
    int rc;
    rc = pci_read_config_word(imc.pdev, ERRCMD_OFFSET, &errcmd);
    if (rc)
    return pcibios_err_to_errno(rc);
    if (enable)
    errcmd |= ERRCMD_CE | ERRSTS_UE;
    else
    errcmd &= ~(ERRCMD_CE | ERRSTS_UE);
    rc = pci_write_config_word(imc.pdev, ERRCMD_OFFSET, errcmd);
    if (rc)
    return pcibios_err_to_errno(rc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ecclog_handler() -> c_int {
    static int ecclog_handler(void)
    {
    struct igen6_imc *imc;
    int i, n = 0;
    u64 ecclog;
    for (i = 0; i < res_cfg.num_imc; i++) {
    imc = &igen6_pvt.imc[i];
// errsts_clear() isn't NMI-safe. Delay it in the IRQ context
    ecclog = ecclog_read_and_clear(imc);
    if (!ecclog)
    continue;
    if (!ecclog_gen_pool_add(i, ecclog))
    irq_work_queue(&ecclog_irq_work);
    n++;
    }
    return n;
    }
#[no_mangle]
unsafe extern "C" fn ecclog_work_cb(work: *mut work_struct) {
    static void ecclog_work_cb(struct work_struct *work)
    {
    struct ecclog_node *node, *tmp;
    struct mem_ctl_info *mci;
    struct llist_node *head;
    struct decoded_addr res;
    u64 eaddr;
    head = llist_del_all(&ecclog_llist);
    if (!head)
    return;
    llist_for_each_entry_safe(node, tmp, head, llnode) {
    memset(&res, 0, sizeof(res));
    eaddr	     = node.ecclog & res_cfg.reg_eccerrlog_addr_mask;
    res.mc	     = node.mc;
    res.sys_addr = res_cfg.err_addr_to_sys_addr(eaddr, res.mc);
    res.imc_addr = res_cfg.err_addr_to_imc_addr(eaddr, res.mc);
    mci = igen6_pvt.imc[res.mc].mci;
    edac_dbg(2, "MC %d, ecclog = 0x%llx\n", node.mc, node.ecclog);
    igen6_mc_printk(mci, KERN_DEBUG, "HANDLING IBECC MEMORY ERROR\n");
    igen6_mc_printk(mci, KERN_DEBUG, "ADDR 0x%llx ", res.sys_addr);
    if (!igen6_decode(&res))
    igen6_output_error(&res, mci, node.ecclog);
    gen_pool_free(ecclog_pool, (unsigned long)node, sizeof(*node));
    }
    }
#[no_mangle]
unsafe extern "C" fn ecclog_irq_work_cb(irq_work: *mut irq_work) {
    static void ecclog_irq_work_cb(struct irq_work *irq_work)
    {
    int i;
    for (i = 0; i < res_cfg.num_imc; i++)
    errsts_clear(&igen6_pvt.imc[i]);
    if (!llist_empty(&ecclog_llist))
    schedule_work(&ecclog_work);
    }
#[no_mangle]
unsafe extern "C" fn ecclog_nmi_handler(cmd: c_uint, regs: *mut pt_regs) -> c_int {
    static int ecclog_nmi_handler(unsigned int cmd, struct pt_regs *regs)
    {
    unsigned char reason;
    if (!ecclog_handler())
    return NMI_DONE;
//
// Both In-Band ECC correctable error and uncorrectable error are
// reported by SERR# NMI. The NMI generic code (see pci_serr_error())
// doesn't clear the bit NMI_REASON_CLEAR_SERR (in port 0x61) to
// re-enable the SERR# NMI after NMI handling. So clear this bit here
// to re-enable SERR# NMI for receiving future In-Band ECC errors.
//
    reason  = x86_platform.get_nmi_reason() & NMI_REASON_CLEAR_MASK;
    reason |= NMI_REASON_CLEAR_SERR;
    outb(reason, NMI_REASON_PORT);
    reason &= ~NMI_REASON_CLEAR_SERR;
    outb(reason, NMI_REASON_PORT);
    return NMI_HANDLED;
    }
    static int ecclog_mce_handler(struct notifier_block *nb, unsigned long val,
    void *data)
    {
    struct mce *mce = (struct mce *)data;
    char *type;
    if (mce.kflags & MCE_HANDLED_CEC)
    return NOTIFY_DONE;
//
// Ignore unless this is a memory related error.
// We don't check the bit MCI_STATUS_ADDRV of MCi_STATUS here,
// since this bit isn't set on some CPU (e.g., Tiger Lake UP3).
//
    if ((mce.status & 0xefff) >> 7 != 1)
    return NOTIFY_DONE;
    if (mce.mcgstatus & MCG_STATUS_MCIP)
    type = "Exception";
    else
    type = "Event";
    edac_dbg(0, "CPU %d: Machine Check %s: 0x%llx Bank %d: 0x%llx\n",
    mce.extcpu, type, mce.mcgstatus,
    mce.bank, mce.status);
    edac_dbg(0, "TSC 0x%llx\n", mce.tsc);
    edac_dbg(0, "ADDR 0x%llx\n", mce.addr);
    edac_dbg(0, "MISC 0x%llx\n", mce.misc);
    edac_dbg(0, "PROCESSOR %u:0x%x TIME %llu SOCKET %u APIC 0x%x\n",
    mce.cpuvendor, mce.cpuid, mce.time,
    mce.socketid, mce.apicid);
//
// We just use the Machine Check for the memory error notification.
// Each memory controller is associated with an IBECC instance.
// Directly read and clear the error information(error address and
// error type) on all the IBECC instances so that we know on which
// memory controller the memory error(s) occurred.
//
    if (!ecclog_handler())
    return NOTIFY_DONE;
    mce.kflags |= MCE_HANDLED_EDAC;
    return NOTIFY_DONE;
    }
    static struct notifier_block ecclog_mce_dec = {
    .notifier_call	= ecclog_mce_handler,
    .priority	= MCE_PRIO_EDAC,
    };
#[no_mangle]
unsafe extern "C" fn igen6_check_ecc(imc: *mut igen6_imc) -> bool {
    static bool igen6_check_ecc(struct igen6_imc *imc)
    {
    let mut activate: u32 = readl(imc.window + IBECC_ACTIVATE_OFFSET);
    return !!(activate & IBECC_ACTIVATE_EN);
    }
#[no_mangle]
unsafe extern "C" fn igen6_get_dimm_config(mci: *mut mem_ctl_info) -> c_int {
    static int igen6_get_dimm_config(struct mem_ctl_info *mci)
    {
    struct igen6_imc *imc = mci.pvt_info;
    int i, j, ndimms, mc = imc.mc;
    struct dimm_info *dimm;
    enum mem_type mtype;
    enum dev_type dtype;
    u64 dsize;
    bool ecc;
    edac_dbg(2, "\n");
    mtype = get_mem_type(imc);
    ecc = igen6_check_ecc(imc);
    set_chan_params(imc);
    for (i = 0; i < NUM_CHANNELS; i++) {
    set_dimm_params(imc, i);
    imc.size += imc.dimm_s_size[i];
    imc.size += imc.dimm_l_size[i];
    ndimms = 0;
    for (j = 0; j < NUM_DIMMS; j++) {
    dimm = edac_get_dimm(mci, i, j, 0);
    dtype = get_dev_type(imc, i, j);
    dsize = get_dimm_size(imc, i, j);
    if (!dsize)
    continue;
    dimm.grain = 64;
    dimm.mtype = mtype;
    dimm.dtype = dtype;
    dimm.nr_pages  = MiB_TO_PAGES(dsize >> 20);
    dimm.edac_mode = EDAC_SECDED;
    snprintf(dimm.label, sizeof(dimm.label),
    "MC#%d_Chan#%d_DIMM#%d", mc, i, j);
    edac_dbg(0, "MC %d, Channel %d, DIMM %d, Size %llu MiB (%u pages)\n",
    mc, i, j, dsize >> 20, dimm.nr_pages);
    ndimms++;
    }
    if (ndimms && !ecc) {
    igen6_printk(KERN_ERR, "MC%d In-Band ECC is disabled\n", mc);
    return -ENODEV;
    }
    }
    edac_dbg(0, "MC %d, total size %llu MiB\n", mc, imc.size >> 20);
    return 0;
    }

// Top of upper usable DRAM
    static u64 igen6_touud;
pub const TOUUD_OFFSET: c_uint = 0xa8;
#[no_mangle]
unsafe extern "C" fn igen6_reg_dump(imc: *mut igen6_imc) {
    static void igen6_reg_dump(struct igen6_imc *imc)
    {
    int i;
    edac_dbg(2, "CHANNEL_HASH     : 0x%x\n",
    readl(imc.window + CHANNEL_HASH_OFFSET));
    edac_dbg(2, "CHANNEL_EHASH    : 0x%x\n",
    readl(imc.window + CHANNEL_EHASH_OFFSET));
    edac_dbg(2, "MAD_INTER_CHANNEL: 0x%x\n",
    readl(imc.window + MAD_INTER_CHANNEL_OFFSET));
    edac_dbg(2, "ECC_ERROR_LOG    : 0x%llx\n",
    readq(imc.window + ECC_ERROR_LOG_OFFSET));
    for (i = 0; i < NUM_CHANNELS; i++) {
    edac_dbg(2, "MAD_INTRA_CH%d    : 0x%x\n", i,
    readl(imc.window + MAD_INTRA_CH0_OFFSET + i * 4));
    edac_dbg(2, "MAD_DIMM_CH%d     : 0x%x\n", i,
    readl(imc.window + MAD_DIMM_CH0_OFFSET + i * 4));
    }
    edac_dbg(2, "TOLUD            : 0x%x", igen6_tolud);
    edac_dbg(2, "TOUUD            : 0x%llx", igen6_touud);
    edac_dbg(2, "TOM              : 0x%llx", igen6_tom);
    }
    static struct dentry *igen6_test;
#[no_mangle]
unsafe extern "C" fn debugfs_u64_set(data: *mut c_void, val: u64) -> c_int {
    static int debugfs_u64_set(void *data, u64 val)
    {
    u64 ecclog;
    if ((val >= igen6_tolud && val < _4GB) || val >= igen6_touud) {
    edac_dbg(0, "Address 0x%llx out of range\n", val);
    return 0;
    }
    pr_warn_once("Fake error to 0x%llx injected via debugfs\n", val);
    ecclog = (val & res_cfg.reg_eccerrlog_addr_mask) | ECC_ERROR_LOG_CE;
    if (!ecclog_gen_pool_add(0, ecclog))
    irq_work_queue(&ecclog_irq_work);
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(fops_u64_wo, core::ptr::null_mut(), debugfs_u64_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn igen6_debug_setup() {
    static void igen6_debug_setup(void)
    {
    igen6_test = edac_debugfs_create_dir("igen6_test");
    if (!igen6_test)
    return;
    if (!edac_debugfs_create_file("addr", 0200, igen6_test,
    core::ptr::null_mut(), &fops_u64_wo)) {
    debugfs_remove(igen6_test);
    igen6_test = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn igen6_debug_teardown() {
    static void igen6_debug_teardown(void)
    {
    debugfs_remove_recursive(igen6_test);
    }

    static void igen6_reg_dump(struct igen6_imc *imc) {}
    static void igen6_debug_setup(void) {}
    static void igen6_debug_teardown(void) {}

    static struct igen6_pvt *igen6_pvt_setup(struct pci_dev *pdev)
    {
    void __iomem *memss_pma_cr;
    struct igen6_pvt *pvt;
    int imc_num, rc;
    u64 mchbar;
    rc = get_mchbar(pdev, &mchbar);
    if (rc)
    return core::ptr::null_mut();
    imc_num = get_imc_num(mchbar);
    if (!imc_num) {
    igen6_printk(KERN_ERR, "No mc found.\n");
    return core::ptr::null_mut();
    }
    edac_dbg(2, "%d mcs found.\n", imc_num);
// Use the runtime detected IMC count.
    if (res_cfg.num_imc != imc_num)
    res_cfg.num_imc = imc_num;
    pvt = kzalloc_flex(*pvt, imc, imc_num);
    if (!pvt)
    return core::ptr::null_mut();
    memss_pma_cr = ioremap(mchbar, MCHBAR_SIZE * 2);
    if (!memss_pma_cr) {
    kfree(pvt);
    return core::ptr::null_mut();
    }
    pvt.memss_pma_cr = memss_pma_cr;
    return pvt;
    }
#[no_mangle]
unsafe extern "C" fn igen6_pvt_release(pvt: *mut igen6_pvt) {
    static void igen6_pvt_release(struct igen6_pvt *pvt)
    {
    iounmap(pvt.memss_pma_cr);
    kfree(pvt);
    }
#[no_mangle]
unsafe extern "C" fn igen6_pci_setup(pdev: *mut pci_dev, mchbar: *mut u64) -> c_int {
    static int igen6_pci_setup(struct pci_dev *pdev, u64 *mchbar)
    {
    union  {
    u64 v;
    struct {
    u32 v_lo;
    u32 v_hi;
    };
    } u;
    edac_dbg(2, "\n");
    if (!res_cfg.ibecc_available(pdev)) {
    edac_dbg(2, "No In-Band ECC IP\n");
    goto fail;
    }
    if (pci_read_config_dword(pdev, TOLUD_OFFSET, &igen6_tolud)) {
    igen6_printk(KERN_ERR, "Failed to read TOLUD\n");
    goto fail;
    }
    igen6_tolud &= GENMASK(31, 20);
    if (pci_read_config_dword(pdev, TOM_OFFSET, &u.v_lo)) {
    igen6_printk(KERN_ERR, "Failed to read lower TOM\n");
    goto fail;
    }
    if (pci_read_config_dword(pdev, TOM_OFFSET + 4, &u.v_hi)) {
    igen6_printk(KERN_ERR, "Failed to read upper TOM\n");
    goto fail;
    }
    igen6_tom = u.v & res_cfg.reg_tom_mask;
    if (get_mchbar(pdev, mchbar))
    goto fail;

    if (pci_read_config_dword(pdev, TOUUD_OFFSET, &u.v_lo))
    edac_dbg(2, "Failed to read lower TOUUD\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pci_read_config_dword(pdev, 4: TOUUD_OFFSET +, _arg: &u.v_hi)) -> else {
    else if (pci_read_config_dword(pdev, TOUUD_OFFSET + 4, &u.v_hi))
    edac_dbg(2, "Failed to read upper TOUUD\n");
    else
    igen6_touud = u.v & res_cfg.reg_touud_mask;

    return 0;
    fail:
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn igen6_check(mci: *mut mem_ctl_info) {
    static void igen6_check(struct mem_ctl_info *mci)
    {
    struct igen6_imc *imc = mci.pvt_info;
    u64 ecclog;
// errsts_clear() isn't NMI-safe. Delay it in the IRQ context
    ecclog = ecclog_read_and_clear(imc);
    if (!ecclog)
    return;
    if (!ecclog_gen_pool_add(imc.mc, ecclog))
    irq_work_queue(&ecclog_irq_work);
    }
#[no_mangle]
unsafe extern "C" fn imc_release(dev: *mut device) {
    static void imc_release(struct device *dev)
    {
// Nothing to do, the 'imc' owns the 'dev' and will also release it.
    }
#[no_mangle]
unsafe extern "C" fn igen6_register_mci(mc: c_int, window: *mut void __iomem, pdev: *mut pci_dev) -> c_int {
    static int igen6_register_mci(int mc, void __iomem *window, struct pci_dev *pdev)
    {
    struct edac_mc_layer layers[2];
    struct mem_ctl_info *mci;
    struct igen6_imc *imc;
    int rc;
    edac_dbg(2, "\n");
    layers[0].type = EDAC_MC_LAYER_CHANNEL;
    layers[0].size = NUM_CHANNELS;
    layers[0].is_virt_csrow = false;
    layers[1].type = EDAC_MC_LAYER_SLOT;
    layers[1].size = NUM_DIMMS;
    layers[1].is_virt_csrow = true;
    mci = edac_mc_alloc(mc, ARRAY_SIZE(layers), layers, 0);
    if (!mci) {
    rc = -ENOMEM;
    goto fail;
    }
    mci.ctl_name = kasprintf(GFP_KERNEL, "Intel_client_SoC MC#%d", mc);
    if (!mci.ctl_name) {
    rc = -ENOMEM;
    goto fail2;
    }
    mci.mtype_cap = MEM_FLAG_LPDDR4 | MEM_FLAG_DDR4;
    mci.edac_ctl_cap = EDAC_FLAG_SECDED;
    mci.edac_cap = EDAC_FLAG_SECDED;
    mci.mod_name = EDAC_MOD_STR;
    mci.dev_name = pci_name(pdev);
    if (edac_op_state == EDAC_OPSTATE_POLL)
    mci.edac_check = igen6_check;
    mci.pvt_info = &igen6_pvt.imc[mc];
    imc = mci.pvt_info;
    imc.dev.release = imc_release;
    device_initialize(&imc.dev);
//
// EDAC core uses mci->pdev(pointer of structure device) as
// memory controller ID. The client SoCs attach one or more
// memory controllers to single pci_dev (single pci_dev->dev
// can be for multiple memory controllers).
//
// To make mci->pdev unique, assign pci_dev->dev to mci->pdev
// for the first memory controller and assign a unique imc->dev
// to mci->pdev for each non-first memory controller.
//
    mci.pdev = mc ? &imc.dev : &pdev.dev;
    imc.mc	= mc;
    imc.pdev = pdev;
    imc.window = window;
    igen6_reg_dump(imc);
    rc = igen6_get_dimm_config(mci);
    if (rc)
    goto fail3;
    rc = edac_mc_add_mc(mci);
    if (rc) {
    igen6_printk(KERN_ERR, "Failed to register mci#%d\n", mc);
    goto fail3;
    }
    imc.mci = mci;
    return 0;
    fail3:
    put_device(&imc.dev);
    mci.pvt_info = core::ptr::null_mut();
    kfree(mci.ctl_name);
    fail2:
    edac_mc_free(mci);
    fail:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn igen6_unregister_mcis() {
    static void igen6_unregister_mcis(void)
    {
    struct mem_ctl_info *mci;
    struct igen6_imc *imc;
    int i;
    edac_dbg(2, "\n");
    for (i = 0; i < res_cfg.num_imc; i++) {
    imc = &igen6_pvt.imc[i];
    mci = imc.mci;
    if (!mci)
    continue;
    edac_mc_del_mc(mci.pdev);
    kfree(mci.ctl_name);
    mci.pvt_info = core::ptr::null_mut();
    edac_mc_free(mci);
    put_device(&imc.dev);
    iounmap(imc.window);
    }
    }
#[no_mangle]
unsafe extern "C" fn igen6_register_mcis(pdev: *mut pci_dev, mchbar: u64) -> c_int {
    static int igen6_register_mcis(struct pci_dev *pdev, u64 mchbar)
    {
    void __iomem *window;
    int lmc, pmc, rc;
    for (lmc = 0, pmc = 0; pmc < MAX_IMC_TO_PROBE; pmc++) {
    window = map_imc_window(mchbar, pmc);
    if (!window)
    continue;
    rc = igen6_register_mci(lmc, window, pdev);
    if (rc)
    goto err_unregister;
// Done, if all present MCs are detected and registered.
    if (++lmc >= res_cfg.num_imc)
    break;
    }
    if (!lmc) {
    igen6_printk(KERN_ERR, "No mc found.\n");
    return -ENODEV;
    }
    if (lmc < res_cfg.num_imc) {
    igen6_printk(KERN_DEBUG, "Expected %d mcs, but only %d detected.",
    res_cfg.num_imc, lmc);
    res_cfg.num_imc = lmc;
    }
    return 0;
    err_unregister:
    iounmap(window);
    igen6_unregister_mcis();
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn igen6_mem_slice_setup(mchbar: u64) -> c_int {
    static int igen6_mem_slice_setup(u64 mchbar)
    {
    struct igen6_imc *imc = &igen6_pvt.imc[0];
    let mut base: u64 = mchbar + res_cfg.cmf_base;
    let mut offset: u32 = res_cfg.ms_hash_offset;
    let mut size: u32 = res_cfg.cmf_size;
    u64 ms_s_size, ms_hash;
    void __iomem *cmf;
    int ms_l_map;
    edac_dbg(2, "\n");
    if (imc[0].size < imc[1].size) {
    ms_s_size = imc[0].size;
    ms_l_map  = 1;
    } else {
    ms_s_size = imc[1].size;
    ms_l_map  = 0;
    }
    igen6_pvt.ms_s_size = ms_s_size;
    igen6_pvt.ms_l_map  = ms_l_map;
    edac_dbg(0, "ms_s_size: %llu MiB, ms_l_map %d\n",
    ms_s_size >> 20, ms_l_map);
    if (!size)
    return 0;
    cmf = ioremap(base, size);
    if (!cmf) {
    igen6_printk(KERN_ERR, "Failed to ioremap cmf 0x%llx\n", base);
    return -ENODEV;
    }
    ms_hash = readq(cmf + offset);
    igen6_pvt.ms_hash = ms_hash;
    edac_dbg(0, "MEM_SLICE_HASH: 0x%llx\n", ms_hash);
    iounmap(cmf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn register_err_handler() -> c_int {
    static int register_err_handler(void)
    {
    int rc;
    if (res_cfg.machine_check) {
    mce_register_decode_chain(&ecclog_mce_dec);
    return 0;
    }
    rc = register_nmi_handler(NMI_SERR, ecclog_nmi_handler,
    0, IGEN6_NMI_NAME);
    if (rc) {
    igen6_printk(KERN_ERR, "Failed to register NMI handler\n");
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unregister_err_handler() {
    static void unregister_err_handler(void)
    {
    if (res_cfg.machine_check) {
    mce_unregister_decode_chain(&ecclog_mce_dec);
    return;
    }
    unregister_nmi_handler(NMI_SERR, IGEN6_NMI_NAME);
    }
#[no_mangle]
unsafe extern "C" fn opstate_set(cfg: *const res_config, ent: *const pci_device_id) {
    static void opstate_set(const struct res_config *cfg, const struct pci_device_id *ent)
    {
//
// Quirk: Certain SoCs' error reporting interrupts don't work.
// Force polling mode for them to ensure that memory error
// events can be handled.
//
    if (ent.device == DID_ADL_N_SKU4) {
    edac_op_state = EDAC_OPSTATE_POLL;
    return;
    }
// Set the mode according to the configuration data.
    if (cfg.machine_check)
    edac_op_state = EDAC_OPSTATE_INT;
    else
    edac_op_state = EDAC_OPSTATE_NMI;
    }
#[no_mangle]
unsafe extern "C" fn igen6_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int igen6_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    u64 mchbar;
    int rc;
    edac_dbg(2, "\n");
    res_cfg = (struct res_config *)ent.driver_data;
    igen6_pvt = igen6_pvt_setup(pdev);
    if (!igen6_pvt)
    return -ENOMEM;
    rc = igen6_pci_setup(pdev, &mchbar);
    if (rc)
    goto fail;
    opstate_set(res_cfg, ent);
    rc = igen6_register_mcis(pdev, mchbar);
    if (rc)
    goto fail;
    if (res_cfg.num_imc > 1) {
    rc = igen6_mem_slice_setup(mchbar);
    if (rc)
    goto fail2;
    }
    ecclog_pool = ecclog_gen_pool_create();
    if (!ecclog_pool) {
    rc = -ENOMEM;
    goto fail2;
    }
    INIT_WORK(&ecclog_work, ecclog_work_cb);
    init_irq_work(&ecclog_irq_work, ecclog_irq_work_cb);
    rc = register_err_handler();
    if (rc)
    goto fail3;
// Enable error reporting
    rc = errcmd_enable_error_reporting(true);
    if (rc) {
    igen6_printk(KERN_ERR, "Failed to enable error reporting\n");
    goto fail4;
    }
// Check if any pending errors before/during the registration of the error handler
    ecclog_handler();
    igen6_debug_setup();
    return 0;
    fail4:
    unregister_nmi_handler(NMI_SERR, IGEN6_NMI_NAME);
    fail3:
    gen_pool_destroy(ecclog_pool);
    fail2:
    igen6_unregister_mcis();
    fail:
    igen6_pvt_release(igen6_pvt);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn igen6_remove(pdev: *mut pci_dev) {
    static void igen6_remove(struct pci_dev *pdev)
    {
    edac_dbg(2, "\n");
    igen6_debug_teardown();
    errcmd_enable_error_reporting(false);
    unregister_err_handler();
    irq_work_sync(&ecclog_irq_work);
    flush_work(&ecclog_work);
    gen_pool_destroy(ecclog_pool);
    igen6_unregister_mcis();
    igen6_pvt_release(igen6_pvt);
    }
    static struct pci_driver igen6_driver = {
    .name     = EDAC_MOD_STR,
    .probe    = igen6_probe,
    .remove   = igen6_remove,
    .id_table = igen6_pci_tbl,
    };
#[no_mangle]
unsafe extern "C" fn igen6_init() -> int __init {
    static int __init igen6_init(void)
    {
    const char *owner;
    int rc;
    edac_dbg(2, "\n");
    if (ghes_get_devices())
    return -EBUSY;
    owner = edac_get_owner();
    if (owner && strncmp(owner, EDAC_MOD_STR, sizeof(EDAC_MOD_STR)))
    return -EBUSY;
    rc = pci_register_driver(&igen6_driver);
    if (rc)
    return rc;
    igen6_printk(KERN_INFO, "%s\n", IGEN6_REVISION);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn igen6_exit() -> void __exit {
    static void __exit igen6_exit(void)
    {
    edac_dbg(2, "\n");
    pci_unregister_driver(&igen6_driver);
    }
    module_init(igen6_init);
    module_exit(igen6_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Qiuxu Zhuo");
    MODULE_DESCRIPTION("MC Driver for Intel client SoC using In-Band ECC");
