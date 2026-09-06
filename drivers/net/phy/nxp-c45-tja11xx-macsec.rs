//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/nxp-c45-tja11xx-macsec.c
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
// NXP C45 PTP PHY driver interface
// Copyright 2023 NXP
// Author: Radu Pirea <radu-nicolae.pirea@oss.nxp.com>
//

pub const MACSEC_REG_SIZE: c_int = 32;
pub const TX_SC_MAX: c_int = 4;

pub const VEND1_MACSEC_BASE: c_uint = 0x9000;
pub const MACSEC_CFG: c_uint = 0x0000;

pub const MACSEC_TPNET: c_uint = 0x0044;
pub const PN_WRAP_THRESHOLD: c_uint = 0xffffffff;
pub const MACSEC_RXSCA: c_uint = 0x0080;
pub const MACSEC_RXSCKA: c_uint = 0x0084;
pub const MACSEC_TXSCA: c_uint = 0x00C0;
pub const MACSEC_TXSCKA: c_uint = 0x00C4;
pub const MACSEC_RXSC_SCI_1H: c_uint = 0x0100;
pub const MACSEC_RXSC_CFG: c_uint = 0x0128;

pub const MACSEC_RXSC_CFG_VF_OFF: c_int = 8;
pub const MACSEC_RPW: c_uint = 0x012C;
pub const MACSEC_RXSA_A_CS: c_uint = 0x0180;
pub const MACSEC_RXSA_A_NPN: c_uint = 0x0184;
pub const MACSEC_RXSA_A_XNPN: c_uint = 0x0188;
pub const MACSEC_RXSA_A_LNPN: c_uint = 0x018C;
pub const MACSEC_RXSA_A_LXNPN: c_uint = 0x0190;
pub const MACSEC_RXSA_B_CS: c_uint = 0x01C0;
pub const MACSEC_RXSA_B_NPN: c_uint = 0x01C4;
pub const MACSEC_RXSA_B_XNPN: c_uint = 0x01C8;
pub const MACSEC_RXSA_B_LNPN: c_uint = 0x01CC;
pub const MACSEC_RXSA_B_LXNPN: c_uint = 0x01D0;
pub const MACSEC_RXSA_CS_AN_OFF: c_int = 1;

pub const MACSEC_TXSC_SCI_1H: c_uint = 0x0200;
pub const MACSEC_TXSC_CFG: c_uint = 0x0228;

pub const MACSEC_TXSC_CFG_AN_OFF: c_int = 18;

pub const MACSEC_TXSA_A_CS: c_uint = 0x0280;
pub const MACSEC_TXSA_A_NPN: c_uint = 0x0284;
pub const MACSEC_TXSA_A_XNPN: c_uint = 0x0288;
pub const MACSEC_TXSA_B_CS: c_uint = 0x02C0;
pub const MACSEC_TXSA_B_NPN: c_uint = 0x02C4;
pub const MACSEC_TXSA_B_XNPN: c_uint = 0x02C8;

pub const MACSEC_EVR: c_uint = 0x0400;
pub const MACSEC_EVER: c_uint = 0x0404;
pub const MACSEC_RXSA_A_KA: c_uint = 0x0700;
pub const MACSEC_RXSA_A_SSCI: c_uint = 0x0720;
pub const MACSEC_RXSA_A_SALT: c_uint = 0x0724;
pub const MACSEC_RXSA_B_KA: c_uint = 0x0740;
pub const MACSEC_RXSA_B_SSCI: c_uint = 0x0760;
pub const MACSEC_RXSA_B_SALT: c_uint = 0x0764;
pub const MACSEC_TXSA_A_KA: c_uint = 0x0780;
pub const MACSEC_TXSA_A_SSCI: c_uint = 0x07A0;
pub const MACSEC_TXSA_A_SALT: c_uint = 0x07A4;
pub const MACSEC_TXSA_B_KA: c_uint = 0x07C0;
pub const MACSEC_TXSA_B_SSCI: c_uint = 0x07E0;
pub const MACSEC_TXSA_B_SALT: c_uint = 0x07E4;
pub const MACSEC_UPFR0D2: c_uint = 0x0A08;
pub const MACSEC_UPFR0M1: c_uint = 0x0A10;

pub const MACSEC_UPFR0M2: c_uint = 0x0A14;
pub const ETYPE_MASK: c_uint = 0xffff;
pub const MACSEC_UPFR0R: c_uint = 0x0A18;

pub const ADPTR_CNTRL: c_uint = 0x0F00;

pub const ADPTR_TX_TAG_CNTRL: c_uint = 0x0F0C;

pub const TX_SC_FLT_BASE: c_uint = 0x800;
pub const TX_SC_FLT_SIZE: c_uint = 0x10;

    TX_SC_FLT_SIZE * (flt_id))
pub const TX_SC_FLT_OFF_MAC_DA_SA: c_uint = 0x04;
pub const TX_SC_FLT_OFF_MAC_SA: c_uint = 0x08;
pub const TX_SC_FLT_OFF_MAC_CFG: c_uint = 0x0C;

pub const MACSEC_INOV1HS: c_uint = 0x0140;
pub const MACSEC_INOV2HS: c_uint = 0x0144;
pub const MACSEC_INOD1HS: c_uint = 0x0148;
pub const MACSEC_INOD2HS: c_uint = 0x014C;
pub const MACSEC_RXSCIPUS: c_uint = 0x0150;
pub const MACSEC_RXSCIPDS: c_uint = 0x0154;
pub const MACSEC_RXSCIPLS: c_uint = 0x0158;
pub const MACSEC_RXAN0INUSS: c_uint = 0x0160;
pub const MACSEC_RXAN0IPUSS: c_uint = 0x0170;
pub const MACSEC_RXSA_A_IPOS: c_uint = 0x0194;
pub const MACSEC_RXSA_A_IPIS: c_uint = 0x01B0;
pub const MACSEC_RXSA_A_IPNVS: c_uint = 0x01B4;
pub const MACSEC_RXSA_B_IPOS: c_uint = 0x01D4;
pub const MACSEC_RXSA_B_IPIS: c_uint = 0x01F0;
pub const MACSEC_RXSA_B_IPNVS: c_uint = 0x01F4;
pub const MACSEC_OPUS: c_uint = 0x021C;
pub const MACSEC_OPTLS: c_uint = 0x022C;
pub const MACSEC_OOP1HS: c_uint = 0x0240;
pub const MACSEC_OOP2HS: c_uint = 0x0244;
pub const MACSEC_OOE1HS: c_uint = 0x0248;
pub const MACSEC_OOE2HS: c_uint = 0x024C;
pub const MACSEC_TXSA_A_OPPS: c_uint = 0x028C;
pub const MACSEC_TXSA_A_OPES: c_uint = 0x0290;
pub const MACSEC_TXSA_B_OPPS: c_uint = 0x02CC;
pub const MACSEC_TXSA_B_OPES: c_uint = 0x02D0;
pub const MACSEC_INPWTS: c_uint = 0x0630;
pub const MACSEC_INPBTS: c_uint = 0x0638;
pub const MACSEC_IPSNFS: c_uint = 0x063C;

    enum nxp_c45_sa_type {
    TX_SA,
    RX_SA,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_c45_sa {
    pub sa: *mut c_void,
    pub regs: *const nxp_c45_sa_regs,
    pub type: enum nxp_c45_sa_type,
    pub is_key_a: bool,
    pub an: u8,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_c45_secy {
    pub secy: *mut macsec_secy,
    pub rx_sc: *mut macsec_rx_sc,
    pub sa_list: list_head,
    pub secy_id: c_int,
    pub rx_sc0_impl: bool,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_c45_macsec {
    pub secy_list: list_head,
    pub TX_SC_MAX): DECLARE_BITMAP(secy_bitmap,,
    pub TX_SC_MAX): DECLARE_BITMAP(tx_sc_bitmap,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_c45_sa_regs {
    pub cs: u16,
    pub npn: u16,
    pub xnpn: u16,
    pub lnpn: u16,
    pub lxnpn: u16,
    pub ka: u16,
    pub ssci: u16,
    pub salt: u16,
    pub ipis: u16,
    pub ipnvs: u16,
    pub ipos: u16,
    pub opps: u16,
    pub opes: u16,
}

    static const struct nxp_c45_sa_regs rx_sa_a_regs = {
    .cs	= MACSEC_RXSA_A_CS,
    .npn	= MACSEC_RXSA_A_NPN,
    .xnpn	= MACSEC_RXSA_A_XNPN,
    .lnpn	= MACSEC_RXSA_A_LNPN,
    .lxnpn	= MACSEC_RXSA_A_LXNPN,
    .ka	= MACSEC_RXSA_A_KA,
    .ssci	= MACSEC_RXSA_A_SSCI,
    .salt	= MACSEC_RXSA_A_SALT,
    .ipis	= MACSEC_RXSA_A_IPIS,
    .ipnvs	= MACSEC_RXSA_A_IPNVS,
    .ipos	= MACSEC_RXSA_A_IPOS,
    };
    static const struct nxp_c45_sa_regs rx_sa_b_regs = {
    .cs	= MACSEC_RXSA_B_CS,
    .npn	= MACSEC_RXSA_B_NPN,
    .xnpn	= MACSEC_RXSA_B_XNPN,
    .lnpn	= MACSEC_RXSA_B_LNPN,
    .lxnpn	= MACSEC_RXSA_B_LXNPN,
    .ka	= MACSEC_RXSA_B_KA,
    .ssci	= MACSEC_RXSA_B_SSCI,
    .salt	= MACSEC_RXSA_B_SALT,
    .ipis	= MACSEC_RXSA_B_IPIS,
    .ipnvs	= MACSEC_RXSA_B_IPNVS,
    .ipos	= MACSEC_RXSA_B_IPOS,
    };
    static const struct nxp_c45_sa_regs tx_sa_a_regs = {
    .cs	= MACSEC_TXSA_A_CS,
    .npn	= MACSEC_TXSA_A_NPN,
    .xnpn	= MACSEC_TXSA_A_XNPN,
    .ka	= MACSEC_TXSA_A_KA,
    .ssci	= MACSEC_TXSA_A_SSCI,
    .salt	= MACSEC_TXSA_A_SALT,
    .opps	= MACSEC_TXSA_A_OPPS,
    .opes	= MACSEC_TXSA_A_OPES,
    };
    static const struct nxp_c45_sa_regs tx_sa_b_regs = {
    .cs	= MACSEC_TXSA_B_CS,
    .npn	= MACSEC_TXSA_B_NPN,
    .xnpn	= MACSEC_TXSA_B_XNPN,
    .ka	= MACSEC_TXSA_B_KA,
    .ssci	= MACSEC_TXSA_B_SSCI,
    .salt	= MACSEC_TXSA_B_SALT,
    .opps	= MACSEC_TXSA_B_OPPS,
    .opes	= MACSEC_TXSA_B_OPES,
    };
    static const
    struct nxp_c45_sa_regs *nxp_c45_sa_regs_get(enum nxp_c45_sa_type sa_type,
    bool key_a)
    {
    if (sa_type == RX_SA)
    if (key_a)
    return &rx_sa_a_regs;
    else
    return &rx_sa_b_regs;
#[no_mangle]
pub unsafe extern "C" fn if(TX_SA: sa_type ==) -> else {
    else if (sa_type == TX_SA)
    if (key_a)
    return &tx_sa_a_regs;
    else
    return &tx_sa_b_regs;
    else
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_macsec_write(phydev: *mut phy_device, addr: u16, value: u32) -> c_int {
    static int nxp_c45_macsec_write(struct phy_device *phydev, u16 addr, u32 value)
    {
    let mut lvalue: u32 = value;
    u16 laddr;
    int ret;
    WARN_ON_ONCE(addr % 4);
    phydev_dbg(phydev, "write addr 0x%x value 0x%x\n", addr, value);
    laddr = VEND1_MACSEC_BASE + addr / 2;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, laddr, lvalue);
    if (ret)
    return ret;
    laddr += 1;
    lvalue >>= 16;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, laddr, lvalue);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_macsec_read(phydev: *mut phy_device, addr: u16, value: *mut u32) -> c_int {
    static int nxp_c45_macsec_read(struct phy_device *phydev, u16 addr, u32 *value)
    {
    u32 lvalue;
    u16 laddr;
    int ret;
    WARN_ON_ONCE(addr % 4);
    laddr = VEND1_MACSEC_BASE + addr / 2;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, laddr);
    if (ret < 0)
    return ret;
    laddr += 1;
    lvalue = (u32)ret & 0xffff;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, laddr);
    if (ret < 0)
    return ret;
    lvalue |= (u32)ret << 16;
// value = lvalue;
    phydev_dbg(phydev, "read addr 0x%x value 0x%x\n", addr, *value);
    return 0;
    }
    static void nxp_c45_macsec_read32_64(struct phy_device *phydev, u16 addr,
    u64 *value)
    {
    u32 lvalue;
    nxp_c45_macsec_read(phydev, addr, &lvalue);
// value = lvalue;
    }
    static void nxp_c45_macsec_read64(struct phy_device *phydev, u16 addr,
    u64 *value)
    {
    u32 lvalue;
    nxp_c45_macsec_read(phydev, addr, &lvalue);
// value = (u64)lvalue << 32;
    nxp_c45_macsec_read(phydev, addr + 4, &lvalue);
// value |= lvalue;
    }
    static void nxp_c45_secy_irq_en(struct phy_device *phydev,
    struct nxp_c45_secy *phy_secy, bool en)
    {
    u32 reg;
    nxp_c45_macsec_read(phydev, MACSEC_EVER, &reg);
    if (en)
    reg |= TX_SC_BIT(phy_secy.secy_id);
    else
    reg &= ~TX_SC_BIT(phy_secy.secy_id);
    nxp_c45_macsec_write(phydev, MACSEC_EVER, reg);
    }
    static struct nxp_c45_secy *nxp_c45_find_secy(struct list_head *secy_list,
    sci_t sci)
    {
    struct nxp_c45_secy *pos, *tmp;
    list_for_each_entry_safe(pos, tmp, secy_list, list)
    if (pos.secy.sci == sci)
    return pos;
    return ERR_PTR(-EINVAL);
    }
    static struct
    nxp_c45_secy *nxp_c45_find_secy_by_id(struct list_head *secy_list,
    int id)
    {
    struct nxp_c45_secy *pos, *tmp;
    list_for_each_entry_safe(pos, tmp, secy_list, list)
    if (pos.secy_id == id)
    return pos;
    return ERR_PTR(-EINVAL);
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_secy_free(phy_secy: *mut nxp_c45_secy) {
    static void nxp_c45_secy_free(struct nxp_c45_secy *phy_secy)
    {
    list_del(&phy_secy.list);
    kfree(phy_secy);
    }
    static struct nxp_c45_sa *nxp_c45_find_sa(struct list_head *sa_list,
    enum nxp_c45_sa_type sa_type, u8 an)
    {
    struct nxp_c45_sa *pos, *tmp;
    list_for_each_entry_safe(pos, tmp, sa_list, list)
    if (pos.an == an && pos.type == sa_type)
    return pos;
    return ERR_PTR(-EINVAL);
    }
    static struct nxp_c45_sa *nxp_c45_sa_alloc(struct list_head *sa_list, void *sa,
    enum nxp_c45_sa_type sa_type, u8 an)
    {
    struct nxp_c45_sa *first = core::ptr::null_mut(), *pos, *tmp;
    let mut occurrences: c_int = 0;
    list_for_each_entry_safe(pos, tmp, sa_list, list) {
    if (pos.type != sa_type)
    continue;
    if (pos.an == an)
    return ERR_PTR(-EINVAL);
    first = pos;
    occurrences++;
    if (occurrences >= 2)
    return ERR_PTR(-ENOSPC);
    }
    tmp = kzalloc_obj(*tmp);
    if (!tmp)
    return ERR_PTR(-ENOMEM);
    if (first)
    tmp.is_key_a = !first.is_key_a;
    else
    tmp.is_key_a = true;
    tmp.sa = sa;
    tmp.type = sa_type;
    tmp.an = an;
    tmp.regs = nxp_c45_sa_regs_get(tmp.type, tmp.is_key_a);
    list_add_tail(&tmp.list, sa_list);
    return tmp;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_sa_free(sa: *mut nxp_c45_sa) {
    static void nxp_c45_sa_free(struct nxp_c45_sa *sa)
    {
    list_del(&sa.list);
    kfree(sa);
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_sa_list_free(sa_list: *mut list_head) {
    static void nxp_c45_sa_list_free(struct list_head *sa_list)
    {
    struct nxp_c45_sa *pos, *tmp;
    list_for_each_entry_safe(pos, tmp, sa_list, list)
    nxp_c45_sa_free(pos);
    }
    static void nxp_c45_sa_set_pn(struct phy_device *phydev,
    struct nxp_c45_sa *sa, u64 pn,
    u32 replay_window)
    {
    const struct nxp_c45_sa_regs *sa_regs = sa.regs;
    let mut npn: pn_t = {.full64 = pn};
    pn_t lnpn;
    nxp_c45_macsec_write(phydev, sa_regs.npn, npn.lower);
    nxp_c45_macsec_write(phydev, sa_regs.xnpn, npn.upper);
    if (sa.type != RX_SA)
    return;
    if (pn > replay_window)
    lnpn.full64 = pn - replay_window;
    else
    lnpn.full64 = 1;
    nxp_c45_macsec_write(phydev, sa_regs.lnpn, lnpn.lower);
    nxp_c45_macsec_write(phydev, sa_regs.lxnpn, lnpn.upper);
    }
    static void nxp_c45_sa_set_key(struct macsec_context *ctx,
    const struct nxp_c45_sa_regs *sa_regs,
    u8 *salt, ssci_t ssci)
    {
    struct phy_device *phydev = ctx.phydev;
    let mut key_size: u32 = ctx.secy.key_len / 4;
    let mut salt_size: u32 = MACSEC_SALT_LEN / 4;
    u32 *key_u32 = (u32 *)ctx.sa.key;
    u32 *salt_u32 = (u32 *)salt;
    u32 reg, value;
    int i;
    for (i = 0; i < key_size; i++) {
    reg = sa_regs.ka + i * 4;
    value = ( u32)cpu_to_be32(key_u32[i]);
    nxp_c45_macsec_write(phydev, reg, value);
    }
    if (ctx.secy.xpn) {
    for (i = 0; i < salt_size; i++) {
    reg = sa_regs.salt + (2 - i) * 4;
    value = ( u32)cpu_to_be32(salt_u32[i]);
    nxp_c45_macsec_write(phydev, reg, value);
    }
    value = ( u32)cpu_to_be32(( u32)ssci);
    nxp_c45_macsec_write(phydev, sa_regs.ssci, value);
    }
    nxp_c45_macsec_write(phydev, sa_regs.cs, MACSEC_SA_CS_A);
    }
    static void nxp_c45_rx_sa_clear_stats(struct phy_device *phydev,
    struct nxp_c45_sa *sa)
    {
    nxp_c45_macsec_write(phydev, sa.regs.ipis, 0);
    nxp_c45_macsec_write(phydev, sa.regs.ipnvs, 0);
    nxp_c45_macsec_write(phydev, sa.regs.ipos, 0);
    nxp_c45_macsec_write(phydev, MACSEC_RXAN0INUSS + sa.an * 4, 0);
    nxp_c45_macsec_write(phydev, MACSEC_RXAN0IPUSS + sa.an * 4, 0);
    }
    static void nxp_c45_rx_sa_read_stats(struct phy_device *phydev,
    struct nxp_c45_sa *sa,
    struct macsec_rx_sa_stats *stats)
    {
    nxp_c45_macsec_read(phydev, sa.regs.ipis, &stats.InPktsInvalid);
    nxp_c45_macsec_read(phydev, sa.regs.ipnvs, &stats.InPktsNotValid);
    nxp_c45_macsec_read(phydev, sa.regs.ipos, &stats.InPktsOK);
    }
    static void nxp_c45_tx_sa_clear_stats(struct phy_device *phydev,
    struct nxp_c45_sa *sa)
    {
    nxp_c45_macsec_write(phydev, sa.regs.opps, 0);
    nxp_c45_macsec_write(phydev, sa.regs.opes, 0);
    }
    static void nxp_c45_tx_sa_read_stats(struct phy_device *phydev,
    struct nxp_c45_sa *sa,
    struct macsec_tx_sa_stats *stats)
    {
    nxp_c45_macsec_read(phydev, sa.regs.opps, &stats.OutPktsProtected);
    nxp_c45_macsec_read(phydev, sa.regs.opes, &stats.OutPktsEncrypted);
    }
    static void nxp_c45_rx_sa_update(struct phy_device *phydev,
    struct nxp_c45_sa *sa, bool en)
    {
    const struct nxp_c45_sa_regs *sa_regs = sa.regs;
    u32 cfg;
    cfg = sa.an << MACSEC_RXSA_CS_AN_OFF;
    cfg |= en ? MACSEC_RXSA_CS_EN : 0;
    nxp_c45_macsec_write(phydev, sa_regs.cs, cfg);
    }
    static void nxp_c45_tx_sa_update(struct phy_device *phydev,
    struct nxp_c45_sa *sa, bool en)
    {
    let mut cfg: u32 = 0;
    nxp_c45_macsec_read(phydev, MACSEC_TXSC_CFG, &cfg);
    cfg &= ~MACSEC_TXSC_CFG_AN_MASK;
    cfg |= sa.an << MACSEC_TXSC_CFG_AN_OFF;
    if (sa.is_key_a)
    cfg &= ~MACSEC_TXSC_CFG_ASA;
    else
    cfg |= MACSEC_TXSC_CFG_ASA;
    if (en)
    cfg |= MACSEC_TXSC_CFG_SCE;
    else
    cfg &= ~MACSEC_TXSC_CFG_SCE;
    nxp_c45_macsec_write(phydev, MACSEC_TXSC_CFG, cfg);
    }
    static void nxp_c45_set_sci(struct phy_device *phydev, u16 sci_base_addr,
    sci_t sci)
    {
    let mut lsci: u64 = sci_to_cpu(sci);
    nxp_c45_macsec_write(phydev, sci_base_addr, lsci >> 32);
    nxp_c45_macsec_write(phydev, sci_base_addr + 4, lsci);
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_port_is_1(sci: sci_t) -> bool {
    static bool nxp_c45_port_is_1(sci_t sci)
    {
    let mut port: u16 = sci_to_cpu(sci);
    let mut port: return = = 1;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_select_secy(phydev: *mut phy_device, id: u8) {
    static void nxp_c45_select_secy(struct phy_device *phydev, u8 id)
    {
    nxp_c45_macsec_write(phydev, MACSEC_RXSCA, id);
    nxp_c45_macsec_write(phydev, MACSEC_RXSCKA, id);
    nxp_c45_macsec_write(phydev, MACSEC_TXSCA, id);
    nxp_c45_macsec_write(phydev, MACSEC_TXSCKA, id);
    }
    static bool nxp_c45_secy_valid(struct nxp_c45_secy *phy_secy,
    bool can_rx_sc0_impl)
    {
    let mut end_station: bool = phy_secy.secy.tx_sc.end_station;
    let mut scb: bool = phy_secy.secy.tx_sc.scb;
    phy_secy.rx_sc0_impl = false;
    if (end_station) {
    if (!nxp_c45_port_is_1(phy_secy.secy.sci))
    return false;
    if (!phy_secy.rx_sc)
    return true;
    return nxp_c45_port_is_1(phy_secy.rx_sc.sci);
    }
    if (scb)
    return false;
    if (!can_rx_sc0_impl)
    return false;
    if (phy_secy.secy_id != 0)
    return false;
    phy_secy.rx_sc0_impl = true;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_rx_sc0_impl(phy_secy: *mut nxp_c45_secy) -> bool {
    static bool nxp_c45_rx_sc0_impl(struct nxp_c45_secy *phy_secy)
    {
    let mut end_station: bool = phy_secy.secy.tx_sc.end_station;
    let mut send_sci: bool = phy_secy.secy.tx_sc.send_sci;
    let mut scb: bool = phy_secy.secy.tx_sc.scb;
    return !end_station && !send_sci && !scb;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mac_addr_free(ctx: *mut macsec_context) -> bool {
    static bool nxp_c45_mac_addr_free(struct macsec_context *ctx)
    {
    struct nxp_c45_phy *priv = ctx.phydev.priv;
    struct nxp_c45_secy *pos, *tmp;
    list_for_each_entry_safe(pos, tmp, &priv.macsec.secy_list, list) {
    if (pos.secy == ctx.secy)
    continue;
    if (memcmp(pos.secy.netdev.dev_addr,
    ctx.secy.netdev.dev_addr, ETH_ALEN) == 0)
    return false;
    }
    return true;
    }
    static void nxp_c45_tx_sc_en_flt(struct phy_device *phydev, int secy_id,
    bool en)
    {
    let mut tx_flt_base: u32 = TX_FLT_BASE(secy_id);
    let mut reg: u32 = 0;
    nxp_c45_macsec_read(phydev, TX_SC_FLT_MAC_CFG(tx_flt_base), &reg);
    if (en)
    reg |= TX_SC_FLT_EN;
    else
    reg &= ~TX_SC_FLT_EN;
    nxp_c45_macsec_write(phydev, TX_SC_FLT_MAC_CFG(tx_flt_base), reg);
    }
    static void nxp_c45_tx_sc_set_flt(struct phy_device *phydev,
    struct nxp_c45_secy *phy_secy)
    {
    const u8 *dev_addr = phy_secy.secy.netdev.dev_addr;
    let mut tx_flt_base: u32 = TX_FLT_BASE(phy_secy.secy_id);
    u32 reg;
    reg = dev_addr[0] << 8 | dev_addr[1];
    nxp_c45_macsec_write(phydev, TX_SC_FLT_MAC_DA_SA(tx_flt_base), reg);
    reg = dev_addr[5] | dev_addr[4] << 8 | dev_addr[3] << 16 |
    dev_addr[2] << 24;
    nxp_c45_macsec_write(phydev, TX_SC_FLT_MAC_SA(tx_flt_base), reg);
    nxp_c45_macsec_read(phydev, TX_SC_FLT_MAC_CFG(tx_flt_base), &reg);
    reg &= TX_SC_FLT_EN;
    reg |= TX_SC_FLT_BY_SA | phy_secy.secy_id;
    nxp_c45_macsec_write(phydev, TX_SC_FLT_MAC_CFG(tx_flt_base), reg);
    }
    static void nxp_c45_tx_sc_update(struct phy_device *phydev,
    struct nxp_c45_secy *phy_secy)
    {
    let mut cfg: u32 = 0;
    nxp_c45_macsec_read(phydev, MACSEC_TXSC_CFG, &cfg);
    phydev_dbg(phydev, "XPN %s\n", phy_secy.secy.xpn ? "on" : "off");
    if (phy_secy.secy.xpn)
    cfg |= MACSEC_TXSC_CFG_XPN;
    else
    cfg &= ~MACSEC_TXSC_CFG_XPN;
    phydev_dbg(phydev, "key len %u\n", phy_secy.secy.key_len);
    if (phy_secy.secy.key_len == 32)
    cfg |= MACSEC_TXSC_CFG_AES_256;
    else
    cfg &= ~MACSEC_TXSC_CFG_AES_256;
    phydev_dbg(phydev, "encryption %s\n",
    phy_secy.secy.tx_sc.encrypt ? "on" : "off");
    if (phy_secy.secy.tx_sc.encrypt)
    cfg |= MACSEC_TXSC_CFG_ENCRYPT;
    else
    cfg &= ~MACSEC_TXSC_CFG_ENCRYPT;
    phydev_dbg(phydev, "protect frames %s\n",
    phy_secy.secy.protect_frames ? "on" : "off");
    if (phy_secy.secy.protect_frames)
    cfg |= MACSEC_TXSC_CFG_PROTECT;
    else
    cfg &= ~MACSEC_TXSC_CFG_PROTECT;
    phydev_dbg(phydev, "send sci %s\n",
    phy_secy.secy.tx_sc.send_sci ? "on" : "off");
    if (phy_secy.secy.tx_sc.send_sci)
    cfg |= MACSEC_TXSC_CFG_SEND_SCI;
    else
    cfg &= ~MACSEC_TXSC_CFG_SEND_SCI;
    phydev_dbg(phydev, "end station %s\n",
    phy_secy.secy.tx_sc.end_station ? "on" : "off");
    if (phy_secy.secy.tx_sc.end_station)
    cfg |= MACSEC_TXSC_CFG_END_STATION;
    else
    cfg &= ~MACSEC_TXSC_CFG_END_STATION;
    phydev_dbg(phydev, "scb %s\n",
    phy_secy.secy.tx_sc.scb ? "on" : "off");
    if (phy_secy.secy.tx_sc.scb)
    cfg |= MACSEC_TXSC_CFG_SCB;
    else
    cfg &= ~MACSEC_TXSC_CFG_SCB;
    nxp_c45_macsec_write(phydev, MACSEC_TXSC_CFG, cfg);
    }
    static void nxp_c45_tx_sc_clear_stats(struct phy_device *phydev,
    struct nxp_c45_secy *phy_secy)
    {
    struct nxp_c45_sa *pos, *tmp;
    list_for_each_entry_safe(pos, tmp, &phy_secy.sa_list, list)
    if (pos.type == TX_SA)
    nxp_c45_tx_sa_clear_stats(phydev, pos);
    nxp_c45_macsec_write(phydev, MACSEC_OPUS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_OPTLS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_OOP1HS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_OOP2HS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_OOE1HS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_OOE2HS, 0);
    }
    static void nxp_c45_set_rx_sc0_impl(struct phy_device *phydev,
    bool enable)
    {
    let mut reg: u32 = 0;
    nxp_c45_macsec_read(phydev, MACSEC_CFG, &reg);
    if (enable)
    reg |= MACSEC_CFG_S0I;
    else
    reg &= ~MACSEC_CFG_S0I;
    nxp_c45_macsec_write(phydev, MACSEC_CFG, reg);
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_is_rx_sc0_impl(secy_list: *mut list_head) -> bool {
    static bool nxp_c45_is_rx_sc0_impl(struct list_head *secy_list)
    {
    struct nxp_c45_secy *pos, *tmp;
    list_for_each_entry_safe(pos, tmp, secy_list, list)
    if (pos.rx_sc0_impl)
    return pos.rx_sc0_impl;
    return false;
    }
    static void nxp_c45_rx_sc_en(struct phy_device *phydev,
    struct macsec_rx_sc *rx_sc, bool en)
    {
    let mut reg: u32 = 0;
    nxp_c45_macsec_read(phydev, MACSEC_RXSC_CFG, &reg);
    if (rx_sc.active && en)
    reg |= MACSEC_RXSC_CFG_SCI_EN;
    else
    reg &= ~MACSEC_RXSC_CFG_SCI_EN;
    nxp_c45_macsec_write(phydev, MACSEC_RXSC_CFG, reg);
    }
    static void nxp_c45_rx_sc_update(struct phy_device *phydev,
    struct nxp_c45_secy *phy_secy)
    {
    struct macsec_rx_sc *rx_sc = phy_secy.rx_sc;
    struct nxp_c45_phy *priv = phydev.priv;
    let mut cfg: u32 = 0;
    nxp_c45_macsec_read(phydev, MACSEC_RXSC_CFG, &cfg);
    cfg &= ~MACSEC_RXSC_CFG_VF_MASK;
    cfg = phy_secy.secy.validate_frames << MACSEC_RXSC_CFG_VF_OFF;
    phydev_dbg(phydev, "validate frames %u\n",
    phy_secy.secy.validate_frames);
    phydev_dbg(phydev, "replay_protect %s window %u\n",
    phy_secy.secy.replay_protect ? "on" : "off",
    phy_secy.secy.replay_window);
    if (phy_secy.secy.replay_protect) {
    cfg |= MACSEC_RXSC_CFG_RP;
    nxp_c45_macsec_write(phydev, MACSEC_RPW,
    phy_secy.secy.replay_window);
    } else {
    cfg &= ~MACSEC_RXSC_CFG_RP;
    }
    phydev_dbg(phydev, "rx_sc.active %s\n",
    rx_sc.active ? "on" : "off");
    if (rx_sc.active &&
    test_bit(phy_secy.secy_id, priv.macsec.secy_bitmap))
    cfg |= MACSEC_RXSC_CFG_SCI_EN;
    else
    cfg &= ~MACSEC_RXSC_CFG_SCI_EN;
    phydev_dbg(phydev, "key len %u\n", phy_secy.secy.key_len);
    if (phy_secy.secy.key_len == 32)
    cfg |= MACSEC_RXSC_CFG_AES_256;
    else
    cfg &= ~MACSEC_RXSC_CFG_AES_256;
    phydev_dbg(phydev, "XPN %s\n", phy_secy.secy.xpn ? "on" : "off");
    if (phy_secy.secy.xpn)
    cfg |= MACSEC_RXSC_CFG_XPN;
    else
    cfg &= ~MACSEC_RXSC_CFG_XPN;
    nxp_c45_macsec_write(phydev, MACSEC_RXSC_CFG, cfg);
    }
    static void nxp_c45_rx_sc_clear_stats(struct phy_device *phydev,
    struct nxp_c45_secy *phy_secy)
    {
    struct nxp_c45_sa *pos, *tmp;
    int i;
    list_for_each_entry_safe(pos, tmp, &phy_secy.sa_list, list)
    if (pos.type == RX_SA)
    nxp_c45_rx_sa_clear_stats(phydev, pos);
    nxp_c45_macsec_write(phydev, MACSEC_INOD1HS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_INOD2HS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_INOV1HS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_INOV2HS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_RXSCIPDS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_RXSCIPLS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_RXSCIPUS, 0);
    for (i = 0; i < MACSEC_NUM_AN; i++) {
    nxp_c45_macsec_write(phydev, MACSEC_RXAN0INUSS + i * 4, 0);
    nxp_c45_macsec_write(phydev, MACSEC_RXAN0IPUSS + i * 4, 0);
    }
    }
    static void nxp_c45_rx_sc_del(struct phy_device *phydev,
    struct nxp_c45_secy *phy_secy)
    {
    struct nxp_c45_sa *pos, *tmp;
    nxp_c45_macsec_write(phydev, MACSEC_RXSC_CFG, 0);
    nxp_c45_macsec_write(phydev, MACSEC_RPW, 0);
    nxp_c45_set_sci(phydev, MACSEC_RXSC_SCI_1H, 0);
    nxp_c45_rx_sc_clear_stats(phydev, phy_secy);
    list_for_each_entry_safe(pos, tmp, &phy_secy.sa_list, list) {
    if (pos.type == RX_SA) {
    nxp_c45_rx_sa_update(phydev, pos, false);
    nxp_c45_sa_free(pos);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_clear_global_stats(phydev: *mut phy_device) {
    static void nxp_c45_clear_global_stats(struct phy_device *phydev)
    {
    nxp_c45_macsec_write(phydev, MACSEC_INPBTS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_INPWTS, 0);
    nxp_c45_macsec_write(phydev, MACSEC_IPSNFS, 0);
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_macsec_en(phydev: *mut phy_device, en: bool) {
    static void nxp_c45_macsec_en(struct phy_device *phydev, bool en)
    {
    u32 reg;
    nxp_c45_macsec_read(phydev, MACSEC_CFG, &reg);
    if (en)
    reg |= MACSEC_CFG_BYPASS;
    else
    reg &= ~MACSEC_CFG_BYPASS;
    nxp_c45_macsec_write(phydev, MACSEC_CFG, reg);
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_dev_open(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_dev_open(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_tx_sc_en_flt(phydev, phy_secy.secy_id, true);
    nxp_c45_set_rx_sc0_impl(phydev, phy_secy.rx_sc0_impl);
    if (phy_secy.rx_sc)
    nxp_c45_rx_sc_en(phydev, phy_secy.rx_sc, true);
    if (bitmap_empty(priv.macsec.secy_bitmap, TX_SC_MAX))
    nxp_c45_macsec_en(phydev, true);
    set_bit(phy_secy.secy_id, priv.macsec.secy_bitmap);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_dev_stop(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_dev_stop(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_tx_sc_en_flt(phydev, phy_secy.secy_id, false);
    if (phy_secy.rx_sc)
    nxp_c45_rx_sc_en(phydev, phy_secy.rx_sc, false);
    nxp_c45_set_rx_sc0_impl(phydev, false);
    clear_bit(phy_secy.secy_id, priv.macsec.secy_bitmap);
    if (bitmap_empty(priv.macsec.secy_bitmap, TX_SC_MAX))
    nxp_c45_macsec_en(phydev, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_add_secy(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_add_secy(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    bool can_rx_sc0_impl;
    int idx;
    phydev_dbg(phydev, "add SecY SCI %016llx\n",
    sci_to_cpu(ctx.secy.sci));
    if (!nxp_c45_mac_addr_free(ctx))
    return -EBUSY;
    if (nxp_c45_is_rx_sc0_impl(&priv.macsec.secy_list))
    return -EBUSY;
    idx = find_first_zero_bit(priv.macsec.tx_sc_bitmap, TX_SC_MAX);
    if (idx == TX_SC_MAX)
    return -ENOSPC;
    phy_secy = kzalloc_obj(*phy_secy);
    if (!phy_secy)
    return -ENOMEM;
    INIT_LIST_HEAD(&phy_secy.sa_list);
    phy_secy.secy = ctx.secy;
    phy_secy.secy_id = idx;
// If the point to point mode should be enabled, we should have no
// SecY added yet.
//
    can_rx_sc0_impl = list_count_nodes(&priv.macsec.secy_list) == 0;
    if (!nxp_c45_secy_valid(phy_secy, can_rx_sc0_impl)) {
    kfree(phy_secy);
    return -EINVAL;
    }
    phy_secy.rx_sc0_impl = nxp_c45_rx_sc0_impl(phy_secy);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_set_sci(phydev, MACSEC_TXSC_SCI_1H, ctx.secy.sci);
    nxp_c45_tx_sc_set_flt(phydev, phy_secy);
    nxp_c45_tx_sc_update(phydev, phy_secy);
    if (phy_interrupt_is_valid(phydev))
    nxp_c45_secy_irq_en(phydev, phy_secy, true);
    set_bit(idx, priv.macsec.tx_sc_bitmap);
    list_add_tail(&phy_secy.list, &priv.macsec.secy_list);
    return 0;
    }
    static void nxp_c45_tx_sa_next(struct nxp_c45_secy *phy_secy,
    struct nxp_c45_sa *next_sa, u8 encoding_sa)
    {
    struct nxp_c45_sa *sa;
    sa = nxp_c45_find_sa(&phy_secy.sa_list, TX_SA, encoding_sa);
    if (!IS_ERR(sa)) {
    memcpy(next_sa, sa, sizeof(*sa));
    } else {
    next_sa.is_key_a = true;
    next_sa.an = encoding_sa;
    }
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_upd_secy(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_upd_secy(struct macsec_context *ctx)
    {
    let mut encoding_sa: u8 = ctx.secy.tx_sc.encoding_sa;
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    struct nxp_c45_sa next_sa;
    bool can_rx_sc0_impl;
    phydev_dbg(phydev, "update SecY SCI %016llx\n",
    sci_to_cpu(ctx.secy.sci));
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    if (!nxp_c45_mac_addr_free(ctx))
    return -EBUSY;
// If the point to point mode should be enabled, we should have only
// one SecY added, respectively the updated one.
//
    can_rx_sc0_impl = list_count_nodes(&priv.macsec.secy_list) == 1;
    if (!nxp_c45_secy_valid(phy_secy, can_rx_sc0_impl))
    return -EINVAL;
    phy_secy.rx_sc0_impl = nxp_c45_rx_sc0_impl(phy_secy);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_tx_sc_set_flt(phydev, phy_secy);
    nxp_c45_tx_sc_update(phydev, phy_secy);
    nxp_c45_tx_sa_next(phy_secy, &next_sa, encoding_sa);
    nxp_c45_tx_sa_update(phydev, &next_sa, ctx.secy.operational);
    nxp_c45_set_rx_sc0_impl(phydev, phy_secy.rx_sc0_impl);
    if (phy_secy.rx_sc)
    nxp_c45_rx_sc_update(phydev, phy_secy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_del_secy(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_del_secy(struct macsec_context *ctx)
    {
    let mut encoding_sa: u8 = ctx.secy.tx_sc.encoding_sa;
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    struct nxp_c45_sa next_sa;
    phydev_dbg(phydev, "delete SecY SCI %016llx\n",
    sci_to_cpu(ctx.secy.sci));
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_mdo_dev_stop(ctx);
    nxp_c45_tx_sa_next(phy_secy, &next_sa, encoding_sa);
    nxp_c45_tx_sa_update(phydev, &next_sa, false);
    nxp_c45_tx_sc_clear_stats(phydev, phy_secy);
    if (phy_secy.rx_sc)
    nxp_c45_rx_sc_del(phydev, phy_secy);
    nxp_c45_sa_list_free(&phy_secy.sa_list);
    if (phy_interrupt_is_valid(phydev))
    nxp_c45_secy_irq_en(phydev, phy_secy, false);
    clear_bit(phy_secy.secy_id, priv.macsec.tx_sc_bitmap);
    nxp_c45_secy_free(phy_secy);
    if (list_empty(&priv.macsec.secy_list))
    nxp_c45_clear_global_stats(phydev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_add_rxsc(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_add_rxsc(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    phydev_dbg(phydev, "add RX SC SCI %016llx %s\n",
    sci_to_cpu(ctx.rx_sc.sci),
    ctx.rx_sc.active ? "enabled" : "disabled");
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    if (phy_secy.rx_sc)
    return -ENOSPC;
    if (phy_secy.secy.tx_sc.end_station &&
    !nxp_c45_port_is_1(ctx.rx_sc.sci))
    return -EINVAL;
    phy_secy.rx_sc = ctx.rx_sc;
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_set_sci(phydev, MACSEC_RXSC_SCI_1H, ctx.rx_sc.sci);
    nxp_c45_rx_sc_update(phydev, phy_secy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_upd_rxsc(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_upd_rxsc(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    phydev_dbg(phydev, "update RX SC SCI %016llx %s\n",
    sci_to_cpu(ctx.rx_sc.sci),
    ctx.rx_sc.active ? "enabled" : "disabled");
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_rx_sc_update(phydev, phy_secy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_del_rxsc(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_del_rxsc(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    phydev_dbg(phydev, "delete RX SC SCI %016llx %s\n",
    sci_to_cpu(ctx.rx_sc.sci),
    ctx.rx_sc.active ? "enabled" : "disabled");
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_rx_sc_del(phydev, phy_secy);
    phy_secy.rx_sc = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_add_rxsa(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_add_rxsa(struct macsec_context *ctx)
    {
    struct macsec_rx_sa *rx_sa = ctx.sa.rx_sa;
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    let mut an: u8 = ctx.sa.assoc_num;
    struct nxp_c45_sa *sa;
    phydev_dbg(phydev, "add RX SA %u %s to RX SC SCI %016llx\n",
    an, rx_sa.active ? "enabled" : "disabled",
    sci_to_cpu(rx_sa.sc.sci));
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    sa = nxp_c45_sa_alloc(&phy_secy.sa_list, rx_sa, RX_SA, an);
    if (IS_ERR(sa))
    return PTR_ERR(sa);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_sa_set_pn(phydev, sa, rx_sa.next_pn,
    ctx.secy.replay_window);
    nxp_c45_sa_set_key(ctx, sa.regs, rx_sa.key.salt.bytes, rx_sa.ssci);
    nxp_c45_rx_sa_update(phydev, sa, rx_sa.active);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_upd_rxsa(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_upd_rxsa(struct macsec_context *ctx)
    {
    struct macsec_rx_sa *rx_sa = ctx.sa.rx_sa;
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    let mut an: u8 = ctx.sa.assoc_num;
    struct nxp_c45_sa *sa;
    phydev_dbg(phydev, "update RX SA %u %s to RX SC SCI %016llx\n",
    an, rx_sa.active ? "enabled" : "disabled",
    sci_to_cpu(rx_sa.sc.sci));
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    sa = nxp_c45_find_sa(&phy_secy.sa_list, RX_SA, an);
    if (IS_ERR(sa))
    return PTR_ERR(sa);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    if (ctx.sa.update_pn)
    nxp_c45_sa_set_pn(phydev, sa, rx_sa.next_pn,
    ctx.secy.replay_window);
    nxp_c45_rx_sa_update(phydev, sa, rx_sa.active);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_del_rxsa(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_del_rxsa(struct macsec_context *ctx)
    {
    struct macsec_rx_sa *rx_sa = ctx.sa.rx_sa;
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    let mut an: u8 = ctx.sa.assoc_num;
    struct nxp_c45_sa *sa;
    phydev_dbg(phydev, "delete RX SA %u %s to RX SC SCI %016llx\n",
    an, rx_sa.active ? "enabled" : "disabled",
    sci_to_cpu(rx_sa.sc.sci));
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    sa = nxp_c45_find_sa(&phy_secy.sa_list, RX_SA, an);
    if (IS_ERR(sa))
    return PTR_ERR(sa);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_rx_sa_update(phydev, sa, false);
    nxp_c45_rx_sa_clear_stats(phydev, sa);
    nxp_c45_sa_free(sa);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_add_txsa(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_add_txsa(struct macsec_context *ctx)
    {
    struct macsec_tx_sa *tx_sa = ctx.sa.tx_sa;
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    let mut an: u8 = ctx.sa.assoc_num;
    struct nxp_c45_sa *sa;
    phydev_dbg(phydev, "add TX SA %u %s to TX SC %016llx\n",
    an, ctx.sa.tx_sa.active ? "enabled" : "disabled",
    sci_to_cpu(ctx.secy.sci));
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    sa = nxp_c45_sa_alloc(&phy_secy.sa_list, tx_sa, TX_SA, an);
    if (IS_ERR(sa))
    return PTR_ERR(sa);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_sa_set_pn(phydev, sa, tx_sa.next_pn, 0);
    nxp_c45_sa_set_key(ctx, sa.regs, tx_sa.key.salt.bytes, tx_sa.ssci);
    if (ctx.secy.tx_sc.encoding_sa == sa.an)
    nxp_c45_tx_sa_update(phydev, sa, tx_sa.active);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_upd_txsa(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_upd_txsa(struct macsec_context *ctx)
    {
    struct macsec_tx_sa *tx_sa = ctx.sa.tx_sa;
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    let mut an: u8 = ctx.sa.assoc_num;
    struct nxp_c45_sa *sa;
    phydev_dbg(phydev, "update TX SA %u %s to TX SC %016llx\n",
    an, ctx.sa.tx_sa.active ? "enabled" : "disabled",
    sci_to_cpu(ctx.secy.sci));
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    sa = nxp_c45_find_sa(&phy_secy.sa_list, TX_SA, an);
    if (IS_ERR(sa))
    return PTR_ERR(sa);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    if (ctx.sa.update_pn)
    nxp_c45_sa_set_pn(phydev, sa, tx_sa.next_pn, 0);
    if (ctx.secy.tx_sc.encoding_sa == sa.an)
    nxp_c45_tx_sa_update(phydev, sa, tx_sa.active);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_del_txsa(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_del_txsa(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *phy_secy;
    let mut an: u8 = ctx.sa.assoc_num;
    struct nxp_c45_sa *sa;
    phydev_dbg(phydev, "delete TX SA %u %s to TX SC %016llx\n",
    an, ctx.sa.tx_sa.active ? "enabled" : "disabled",
    sci_to_cpu(ctx.secy.sci));
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    sa = nxp_c45_find_sa(&phy_secy.sa_list, TX_SA, an);
    if (IS_ERR(sa))
    return PTR_ERR(sa);
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    if (ctx.secy.tx_sc.encoding_sa == sa.an)
    nxp_c45_tx_sa_update(phydev, sa, false);
    nxp_c45_tx_sa_clear_stats(phydev, sa);
    nxp_c45_sa_free(sa);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_get_dev_stats(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_get_dev_stats(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct macsec_dev_stats  *dev_stats;
    struct nxp_c45_secy *phy_secy;
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    dev_stats = ctx.stats.dev_stats;
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_macsec_read32_64(phydev, MACSEC_OPUS,
    &dev_stats.OutPktsUntagged);
    nxp_c45_macsec_read32_64(phydev, MACSEC_OPTLS,
    &dev_stats.OutPktsTooLong);
    nxp_c45_macsec_read32_64(phydev, MACSEC_INPBTS,
    &dev_stats.InPktsBadTag);
    if (phy_secy.secy.validate_frames == MACSEC_VALIDATE_STRICT)
    nxp_c45_macsec_read32_64(phydev, MACSEC_INPWTS,
    &dev_stats.InPktsNoTag);
    else
    nxp_c45_macsec_read32_64(phydev, MACSEC_INPWTS,
    &dev_stats.InPktsUntagged);
    if (phy_secy.secy.validate_frames == MACSEC_VALIDATE_STRICT)
    nxp_c45_macsec_read32_64(phydev, MACSEC_IPSNFS,
    &dev_stats.InPktsNoSCI);
    else
    nxp_c45_macsec_read32_64(phydev, MACSEC_IPSNFS,
    &dev_stats.InPktsUnknownSCI);
// Always 0.
    dev_stats.InPktsOverrun = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_get_tx_sc_stats(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_get_tx_sc_stats(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct macsec_tx_sa_stats tx_sa_stats;
    struct macsec_tx_sc_stats *stats;
    struct nxp_c45_secy *phy_secy;
    struct nxp_c45_sa *pos, *tmp;
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    stats = ctx.stats.tx_sc_stats;
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_macsec_read64(phydev, MACSEC_OOE1HS,
    &stats.OutOctetsEncrypted);
    nxp_c45_macsec_read64(phydev, MACSEC_OOP1HS,
    &stats.OutOctetsProtected);
    list_for_each_entry_safe(pos, tmp, &phy_secy.sa_list, list) {
    if (pos.type != TX_SA)
    continue;
    memset(&tx_sa_stats, 0, sizeof(tx_sa_stats));
    nxp_c45_tx_sa_read_stats(phydev, pos, &tx_sa_stats);
    stats.OutPktsEncrypted += tx_sa_stats.OutPktsEncrypted;
    stats.OutPktsProtected += tx_sa_stats.OutPktsProtected;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_get_tx_sa_stats(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_get_tx_sa_stats(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct macsec_tx_sa_stats *stats;
    struct nxp_c45_secy *phy_secy;
    let mut an: u8 = ctx.sa.assoc_num;
    struct nxp_c45_sa *sa;
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    sa = nxp_c45_find_sa(&phy_secy.sa_list, TX_SA, an);
    if (IS_ERR(sa))
    return PTR_ERR(sa);
    stats = ctx.stats.tx_sa_stats;
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_tx_sa_read_stats(phydev, sa, stats);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_get_rx_sc_stats(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_get_rx_sc_stats(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct macsec_rx_sa_stats rx_sa_stats;
    struct macsec_rx_sc_stats *stats;
    struct nxp_c45_secy *phy_secy;
    struct nxp_c45_sa *pos, *tmp;
    let mut reg: u32 = 0;
    int i;
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    if (phy_secy.rx_sc != ctx.rx_sc)
    return -EINVAL;
    stats = ctx.stats.rx_sc_stats;
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    list_for_each_entry_safe(pos, tmp, &phy_secy.sa_list, list) {
    if (pos.type != RX_SA)
    continue;
    memset(&rx_sa_stats, 0, sizeof(rx_sa_stats));
    nxp_c45_rx_sa_read_stats(phydev, pos, &rx_sa_stats);
    stats.InPktsInvalid += rx_sa_stats.InPktsInvalid;
    stats.InPktsNotValid += rx_sa_stats.InPktsNotValid;
    stats.InPktsOK += rx_sa_stats.InPktsOK;
    }
    for (i = 0; i < MACSEC_NUM_AN; i++) {
    nxp_c45_macsec_read(phydev, MACSEC_RXAN0INUSS + i * 4, &reg);
    stats.InPktsNotUsingSA += reg;
    nxp_c45_macsec_read(phydev, MACSEC_RXAN0IPUSS + i * 4, &reg);
    stats.InPktsUnusedSA += reg;
    }
    nxp_c45_macsec_read64(phydev, MACSEC_INOD1HS,
    &stats.InOctetsDecrypted);
    nxp_c45_macsec_read64(phydev, MACSEC_INOV1HS,
    &stats.InOctetsValidated);
    nxp_c45_macsec_read32_64(phydev, MACSEC_RXSCIPDS,
    &stats.InPktsDelayed);
    nxp_c45_macsec_read32_64(phydev, MACSEC_RXSCIPLS,
    &stats.InPktsLate);
    nxp_c45_macsec_read32_64(phydev, MACSEC_RXSCIPUS,
    &stats.InPktsUnchecked);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_c45_mdo_get_rx_sa_stats(ctx: *mut macsec_context) -> c_int {
    static int nxp_c45_mdo_get_rx_sa_stats(struct macsec_context *ctx)
    {
    struct phy_device *phydev = ctx.phydev;
    struct nxp_c45_phy *priv = phydev.priv;
    struct macsec_rx_sa_stats *stats;
    struct nxp_c45_secy *phy_secy;
    let mut an: u8 = ctx.sa.assoc_num;
    struct nxp_c45_sa *sa;
    phy_secy = nxp_c45_find_secy(&priv.macsec.secy_list, ctx.secy.sci);
    if (IS_ERR(phy_secy))
    return PTR_ERR(phy_secy);
    sa = nxp_c45_find_sa(&phy_secy.sa_list, RX_SA, an);
    if (IS_ERR(sa))
    return PTR_ERR(sa);
    stats = ctx.stats.rx_sa_stats;
    nxp_c45_select_secy(phydev, phy_secy.secy_id);
    nxp_c45_rx_sa_read_stats(phydev, sa, stats);
    nxp_c45_macsec_read(phydev, MACSEC_RXAN0INUSS + an * 4,
    &stats.InPktsNotUsingSA);
    nxp_c45_macsec_read(phydev, MACSEC_RXAN0IPUSS + an * 4,
    &stats.InPktsUnusedSA);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tja11xx_tlv_header {
    pub eth: ethhdr,
    pub subtype: u8,
    pub len: u8,
    pub payload: [u8; 28],
}

    static int nxp_c45_mdo_insert_tx_tag(struct phy_device *phydev,
    struct sk_buff *skb)
    {
    struct tja11xx_tlv_header *tlv;
    struct ethhdr *eth;
    eth = eth_hdr(skb);
    tlv = skb_push(skb, TJA11XX_TLV_TX_NEEDED_HEADROOM);
    memmove(tlv, eth, sizeof(*eth));
    skb_reset_mac_header(skb);
    tlv.eth.h_proto = htons(ETH_P_TJA11XX_TLV);
    tlv.subtype = 1;
    tlv.len = sizeof(tlv.payload);
    memset(tlv.payload, 0, sizeof(tlv.payload));
    return 0;
    }
    static const struct macsec_ops nxp_c45_macsec_ops = {
    .mdo_dev_open = nxp_c45_mdo_dev_open,
    .mdo_dev_stop = nxp_c45_mdo_dev_stop,
    .mdo_add_secy = nxp_c45_mdo_add_secy,
    .mdo_upd_secy = nxp_c45_mdo_upd_secy,
    .mdo_del_secy = nxp_c45_mdo_del_secy,
    .mdo_add_rxsc = nxp_c45_mdo_add_rxsc,
    .mdo_upd_rxsc = nxp_c45_mdo_upd_rxsc,
    .mdo_del_rxsc = nxp_c45_mdo_del_rxsc,
    .mdo_add_rxsa = nxp_c45_mdo_add_rxsa,
    .mdo_upd_rxsa = nxp_c45_mdo_upd_rxsa,
    .mdo_del_rxsa = nxp_c45_mdo_del_rxsa,
    .mdo_add_txsa = nxp_c45_mdo_add_txsa,
    .mdo_upd_txsa = nxp_c45_mdo_upd_txsa,
    .mdo_del_txsa = nxp_c45_mdo_del_txsa,
    .mdo_get_dev_stats = nxp_c45_mdo_get_dev_stats,
    .mdo_get_tx_sc_stats = nxp_c45_mdo_get_tx_sc_stats,
    .mdo_get_tx_sa_stats = nxp_c45_mdo_get_tx_sa_stats,
    .mdo_get_rx_sc_stats = nxp_c45_mdo_get_rx_sc_stats,
    .mdo_get_rx_sa_stats = nxp_c45_mdo_get_rx_sa_stats,
    .mdo_insert_tx_tag = nxp_c45_mdo_insert_tx_tag,
    .needed_headroom = TJA11XX_TLV_TX_NEEDED_HEADROOM,
    .needed_tailroom = TJA11XX_TLV_NEEDED_TAILROOM,
    };
#[no_mangle]
pub unsafe extern "C" fn nxp_c45_macsec_config_init(phydev: *mut phy_device) -> c_int {
    int nxp_c45_macsec_config_init(struct phy_device *phydev)
    {
    struct nxp_c45_phy *priv = phydev.priv;
    int ret;
    if (!priv.macsec)
    return 0;
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_VEND1, VEND1_PORT_FUNC_ENABLES,
    MACSEC_EN | ADAPTER_EN);
    if (ret)
    return ret;
    ret = nxp_c45_macsec_write(phydev, ADPTR_CNTRL, ADPTR_CNTRL_CONFIG_EN |
    ADPTR_CNTRL_ADPTR_EN);
    if (ret)
    return ret;
    ret = nxp_c45_macsec_write(phydev, ADPTR_TX_TAG_CNTRL,
    ADPTR_TX_TAG_CNTRL_ENA);
    if (ret)
    return ret;
    ret = nxp_c45_macsec_write(phydev, ADPTR_CNTRL, ADPTR_CNTRL_ADPTR_EN);
    if (ret)
    return ret;
    ret = nxp_c45_macsec_write(phydev, MACSEC_TPNET, PN_WRAP_THRESHOLD);
    if (ret)
    return ret;
// Set MKA filter.
    ret = nxp_c45_macsec_write(phydev, MACSEC_UPFR0D2, ETH_P_PAE);
    if (ret)
    return ret;
    ret = nxp_c45_macsec_write(phydev, MACSEC_UPFR0M1, MACSEC_OVP);
    if (ret)
    return ret;
    ret = nxp_c45_macsec_write(phydev, MACSEC_UPFR0M2, ETYPE_MASK);
    if (ret)
    return ret;
    ret = nxp_c45_macsec_write(phydev, MACSEC_UPFR0R, MACSEC_UPFR_EN);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_c45_macsec_probe(phydev: *mut phy_device) -> c_int {
    int nxp_c45_macsec_probe(struct phy_device *phydev)
    {
    struct nxp_c45_phy *priv = phydev.priv;
    struct device *dev = &phydev.mdio.dev;
    priv.macsec = devm_kzalloc(dev, sizeof(*priv.macsec), GFP_KERNEL);
    if (!priv.macsec)
    return -ENOMEM;
    INIT_LIST_HEAD(&priv.macsec.secy_list);
    phydev.macsec_ops = &nxp_c45_macsec_ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_c45_macsec_remove(phydev: *mut phy_device) {
    void nxp_c45_macsec_remove(struct phy_device *phydev)
    {
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *secy_p, *secy_t;
    struct nxp_c45_sa *sa_p, *sa_t;
    struct list_head *secy_list;
    if (!priv.macsec)
    return;
    secy_list = &priv.macsec.secy_list;
    nxp_c45_macsec_en(phydev, false);
    list_for_each_entry_safe(secy_p, secy_t, secy_list, list) {
    list_for_each_entry_safe(sa_p, sa_t, &secy_p.sa_list, list)
    nxp_c45_sa_free(sa_p);
    nxp_c45_secy_free(secy_p);
    }
    }
    void nxp_c45_handle_macsec_interrupt(struct phy_device *phydev,
    irqreturn_t *ret)
    {
    struct nxp_c45_phy *priv = phydev.priv;
    struct nxp_c45_secy *secy;
    struct nxp_c45_sa *sa;
    u8 encoding_sa;
    int secy_id;
    let mut reg: u32 = 0;
    if (!priv.macsec)
    return;
    do {
    nxp_c45_macsec_read(phydev, MACSEC_EVR, &reg);
    if (!reg)
    return;
    secy_id = MACSEC_REG_SIZE - ffs(reg);
    secy = nxp_c45_find_secy_by_id(&priv.macsec.secy_list,
    secy_id);
    if (IS_ERR(secy)) {
    WARN_ON(1);
    goto macsec_ack_irq;
    }
    encoding_sa = secy.secy.tx_sc.encoding_sa;
    phydev_dbg(phydev, "pn_wrapped: TX SC %d, encoding_sa %u\n",
    secy.secy_id, encoding_sa);
    sa = nxp_c45_find_sa(&secy.sa_list, TX_SA, encoding_sa);
    if (!IS_ERR(sa))
    macsec_pn_wrapped(secy.secy, sa.sa);
    else
    WARN_ON(1);
    macsec_ack_irq:
    nxp_c45_macsec_write(phydev, MACSEC_EVR,
    TX_SC_BIT(secy_id));
// ret = IRQ_HANDLED;
    } while (reg);
    }
