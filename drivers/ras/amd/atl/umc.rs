//! Automatically rewritten from C to Rust
//! Source: drivers/ras/amd/atl/umc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AMD Address Translation Library
//
// umc.c : Unified Memory Controller (UMC) topology helpers
//
// Copyright (c) 2023, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Yazen Ghannam <Yazen.Ghannam@amd.com>
//

//
// MI300 has a fixed, model-specific mapping between a UMC instance and
// its related Data Fabric Coherent Station instance.
//
// The MCA_IPID_UMC[InstanceId] field holds a unique identifier for the
// UMC instance within a Node. Use this to find the appropriate Coherent
// Station ID.
//
// Redundant bits were removed from the map below.
//
    static const u16 umc_coh_st_map[32] = {
    0x393, 0x293, 0x193, 0x093,
    0x392, 0x292, 0x192, 0x092,
    0x391, 0x291, 0x191, 0x091,
    0x390, 0x290, 0x190, 0x090,
    0x793, 0x693, 0x593, 0x493,
    0x792, 0x692, 0x592, 0x492,
    0x791, 0x691, 0x591, 0x491,
    0x790, 0x690, 0x590, 0x490,
    };

#[no_mangle]
unsafe extern "C" fn get_coh_st_inst_id_mi300(err: *mut atl_err) -> u8 {
    static u8 get_coh_st_inst_id_mi300(struct atl_err *err)
    {
    let mut umc_id: u16 = FIELD_GET(UMC_ID_MI300, err.ipid);
    u8 i;
    for (i = 0; i < ARRAY_SIZE(umc_coh_st_map); i++) {
    if (umc_id == umc_coh_st_map[i])
    break;
    }
    WARN_ON_ONCE(i >= ARRAY_SIZE(umc_coh_st_map));
    return i;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xor_bits {
    pub xor_enable: bool,
    pub col_xor: u16,
    pub row_xor: u32,
}

pub const NUM_BANK_BITS: c_int = 4;
pub const NUM_COL_BITS: c_int = 5;
pub const NUM_SID_BITS: c_int = 2;
    static struct {
// UMC::CH::AddrHashBank
    struct xor_bits	bank[NUM_BANK_BITS];
// UMC::CH::AddrHashPC
    struct xor_bits	pc;
// UMC::CH::AddrHashPC2
    u8		bank_xor;
    } addr_hash;
    static struct {
    u8 bank[NUM_BANK_BITS];
    u8 col[NUM_COL_BITS];
    u8 sid[NUM_SID_BITS];
    u8 num_row_lo;
    u8 num_row_hi;
    u8 row_lo;
    u8 row_hi;
    u8 pc;
    } bit_shifts;
pub const MI300_UMC_CH_BASE: c_uint = 0x90000;

//
// Read UMC::CH::AddrHash{Bank,PC,PC2} registers to get XOR bits used
// for hashing.
//
// Also, read UMC::CH::Addr{Cfg,Sel,Sel2} and UMC::CH:ColSelLo registers to
// get the values needed to reconstruct the normalized address. Apply additional
// offsets to the raw register values, as needed.
//
// Do this during module init, since the values will not change during run time.
//
// These registers are instantiated for each UMC across each AMD Node.
// However, they should be identically programmed due to the fixed hardware
// design of MI300 systems. So read the values from Node 0 UMC 0 and keep a
// single global structure for simplicity.
//
#[no_mangle]
pub unsafe extern "C" fn get_umc_info_mi300() -> c_int {
    int get_umc_info_mi300(void)
    {
    u32 temp;
    int ret;
    u8 i;
    for (i = 0; i < NUM_BANK_BITS; i++) {
    ret = amd_smn_read(0, MI300_ADDR_HASH_BANK0 + (i * 4), &temp);
    if (ret)
    return ret;
    addr_hash.bank[i].xor_enable = FIELD_GET(ADDR_HASH_XOR_EN,  temp);
    addr_hash.bank[i].col_xor    = FIELD_GET(ADDR_HASH_COL_XOR, temp);
    addr_hash.bank[i].row_xor    = FIELD_GET(ADDR_HASH_ROW_XOR, temp);
    }
    ret = amd_smn_read(0, MI300_ADDR_HASH_PC, &temp);
    if (ret)
    return ret;
    addr_hash.pc.xor_enable = FIELD_GET(ADDR_HASH_XOR_EN,  temp);
    addr_hash.pc.col_xor    = FIELD_GET(ADDR_HASH_COL_XOR, temp);
    addr_hash.pc.row_xor    = FIELD_GET(ADDR_HASH_ROW_XOR, temp);
    ret = amd_smn_read(0, MI300_ADDR_HASH_PC2, &temp);
    if (ret)
    return ret;
    addr_hash.bank_xor = FIELD_GET(ADDR_HASH_BANK_XOR, temp);
    ret = amd_smn_read(0, MI300_ADDR_CFG, &temp);
    if (ret)
    return ret;
    bit_shifts.num_row_hi = FIELD_GET(ADDR_CFG_NUM_ROW_HI, temp);
    bit_shifts.num_row_lo = 10 + FIELD_GET(ADDR_CFG_NUM_ROW_LO, temp);
    ret = amd_smn_read(0, MI300_ADDR_SEL, &temp);
    if (ret)
    return ret;
    bit_shifts.bank[0] = 5 + FIELD_GET(ADDR_SEL_BANK0, temp);
    bit_shifts.bank[1] = 5 + FIELD_GET(ADDR_SEL_BANK1, temp);
    bit_shifts.bank[2] = 5 + FIELD_GET(ADDR_SEL_BANK2, temp);
    bit_shifts.bank[3] = 5 + FIELD_GET(ADDR_SEL_BANK3, temp);
// Use BankBit4 for the SID0 position.
    bit_shifts.sid[0]  = 5 + FIELD_GET(ADDR_SEL_BANK4, temp);
    bit_shifts.row_lo  = 12 + FIELD_GET(ADDR_SEL_ROW_LO, temp);
    bit_shifts.row_hi  = 24 + FIELD_GET(ADDR_SEL_ROW_HI, temp);
    ret = amd_smn_read(0, MI300_COL_SEL_LO, &temp);
    if (ret)
    return ret;
    bit_shifts.col[0] = 2 + FIELD_GET(COL_SEL_LO_COL0, temp);
    bit_shifts.col[1] = 2 + FIELD_GET(COL_SEL_LO_COL1, temp);
    bit_shifts.col[2] = 2 + FIELD_GET(COL_SEL_LO_COL2, temp);
    bit_shifts.col[3] = 2 + FIELD_GET(COL_SEL_LO_COL3, temp);
    bit_shifts.col[4] = 2 + FIELD_GET(COL_SEL_LO_COL4, temp);
    ret = amd_smn_read(0, MI300_ADDR_SEL_2, &temp);
    if (ret)
    return ret;
// Use BankBit5 for the SID1 position.
    bit_shifts.sid[1] = 5 + FIELD_GET(ADDR_SEL_2_BANK5, temp);
    bit_shifts.pc	  = 5 + FIELD_GET(ADDR_SEL_2_CHAN, temp);
    return 0;
    }
//
// MI300 systems report a DRAM address in MCA_ADDR for DRAM ECC errors. This must
// be converted to the intermediate normalized address (NA) before translating to a
// system physical address.
//
// The DRAM address includes bank, row, and column. Also included are bits for
// pseudochannel (PC) and stack ID (SID).
//
// Abbreviations: (S)tack ID, (P)seudochannel, (R)ow, (B)ank, (C)olumn, (Z)ero
//
// The MCA address format is as follows:
// MCA_ADDR[27:0] = {S[1:0], P[0], R[14:0], B[3:0], C[4:0], Z[0]}
//
// Additionally, the PC and Bank bits may be hashed. This must be accounted for before
// reconstructing the normalized address.
//

#[no_mangle]
unsafe extern "C" fn convert_dram_to_norm_addr_mi300(addr: c_ulong) -> c_ulong {
    static unsigned long convert_dram_to_norm_addr_mi300(unsigned long addr)
    {
    u16 i, col, row, bank, pc, sid;
    u32 temp;
    col  = FIELD_GET(MI300_UMC_MCA_COL,  addr);
    bank = FIELD_GET(MI300_UMC_MCA_BANK, addr);
    row  = FIELD_GET(MI300_UMC_MCA_ROW,  addr);
    pc   = FIELD_GET(MI300_UMC_MCA_PC,   addr);
    sid  = FIELD_GET(MI300_UMC_MCA_SID,  addr);
// Calculate hash for each Bank bit.
    for (i = 0; i < NUM_BANK_BITS; i++) {
    if (!addr_hash.bank[i].xor_enable)
    continue;
    temp  = hweight16(col & addr_hash.bank[i].col_xor) & 1;
    temp ^= hweight16(row & addr_hash.bank[i].row_xor) & 1;
    bank ^= temp << i;
    }
// Calculate hash for PC bit.
    if (addr_hash.pc.xor_enable) {
    temp  = hweight16(col & addr_hash.pc.col_xor) & 1;
    temp ^= hweight16(row & addr_hash.pc.row_xor) & 1;
// Bits SID[1:0] act as Bank[5:4] for PC hash, so apply them here.
    temp ^= hweight16((bank | sid << NUM_BANK_BITS) & addr_hash.bank_xor) & 1;
    pc   ^= temp;
    }
// Reconstruct the normalized address starting with NA[4:0] = 0
    addr  = 0;
// Column bits
    for (i = 0; i < NUM_COL_BITS; i++) {
    temp  = (col >> i) & 0x1;
    addr |= temp << bit_shifts.col[i];
    }
// Bank bits
    for (i = 0; i < NUM_BANK_BITS; i++) {
    temp  = (bank >> i) & 0x1;
    addr |= temp << bit_shifts.bank[i];
    }
// Row lo bits
    for (i = 0; i < bit_shifts.num_row_lo; i++) {
    temp  = (row >> i) & 0x1;
    addr |= temp << (i + bit_shifts.row_lo);
    }
// Row hi bits
    for (i = 0; i < bit_shifts.num_row_hi; i++) {
    temp  = (row >> (i + bit_shifts.num_row_lo)) & 0x1;
    addr |= temp << (i + bit_shifts.row_hi);
    }
// PC bit
    addr |= pc << bit_shifts.pc;
// SID bits
    for (i = 0; i < NUM_SID_BITS; i++) {
    temp  = (sid >> i) & 0x1;
    addr |= temp << bit_shifts.sid[i];
    }
    pr_debug("Addr=0x%016lx", addr);
    pr_debug("Bank=%u Row=%u Column=%u PC=%u SID=%u", bank, row, col, pc, sid);
    return addr;
    }
//
// When a DRAM ECC error occurs on MI300 systems, it is recommended to retire
// all memory within that DRAM row. This applies to the memory with a DRAM
// bank.
//
// To find the memory addresses, loop through permutations of the DRAM column
// bits and find the System Physical address of each. The column bits are used
// to calculate the intermediate Normalized address, so all permutations should
// be checked.
//
// See amd_atl::convert_dram_to_norm_addr_mi300() for MI300 address formats.
//

#[no_mangle]
unsafe extern "C" fn _retire_row_mi300(a_err: *mut atl_err) {
    static void _retire_row_mi300(struct atl_err *a_err)
    {
    unsigned long addr;
    struct page *p;
    u8 col;
    for (col = 0; col < MI300_NUM_COL; col++) {
    a_err.addr &= ~MI300_UMC_MCA_COL;
    a_err.addr |= FIELD_PREP(MI300_UMC_MCA_COL, col);
    addr = amd_convert_umc_mca_addr_to_sys_addr(a_err);
    if (IS_ERR_VALUE(addr))
    continue;
    addr = PHYS_PFN(addr);
//
// Skip invalid or already poisoned pages to avoid unnecessary
// error messages from memory_failure().
//
    p = pfn_to_online_page(addr);
    if (!p)
    continue;
    if (PageHWPoison(p))
    continue;
    memory_failure(addr, 0);
    }
    }
//
// In addition to the column bits, the row[13] bit should also be included when
// calculating addresses affected by a physical row.
//
// Instead of running through another loop over a single bit, just run through
// the column bits twice and flip the row[13] bit in-between.
//
// See MI300_UMC_MCA_ROW for the row bits in MCA_ADDR_UMC value.
//
#[no_mangle]
unsafe extern "C" fn retire_row_mi300(a_err: *mut atl_err) {
    static void retire_row_mi300(struct atl_err *a_err)
    {
    _retire_row_mi300(a_err);
    a_err.addr ^= MI300_UMC_MCA_ROW13;
    _retire_row_mi300(a_err);
    }
#[no_mangle]
pub unsafe extern "C" fn amd_retire_dram_row(a_err: *mut atl_err) {
    void amd_retire_dram_row(struct atl_err *a_err)
    {
    if (df_cfg.rev == DF4p5 && df_cfg.flags.heterogeneous)
    return retire_row_mi300(a_err);
    }
    EXPORT_SYMBOL_GPL(amd_retire_dram_row);
#[no_mangle]
unsafe extern "C" fn get_addr(addr: c_ulong) -> c_ulong {
    static unsigned long get_addr(unsigned long addr)
    {
    if (df_cfg.rev == DF4p5 && df_cfg.flags.heterogeneous)
    return convert_dram_to_norm_addr_mi300(addr);
    return addr;
    }

#[no_mangle]
unsafe extern "C" fn get_die_id(err: *mut atl_err) -> u8 {
    static u8 get_die_id(struct atl_err *err)
    {
//
// AMD Node ID is provided in MCA_IPID[InstanceIdHi], and this
// needs to be divided by 4 to get the internal Die ID.
//
    if (df_cfg.rev == DF4p5 && df_cfg.flags.heterogeneous) {
    let mut node_id: u8 = FIELD_GET(MCA_IPID_INST_ID_HI, err.ipid);
    return node_id >> 2;
    }
//
// For CPUs, this is the AMD Node ID modulo the number
// of AMD Nodes per socket.
//
    return topology_amd_node_id(err.cpu) % topology_amd_nodes_per_pkg();
    }

#[no_mangle]
unsafe extern "C" fn get_coh_st_inst_id(err: *mut atl_err) -> u8 {
    static u8 get_coh_st_inst_id(struct atl_err *err)
    {
    if (df_cfg.rev == DF4p5 && df_cfg.flags.heterogeneous)
    return get_coh_st_inst_id_mi300(err);
    return FIELD_GET(UMC_CHANNEL_NUM, err.ipid);
    }
#[no_mangle]
pub unsafe extern "C" fn convert_umc_mca_addr_to_sys_addr(err: *mut atl_err) -> c_ulong {
    unsigned long convert_umc_mca_addr_to_sys_addr(struct atl_err *err)
    {
    let mut socket_id: u8 = topology_physical_package_id(err.cpu);
    let mut coh_st_inst_id: u8 = get_coh_st_inst_id(err);
    let mut addr: c_ulong = get_addr(err.addr);
    let mut die_id: u8 = get_die_id(err);
    unsigned long ret_addr;
    pr_debug("socket_id=0x%x die_id=0x%x coh_st_inst_id=0x%x addr=0x%016lx",
    socket_id, die_id, coh_st_inst_id, addr);
    ret_addr = prm_umc_norm_to_sys_addr(socket_id, err.ipid, addr);
    if (!IS_ERR_VALUE(ret_addr) || df_cfg.flags.prm_only)
    return ret_addr;
    return norm_to_sys_addr(socket_id, die_id, coh_st_inst_id, addr);
    }
