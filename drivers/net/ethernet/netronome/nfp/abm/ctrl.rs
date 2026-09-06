//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/abm/ctrl.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2018 Netronome Systems, Inc.

pub const NFP_QLVL_STRIDE: c_int = 16;
pub const NFP_QLVL_BLOG_BYTES: c_int = 0;
pub const NFP_QLVL_BLOG_PKTS: c_int = 4;
pub const NFP_QLVL_THRS: c_int = 8;
pub const NFP_QLVL_ACT: c_int = 12;

pub const NFP_QMSTAT_STRIDE: c_int = 32;
pub const NFP_QMSTAT_NON_STO: c_int = 0;
pub const NFP_QMSTAT_STO: c_int = 8;
pub const NFP_QMSTAT_DROP: c_int = 16;
pub const NFP_QMSTAT_ECN: c_int = 24;

pub const NFP_Q_STAT_STRIDE: c_int = 16;
pub const NFP_Q_STAT_PKTS: c_int = 0;
pub const NFP_Q_STAT_BYTES: c_int = 8;

    static int
    nfp_abm_ctrl_stat(struct nfp_abm_link *alink, const struct nfp_rtsym *sym,
    unsigned int stride, unsigned int offset, unsigned int band,
    unsigned int queue, bool is_u64, u64 *res)
    {
    struct nfp_cpp *cpp = alink.abm.app.cpp;
    u64 val, sym_offset;
    unsigned int qid;
    u32 val32;
    int err;
    qid = band * NFP_NET_MAX_RX_RINGS + alink.queue_base + queue;
    sym_offset = qid * stride + offset;
    if (is_u64)
    err = __nfp_rtsym_readq(cpp, sym, 3, 0, sym_offset, &val);
    else
    err = __nfp_rtsym_readl(cpp, sym, 3, 0, sym_offset, &val32);
    if (err) {
    nfp_err(cpp, "RED offload reading stat failed on vNIC %d band %d queue %d (+ %d)\n",
    alink.id, band, queue, alink.queue_base);
    return err;
    }
// res = is_u64 ? val : val32;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __nfp_abm_ctrl_set_q_lvl(abm: *mut nfp_abm, id: c_uint, val: u32) -> c_int {
    int __nfp_abm_ctrl_set_q_lvl(struct nfp_abm *abm, unsigned int id, u32 val)
    {
    struct nfp_cpp *cpp = abm.app.cpp;
    u64 sym_offset;
    int err;
    __clear_bit(id, abm.threshold_undef);
    if (abm.thresholds[id] == val)
    return 0;
    sym_offset = id * NFP_QLVL_STRIDE + NFP_QLVL_THRS;
    err = __nfp_rtsym_writel(cpp, abm.q_lvls, 4, 0, sym_offset, val);
    if (err) {
    nfp_err(cpp,
    "RED offload setting level failed on subqueue %d\n",
    id);
    return err;
    }
    abm.thresholds[id] = val;
    return 0;
    }
    int nfp_abm_ctrl_set_q_lvl(struct nfp_abm_link *alink, unsigned int band,
    unsigned int queue, u32 val)
    {
    unsigned int threshold;
    threshold = band * NFP_NET_MAX_RX_RINGS + alink.queue_base + queue;
    return __nfp_abm_ctrl_set_q_lvl(alink.abm, threshold, val);
    }
    int __nfp_abm_ctrl_set_q_act(struct nfp_abm *abm, unsigned int id,
    enum nfp_abm_q_action act)
    {
    struct nfp_cpp *cpp = abm.app.cpp;
    u64 sym_offset;
    int err;
    if (abm.actions[id] == act)
    return 0;
    sym_offset = id * NFP_QLVL_STRIDE + NFP_QLVL_ACT;
    err = __nfp_rtsym_writel(cpp, abm.q_lvls, 4, 0, sym_offset, act);
    if (err) {
    nfp_err(cpp,
    "RED offload setting action failed on subqueue %d\n",
    id);
    return err;
    }
    abm.actions[id] = act;
    return 0;
    }
    int nfp_abm_ctrl_set_q_act(struct nfp_abm_link *alink, unsigned int band,
    unsigned int queue, enum nfp_abm_q_action act)
    {
    unsigned int qid;
    qid = band * NFP_NET_MAX_RX_RINGS + alink.queue_base + queue;
    return __nfp_abm_ctrl_set_q_act(alink.abm, qid, act);
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_abm_ctrl_stat_non_sto(alink: *mut nfp_abm_link, queue: c_uint) -> u64 {
    u64 nfp_abm_ctrl_stat_non_sto(struct nfp_abm_link *alink, unsigned int queue)
    {
    unsigned int band;
    u64 val, sum = 0;
    for (band = 0; band < alink.abm.num_bands; band++) {
    if (nfp_abm_ctrl_stat(alink, alink.abm.qm_stats,
    NFP_QMSTAT_STRIDE, NFP_QMSTAT_NON_STO,
    band, queue, true, &val))
    return 0;
    sum += val;
    }
    return sum;
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_abm_ctrl_stat_sto(alink: *mut nfp_abm_link, queue: c_uint) -> u64 {
    u64 nfp_abm_ctrl_stat_sto(struct nfp_abm_link *alink, unsigned int queue)
    {
    unsigned int band;
    u64 val, sum = 0;
    for (band = 0; band < alink.abm.num_bands; band++) {
    if (nfp_abm_ctrl_stat(alink, alink.abm.qm_stats,
    NFP_QMSTAT_STRIDE, NFP_QMSTAT_STO,
    band, queue, true, &val))
    return 0;
    sum += val;
    }
    return sum;
    }
    static int
    nfp_abm_ctrl_stat_basic(struct nfp_abm_link *alink, unsigned int band,
    unsigned int queue, unsigned int off, u64 *val)
    {
    if (!nfp_abm_has_prio(alink.abm)) {
    if (!band) {
    let mut id: c_uint = alink.queue_base + queue;
// val = nn_readq(alink->vnic,
    NFP_NET_CFG_RXR_STATS(id) + off);
    } else {
// val = 0;
    }
    return 0;
    } else {
    return nfp_abm_ctrl_stat(alink, alink.abm.q_stats,
    NFP_Q_STAT_STRIDE, off, band, queue,
    true, val);
    }
    }
    int nfp_abm_ctrl_read_q_stats(struct nfp_abm_link *alink, unsigned int band,
    unsigned int queue, struct nfp_alink_stats *stats)
    {
    int err;
    err = nfp_abm_ctrl_stat_basic(alink, band, queue, NFP_Q_STAT_PKTS,
    &stats.tx_pkts);
    if (err)
    return err;
    err = nfp_abm_ctrl_stat_basic(alink, band, queue, NFP_Q_STAT_BYTES,
    &stats.tx_bytes);
    if (err)
    return err;
    err = nfp_abm_ctrl_stat(alink, alink.abm.q_lvls, NFP_QLVL_STRIDE,
    NFP_QLVL_BLOG_BYTES, band, queue, false,
    &stats.backlog_bytes);
    if (err)
    return err;
    err = nfp_abm_ctrl_stat(alink, alink.abm.q_lvls,
    NFP_QLVL_STRIDE, NFP_QLVL_BLOG_PKTS,
    band, queue, false, &stats.backlog_pkts);
    if (err)
    return err;
    err = nfp_abm_ctrl_stat(alink, alink.abm.qm_stats,
    NFP_QMSTAT_STRIDE, NFP_QMSTAT_DROP,
    band, queue, true, &stats.drops);
    if (err)
    return err;
    return nfp_abm_ctrl_stat(alink, alink.abm.qm_stats,
    NFP_QMSTAT_STRIDE, NFP_QMSTAT_ECN,
    band, queue, true, &stats.overlimits);
    }
    int nfp_abm_ctrl_read_q_xstats(struct nfp_abm_link *alink,
    unsigned int band, unsigned int queue,
    struct nfp_alink_xstats *xstats)
    {
    int err;
    err = nfp_abm_ctrl_stat(alink, alink.abm.qm_stats,
    NFP_QMSTAT_STRIDE, NFP_QMSTAT_DROP,
    band, queue, true, &xstats.pdrop);
    if (err)
    return err;
    return nfp_abm_ctrl_stat(alink, alink.abm.qm_stats,
    NFP_QMSTAT_STRIDE, NFP_QMSTAT_ECN,
    band, queue, true, &xstats.ecn_marked);
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_abm_ctrl_qm_enable(abm: *mut nfp_abm) -> c_int {
    int nfp_abm_ctrl_qm_enable(struct nfp_abm *abm)
    {
    return nfp_mbox_cmd(abm.app.pf, NFP_MBOX_PCIE_ABM_ENABLE,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_abm_ctrl_qm_disable(abm: *mut nfp_abm) -> c_int {
    int nfp_abm_ctrl_qm_disable(struct nfp_abm *abm)
    {
    return nfp_mbox_cmd(abm.app.pf, NFP_MBOX_PCIE_ABM_DISABLE,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_abm_ctrl_prio_map_update(alink: *mut nfp_abm_link, packed: *mut u32) -> c_int {
    int nfp_abm_ctrl_prio_map_update(struct nfp_abm_link *alink, u32 *packed)
    {
    let mut cmd: u32 = NFP_NET_CFG_MBOX_CMD_PCI_DSCP_PRIOMAP_SET;
    struct nfp_net *nn = alink.vnic;
    unsigned int i;
    int err;
    err = nfp_net_mbox_lock(nn, alink.abm.prio_map_len);
    if (err)
    return err;
// Write data_len and wipe reserved
    nn_writeq(nn, nn.tlv_caps.mbox_off + NFP_NET_ABM_MBOX_DATALEN,
    alink.abm.prio_map_len);
    for (i = 0; i < alink.abm.prio_map_len; i += sizeof(u32))
    nn_writel(nn, nn.tlv_caps.mbox_off + NFP_NET_ABM_MBOX_DATA + i,
    packed[i / sizeof(u32)]);
    err = nfp_net_mbox_reconfig_and_unlock(nn, cmd);
    if (err)
    nfp_err(alink.abm.app.cpp,
    "setting DSCP . VQ map failed with error %d\n", err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nfp_abm_ctrl_prio_check_params(alink: *mut nfp_abm_link) -> c_int {
    static int nfp_abm_ctrl_prio_check_params(struct nfp_abm_link *alink)
    {
    struct nfp_abm *abm = alink.abm;
    struct nfp_net *nn = alink.vnic;
    unsigned int min_mbox_sz;
    if (!nfp_abm_has_prio(alink.abm))
    return 0;
    min_mbox_sz = NFP_NET_ABM_MBOX_DATA + alink.abm.prio_map_len;
    if (nn.tlv_caps.mbox_len < min_mbox_sz) {
    nfp_err(abm.app.pf.cpp, "vNIC mailbox too small for prio offload: %u, need: %u\n",
    nn.tlv_caps.mbox_len,  min_mbox_sz);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_abm_ctrl_read_params(alink: *mut nfp_abm_link) -> c_int {
    int nfp_abm_ctrl_read_params(struct nfp_abm_link *alink)
    {
    alink.queue_base = nn_readl(alink.vnic, NFP_NET_CFG_START_RXQ);
    alink.queue_base /= alink.vnic.stride_rx;
    return nfp_abm_ctrl_prio_check_params(alink);
    }
#[no_mangle]
unsafe extern "C" fn nfp_abm_ctrl_prio_map_size(abm: *mut nfp_abm) -> c_uint {
    static unsigned int nfp_abm_ctrl_prio_map_size(struct nfp_abm *abm)
    {
    unsigned int size;
    size = roundup_pow_of_two(order_base_2(abm.num_bands));
    size = DIV_ROUND_UP(size * abm.num_prios, BITS_PER_BYTE);
    size = round_up(size, sizeof(u32));
    return size;
    }
    static const struct nfp_rtsym *
    nfp_abm_ctrl_find_rtsym(struct nfp_pf *pf, const char *name, unsigned int size)
    {
    const struct nfp_rtsym *sym;
    sym = nfp_rtsym_lookup(pf.rtbl, name);
    if (!sym) {
    nfp_err(pf.cpp, "Symbol '%s' not found\n", name);
    return ERR_PTR(-ENOENT);
    }
    if (nfp_rtsym_size(sym) != size) {
    nfp_err(pf.cpp,
    "Symbol '%s' wrong size: expected %u got %llu\n",
    name, size, nfp_rtsym_size(sym));
    return ERR_PTR(-EINVAL);
    }
    return sym;
    }
    static const struct nfp_rtsym *
    nfp_abm_ctrl_find_q_rtsym(struct nfp_abm *abm, const char *name_fmt,
    size_t size)
    {
    char pf_symbol[64];
    size = array3_size(size, abm.num_bands, NFP_NET_MAX_RX_RINGS);
    snprintf(pf_symbol, sizeof(pf_symbol), name_fmt,
    abm.pf_id, nfp_abm_has_prio(abm) ? "_per_band" : "");
    return nfp_abm_ctrl_find_rtsym(abm.app.pf, pf_symbol, size);
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_abm_ctrl_find_addrs(abm: *mut nfp_abm) -> c_int {
    int nfp_abm_ctrl_find_addrs(struct nfp_abm *abm)
    {
    struct nfp_pf *pf = abm.app.pf;
    const struct nfp_rtsym *sym;
    int res;
    abm.pf_id = nfp_cppcore_pcie_unit(pf.cpp);
// Check if Qdisc offloads are supported
    res = nfp_pf_rtsym_read_optional(pf, NFP_RED_SUPPORT_SYM_NAME, 1);
    if (res < 0)
    return res;
    abm.red_support = res;
// Read count of prios and prio bands
    res = nfp_pf_rtsym_read_optional(pf, NFP_NUM_BANDS_SYM_NAME, 1);
    if (res < 0)
    return res;
    abm.num_bands = res;
    res = nfp_pf_rtsym_read_optional(pf, NFP_NUM_PRIOS_SYM_NAME, 1);
    if (res < 0)
    return res;
    abm.num_prios = res;
// Read available actions
    res = nfp_pf_rtsym_read_optional(pf, NFP_ACT_MASK_SYM_NAME,
    BIT(NFP_ABM_ACT_MARK_DROP));
    if (res < 0)
    return res;
    abm.action_mask = res;
    abm.prio_map_len = nfp_abm_ctrl_prio_map_size(abm);
    abm.dscp_mask = GENMASK(7, 8 - order_base_2(abm.num_prios));
// Check values are sane, U16_MAX is arbitrarily chosen as max
    if (!is_power_of_2(abm.num_bands) || !is_power_of_2(abm.num_prios) ||
    abm.num_bands > U16_MAX || abm.num_prios > U16_MAX ||
    (abm.num_bands == 1) != (abm.num_prios == 1)) {
    nfp_err(pf.cpp,
    "invalid priomap description num bands: %u and num prios: %u\n",
    abm.num_bands, abm.num_prios);
    return -EINVAL;
    }
// Find level and stat symbols
    if (!abm.red_support)
    return 0;
    sym = nfp_abm_ctrl_find_q_rtsym(abm, NFP_QLVL_SYM_NAME,
    NFP_QLVL_STRIDE);
    if (IS_ERR(sym))
    return PTR_ERR(sym);
    abm.q_lvls = sym;
    sym = nfp_abm_ctrl_find_q_rtsym(abm, NFP_QMSTAT_SYM_NAME,
    NFP_QMSTAT_STRIDE);
    if (IS_ERR(sym))
    return PTR_ERR(sym);
    abm.qm_stats = sym;
    if (nfp_abm_has_prio(abm)) {
    sym = nfp_abm_ctrl_find_q_rtsym(abm, NFP_Q_STAT_SYM_NAME,
    NFP_Q_STAT_STRIDE);
    if (IS_ERR(sym))
    return PTR_ERR(sym);
    abm.q_stats = sym;
    }
    return 0;
    }
