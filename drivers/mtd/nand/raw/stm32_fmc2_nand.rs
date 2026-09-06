//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/stm32_fmc2_nand.c
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
// Copyright (C) STMicroelectronics 2018
// Author: Christophe Kerello <christophe.kerello@st.com>
//

// Bad block marker length
pub const FMC2_BBM_LEN: c_int = 2;
// ECC step size
pub const FMC2_ECC_STEP_SIZE: c_int = 512;
// BCHDSRx registers length
pub const FMC2_BCHDSRS_LEN: c_int = 20;
// HECCR length
pub const FMC2_HECCR_LEN: c_int = 4;
// Max requests done for a 8k nand page size
pub const FMC2_MAX_SG: c_int = 16;
// Max chip enable
pub const FMC2_MAX_CE: c_int = 4;
// Max ECC buffer length

pub const FMC2_TIMEOUT_MS: c_int = 5000;
// Timings
pub const FMC2_THIZ: c_int = 1;
pub const FMC2_TIO: c_int = 8000;
pub const FMC2_TSYNC: c_int = 3000;
pub const FMC2_PCR_TIMING_MASK: c_uint = 0xf;
pub const FMC2_PMEM_PATT_TIMING_MASK: c_uint = 0xff;
// FMC2 Controller Registers
pub const FMC2_BCR1: c_uint = 0x0;
pub const FMC2_PCR: c_uint = 0x80;
pub const FMC2_SR: c_uint = 0x84;
pub const FMC2_PMEM: c_uint = 0x88;
pub const FMC2_PATT: c_uint = 0x8c;
pub const FMC2_HECCR: c_uint = 0x94;
pub const FMC2_ISR: c_uint = 0x184;
pub const FMC2_ICR: c_uint = 0x188;
pub const FMC2_CSQCR: c_uint = 0x200;
pub const FMC2_CSQCFGR1: c_uint = 0x204;
pub const FMC2_CSQCFGR2: c_uint = 0x208;
pub const FMC2_CSQCFGR3: c_uint = 0x20c;
pub const FMC2_CSQAR1: c_uint = 0x210;
pub const FMC2_CSQAR2: c_uint = 0x214;
pub const FMC2_CSQIER: c_uint = 0x220;
pub const FMC2_CSQISR: c_uint = 0x224;
pub const FMC2_CSQICR: c_uint = 0x228;
pub const FMC2_CSQEMSR: c_uint = 0x230;
pub const FMC2_BCHIER: c_uint = 0x250;
pub const FMC2_BCHISR: c_uint = 0x254;
pub const FMC2_BCHICR: c_uint = 0x258;
pub const FMC2_BCHPBR1: c_uint = 0x260;
pub const FMC2_BCHPBR2: c_uint = 0x264;
pub const FMC2_BCHPBR3: c_uint = 0x268;
pub const FMC2_BCHPBR4: c_uint = 0x26c;
pub const FMC2_BCHDSR0: c_uint = 0x27c;
pub const FMC2_BCHDSR1: c_uint = 0x280;
pub const FMC2_BCHDSR2: c_uint = 0x284;
pub const FMC2_BCHDSR3: c_uint = 0x288;
pub const FMC2_BCHDSR4: c_uint = 0x28c;
// Register: FMC2_BCR1

// Register: FMC2_PCR

pub const FMC2_PCR_PWID_BUSWIDTH_8: c_int = 0;
pub const FMC2_PCR_PWID_BUSWIDTH_16: c_int = 1;

pub const FMC2_PCR_TCLR_DEFAULT: c_uint = 0xf;

pub const FMC2_PCR_TAR_DEFAULT: c_uint = 0xf;

pub const FMC2_PCR_ECCSS_512: c_int = 1;
pub const FMC2_PCR_ECCSS_2048: c_int = 3;

// Register: FMC2_SR

// Register: FMC2_PMEM

pub const FMC2_PMEM_DEFAULT: c_uint = 0x0a0a0a0a;
// Register: FMC2_PATT

pub const FMC2_PATT_DEFAULT: c_uint = 0x0a0a0a0a;
// Register: FMC2_ISR

// Register: FMC2_ICR

// Register: FMC2_CSQCR

// Register: FMC2_CSQCFGR1

// Register: FMC2_CSQCFGR2

// Register: FMC2_CSQCFGR3

// Register: FMC2_CSQCAR1

// Register: FMC2_CSQCAR2

// Register: FMC2_CSQIER

// Register: FMC2_CSQICR

// Register: FMC2_CSQEMSR

// Register: FMC2_BCHIER

// Register: FMC2_BCHICR

// Register: FMC2_BCHDSR0

// Register: FMC2_BCHDSR1

// Register: FMC2_BCHDSR2

// Register: FMC2_BCHDSR3

// Register: FMC2_BCHDSR4

    enum stm32_fmc2_ecc {
    FMC2_ECC_HAM = 1,
    FMC2_ECC_BCH4 = 4,
    FMC2_ECC_BCH8 = 8
    };
    enum stm32_fmc2_irq_state {
    FMC2_IRQ_UNKNOWN = 0,
    FMC2_IRQ_BCH,
    FMC2_IRQ_SEQ
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_fmc2_timings {
    pub tclr: u8,
    pub tar: u8,
    pub thiz: u8,
    pub twait: u8,
    pub thold_mem: u8,
    pub tset_mem: u8,
    pub thold_att: u8,
    pub tset_att: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_fmc2_nand {
    pub chip: nand_chip,
    pub wp_gpio: *mut gpio_desc,
    pub timings: stm32_fmc2_timings,
    pub ncs: c_int,
    pub cs_used: [c_int; FMC2_MAX_CE],
}

    static inline struct stm32_fmc2_nand *to_fmc2_nand(struct nand_chip *chip)
    {
    return container_of(chip, struct stm32_fmc2_nand, chip);
    }
    struct stm32_fmc2_nfc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_fmc2_nfc_data {
    pub max_ncs: c_int,
    pub nfc): *mut *mut int (set_cdev)(struct stm32_fmc2_nfc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_fmc2_nfc {
    pub base: nand_controller,
    pub nand: stm32_fmc2_nand,
    pub dev: *mut device,
    pub cdev: *mut device,
    pub regmap: *mut regmap,
    pub data_base: [*mut void __iomem; FMC2_MAX_CE],
    pub cmd_base: [*mut void __iomem; FMC2_MAX_CE],
    pub addr_base: [*mut void __iomem; FMC2_MAX_CE],
    pub io_phys_addr: phys_addr_t,
    pub data_phys_addr: [phys_addr_t; FMC2_MAX_CE],
    pub clk: *mut clk,
    pub irq_state: u8,
    pub data: *const stm32_fmc2_nfc_data,
    pub dma_tx_ch: *mut dma_chan,
    pub dma_rx_ch: *mut dma_chan,
    pub dma_ecc_ch: *mut dma_chan,
    pub dma_data_sg: sg_table,
    pub dma_ecc_sg: sg_table,
    pub ecc_buf: *mut u8,
    pub dma_ecc_addr: dma_addr_t,
    pub dma_ecc_len: c_int,
    pub tx_dma_max_burst: u32,
    pub rx_dma_max_burst: u32,
    pub complete: completion,
    pub dma_data_complete: completion,
    pub dma_ecc_complete: completion,
    pub cs_assigned: u8,
    pub cs_sel: c_int,
}

    static inline struct stm32_fmc2_nfc *to_stm32_nfc(struct nand_controller *base)
    {
    return container_of(base, struct stm32_fmc2_nfc, base);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_timings_init(chip: *mut nand_chip) {
    static void stm32_fmc2_nfc_timings_init(struct nand_chip *chip)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    struct stm32_fmc2_nand *nand = to_fmc2_nand(chip);
    struct stm32_fmc2_timings *timings = &nand.timings;
    u32 pmem, patt;
// Set tclr/tar timings
    regmap_update_bits(nfc.regmap, FMC2_PCR,
    FMC2_PCR_TCLR | FMC2_PCR_TAR,
    FIELD_PREP(FMC2_PCR_TCLR, timings.tclr) |
    FIELD_PREP(FMC2_PCR_TAR, timings.tar));
// Set tset/twait/thold/thiz timings in common bank
    pmem = FIELD_PREP(FMC2_PMEM_MEMSET, timings.tset_mem);
    pmem |= FIELD_PREP(FMC2_PMEM_MEMWAIT, timings.twait);
    pmem |= FIELD_PREP(FMC2_PMEM_MEMHOLD, timings.thold_mem);
    pmem |= FIELD_PREP(FMC2_PMEM_MEMHIZ, timings.thiz);
    regmap_write(nfc.regmap, FMC2_PMEM, pmem);
// Set tset/twait/thold/thiz timings in attribut bank
    patt = FIELD_PREP(FMC2_PATT_ATTSET, timings.tset_att);
    patt |= FIELD_PREP(FMC2_PATT_ATTWAIT, timings.twait);
    patt |= FIELD_PREP(FMC2_PATT_ATTHOLD, timings.thold_att);
    patt |= FIELD_PREP(FMC2_PATT_ATTHIZ, timings.thiz);
    regmap_write(nfc.regmap, FMC2_PATT, patt);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_setup(chip: *mut nand_chip) {
    static void stm32_fmc2_nfc_setup(struct nand_chip *chip)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    let mut pcr: u32 = 0, pcr_mask;
// Configure ECC algorithm (default configuration is Hamming)
    pcr_mask = FMC2_PCR_ECCALG;
    pcr_mask |= FMC2_PCR_BCHECC;
    if (chip.ecc.strength == FMC2_ECC_BCH8) {
    pcr |= FMC2_PCR_ECCALG;
    pcr |= FMC2_PCR_BCHECC;
    } else if (chip.ecc.strength == FMC2_ECC_BCH4) {
    pcr |= FMC2_PCR_ECCALG;
    }
// Set buswidth
    pcr_mask |= FMC2_PCR_PWID;
    if (chip.options & NAND_BUSWIDTH_16)
    pcr |= FIELD_PREP(FMC2_PCR_PWID, FMC2_PCR_PWID_BUSWIDTH_16);
// Set ECC sector size
    pcr_mask |= FMC2_PCR_ECCSS;
    pcr |= FIELD_PREP(FMC2_PCR_ECCSS, FMC2_PCR_ECCSS_512);
    regmap_update_bits(nfc.regmap, FMC2_PCR, pcr_mask, pcr);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_select_chip(chip: *mut nand_chip, chipnr: c_int) -> c_int {
    static int stm32_fmc2_nfc_select_chip(struct nand_chip *chip, int chipnr)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    struct stm32_fmc2_nand *nand = to_fmc2_nand(chip);
    struct dma_slave_config dma_cfg;
    int ret;
    if (nand.cs_used[chipnr] == nfc.cs_sel)
    return 0;
    nfc.cs_sel = nand.cs_used[chipnr];
    stm32_fmc2_nfc_setup(chip);
    stm32_fmc2_nfc_timings_init(chip);
    if (nfc.dma_tx_ch) {
    memset(&dma_cfg, 0, sizeof(dma_cfg));
    dma_cfg.dst_addr = nfc.data_phys_addr[nfc.cs_sel];
    dma_cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dma_cfg.dst_maxburst = nfc.tx_dma_max_burst /
    dma_cfg.dst_addr_width;
    ret = dmaengine_slave_config(nfc.dma_tx_ch, &dma_cfg);
    if (ret) {
    dev_err(nfc.dev, "tx DMA engine slave config failed\n");
    return ret;
    }
    }
    if (nfc.dma_rx_ch) {
    memset(&dma_cfg, 0, sizeof(dma_cfg));
    dma_cfg.src_addr = nfc.data_phys_addr[nfc.cs_sel];
    dma_cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dma_cfg.src_maxburst = nfc.rx_dma_max_burst /
    dma_cfg.src_addr_width;
    ret = dmaengine_slave_config(nfc.dma_rx_ch, &dma_cfg);
    if (ret) {
    dev_err(nfc.dev, "rx DMA engine slave config failed\n");
    return ret;
    }
    }
    if (nfc.dma_ecc_ch) {
//
// Hamming: we read HECCR register
// BCH4/BCH8: we read BCHDSRSx registers
//
    memset(&dma_cfg, 0, sizeof(dma_cfg));
    dma_cfg.src_addr = nfc.io_phys_addr;
    dma_cfg.src_addr += chip.ecc.strength == FMC2_ECC_HAM ?
    FMC2_HECCR : FMC2_BCHDSR0;
    dma_cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    ret = dmaengine_slave_config(nfc.dma_ecc_ch, &dma_cfg);
    if (ret) {
    dev_err(nfc.dev, "ECC DMA engine slave config failed\n");
    return ret;
    }
// Calculate ECC length needed for one sector
    nfc.dma_ecc_len = chip.ecc.strength == FMC2_ECC_HAM ?
    FMC2_HECCR_LEN : FMC2_BCHDSRS_LEN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_set_buswidth_16(nfc: *mut stm32_fmc2_nfc, set: bool) {
    static void stm32_fmc2_nfc_set_buswidth_16(struct stm32_fmc2_nfc *nfc, bool set)
    {
    u32 pcr;
    pcr = set ? FIELD_PREP(FMC2_PCR_PWID, FMC2_PCR_PWID_BUSWIDTH_16) :
    FIELD_PREP(FMC2_PCR_PWID, FMC2_PCR_PWID_BUSWIDTH_8);
    regmap_update_bits(nfc.regmap, FMC2_PCR, FMC2_PCR_PWID, pcr);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_set_ecc(nfc: *mut stm32_fmc2_nfc, enable: bool) {
    static void stm32_fmc2_nfc_set_ecc(struct stm32_fmc2_nfc *nfc, bool enable)
    {
    regmap_update_bits(nfc.regmap, FMC2_PCR, FMC2_PCR_ECCEN,
    enable ? FMC2_PCR_ECCEN : 0);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_enable_seq_irq(nfc: *mut stm32_fmc2_nfc) {
    static void stm32_fmc2_nfc_enable_seq_irq(struct stm32_fmc2_nfc *nfc)
    {
    nfc.irq_state = FMC2_IRQ_SEQ;
    regmap_update_bits(nfc.regmap, FMC2_CSQIER,
    FMC2_CSQIER_TCIE, FMC2_CSQIER_TCIE);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_disable_seq_irq(nfc: *mut stm32_fmc2_nfc) {
    static void stm32_fmc2_nfc_disable_seq_irq(struct stm32_fmc2_nfc *nfc)
    {
    regmap_update_bits(nfc.regmap, FMC2_CSQIER, FMC2_CSQIER_TCIE, 0);
    nfc.irq_state = FMC2_IRQ_UNKNOWN;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_clear_seq_irq(nfc: *mut stm32_fmc2_nfc) {
    static void stm32_fmc2_nfc_clear_seq_irq(struct stm32_fmc2_nfc *nfc)
    {
    regmap_write(nfc.regmap, FMC2_CSQICR, FMC2_CSQICR_CLEAR_IRQ);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_enable_bch_irq(nfc: *mut stm32_fmc2_nfc, mode: c_int) {
    static void stm32_fmc2_nfc_enable_bch_irq(struct stm32_fmc2_nfc *nfc, int mode)
    {
    nfc.irq_state = FMC2_IRQ_BCH;
    if (mode == NAND_ECC_WRITE)
    regmap_update_bits(nfc.regmap, FMC2_BCHIER,
    FMC2_BCHIER_EPBRIE, FMC2_BCHIER_EPBRIE);
    else
    regmap_update_bits(nfc.regmap, FMC2_BCHIER,
    FMC2_BCHIER_DERIE, FMC2_BCHIER_DERIE);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_disable_bch_irq(nfc: *mut stm32_fmc2_nfc) {
    static void stm32_fmc2_nfc_disable_bch_irq(struct stm32_fmc2_nfc *nfc)
    {
    regmap_update_bits(nfc.regmap, FMC2_BCHIER,
    FMC2_BCHIER_DERIE | FMC2_BCHIER_EPBRIE, 0);
    nfc.irq_state = FMC2_IRQ_UNKNOWN;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_clear_bch_irq(nfc: *mut stm32_fmc2_nfc) {
    static void stm32_fmc2_nfc_clear_bch_irq(struct stm32_fmc2_nfc *nfc)
    {
    regmap_write(nfc.regmap, FMC2_BCHICR, FMC2_BCHICR_CLEAR_IRQ);
    }
//
// Enable ECC logic and reset syndrome/parity bits previously calculated
// Syndrome/parity bits is cleared by setting the ECCEN bit to 0
//
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_hwctl(chip: *mut nand_chip, mode: c_int) {
    static void stm32_fmc2_nfc_hwctl(struct nand_chip *chip, int mode)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    stm32_fmc2_nfc_set_ecc(nfc, false);
    if (chip.ecc.strength != FMC2_ECC_HAM) {
    regmap_update_bits(nfc.regmap, FMC2_PCR, FMC2_PCR_WEN,
    mode == NAND_ECC_WRITE ? FMC2_PCR_WEN : 0);
    reinit_completion(&nfc.complete);
    stm32_fmc2_nfc_clear_bch_irq(nfc);
    stm32_fmc2_nfc_enable_bch_irq(nfc, mode);
    }
    stm32_fmc2_nfc_set_ecc(nfc, true);
    }
//
// ECC Hamming calculation
// ECC is 3 bytes for 512 bytes of data (supports error correction up to
// max of 1-bit)
//
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_ham_set_ecc(ecc_sta: u32, ecc: *mut u8) {
    static void stm32_fmc2_nfc_ham_set_ecc(const u32 ecc_sta, u8 *ecc)
    {
    ecc[0] = ecc_sta;
    ecc[1] = ecc_sta >> 8;
    ecc[2] = ecc_sta >> 16;
    }
    static int stm32_fmc2_nfc_ham_calculate(struct nand_chip *chip, const u8 *data,
    u8 *ecc)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    u32 sr, heccr;
    int ret;
    ret = regmap_read_poll_timeout(nfc.regmap, FMC2_SR, sr,
    sr & FMC2_SR_NWRF, 1,
    1000 * FMC2_TIMEOUT_MS);
    if (ret) {
    dev_err(nfc.dev, "ham timeout\n");
    return ret;
    }
    regmap_read(nfc.regmap, FMC2_HECCR, &heccr);
    stm32_fmc2_nfc_ham_set_ecc(heccr, ecc);
    stm32_fmc2_nfc_set_ecc(nfc, false);
    return 0;
    }
    static int stm32_fmc2_nfc_ham_correct(struct nand_chip *chip, u8 *dat,
    u8 *read_ecc, u8 *calc_ecc)
    {
    let mut bit_position: u8 = 0, b0, b1, b2;
    let mut byte_addr: u32 = 0, b;
    u32 i, shifting = 1;
// Indicate which bit and byte is faulty (if any)
    b0 = read_ecc[0] ^ calc_ecc[0];
    b1 = read_ecc[1] ^ calc_ecc[1];
    b2 = read_ecc[2] ^ calc_ecc[2];
    b = b0 | (b1 << 8) | (b2 << 16);
// No errors
    if (likely(!b))
    return 0;
// Calculate bit position
    for (i = 0; i < 3; i++) {
    switch (b % 4) {
    case 2:
    bit_position += shifting;
    break;
    case 1:
    break;
    default:
    return -EBADMSG;
    }
    shifting <<= 1;
    b >>= 2;
    }
// Calculate byte position
    shifting = 1;
    for (i = 0; i < 9; i++) {
    switch (b % 4) {
    case 2:
    byte_addr += shifting;
    break;
    case 1:
    break;
    default:
    return -EBADMSG;
    }
    shifting <<= 1;
    b >>= 2;
    }
// Flip the bit
    dat[byte_addr] ^= (1 << bit_position);
    return 1;
    }
//
// ECC BCH calculation and correction
// ECC is 7/13 bytes for 512 bytes of data (supports error correction up to
// max of 4-bit/8-bit)
//
    static int stm32_fmc2_nfc_bch_calculate(struct nand_chip *chip, const u8 *data,
    u8 *ecc)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    u32 bchpbr;
// Wait until the BCH code is ready
    if (!wait_for_completion_timeout(&nfc.complete,
    msecs_to_jiffies(FMC2_TIMEOUT_MS))) {
    dev_err(nfc.dev, "bch timeout\n");
    stm32_fmc2_nfc_disable_bch_irq(nfc);
    return -ETIMEDOUT;
    }
// Read parity bits
    regmap_read(nfc.regmap, FMC2_BCHPBR1, &bchpbr);
    ecc[0] = bchpbr;
    ecc[1] = bchpbr >> 8;
    ecc[2] = bchpbr >> 16;
    ecc[3] = bchpbr >> 24;
    regmap_read(nfc.regmap, FMC2_BCHPBR2, &bchpbr);
    ecc[4] = bchpbr;
    ecc[5] = bchpbr >> 8;
    ecc[6] = bchpbr >> 16;
    if (chip.ecc.strength == FMC2_ECC_BCH8) {
    ecc[7] = bchpbr >> 24;
    regmap_read(nfc.regmap, FMC2_BCHPBR3, &bchpbr);
    ecc[8] = bchpbr;
    ecc[9] = bchpbr >> 8;
    ecc[10] = bchpbr >> 16;
    ecc[11] = bchpbr >> 24;
    regmap_read(nfc.regmap, FMC2_BCHPBR4, &bchpbr);
    ecc[12] = bchpbr;
    }
    stm32_fmc2_nfc_set_ecc(nfc, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_bch_decode(eccsize: c_int, dat: *mut u8, ecc_sta: *mut u32) -> c_int {
    static int stm32_fmc2_nfc_bch_decode(int eccsize, u8 *dat, u32 *ecc_sta)
    {
    let mut bchdsr0: u32 = ecc_sta[0];
    let mut bchdsr1: u32 = ecc_sta[1];
    let mut bchdsr2: u32 = ecc_sta[2];
    let mut bchdsr3: u32 = ecc_sta[3];
    let mut bchdsr4: u32 = ecc_sta[4];
    u16 pos[8];
    int i, den;
    let mut nb_errs: c_uint = 0;
// No errors found
    if (likely(!(bchdsr0 & FMC2_BCHDSR0_DEF)))
    return 0;
// Too many errors detected
    if (unlikely(bchdsr0 & FMC2_BCHDSR0_DUE))
    return -EBADMSG;
    pos[0] = FIELD_GET(FMC2_BCHDSR1_EBP1, bchdsr1);
    pos[1] = FIELD_GET(FMC2_BCHDSR1_EBP2, bchdsr1);
    pos[2] = FIELD_GET(FMC2_BCHDSR2_EBP3, bchdsr2);
    pos[3] = FIELD_GET(FMC2_BCHDSR2_EBP4, bchdsr2);
    pos[4] = FIELD_GET(FMC2_BCHDSR3_EBP5, bchdsr3);
    pos[5] = FIELD_GET(FMC2_BCHDSR3_EBP6, bchdsr3);
    pos[6] = FIELD_GET(FMC2_BCHDSR4_EBP7, bchdsr4);
    pos[7] = FIELD_GET(FMC2_BCHDSR4_EBP8, bchdsr4);
    den = FIELD_GET(FMC2_BCHDSR0_DEN, bchdsr0);
    for (i = 0; i < den; i++) {
    if (pos[i] < eccsize * 8) {
    change_bit(pos[i], (unsigned long *)dat);
    nb_errs++;
    }
    }
    return nb_errs;
    }
    static int stm32_fmc2_nfc_bch_correct(struct nand_chip *chip, u8 *dat,
    u8 *read_ecc, u8 *calc_ecc)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    u32 ecc_sta[5];
// Wait until the decoding error is ready
    if (!wait_for_completion_timeout(&nfc.complete,
    msecs_to_jiffies(FMC2_TIMEOUT_MS))) {
    dev_err(nfc.dev, "bch timeout\n");
    stm32_fmc2_nfc_disable_bch_irq(nfc);
    return -ETIMEDOUT;
    }
    regmap_bulk_read(nfc.regmap, FMC2_BCHDSR0, ecc_sta, 5);
    stm32_fmc2_nfc_set_ecc(nfc, false);
    return stm32_fmc2_nfc_bch_decode(chip.ecc.size, dat, ecc_sta);
    }
    static int stm32_fmc2_nfc_read_page(struct nand_chip *chip, u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    int ret, i, s, stat, eccsize = chip.ecc.size;
    let mut eccbytes: c_int = chip.ecc.bytes;
    let mut eccsteps: c_int = chip.ecc.steps;
    let mut eccstrength: c_int = chip.ecc.strength;
    u8 *p = buf;
    u8 *ecc_calc = chip.ecc.calc_buf;
    u8 *ecc_code = chip.ecc.code_buf;
    let mut max_bitflips: c_uint = 0;
    ret = nand_read_page_op(chip, page, 0, core::ptr::null_mut(), 0);
    if (ret)
    return ret;
    for (i = mtd.writesize + FMC2_BBM_LEN, s = 0; s < eccsteps;
    s++, i += eccbytes, p += eccsize) {
    chip.ecc.hwctl(chip, NAND_ECC_READ);
// Read the nand page sector (512 bytes)
    ret = nand_change_read_column_op(chip, s * eccsize, p,
    eccsize, false);
    if (ret)
    return ret;
// Read the corresponding ECC bytes
    ret = nand_change_read_column_op(chip, i, ecc_code,
    eccbytes, false);
    if (ret)
    return ret;
// Correct the data
    stat = chip.ecc.correct(chip, p, ecc_code, ecc_calc);
    if (stat == -EBADMSG)
// Check for empty pages with bitflips
    stat = nand_check_erased_ecc_chunk(p, eccsize,
    ecc_code, eccbytes,
    core::ptr::null_mut(), 0,
    eccstrength);
    if (stat < 0) {
    mtd.ecc_stats.failed++;
    } else {
    mtd.ecc_stats.corrected += stat;
    max_bitflips = max_t(unsigned int, max_bitflips, stat);
    }
    }
// Read oob
    if (oob_required) {
    ret = nand_change_read_column_op(chip, mtd.writesize,
    chip.oob_poi, mtd.oobsize,
    false);
    if (ret)
    return ret;
    }
    return max_bitflips;
    }
// Sequencer read/write configuration
    static void stm32_fmc2_nfc_rw_page_init(struct nand_chip *chip, int page,
    int raw, bool write_data)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    struct mtd_info *mtd = nand_to_mtd(chip);
    let mut ecc_offset: u32 = mtd.writesize + FMC2_BBM_LEN;
//
// cfg[0] => csqcfgr1, cfg[1] => csqcfgr2, cfg[2] => csqcfgr3
// cfg[3] => csqar1, cfg[4] => csqar2
//
    u32 cfg[5];
    regmap_update_bits(nfc.regmap, FMC2_PCR, FMC2_PCR_WEN,
    write_data ? FMC2_PCR_WEN : 0);
//
// - Set Program Page/Page Read command
// - Enable DMA request data
// - Set timings
//
    cfg[0] = FMC2_CSQCFGR1_DMADEN | FMC2_CSQCFGR1_CMD1T;
    if (write_data)
    cfg[0] |= FIELD_PREP(FMC2_CSQCFGR1_CMD1, NAND_CMD_SEQIN);
    else
    cfg[0] |= FIELD_PREP(FMC2_CSQCFGR1_CMD1, NAND_CMD_READ0) |
    FMC2_CSQCFGR1_CMD2EN |
    FIELD_PREP(FMC2_CSQCFGR1_CMD2, NAND_CMD_READSTART) |
    FMC2_CSQCFGR1_CMD2T;
//
// - Set Random Data Input/Random Data Read command
// - Enable the sequencer to access the Spare data area
// - Enable  DMA request status decoding for read
// - Set timings
//
    if (write_data)
    cfg[1] = FIELD_PREP(FMC2_CSQCFGR2_RCMD1, NAND_CMD_RNDIN);
    else
    cfg[1] = FIELD_PREP(FMC2_CSQCFGR2_RCMD1, NAND_CMD_RNDOUT) |
    FMC2_CSQCFGR2_RCMD2EN |
    FIELD_PREP(FMC2_CSQCFGR2_RCMD2, NAND_CMD_RNDOUTSTART) |
    FMC2_CSQCFGR2_RCMD1T |
    FMC2_CSQCFGR2_RCMD2T;
    if (!raw) {
    cfg[1] |= write_data ? 0 : FMC2_CSQCFGR2_DMASEN;
    cfg[1] |= FMC2_CSQCFGR2_SQSDTEN;
    }
//
// - Set the number of sectors to be written
// - Set timings
//
    cfg[2] = FIELD_PREP(FMC2_CSQCFGR3_SNBR, chip.ecc.steps - 1);
    if (write_data) {
    cfg[2] |= FMC2_CSQCFGR3_RAC2T;
    if (chip.options & NAND_ROW_ADDR_3)
    cfg[2] |= FMC2_CSQCFGR3_AC5T;
    else
    cfg[2] |= FMC2_CSQCFGR3_AC4T;
    }
//
// Set the fourth first address cycles
// Byte 1 and byte 2 => column, we start at 0x0
// Byte 3 and byte 4 => page
//
    cfg[3] = FIELD_PREP(FMC2_CSQCAR1_ADDC3, page);
    cfg[3] |= FIELD_PREP(FMC2_CSQCAR1_ADDC4, page >> 8);
//
// - Set chip enable number
// - Set ECC byte offset in the spare area
// - Calculate the number of address cycles to be issued
// - Set byte 5 of address cycle if needed
//
    cfg[4] = FIELD_PREP(FMC2_CSQCAR2_NANDCEN, nfc.cs_sel);
    if (chip.options & NAND_BUSWIDTH_16)
    cfg[4] |= FIELD_PREP(FMC2_CSQCAR2_SAO, ecc_offset >> 1);
    else
    cfg[4] |= FIELD_PREP(FMC2_CSQCAR2_SAO, ecc_offset);
    if (chip.options & NAND_ROW_ADDR_3) {
    cfg[0] |= FIELD_PREP(FMC2_CSQCFGR1_ACYNBR, 5);
    cfg[4] |= FIELD_PREP(FMC2_CSQCAR2_ADDC5, page >> 16);
    } else {
    cfg[0] |= FIELD_PREP(FMC2_CSQCFGR1_ACYNBR, 4);
    }
    regmap_bulk_write(nfc.regmap, FMC2_CSQCFGR1, cfg, 5);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_dma_callback(arg: *mut c_void) {
    static void stm32_fmc2_nfc_dma_callback(void *arg)
    {
    complete((struct completion *)arg);
    }
// Read/write data from/to a page
    static int stm32_fmc2_nfc_xfer(struct nand_chip *chip, const u8 *buf,
    int raw, bool write_data)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    struct dma_async_tx_descriptor *desc_data, *desc_ecc;
    struct scatterlist *sg;
    struct dma_chan *dma_ch = nfc.dma_rx_ch;
    let mut dma_data_dir: enum dma_data_direction = DMA_FROM_DEVICE;
    let mut dma_transfer_dir: enum dma_transfer_direction = DMA_DEV_TO_MEM;
    let mut eccsteps: c_int = chip.ecc.steps;
    let mut eccsize: c_int = chip.ecc.size;
    let mut timeout: c_ulong = msecs_to_jiffies(FMC2_TIMEOUT_MS);
    const u8 *p = buf;
    int s, ret;
// Configure DMA data
    if (write_data) {
    dma_data_dir = DMA_TO_DEVICE;
    dma_transfer_dir = DMA_MEM_TO_DEV;
    dma_ch = nfc.dma_tx_ch;
    }
    for_each_sg(nfc.dma_data_sg.sgl, sg, eccsteps, s) {
    sg_set_buf(sg, p, eccsize);
    p += eccsize;
    }
    ret = dma_map_sg(nfc.dev, nfc.dma_data_sg.sgl,
    eccsteps, dma_data_dir);
    if (!ret)
    return -EIO;
    desc_data = dmaengine_prep_slave_sg(dma_ch, nfc.dma_data_sg.sgl,
    eccsteps, dma_transfer_dir,
    DMA_PREP_INTERRUPT);
    if (!desc_data) {
    ret = -ENOMEM;
    goto err_unmap_data;
    }
    reinit_completion(&nfc.dma_data_complete);
    reinit_completion(&nfc.complete);
    desc_data.callback = stm32_fmc2_nfc_dma_callback;
    desc_data.callback_param = &nfc.dma_data_complete;
    ret = dma_submit_error(dmaengine_submit(desc_data));
    if (ret)
    goto err_unmap_data;
    dma_async_issue_pending(dma_ch);
    if (!write_data && !raw) {
// Configure DMA ECC status
    for_each_sg(nfc.dma_ecc_sg.sgl, sg, eccsteps, s) {
    sg_dma_address(sg) = nfc.dma_ecc_addr +
    s * nfc.dma_ecc_len;
    sg_dma_len(sg) = nfc.dma_ecc_len;
    }
    desc_ecc = dmaengine_prep_slave_sg(nfc.dma_ecc_ch,
    nfc.dma_ecc_sg.sgl,
    eccsteps, dma_transfer_dir,
    DMA_PREP_INTERRUPT);
    if (!desc_ecc) {
    ret = -ENOMEM;
    goto err_unmap_data;
    }
    reinit_completion(&nfc.dma_ecc_complete);
    desc_ecc.callback = stm32_fmc2_nfc_dma_callback;
    desc_ecc.callback_param = &nfc.dma_ecc_complete;
    ret = dma_submit_error(dmaengine_submit(desc_ecc));
    if (ret)
    goto err_unmap_data;
    dma_async_issue_pending(nfc.dma_ecc_ch);
    }
    stm32_fmc2_nfc_clear_seq_irq(nfc);
    stm32_fmc2_nfc_enable_seq_irq(nfc);
// Start the transfer
    regmap_update_bits(nfc.regmap, FMC2_CSQCR,
    FMC2_CSQCR_CSQSTART, FMC2_CSQCR_CSQSTART);
// Wait end of sequencer transfer
    if (!wait_for_completion_timeout(&nfc.complete, timeout)) {
    dev_err(nfc.dev, "seq timeout\n");
    stm32_fmc2_nfc_disable_seq_irq(nfc);
    dmaengine_terminate_all(dma_ch);
    if (!write_data && !raw)
    dmaengine_terminate_all(nfc.dma_ecc_ch);
    ret = -ETIMEDOUT;
    goto err_unmap_data;
    }
// Wait DMA data transfer completion
    if (!wait_for_completion_timeout(&nfc.dma_data_complete, timeout)) {
    dev_err(nfc.dev, "data DMA timeout\n");
    dmaengine_terminate_all(dma_ch);
    ret = -ETIMEDOUT;
    }
// Wait DMA ECC transfer completion
    if (!write_data && !raw) {
    if (!wait_for_completion_timeout(&nfc.dma_ecc_complete,
    timeout)) {
    dev_err(nfc.dev, "ECC DMA timeout\n");
    dmaengine_terminate_all(nfc.dma_ecc_ch);
    ret = -ETIMEDOUT;
    }
    }
    err_unmap_data:
    dma_unmap_sg(nfc.dev, nfc.dma_data_sg.sgl, eccsteps, dma_data_dir);
    return ret;
    }
    static int stm32_fmc2_nfc_seq_write(struct nand_chip *chip, const u8 *buf,
    int oob_required, int page, int raw)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    int ret;
// Configure the sequencer
    stm32_fmc2_nfc_rw_page_init(chip, page, raw, true);
// Write the page
    ret = stm32_fmc2_nfc_xfer(chip, buf, raw, true);
    if (ret)
    return ret;
// Write oob
    if (oob_required) {
    let mut offset_in_page: c_uint = mtd.writesize;
    const void *buf = chip.oob_poi;
    let mut len: c_uint = mtd.oobsize;
    if (!raw) {
    struct mtd_oob_region oob_free;
    mtd_ooblayout_free(mtd, 0, &oob_free);
    offset_in_page += oob_free.offset;
    buf += oob_free.offset;
    len = oob_free.length;
    }
    ret = nand_change_write_column_op(chip, offset_in_page,
    buf, len, false);
    if (ret)
    return ret;
    }
    return nand_prog_page_end_op(chip);
    }
    static int stm32_fmc2_nfc_seq_write_page(struct nand_chip *chip, const u8 *buf,
    int oob_required, int page)
    {
    int ret;
    ret = stm32_fmc2_nfc_select_chip(chip, chip.cur_cs);
    if (ret)
    return ret;
    return stm32_fmc2_nfc_seq_write(chip, buf, oob_required, page, false);
    }
    static int stm32_fmc2_nfc_seq_write_page_raw(struct nand_chip *chip,
    const u8 *buf, int oob_required,
    int page)
    {
    int ret;
    ret = stm32_fmc2_nfc_select_chip(chip, chip.cur_cs);
    if (ret)
    return ret;
    return stm32_fmc2_nfc_seq_write(chip, buf, oob_required, page, true);
    }
// Get a status indicating which sectors have errors
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_get_mapping_status(nfc: *mut stm32_fmc2_nfc) -> u16 {
    static u16 stm32_fmc2_nfc_get_mapping_status(struct stm32_fmc2_nfc *nfc)
    {
    u32 csqemsr;
    regmap_read(nfc.regmap, FMC2_CSQEMSR, &csqemsr);
    return FIELD_GET(FMC2_CSQEMSR_SEM, csqemsr);
    }
    static int stm32_fmc2_nfc_seq_correct(struct nand_chip *chip, u8 *dat,
    u8 *read_ecc, u8 *calc_ecc)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    let mut eccbytes: c_int = chip.ecc.bytes;
    let mut eccsteps: c_int = chip.ecc.steps;
    let mut eccstrength: c_int = chip.ecc.strength;
    int i, s, eccsize = chip.ecc.size;
    u32 *ecc_sta = (u32 *)nfc.ecc_buf;
    let mut sta_map: u16 = stm32_fmc2_nfc_get_mapping_status(nfc);
    let mut max_bitflips: c_uint = 0;
    for (i = 0, s = 0; s < eccsteps; s++, i += eccbytes, dat += eccsize) {
    let mut stat: c_int = 0;
    if (eccstrength == FMC2_ECC_HAM) {
// Ecc_sta = FMC2_HECCR
    if (sta_map & BIT(s)) {
    stm32_fmc2_nfc_ham_set_ecc(*ecc_sta,
    &calc_ecc[i]);
    stat = stm32_fmc2_nfc_ham_correct(chip, dat,
    &read_ecc[i],
    &calc_ecc[i]);
    }
    ecc_sta++;
    } else {
//
// Ecc_sta[0] = FMC2_BCHDSR0
// Ecc_sta[1] = FMC2_BCHDSR1
// Ecc_sta[2] = FMC2_BCHDSR2
// Ecc_sta[3] = FMC2_BCHDSR3
// Ecc_sta[4] = FMC2_BCHDSR4
//
    if (sta_map & BIT(s))
    stat = stm32_fmc2_nfc_bch_decode(eccsize, dat,
    ecc_sta);
    ecc_sta += 5;
    }
    if (stat == -EBADMSG)
// Check for empty pages with bitflips
    stat = nand_check_erased_ecc_chunk(dat, eccsize,
    &read_ecc[i],
    eccbytes,
    core::ptr::null_mut(), 0,
    eccstrength);
    if (stat < 0) {
    mtd.ecc_stats.failed++;
    } else {
    mtd.ecc_stats.corrected += stat;
    max_bitflips = max_t(unsigned int, max_bitflips, stat);
    }
    }
    return max_bitflips;
    }
    static int stm32_fmc2_nfc_seq_read_page(struct nand_chip *chip, u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    u8 *ecc_calc = chip.ecc.calc_buf;
    u8 *ecc_code = chip.ecc.code_buf;
    u16 sta_map;
    int ret;
    ret = stm32_fmc2_nfc_select_chip(chip, chip.cur_cs);
    if (ret)
    return ret;
// Configure the sequencer
    stm32_fmc2_nfc_rw_page_init(chip, page, 0, false);
// Read the page
    ret = stm32_fmc2_nfc_xfer(chip, buf, 0, false);
    if (ret)
    return ret;
    sta_map = stm32_fmc2_nfc_get_mapping_status(nfc);
// Check if errors happen
    if (likely(!sta_map)) {
    if (oob_required)
    return nand_change_read_column_op(chip, mtd.writesize,
    chip.oob_poi,
    mtd.oobsize, false);
    return 0;
    }
// Read oob
    ret = nand_change_read_column_op(chip, mtd.writesize,
    chip.oob_poi, mtd.oobsize, false);
    if (ret)
    return ret;
    ret = mtd_ooblayout_get_eccbytes(mtd, ecc_code, chip.oob_poi, 0,
    chip.ecc.total);
    if (ret)
    return ret;
// Correct data
    return chip.ecc.correct(chip, buf, ecc_code, ecc_calc);
    }
    static int stm32_fmc2_nfc_seq_read_page_raw(struct nand_chip *chip, u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    int ret;
    ret = stm32_fmc2_nfc_select_chip(chip, chip.cur_cs);
    if (ret)
    return ret;
// Configure the sequencer
    stm32_fmc2_nfc_rw_page_init(chip, page, 1, false);
// Read the page
    ret = stm32_fmc2_nfc_xfer(chip, buf, 1, false);
    if (ret)
    return ret;
// Read oob
    if (oob_required)
    return nand_change_read_column_op(chip, mtd.writesize,
    chip.oob_poi, mtd.oobsize,
    false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_fmc2_nfc_irq(int irq, void *dev_id)
    {
    struct stm32_fmc2_nfc *nfc = (struct stm32_fmc2_nfc *)dev_id;
    if (nfc.irq_state == FMC2_IRQ_SEQ)
// Sequencer is used
    stm32_fmc2_nfc_disable_seq_irq(nfc);
#[no_mangle]
pub unsafe extern "C" fn if(FMC2_IRQ_BCH: nfc->irq_state ==) -> else {
    else if (nfc.irq_state == FMC2_IRQ_BCH)
// BCH is used
    stm32_fmc2_nfc_disable_bch_irq(nfc);
    complete(&nfc.complete);
    return IRQ_HANDLED;
    }
    static void stm32_fmc2_nfc_read_data(struct nand_chip *chip, void *buf,
    unsigned int len, bool force_8bit)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    void __iomem *io_addr_r = nfc.data_base[nfc.cs_sel];
    if (force_8bit && chip.options & NAND_BUSWIDTH_16)
// Reconfigure bus width to 8-bit
    stm32_fmc2_nfc_set_buswidth_16(nfc, false);
    if (!IS_ALIGNED((uintptr_t)buf, sizeof(u32))) {
    if (!IS_ALIGNED((uintptr_t)buf, sizeof(u16)) && len) {
// (u8 *)buf = readb_relaxed(io_addr_r);
    buf += sizeof(u8);
    len -= sizeof(u8);
    }
    if (!IS_ALIGNED((uintptr_t)buf, sizeof(u32)) &&
    len >= sizeof(u16)) {
// (u16 *)buf = readw_relaxed(io_addr_r);
    buf += sizeof(u16);
    len -= sizeof(u16);
    }
    }
// Buf is aligned
    while (len >= sizeof(u32)) {
// (u32 *)buf = readl_relaxed(io_addr_r);
    buf += sizeof(u32);
    len -= sizeof(u32);
    }
// Read remaining bytes
    if (len >= sizeof(u16)) {
// (u16 *)buf = readw_relaxed(io_addr_r);
    buf += sizeof(u16);
    len -= sizeof(u16);
    }
    if (len)
// (u8 *)buf = readb_relaxed(io_addr_r);
    if (force_8bit && chip.options & NAND_BUSWIDTH_16)
// Reconfigure bus width to 16-bit
    stm32_fmc2_nfc_set_buswidth_16(nfc, true);
    }
    static void stm32_fmc2_nfc_write_data(struct nand_chip *chip, const void *buf,
    unsigned int len, bool force_8bit)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    void __iomem *io_addr_w = nfc.data_base[nfc.cs_sel];
    if (force_8bit && chip.options & NAND_BUSWIDTH_16)
// Reconfigure bus width to 8-bit
    stm32_fmc2_nfc_set_buswidth_16(nfc, false);
    if (!IS_ALIGNED((uintptr_t)buf, sizeof(u32))) {
    if (!IS_ALIGNED((uintptr_t)buf, sizeof(u16)) && len) {
    writeb_relaxed(*(u8 *)buf, io_addr_w);
    buf += sizeof(u8);
    len -= sizeof(u8);
    }
    if (!IS_ALIGNED((uintptr_t)buf, sizeof(u32)) &&
    len >= sizeof(u16)) {
    writew_relaxed(*(u16 *)buf, io_addr_w);
    buf += sizeof(u16);
    len -= sizeof(u16);
    }
    }
// Buf is aligned
    while (len >= sizeof(u32)) {
    writel_relaxed(*(u32 *)buf, io_addr_w);
    buf += sizeof(u32);
    len -= sizeof(u32);
    }
// Write remaining bytes
    if (len >= sizeof(u16)) {
    writew_relaxed(*(u16 *)buf, io_addr_w);
    buf += sizeof(u16);
    len -= sizeof(u16);
    }
    if (len)
    writeb_relaxed(*(u8 *)buf, io_addr_w);
    if (force_8bit && chip.options & NAND_BUSWIDTH_16)
// Reconfigure bus width to 16-bit
    stm32_fmc2_nfc_set_buswidth_16(nfc, true);
    }
    static int stm32_fmc2_nfc_waitrdy(struct nand_chip *chip,
    unsigned long timeout_ms)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    const struct nand_sdr_timings *timings;
    u32 isr, sr;
// Check if there is no pending requests to the NAND flash
    if (regmap_read_poll_timeout(nfc.regmap, FMC2_SR, sr,
    sr & FMC2_SR_NWRF, 1,
    1000 * FMC2_TIMEOUT_MS))
    dev_warn(nfc.dev, "Waitrdy timeout\n");
// Wait tWB before R/B# signal is low
    timings = nand_get_sdr_timings(nand_get_interface_config(chip));
    ndelay(PSEC_TO_NSEC(timings.tWB_max));
// R/B# signal is low, clear high level flag
    regmap_write(nfc.regmap, FMC2_ICR, FMC2_ICR_CIHLF);
// Wait R/B# signal is high
    return regmap_read_poll_timeout(nfc.regmap, FMC2_ISR, isr,
    isr & FMC2_ISR_IHLF, 5,
    1000 * FMC2_TIMEOUT_MS);
    }
    static int stm32_fmc2_nfc_exec_op(struct nand_chip *chip,
    const struct nand_operation *op,
    bool check_only)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    const struct nand_op_instr *instr = core::ptr::null_mut();
    unsigned int op_id, i, timeout;
    int ret;
    if (check_only)
    return 0;
    ret = stm32_fmc2_nfc_select_chip(chip, op.cs);
    if (ret)
    return ret;
    for (op_id = 0; op_id < op.ninstrs; op_id++) {
    instr = &op.instrs[op_id];
    switch (instr.type) {
    case NAND_OP_CMD_INSTR:
    writeb_relaxed(instr.ctx.cmd.opcode,
    nfc.cmd_base[nfc.cs_sel]);
    break;
    case NAND_OP_ADDR_INSTR:
    for (i = 0; i < instr.ctx.addr.naddrs; i++)
    writeb_relaxed(instr.ctx.addr.addrs[i],
    nfc.addr_base[nfc.cs_sel]);
    break;
    case NAND_OP_DATA_IN_INSTR:
    stm32_fmc2_nfc_read_data(chip, instr.ctx.data.buf.in,
    instr.ctx.data.len,
    instr.ctx.data.force_8bit);
    break;
    case NAND_OP_DATA_OUT_INSTR:
    stm32_fmc2_nfc_write_data(chip, instr.ctx.data.buf.out,
    instr.ctx.data.len,
    instr.ctx.data.force_8bit);
    break;
    case NAND_OP_WAITRDY_INSTR:
    timeout = instr.ctx.waitrdy.timeout_ms;
    ret = stm32_fmc2_nfc_waitrdy(chip, timeout);
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_init(nfc: *mut stm32_fmc2_nfc) {
    static void stm32_fmc2_nfc_init(struct stm32_fmc2_nfc *nfc)
    {
    u32 pcr;
    regmap_read(nfc.regmap, FMC2_PCR, &pcr);
// Set CS used to undefined
    nfc.cs_sel = -1;
// Enable wait feature and nand flash memory bank
    pcr |= FMC2_PCR_PWAITEN;
    pcr |= FMC2_PCR_PBKEN;
// Set buswidth to 8 bits mode for identification
    pcr &= ~FMC2_PCR_PWID;
// ECC logic is disabled
    pcr &= ~FMC2_PCR_ECCEN;
// Default mode
    pcr &= ~FMC2_PCR_ECCALG;
    pcr &= ~FMC2_PCR_BCHECC;
    pcr &= ~FMC2_PCR_WEN;
// Set default ECC sector size
    pcr &= ~FMC2_PCR_ECCSS;
    pcr |= FIELD_PREP(FMC2_PCR_ECCSS, FMC2_PCR_ECCSS_2048);
// Set default tclr/tar timings
    pcr &= ~FMC2_PCR_TCLR;
    pcr |= FIELD_PREP(FMC2_PCR_TCLR, FMC2_PCR_TCLR_DEFAULT);
    pcr &= ~FMC2_PCR_TAR;
    pcr |= FIELD_PREP(FMC2_PCR_TAR, FMC2_PCR_TAR_DEFAULT);
// Enable FMC2 controller
    if (nfc.dev == nfc.cdev)
    regmap_update_bits(nfc.regmap, FMC2_BCR1,
    FMC2_BCR1_FMC2EN, FMC2_BCR1_FMC2EN);
    regmap_write(nfc.regmap, FMC2_PCR, pcr);
    regmap_write(nfc.regmap, FMC2_PMEM, FMC2_PMEM_DEFAULT);
    regmap_write(nfc.regmap, FMC2_PATT, FMC2_PATT_DEFAULT);
    }
    static void stm32_fmc2_nfc_calc_timings(struct nand_chip *chip,
    const struct nand_sdr_timings *sdrt)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    struct stm32_fmc2_nand *nand = to_fmc2_nand(chip);
    struct stm32_fmc2_timings *tims = &nand.timings;
    let mut hclk: c_ulong = clk_get_rate(nfc.clk);
    let mut hclkp: c_ulong = NSEC_PER_SEC / (hclk / 1000);
    unsigned long timing, tar, tclr, thiz, twait;
    unsigned long tset_mem, tset_att, thold_mem, thold_att;
    tar = max_t(unsigned long, hclkp, sdrt.tAR_min);
    timing = DIV_ROUND_UP(tar, hclkp) - 1;
    tims.tar = min_t(unsigned long, timing, FMC2_PCR_TIMING_MASK);
    tclr = max_t(unsigned long, hclkp, sdrt.tCLR_min);
    timing = DIV_ROUND_UP(tclr, hclkp) - 1;
    tims.tclr = min_t(unsigned long, timing, FMC2_PCR_TIMING_MASK);
    tims.thiz = FMC2_THIZ;
    thiz = (tims.thiz + 1) * hclkp;
//
// tWAIT > tRP
// tWAIT > tWP
// tWAIT > tREA + tIO
//
    twait = max_t(unsigned long, hclkp, sdrt.tRP_min);
    twait = max_t(unsigned long, twait, sdrt.tWP_min);
    twait = max_t(unsigned long, twait, sdrt.tREA_max + FMC2_TIO);
    timing = DIV_ROUND_UP(twait, hclkp);
    tims.twait = clamp_val(timing, 1, FMC2_PMEM_PATT_TIMING_MASK);
//
// tSETUP_MEM > tCS - tWAIT
// tSETUP_MEM > tALS - tWAIT
// tSETUP_MEM > tDS - (tWAIT - tHIZ)
//
    tset_mem = hclkp;
    if (sdrt.tCS_min > twait && (tset_mem < sdrt.tCS_min - twait))
    tset_mem = sdrt.tCS_min - twait;
    if (sdrt.tALS_min > twait && (tset_mem < sdrt.tALS_min - twait))
    tset_mem = sdrt.tALS_min - twait;
    if (twait > thiz && (sdrt.tDS_min > twait - thiz) &&
    (tset_mem < sdrt.tDS_min - (twait - thiz)))
    tset_mem = sdrt.tDS_min - (twait - thiz);
    timing = DIV_ROUND_UP(tset_mem, hclkp);
    tims.tset_mem = clamp_val(timing, 1, FMC2_PMEM_PATT_TIMING_MASK);
//
// tHOLD_MEM > tCH
// tHOLD_MEM > tREH - tSETUP_MEM
// tHOLD_MEM > max(tRC, tWC) - (tSETUP_MEM + tWAIT)
//
    thold_mem = max_t(unsigned long, hclkp, sdrt.tCH_min);
    if (sdrt.tREH_min > tset_mem &&
    (thold_mem < sdrt.tREH_min - tset_mem))
    thold_mem = sdrt.tREH_min - tset_mem;
    if ((sdrt.tRC_min > tset_mem + twait) &&
    (thold_mem < sdrt.tRC_min - (tset_mem + twait)))
    thold_mem = sdrt.tRC_min - (tset_mem + twait);
    if ((sdrt.tWC_min > tset_mem + twait) &&
    (thold_mem < sdrt.tWC_min - (tset_mem + twait)))
    thold_mem = sdrt.tWC_min - (tset_mem + twait);
    timing = DIV_ROUND_UP(thold_mem, hclkp);
    tims.thold_mem = clamp_val(timing, 1, FMC2_PMEM_PATT_TIMING_MASK);
//
// tSETUP_ATT > tCS - tWAIT
// tSETUP_ATT > tCLS - tWAIT
// tSETUP_ATT > tALS - tWAIT
// tSETUP_ATT > tRHW - tHOLD_MEM
// tSETUP_ATT > tDS - (tWAIT - tHIZ)
//
    tset_att = hclkp;
    if (sdrt.tCS_min > twait && (tset_att < sdrt.tCS_min - twait))
    tset_att = sdrt.tCS_min - twait;
    if (sdrt.tCLS_min > twait && (tset_att < sdrt.tCLS_min - twait))
    tset_att = sdrt.tCLS_min - twait;
    if (sdrt.tALS_min > twait && (tset_att < sdrt.tALS_min - twait))
    tset_att = sdrt.tALS_min - twait;
    if (sdrt.tRHW_min > thold_mem &&
    (tset_att < sdrt.tRHW_min - thold_mem))
    tset_att = sdrt.tRHW_min - thold_mem;
    if (twait > thiz && (sdrt.tDS_min > twait - thiz) &&
    (tset_att < sdrt.tDS_min - (twait - thiz)))
    tset_att = sdrt.tDS_min - (twait - thiz);
    timing = DIV_ROUND_UP(tset_att, hclkp);
    tims.tset_att = clamp_val(timing, 1, FMC2_PMEM_PATT_TIMING_MASK);
//
// tHOLD_ATT > tALH
// tHOLD_ATT > tCH
// tHOLD_ATT > tCLH
// tHOLD_ATT > tCOH
// tHOLD_ATT > tDH
// tHOLD_ATT > tWB + tIO + tSYNC - tSETUP_MEM
// tHOLD_ATT > tADL - tSETUP_MEM
// tHOLD_ATT > tWH - tSETUP_MEM
// tHOLD_ATT > tWHR - tSETUP_MEM
// tHOLD_ATT > tRC - (tSETUP_ATT + tWAIT)
// tHOLD_ATT > tWC - (tSETUP_ATT + tWAIT)
//
    thold_att = max_t(unsigned long, hclkp, sdrt.tALH_min);
    thold_att = max_t(unsigned long, thold_att, sdrt.tCH_min);
    thold_att = max_t(unsigned long, thold_att, sdrt.tCLH_min);
    thold_att = max_t(unsigned long, thold_att, sdrt.tCOH_min);
    thold_att = max_t(unsigned long, thold_att, sdrt.tDH_min);
    if ((sdrt.tWB_max + FMC2_TIO + FMC2_TSYNC > tset_mem) &&
    (thold_att < sdrt.tWB_max + FMC2_TIO + FMC2_TSYNC - tset_mem))
    thold_att = sdrt.tWB_max + FMC2_TIO + FMC2_TSYNC - tset_mem;
    if (sdrt.tADL_min > tset_mem &&
    (thold_att < sdrt.tADL_min - tset_mem))
    thold_att = sdrt.tADL_min - tset_mem;
    if (sdrt.tWH_min > tset_mem &&
    (thold_att < sdrt.tWH_min - tset_mem))
    thold_att = sdrt.tWH_min - tset_mem;
    if (sdrt.tWHR_min > tset_mem &&
    (thold_att < sdrt.tWHR_min - tset_mem))
    thold_att = sdrt.tWHR_min - tset_mem;
    if ((sdrt.tRC_min > tset_att + twait) &&
    (thold_att < sdrt.tRC_min - (tset_att + twait)))
    thold_att = sdrt.tRC_min - (tset_att + twait);
    if ((sdrt.tWC_min > tset_att + twait) &&
    (thold_att < sdrt.tWC_min - (tset_att + twait)))
    thold_att = sdrt.tWC_min - (tset_att + twait);
    timing = DIV_ROUND_UP(thold_att, hclkp);
    tims.thold_att = clamp_val(timing, 1, FMC2_PMEM_PATT_TIMING_MASK);
    }
    static int stm32_fmc2_nfc_setup_interface(struct nand_chip *chip, int chipnr,
    const struct nand_interface_config *conf)
    {
    const struct nand_sdr_timings *sdrt;
    sdrt = nand_get_sdr_timings(conf);
    if (IS_ERR(sdrt))
    return PTR_ERR(sdrt);
    if (conf.timings.mode > 3)
    return -EOPNOTSUPP;
    if (chipnr == NAND_DATA_IFACE_CHECK_ONLY)
    return 0;
    stm32_fmc2_nfc_calc_timings(chip, sdrt);
    stm32_fmc2_nfc_timings_init(chip);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_dma_setup(nfc: *mut stm32_fmc2_nfc) -> c_int {
    static int stm32_fmc2_nfc_dma_setup(struct stm32_fmc2_nfc *nfc)
    {
    struct dma_slave_caps caps;
    let mut ret: c_int = 0;
    nfc.dma_tx_ch = dma_request_chan(nfc.dev, "tx");
    if (IS_ERR(nfc.dma_tx_ch)) {
    ret = PTR_ERR(nfc.dma_tx_ch);
    if (ret != -ENODEV && ret != -EPROBE_DEFER)
    dev_err(nfc.dev,
    "failed to request tx DMA channel: %d\n", ret);
    nfc.dma_tx_ch = core::ptr::null_mut();
    goto err_dma;
    }
    ret = dma_get_slave_caps(nfc.dma_tx_ch, &caps);
    if (ret)
    return ret;
    nfc.tx_dma_max_burst = caps.max_burst;
    nfc.dma_rx_ch = dma_request_chan(nfc.dev, "rx");
    if (IS_ERR(nfc.dma_rx_ch)) {
    ret = PTR_ERR(nfc.dma_rx_ch);
    if (ret != -ENODEV && ret != -EPROBE_DEFER)
    dev_err(nfc.dev,
    "failed to request rx DMA channel: %d\n", ret);
    nfc.dma_rx_ch = core::ptr::null_mut();
    goto err_dma;
    }
    ret = dma_get_slave_caps(nfc.dma_rx_ch, &caps);
    if (ret)
    return ret;
    nfc.rx_dma_max_burst = caps.max_burst;
    nfc.dma_ecc_ch = dma_request_chan(nfc.dev, "ecc");
    if (IS_ERR(nfc.dma_ecc_ch)) {
    ret = PTR_ERR(nfc.dma_ecc_ch);
    if (ret != -ENODEV && ret != -EPROBE_DEFER)
    dev_err(nfc.dev,
    "failed to request ecc DMA channel: %d\n", ret);
    nfc.dma_ecc_ch = core::ptr::null_mut();
    goto err_dma;
    }
    ret = sg_alloc_table(&nfc.dma_ecc_sg, FMC2_MAX_SG, GFP_KERNEL);
    if (ret)
    return ret;
// Allocate a buffer to store ECC status registers
    nfc.ecc_buf = dmam_alloc_coherent(nfc.dev, FMC2_MAX_ECC_BUF_LEN,
    &nfc.dma_ecc_addr, GFP_KERNEL);
    if (!nfc.ecc_buf)
    return -ENOMEM;
    ret = sg_alloc_table(&nfc.dma_data_sg, FMC2_MAX_SG, GFP_KERNEL);
    if (ret)
    return ret;
    init_completion(&nfc.dma_data_complete);
    init_completion(&nfc.dma_ecc_complete);
    return 0;
    err_dma:
    if (ret == -ENODEV) {
    dev_warn(nfc.dev,
    "DMAs not defined in the DT, polling mode is used\n");
    ret = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_nand_callbacks_setup(chip: *mut nand_chip) {
    static void stm32_fmc2_nfc_nand_callbacks_setup(struct nand_chip *chip)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
//
// Specific callbacks to read/write a page depending on
// the mode (polling/sequencer) and the algo used (Hamming, BCH).
//
    if (nfc.dma_tx_ch && nfc.dma_rx_ch && nfc.dma_ecc_ch) {
// DMA => use sequencer mode callbacks
    chip.ecc.correct = stm32_fmc2_nfc_seq_correct;
    chip.ecc.write_page = stm32_fmc2_nfc_seq_write_page;
    chip.ecc.read_page = stm32_fmc2_nfc_seq_read_page;
    chip.ecc.write_page_raw = stm32_fmc2_nfc_seq_write_page_raw;
    chip.ecc.read_page_raw = stm32_fmc2_nfc_seq_read_page_raw;
    } else {
// No DMA => use polling mode callbacks
    chip.ecc.hwctl = stm32_fmc2_nfc_hwctl;
    if (chip.ecc.strength == FMC2_ECC_HAM) {
// Hamming is used
    chip.ecc.calculate = stm32_fmc2_nfc_ham_calculate;
    chip.ecc.correct = stm32_fmc2_nfc_ham_correct;
    chip.ecc.options |= NAND_ECC_GENERIC_ERASED_CHECK;
    } else {
// BCH is used
    chip.ecc.calculate = stm32_fmc2_nfc_bch_calculate;
    chip.ecc.correct = stm32_fmc2_nfc_bch_correct;
    chip.ecc.read_page = stm32_fmc2_nfc_read_page;
    }
    }
// Specific configurations depending on the algo used
    if (chip.ecc.strength == FMC2_ECC_HAM)
    chip.ecc.bytes = chip.options & NAND_BUSWIDTH_16 ? 4 : 3;
#[no_mangle]
pub unsafe extern "C" fn if(FMC2_ECC_BCH8: chip->ecc.strength ==) -> else {
    else if (chip.ecc.strength == FMC2_ECC_BCH8)
    chip.ecc.bytes = chip.options & NAND_BUSWIDTH_16 ? 14 : 13;
    else
    chip.ecc.bytes = chip.options & NAND_BUSWIDTH_16 ? 8 : 7;
    }
    static int stm32_fmc2_nfc_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    struct nand_ecc_ctrl *ecc = &chip.ecc;
    if (section)
    return -ERANGE;
    oobregion.length = ecc.total;
    oobregion.offset = FMC2_BBM_LEN;
    return 0;
    }
    static int stm32_fmc2_nfc_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    struct nand_ecc_ctrl *ecc = &chip.ecc;
    if (section)
    return -ERANGE;
    oobregion.length = mtd.oobsize - ecc.total - FMC2_BBM_LEN;
    oobregion.offset = ecc.total + FMC2_BBM_LEN;
    return 0;
    }
    static const struct mtd_ooblayout_ops stm32_fmc2_nfc_ooblayout_ops = {
    .ecc = stm32_fmc2_nfc_ooblayout_ecc,
    .free = stm32_fmc2_nfc_ooblayout_free,
    };
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_calc_ecc_bytes(step_size: c_int, strength: c_int) -> c_int {
    static int stm32_fmc2_nfc_calc_ecc_bytes(int step_size, int strength)
    {
// Hamming
    if (strength == FMC2_ECC_HAM)
    return 4;
// BCH8
    if (strength == FMC2_ECC_BCH8)
    return 14;
// BCH4
    return 8;
    }
    NAND_ECC_CAPS_SINGLE(stm32_fmc2_nfc_ecc_caps, stm32_fmc2_nfc_calc_ecc_bytes,
    FMC2_ECC_STEP_SIZE,
    FMC2_ECC_HAM, FMC2_ECC_BCH4, FMC2_ECC_BCH8);
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_attach_chip(chip: *mut nand_chip) -> c_int {
    static int stm32_fmc2_nfc_attach_chip(struct nand_chip *chip)
    {
    struct stm32_fmc2_nfc *nfc = to_stm32_nfc(chip.controller);
    struct mtd_info *mtd = nand_to_mtd(chip);
    int ret;
//
// Only NAND_ECC_ENGINE_TYPE_ON_HOST mode is actually supported
// Hamming => ecc.strength = 1
// BCH4 => ecc.strength = 4
// BCH8 => ecc.strength = 8
// ECC sector size = 512
//
    if (chip.ecc.engine_type != NAND_ECC_ENGINE_TYPE_ON_HOST) {
    dev_err(nfc.dev,
    "nand_ecc_engine_type is not well defined in the DT\n");
    return -EINVAL;
    }
// Default ECC settings in case they are not set in the device tree
    if (!chip.ecc.size)
    chip.ecc.size = FMC2_ECC_STEP_SIZE;
    if (!chip.ecc.strength)
    chip.ecc.strength = FMC2_ECC_BCH8;
    ret = nand_ecc_choose_conf(chip, &stm32_fmc2_nfc_ecc_caps,
    mtd.oobsize - FMC2_BBM_LEN);
    if (ret) {
    dev_err(nfc.dev, "no valid ECC settings set\n");
    return ret;
    }
    if (mtd.writesize / chip.ecc.size > FMC2_MAX_SG) {
    dev_err(nfc.dev, "nand page size is not supported\n");
    return -EINVAL;
    }
    if (chip.bbt_options & NAND_BBT_USE_FLASH)
    chip.bbt_options |= NAND_BBT_NO_OOB;
    stm32_fmc2_nfc_nand_callbacks_setup(chip);
    mtd_set_ooblayout(mtd, &stm32_fmc2_nfc_ooblayout_ops);
    stm32_fmc2_nfc_setup(chip);
    return 0;
    }
    static const struct nand_controller_ops stm32_fmc2_nfc_controller_ops = {
    .attach_chip = stm32_fmc2_nfc_attach_chip,
    .exec_op = stm32_fmc2_nfc_exec_op,
    .setup_interface = stm32_fmc2_nfc_setup_interface,
    };
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_wp_enable(nand: *mut stm32_fmc2_nand) {
    static void stm32_fmc2_nfc_wp_enable(struct stm32_fmc2_nand *nand)
    {
    if (nand.wp_gpio)
    gpiod_set_value(nand.wp_gpio, 1);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_wp_disable(nand: *mut stm32_fmc2_nand) {
    static void stm32_fmc2_nfc_wp_disable(struct stm32_fmc2_nand *nand)
    {
    if (nand.wp_gpio)
    gpiod_set_value(nand.wp_gpio, 0);
    }
    static int stm32_fmc2_nfc_parse_child(struct stm32_fmc2_nfc *nfc,
    struct device_node *dn)
    {
    struct stm32_fmc2_nand *nand = &nfc.nand;
    u32 cs;
    int ret, i;
    if (!of_get_property(dn, "reg", &nand.ncs))
    return -EINVAL;
    nand.ncs /= sizeof(u32);
    if (!nand.ncs) {
    dev_err(nfc.dev, "invalid reg property size\n");
    return -EINVAL;
    }
    for (i = 0; i < nand.ncs; i++) {
    ret = of_property_read_u32_index(dn, "reg", i, &cs);
    if (ret) {
    dev_err(nfc.dev, "could not retrieve reg property: %d\n",
    ret);
    return ret;
    }
    if (cs >= nfc.data.max_ncs) {
    dev_err(nfc.dev, "invalid reg value: %d\n", cs);
    return -EINVAL;
    }
    if (nfc.cs_assigned & BIT(cs)) {
    dev_err(nfc.dev, "cs already assigned: %d\n", cs);
    return -EINVAL;
    }
    nfc.cs_assigned |= BIT(cs);
    nand.cs_used[i] = cs;
    }
    nand.wp_gpio = devm_fwnode_gpiod_get(nfc.dev, of_fwnode_handle(dn),
    "wp", GPIOD_OUT_HIGH, "wp");
    if (IS_ERR(nand.wp_gpio)) {
    ret = PTR_ERR(nand.wp_gpio);
    if (ret != -ENOENT)
    return dev_err_probe(nfc.dev, ret,
    "failed to request WP GPIO\n");
    nand.wp_gpio = core::ptr::null_mut();
    }
    nand_set_flash_node(&nand.chip, dn);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_parse_dt(nfc: *mut stm32_fmc2_nfc) -> c_int {
    static int stm32_fmc2_nfc_parse_dt(struct stm32_fmc2_nfc *nfc)
    {
    struct device_node *dn = nfc.dev.of_node;
    let mut nchips: c_int = of_get_child_count(dn);
    let mut ret: c_int = 0;
    if (!nchips) {
    dev_err(nfc.dev, "NAND chip not defined\n");
    return -EINVAL;
    }
    if (nchips > 1) {
    dev_err(nfc.dev, "too many NAND chips defined\n");
    return -EINVAL;
    }
    for_each_child_of_node_scoped(dn, child) {
    ret = stm32_fmc2_nfc_parse_child(nfc, child);
    if (ret < 0)
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_set_cdev(nfc: *mut stm32_fmc2_nfc) -> c_int {
    static int stm32_fmc2_nfc_set_cdev(struct stm32_fmc2_nfc *nfc)
    {
    struct device *dev = nfc.dev;
    let mut ebi_found: bool = false;
    if (dev.parent && of_device_is_compatible(dev.parent.of_node,
    "st,stm32mp1-fmc2-ebi"))
    ebi_found = true;
    if (of_device_is_compatible(dev.of_node, "st,stm32mp1-fmc2-nfc")) {
    if (ebi_found) {
    nfc.cdev = dev.parent;
    return 0;
    }
    return -EINVAL;
    }
    if (ebi_found)
    return -EINVAL;
    nfc.cdev = dev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_fmc2_nfc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct reset_control *rstc;
    struct stm32_fmc2_nfc *nfc;
    struct stm32_fmc2_nand *nand;
    struct resource *res;
    struct mtd_info *mtd;
    struct nand_chip *chip;
    struct resource cres;
    int chip_cs, mem_region, ret, irq;
    let mut start_region: c_int = 0;
    nfc = devm_kzalloc(dev, sizeof(*nfc), GFP_KERNEL);
    if (!nfc)
    return -ENOMEM;
    nfc.dev = dev;
    nand_controller_init(&nfc.base);
    nfc.base.ops = &stm32_fmc2_nfc_controller_ops;
    nfc.data = of_device_get_match_data(dev);
    if (!nfc.data)
    return -EINVAL;
    if (nfc.data.set_cdev) {
    ret = nfc.data.set_cdev(nfc);
    if (ret)
    return ret;
    } else {
    nfc.cdev = dev.parent;
    }
    ret = stm32_fmc2_nfc_parse_dt(nfc);
    if (ret)
    return ret;
    ret = of_address_to_resource(nfc.cdev.of_node, 0, &cres);
    if (ret)
    return ret;
    nfc.io_phys_addr = cres.start;
    nfc.regmap = device_node_to_regmap(nfc.cdev.of_node);
    if (IS_ERR(nfc.regmap))
    return PTR_ERR(nfc.regmap);
    if (nfc.dev == nfc.cdev)
    start_region = 1;
    for (chip_cs = 0, mem_region = start_region; chip_cs < nfc.data.max_ncs;
    chip_cs++, mem_region += 3) {
    if (!(nfc.cs_assigned & BIT(chip_cs)))
    continue;
    nfc.data_base[chip_cs] = devm_platform_get_and_ioremap_resource(pdev,
    mem_region, &res);
    if (IS_ERR(nfc.data_base[chip_cs]))
    return PTR_ERR(nfc.data_base[chip_cs]);
    nfc.data_phys_addr[chip_cs] = res.start;
    nfc.cmd_base[chip_cs] = devm_platform_ioremap_resource(pdev, mem_region + 1);
    if (IS_ERR(nfc.cmd_base[chip_cs]))
    return PTR_ERR(nfc.cmd_base[chip_cs]);
    nfc.addr_base[chip_cs] = devm_platform_ioremap_resource(pdev, mem_region + 2);
    if (IS_ERR(nfc.addr_base[chip_cs]))
    return PTR_ERR(nfc.addr_base[chip_cs]);
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, stm32_fmc2_nfc_irq, 0,
    dev_name(dev), nfc);
    if (ret) {
    dev_err(dev, "failed to request irq\n");
    return ret;
    }
    init_completion(&nfc.complete);
    nfc.clk = devm_clk_get_enabled(nfc.cdev, core::ptr::null_mut());
    if (IS_ERR(nfc.clk)) {
    dev_err(dev, "can not get and enable the clock\n");
    return PTR_ERR(nfc.clk);
    }
    rstc = devm_reset_control_get(dev, core::ptr::null_mut());
    if (IS_ERR(rstc)) {
    ret = PTR_ERR(rstc);
    if (ret == -EPROBE_DEFER)
    return ret;
    } else {
    reset_control_assert(rstc);
    reset_control_deassert(rstc);
    }
    ret = stm32_fmc2_nfc_dma_setup(nfc);
    if (ret)
    goto err_release_dma;
    stm32_fmc2_nfc_init(nfc);
    nand = &nfc.nand;
    chip = &nand.chip;
    mtd = nand_to_mtd(chip);
    mtd.dev.parent = dev;
    chip.controller = &nfc.base;
    chip.options |= NAND_BUSWIDTH_AUTO | NAND_NO_SUBPAGE_WRITE |
    NAND_USES_DMA;
    stm32_fmc2_nfc_wp_disable(nand);
// Scan to find existence of the device
    ret = nand_scan(chip, nand.ncs);
    if (ret)
    goto err_wp_enable;
    ret = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    if (ret)
    goto err_nand_cleanup;
    platform_set_drvdata(pdev, nfc);
    return 0;
    err_nand_cleanup:
    nand_cleanup(chip);
    err_wp_enable:
    stm32_fmc2_nfc_wp_enable(nand);
    err_release_dma:
    if (nfc.dma_ecc_ch)
    dma_release_channel(nfc.dma_ecc_ch);
    if (nfc.dma_tx_ch)
    dma_release_channel(nfc.dma_tx_ch);
    if (nfc.dma_rx_ch)
    dma_release_channel(nfc.dma_rx_ch);
    sg_free_table(&nfc.dma_data_sg);
    sg_free_table(&nfc.dma_ecc_sg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_remove(pdev: *mut platform_device) {
    static void stm32_fmc2_nfc_remove(struct platform_device *pdev)
    {
    struct stm32_fmc2_nfc *nfc = platform_get_drvdata(pdev);
    struct stm32_fmc2_nand *nand = &nfc.nand;
    struct nand_chip *chip = &nand.chip;
    int ret;
    ret = mtd_device_unregister(nand_to_mtd(chip));
    WARN_ON(ret);
    nand_cleanup(chip);
    if (nfc.dma_ecc_ch)
    dma_release_channel(nfc.dma_ecc_ch);
    if (nfc.dma_tx_ch)
    dma_release_channel(nfc.dma_tx_ch);
    if (nfc.dma_rx_ch)
    dma_release_channel(nfc.dma_rx_ch);
    sg_free_table(&nfc.dma_data_sg);
    sg_free_table(&nfc.dma_ecc_sg);
    stm32_fmc2_nfc_wp_enable(nand);
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stm32_fmc2_nfc_suspend(struct device *dev)
    {
    struct stm32_fmc2_nfc *nfc = dev_get_drvdata(dev);
    struct stm32_fmc2_nand *nand = &nfc.nand;
    clk_disable_unprepare(nfc.clk);
    stm32_fmc2_nfc_wp_enable(nand);
    pinctrl_pm_select_sleep_state(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_fmc2_nfc_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stm32_fmc2_nfc_resume(struct device *dev)
    {
    struct stm32_fmc2_nfc *nfc = dev_get_drvdata(dev);
    struct stm32_fmc2_nand *nand = &nfc.nand;
    int chip_cs, ret;
    pinctrl_pm_select_default_state(dev);
    ret = clk_prepare_enable(nfc.clk);
    if (ret) {
    dev_err(dev, "can not enable the clock\n");
    return ret;
    }
    stm32_fmc2_nfc_init(nfc);
    stm32_fmc2_nfc_wp_disable(nand);
    for (chip_cs = 0; chip_cs < nfc.data.max_ncs; chip_cs++) {
    if (!(nfc.cs_assigned & BIT(chip_cs)))
    continue;
    nand_reset(&nand.chip, chip_cs);
    }
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(stm32_fmc2_nfc_pm_ops, stm32_fmc2_nfc_suspend,
    stm32_fmc2_nfc_resume);
    static const struct stm32_fmc2_nfc_data stm32_fmc2_nfc_mp1_data = {
    .max_ncs = 2,
    .set_cdev = stm32_fmc2_nfc_set_cdev,
    };
    static const struct stm32_fmc2_nfc_data stm32_fmc2_nfc_mp25_data = {
    .max_ncs = 4,
    };
    static const struct of_device_id stm32_fmc2_nfc_match[] = {
    {
    .compatible = "st,stm32mp15-fmc2",
    .data = &stm32_fmc2_nfc_mp1_data,
    },
    {
    .compatible = "st,stm32mp1-fmc2-nfc",
    .data = &stm32_fmc2_nfc_mp1_data,
    },
    {
    .compatible = "st,stm32mp25-fmc2-nfc",
    .data = &stm32_fmc2_nfc_mp25_data,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, stm32_fmc2_nfc_match);
    static struct platform_driver stm32_fmc2_nfc_driver = {
    .probe	= stm32_fmc2_nfc_probe,
    .remove = stm32_fmc2_nfc_remove,
    .driver	= {
    .name = "stm32_fmc2_nfc",
    .of_match_table = stm32_fmc2_nfc_match,
    .pm = &stm32_fmc2_nfc_pm_ops,
    },
    };
    module_platform_driver(stm32_fmc2_nfc_driver);
    MODULE_AUTHOR("Christophe Kerello <christophe.kerello@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STM32 FMC2 NFC driver");
    MODULE_LICENSE("GPL v2");
