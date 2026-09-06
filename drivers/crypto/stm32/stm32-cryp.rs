//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/stm32/stm32-cryp.c
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
// Copyright (C) STMicroelectronics SA 2017
// Author: Fabien Dessenne <fabien.dessenne@st.com>
// Ux500 support taken from snippets in the old Ux500 cryp driver
//

// Bit [0] encrypt / decrypt

// Bit [8..1] algo & operation mode

// Mode mask = bits [15..0]

// Bit [31..16] status

// Registers
pub const CRYP_CR: c_uint = 0x00000000;
pub const CRYP_SR: c_uint = 0x00000004;
pub const CRYP_DIN: c_uint = 0x00000008;
pub const CRYP_DOUT: c_uint = 0x0000000C;
pub const CRYP_DMACR: c_uint = 0x00000010;
pub const CRYP_IMSCR: c_uint = 0x00000014;
pub const CRYP_RISR: c_uint = 0x00000018;
pub const CRYP_MISR: c_uint = 0x0000001C;
pub const CRYP_K0LR: c_uint = 0x00000020;
pub const CRYP_K0RR: c_uint = 0x00000024;
pub const CRYP_K1LR: c_uint = 0x00000028;
pub const CRYP_K1RR: c_uint = 0x0000002C;
pub const CRYP_K2LR: c_uint = 0x00000030;
pub const CRYP_K2RR: c_uint = 0x00000034;
pub const CRYP_K3LR: c_uint = 0x00000038;
pub const CRYP_K3RR: c_uint = 0x0000003C;
pub const CRYP_IV0LR: c_uint = 0x00000040;
pub const CRYP_IV0RR: c_uint = 0x00000044;
pub const CRYP_IV1LR: c_uint = 0x00000048;
pub const CRYP_IV1RR: c_uint = 0x0000004C;
pub const CRYP_CSGCMCCM0R: c_uint = 0x00000050;
pub const CRYP_CSGCM0R: c_uint = 0x00000070;
pub const UX500_CRYP_CR: c_uint = 0x00000000;
pub const UX500_CRYP_SR: c_uint = 0x00000004;
pub const UX500_CRYP_DIN: c_uint = 0x00000008;
pub const UX500_CRYP_DINSIZE: c_uint = 0x0000000C;
pub const UX500_CRYP_DOUT: c_uint = 0x00000010;
pub const UX500_CRYP_DOUSIZE: c_uint = 0x00000014;
pub const UX500_CRYP_DMACR: c_uint = 0x00000018;
pub const UX500_CRYP_IMSC: c_uint = 0x0000001C;
pub const UX500_CRYP_RIS: c_uint = 0x00000020;
pub const UX500_CRYP_MIS: c_uint = 0x00000024;
pub const UX500_CRYP_K1L: c_uint = 0x00000028;
pub const UX500_CRYP_K1R: c_uint = 0x0000002C;
pub const UX500_CRYP_K2L: c_uint = 0x00000030;
pub const UX500_CRYP_K2R: c_uint = 0x00000034;
pub const UX500_CRYP_K3L: c_uint = 0x00000038;
pub const UX500_CRYP_K3R: c_uint = 0x0000003C;
pub const UX500_CRYP_K4L: c_uint = 0x00000040;
pub const UX500_CRYP_K4R: c_uint = 0x00000044;
pub const UX500_CRYP_IV0L: c_uint = 0x00000048;
pub const UX500_CRYP_IV0R: c_uint = 0x0000004C;
pub const UX500_CRYP_IV1L: c_uint = 0x00000050;
pub const UX500_CRYP_IV1R: c_uint = 0x00000054;
// Registers values
pub const CR_DEC_NOT_ENC: c_uint = 0x00000004;
pub const CR_TDES_ECB: c_uint = 0x00000000;
pub const CR_TDES_CBC: c_uint = 0x00000008;
pub const CR_DES_ECB: c_uint = 0x00000010;
pub const CR_DES_CBC: c_uint = 0x00000018;
pub const CR_AES_ECB: c_uint = 0x00000020;
pub const CR_AES_CBC: c_uint = 0x00000028;
pub const CR_AES_CTR: c_uint = 0x00000030;
pub const CR_AES_KP: c_uint = 0x00000038 /* Not on Ux500 */;
pub const CR_AES_XTS: c_uint = 0x00000038 /* Only on Ux500 */;
pub const CR_AES_GCM: c_uint = 0x00080000;
pub const CR_AES_CCM: c_uint = 0x00080008;
pub const CR_AES_UNKNOWN: c_uint = 0xFFFFFFFF;
pub const CR_ALGO_MASK: c_uint = 0x00080038;
pub const CR_DATA32: c_uint = 0x00000000;
pub const CR_DATA16: c_uint = 0x00000040;
pub const CR_DATA8: c_uint = 0x00000080;
pub const CR_DATA1: c_uint = 0x000000C0;
pub const CR_KEY128: c_uint = 0x00000000;
pub const CR_KEY192: c_uint = 0x00000100;
pub const CR_KEY256: c_uint = 0x00000200;
pub const CR_KEYRDEN: c_uint = 0x00000400 /* Only on Ux500 */;
pub const CR_KSE: c_uint = 0x00000800 /* Only on Ux500 */;
pub const CR_FFLUSH: c_uint = 0x00004000;
pub const CR_CRYPEN: c_uint = 0x00008000;
pub const CR_PH_INIT: c_uint = 0x00000000;
pub const CR_PH_HEADER: c_uint = 0x00010000;
pub const CR_PH_PAYLOAD: c_uint = 0x00020000;
pub const CR_PH_FINAL: c_uint = 0x00030000;
pub const CR_PH_MASK: c_uint = 0x00030000;
pub const CR_NBPBL_SHIFT: c_int = 20;

// Misc

pub const GCM_CTR_INIT: c_int = 2;
pub const CRYP_AUTOSUSPEND_DELAY: c_int = 50;
pub const CRYP_DMA_BURST_REG: c_int = 4;
    enum stm32_dma_mode {
    NO_DMA,
    DMA_PLAIN_SG,
    DMA_NEED_SG_TRUNC
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_cryp_caps {
    pub aeads_support: bool,
    pub linear_aes_key: bool,
    pub kp_mode: bool,
    pub iv_protection: bool,
    pub swap_final: bool,
    pub padding_wa: bool,
    pub cr: u32,
    pub sr: u32,
    pub din: u32,
    pub dout: u32,
    pub dmacr: u32,
    pub imsc: u32,
    pub mis: u32,
    pub k1l: u32,
    pub k1r: u32,
    pub k3r: u32,
    pub iv0l: u32,
    pub iv0r: u32,
    pub iv1l: u32,
    pub iv1r: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_cryp_ctx {
    pub cryp: *mut stm32_cryp,
    pub keylen: c_int,
    pub sizeof(u32)]: __be32 key[AES_KEYSIZE_256 /,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_cryp_reqctx {
    pub mode: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_cryp {
    pub list: list_head,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub phys_base: phys_addr_t,
    pub clk: *mut clk,
    pub flags: c_ulong,
    pub irq_status: u32,
    pub caps: *const stm32_cryp_caps,
    pub ctx: *mut stm32_cryp_ctx,
    pub engine: *mut crypto_engine,
    pub req: *mut skcipher_request,
    pub areq: *mut aead_request,
    pub authsize: usize,
    pub hw_blocksize: usize,
    pub payload_in: usize,
    pub header_in: usize,
    pub payload_out: usize,
// DMA process fields
    pub in_sg: *mut scatterlist,
    pub header_sg: *mut scatterlist,
    pub out_sg: *mut scatterlist,
    pub in_sg_len: usize,
    pub header_sg_len: usize,
    pub out_sg_len: usize,
    pub dma_completion: completion,
    pub dma_lch_in: *mut dma_chan,
    pub dma_lch_out: *mut dma_chan,
    pub dma_mode: enum stm32_dma_mode,
// IT process fields
    pub in_walk: scatter_walk,
    pub out_walk: scatter_walk,
    pub last_ctr: [__be32; 4],
    pub gcm_ctr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_cryp_list {
    pub dev_list: list_head,
    pub /: *mut *mut spinlock_t lock; / protect dev_list,
}

    static struct stm32_cryp_list cryp_list = {
    .dev_list = LIST_HEAD_INIT(cryp_list.dev_list),
    .lock     = __SPIN_LOCK_UNLOCKED(cryp_list.lock),
    };
#[no_mangle]
pub unsafe extern "C" fn is_aes(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_aes(struct stm32_cryp *cryp)
    {
    return cryp.flags & FLG_AES;
    }
#[no_mangle]
pub unsafe extern "C" fn is_des(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_des(struct stm32_cryp *cryp)
    {
    return cryp.flags & FLG_DES;
    }
#[no_mangle]
pub unsafe extern "C" fn is_tdes(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_tdes(struct stm32_cryp *cryp)
    {
    return cryp.flags & FLG_TDES;
    }
#[no_mangle]
pub unsafe extern "C" fn is_ecb(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_ecb(struct stm32_cryp *cryp)
    {
    return cryp.flags & FLG_ECB;
    }
#[no_mangle]
pub unsafe extern "C" fn is_cbc(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_cbc(struct stm32_cryp *cryp)
    {
    return cryp.flags & FLG_CBC;
    }
#[no_mangle]
pub unsafe extern "C" fn is_ctr(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_ctr(struct stm32_cryp *cryp)
    {
    return cryp.flags & FLG_CTR;
    }
#[no_mangle]
pub unsafe extern "C" fn is_gcm(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_gcm(struct stm32_cryp *cryp)
    {
    return cryp.flags & FLG_GCM;
    }
#[no_mangle]
pub unsafe extern "C" fn is_ccm(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_ccm(struct stm32_cryp *cryp)
    {
    return cryp.flags & FLG_CCM;
    }
#[no_mangle]
pub unsafe extern "C" fn is_encrypt(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_encrypt(struct stm32_cryp *cryp)
    {
    return cryp.flags & FLG_ENCRYPT;
    }
#[no_mangle]
pub unsafe extern "C" fn is_decrypt(cryp: *mut stm32_cryp) -> bool {
    static inline bool is_decrypt(struct stm32_cryp *cryp)
    {
    return !is_encrypt(cryp);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_cryp_read(cryp: *mut stm32_cryp, ofst: u32) -> u32 {
    static inline u32 stm32_cryp_read(struct stm32_cryp *cryp, u32 ofst)
    {
    return readl_relaxed(cryp.regs + ofst);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_cryp_write(cryp: *mut stm32_cryp, ofst: u32, val: u32) {
    static inline void stm32_cryp_write(struct stm32_cryp *cryp, u32 ofst, u32 val)
    {
    writel_relaxed(val, cryp.regs + ofst);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_cryp_wait_busy(cryp: *mut stm32_cryp) -> c_int {
    static inline int stm32_cryp_wait_busy(struct stm32_cryp *cryp)
    {
    u32 status;
    return readl_relaxed_poll_timeout(cryp.regs + cryp.caps.sr, status,
    !(status & SR_BUSY), 10, 100000);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_cryp_enable(cryp: *mut stm32_cryp) {
    static inline void stm32_cryp_enable(struct stm32_cryp *cryp)
    {
    writel_relaxed(readl_relaxed(cryp.regs + cryp.caps.cr) | CR_CRYPEN,
    cryp.regs + cryp.caps.cr);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_cryp_wait_enable(cryp: *mut stm32_cryp) -> c_int {
    static inline int stm32_cryp_wait_enable(struct stm32_cryp *cryp)
    {
    u32 status;
    return readl_relaxed_poll_timeout(cryp.regs + cryp.caps.cr, status,
    !(status & CR_CRYPEN), 10, 100000);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_cryp_wait_input(cryp: *mut stm32_cryp) -> c_int {
    static inline int stm32_cryp_wait_input(struct stm32_cryp *cryp)
    {
    u32 status;
    return readl_relaxed_poll_timeout_atomic(cryp.regs + cryp.caps.sr, status,
    status & SR_IFNF, 1, 10);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_cryp_wait_output(cryp: *mut stm32_cryp) -> c_int {
    static inline int stm32_cryp_wait_output(struct stm32_cryp *cryp)
    {
    u32 status;
    return readl_relaxed_poll_timeout_atomic(cryp.regs + cryp.caps.sr, status,
    status & SR_OFNE, 1, 10);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_cryp_key_read_enable(cryp: *mut stm32_cryp) {
    static inline void stm32_cryp_key_read_enable(struct stm32_cryp *cryp)
    {
    writel_relaxed(readl_relaxed(cryp.regs + cryp.caps.cr) | CR_KEYRDEN,
    cryp.regs + cryp.caps.cr);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_cryp_key_read_disable(cryp: *mut stm32_cryp) {
    static inline void stm32_cryp_key_read_disable(struct stm32_cryp *cryp)
    {
    writel_relaxed(readl_relaxed(cryp.regs + cryp.caps.cr) & ~CR_KEYRDEN,
    cryp.regs + cryp.caps.cr);
    }
    static void stm32_cryp_irq_read_data(struct stm32_cryp *cryp);
    static void stm32_cryp_irq_write_data(struct stm32_cryp *cryp);
    static void stm32_cryp_irq_write_gcmccm_header(struct stm32_cryp *cryp);
    static int stm32_cryp_read_auth_tag(struct stm32_cryp *cryp);
    static void stm32_cryp_finish_req(struct stm32_cryp *cryp, int err);
    static int stm32_cryp_dma_start(struct stm32_cryp *cryp);
    static int stm32_cryp_it_start(struct stm32_cryp *cryp);
    static struct stm32_cryp *stm32_cryp_find_dev(struct stm32_cryp_ctx *ctx)
    {
    struct stm32_cryp *cryp;
    spin_lock_bh(&cryp_list.lock);
    if (!ctx.cryp)
    ctx.cryp = list_first_entry_or_null(&cryp_list.dev_list,
    struct stm32_cryp, list);
    cryp = ctx.cryp;
    spin_unlock_bh(&cryp_list.lock);
    return cryp;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_hw_write_iv(cryp: *mut stm32_cryp, iv: *mut __be32) {
    static void stm32_cryp_hw_write_iv(struct stm32_cryp *cryp, __be32 *iv)
    {
    if (!iv)
    return;
    stm32_cryp_write(cryp, cryp.caps.iv0l, be32_to_cpu(*iv++));
    stm32_cryp_write(cryp, cryp.caps.iv0r, be32_to_cpu(*iv++));
    if (is_aes(cryp)) {
    stm32_cryp_write(cryp, cryp.caps.iv1l, be32_to_cpu(*iv++));
    stm32_cryp_write(cryp, cryp.caps.iv1r, be32_to_cpu(*iv++));
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_get_iv(cryp: *mut stm32_cryp) {
    static void stm32_cryp_get_iv(struct stm32_cryp *cryp)
    {
    struct skcipher_request *req = cryp.req;
    __be32 *tmp = (void *)req.iv;
    if (!tmp)
    return;
    if (cryp.caps.iv_protection)
    stm32_cryp_key_read_enable(cryp);
// tmp++ = cpu_to_be32(stm32_cryp_read(cryp, cryp->caps->iv0l));
// tmp++ = cpu_to_be32(stm32_cryp_read(cryp, cryp->caps->iv0r));
    if (is_aes(cryp)) {
// tmp++ = cpu_to_be32(stm32_cryp_read(cryp, cryp->caps->iv1l));
// tmp++ = cpu_to_be32(stm32_cryp_read(cryp, cryp->caps->iv1r));
    }
    if (cryp.caps.iv_protection)
    stm32_cryp_key_read_disable(cryp);
    }
//
// ux500_swap_bits_in_byte() - mirror the bits in a byte
// @b: the byte to be mirrored
//
// The bits are swapped the following way:
// Byte b include bits 0-7, nibble 1 (n1) include bits 0-3 and
// nibble 2 (n2) bits 4-7.
//
// Nibble 1 (n1):
// (The "old" (moved) bit is replaced with a zero)
// 1. Move bit 6 and 7, 4 positions to the left.
// 2. Move bit 3 and 5, 2 positions to the left.
// 3. Move bit 1-4, 1 position to the left.
//
// Nibble 2 (n2):
// 1. Move bit 0 and 1, 4 positions to the right.
// 2. Move bit 2 and 4, 2 positions to the right.
// 3. Move bit 3-6, 1 position to the right.
//
// Combine the two nibbles to a complete and swapped byte.
//
#[no_mangle]
pub unsafe extern "C" fn ux500_swap_bits_in_byte(b: u8) -> u8 {
    static inline u8 ux500_swap_bits_in_byte(u8 b)
    {
pub const R_SHIFT_4_MASK: c_uint = 0xc0 /* Bits 6 and 7, right shift 4 */;
pub const R_SHIFT_2_MASK: c_uint = 0x28 /* (After right shift 4) Bits 3 and 5,;
    right shift 2 */
pub const R_SHIFT_1_MASK: c_uint = 0x1e /* (After right shift 2) Bits 1-4,;
    right shift 1 */
pub const L_SHIFT_4_MASK: c_uint = 0x03 /* Bits 0 and 1, left shift 4 */;
pub const L_SHIFT_2_MASK: c_uint = 0x14 /* (After left shift 4) Bits 2 and 4,;
    left shift 2 */
pub const L_SHIFT_1_MASK: c_uint = 0x78 /* (After left shift 1) Bits 3-6,;
    left shift 1 */
    u8 n1;
    u8 n2;
// Swap most significant nibble
// Right shift 4, bits 6 and 7
    n1 = ((b  & R_SHIFT_4_MASK) >> 4) | (b  & ~(R_SHIFT_4_MASK >> 4));
// Right shift 2, bits 3 and 5
    n1 = ((n1 & R_SHIFT_2_MASK) >> 2) | (n1 & ~(R_SHIFT_2_MASK >> 2));
// Right shift 1, bits 1-4
    n1 = (n1  & R_SHIFT_1_MASK) >> 1;
// Swap least significant nibble
// Left shift 4, bits 0 and 1
    n2 = ((b  & L_SHIFT_4_MASK) << 4) | (b  & ~(L_SHIFT_4_MASK << 4));
// Left shift 2, bits 2 and 4
    n2 = ((n2 & L_SHIFT_2_MASK) << 2) | (n2 & ~(L_SHIFT_2_MASK << 2));
// Left shift 1, bits 3-6
    n2 = (n2  & L_SHIFT_1_MASK) << 1;
    return n1 | n2;
    }
//
// ux500_swizzle_key() - Shuffle around words and bits in the AES key
// @in: key to swizzle
// @out: swizzled key
// @len: length of key, in bytes
//
// This "key swizzling procedure" is described in the examples in the
// DB8500 design specification. There is no real description of why
// the bits have been arranged like this in the hardware.
//
#[no_mangle]
pub unsafe extern "C" fn ux500_swizzle_key(in: *const u8, out: *mut u8, len: u32) {
    static inline void ux500_swizzle_key(const u8 *in, u8 *out, u32 len)
    {
    let mut i: c_int = 0;
    let mut bpw: c_int = sizeof(u32);
    int j;
    let mut index: c_int = 0;
    j = len - bpw;
    while (j >= 0) {
    for (i = 0; i < bpw; i++) {
    index = len - j - bpw + i;
    out[j + i] =
    ux500_swap_bits_in_byte(in[index]);
    }
    j -= bpw;
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_hw_write_key(c: *mut stm32_cryp) {
    static void stm32_cryp_hw_write_key(struct stm32_cryp *c)
    {
    unsigned int i;
    int r_id;
    if (is_des(c)) {
    stm32_cryp_write(c, c.caps.k1l, be32_to_cpu(c.ctx.key[0]));
    stm32_cryp_write(c, c.caps.k1r, be32_to_cpu(c.ctx.key[1]));
    return;
    }
//
// On the Ux500 the AES key is considered as a single bit sequence
// of 128, 192 or 256 bits length. It is written linearly into the
// registers from K1L and down, and need to be processed to become
// a proper big-endian bit sequence.
//
    if (is_aes(c) && c.caps.linear_aes_key) {
    u32 tmpkey[8];
    ux500_swizzle_key((u8 *)c.ctx.key,
    (u8 *)tmpkey, c.ctx.keylen);
    r_id = c.caps.k1l;
    for (i = 0; i < c.ctx.keylen / sizeof(u32); i++, r_id += 4)
    stm32_cryp_write(c, r_id, tmpkey[i]);
    return;
    }
    r_id = c.caps.k3r;
    for (i = c.ctx.keylen / sizeof(u32); i > 0; i--, r_id -= 4)
    stm32_cryp_write(c, r_id, be32_to_cpu(c.ctx.key[i - 1]));
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_get_hw_mode(cryp: *mut stm32_cryp) -> u32 {
    static u32 stm32_cryp_get_hw_mode(struct stm32_cryp *cryp)
    {
    if (is_aes(cryp) && is_ecb(cryp))
    return CR_AES_ECB;
    if (is_aes(cryp) && is_cbc(cryp))
    return CR_AES_CBC;
    if (is_aes(cryp) && is_ctr(cryp))
    return CR_AES_CTR;
    if (is_aes(cryp) && is_gcm(cryp))
    return CR_AES_GCM;
    if (is_aes(cryp) && is_ccm(cryp))
    return CR_AES_CCM;
    if (is_des(cryp) && is_ecb(cryp))
    return CR_DES_ECB;
    if (is_des(cryp) && is_cbc(cryp))
    return CR_DES_CBC;
    if (is_tdes(cryp) && is_ecb(cryp))
    return CR_TDES_ECB;
    if (is_tdes(cryp) && is_cbc(cryp))
    return CR_TDES_CBC;
    dev_err(cryp.dev, "Unknown mode\n");
    return CR_AES_UNKNOWN;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_get_input_text_len(cryp: *mut stm32_cryp) -> c_uint {
    static unsigned int stm32_cryp_get_input_text_len(struct stm32_cryp *cryp)
    {
    return is_encrypt(cryp) ? cryp.areq.cryptlen :
    cryp.areq.cryptlen - cryp.authsize;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_gcm_init(cryp: *mut stm32_cryp, cfg: u32) -> c_int {
    static int stm32_cryp_gcm_init(struct stm32_cryp *cryp, u32 cfg)
    {
    int ret;
    __be32 iv[4];
// Phase 1 : init
    memcpy(iv, cryp.areq.iv, 12);
    iv[3] = cpu_to_be32(GCM_CTR_INIT);
    cryp.gcm_ctr = GCM_CTR_INIT;
    stm32_cryp_hw_write_iv(cryp, iv);
    stm32_cryp_write(cryp, cryp.caps.cr, cfg | CR_PH_INIT | CR_CRYPEN);
// Wait for end of processing
    ret = stm32_cryp_wait_enable(cryp);
    if (ret) {
    dev_err(cryp.dev, "Timeout (gcm init)\n");
    return ret;
    }
// Prepare next phase
    if (cryp.areq.assoclen) {
    cfg |= CR_PH_HEADER;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    } else if (stm32_cryp_get_input_text_len(cryp)) {
    cfg |= CR_PH_PAYLOAD;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_crypt_gcmccm_end_header(cryp: *mut stm32_cryp) {
    static void stm32_crypt_gcmccm_end_header(struct stm32_cryp *cryp)
    {
    u32 cfg;
    int err;
// Check if whole header written
    if (!cryp.header_in) {
// Wait for completion
    err = stm32_cryp_wait_busy(cryp);
    if (err) {
    dev_err(cryp.dev, "Timeout (gcm/ccm header)\n");
    stm32_cryp_write(cryp, cryp.caps.imsc, 0);
    stm32_cryp_finish_req(cryp, err);
    return;
    }
    if (stm32_cryp_get_input_text_len(cryp)) {
// Phase 3 : payload
    cfg = stm32_cryp_read(cryp, cryp.caps.cr);
    cfg &= ~CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    cfg &= ~CR_PH_MASK;
    cfg |= CR_PH_PAYLOAD | CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    } else {
//
// Phase 4 : tag.
// Nothing to read, nothing to write, caller have to
// end request
//
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_write_ccm_first_header(cryp: *mut stm32_cryp) {
    static void stm32_cryp_write_ccm_first_header(struct stm32_cryp *cryp)
    {
    size_t written;
    size_t len;
    let mut alen: u32 = cryp.areq.assoclen;
    u32 block[AES_BLOCK_32] = {0};
    u8 *b8 = (u8 *)block;
    if (alen <= 65280) {
// Write first u32 of B1
    b8[0] = (alen >> 8) & 0xFF;
    b8[1] = alen & 0xFF;
    len = 2;
    } else {
// Build the two first u32 of B1
    b8[0] = 0xFF;
    b8[1] = 0xFE;
    b8[2] = (alen & 0xFF000000) >> 24;
    b8[3] = (alen & 0x00FF0000) >> 16;
    b8[4] = (alen & 0x0000FF00) >> 8;
    b8[5] = alen & 0x000000FF;
    len = 6;
    }
    written = min_t(size_t, AES_BLOCK_SIZE - len, alen);
    memcpy_from_scatterwalk((char *)block + len, &cryp.in_walk, written);
    writesl(cryp.regs + cryp.caps.din, block, AES_BLOCK_32);
    cryp.header_in -= written;
    stm32_crypt_gcmccm_end_header(cryp);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_ccm_init(cryp: *mut stm32_cryp, cfg: u32) -> c_int {
    static int stm32_cryp_ccm_init(struct stm32_cryp *cryp, u32 cfg)
    {
    int ret;
    u32 iv_32[AES_BLOCK_32], b0_32[AES_BLOCK_32];
    u8 *iv = (u8 *)iv_32, *b0 = (u8 *)b0_32;
    __be32 *bd;
    u32 *d;
    unsigned int i, textlen;
// Phase 1 : init. Firstly set the CTR value to 1 (not 0)
    memcpy(iv, cryp.areq.iv, AES_BLOCK_SIZE);
    memset(iv + AES_BLOCK_SIZE - 1 - iv[0], 0, iv[0] + 1);
    iv[AES_BLOCK_SIZE - 1] = 1;
    stm32_cryp_hw_write_iv(cryp, (__be32 *)iv);
// Build B0
    memcpy(b0, iv, AES_BLOCK_SIZE);
    b0[0] |= (8 * ((cryp.authsize - 2) / 2));
    if (cryp.areq.assoclen)
    b0[0] |= 0x40;
    textlen = stm32_cryp_get_input_text_len(cryp);
    b0[AES_BLOCK_SIZE - 2] = textlen >> 8;
    b0[AES_BLOCK_SIZE - 1] = textlen & 0xFF;
// Enable HW
    stm32_cryp_write(cryp, cryp.caps.cr, cfg | CR_PH_INIT | CR_CRYPEN);
// Write B0
    d = (u32 *)b0;
    bd = (__be32 *)b0;
    for (i = 0; i < AES_BLOCK_32; i++) {
    let mut xd: u32 = d[i];
    if (!cryp.caps.padding_wa)
    xd = be32_to_cpu(bd[i]);
    stm32_cryp_write(cryp, cryp.caps.din, xd);
    }
// Wait for end of processing
    ret = stm32_cryp_wait_enable(cryp);
    if (ret) {
    dev_err(cryp.dev, "Timeout (ccm init)\n");
    return ret;
    }
// Prepare next phase
    if (cryp.areq.assoclen) {
    cfg |= CR_PH_HEADER | CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// Write first (special) block (may move to next phase [payload])
    stm32_cryp_write_ccm_first_header(cryp);
    } else if (stm32_cryp_get_input_text_len(cryp)) {
    cfg |= CR_PH_PAYLOAD;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_hw_init(cryp: *mut stm32_cryp) -> c_int {
    static int stm32_cryp_hw_init(struct stm32_cryp *cryp)
    {
    int ret;
    u32 cfg, hw_mode;
    pm_runtime_get_sync(cryp.dev);
// Disable interrupt
    stm32_cryp_write(cryp, cryp.caps.imsc, 0);
// Set configuration
    cfg = CR_DATA8 | CR_FFLUSH;
    switch (cryp.ctx.keylen) {
    case AES_KEYSIZE_128:
    cfg |= CR_KEY128;
    break;
    case AES_KEYSIZE_192:
    cfg |= CR_KEY192;
    break;
    default:
    case AES_KEYSIZE_256:
    cfg |= CR_KEY256;
    break;
    }
    hw_mode = stm32_cryp_get_hw_mode(cryp);
    if (hw_mode == CR_AES_UNKNOWN)
    return -EINVAL;
// AES ECB/CBC decrypt: run key preparation first
    if (is_decrypt(cryp) &&
    ((hw_mode == CR_AES_ECB) || (hw_mode == CR_AES_CBC))) {
// Configure in key preparation mode
    if (cryp.caps.kp_mode)
    stm32_cryp_write(cryp, cryp.caps.cr,
    cfg | CR_AES_KP);
    else
    stm32_cryp_write(cryp,
    cryp.caps.cr, cfg | CR_AES_ECB | CR_KSE);
// Set key only after full configuration done
    stm32_cryp_hw_write_key(cryp);
// Start prepare key
    stm32_cryp_enable(cryp);
// Wait for end of processing
    ret = stm32_cryp_wait_busy(cryp);
    if (ret) {
    dev_err(cryp.dev, "Timeout (key preparation)\n");
    return ret;
    }
    cfg |= hw_mode | CR_DEC_NOT_ENC;
// Apply updated config (Decrypt + algo) and flush
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    } else {
    cfg |= hw_mode;
    if (is_decrypt(cryp))
    cfg |= CR_DEC_NOT_ENC;
// Apply config and flush
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// Set key only after configuration done
    stm32_cryp_hw_write_key(cryp);
    }
    switch (hw_mode) {
    case CR_AES_GCM:
    case CR_AES_CCM:
// Phase 1 : init
    if (hw_mode == CR_AES_CCM)
    ret = stm32_cryp_ccm_init(cryp, cfg);
    else
    ret = stm32_cryp_gcm_init(cryp, cfg);
    if (ret)
    return ret;
    break;
    case CR_DES_CBC:
    case CR_TDES_CBC:
    case CR_AES_CBC:
    case CR_AES_CTR:
    stm32_cryp_hw_write_iv(cryp, (__be32 *)cryp.req.iv);
    break;
    default:
    break;
    }
// Enable now
    stm32_cryp_enable(cryp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_finish_req(cryp: *mut stm32_cryp, err: c_int) {
    static void stm32_cryp_finish_req(struct stm32_cryp *cryp, int err)
    {
    if (!err && (is_gcm(cryp) || is_ccm(cryp)))
// Phase 4 : output tag
    err = stm32_cryp_read_auth_tag(cryp);
    if (!err && (!(is_gcm(cryp) || is_ccm(cryp) || is_ecb(cryp))))
    stm32_cryp_get_iv(cryp);
    pm_runtime_put_autosuspend(cryp.dev);
    if (is_gcm(cryp) || is_ccm(cryp))
    crypto_finalize_aead_request(cryp.engine, cryp.areq, err);
    else
    crypto_finalize_skcipher_request(cryp.engine, cryp.req, err);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_header_dma_callback(param: *mut c_void) {
    static void stm32_cryp_header_dma_callback(void *param)
    {
    struct stm32_cryp *cryp = (struct stm32_cryp *)param;
    int ret;
    u32 reg;
    dma_unmap_sg(cryp.dev, cryp.header_sg, cryp.header_sg_len, DMA_TO_DEVICE);
    reg = stm32_cryp_read(cryp, cryp.caps.dmacr);
    stm32_cryp_write(cryp, cryp.caps.dmacr, reg & ~(DMACR_DOEN | DMACR_DIEN));
    kfree(cryp.header_sg);
    reg = stm32_cryp_read(cryp, cryp.caps.cr);
    if (cryp.header_in) {
    stm32_cryp_write(cryp, cryp.caps.cr, reg | CR_CRYPEN);
    ret = stm32_cryp_wait_input(cryp);
    if (ret) {
    dev_err(cryp.dev, "input header ready timeout after dma\n");
    stm32_cryp_finish_req(cryp, ret);
    return;
    }
    stm32_cryp_irq_write_gcmccm_header(cryp);
    WARN_ON(cryp.header_in);
    }
    if (stm32_cryp_get_input_text_len(cryp)) {
// Phase 3 : payload
    reg = stm32_cryp_read(cryp, cryp.caps.cr);
    stm32_cryp_write(cryp, cryp.caps.cr, reg & ~CR_CRYPEN);
    reg &= ~CR_PH_MASK;
    reg |= CR_PH_PAYLOAD | CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, reg);
    if (cryp.flags & FLG_IN_OUT_DMA) {
    ret = stm32_cryp_dma_start(cryp);
    if (ret)
    stm32_cryp_finish_req(cryp, ret);
    } else {
    stm32_cryp_it_start(cryp);
    }
    } else {
//
// Phase 4 : tag.
// Nothing to read, nothing to write => end request
//
    stm32_cryp_finish_req(cryp, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_dma_callback(param: *mut c_void) {
    static void stm32_cryp_dma_callback(void *param)
    {
    struct stm32_cryp *cryp = (struct stm32_cryp *)param;
    int ret;
    u32 reg;
    complete(&cryp.dma_completion); /* completion to indicate no timeout */
    dma_sync_sg_for_device(cryp.dev, cryp.out_sg, cryp.out_sg_len, DMA_FROM_DEVICE);
    if (cryp.in_sg != cryp.out_sg)
    dma_unmap_sg(cryp.dev, cryp.in_sg, cryp.in_sg_len, DMA_TO_DEVICE);
    dma_unmap_sg(cryp.dev, cryp.out_sg, cryp.out_sg_len, DMA_FROM_DEVICE);
    reg = stm32_cryp_read(cryp, cryp.caps.dmacr);
    stm32_cryp_write(cryp, cryp.caps.dmacr, reg & ~(DMACR_DOEN | DMACR_DIEN));
    reg = stm32_cryp_read(cryp, cryp.caps.cr);
    if (is_gcm(cryp) || is_ccm(cryp)) {
    kfree(cryp.in_sg);
    kfree(cryp.out_sg);
    } else {
    if (cryp.in_sg != cryp.req.src)
    kfree(cryp.in_sg);
    if (cryp.out_sg != cryp.req.dst)
    kfree(cryp.out_sg);
    }
    if (cryp.payload_in) {
    stm32_cryp_write(cryp, cryp.caps.cr, reg | CR_CRYPEN);
    ret = stm32_cryp_wait_input(cryp);
    if (ret) {
    dev_err(cryp.dev, "input ready timeout after dma\n");
    stm32_cryp_finish_req(cryp, ret);
    return;
    }
    stm32_cryp_irq_write_data(cryp);
    ret = stm32_cryp_wait_output(cryp);
    if (ret) {
    dev_err(cryp.dev, "output ready timeout after dma\n");
    stm32_cryp_finish_req(cryp, ret);
    return;
    }
    stm32_cryp_irq_read_data(cryp);
    }
    stm32_cryp_finish_req(cryp, 0);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_header_dma_start(cryp: *mut stm32_cryp) -> c_int {
    static int stm32_cryp_header_dma_start(struct stm32_cryp *cryp)
    {
    int ret;
    struct dma_async_tx_descriptor *tx_in;
    u32 reg;
    size_t align_size;
    ret = dma_map_sg(cryp.dev, cryp.header_sg, cryp.header_sg_len, DMA_TO_DEVICE);
    if (!ret) {
    dev_err(cryp.dev, "dma_map_sg() error\n");
    return -ENOMEM;
    }
    dma_sync_sg_for_device(cryp.dev, cryp.header_sg, cryp.header_sg_len, DMA_TO_DEVICE);
    tx_in = dmaengine_prep_slave_sg(cryp.dma_lch_in, cryp.header_sg, cryp.header_sg_len,
    DMA_MEM_TO_DEV, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!tx_in) {
    dev_err(cryp.dev, "IN prep_slave_sg() failed\n");
    return -EINVAL;
    }
    tx_in.callback_param = cryp;
    tx_in.callback = stm32_cryp_header_dma_callback;
// Advance scatterwalk to not DMA'ed data
    align_size = ALIGN_DOWN(cryp.header_in, cryp.hw_blocksize);
    scatterwalk_skip(&cryp.in_walk, align_size);
    cryp.header_in -= align_size;
    ret = dma_submit_error(dmaengine_submit(tx_in));
    if (ret < 0) {
    dev_err(cryp.dev, "DMA in submit failed\n");
    return ret;
    }
    dma_async_issue_pending(cryp.dma_lch_in);
    reg = stm32_cryp_read(cryp, cryp.caps.dmacr);
    stm32_cryp_write(cryp, cryp.caps.dmacr, reg | DMACR_DIEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_dma_start(cryp: *mut stm32_cryp) -> c_int {
    static int stm32_cryp_dma_start(struct stm32_cryp *cryp)
    {
    int ret;
    size_t align_size;
    struct dma_async_tx_descriptor *tx_in, *tx_out;
    u32 reg;
    if (cryp.in_sg != cryp.out_sg) {
    ret = dma_map_sg(cryp.dev, cryp.in_sg, cryp.in_sg_len, DMA_TO_DEVICE);
    if (!ret) {
    dev_err(cryp.dev, "dma_map_sg() error\n");
    return -ENOMEM;
    }
    }
    ret = dma_map_sg(cryp.dev, cryp.out_sg, cryp.out_sg_len, DMA_FROM_DEVICE);
    if (!ret) {
    dev_err(cryp.dev, "dma_map_sg() error\n");
    return -ENOMEM;
    }
    dma_sync_sg_for_device(cryp.dev, cryp.in_sg, cryp.in_sg_len, DMA_TO_DEVICE);
    tx_in = dmaengine_prep_slave_sg(cryp.dma_lch_in, cryp.in_sg, cryp.in_sg_len,
    DMA_MEM_TO_DEV, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!tx_in) {
    dev_err(cryp.dev, "IN prep_slave_sg() failed\n");
    return -EINVAL;
    }
// No callback necessary
    tx_in.callback_param = cryp;
    tx_in.callback = core::ptr::null_mut();
    tx_out = dmaengine_prep_slave_sg(cryp.dma_lch_out, cryp.out_sg, cryp.out_sg_len,
    DMA_DEV_TO_MEM, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!tx_out) {
    dev_err(cryp.dev, "OUT prep_slave_sg() failed\n");
    return -EINVAL;
    }
    reinit_completion(&cryp.dma_completion);
    tx_out.callback = stm32_cryp_dma_callback;
    tx_out.callback_param = cryp;
// Advance scatterwalk to not DMA'ed data
    align_size = ALIGN_DOWN(cryp.payload_in, cryp.hw_blocksize);
    scatterwalk_skip(&cryp.in_walk, align_size);
    cryp.payload_in -= align_size;
    ret = dma_submit_error(dmaengine_submit(tx_in));
    if (ret < 0) {
    dev_err(cryp.dev, "DMA in submit failed\n");
    return ret;
    }
    dma_async_issue_pending(cryp.dma_lch_in);
// Advance scatterwalk to not DMA'ed data
    scatterwalk_skip(&cryp.out_walk, align_size);
    cryp.payload_out -= align_size;
    ret = dma_submit_error(dmaengine_submit(tx_out));
    if (ret < 0) {
    dev_err(cryp.dev, "DMA out submit failed\n");
    return ret;
    }
    dma_async_issue_pending(cryp.dma_lch_out);
    reg = stm32_cryp_read(cryp, cryp.caps.dmacr);
    stm32_cryp_write(cryp, cryp.caps.dmacr, reg | DMACR_DOEN | DMACR_DIEN);
    if (!wait_for_completion_timeout(&cryp.dma_completion, msecs_to_jiffies(1000))) {
    dev_err(cryp.dev, "DMA out timed out\n");
    dmaengine_terminate_sync(cryp.dma_lch_out);
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_it_start(cryp: *mut stm32_cryp) -> c_int {
    static int stm32_cryp_it_start(struct stm32_cryp *cryp)
    {
// Enable interrupt and let the IRQ handler do everything
    stm32_cryp_write(cryp, cryp.caps.imsc, IMSCR_IN | IMSCR_OUT);
    return 0;
    }
    static int stm32_cryp_cipher_one_req(struct crypto_engine *engine, void *areq);
#[no_mangle]
unsafe extern "C" fn stm32_cryp_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int stm32_cryp_init_tfm(struct crypto_skcipher *tfm)
    {
    crypto_skcipher_set_reqsize(tfm, sizeof(struct stm32_cryp_reqctx));
    return 0;
    }
    static int stm32_cryp_aead_one_req(struct crypto_engine *engine, void *areq);
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_aead_init(tfm: *mut crypto_aead) -> c_int {
    static int stm32_cryp_aes_aead_init(struct crypto_aead *tfm)
    {
    crypto_aead_set_reqsize(tfm, sizeof(struct stm32_cryp_reqctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_crypt(req: *mut skcipher_request, mode: c_ulong) -> c_int {
    static int stm32_cryp_crypt(struct skcipher_request *req, unsigned long mode)
    {
    struct stm32_cryp_ctx *ctx = crypto_skcipher_ctx(
    crypto_skcipher_reqtfm(req));
    struct stm32_cryp_reqctx *rctx = skcipher_request_ctx(req);
    struct stm32_cryp *cryp = stm32_cryp_find_dev(ctx);
    if (!cryp)
    return -ENODEV;
    rctx.mode = mode;
    return crypto_transfer_skcipher_request_to_engine(cryp.engine, req);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aead_crypt(req: *mut aead_request, mode: c_ulong) -> c_int {
    static int stm32_cryp_aead_crypt(struct aead_request *req, unsigned long mode)
    {
    struct stm32_cryp_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    struct stm32_cryp_reqctx *rctx = aead_request_ctx(req);
    struct stm32_cryp *cryp = stm32_cryp_find_dev(ctx);
    if (!cryp)
    return -ENODEV;
    rctx.mode = mode;
    return crypto_transfer_aead_request_to_engine(cryp.engine, req);
    }
    static int stm32_cryp_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct stm32_cryp_ctx *ctx = crypto_skcipher_ctx(tfm);
    memcpy(ctx.key, key, keylen);
    ctx.keylen = keylen;
    return 0;
    }
    static int stm32_cryp_aes_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    if (keylen != AES_KEYSIZE_128 && keylen != AES_KEYSIZE_192 &&
    keylen != AES_KEYSIZE_256)
    return -EINVAL;
    else
    return stm32_cryp_setkey(tfm, key, keylen);
    }
    static int stm32_cryp_des_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    return verify_skcipher_des_key(tfm, key) ?:
    stm32_cryp_setkey(tfm, key, keylen);
    }
    static int stm32_cryp_tdes_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    return verify_skcipher_des3_key(tfm, key) ?:
    stm32_cryp_setkey(tfm, key, keylen);
    }
    static int stm32_cryp_aes_aead_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct stm32_cryp_ctx *ctx = crypto_aead_ctx(tfm);
    if (keylen != AES_KEYSIZE_128 && keylen != AES_KEYSIZE_192 &&
    keylen != AES_KEYSIZE_256)
    return -EINVAL;
    memcpy(ctx.key, key, keylen);
    ctx.keylen = keylen;
    return 0;
    }
    static int stm32_cryp_aes_gcm_setauthsize(struct crypto_aead *tfm,
    unsigned int authsize)
    {
    switch (authsize) {
    case 4:
    case 8:
    case 12:
    case 13:
    case 14:
    case 15:
    case 16:
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int stm32_cryp_aes_ccm_setauthsize(struct crypto_aead *tfm,
    unsigned int authsize)
    {
    switch (authsize) {
    case 4:
    case 6:
    case 8:
    case 10:
    case 12:
    case 14:
    case 16:
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_aes_ecb_encrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % AES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_AES | FLG_ECB | FLG_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_aes_ecb_decrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % AES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_AES | FLG_ECB);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_aes_cbc_encrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % AES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_AES | FLG_CBC | FLG_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_aes_cbc_decrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % AES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_AES | FLG_CBC);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_ctr_encrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_aes_ctr_encrypt(struct skcipher_request *req)
    {
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_AES | FLG_CTR | FLG_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_ctr_decrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_aes_ctr_decrypt(struct skcipher_request *req)
    {
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_AES | FLG_CTR);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_gcm_encrypt(req: *mut aead_request) -> c_int {
    static int stm32_cryp_aes_gcm_encrypt(struct aead_request *req)
    {
    return stm32_cryp_aead_crypt(req, FLG_AES | FLG_GCM | FLG_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_gcm_decrypt(req: *mut aead_request) -> c_int {
    static int stm32_cryp_aes_gcm_decrypt(struct aead_request *req)
    {
    return stm32_cryp_aead_crypt(req, FLG_AES | FLG_GCM);
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_ccm_check_iv(iv: *const u8) -> c_int {
    static inline int crypto_ccm_check_iv(const u8 *iv)
    {
// 2 <= L <= 8, so 1 <= L' <= 7.
    if (iv[0] < 1 || iv[0] > 7)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_ccm_encrypt(req: *mut aead_request) -> c_int {
    static int stm32_cryp_aes_ccm_encrypt(struct aead_request *req)
    {
    int err;
    err = crypto_ccm_check_iv(req.iv);
    if (err)
    return err;
    return stm32_cryp_aead_crypt(req, FLG_AES | FLG_CCM | FLG_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aes_ccm_decrypt(req: *mut aead_request) -> c_int {
    static int stm32_cryp_aes_ccm_decrypt(struct aead_request *req)
    {
    int err;
    err = crypto_ccm_check_iv(req.iv);
    if (err)
    return err;
    return stm32_cryp_aead_crypt(req, FLG_AES | FLG_CCM);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_des_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_des_ecb_encrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % DES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_DES | FLG_ECB | FLG_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_des_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_des_ecb_decrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % DES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_DES | FLG_ECB);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_des_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_des_cbc_encrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % DES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_DES | FLG_CBC | FLG_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_des_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_des_cbc_decrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % DES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_DES | FLG_CBC);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_tdes_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_tdes_ecb_encrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % DES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_TDES | FLG_ECB | FLG_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_tdes_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_tdes_ecb_decrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % DES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_TDES | FLG_ECB);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_tdes_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_tdes_cbc_encrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % DES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_TDES | FLG_CBC | FLG_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_tdes_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int stm32_cryp_tdes_cbc_decrypt(struct skcipher_request *req)
    {
    if (req.cryptlen % DES_BLOCK_SIZE)
    return -EINVAL;
    if (req.cryptlen == 0)
    return 0;
    return stm32_cryp_crypt(req, FLG_TDES | FLG_CBC);
    }
    static enum stm32_dma_mode stm32_cryp_dma_check_sg(struct scatterlist *test_sg, size_t len,
    size_t block_size)
    {
    struct scatterlist *sg;
    int i;
    if (len <= 16)
    return NO_DMA; /* Faster */
    for_each_sg(test_sg, sg, sg_nents(test_sg), i) {
    if (!IS_ALIGNED(sg.length, block_size) && !sg_is_last(sg))
    return NO_DMA;
    if (sg.offset % sizeof(u32))
    return NO_DMA;
    if (sg_is_last(sg) && !IS_ALIGNED(sg.length, AES_BLOCK_SIZE))
    return DMA_NEED_SG_TRUNC;
    }
    return DMA_PLAIN_SG;
    }
    static enum stm32_dma_mode stm32_cryp_dma_check(struct stm32_cryp *cryp, struct scatterlist *in_sg,
    struct scatterlist *out_sg)
    {
    let mut ret: enum stm32_dma_mode = DMA_PLAIN_SG;
    if (!is_aes(cryp))
    return NO_DMA;
    if (!cryp.dma_lch_in || !cryp.dma_lch_out)
    return NO_DMA;
    ret = stm32_cryp_dma_check_sg(in_sg, cryp.payload_in, AES_BLOCK_SIZE);
    if (ret == NO_DMA)
    return ret;
    ret = stm32_cryp_dma_check_sg(out_sg, cryp.payload_out, AES_BLOCK_SIZE);
    if (ret == NO_DMA)
    return ret;
// Check CTR counter overflow
    if (is_aes(cryp) && is_ctr(cryp)) {
    u32 c;
    __be32 iv3;
    memcpy(&iv3, &cryp.req.iv[3 * sizeof(u32)], sizeof(iv3));
    c = be32_to_cpu(iv3);
    if ((c + cryp.payload_in) < cryp.payload_in)
    return NO_DMA;
    }
// Workaround
    if (is_aes(cryp) && is_ctr(cryp) && ret == DMA_NEED_SG_TRUNC)
    return NO_DMA;
    return ret;
    }
    static int stm32_cryp_truncate_sg(struct scatterlist **new_sg, size_t *new_sg_len,
    struct scatterlist *sg, off_t skip, size_t size)
    {
    struct scatterlist *cur;
    int alloc_sg_len;
// new_sg_len = 0;
    if (!sg || !size) {
// new_sg = NULL;
    return 0;
    }
    alloc_sg_len = sg_nents_for_len(sg, skip + size);
    if (alloc_sg_len < 0)
    return alloc_sg_len;
// We allocate to much sg entry, but it is easier
// new_sg = kmalloc_objs(struct scatterlist, (size_t)alloc_sg_len);
    if (!*new_sg)
    return -ENOMEM;
    sg_init_table(*new_sg, (unsigned int)alloc_sg_len);
    cur = *new_sg;
    while (sg && size) {
    let mut len: c_uint = sg.length;
    let mut offset: c_uint = sg.offset;
    if (skip > len) {
    skip -= len;
    sg = sg_next(sg);
    continue;
    }
    if (skip) {
    len -= skip;
    offset += skip;
    skip = 0;
    }
    if (size < len)
    len = size;
    if (len > 0) {
    (*new_sg_len)++;
    size -= len;
    sg_set_page(cur, sg_page(sg), len, offset);
    if (size == 0)
    sg_mark_end(cur);
    cur = sg_next(cur);
    }
    sg = sg_next(sg);
    }
    return 0;
    }
    static int stm32_cryp_cipher_prepare(struct stm32_cryp *cryp, struct scatterlist *in_sg,
    struct scatterlist *out_sg)
    {
    size_t align_size;
    int ret;
    cryp.dma_mode = stm32_cryp_dma_check(cryp, in_sg, out_sg);
    scatterwalk_start(&cryp.in_walk, in_sg);
    scatterwalk_start(&cryp.out_walk, out_sg);
    if (cryp.dma_mode == NO_DMA) {
    cryp.flags &= ~FLG_IN_OUT_DMA;
    if (is_ctr(cryp))
    memset(cryp.last_ctr, 0, sizeof(cryp.last_ctr));
    } else if (cryp.dma_mode == DMA_NEED_SG_TRUNC) {
    cryp.flags |= FLG_IN_OUT_DMA;
    align_size = ALIGN_DOWN(cryp.payload_in, cryp.hw_blocksize);
    ret = stm32_cryp_truncate_sg(&cryp.in_sg, &cryp.in_sg_len, in_sg, 0, align_size);
    if (ret)
    return ret;
    ret = stm32_cryp_truncate_sg(&cryp.out_sg, &cryp.out_sg_len, out_sg, 0,
    align_size);
    if (ret) {
    kfree(cryp.in_sg);
    return ret;
    }
    } else {
    cryp.flags |= FLG_IN_OUT_DMA;
    cryp.in_sg = in_sg;
    cryp.out_sg = out_sg;
    ret = sg_nents_for_len(cryp.in_sg, cryp.payload_in);
    if (ret < 0)
    return ret;
    cryp.in_sg_len = (size_t)ret;
    ret = sg_nents_for_len(out_sg, cryp.payload_out);
    if (ret < 0)
    return ret;
    cryp.out_sg_len = (size_t)ret;
    }
    return 0;
    }
    static int stm32_cryp_aead_prepare(struct stm32_cryp *cryp, struct scatterlist *in_sg,
    struct scatterlist *out_sg)
    {
    size_t align_size;
    off_t skip;
    int ret, ret2;
    cryp.header_sg = core::ptr::null_mut();
    cryp.in_sg = core::ptr::null_mut();
    cryp.out_sg = core::ptr::null_mut();
    if (!cryp.dma_lch_in || !cryp.dma_lch_out) {
    cryp.dma_mode = NO_DMA;
    cryp.flags &= ~(FLG_IN_OUT_DMA | FLG_HEADER_DMA);
    return 0;
    }
// CCM hw_init may have advanced in header
    skip = cryp.areq.assoclen - cryp.header_in;
    align_size = ALIGN_DOWN(cryp.header_in, cryp.hw_blocksize);
    ret = stm32_cryp_truncate_sg(&cryp.header_sg, &cryp.header_sg_len, in_sg, skip,
    align_size);
    if (ret)
    return ret;
    ret = stm32_cryp_dma_check_sg(cryp.header_sg, align_size, AES_BLOCK_SIZE);
    if (ret == NO_DMA) {
// We cannot DMA the header
    kfree(cryp.header_sg);
    cryp.header_sg = core::ptr::null_mut();
    cryp.flags &= ~FLG_HEADER_DMA;
    } else {
    cryp.flags |= FLG_HEADER_DMA;
    }
// Now skip all header to be at payload start
    skip = cryp.areq.assoclen;
    align_size = ALIGN_DOWN(cryp.payload_in, cryp.hw_blocksize);
    ret = stm32_cryp_truncate_sg(&cryp.in_sg, &cryp.in_sg_len, in_sg, skip, align_size);
    if (ret) {
    kfree(cryp.header_sg);
    return ret;
    }
// For out buffer align_size is same as in buffer
    ret = stm32_cryp_truncate_sg(&cryp.out_sg, &cryp.out_sg_len, out_sg, skip, align_size);
    if (ret) {
    kfree(cryp.header_sg);
    kfree(cryp.in_sg);
    return ret;
    }
    ret = stm32_cryp_dma_check_sg(cryp.in_sg, align_size, AES_BLOCK_SIZE);
    ret2 = stm32_cryp_dma_check_sg(cryp.out_sg, align_size, AES_BLOCK_SIZE);
    if (ret == NO_DMA || ret2 == NO_DMA) {
    kfree(cryp.in_sg);
    cryp.in_sg = core::ptr::null_mut();
    kfree(cryp.out_sg);
    cryp.out_sg = core::ptr::null_mut();
    cryp.flags &= ~FLG_IN_OUT_DMA;
    } else {
    cryp.flags |= FLG_IN_OUT_DMA;
    }
    return 0;
    }
    static int stm32_cryp_prepare_req(struct skcipher_request *req,
    struct aead_request *areq)
    {
    struct stm32_cryp_ctx *ctx;
    struct stm32_cryp *cryp;
    struct stm32_cryp_reqctx *rctx;
    struct scatterlist *in_sg, *out_sg;
    int ret;
    if (!req && !areq)
    return -EINVAL;
    ctx = req ? crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)) :
    crypto_aead_ctx(crypto_aead_reqtfm(areq));
    cryp = ctx.cryp;
    rctx = req ? skcipher_request_ctx(req) : aead_request_ctx(areq);
    rctx.mode &= FLG_MODE_MASK;
    cryp.flags = (cryp.flags & ~FLG_MODE_MASK) | rctx.mode;
    cryp.hw_blocksize = is_aes(cryp) ? AES_BLOCK_SIZE : DES_BLOCK_SIZE;
    cryp.ctx = ctx;
    if (req) {
    cryp.req = req;
    cryp.areq = core::ptr::null_mut();
    cryp.header_in = 0;
    cryp.payload_in = req.cryptlen;
    cryp.payload_out = req.cryptlen;
    cryp.authsize = 0;
    in_sg = req.src;
    out_sg = req.dst;
    ret = stm32_cryp_cipher_prepare(cryp, in_sg, out_sg);
    if (ret)
    return ret;
    ret = stm32_cryp_hw_init(cryp);
    } else {
//
// Length of input and output data:
// Encryption case:
// INPUT  = AssocData   ||     PlainText
// <- assoclen ->  <- cryptlen ->
//
// OUTPUT = AssocData    ||   CipherText   ||      AuthTag
// <- assoclen ->  <-- cryptlen -->  <- authsize ->
//
// Decryption case:
// INPUT  =  AssocData     ||    CipherTex   ||       AuthTag
// <- assoclen --->  <---------- cryptlen ---------->
//
// OUTPUT = AssocData    ||               PlainText
// <- assoclen ->  <- cryptlen - authsize ->
//
    cryp.areq = areq;
    cryp.req = core::ptr::null_mut();
    cryp.authsize = crypto_aead_authsize(crypto_aead_reqtfm(areq));
    if (is_encrypt(cryp)) {
    cryp.payload_in = areq.cryptlen;
    cryp.header_in = areq.assoclen;
    cryp.payload_out = areq.cryptlen;
    } else {
    cryp.payload_in = areq.cryptlen - cryp.authsize;
    cryp.header_in = areq.assoclen;
    cryp.payload_out = cryp.payload_in;
    }
    in_sg = areq.src;
    out_sg = areq.dst;
    scatterwalk_start(&cryp.in_walk, in_sg);
// In output, jump after assoc data
    scatterwalk_start_at_pos(&cryp.out_walk, out_sg,
    areq.assoclen);
    ret = stm32_cryp_hw_init(cryp);
    if (ret)
    return ret;
    ret = stm32_cryp_aead_prepare(cryp, in_sg, out_sg);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_cipher_one_req(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int stm32_cryp_cipher_one_req(struct crypto_engine *engine, void *areq)
    {
    struct skcipher_request *req = container_of(areq,
    struct skcipher_request,
    base);
    struct stm32_cryp_ctx *ctx = crypto_skcipher_ctx(
    crypto_skcipher_reqtfm(req));
    struct stm32_cryp *cryp = ctx.cryp;
    int ret;
    if (!cryp)
    return -ENODEV;
    ret = stm32_cryp_prepare_req(req, core::ptr::null_mut());
    if (ret)
    return ret;
    if (cryp.flags & FLG_IN_OUT_DMA)
    ret = stm32_cryp_dma_start(cryp);
    else
    ret = stm32_cryp_it_start(cryp);
    if (ret == -ETIMEDOUT)
    stm32_cryp_finish_req(cryp, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_aead_one_req(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int stm32_cryp_aead_one_req(struct crypto_engine *engine, void *areq)
    {
    struct aead_request *req = container_of(areq, struct aead_request,
    base);
    struct stm32_cryp_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    struct stm32_cryp *cryp = ctx.cryp;
    int err;
    if (!cryp)
    return -ENODEV;
    err = stm32_cryp_prepare_req(core::ptr::null_mut(), req);
    if (err)
    return err;
    if (!stm32_cryp_get_input_text_len(cryp) && !cryp.header_in &&
    !(cryp.flags & FLG_HEADER_DMA)) {
// No input data to process: get tag and finish
    stm32_cryp_finish_req(cryp, 0);
    return 0;
    }
    if (cryp.flags & FLG_HEADER_DMA)
    return stm32_cryp_header_dma_start(cryp);
    if (!cryp.header_in && cryp.flags & FLG_IN_OUT_DMA)
    return stm32_cryp_dma_start(cryp);
    return stm32_cryp_it_start(cryp);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_read_auth_tag(cryp: *mut stm32_cryp) -> c_int {
    static int stm32_cryp_read_auth_tag(struct stm32_cryp *cryp)
    {
    u32 cfg, size_bit;
    unsigned int i;
    let mut ret: c_int = 0;
// Update Config
    cfg = stm32_cryp_read(cryp, cryp.caps.cr);
    cfg &= ~CR_PH_MASK;
    cfg |= CR_PH_FINAL;
    cfg &= ~CR_DEC_NOT_ENC;
    cfg |= CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    if (is_gcm(cryp)) {
// GCM: write aad and payload size (in bits)
    size_bit = cryp.areq.assoclen * 8;
    if (cryp.caps.swap_final)
    size_bit = ( u32)cpu_to_be32(size_bit);
    stm32_cryp_write(cryp, cryp.caps.din, 0);
    stm32_cryp_write(cryp, cryp.caps.din, size_bit);
    size_bit = is_encrypt(cryp) ? cryp.areq.cryptlen :
    cryp.areq.cryptlen - cryp.authsize;
    size_bit *= 8;
    if (cryp.caps.swap_final)
    size_bit = ( u32)cpu_to_be32(size_bit);
    stm32_cryp_write(cryp, cryp.caps.din, 0);
    stm32_cryp_write(cryp, cryp.caps.din, size_bit);
    } else {
// CCM: write CTR0
    u32 iv32[AES_BLOCK_32];
    u8 *iv = (u8 *)iv32;
    __be32 *biv = (__be32 *)iv32;
    memcpy(iv, cryp.areq.iv, AES_BLOCK_SIZE);
    memset(iv + AES_BLOCK_SIZE - 1 - iv[0], 0, iv[0] + 1);
    for (i = 0; i < AES_BLOCK_32; i++) {
    let mut xiv: u32 = iv32[i];
    if (!cryp.caps.padding_wa)
    xiv = be32_to_cpu(biv[i]);
    stm32_cryp_write(cryp, cryp.caps.din, xiv);
    }
    }
// Wait for output data
    ret = stm32_cryp_wait_output(cryp);
    if (ret) {
    dev_err(cryp.dev, "Timeout (read tag)\n");
    return ret;
    }
    if (is_encrypt(cryp)) {
    u32 out_tag[AES_BLOCK_32];
// Get and write tag
    readsl(cryp.regs + cryp.caps.dout, out_tag, AES_BLOCK_32);
    memcpy_to_scatterwalk(&cryp.out_walk, out_tag, cryp.authsize);
    } else {
// Get and check tag
    u32 in_tag[AES_BLOCK_32], out_tag[AES_BLOCK_32];
    memcpy_from_scatterwalk(in_tag, &cryp.in_walk, cryp.authsize);
    readsl(cryp.regs + cryp.caps.dout, out_tag, AES_BLOCK_32);
    if (crypto_memneq(in_tag, out_tag, cryp.authsize))
    ret = -EBADMSG;
    }
// Disable cryp
    cfg &= ~CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_check_ctr_counter(cryp: *mut stm32_cryp) {
    static void stm32_cryp_check_ctr_counter(struct stm32_cryp *cryp)
    {
    u32 cr;
    if (unlikely(cryp.last_ctr[3] == cpu_to_be32(0xFFFFFFFF))) {
//
// In this case, we need to increment manually the ctr counter,
// as HW doesn't handle the U32 carry.
//
    crypto_inc((u8 *)cryp.last_ctr, sizeof(cryp.last_ctr));
    cr = stm32_cryp_read(cryp, cryp.caps.cr);
    stm32_cryp_write(cryp, cryp.caps.cr, cr & ~CR_CRYPEN);
    stm32_cryp_hw_write_iv(cryp, cryp.last_ctr);
    stm32_cryp_write(cryp, cryp.caps.cr, cr);
    }
// The IV registers are BE
    cryp.last_ctr[0] = cpu_to_be32(stm32_cryp_read(cryp, cryp.caps.iv0l));
    cryp.last_ctr[1] = cpu_to_be32(stm32_cryp_read(cryp, cryp.caps.iv0r));
    cryp.last_ctr[2] = cpu_to_be32(stm32_cryp_read(cryp, cryp.caps.iv1l));
    cryp.last_ctr[3] = cpu_to_be32(stm32_cryp_read(cryp, cryp.caps.iv1r));
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_irq_read_data(cryp: *mut stm32_cryp) {
    static void stm32_cryp_irq_read_data(struct stm32_cryp *cryp)
    {
    u32 block[AES_BLOCK_32];
    readsl(cryp.regs + cryp.caps.dout, block, cryp.hw_blocksize / sizeof(u32));
    memcpy_to_scatterwalk(&cryp.out_walk, block, min(cryp.hw_blocksize,
    cryp.payload_out));
    cryp.payload_out -= min(cryp.hw_blocksize, cryp.payload_out);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_irq_write_block(cryp: *mut stm32_cryp) {
    static void stm32_cryp_irq_write_block(struct stm32_cryp *cryp)
    {
    u32 block[AES_BLOCK_32] = {0};
    memcpy_from_scatterwalk(block, &cryp.in_walk, min(cryp.hw_blocksize,
    cryp.payload_in));
    writesl(cryp.regs + cryp.caps.din, block, cryp.hw_blocksize / sizeof(u32));
    cryp.payload_in -= min(cryp.hw_blocksize, cryp.payload_in);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_irq_write_gcm_padded_data(cryp: *mut stm32_cryp) {
    static void stm32_cryp_irq_write_gcm_padded_data(struct stm32_cryp *cryp)
    {
    int err;
    u32 cfg, block[AES_BLOCK_32] = {0};
    unsigned int i;
// 'Special workaround' procedure described in the datasheet
// a) disable ip
    stm32_cryp_write(cryp, cryp.caps.imsc, 0);
    cfg = stm32_cryp_read(cryp, cryp.caps.cr);
    cfg &= ~CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// b) Update IV1R
    stm32_cryp_write(cryp, cryp.caps.iv1r, cryp.gcm_ctr - 2);
// c) change mode to CTR
    cfg &= ~CR_ALGO_MASK;
    cfg |= CR_AES_CTR;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// a) enable IP
    cfg |= CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// b) pad and write the last block
    stm32_cryp_irq_write_block(cryp);
// wait end of process
    err = stm32_cryp_wait_output(cryp);
    if (err) {
    dev_err(cryp.dev, "Timeout (write gcm last data)\n");
    return stm32_cryp_finish_req(cryp, err);
    }
// c) get and store encrypted data
//
// Same code as stm32_cryp_irq_read_data(), but we want to store
// block value
//
    readsl(cryp.regs + cryp.caps.dout, block, cryp.hw_blocksize / sizeof(u32));
    memcpy_to_scatterwalk(&cryp.out_walk, block, min(cryp.hw_blocksize,
    cryp.payload_out));
    cryp.payload_out -= min(cryp.hw_blocksize, cryp.payload_out);
// d) change mode back to AES GCM
    cfg &= ~CR_ALGO_MASK;
    cfg |= CR_AES_GCM;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// e) change phase to Final
    cfg &= ~CR_PH_MASK;
    cfg |= CR_PH_FINAL;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// f) write padded data
    writesl(cryp.regs + cryp.caps.din, block, AES_BLOCK_32);
// g) Empty fifo out
    err = stm32_cryp_wait_output(cryp);
    if (err) {
    dev_err(cryp.dev, "Timeout (write gcm padded data)\n");
    return stm32_cryp_finish_req(cryp, err);
    }
    for (i = 0; i < AES_BLOCK_32; i++)
    stm32_cryp_read(cryp, cryp.caps.dout);
// h) run the he normal Final phase
    stm32_cryp_finish_req(cryp, 0);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_irq_set_npblb(cryp: *mut stm32_cryp) {
    static void stm32_cryp_irq_set_npblb(struct stm32_cryp *cryp)
    {
    u32 cfg;
// disable ip, set NPBLB and reneable ip
    cfg = stm32_cryp_read(cryp, cryp.caps.cr);
    cfg &= ~CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    cfg |= (cryp.hw_blocksize - cryp.payload_in) << CR_NBPBL_SHIFT;
    cfg |= CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_irq_write_ccm_padded_data(cryp: *mut stm32_cryp) {
    static void stm32_cryp_irq_write_ccm_padded_data(struct stm32_cryp *cryp)
    {
    let mut err: c_int = 0;
    u32 cfg, iv1tmp;
    u32 cstmp1[AES_BLOCK_32], cstmp2[AES_BLOCK_32];
    u32 block[AES_BLOCK_32] = {0};
    unsigned int i;
// 'Special workaround' procedure described in the datasheet
// a) disable ip
    stm32_cryp_write(cryp, cryp.caps.imsc, 0);
    cfg = stm32_cryp_read(cryp, cryp.caps.cr);
    cfg &= ~CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// b) get IV1 from CRYP_CSGCMCCM7
    iv1tmp = stm32_cryp_read(cryp, CRYP_CSGCMCCM0R + 7 * 4);
// c) Load CRYP_CSGCMCCMxR
    for (i = 0; i < ARRAY_SIZE(cstmp1); i++)
    cstmp1[i] = stm32_cryp_read(cryp, CRYP_CSGCMCCM0R + i * 4);
// d) Write IV1R
    stm32_cryp_write(cryp, cryp.caps.iv1r, iv1tmp);
// e) change mode to CTR
    cfg &= ~CR_ALGO_MASK;
    cfg |= CR_AES_CTR;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// a) enable IP
    cfg |= CR_CRYPEN;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// b) pad and write the last block
    stm32_cryp_irq_write_block(cryp);
// wait end of process
    err = stm32_cryp_wait_output(cryp);
    if (err) {
    dev_err(cryp.dev, "Timeout (write ccm padded data)\n");
    return stm32_cryp_finish_req(cryp, err);
    }
// c) get and store decrypted data
//
// Same code as stm32_cryp_irq_read_data(), but we want to store
// block value
//
    readsl(cryp.regs + cryp.caps.dout, block, cryp.hw_blocksize / sizeof(u32));
    memcpy_to_scatterwalk(&cryp.out_walk, block, min(cryp.hw_blocksize,
    cryp.payload_out));
    cryp.payload_out -= min(cryp.hw_blocksize, cryp.payload_out);
// d) Load again CRYP_CSGCMCCMxR
    for (i = 0; i < ARRAY_SIZE(cstmp2); i++)
    cstmp2[i] = stm32_cryp_read(cryp, CRYP_CSGCMCCM0R + i * 4);
// e) change mode back to AES CCM
    cfg &= ~CR_ALGO_MASK;
    cfg |= CR_AES_CCM;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// f) change phase to header
    cfg &= ~CR_PH_MASK;
    cfg |= CR_PH_HEADER;
    stm32_cryp_write(cryp, cryp.caps.cr, cfg);
// g) XOR and write padded data
    for (i = 0; i < ARRAY_SIZE(block); i++) {
    block[i] ^= cstmp1[i];
    block[i] ^= cstmp2[i];
    stm32_cryp_write(cryp, cryp.caps.din, block[i]);
    }
// h) wait for completion
    err = stm32_cryp_wait_busy(cryp);
    if (err)
    dev_err(cryp.dev, "Timeout (write ccm padded data)\n");
// i) run the he normal Final phase
    stm32_cryp_finish_req(cryp, err);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_irq_write_data(cryp: *mut stm32_cryp) {
    static void stm32_cryp_irq_write_data(struct stm32_cryp *cryp)
    {
    if (unlikely(!cryp.payload_in)) {
    dev_warn(cryp.dev, "No more data to process\n");
    return;
    }
    if (unlikely(cryp.payload_in < AES_BLOCK_SIZE &&
    (stm32_cryp_get_hw_mode(cryp) == CR_AES_GCM) &&
    is_encrypt(cryp))) {
// Padding for AES GCM encryption
    if (cryp.caps.padding_wa) {
// Special case 1
    stm32_cryp_irq_write_gcm_padded_data(cryp);
    return;
    }
// Setting padding bytes (NBBLB)
    stm32_cryp_irq_set_npblb(cryp);
    }
    if (unlikely((cryp.payload_in < AES_BLOCK_SIZE) &&
    (stm32_cryp_get_hw_mode(cryp) == CR_AES_CCM) &&
    is_decrypt(cryp))) {
// Padding for AES CCM decryption
    if (cryp.caps.padding_wa) {
// Special case 2
    stm32_cryp_irq_write_ccm_padded_data(cryp);
    return;
    }
// Setting padding bytes (NBBLB)
    stm32_cryp_irq_set_npblb(cryp);
    }
    if (is_aes(cryp) && is_ctr(cryp))
    stm32_cryp_check_ctr_counter(cryp);
    stm32_cryp_irq_write_block(cryp);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_irq_write_gcmccm_header(cryp: *mut stm32_cryp) {
    static void stm32_cryp_irq_write_gcmccm_header(struct stm32_cryp *cryp)
    {
    u32 block[AES_BLOCK_32] = {0};
    size_t written;
    written = min(AES_BLOCK_SIZE, cryp.header_in);
    memcpy_from_scatterwalk(block, &cryp.in_walk, written);
    writesl(cryp.regs + cryp.caps.din, block, AES_BLOCK_32);
    cryp.header_in -= written;
    stm32_crypt_gcmccm_end_header(cryp);
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_irq_thread(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_cryp_irq_thread(int irq, void *arg)
    {
    struct stm32_cryp *cryp = arg;
    u32 ph;
    let mut it_mask: u32 = stm32_cryp_read(cryp, cryp.caps.imsc);
    if (cryp.irq_status & MISR_OUT)
// Output FIFO IRQ: read data
    stm32_cryp_irq_read_data(cryp);
    if (cryp.irq_status & MISR_IN) {
    if (is_gcm(cryp) || is_ccm(cryp)) {
    ph = stm32_cryp_read(cryp, cryp.caps.cr) & CR_PH_MASK;
    if (unlikely(ph == CR_PH_HEADER))
// Write Header
    stm32_cryp_irq_write_gcmccm_header(cryp);
    else
// Input FIFO IRQ: write data
    stm32_cryp_irq_write_data(cryp);
    if (is_gcm(cryp))
    cryp.gcm_ctr++;
    } else {
// Input FIFO IRQ: write data
    stm32_cryp_irq_write_data(cryp);
    }
    }
// Mask useless interrupts
    if (!cryp.payload_in && !cryp.header_in)
    it_mask &= ~IMSCR_IN;
    if (!cryp.payload_out)
    it_mask &= ~IMSCR_OUT;
    stm32_cryp_write(cryp, cryp.caps.imsc, it_mask);
    if (!cryp.payload_in && !cryp.header_in && !cryp.payload_out) {
    local_bh_disable();
    stm32_cryp_finish_req(cryp, 0);
    local_bh_enable();
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_irq(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_cryp_irq(int irq, void *arg)
    {
    struct stm32_cryp *cryp = arg;
    cryp.irq_status = stm32_cryp_read(cryp, cryp.caps.mis);
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_dma_init(cryp: *mut stm32_cryp) -> c_int {
    static int stm32_cryp_dma_init(struct stm32_cryp *cryp)
    {
    struct dma_slave_config dma_conf;
    struct dma_chan *chan;
    int ret;
    memset(&dma_conf, 0, sizeof(dma_conf));
    dma_conf.direction = DMA_MEM_TO_DEV;
    dma_conf.dst_addr = cryp.phys_base + cryp.caps.din;
    dma_conf.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dma_conf.dst_maxburst = CRYP_DMA_BURST_REG;
    dma_conf.device_fc = false;
    chan = dma_request_chan(cryp.dev, "in");
    if (IS_ERR(chan))
    return PTR_ERR(chan);
    cryp.dma_lch_in = chan;
    ret = dmaengine_slave_config(cryp.dma_lch_in, &dma_conf);
    if (ret) {
    dma_release_channel(cryp.dma_lch_in);
    cryp.dma_lch_in = core::ptr::null_mut();
    dev_err(cryp.dev, "Couldn't configure DMA in slave.\n");
    return ret;
    }
    memset(&dma_conf, 0, sizeof(dma_conf));
    dma_conf.direction = DMA_DEV_TO_MEM;
    dma_conf.src_addr = cryp.phys_base + cryp.caps.dout;
    dma_conf.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dma_conf.src_maxburst = CRYP_DMA_BURST_REG;
    dma_conf.device_fc = false;
    chan = dma_request_chan(cryp.dev, "out");
    if (IS_ERR(chan)) {
    dma_release_channel(cryp.dma_lch_in);
    cryp.dma_lch_in = core::ptr::null_mut();
    return PTR_ERR(chan);
    }
    cryp.dma_lch_out = chan;
    ret = dmaengine_slave_config(cryp.dma_lch_out, &dma_conf);
    if (ret) {
    dma_release_channel(cryp.dma_lch_out);
    cryp.dma_lch_out = core::ptr::null_mut();
    dev_err(cryp.dev, "Couldn't configure DMA out slave.\n");
    dma_release_channel(cryp.dma_lch_in);
    cryp.dma_lch_in = core::ptr::null_mut();
    return ret;
    }
    init_completion(&cryp.dma_completion);
    return 0;
    }
    static struct skcipher_engine_alg crypto_algs[] = {
    {
    .base = {
    .base.cra_name		= "ecb(aes)",
    .base.cra_driver_name	= "stm32-ecb-aes",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct stm32_cryp_ctx),
    .base.cra_alignmask	= 0,
    .base.cra_module	= THIS_MODULE,
    .init			= stm32_cryp_init_tfm,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .setkey			= stm32_cryp_aes_setkey,
    .encrypt		= stm32_cryp_aes_ecb_encrypt,
    .decrypt		= stm32_cryp_aes_ecb_decrypt,
    },
    .op = {
    .do_one_request = stm32_cryp_cipher_one_req,
    },
    },
    {
    .base = {
    .base.cra_name		= "cbc(aes)",
    .base.cra_driver_name	= "stm32-cbc-aes",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct stm32_cryp_ctx),
    .base.cra_alignmask	= 0,
    .base.cra_module	= THIS_MODULE,
    .init			= stm32_cryp_init_tfm,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .ivsize			= AES_BLOCK_SIZE,
    .setkey			= stm32_cryp_aes_setkey,
    .encrypt		= stm32_cryp_aes_cbc_encrypt,
    .decrypt		= stm32_cryp_aes_cbc_decrypt,
    },
    .op = {
    .do_one_request = stm32_cryp_cipher_one_req,
    },
    },
    {
    .base = {
    .base.cra_name		= "ctr(aes)",
    .base.cra_driver_name	= "stm32-ctr-aes",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY,
    .base.cra_blocksize	= 1,
    .base.cra_ctxsize	= sizeof(struct stm32_cryp_ctx),
    .base.cra_alignmask	= 0,
    .base.cra_module	= THIS_MODULE,
    .init			= stm32_cryp_init_tfm,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .ivsize			= AES_BLOCK_SIZE,
    .setkey			= stm32_cryp_aes_setkey,
    .encrypt		= stm32_cryp_aes_ctr_encrypt,
    .decrypt		= stm32_cryp_aes_ctr_decrypt,
    },
    .op = {
    .do_one_request = stm32_cryp_cipher_one_req,
    },
    },
    {
    .base = {
    .base.cra_name		= "ecb(des)",
    .base.cra_driver_name	= "stm32-ecb-des",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY,
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct stm32_cryp_ctx),
    .base.cra_alignmask	= 0,
    .base.cra_module	= THIS_MODULE,
    .init			= stm32_cryp_init_tfm,
    .min_keysize		= DES_BLOCK_SIZE,
    .max_keysize		= DES_BLOCK_SIZE,
    .setkey			= stm32_cryp_des_setkey,
    .encrypt		= stm32_cryp_des_ecb_encrypt,
    .decrypt		= stm32_cryp_des_ecb_decrypt,
    },
    .op = {
    .do_one_request = stm32_cryp_cipher_one_req,
    },
    },
    {
    .base = {
    .base.cra_name		= "cbc(des)",
    .base.cra_driver_name	= "stm32-cbc-des",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY,
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct stm32_cryp_ctx),
    .base.cra_alignmask	= 0,
    .base.cra_module	= THIS_MODULE,
    .init			= stm32_cryp_init_tfm,
    .min_keysize		= DES_BLOCK_SIZE,
    .max_keysize		= DES_BLOCK_SIZE,
    .ivsize			= DES_BLOCK_SIZE,
    .setkey			= stm32_cryp_des_setkey,
    .encrypt		= stm32_cryp_des_cbc_encrypt,
    .decrypt		= stm32_cryp_des_cbc_decrypt,
    },
    .op = {
    .do_one_request = stm32_cryp_cipher_one_req,
    },
    },
    {
    .base = {
    .base.cra_name		= "ecb(des3_ede)",
    .base.cra_driver_name	= "stm32-ecb-des3",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY,
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct stm32_cryp_ctx),
    .base.cra_alignmask	= 0,
    .base.cra_module	= THIS_MODULE,
    .init			= stm32_cryp_init_tfm,
    .min_keysize		= 3 * DES_BLOCK_SIZE,
    .max_keysize		= 3 * DES_BLOCK_SIZE,
    .setkey			= stm32_cryp_tdes_setkey,
    .encrypt		= stm32_cryp_tdes_ecb_encrypt,
    .decrypt		= stm32_cryp_tdes_ecb_decrypt,
    },
    .op = {
    .do_one_request = stm32_cryp_cipher_one_req,
    },
    },
    {
    .base = {
    .base.cra_name		= "cbc(des3_ede)",
    .base.cra_driver_name	= "stm32-cbc-des3",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY,
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct stm32_cryp_ctx),
    .base.cra_alignmask	= 0,
    .base.cra_module	= THIS_MODULE,
    .init			= stm32_cryp_init_tfm,
    .min_keysize		= 3 * DES_BLOCK_SIZE,
    .max_keysize		= 3 * DES_BLOCK_SIZE,
    .ivsize			= DES_BLOCK_SIZE,
    .setkey			= stm32_cryp_tdes_setkey,
    .encrypt		= stm32_cryp_tdes_cbc_encrypt,
    .decrypt		= stm32_cryp_tdes_cbc_decrypt,
    },
    .op = {
    .do_one_request = stm32_cryp_cipher_one_req,
    },
    },
    };
    static struct aead_engine_alg aead_algs[] = {
    {
    .base.setkey		= stm32_cryp_aes_aead_setkey,
    .base.setauthsize	= stm32_cryp_aes_gcm_setauthsize,
    .base.encrypt		= stm32_cryp_aes_gcm_encrypt,
    .base.decrypt		= stm32_cryp_aes_gcm_decrypt,
    .base.init		= stm32_cryp_aes_aead_init,
    .base.ivsize		= 12,
    .base.maxauthsize	= AES_BLOCK_SIZE,
    .base.base = {
    .cra_name		= "gcm(aes)",
    .cra_driver_name	= "stm32-gcm-aes",
    .cra_priority		= 300,
    .cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct stm32_cryp_ctx),
    .cra_alignmask		= 0,
    .cra_module		= THIS_MODULE,
    },
    .op = {
    .do_one_request = stm32_cryp_aead_one_req,
    },
    },
    {
    .base.setkey		= stm32_cryp_aes_aead_setkey,
    .base.setauthsize	= stm32_cryp_aes_ccm_setauthsize,
    .base.encrypt		= stm32_cryp_aes_ccm_encrypt,
    .base.decrypt		= stm32_cryp_aes_ccm_decrypt,
    .base.init		= stm32_cryp_aes_aead_init,
    .base.ivsize		= AES_BLOCK_SIZE,
    .base.maxauthsize	= AES_BLOCK_SIZE,
    .base.base = {
    .cra_name		= "ccm(aes)",
    .cra_driver_name	= "stm32-ccm-aes",
    .cra_priority		= 300,
    .cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct stm32_cryp_ctx),
    .cra_alignmask		= 0,
    .cra_module		= THIS_MODULE,
    },
    .op = {
    .do_one_request = stm32_cryp_aead_one_req,
    },
    },
    };
    static const struct stm32_cryp_caps ux500_data = {
    .aeads_support = false,
    .linear_aes_key = true,
    .kp_mode = false,
    .iv_protection = true,
    .swap_final = true,
    .padding_wa = true,
    .cr = UX500_CRYP_CR,
    .sr = UX500_CRYP_SR,
    .din = UX500_CRYP_DIN,
    .dout = UX500_CRYP_DOUT,
    .dmacr = UX500_CRYP_DMACR,
    .imsc = UX500_CRYP_IMSC,
    .mis = UX500_CRYP_MIS,
    .k1l = UX500_CRYP_K1L,
    .k1r = UX500_CRYP_K1R,
    .k3r = UX500_CRYP_K3R,
    .iv0l = UX500_CRYP_IV0L,
    .iv0r = UX500_CRYP_IV0R,
    .iv1l = UX500_CRYP_IV1L,
    .iv1r = UX500_CRYP_IV1R,
    };
    static const struct stm32_cryp_caps f7_data = {
    .aeads_support = true,
    .linear_aes_key = false,
    .kp_mode = true,
    .iv_protection = false,
    .swap_final = true,
    .padding_wa = true,
    .cr = CRYP_CR,
    .sr = CRYP_SR,
    .din = CRYP_DIN,
    .dout = CRYP_DOUT,
    .dmacr = CRYP_DMACR,
    .imsc = CRYP_IMSCR,
    .mis = CRYP_MISR,
    .k1l = CRYP_K1LR,
    .k1r = CRYP_K1RR,
    .k3r = CRYP_K3RR,
    .iv0l = CRYP_IV0LR,
    .iv0r = CRYP_IV0RR,
    .iv1l = CRYP_IV1LR,
    .iv1r = CRYP_IV1RR,
    };
    static const struct stm32_cryp_caps mp1_data = {
    .aeads_support = true,
    .linear_aes_key = false,
    .kp_mode = true,
    .iv_protection = false,
    .swap_final = false,
    .padding_wa = false,
    .cr = CRYP_CR,
    .sr = CRYP_SR,
    .din = CRYP_DIN,
    .dout = CRYP_DOUT,
    .dmacr = CRYP_DMACR,
    .imsc = CRYP_IMSCR,
    .mis = CRYP_MISR,
    .k1l = CRYP_K1LR,
    .k1r = CRYP_K1RR,
    .k3r = CRYP_K3RR,
    .iv0l = CRYP_IV0LR,
    .iv0r = CRYP_IV0RR,
    .iv1l = CRYP_IV1LR,
    .iv1r = CRYP_IV1RR,
    };
    static const struct of_device_id stm32_dt_ids[] = {
    { .compatible = "stericsson,ux500-cryp", .data = &ux500_data},
    { .compatible = "st,stm32f756-cryp", .data = &f7_data},
    { .compatible = "st,stm32mp1-cryp", .data = &mp1_data},
    {},
    };
    MODULE_DEVICE_TABLE(of, stm32_dt_ids);
#[no_mangle]
unsafe extern "C" fn stm32_cryp_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_cryp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct stm32_cryp *cryp;
    struct reset_control *rst;
    int irq, ret;
    cryp = devm_kzalloc(dev, sizeof(*cryp), GFP_KERNEL);
    if (!cryp)
    return -ENOMEM;
    cryp.caps = of_device_get_match_data(dev);
    if (!cryp.caps)
    return -ENODEV;
    cryp.dev = dev;
    cryp.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(cryp.regs))
    return PTR_ERR(cryp.regs);
    cryp.phys_base = platform_get_resource(pdev, IORESOURCE_MEM, 0).start;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, stm32_cryp_irq,
    stm32_cryp_irq_thread, IRQF_ONESHOT,
    dev_name(dev), cryp);
    if (ret)
    return ret;
    cryp.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(cryp.clk)) {
    dev_err_probe(dev, PTR_ERR(cryp.clk), "Could not get clock\n");
    return PTR_ERR(cryp.clk);
    }
    ret = clk_prepare_enable(cryp.clk);
    if (ret) {
    dev_err(cryp.dev, "Failed to enable clock\n");
    return ret;
    }
    pm_runtime_set_autosuspend_delay(dev, CRYP_AUTOSUSPEND_DELAY);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    rst = devm_reset_control_get(dev, core::ptr::null_mut());
    if (IS_ERR(rst)) {
    ret = PTR_ERR(rst);
    if (ret == -EPROBE_DEFER)
    goto err_rst;
    } else {
    reset_control_assert(rst);
    udelay(2);
    reset_control_deassert(rst);
    }
    platform_set_drvdata(pdev, cryp);
    ret = stm32_cryp_dma_init(cryp);
    switch (ret) {
    case 0:
    break;
    case -ENODEV:
    dev_dbg(dev, "DMA mode not available\n");
    break;
    default:
    goto err_dma;
    }
    spin_lock(&cryp_list.lock);
    list_add(&cryp.list, &cryp_list.dev_list);
    spin_unlock(&cryp_list.lock);
// Initialize crypto engine
    cryp.engine = crypto_engine_alloc_init(dev, 1);
    if (!cryp.engine) {
    dev_err(dev, "Could not init crypto engine\n");
    ret = -ENOMEM;
    goto err_engine1;
    }
    ret = crypto_engine_start(cryp.engine);
    if (ret) {
    dev_err(dev, "Could not start crypto engine\n");
    goto err_engine2;
    }
    ret = crypto_engine_register_skciphers(crypto_algs, ARRAY_SIZE(crypto_algs));
    if (ret) {
    dev_err(dev, "Could not register algs\n");
    goto err_algs;
    }
    if (cryp.caps.aeads_support) {
    ret = crypto_engine_register_aeads(aead_algs, ARRAY_SIZE(aead_algs));
    if (ret)
    goto err_aead_algs;
    }
    dev_info(dev, "Initialized\n");
    pm_runtime_put_sync(dev);
    return 0;
    err_aead_algs:
    crypto_engine_unregister_skciphers(crypto_algs, ARRAY_SIZE(crypto_algs));
    err_algs:
    err_engine2:
    crypto_engine_exit(cryp.engine);
    err_engine1:
    spin_lock(&cryp_list.lock);
    list_del(&cryp.list);
    spin_unlock(&cryp_list.lock);
    if (cryp.dma_lch_in)
    dma_release_channel(cryp.dma_lch_in);
    if (cryp.dma_lch_out)
    dma_release_channel(cryp.dma_lch_out);
    err_dma:
    err_rst:
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    clk_disable_unprepare(cryp.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_remove(pdev: *mut platform_device) {
    static void stm32_cryp_remove(struct platform_device *pdev)
    {
    struct stm32_cryp *cryp = platform_get_drvdata(pdev);
    int ret;
    ret = pm_runtime_get_sync(cryp.dev);
    if (cryp.caps.aeads_support)
    crypto_engine_unregister_aeads(aead_algs, ARRAY_SIZE(aead_algs));
    crypto_engine_unregister_skciphers(crypto_algs, ARRAY_SIZE(crypto_algs));
    crypto_engine_exit(cryp.engine);
    spin_lock(&cryp_list.lock);
    list_del(&cryp.list);
    spin_unlock(&cryp_list.lock);
    if (cryp.dma_lch_in)
    dma_release_channel(cryp.dma_lch_in);
    if (cryp.dma_lch_out)
    dma_release_channel(cryp.dma_lch_out);
    pm_runtime_disable(cryp.dev);
    pm_runtime_put_noidle(cryp.dev);
    if (ret >= 0)
    clk_disable_unprepare(cryp.clk);
    }

#[no_mangle]
unsafe extern "C" fn stm32_cryp_runtime_suspend(dev: *mut device) -> c_int {
    static int stm32_cryp_runtime_suspend(struct device *dev)
    {
    struct stm32_cryp *cryp = dev_get_drvdata(dev);
    clk_disable_unprepare(cryp.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cryp_runtime_resume(dev: *mut device) -> c_int {
    static int stm32_cryp_runtime_resume(struct device *dev)
    {
    struct stm32_cryp *cryp = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(cryp.clk);
    if (ret) {
    dev_err(cryp.dev, "Failed to prepare_enable clock\n");
    return ret;
    }
    return 0;
    }

    static const struct dev_pm_ops stm32_cryp_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    SET_RUNTIME_PM_OPS(stm32_cryp_runtime_suspend,
    stm32_cryp_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver stm32_cryp_driver = {
    .probe  = stm32_cryp_probe,
    .remove = stm32_cryp_remove,
    .driver = {
    .name           = DRIVER_NAME,
    .pm		= &stm32_cryp_pm_ops,
    .of_match_table = stm32_dt_ids,
    },
    };
    module_platform_driver(stm32_cryp_driver);
    MODULE_AUTHOR("Fabien Dessenne <fabien.dessenne@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STM32 CRYP hardware driver");
    MODULE_LICENSE("GPL");
