//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/debugfs.c
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
// debugfs.c - DesignWare USB3 DRD Controller DebugFS file
//
// Copyright (C) 2010-2011 Texas Instruments Incorporated - https://www.ti.com
//
// Authors: Felipe Balbi <balbi@ti.com>,
// Sebastian Andrzej Siewior <bigeasy@linutronix.de>
//

pub const DWC3_LSP_MUX_UNSELECTED: c_uint = 0xfffff;

    {							\
    .name	= __stringify(nm),			\
    .offset	= DWC3_ ##nm,				\
    }

    {					\
    .name = "DEPCMDPAR2("__stringify(n)")",	\
    .offset = DWC3_DEPCMDPAR2(n),	\
    },					\
    {					\
    .name = "DEPCMDPAR1("__stringify(n)")",	\
    .offset = DWC3_DEPCMDPAR1(n),	\
    },					\
    {					\
    .name = "DEPCMDPAR0("__stringify(n)")",	\
    .offset = DWC3_DEPCMDPAR0(n),	\
    },					\
    {					\
    .name = "DEPCMD("__stringify(n)")",	\
    .offset = DWC3_DEPCMD(n),		\
    }
    static const struct debugfs_reg32 dwc3_regs[] = {
    dump_register(GSBUSCFG0),
    dump_register(GSBUSCFG1),
    dump_register(GTXTHRCFG),
    dump_register(GRXTHRCFG),
    dump_register(GCTL),
    dump_register(GEVTEN),
    dump_register(GSTS),
    dump_register(GUCTL1),
    dump_register(GSNPSID),
    dump_register(GGPIO),
    dump_register(GUID),
    dump_register(GUCTL),
    dump_register(GBUSERRADDR0),
    dump_register(GBUSERRADDR1),
    dump_register(GPRTBIMAP0),
    dump_register(GPRTBIMAP1),
    dump_register(GHWPARAMS0),
    dump_register(GHWPARAMS1),
    dump_register(GHWPARAMS2),
    dump_register(GHWPARAMS3),
    dump_register(GHWPARAMS4),
    dump_register(GHWPARAMS5),
    dump_register(GHWPARAMS6),
    dump_register(GHWPARAMS7),
    dump_register(GDBGFIFOSPACE),
    dump_register(GDBGLTSSM),
    dump_register(GDBGBMU),
    dump_register(GPRTBIMAP_HS0),
    dump_register(GPRTBIMAP_HS1),
    dump_register(GPRTBIMAP_FS0),
    dump_register(GPRTBIMAP_FS1),
    dump_register(GUCTL2),
    dump_register(VER_NUMBER),
    dump_register(VER_TYPE),
    dump_register(GUSB2PHYCFG(0)),
    dump_register(GUSB2PHYCFG(1)),
    dump_register(GUSB2PHYCFG(2)),
    dump_register(GUSB2PHYCFG(3)),
    dump_register(GUSB2PHYCFG(4)),
    dump_register(GUSB2PHYCFG(5)),
    dump_register(GUSB2PHYCFG(6)),
    dump_register(GUSB2PHYCFG(7)),
    dump_register(GUSB2PHYCFG(8)),
    dump_register(GUSB2PHYCFG(9)),
    dump_register(GUSB2PHYCFG(10)),
    dump_register(GUSB2PHYCFG(11)),
    dump_register(GUSB2PHYCFG(12)),
    dump_register(GUSB2PHYCFG(13)),
    dump_register(GUSB2PHYCFG(14)),
    dump_register(GUSB2PHYCFG(15)),
    dump_register(GUSB2I2CCTL(0)),
    dump_register(GUSB2I2CCTL(1)),
    dump_register(GUSB2I2CCTL(2)),
    dump_register(GUSB2I2CCTL(3)),
    dump_register(GUSB2I2CCTL(4)),
    dump_register(GUSB2I2CCTL(5)),
    dump_register(GUSB2I2CCTL(6)),
    dump_register(GUSB2I2CCTL(7)),
    dump_register(GUSB2I2CCTL(8)),
    dump_register(GUSB2I2CCTL(9)),
    dump_register(GUSB2I2CCTL(10)),
    dump_register(GUSB2I2CCTL(11)),
    dump_register(GUSB2I2CCTL(12)),
    dump_register(GUSB2I2CCTL(13)),
    dump_register(GUSB2I2CCTL(14)),
    dump_register(GUSB2I2CCTL(15)),
    dump_register(GUSB2PHYACC(0)),
    dump_register(GUSB2PHYACC(1)),
    dump_register(GUSB2PHYACC(2)),
    dump_register(GUSB2PHYACC(3)),
    dump_register(GUSB2PHYACC(4)),
    dump_register(GUSB2PHYACC(5)),
    dump_register(GUSB2PHYACC(6)),
    dump_register(GUSB2PHYACC(7)),
    dump_register(GUSB2PHYACC(8)),
    dump_register(GUSB2PHYACC(9)),
    dump_register(GUSB2PHYACC(10)),
    dump_register(GUSB2PHYACC(11)),
    dump_register(GUSB2PHYACC(12)),
    dump_register(GUSB2PHYACC(13)),
    dump_register(GUSB2PHYACC(14)),
    dump_register(GUSB2PHYACC(15)),
    dump_register(GUSB3PIPECTL(0)),
    dump_register(GUSB3PIPECTL(1)),
    dump_register(GUSB3PIPECTL(2)),
    dump_register(GUSB3PIPECTL(3)),
    dump_register(GUSB3PIPECTL(4)),
    dump_register(GUSB3PIPECTL(5)),
    dump_register(GUSB3PIPECTL(6)),
    dump_register(GUSB3PIPECTL(7)),
    dump_register(GUSB3PIPECTL(8)),
    dump_register(GUSB3PIPECTL(9)),
    dump_register(GUSB3PIPECTL(10)),
    dump_register(GUSB3PIPECTL(11)),
    dump_register(GUSB3PIPECTL(12)),
    dump_register(GUSB3PIPECTL(13)),
    dump_register(GUSB3PIPECTL(14)),
    dump_register(GUSB3PIPECTL(15)),
    dump_register(GTXFIFOSIZ(0)),
    dump_register(GTXFIFOSIZ(1)),
    dump_register(GTXFIFOSIZ(2)),
    dump_register(GTXFIFOSIZ(3)),
    dump_register(GTXFIFOSIZ(4)),
    dump_register(GTXFIFOSIZ(5)),
    dump_register(GTXFIFOSIZ(6)),
    dump_register(GTXFIFOSIZ(7)),
    dump_register(GTXFIFOSIZ(8)),
    dump_register(GTXFIFOSIZ(9)),
    dump_register(GTXFIFOSIZ(10)),
    dump_register(GTXFIFOSIZ(11)),
    dump_register(GTXFIFOSIZ(12)),
    dump_register(GTXFIFOSIZ(13)),
    dump_register(GTXFIFOSIZ(14)),
    dump_register(GTXFIFOSIZ(15)),
    dump_register(GTXFIFOSIZ(16)),
    dump_register(GTXFIFOSIZ(17)),
    dump_register(GTXFIFOSIZ(18)),
    dump_register(GTXFIFOSIZ(19)),
    dump_register(GTXFIFOSIZ(20)),
    dump_register(GTXFIFOSIZ(21)),
    dump_register(GTXFIFOSIZ(22)),
    dump_register(GTXFIFOSIZ(23)),
    dump_register(GTXFIFOSIZ(24)),
    dump_register(GTXFIFOSIZ(25)),
    dump_register(GTXFIFOSIZ(26)),
    dump_register(GTXFIFOSIZ(27)),
    dump_register(GTXFIFOSIZ(28)),
    dump_register(GTXFIFOSIZ(29)),
    dump_register(GTXFIFOSIZ(30)),
    dump_register(GTXFIFOSIZ(31)),
    dump_register(GRXFIFOSIZ(0)),
    dump_register(GRXFIFOSIZ(1)),
    dump_register(GRXFIFOSIZ(2)),
    dump_register(GRXFIFOSIZ(3)),
    dump_register(GRXFIFOSIZ(4)),
    dump_register(GRXFIFOSIZ(5)),
    dump_register(GRXFIFOSIZ(6)),
    dump_register(GRXFIFOSIZ(7)),
    dump_register(GRXFIFOSIZ(8)),
    dump_register(GRXFIFOSIZ(9)),
    dump_register(GRXFIFOSIZ(10)),
    dump_register(GRXFIFOSIZ(11)),
    dump_register(GRXFIFOSIZ(12)),
    dump_register(GRXFIFOSIZ(13)),
    dump_register(GRXFIFOSIZ(14)),
    dump_register(GRXFIFOSIZ(15)),
    dump_register(GRXFIFOSIZ(16)),
    dump_register(GRXFIFOSIZ(17)),
    dump_register(GRXFIFOSIZ(18)),
    dump_register(GRXFIFOSIZ(19)),
    dump_register(GRXFIFOSIZ(20)),
    dump_register(GRXFIFOSIZ(21)),
    dump_register(GRXFIFOSIZ(22)),
    dump_register(GRXFIFOSIZ(23)),
    dump_register(GRXFIFOSIZ(24)),
    dump_register(GRXFIFOSIZ(25)),
    dump_register(GRXFIFOSIZ(26)),
    dump_register(GRXFIFOSIZ(27)),
    dump_register(GRXFIFOSIZ(28)),
    dump_register(GRXFIFOSIZ(29)),
    dump_register(GRXFIFOSIZ(30)),
    dump_register(GRXFIFOSIZ(31)),
    dump_register(GEVNTADRLO(0)),
    dump_register(GEVNTADRHI(0)),
    dump_register(GEVNTSIZ(0)),
    dump_register(GEVNTCOUNT(0)),
    dump_register(GHWPARAMS8),
    dump_register(GUCTL3),
    dump_register(GFLADJ),
    dump_register(DCFG),
    dump_register(DCTL),
    dump_register(DEVTEN),
    dump_register(DSTS),
    dump_register(DGCMDPAR),
    dump_register(DGCMD),
    dump_register(DALEPENA),
    dump_ep_register_set(0),
    dump_ep_register_set(1),
    dump_ep_register_set(2),
    dump_ep_register_set(3),
    dump_ep_register_set(4),
    dump_ep_register_set(5),
    dump_ep_register_set(6),
    dump_ep_register_set(7),
    dump_ep_register_set(8),
    dump_ep_register_set(9),
    dump_ep_register_set(10),
    dump_ep_register_set(11),
    dump_ep_register_set(12),
    dump_ep_register_set(13),
    dump_ep_register_set(14),
    dump_ep_register_set(15),
    dump_ep_register_set(16),
    dump_ep_register_set(17),
    dump_ep_register_set(18),
    dump_ep_register_set(19),
    dump_ep_register_set(20),
    dump_ep_register_set(21),
    dump_ep_register_set(22),
    dump_ep_register_set(23),
    dump_ep_register_set(24),
    dump_ep_register_set(25),
    dump_ep_register_set(26),
    dump_ep_register_set(27),
    dump_ep_register_set(28),
    dump_ep_register_set(29),
    dump_ep_register_set(30),
    dump_ep_register_set(31),
    dump_register(OCFG),
    dump_register(OCTL),
    dump_register(OEVT),
    dump_register(OEVTEN),
    dump_register(OSTS),
    };
#[no_mangle]
unsafe extern "C" fn dwc3_host_lsp(s: *mut seq_file) {
    static void dwc3_host_lsp(struct seq_file *s)
    {
    struct dwc3		*dwc = s.private;
    bool			dbc_enabled;
    u32			sel;
    u32			reg;
    u32			val;
    dbc_enabled = !!(dwc.hwparams.hwparams1 & DWC3_GHWPARAMS1_ENDBC);
    sel = dwc.dbg_lsp_select;
    if (sel == DWC3_LSP_MUX_UNSELECTED) {
    seq_puts(s, "Write LSP selection to print for host\n");
    return;
    }
    reg = DWC3_GDBGLSPMUX_HOSTSELECT(sel);
    dwc3_writel(dwc, DWC3_GDBGLSPMUX, reg);
    val = dwc3_readl(dwc, DWC3_GDBGLSP);
    seq_printf(s, "GDBGLSP[%d] = 0x%08x\n", sel, val);
    if (dbc_enabled && sel < 256) {
    reg |= DWC3_GDBGLSPMUX_ENDBC;
    dwc3_writel(dwc, DWC3_GDBGLSPMUX, reg);
    val = dwc3_readl(dwc, DWC3_GDBGLSP);
    seq_printf(s, "GDBGLSP_DBC[%d] = 0x%08x\n", sel, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn dwc3_gadget_lsp(s: *mut seq_file) {
    static void dwc3_gadget_lsp(struct seq_file *s)
    {
    struct dwc3		*dwc = s.private;
    int			i;
    u32			reg;
    for (i = 0; i < 16; i++) {
    reg = DWC3_GDBGLSPMUX_DEVSELECT(i);
    dwc3_writel(dwc, DWC3_GDBGLSPMUX, reg);
    reg = dwc3_readl(dwc, DWC3_GDBGLSP);
    seq_printf(s, "GDBGLSP[%d] = 0x%08x\n", i, reg);
    }
    }
#[no_mangle]
unsafe extern "C" fn dwc3_lsp_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_lsp_show(struct seq_file *s, void *unused)
    {
    struct dwc3		*dwc = s.private;
    unsigned int		current_mode;
    unsigned long		flags;
    u32			reg;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    reg = dwc3_readl(dwc, DWC3_GSTS);
    current_mode = DWC3_GSTS_CURMOD(reg);
    switch (current_mode) {
    case DWC3_GSTS_CURMOD_HOST:
    dwc3_host_lsp(s);
    break;
    case DWC3_GSTS_CURMOD_DEVICE:
    dwc3_gadget_lsp(s);
    break;
    default:
    seq_puts(s, "Mode is unknown, no LSP register printed\n");
    break;
    }
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_lsp_open(inode: *mut inode, file: *mut file) -> c_int {
    static int dwc3_lsp_open(struct inode *inode, struct file *file)
    {
    return single_open(file, dwc3_lsp_show, inode.i_private);
    }
    static ssize_t dwc3_lsp_write(struct file *file, const char __user *ubuf,
    size_t count, loff_t *ppos)
    {
    struct seq_file		*s = file.private_data;
    struct dwc3		*dwc = s.private;
    unsigned long		flags;
    char			buf[32] = { 0 };
    u32			sel;
    int			ret;
    if (copy_from_user(&buf, ubuf, min_t(size_t, sizeof(buf) - 1, count)))
    return -EFAULT;
    ret = kstrtouint(buf, 0, &sel);
    if (ret)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    dwc.dbg_lsp_select = sel;
    spin_unlock_irqrestore(&dwc.lock, flags);
    return count;
    }
    static const struct file_operations dwc3_lsp_fops = {
    .open			= dwc3_lsp_open,
    .write			= dwc3_lsp_write,
    .read			= seq_read,
    .llseek			= seq_lseek,
    .release		= single_release,
    };
#[no_mangle]
unsafe extern "C" fn dwc3_mode_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_mode_show(struct seq_file *s, void *unused)
    {
    struct dwc3		*dwc = s.private;
    unsigned long		flags;
    u32			reg;
    u32			mode;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    reg = dwc3_readl(dwc, DWC3_GCTL);
    spin_unlock_irqrestore(&dwc.lock, flags);
    mode = DWC3_GCTL_PRTCAP(reg);
    switch (mode) {
    case DWC3_GCTL_PRTCAP_HOST:
    case DWC3_GCTL_PRTCAP_DEVICE:
    case DWC3_GCTL_PRTCAP_OTG:
    seq_printf(s, "%s\n", dwc3_mode_string(mode));
    break;
    default:
    seq_printf(s, "UNKNOWN %08x\n", mode);
    }
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_mode_open(inode: *mut inode, file: *mut file) -> c_int {
    static int dwc3_mode_open(struct inode *inode, struct file *file)
    {
    return single_open(file, dwc3_mode_show, inode.i_private);
    }
    static ssize_t dwc3_mode_write(struct file *file,
    const char __user *ubuf, size_t count, loff_t *ppos)
    {
    struct seq_file		*s = file.private_data;
    struct dwc3		*dwc = s.private;
    let mut mode: u32 = 0;
    char			buf[32];
    if (copy_from_user(&buf, ubuf, min_t(size_t, sizeof(buf) - 1, count)))
    return -EFAULT;
    if (dwc.dr_mode != USB_DR_MODE_OTG)
    return count;
    if (!strncmp(buf, "host", 4))
    mode = DWC3_GCTL_PRTCAP_HOST;
    if (!strncmp(buf, "device", 6))
    mode = DWC3_GCTL_PRTCAP_DEVICE;
    if (!strncmp(buf, "otg", 3))
    mode = DWC3_GCTL_PRTCAP_OTG;
    dwc3_set_mode(dwc, mode);
    return count;
    }
    static const struct file_operations dwc3_mode_fops = {
    .open			= dwc3_mode_open,
    .write			= dwc3_mode_write,
    .read			= seq_read,
    .llseek			= seq_lseek,
    .release		= single_release,
    };
#[no_mangle]
unsafe extern "C" fn dwc3_testmode_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_testmode_show(struct seq_file *s, void *unused)
    {
    struct dwc3		*dwc = s.private;
    unsigned long		flags;
    u32			reg;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    reg = dwc3_readl(dwc, DWC3_DCTL);
    reg &= DWC3_DCTL_TSTCTRL_MASK;
    reg >>= 1;
    spin_unlock_irqrestore(&dwc.lock, flags);
    switch (reg) {
    case 0:
    seq_puts(s, "no test\n");
    break;
    case USB_TEST_J:
    seq_puts(s, "test_j\n");
    break;
    case USB_TEST_K:
    seq_puts(s, "test_k\n");
    break;
    case USB_TEST_SE0_NAK:
    seq_puts(s, "test_se0_nak\n");
    break;
    case USB_TEST_PACKET:
    seq_puts(s, "test_packet\n");
    break;
    case USB_TEST_FORCE_ENABLE:
    seq_puts(s, "test_force_enable\n");
    break;
    default:
    seq_printf(s, "UNKNOWN %d\n", reg);
    }
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_testmode_open(inode: *mut inode, file: *mut file) -> c_int {
    static int dwc3_testmode_open(struct inode *inode, struct file *file)
    {
    return single_open(file, dwc3_testmode_show, inode.i_private);
    }
    static ssize_t dwc3_testmode_write(struct file *file,
    const char __user *ubuf, size_t count, loff_t *ppos)
    {
    struct seq_file		*s = file.private_data;
    struct dwc3		*dwc = s.private;
    unsigned long		flags;
    let mut testmode: u32 = 0;
    char			buf[32];
    int			ret;
    if (copy_from_user(&buf, ubuf, min_t(size_t, sizeof(buf) - 1, count)))
    return -EFAULT;
    if (!strncmp(buf, "test_j", 6))
    testmode = USB_TEST_J;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "test_k", _arg: 6)) -> else {
    else if (!strncmp(buf, "test_k", 6))
    testmode = USB_TEST_K;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "test_se0_nak", _arg: 12)) -> else {
    else if (!strncmp(buf, "test_se0_nak", 12))
    testmode = USB_TEST_SE0_NAK;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "test_packet", _arg: 11)) -> else {
    else if (!strncmp(buf, "test_packet", 11))
    testmode = USB_TEST_PACKET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "test_force_enable", _arg: 17)) -> else {
    else if (!strncmp(buf, "test_force_enable", 17))
    testmode = USB_TEST_FORCE_ENABLE;
    else
    testmode = 0;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    dwc3_gadget_set_test_mode(dwc, testmode);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return count;
    }
    static const struct file_operations dwc3_testmode_fops = {
    .open			= dwc3_testmode_open,
    .write			= dwc3_testmode_write,
    .read			= seq_read,
    .llseek			= seq_lseek,
    .release		= single_release,
    };
#[no_mangle]
unsafe extern "C" fn dwc3_link_state_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_link_state_show(struct seq_file *s, void *unused)
    {
    struct dwc3		*dwc = s.private;
    unsigned long		flags;
    enum dwc3_link_state	state;
    u32			reg;
    u8			speed;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    reg = dwc3_readl(dwc, DWC3_GSTS);
    if (DWC3_GSTS_CURMOD(reg) != DWC3_GSTS_CURMOD_DEVICE) {
    seq_puts(s, "Not available\n");
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
    reg = dwc3_readl(dwc, DWC3_DSTS);
    state = DWC3_DSTS_USBLNKST(reg);
    speed = reg & DWC3_DSTS_CONNECTSPD;
    seq_printf(s, "%s\n", (speed >= DWC3_DSTS_SUPERSPEED) ?
    dwc3_gadget_link_string(state) :
    dwc3_gadget_hs_link_string(state));
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_link_state_open(inode: *mut inode, file: *mut file) -> c_int {
    static int dwc3_link_state_open(struct inode *inode, struct file *file)
    {
    return single_open(file, dwc3_link_state_show, inode.i_private);
    }
    static ssize_t dwc3_link_state_write(struct file *file,
    const char __user *ubuf, size_t count, loff_t *ppos)
    {
    struct seq_file		*s = file.private_data;
    struct dwc3		*dwc = s.private;
    unsigned long		flags;
    let mut state: enum dwc3_link_state = 0;
    char			buf[32];
    u32			reg;
    u8			speed;
    int			ret;
    if (copy_from_user(&buf, ubuf, min_t(size_t, sizeof(buf) - 1, count)))
    return -EFAULT;
    if (!strncmp(buf, "SS.Disabled", 11))
    state = DWC3_LINK_STATE_SS_DIS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "Rx.Detect", _arg: 9)) -> else {
    else if (!strncmp(buf, "Rx.Detect", 9))
    state = DWC3_LINK_STATE_RX_DET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "SS.Inactive", _arg: 11)) -> else {
    else if (!strncmp(buf, "SS.Inactive", 11))
    state = DWC3_LINK_STATE_SS_INACT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "Recovery", _arg: 8)) -> else {
    else if (!strncmp(buf, "Recovery", 8))
    state = DWC3_LINK_STATE_RECOV;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "Compliance", _arg: 10)) -> else {
    else if (!strncmp(buf, "Compliance", 10))
    state = DWC3_LINK_STATE_CMPLY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "Loopback", _arg: 8)) -> else {
    else if (!strncmp(buf, "Loopback", 8))
    state = DWC3_LINK_STATE_LPBK;
    else
    return -EINVAL;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    reg = dwc3_readl(dwc, DWC3_GSTS);
    if (DWC3_GSTS_CURMOD(reg) != DWC3_GSTS_CURMOD_DEVICE) {
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return -EINVAL;
    }
    reg = dwc3_readl(dwc, DWC3_DSTS);
    speed = reg & DWC3_DSTS_CONNECTSPD;
    if (speed < DWC3_DSTS_SUPERSPEED &&
    state != DWC3_LINK_STATE_RECOV) {
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return -EINVAL;
    }
    dwc3_gadget_set_link_state(dwc, state);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return count;
    }
    static const struct file_operations dwc3_link_state_fops = {
    .open			= dwc3_link_state_open,
    .write			= dwc3_link_state_write,
    .read			= seq_read,
    .llseek			= seq_lseek,
    .release		= single_release,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_ep_file_map {
    pub name: [c_char; 25],
    pub fops: *const *const file_operations,
}

#[no_mangle]
unsafe extern "C" fn dwc3_tx_fifo_size_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_tx_fifo_size_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    u32			mdwidth;
    u32			val;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    val = dwc3_core_fifo_space(dep, DWC3_TXFIFO);
// Convert to bytes
    mdwidth = dwc3_mdwidth(dwc);
    val *= mdwidth;
    val >>= 3;
    seq_printf(s, "%u\n", val);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rx_fifo_size_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_rx_fifo_size_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    u32			mdwidth;
    u32			val;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    val = dwc3_core_fifo_space(dep, DWC3_RXFIFO);
// Convert to bytes
    mdwidth = dwc3_mdwidth(dwc);
    val *= mdwidth;
    val >>= 3;
    seq_printf(s, "%u\n", val);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_tx_request_queue_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_tx_request_queue_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    u32			val;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    val = dwc3_core_fifo_space(dep, DWC3_TXREQQ);
    seq_printf(s, "%u\n", val);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rx_request_queue_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_rx_request_queue_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    u32			val;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    val = dwc3_core_fifo_space(dep, DWC3_RXREQQ);
    seq_printf(s, "%u\n", val);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rx_info_queue_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_rx_info_queue_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    u32			val;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    val = dwc3_core_fifo_space(dep, DWC3_RXINFOQ);
    seq_printf(s, "%u\n", val);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_descriptor_fetch_queue_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_descriptor_fetch_queue_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    u32			val;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    val = dwc3_core_fifo_space(dep, DWC3_DESCFETCHQ);
    seq_printf(s, "%u\n", val);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_event_queue_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_event_queue_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    u32			val;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    val = dwc3_core_fifo_space(dep, DWC3_EVENTQ);
    seq_printf(s, "%u\n", val);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_transfer_type_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_transfer_type_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    spin_lock_irqsave(&dwc.lock, flags);
    if (!(dep.flags & DWC3_EP_ENABLED) || !dep.endpoint.desc) {
    seq_puts(s, "--\n");
    goto out;
    }
    switch (usb_endpoint_type(dep.endpoint.desc)) {
    case USB_ENDPOINT_XFER_CONTROL:
    seq_puts(s, "control\n");
    break;
    case USB_ENDPOINT_XFER_ISOC:
    seq_puts(s, "isochronous\n");
    break;
    case USB_ENDPOINT_XFER_BULK:
    seq_puts(s, "bulk\n");
    break;
    case USB_ENDPOINT_XFER_INT:
    seq_puts(s, "interrupt\n");
    break;
    default:
    seq_puts(s, "--\n");
    }
    out:
    spin_unlock_irqrestore(&dwc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_trb_ring_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_trb_ring_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    int			i;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    if (dep.number <= 1) {
    seq_puts(s, "--\n");
    goto out;
    }
    seq_puts(s, "buffer_addr,size,type,ioc,isp_imi,csp,chn,lst,hwo\n");
    for (i = 0; i < DWC3_TRB_NUM; i++) {
    struct dwc3_trb *trb = &dep.trb_pool[i];
    let mut type: c_uint = DWC3_TRBCTL_TYPE(trb.ctrl);
    seq_printf(s, "%08x%08x,%d,%s,%d,%d,%d,%d,%d,%d       %c%c\n",
    trb.bph, trb.bpl, trb.size,
    dwc3_trb_type_string(type),
    !!(trb.ctrl & DWC3_TRB_CTRL_IOC),
    !!(trb.ctrl & DWC3_TRB_CTRL_ISP_IMI),
    !!(trb.ctrl & DWC3_TRB_CTRL_CSP),
    !!(trb.ctrl & DWC3_TRB_CTRL_CHN),
    !!(trb.ctrl & DWC3_TRB_CTRL_LST),
    !!(trb.ctrl & DWC3_TRB_CTRL_HWO),
    dep.trb_enqueue == i ? 'E' : ' ',
    dep.trb_dequeue == i ? 'D' : ' ');
    }
    out:
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep_info_register_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dwc3_ep_info_register_show(struct seq_file *s, void *unused)
    {
    struct dwc3_ep		*dep = s.private;
    struct dwc3		*dwc = dep.dwc;
    unsigned long		flags;
    u64			ep_info;
    u32			lower_32_bits;
    u32			upper_32_bits;
    u32			reg;
    int			ret;
    ret = pm_runtime_resume_and_get(dwc.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&dwc.lock, flags);
    reg = DWC3_GDBGLSPMUX_EPSELECT(dep.number);
    dwc3_writel(dwc, DWC3_GDBGLSPMUX, reg);
    lower_32_bits = dwc3_readl(dwc, DWC3_GDBGEPINFO0);
    upper_32_bits = dwc3_readl(dwc, DWC3_GDBGEPINFO1);
    ep_info = ((u64)upper_32_bits << 32) | lower_32_bits;
    seq_printf(s, "0x%016llx\n", ep_info);
    spin_unlock_irqrestore(&dwc.lock, flags);
    pm_runtime_put_sync(dwc.dev);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(dwc3_tx_fifo_size);
    DEFINE_SHOW_ATTRIBUTE(dwc3_rx_fifo_size);
    DEFINE_SHOW_ATTRIBUTE(dwc3_tx_request_queue);
    DEFINE_SHOW_ATTRIBUTE(dwc3_rx_request_queue);
    DEFINE_SHOW_ATTRIBUTE(dwc3_rx_info_queue);
    DEFINE_SHOW_ATTRIBUTE(dwc3_descriptor_fetch_queue);
    DEFINE_SHOW_ATTRIBUTE(dwc3_event_queue);
    DEFINE_SHOW_ATTRIBUTE(dwc3_transfer_type);
    DEFINE_SHOW_ATTRIBUTE(dwc3_trb_ring);
    DEFINE_SHOW_ATTRIBUTE(dwc3_ep_info_register);
    static const struct dwc3_ep_file_map dwc3_ep_file_map[] = {
    { "tx_fifo_size", &dwc3_tx_fifo_size_fops, },
    { "rx_fifo_size", &dwc3_rx_fifo_size_fops, },
    { "tx_request_queue", &dwc3_tx_request_queue_fops, },
    { "rx_request_queue", &dwc3_rx_request_queue_fops, },
    { "rx_info_queue", &dwc3_rx_info_queue_fops, },
    { "descriptor_fetch_queue", &dwc3_descriptor_fetch_queue_fops, },
    { "event_queue", &dwc3_event_queue_fops, },
    { "transfer_type", &dwc3_transfer_type_fops, },
    { "trb_ring", &dwc3_trb_ring_fops, },
    { "GDBGEPINFO", &dwc3_ep_info_register_fops, },
    };
#[no_mangle]
pub unsafe extern "C" fn dwc3_debugfs_create_endpoint_dir(dep: *mut dwc3_ep) {
    void dwc3_debugfs_create_endpoint_dir(struct dwc3_ep *dep)
    {
    struct dentry		*dir;
    int			i;
    dir = debugfs_create_dir(dep.name, dep.dwc.debug_root);
    for (i = 0; i < ARRAY_SIZE(dwc3_ep_file_map); i++) {
    const struct file_operations *fops = dwc3_ep_file_map[i].fops;
    const char *name = dwc3_ep_file_map[i].name;
    debugfs_create_file(name, 0444, dir, dep, fops);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_debugfs_remove_endpoint_dir(dep: *mut dwc3_ep) {
    void dwc3_debugfs_remove_endpoint_dir(struct dwc3_ep *dep)
    {
    debugfs_lookup_and_remove(dep.name, dep.dwc.debug_root);
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_debugfs_init(dwc: *mut dwc3) {
    void dwc3_debugfs_init(struct dwc3 *dwc)
    {
    struct dentry		*root;
    dwc.regset = kzalloc_obj(*dwc.regset);
    if (!dwc.regset)
    return;
    dwc.dbg_lsp_select = DWC3_LSP_MUX_UNSELECTED;
    dwc.regset.regs = dwc3_regs;
    dwc.regset.nregs = ARRAY_SIZE(dwc3_regs);
    dwc.regset.base = dwc.regs - DWC3_GLOBALS_REGS_START;
    dwc.regset.dev = dwc.dev;
    root = debugfs_create_dir(dev_name(dwc.dev), usb_debug_root);
    dwc.debug_root = root;
    debugfs_create_regset32("regdump", 0444, root, dwc.regset);
    debugfs_create_file("lsp_dump", 0644, root, dwc, &dwc3_lsp_fops);
    if (IS_ENABLED(CONFIG_USB_DWC3_DUAL_ROLE))
    debugfs_create_file("mode", 0644, root, dwc,
    &dwc3_mode_fops);
    if (IS_ENABLED(CONFIG_USB_DWC3_DUAL_ROLE) ||
    IS_ENABLED(CONFIG_USB_DWC3_GADGET)) {
    debugfs_create_file("testmode", 0644, root, dwc,
    &dwc3_testmode_fops);
    debugfs_create_file("link_state", 0644, root, dwc,
    &dwc3_link_state_fops);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_debugfs_exit(dwc: *mut dwc3) {
    void dwc3_debugfs_exit(struct dwc3 *dwc)
    {
    debugfs_lookup_and_remove(dev_name(dwc.dev), usb_debug_root);
    kfree(dwc.regset);
    }
