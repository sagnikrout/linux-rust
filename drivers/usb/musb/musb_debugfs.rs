//! Automatically rewritten from C to Rust
//! Source: drivers/usb/musb/musb_debugfs.c
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
// MUSB OTG driver debugfs support
//
// Copyright 2010 Nokia Corporation
// Contact: Felipe Balbi <felipe.balbi@nokia.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_register_map {
    pub name: *mut c_char,
    pub offset: unsigned,
    pub size: unsigned,
}

    static const struct musb_register_map musb_regmap[] = {
    { "FAddr",	MUSB_FADDR,	8 },
    { "Power",	MUSB_POWER,	8 },
    { "Frame",	MUSB_FRAME,	16 },
    { "Index",	MUSB_INDEX,	8 },
    { "Testmode",	MUSB_TESTMODE,	8 },
    { "TxMaxPp",	MUSB_TXMAXP,	16 },
    { "TxCSRp",	MUSB_TXCSR,	16 },
    { "RxMaxPp",	MUSB_RXMAXP,	16 },
    { "RxCSR",	MUSB_RXCSR,	16 },
    { "RxCount",	MUSB_RXCOUNT,	16 },
    { "IntrRxE",	MUSB_INTRRXE,	16 },
    { "IntrTxE",	MUSB_INTRTXE,	16 },
    { "IntrUsbE",	MUSB_INTRUSBE,	8 },
    { "DevCtl",	MUSB_DEVCTL,	8 },
    { "VControl",	0x68,		32 },
    { "HWVers",	MUSB_HWVERS,	16 },
    { "LinkInfo",	MUSB_LINKINFO,	8 },
    { "VPLen",	MUSB_VPLEN,	8 },
    { "HS_EOF1",	MUSB_HS_EOF1,	8 },
    { "FS_EOF1",	MUSB_FS_EOF1,	8 },
    { "LS_EOF1",	MUSB_LS_EOF1,	8 },
    { "SOFT_RST",	0x7F,		8 },
    { "DMA_CNTLch0",	0x204,	16 },
    { "DMA_ADDRch0",	0x208,	32 },
    { "DMA_COUNTch0",	0x20C,	32 },
    { "DMA_CNTLch1",	0x214,	16 },
    { "DMA_ADDRch1",	0x218,	32 },
    { "DMA_COUNTch1",	0x21C,	32 },
    { "DMA_CNTLch2",	0x224,	16 },
    { "DMA_ADDRch2",	0x228,	32 },
    { "DMA_COUNTch2",	0x22C,	32 },
    { "DMA_CNTLch3",	0x234,	16 },
    { "DMA_ADDRch3",	0x238,	32 },
    { "DMA_COUNTch3",	0x23C,	32 },
    { "DMA_CNTLch4",	0x244,	16 },
    { "DMA_ADDRch4",	0x248,	32 },
    { "DMA_COUNTch4",	0x24C,	32 },
    { "DMA_CNTLch5",	0x254,	16 },
    { "DMA_ADDRch5",	0x258,	32 },
    { "DMA_COUNTch5",	0x25C,	32 },
    { "DMA_CNTLch6",	0x264,	16 },
    { "DMA_ADDRch6",	0x268,	32 },
    { "DMA_COUNTch6",	0x26C,	32 },
    { "DMA_CNTLch7",	0x274,	16 },
    { "DMA_ADDRch7",	0x278,	32 },
    { "DMA_COUNTch7",	0x27C,	32 },
    { "ConfigData",	MUSB_CONFIGDATA,8 },
    { "BabbleCtl",	MUSB_BABBLE_CTL,8 },
    { "TxFIFOsz",	MUSB_TXFIFOSZ,	8 },
    { "RxFIFOsz",	MUSB_RXFIFOSZ,	8 },
    { "TxFIFOadd",	MUSB_TXFIFOADD,	16 },
    { "RxFIFOadd",	MUSB_RXFIFOADD,	16 },
    { "EPInfo",	MUSB_EPINFO,	8 },
    { "RAMInfo",	MUSB_RAMINFO,	8 },
    {  }	/* Terminating Entry */
    };
#[no_mangle]
unsafe extern "C" fn musb_regdump_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int musb_regdump_show(struct seq_file *s, void *unused)
    {
    struct musb		*musb = s.private;
    unsigned		i;
    seq_printf(s, "MUSB (M)HDRC Register Dump\n");
    pm_runtime_get_sync(musb.controller);
    for (i = 0; i < ARRAY_SIZE(musb_regmap); i++) {
    switch (musb_regmap[i].size) {
    case 8:
    seq_printf(s, "%-12s: %02x\n", musb_regmap[i].name,
    musb_readb(musb.mregs, musb_regmap[i].offset));
    break;
    case 16:
    seq_printf(s, "%-12s: %04x\n", musb_regmap[i].name,
    musb_readw(musb.mregs, musb_regmap[i].offset));
    break;
    case 32:
    seq_printf(s, "%-12s: %08x\n", musb_regmap[i].name,
    musb_readl(musb.mregs, musb_regmap[i].offset));
    break;
    }
    }
    pm_runtime_put_autosuspend(musb.controller);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(musb_regdump);
#[no_mangle]
unsafe extern "C" fn musb_test_mode_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int musb_test_mode_show(struct seq_file *s, void *unused)
    {
    struct musb		*musb = s.private;
    unsigned		test;
    pm_runtime_get_sync(musb.controller);
    test = musb_readb(musb.mregs, MUSB_TESTMODE);
    pm_runtime_put_autosuspend(musb.controller);
    if (test == (MUSB_TEST_FORCE_HOST | MUSB_TEST_FORCE_FS))
    seq_printf(s, "force host full-speed\n");
#[no_mangle]
pub unsafe extern "C" fn if(MUSB_TEST_FORCE_HS): test == (MUSB_TEST_FORCE_HOST |) -> else {
    else if (test == (MUSB_TEST_FORCE_HOST | MUSB_TEST_FORCE_HS))
    seq_printf(s, "force host high-speed\n");
#[no_mangle]
pub unsafe extern "C" fn if(MUSB_TEST_FORCE_HOST: test ==) -> else {
    else if (test == MUSB_TEST_FORCE_HOST)
    seq_printf(s, "force host\n");
#[no_mangle]
pub unsafe extern "C" fn if(MUSB_TEST_FIFO_ACCESS: test ==) -> else {
    else if (test == MUSB_TEST_FIFO_ACCESS)
    seq_printf(s, "fifo access\n");
#[no_mangle]
pub unsafe extern "C" fn if(MUSB_TEST_FORCE_FS: test ==) -> else {
    else if (test == MUSB_TEST_FORCE_FS)
    seq_printf(s, "force full-speed\n");
#[no_mangle]
pub unsafe extern "C" fn if(MUSB_TEST_FORCE_HS: test ==) -> else {
    else if (test == MUSB_TEST_FORCE_HS)
    seq_printf(s, "force high-speed\n");
#[no_mangle]
pub unsafe extern "C" fn if(MUSB_TEST_PACKET: test ==) -> else {
    else if (test == MUSB_TEST_PACKET)
    seq_printf(s, "test packet\n");
#[no_mangle]
pub unsafe extern "C" fn if(MUSB_TEST_K: test ==) -> else {
    else if (test == MUSB_TEST_K)
    seq_printf(s, "test K\n");
#[no_mangle]
pub unsafe extern "C" fn if(MUSB_TEST_J: test ==) -> else {
    else if (test == MUSB_TEST_J)
    seq_printf(s, "test J\n");
#[no_mangle]
pub unsafe extern "C" fn if(MUSB_TEST_SE0_NAK: test ==) -> else {
    else if (test == MUSB_TEST_SE0_NAK)
    seq_printf(s, "test SE0 NAK\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn musb_test_mode_open(inode: *mut inode, file: *mut file) -> c_int {
    static int musb_test_mode_open(struct inode *inode, struct file *file)
    {
    return single_open(file, musb_test_mode_show, inode.i_private);
    }
    static ssize_t musb_test_mode_write(struct file *file,
    const char __user *ubuf, size_t count, loff_t *ppos)
    {
    struct seq_file		*s = file.private_data;
    struct musb		*musb = s.private;
    u8			test;
    char			buf[24];
    memset(buf, 0x00, sizeof(buf));
    if (copy_from_user(buf, ubuf, min_t(size_t, sizeof(buf) - 1, count)))
    return -EFAULT;
    pm_runtime_get_sync(musb.controller);
    test = musb_readb(musb.mregs, MUSB_TESTMODE);
    if (test) {
    dev_err(musb.controller, "Error: test mode is already set. "
    "Please do USB Bus Reset to start a new test.\n");
    goto ret;
    }
    if (strstarts(buf, "force host full-speed"))
    test = MUSB_TEST_FORCE_HOST | MUSB_TEST_FORCE_FS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(buf, high-speed"): "force host) -> else {
    else if (strstarts(buf, "force host high-speed"))
    test = MUSB_TEST_FORCE_HOST | MUSB_TEST_FORCE_HS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(buf, host"): "force) -> else {
    else if (strstarts(buf, "force host"))
    test = MUSB_TEST_FORCE_HOST;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(buf, access"): "fifo) -> else {
    else if (strstarts(buf, "fifo access"))
    test = MUSB_TEST_FIFO_ACCESS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(buf, full-speed"): "force) -> else {
    else if (strstarts(buf, "force full-speed"))
    test = MUSB_TEST_FORCE_FS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(buf, high-speed"): "force) -> else {
    else if (strstarts(buf, "force high-speed"))
    test = MUSB_TEST_FORCE_HS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(buf, packet"): "test) -> else {
    test = MUSB_TEST_PACKET;
    musb_load_testpacket(musb);
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(buf, K"): "test) -> else {
    else if (strstarts(buf, "test K"))
    test = MUSB_TEST_K;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(buf, J"): "test) -> else {
    else if (strstarts(buf, "test J"))
    test = MUSB_TEST_J;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(buf, NAK"): "test SE0) -> else {
    else if (strstarts(buf, "test SE0 NAK"))
    test = MUSB_TEST_SE0_NAK;
    musb_writeb(musb.mregs, MUSB_TESTMODE, test);
    ret:
    pm_runtime_put_autosuspend(musb.controller);
    return count;
    }
    static const struct file_operations musb_test_mode_fops = {
    .open			= musb_test_mode_open,
    .write			= musb_test_mode_write,
    .read			= seq_read,
    .llseek			= seq_lseek,
    .release		= single_release,
    };
#[no_mangle]
unsafe extern "C" fn musb_softconnect_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int musb_softconnect_show(struct seq_file *s, void *unused)
    {
    struct musb	*musb = s.private;
    u8		reg;
    int		connect;
    switch (musb_get_state(musb)) {
    case OTG_STATE_A_HOST:
    case OTG_STATE_A_WAIT_BCON:
    pm_runtime_get_sync(musb.controller);
    reg = musb_readb(musb.mregs, MUSB_DEVCTL);
    connect = reg & MUSB_DEVCTL_SESSION ? 1 : 0;
    pm_runtime_put_autosuspend(musb.controller);
    break;
    default:
    connect = -1;
    }
    seq_printf(s, "%d\n", connect);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn musb_softconnect_open(inode: *mut inode, file: *mut file) -> c_int {
    static int musb_softconnect_open(struct inode *inode, struct file *file)
    {
    return single_open(file, musb_softconnect_show, inode.i_private);
    }
    static ssize_t musb_softconnect_write(struct file *file,
    const char __user *ubuf, size_t count, loff_t *ppos)
    {
    struct seq_file		*s = file.private_data;
    struct musb		*musb = s.private;
    char			buf[2];
    u8			reg;
    memset(buf, 0x00, sizeof(buf));
    if (copy_from_user(&buf, ubuf, min_t(size_t, sizeof(buf) - 1, count)))
    return -EFAULT;
    pm_runtime_get_sync(musb.controller);
    if (!strncmp(buf, "0", 1)) {
    switch (musb_get_state(musb)) {
    case OTG_STATE_A_HOST:
    musb_root_disconnect(musb);
    reg = musb_readb(musb.mregs, MUSB_DEVCTL);
    reg &= ~MUSB_DEVCTL_SESSION;
    musb_writeb(musb.mregs, MUSB_DEVCTL, reg);
    break;
    default:
    break;
    }
    } else if (!strncmp(buf, "1", 1)) {
    switch (musb_get_state(musb)) {
    case OTG_STATE_A_WAIT_BCON:
//
// musb_save_context() called in musb_runtime_suspend()
// might cache devctl with SESSION bit cleared during
// soft-disconnect, so specifically set SESSION bit
// here to preserve it for musb_runtime_resume().
//
    musb.context.devctl |= MUSB_DEVCTL_SESSION;
    reg = musb_readb(musb.mregs, MUSB_DEVCTL);
    reg |= MUSB_DEVCTL_SESSION;
    musb_writeb(musb.mregs, MUSB_DEVCTL, reg);
    break;
    default:
    break;
    }
    }
    pm_runtime_put_autosuspend(musb.controller);
    return count;
    }
//
// In host mode, connect/disconnect the bus without physically
// remove the devices.
//
    static const struct file_operations musb_softconnect_fops = {
    .open			= musb_softconnect_open,
    .write			= musb_softconnect_write,
    .read			= seq_read,
    .llseek			= seq_lseek,
    .release		= single_release,
    };
#[no_mangle]
pub unsafe extern "C" fn musb_init_debugfs(musb: *mut musb) {
    void musb_init_debugfs(struct musb *musb)
    {
    struct dentry *root;
    root = debugfs_create_dir(dev_name(musb.controller), usb_debug_root);
    musb.debugfs_root = root;
    debugfs_create_file("regdump", S_IRUGO, root, musb, &musb_regdump_fops);
    debugfs_create_file("testmode", S_IRUGO | S_IWUSR, root, musb,
    &musb_test_mode_fops);
    debugfs_create_file("softconnect", S_IRUGO | S_IWUSR, root, musb,
    &musb_softconnect_fops);
    }
    void /* __init_or_exit */ musb_exit_debugfs(struct musb *musb)
    {
    debugfs_remove_recursive(musb.debugfs_root);
    }
