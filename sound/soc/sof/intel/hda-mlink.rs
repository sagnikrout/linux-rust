//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/intel/hda-mlink.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Intel Corporation
//
// Management of HDaudio multi-link (capabilities, power, coupling)
//

// worst-case number of sublinks is used for sublink refcount array allocation only

//
// struct hdac_ext2_link - HDAudio extended+alternate link
//
// @hext_link:		hdac_ext_link
// @alt:		flag set for alternate extended links
// @intc:		boolean for interrupt capable
// @ofls:		boolean for offload support
// @lss:		boolean for link synchronization capabilities
// @slcount:		sublink count
// @elid:		extended link ID (AZX_REG_ML_LEPTR_ID_ defines)
// @elver:		extended link version
// @leptr:		extended link pointer
// @eml_lock:		mutual exclusion to access shared registers e.g. CPA/SPA bits
// in LCTL register
// @sublink_ref_count:	array of refcounts, required to power-manage sublinks independently
// @base_ptr:		pointer to shim/ip/shim_vs space
// @instance_offset:	offset between each of @slcount instances managed by link
// @shim_offset:	offset to SHIM register base
// @ip_offset:		offset to IP register base
// @shim_vs_offset:	offset to vendor-specific (VS) SHIM base
// @mic_privacy_mask:	bitmask of sublinks where mic privacy is applied
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_ext2_link {
    pub hext_link: hdac_ext_link,
// read directly from LCAP register
    pub alt: bool,
    pub intc: bool,
    pub ofls: bool,
    pub lss: bool,
    pub slcount: c_int,
    pub elid: c_int,
    pub elver: c_int,
    pub leptr: u32,
    pub /: *mut *mut mutex eml_lock; / prevent concurrent access to e.g. CPA/SPA,
    pub sublink_ref_count: [c_int; HDAML_MAX_SUBLINKS],
// internal values computed from LCAP contents
    pub base_ptr: *mut void __iomem,
    pub instance_offset: u32,
    pub shim_offset: u32,
    pub ip_offset: u32,
    pub shim_vs_offset: u32,
    pub mic_privacy_mask: c_ulong,
}

pub const AZX_REG_SDW_INSTANCE_OFFSET: c_uint = 0x8000;
pub const AZX_REG_SDW_SHIM_OFFSET: c_uint = 0x0;
pub const AZX_REG_SDW_IP_OFFSET: c_uint = 0x100;
pub const AZX_REG_SDW_VS_SHIM_OFFSET: c_uint = 0x6000;

// only one instance supported
pub const AZX_REG_INTEL_DMIC_SHIM_OFFSET: c_uint = 0x0;
pub const AZX_REG_INTEL_DMIC_IP_OFFSET: c_uint = 0x100;
pub const AZX_REG_INTEL_DMIC_VS_SHIM_OFFSET: c_uint = 0x6000;
pub const AZX_REG_INTEL_SSP_INSTANCE_OFFSET: c_uint = 0x1000;
pub const AZX_REG_INTEL_SSP_SHIM_OFFSET: c_uint = 0x0;
pub const AZX_REG_INTEL_SSP_IP_OFFSET: c_uint = 0x100;
pub const AZX_REG_INTEL_SSP_VS_SHIM_OFFSET: c_uint = 0xC00;
// only one instance supported
pub const AZX_REG_INTEL_UAOL_SHIM_OFFSET: c_uint = 0x0;
pub const AZX_REG_INTEL_UAOL_IP_OFFSET: c_uint = 0x100;
pub const AZX_REG_INTEL_UAOL_VS_SHIM_OFFSET: c_uint = 0xC00;
// Microphone privacy
pub const AZX_REG_INTEL_VS_SHIM_PVCCS: c_uint = 0x10;

// HDAML section - this part follows sequences in the hardware specification,
// including naming conventions and the use of the hdaml_ prefix.
// The code is intentionally minimal with limited dependencies on frameworks or
// helpers. Locking and scanning lists is handled at a higher level
//
    static int hdaml_lnk_enum(struct device *dev, struct hdac_ext2_link *h2link,
    void __iomem *remap_addr, void __iomem *ml_addr, int link_idx)
    {
    struct hdac_ext_link *hlink = &h2link.hext_link;
    u32 base_offset;
    hlink.lcaps  = readl(ml_addr + AZX_REG_ML_LCAP);
    h2link.alt = FIELD_GET(AZX_ML_HDA_LCAP_ALT, hlink.lcaps);
// handle alternate extensions
    if (!h2link.alt) {
    h2link.slcount = 1;
//
// LSDIID is initialized by hardware for HDaudio link,
// it needs to be setup by software for alternate links
//
    hlink.lsdiid = readw(ml_addr + AZX_REG_ML_LSDIID);
    dev_dbg(dev, "Link %d: HDAudio - lsdiid=%d\n",
    link_idx, hlink.lsdiid);
    return 0;
    }
    h2link.intc = FIELD_GET(AZX_ML_HDA_LCAP_INTC, hlink.lcaps);
    h2link.ofls = FIELD_GET(AZX_ML_HDA_LCAP_OFLS, hlink.lcaps);
    h2link.lss = FIELD_GET(AZX_ML_HDA_LCAP_LSS, hlink.lcaps);
// read slcount (increment due to zero-based hardware representation
    h2link.slcount = FIELD_GET(AZX_ML_HDA_LCAP_SLCOUNT, hlink.lcaps) + 1;
    dev_dbg(dev, "Link %d: HDAudio extended - sublink count %d\n",
    link_idx, h2link.slcount);
// find IP ID and offsets
    h2link.leptr = readl(ml_addr + AZX_REG_ML_LEPTR);
    h2link.elid = FIELD_GET(AZX_REG_ML_LEPTR_ID, h2link.leptr);
    base_offset = FIELD_GET(AZX_REG_ML_LEPTR_PTR, h2link.leptr);
    h2link.base_ptr = remap_addr + base_offset;
    switch (h2link.elid) {
    case AZX_REG_ML_LEPTR_ID_SDW:
    h2link.instance_offset = AZX_REG_SDW_INSTANCE_OFFSET;
    h2link.shim_offset = AZX_REG_SDW_SHIM_OFFSET;
    h2link.ip_offset = AZX_REG_SDW_IP_OFFSET;
    h2link.shim_vs_offset = AZX_REG_SDW_VS_SHIM_OFFSET;
    dev_dbg(dev, "Link %d: HDAudio extended - SoundWire alternate link, leptr.ptr %#x\n",
    link_idx, base_offset);
    break;
    case AZX_REG_ML_LEPTR_ID_INTEL_DMIC:
    h2link.shim_offset = AZX_REG_INTEL_DMIC_SHIM_OFFSET;
    h2link.ip_offset = AZX_REG_INTEL_DMIC_IP_OFFSET;
    h2link.shim_vs_offset = AZX_REG_INTEL_DMIC_VS_SHIM_OFFSET;
    dev_dbg(dev, "Link %d: HDAudio extended - INTEL DMIC alternate link, leptr.ptr %#x\n",
    link_idx, base_offset);
    break;
    case AZX_REG_ML_LEPTR_ID_INTEL_SSP:
    h2link.instance_offset = AZX_REG_INTEL_SSP_INSTANCE_OFFSET;
    h2link.shim_offset = AZX_REG_INTEL_SSP_SHIM_OFFSET;
    h2link.ip_offset = AZX_REG_INTEL_SSP_IP_OFFSET;
    h2link.shim_vs_offset = AZX_REG_INTEL_SSP_VS_SHIM_OFFSET;
    dev_dbg(dev, "Link %d: HDAudio extended - INTEL SSP alternate link, leptr.ptr %#x\n",
    link_idx, base_offset);
    break;
    case AZX_REG_ML_LEPTR_ID_INTEL_UAOL:
    h2link.shim_offset = AZX_REG_INTEL_UAOL_SHIM_OFFSET;
    h2link.ip_offset = AZX_REG_INTEL_UAOL_IP_OFFSET;
    h2link.shim_vs_offset = AZX_REG_INTEL_UAOL_VS_SHIM_OFFSET;
    dev_dbg(dev, "Link %d: HDAudio extended - INTEL UAOL alternate link, leptr.ptr %#x\n",
    link_idx, base_offset);
    break;
    default:
    dev_err(dev, "Link %d: HDAudio extended - Unsupported alternate link, leptr.id=%#02x value\n",
    link_idx, h2link.elid);
    return -EINVAL;
    }
    return 0;
    }
//
// Hardware recommendations are to wait ~10us before checking any hardware transition
// reported by bits changing status.
// This value does not need to be super-precise, a slack of 5us is perfectly acceptable.
// The worst-case is about 1ms before reporting an issue
//
pub const HDAML_POLL_DELAY_MIN_US: c_int = 10;
pub const HDAML_POLL_DELAY_SLACK_US: c_int = 5;
pub const HDAML_POLL_DELAY_RETRY: c_int = 100;
#[no_mangle]
unsafe extern "C" fn check_sublink_power(lctl: *mut u32 __iomem, sublink: c_int, enabled: bool) -> c_int {
    static int check_sublink_power(u32 __iomem *lctl, int sublink, bool enabled)
    {
    let mut mask: c_int = BIT(sublink) << AZX_ML_LCTL_CPA_SHIFT;
    let mut retry: c_int = HDAML_POLL_DELAY_RETRY;
    u32 val;
    usleep_range(HDAML_POLL_DELAY_MIN_US,
    HDAML_POLL_DELAY_MIN_US + HDAML_POLL_DELAY_SLACK_US);
    do {
    val = readl(lctl);
    if (enabled) {
    if (val & mask)
    return 0;
    } else {
    if (!(val & mask))
    return 0;
    }
    usleep_range(HDAML_POLL_DELAY_MIN_US,
    HDAML_POLL_DELAY_MIN_US + HDAML_POLL_DELAY_SLACK_US);
    } while (--retry);
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_init(lctl: *mut u32 __iomem, sublink: c_int) -> c_int {
    static int hdaml_link_init(u32 __iomem *lctl, int sublink)
    {
    u32 val;
    let mut mask: u32 = BIT(sublink) << AZX_ML_LCTL_SPA_SHIFT;
    val = readl(lctl);
    val |= mask;
    writel(val, lctl);
    return check_sublink_power(lctl, sublink, true);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_shutdown(lctl: *mut u32 __iomem, sublink: c_int) -> c_int {
    static int hdaml_link_shutdown(u32 __iomem *lctl, int sublink)
    {
    u32 val;
    u32 mask;
    val = readl(lctl);
    mask = BIT(sublink) << AZX_ML_LCTL_SPA_SHIFT;
    val &= ~mask;
    writel(val, lctl);
    return check_sublink_power(lctl, sublink, false);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_enable_interrupt(lctl: *mut u32 __iomem, enable: bool) {
    static void hdaml_link_enable_interrupt(u32 __iomem *lctl, bool enable)
    {
    u32 val;
    val = readl(lctl);
    if (enable)
    val |= AZX_ML_LCTL_INTEN;
    else
    val &= ~AZX_ML_LCTL_INTEN;
    writel(val, lctl);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_check_interrupt(lctl: *mut u32 __iomem) -> bool {
    static bool hdaml_link_check_interrupt(u32 __iomem *lctl)
    {
    u32 val;
    val = readl(lctl);
    return val & AZX_ML_LCTL_INTSTS;
    }
#[no_mangle]
unsafe extern "C" fn hdaml_wait_bit(base: *mut void __iomem, offset: c_int, mask: u32, target: u32) -> c_int {
    static int hdaml_wait_bit(void __iomem *base, int offset, u32 mask, u32 target)
    {
    let mut timeout: c_int = HDAML_POLL_DELAY_RETRY;
    u32 reg_read;
    do {
    reg_read = readl(base + offset);
    if ((reg_read & mask) == target)
    return 0;
    timeout--;
    usleep_range(HDAML_POLL_DELAY_MIN_US,
    HDAML_POLL_DELAY_MIN_US + HDAML_POLL_DELAY_SLACK_US);
    } while (timeout != 0);
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_set_syncprd(lsync: *mut u32 __iomem, syncprd: u32) {
    static void hdaml_link_set_syncprd(u32 __iomem *lsync, u32 syncprd)
    {
    u32 val;
    val = readl(lsync);
    val &= ~AZX_REG_ML_LSYNC_SYNCPRD;
    val |= (syncprd & AZX_REG_ML_LSYNC_SYNCPRD);
//
// set SYNCPU but do not wait. The bit is cleared by hardware when
// the link becomes active.
//
    val |= AZX_REG_ML_LSYNC_SYNCPU;
    writel(val, lsync);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_wait_syncpu(lsync: *mut u32 __iomem) -> c_int {
    static int hdaml_link_wait_syncpu(u32 __iomem *lsync)
    {
    return hdaml_wait_bit(lsync, 0, AZX_REG_ML_LSYNC_SYNCPU, 0);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_sync_arm(lsync: *mut u32 __iomem, sublink: c_int) {
    static void hdaml_link_sync_arm(u32 __iomem *lsync, int sublink)
    {
    u32 val;
    val = readl(lsync);
    val |= (AZX_REG_ML_LSYNC_CMDSYNC << sublink);
    writel(val, lsync);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_sync_go(lsync: *mut u32 __iomem) {
    static void hdaml_link_sync_go(u32 __iomem *lsync)
    {
    u32 val;
    val = readl(lsync);
    val |= AZX_REG_ML_LSYNC_SYNCGO;
    writel(val, lsync);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_check_cmdsync(lsync: *mut u32 __iomem, cmdsync_mask: u32) -> bool {
    static bool hdaml_link_check_cmdsync(u32 __iomem *lsync, u32 cmdsync_mask)
    {
    u32 val;
    val = readl(lsync);
    return !!(val & cmdsync_mask);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_get_lsdiid(lsdiid: *mut u16 __iomem) -> u16 {
    static u16 hdaml_link_get_lsdiid(u16 __iomem *lsdiid)
    {
    return readw(lsdiid);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_link_set_lsdiid(lsdiid: *mut u16 __iomem, dev_num: c_int) {
    static void hdaml_link_set_lsdiid(u16 __iomem *lsdiid, int dev_num)
    {
    u16 val;
    val = readw(lsdiid);
    val |= BIT(dev_num);
    writew(val, lsdiid);
    }
    static void hdaml_shim_map_stream_ch(u16 __iomem *pcmsycm, int lchan, int hchan,
    int stream_id, int dir)
    {
    u16 val;
    val = readw(pcmsycm);
    u16p_replace_bits(&val, lchan, GENMASK(3, 0));
    u16p_replace_bits(&val, hchan, GENMASK(7, 4));
    u16p_replace_bits(&val, stream_id, GENMASK(13, 8));
    u16p_replace_bits(&val, dir, BIT(15));
    writew(val, pcmsycm);
    }
#[no_mangle]
unsafe extern "C" fn hdaml_lctl_offload_enable(lctl: *mut u32 __iomem, enable: bool) {
    static void hdaml_lctl_offload_enable(u32 __iomem *lctl, bool enable)
    {
    let mut val: u32 = readl(lctl);
    if (enable)
    val |=  AZX_ML_LCTL_OFLEN;
    else
    val &=  ~AZX_ML_LCTL_OFLEN;
    writel(val, lctl);
    }
// END HDAML section
#[no_mangle]
unsafe extern "C" fn hda_ml_alloc_h2link(bus: *mut hdac_bus, index: c_int) -> c_int {
    static int hda_ml_alloc_h2link(struct hdac_bus *bus, int index)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    int ret;
    h2link = kzalloc_obj(*h2link);
    if (!h2link)
    return -ENOMEM;
// basic initialization
    hlink = &h2link.hext_link;
    hlink.index = index;
    hlink.bus = bus;
    hlink.ml_addr = bus.mlcap + AZX_ML_BASE + (AZX_ML_INTERVAL * index);
    ret = hdaml_lnk_enum(bus.dev, h2link, bus.remap_addr, hlink.ml_addr, index);
    if (ret < 0) {
    kfree(h2link);
    return ret;
    }
    mutex_init(&h2link.eml_lock);
    list_add_tail(&hlink.list, &bus.hlink_list);
//
// HDaudio regular links are powered-on by default, the
// refcount needs to be initialized.
//
    if (!h2link.alt)
    hlink.ref_count = 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_init(bus: *mut hdac_bus) -> c_int {
    int hda_bus_ml_init(struct hdac_bus *bus)
    {
    u32 link_count;
    int ret;
    int i;
    if (!bus.mlcap)
    return 0;
// Enumeration is a one time operation, skip if already done
    if (!list_empty(&bus.hlink_list))
    return 0;
    link_count = readl(bus.mlcap + AZX_REG_ML_MLCD) + 1;
    dev_dbg(bus.dev, "HDAudio Multi-Link count: %d\n", link_count);
    for (i = 0; i < link_count; i++) {
    ret = hda_ml_alloc_h2link(bus, i);
    if (ret < 0) {
    hda_bus_ml_free(bus);
    return ret;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(hda_bus_ml_init, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_free(bus: *mut hdac_bus) {
    void hda_bus_ml_free(struct hdac_bus *bus)
    {
    struct hdac_ext_link *hlink, *_h;
    struct hdac_ext2_link *h2link;
    if (!bus.mlcap)
    return;
    list_for_each_entry_safe(hlink, _h, &bus.hlink_list, list) {
    list_del(&hlink.list);
    h2link = hdac_ext_link_to_ext2(hlink);
    mutex_destroy(&h2link.eml_lock);
    kfree(h2link);
    }
    }
    EXPORT_SYMBOL_NS(hda_bus_ml_free, "SND_SOC_SOF_HDA_MLINK");
    static struct hdac_ext2_link *
    find_ext2_link(struct hdac_bus *bus, bool alt, int elid)
    {
    struct hdac_ext_link *hlink;
    list_for_each_entry(hlink, &bus.hlink_list, list) {
    struct hdac_ext2_link *h2link = hdac_ext_link_to_ext2(hlink);
    if (h2link.alt == alt && h2link.elid == elid)
    return h2link;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_get_count(bus: *mut hdac_bus, alt: bool, elid: c_int) -> c_int {
    int hdac_bus_eml_get_count(struct hdac_bus *bus, bool alt, int elid)
    {
    struct hdac_ext2_link *h2link;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return 0;
    return h2link.slcount;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_get_count, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_enable_interrupt_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, enable: bool) {
    void hdac_bus_eml_enable_interrupt_unlocked(struct hdac_bus *bus, bool alt, int elid, bool enable)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return;
    if (!h2link.intc)
    return;
    hlink = &h2link.hext_link;
    hdaml_link_enable_interrupt(hlink.ml_addr + AZX_REG_ML_LCTL, enable);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_enable_interrupt_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_enable_interrupt(bus: *mut hdac_bus, alt: bool, elid: c_int, enable: bool) {
    void hdac_bus_eml_enable_interrupt(struct hdac_bus *bus, bool alt, int elid, bool enable)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return;
    if (!h2link.intc)
    return;
    hlink = &h2link.hext_link;
    scoped_guard(mutex, &h2link.eml_lock)
    hdaml_link_enable_interrupt(hlink.ml_addr + AZX_REG_ML_LCTL, enable);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_enable_interrupt, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_check_interrupt(bus: *mut hdac_bus, alt: bool, elid: c_int) -> bool {
    bool hdac_bus_eml_check_interrupt(struct hdac_bus *bus, bool alt, int elid)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return false;
    if (!h2link.intc)
    return false;
    hlink = &h2link.hext_link;
    return hdaml_link_check_interrupt(hlink.ml_addr + AZX_REG_ML_LCTL);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_check_interrupt, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_set_syncprd_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, syncprd: u32) -> c_int {
    int hdac_bus_eml_set_syncprd_unlocked(struct hdac_bus *bus, bool alt, int elid, u32 syncprd)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return 0;
    if (!h2link.lss)
    return 0;
    hlink = &h2link.hext_link;
    hdaml_link_set_syncprd(hlink.ml_addr + AZX_REG_ML_LSYNC, syncprd);
    return 0;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_set_syncprd_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_set_syncprd_unlocked(bus: *mut hdac_bus, syncprd: u32) -> c_int {
    int hdac_bus_eml_sdw_set_syncprd_unlocked(struct hdac_bus *bus, u32 syncprd)
    {
    return hdac_bus_eml_set_syncprd_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW, syncprd);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_set_syncprd_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_wait_syncpu_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int) -> c_int {
    int hdac_bus_eml_wait_syncpu_unlocked(struct hdac_bus *bus, bool alt, int elid)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return 0;
    if (!h2link.lss)
    return 0;
    hlink = &h2link.hext_link;
    return hdaml_link_wait_syncpu(hlink.ml_addr + AZX_REG_ML_LSYNC);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_wait_syncpu_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_wait_syncpu_unlocked(bus: *mut hdac_bus) -> c_int {
    int hdac_bus_eml_sdw_wait_syncpu_unlocked(struct hdac_bus *bus)
    {
    return hdac_bus_eml_wait_syncpu_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_wait_syncpu_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sync_arm_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int) {
    void hdac_bus_eml_sync_arm_unlocked(struct hdac_bus *bus, bool alt, int elid, int sublink)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return;
    if (!h2link.lss)
    return;
    hlink = &h2link.hext_link;
    hdaml_link_sync_arm(hlink.ml_addr + AZX_REG_ML_LSYNC, sublink);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sync_arm_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_sync_arm_unlocked(bus: *mut hdac_bus, sublink: c_int) {
    void hdac_bus_eml_sdw_sync_arm_unlocked(struct hdac_bus *bus, int sublink)
    {
    hdac_bus_eml_sync_arm_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW, sublink);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_sync_arm_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sync_go_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int) -> c_int {
    int hdac_bus_eml_sync_go_unlocked(struct hdac_bus *bus, bool alt, int elid)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return 0;
    if (!h2link.lss)
    return 0;
    hlink = &h2link.hext_link;
    hdaml_link_sync_go(hlink.ml_addr + AZX_REG_ML_LSYNC);
    return 0;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sync_go_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_sync_go_unlocked(bus: *mut hdac_bus) -> c_int {
    int hdac_bus_eml_sdw_sync_go_unlocked(struct hdac_bus *bus)
    {
    return hdac_bus_eml_sync_go_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_sync_go_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_check_cmdsync_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int) -> bool {
    bool hdac_bus_eml_check_cmdsync_unlocked(struct hdac_bus *bus, bool alt, int elid)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    u32 cmdsync_mask;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return 0;
    if (!h2link.lss)
    return 0;
    hlink = &h2link.hext_link;
    cmdsync_mask = GENMASK(AZX_REG_ML_LSYNC_CMDSYNC_SHIFT + h2link.slcount - 1,
    AZX_REG_ML_LSYNC_CMDSYNC_SHIFT);
    return hdaml_link_check_cmdsync(hlink.ml_addr + AZX_REG_ML_LSYNC,
    cmdsync_mask);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_check_cmdsync_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_check_cmdsync_unlocked(bus: *mut hdac_bus) -> bool {
    bool hdac_bus_eml_sdw_check_cmdsync_unlocked(struct hdac_bus *bus)
    {
    return hdac_bus_eml_check_cmdsync_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_check_cmdsync_unlocked, "SND_SOC_SOF_HDA_MLINK");
    static int hdac_bus_eml_power_up_base(struct hdac_bus *bus, bool alt, int elid, int sublink,
    bool eml_lock)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    let mut ret: c_int = 0;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return -ENODEV;
    if (sublink >= h2link.slcount)
    return -EINVAL;
    hlink = &h2link.hext_link;
    if (eml_lock)
    mutex_lock(&h2link.eml_lock);
    if (!alt) {
    if (++hlink.ref_count > 1)
    goto skip_init;
    } else {
    if (++h2link.sublink_ref_count[sublink] > 1)
    goto skip_init;
    }
    ret = hdaml_link_init(hlink.ml_addr + AZX_REG_ML_LCTL, sublink);
    if ((h2link.mic_privacy_mask & BIT(sublink)) && !ret) {
    u16 __iomem *pvccs = h2link.base_ptr +
    h2link.shim_vs_offset +
    sublink * h2link.instance_offset +
    AZX_REG_INTEL_VS_SHIM_PVCCS;
    let mut val: u16 = readw(pvccs);
    writew(val | AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTSCHGIE, pvccs);
    if (val & AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTS)
    dev_dbg(bus.dev,
    "sublink %d (%d:%d): Mic privacy is enabled\n",
    sublink, alt, elid);
    }
    skip_init:
    if (eml_lock)
    mutex_unlock(&h2link.eml_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_power_up(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int) -> c_int {
    int hdac_bus_eml_power_up(struct hdac_bus *bus, bool alt, int elid, int sublink)
    {
    return hdac_bus_eml_power_up_base(bus, alt, elid, sublink, true);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_power_up, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_power_up_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int) -> c_int {
    int hdac_bus_eml_power_up_unlocked(struct hdac_bus *bus, bool alt, int elid, int sublink)
    {
    return hdac_bus_eml_power_up_base(bus, alt, elid, sublink, false);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_power_up_unlocked, "SND_SOC_SOF_HDA_MLINK");
    static int hdac_bus_eml_power_down_base(struct hdac_bus *bus, bool alt, int elid, int sublink,
    bool eml_lock)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    let mut ret: c_int = 0;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return -ENODEV;
    if (sublink >= h2link.slcount)
    return -EINVAL;
    hlink = &h2link.hext_link;
    if (eml_lock)
    mutex_lock(&h2link.eml_lock);
    if (!alt) {
    if (--hlink.ref_count > 0)
    goto skip_shutdown;
    } else {
    if (--h2link.sublink_ref_count[sublink] > 0)
    goto skip_shutdown;
    }
    if (h2link.mic_privacy_mask & BIT(sublink)) {
    u16 __iomem *pvccs = h2link.base_ptr +
    h2link.shim_vs_offset +
    sublink * h2link.instance_offset +
    AZX_REG_INTEL_VS_SHIM_PVCCS;
    writew(readw(pvccs) & ~AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTSCHGIE, pvccs);
    }
    ret = hdaml_link_shutdown(hlink.ml_addr + AZX_REG_ML_LCTL, sublink);
    skip_shutdown:
    if (eml_lock)
    mutex_unlock(&h2link.eml_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_power_down(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int) -> c_int {
    int hdac_bus_eml_power_down(struct hdac_bus *bus, bool alt, int elid, int sublink)
    {
    return hdac_bus_eml_power_down_base(bus, alt, elid, sublink, true);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_power_down, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_power_down_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int) -> c_int {
    int hdac_bus_eml_power_down_unlocked(struct hdac_bus *bus, bool alt, int elid, int sublink)
    {
    return hdac_bus_eml_power_down_base(bus, alt, elid, sublink, false);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_power_down_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_power_up_unlocked(bus: *mut hdac_bus, sublink: c_int) -> c_int {
    int hdac_bus_eml_sdw_power_up_unlocked(struct hdac_bus *bus, int sublink)
    {
    return hdac_bus_eml_power_up_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW, sublink);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_power_up_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_power_down_unlocked(bus: *mut hdac_bus, sublink: c_int) -> c_int {
    int hdac_bus_eml_sdw_power_down_unlocked(struct hdac_bus *bus, int sublink)
    {
    return hdac_bus_eml_power_down_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW, sublink);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_power_down_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_get_lsdiid_unlocked(bus: *mut hdac_bus, sublink: c_int, lsdiid: *mut u16) -> c_int {
    int hdac_bus_eml_sdw_get_lsdiid_unlocked(struct hdac_bus *bus, int sublink, u16 *lsdiid)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    if (!h2link)
    return -ENODEV;
    hlink = &h2link.hext_link;
// lsdiid = hdaml_link_get_lsdiid(hlink->ml_addr + AZX_REG_ML_LSDIID_OFFSET(sublink));
    return 0;
    } EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_get_lsdiid_unlocked, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_set_lsdiid(bus: *mut hdac_bus, sublink: c_int, dev_num: c_int) -> c_int {
    int hdac_bus_eml_sdw_set_lsdiid(struct hdac_bus *bus, int sublink, int dev_num)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    if (!h2link)
    return -ENODEV;
    hlink = &h2link.hext_link;
    scoped_guard(mutex, &h2link.eml_lock)
    hdaml_link_set_lsdiid(hlink.ml_addr + AZX_REG_ML_LSDIID_OFFSET(sublink), dev_num);
    return 0;
    } EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_set_lsdiid, "SND_SOC_SOF_HDA_MLINK");
//
// the 'y' parameter comes from the PCMSyCM hardware register naming. 'y' refers to the
// PDI index, i.e. the FIFO used for RX or TX
//
    int hdac_bus_eml_sdw_map_stream_ch(struct hdac_bus *bus, int sublink, int y,
    int channel_mask, int stream_id, int dir)
    {
    struct hdac_ext2_link *h2link;
    u16 __iomem *pcmsycm;
    int hchan;
    int lchan;
    u16 val;
    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    if (!h2link)
    return -ENODEV;
    pcmsycm = h2link.base_ptr + h2link.shim_offset +
    h2link.instance_offset * sublink +
    AZX_REG_SDW_SHIM_PCMSyCM(y);
    if (channel_mask) {
    hchan = __fls(channel_mask);
    lchan = __ffs(channel_mask);
    } else {
    hchan = 0;
    lchan = 0;
    }
    scoped_guard(mutex, &h2link.eml_lock)
    hdaml_shim_map_stream_ch(pcmsycm, lchan, hchan, stream_id, dir);
    val = readw(pcmsycm);
    dev_dbg(bus.dev, "sublink %d channel_mask %#x stream_id %d dir %d pcmscm %#x\n",
    sublink, channel_mask, stream_id, dir, val);
    return 0;
    } EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_map_stream_ch, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_reset_losidv(bus: *mut hdac_bus) {
    void hda_bus_ml_reset_losidv(struct hdac_bus *bus)
    {
    struct hdac_ext_link *hlink;
// Reset stream-to-link mapping
    list_for_each_entry(hlink, &bus.hlink_list, list)
    writel(0, hlink.ml_addr + AZX_REG_ML_LOSIDV);
    }
    EXPORT_SYMBOL_NS(hda_bus_ml_reset_losidv, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_link_get_type(hlink: *mut hdac_ext_link) -> enum hda_bus_ml_link_type {
    enum hda_bus_ml_link_type hda_bus_ml_link_get_type(struct hdac_ext_link *hlink)
    {
    struct hdac_ext2_link *h2link = hdac_ext_link_to_ext2(hlink);
    if (!h2link.alt)
    return HDA_BUS_ML_LINK_HDA;
    switch (h2link.elid) {
    case AZX_REG_ML_LEPTR_ID_SDW:
    return HDA_BUS_ML_LINK_SDW;
    case AZX_REG_ML_LEPTR_ID_INTEL_UAOL:
    return HDA_BUS_ML_LINK_UAOL;
    default:
    return HDA_BUS_ML_LINK_OTHER;
    }
    }
    EXPORT_SYMBOL_NS(hda_bus_ml_link_get_type, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_resume(bus: *mut hdac_bus) -> c_int {
    int hda_bus_ml_resume(struct hdac_bus *bus)
    {
    struct hdac_ext_link *hlink;
    int ret;
// power up links that were active before suspend
    list_for_each_entry(hlink, &bus.hlink_list, list) {
    struct hdac_ext2_link *h2link = hdac_ext_link_to_ext2(hlink);
    if (!h2link.alt && hlink.ref_count) {
    ret = snd_hdac_ext_bus_link_power_up(hlink);
    if (ret < 0)
    return ret;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(hda_bus_ml_resume, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_suspend(bus: *mut hdac_bus) -> c_int {
    int hda_bus_ml_suspend(struct hdac_bus *bus)
    {
    struct hdac_ext_link *hlink;
    int ret;
    list_for_each_entry(hlink, &bus.hlink_list, list) {
    struct hdac_ext2_link *h2link = hdac_ext_link_to_ext2(hlink);
    if (!h2link.alt) {
    ret = snd_hdac_ext_bus_link_power_down(hlink);
    if (ret < 0)
    return ret;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(hda_bus_ml_suspend, "SND_SOC_SOF_HDA_MLINK");
    struct mutex *hdac_bus_eml_get_mutex(struct hdac_bus *bus, bool alt, int elid)
    {
    struct hdac_ext2_link *h2link;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return core::ptr::null_mut();
    return &h2link.eml_lock;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_get_mutex, "SND_SOC_SOF_HDA_MLINK");
    struct hdac_ext_link *hdac_bus_eml_ssp_get_hlink(struct hdac_bus *bus)
    {
    struct hdac_ext2_link *h2link;
    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_INTEL_SSP);
    if (!h2link)
    return core::ptr::null_mut();
    return &h2link.hext_link;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_ssp_get_hlink, "SND_SOC_SOF_HDA_MLINK");
    struct hdac_ext_link *hdac_bus_eml_dmic_get_hlink(struct hdac_bus *bus)
    {
    struct hdac_ext2_link *h2link;
    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_INTEL_DMIC);
    if (!h2link)
    return core::ptr::null_mut();
    return &h2link.hext_link;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_dmic_get_hlink, "SND_SOC_SOF_HDA_MLINK");
    struct hdac_ext_link *hdac_bus_eml_sdw_get_hlink(struct hdac_bus *bus)
    {
    struct hdac_ext2_link *h2link;
    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    if (!h2link)
    return core::ptr::null_mut();
    return &h2link.hext_link;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_get_hlink, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_enable_offload(bus: *mut hdac_bus, alt: bool, elid: c_int, enable: bool) {
    void hdac_bus_eml_enable_offload(struct hdac_bus *bus, bool alt, int elid, bool enable)
    {
    struct hdac_ext2_link *h2link;
    struct hdac_ext_link *hlink;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link || !h2link.ofls)
    return;
    hlink = &h2link.hext_link;
    scoped_guard(mutex, &h2link.eml_lock)
    hdaml_lctl_offload_enable(hlink.ml_addr + AZX_REG_ML_LCTL, enable);
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_enable_offload, "SND_SOC_SOF_HDA_MLINK");
    void hdac_bus_eml_set_mic_privacy_mask(struct hdac_bus *bus, bool alt, int elid,
    unsigned long mask)
    {
    struct hdac_ext2_link *h2link;
    if (!mask)
    return;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return;
    if (__fls(mask) > h2link.slcount) {
    dev_warn(bus.dev,
    "%s: invalid sublink mask for %d:%d, slcount %d: %#lx\n",
    __func__, alt, elid, h2link.slcount, mask);
    return;
    }
    dev_dbg(bus.dev, "sublink mask for %d:%d, slcount %d: %#lx\n", alt,
    elid, h2link.slcount, mask);
    h2link.mic_privacy_mask = mask;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_set_mic_privacy_mask, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_is_mic_privacy_changed(bus: *mut hdac_bus, alt: bool, elid: c_int) -> bool {
    bool hdac_bus_eml_is_mic_privacy_changed(struct hdac_bus *bus, bool alt, int elid)
    {
    struct hdac_ext2_link *h2link;
    let mut changed: bool = false;
    u16 __iomem *pvccs;
    int i;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return false;
// The change in privacy state needs to be acked for each link
    for_each_set_bit(i, &h2link.mic_privacy_mask, h2link.slcount) {
    u16 val;
    if (h2link.sublink_ref_count[i] == 0)
    continue;
    pvccs = h2link.base_ptr +
    h2link.shim_vs_offset +
    i * h2link.instance_offset +
    AZX_REG_INTEL_VS_SHIM_PVCCS;
    val = readw(pvccs);
    if (val & AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTSCHG) {
    writew(val, pvccs);
    changed = true;
    }
    }
    return changed;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_is_mic_privacy_changed, "SND_SOC_SOF_HDA_MLINK");
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_get_mic_privacy_state(bus: *mut hdac_bus, alt: bool, elid: c_int) -> bool {
    bool hdac_bus_eml_get_mic_privacy_state(struct hdac_bus *bus, bool alt, int elid)
    {
    struct hdac_ext2_link *h2link;
    u16 __iomem *pvccs;
    bool state;
    int i;
    h2link = find_ext2_link(bus, alt, elid);
    if (!h2link)
    return false;
    for_each_set_bit(i, &h2link.mic_privacy_mask, h2link.slcount) {
    if (h2link.sublink_ref_count[i] == 0)
    continue;
// Return the privacy state from the first active link
    pvccs = h2link.base_ptr +
    h2link.shim_vs_offset +
    i * h2link.instance_offset +
    AZX_REG_INTEL_VS_SHIM_PVCCS;
    state = readw(pvccs) & AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTS;
    dev_dbg(bus.dev, "alt: %d, elid: %d: Mic privacy is %s\n", alt,
    elid, str_enabled_disabled(state));
    return state;
    }
    return false;
    }
    EXPORT_SYMBOL_NS(hdac_bus_eml_get_mic_privacy_state, "SND_SOC_SOF_HDA_MLINK");

    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("SOF support for HDaudio multi-link");
