//! Automatically rewritten from C to Rust
//! Source: drivers/edac/zynqmp_edac.c
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
// Xilinx ZynqMP OCM ECC Driver
//
// Copyright (C) 2022 Advanced Micro Devices, Inc.
//

pub const ZYNQMP_OCM_EDAC_MSG_SIZE: c_int = 256;

// Error/Interrupt registers
pub const ERR_CTRL_OFST: c_uint = 0x0;
pub const OCM_ISR_OFST: c_uint = 0x04;
pub const OCM_IMR_OFST: c_uint = 0x08;
pub const OCM_IEN_OFST: c_uint = 0x0C;
pub const OCM_IDS_OFST: c_uint = 0x10;
// ECC control register
pub const ECC_CTRL_OFST: c_uint = 0x14;
// Correctable error info registers
pub const CE_FFA_OFST: c_uint = 0x1C;
pub const CE_FFD0_OFST: c_uint = 0x20;
pub const CE_FFD1_OFST: c_uint = 0x24;
pub const CE_FFD2_OFST: c_uint = 0x28;
pub const CE_FFD3_OFST: c_uint = 0x2C;
pub const CE_FFE_OFST: c_uint = 0x30;
// Uncorrectable error info registers
pub const UE_FFA_OFST: c_uint = 0x34;
pub const UE_FFD0_OFST: c_uint = 0x38;
pub const UE_FFD1_OFST: c_uint = 0x3C;
pub const UE_FFD2_OFST: c_uint = 0x40;
pub const UE_FFD3_OFST: c_uint = 0x44;
pub const UE_FFE_OFST: c_uint = 0x48;
// ECC control register bit field definitions
pub const ECC_CTRL_CLR_CE_ERR: c_uint = 0x40;
pub const ECC_CTRL_CLR_UE_ERR: c_uint = 0x80;
// Fault injection data and count registers
pub const OCM_FID0_OFST: c_uint = 0x4C;
pub const OCM_FID1_OFST: c_uint = 0x50;
pub const OCM_FID2_OFST: c_uint = 0x54;
pub const OCM_FID3_OFST: c_uint = 0x58;
pub const OCM_FIC_OFST: c_uint = 0x74;
pub const UE_MAX_BITPOS_LOWER: c_int = 31;
pub const UE_MIN_BITPOS_UPPER: c_int = 32;
pub const UE_MAX_BITPOS_UPPER: c_int = 63;
// Interrupt masks

pub const OCM_NUM_UE_BITPOS: c_int = 2;
pub const OCM_BASEVAL: c_uint = 0xFFFC0000;

//
// struct ecc_error_info - ECC error log information
// @addr:	Fault generated at this address
// @fault_lo:	Generated fault data (lower 32-bit)
// @fault_hi:	Generated fault data (upper 32-bit)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecc_error_info {
    pub addr: u32,
    pub fault_lo: u32,
    pub fault_hi: u32,
}

//
// struct ecc_status - ECC status information to report
// @ce_cnt:	Correctable error count
// @ue_cnt:	Uncorrectable error count
// @ceinfo:	Correctable error log information
// @ueinfo:	Uncorrectable error log information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecc_status {
    pub ce_cnt: u32,
    pub ue_cnt: u32,
    pub ceinfo: ecc_error_info,
    pub ueinfo: ecc_error_info,
}

//
// struct edac_priv - OCM private instance data
// @baseaddr:	Base address of the OCM
// @message:	Buffer for framing the event specific info
// @stat:	ECC status information
// @ce_cnt:	Correctable Error count
// @ue_cnt:	Uncorrectable Error count
// @debugfs_dir:	Directory entry for debugfs
// @ce_bitpos:	Bit position for Correctable Error
// @ue_bitpos:	Array to store UnCorrectable Error bit positions
// @fault_injection_cnt: Fault Injection Counter value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_priv {
    pub baseaddr: *mut void __iomem,
    pub message: [c_char; ZYNQMP_OCM_EDAC_MSG_SIZE],
    pub stat: ecc_status,
    pub ce_cnt: u32,
    pub ue_cnt: u32,

    pub debugfs_dir: *mut dentry,
    pub ce_bitpos: u8,
    pub ue_bitpos: [u8; OCM_NUM_UE_BITPOS],
    pub fault_injection_cnt: u32,

}

//
// get_error_info - Get the current ECC error info
// @base:	Pointer to the base address of the OCM
// @p:		Pointer to the OCM ECC status structure
// @mask:	Status register mask value
//
// Determines there is any ECC error or not
//
#[no_mangle]
unsafe extern "C" fn get_error_info(base: *mut void __iomem, p: *mut ecc_status, mask: c_int) {
    static void get_error_info(void __iomem *base, struct ecc_status *p, int mask)
    {
    if (mask & OCM_CEINTR_MASK) {
    p.ce_cnt++;
    p.ceinfo.fault_lo = readl(base + CE_FFD0_OFST);
    p.ceinfo.fault_hi = readl(base + CE_FFD1_OFST);
    p.ceinfo.addr = (OCM_BASEVAL | readl(base + CE_FFA_OFST));
    writel(ECC_CTRL_CLR_CE_ERR, base + OCM_ISR_OFST);
    } else if (mask & OCM_UEINTR_MASK) {
    p.ue_cnt++;
    p.ueinfo.fault_lo = readl(base + UE_FFD0_OFST);
    p.ueinfo.fault_hi = readl(base + UE_FFD1_OFST);
    p.ueinfo.addr = (OCM_BASEVAL | readl(base + UE_FFA_OFST));
    writel(ECC_CTRL_CLR_UE_ERR, base + OCM_ISR_OFST);
    }
    }
//
// handle_error - Handle error types CE and UE
// @dci:	Pointer to the EDAC device instance
// @p:		Pointer to the OCM ECC status structure
//
// Handles correctable and uncorrectable errors.
//
#[no_mangle]
unsafe extern "C" fn handle_error(dci: *mut edac_device_ctl_info, p: *mut ecc_status) {
    static void handle_error(struct edac_device_ctl_info *dci, struct ecc_status *p)
    {
    struct edac_priv *priv = dci.pvt_info;
    struct ecc_error_info *pinf;
    if (p.ce_cnt) {
    pinf = &p.ceinfo;
    snprintf(priv.message, ZYNQMP_OCM_EDAC_MSG_SIZE,
    "\nOCM ECC error type :%s\nAddr: [0x%x]\nFault Data[0x%08x%08x]",
    "CE", pinf.addr, pinf.fault_hi, pinf.fault_lo);
    edac_device_handle_ce(dci, 0, 0, priv.message);
    }
    if (p.ue_cnt) {
    pinf = &p.ueinfo;
    snprintf(priv.message, ZYNQMP_OCM_EDAC_MSG_SIZE,
    "\nOCM ECC error type :%s\nAddr: [0x%x]\nFault Data[0x%08x%08x]",
    "UE", pinf.addr, pinf.fault_hi, pinf.fault_lo);
    edac_device_handle_ue(dci, 0, 0, priv.message);
    }
    memset(p, 0, sizeof(*p));
    }
//
// intr_handler - ISR routine
// @irq:        irq number
// @dev_id:     device id pointer
//
// Return: IRQ_NONE, if CE/UE interrupt not set or IRQ_HANDLED otherwise
//
#[no_mangle]
unsafe extern "C" fn intr_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t intr_handler(int irq, void *dev_id)
    {
    struct edac_device_ctl_info *dci = dev_id;
    struct edac_priv *priv = dci.pvt_info;
    int regval;
    regval = readl(priv.baseaddr + OCM_ISR_OFST);
    if (!(regval & (OCM_CEINTR_MASK | OCM_UEINTR_MASK))) {
    WARN_ONCE(1, "Unhandled IRQ%d, ISR: 0x%x", irq, regval);
    return IRQ_NONE;
    }
    get_error_info(priv.baseaddr, &priv.stat, regval);
    priv.ce_cnt += priv.stat.ce_cnt;
    priv.ue_cnt += priv.stat.ue_cnt;
    handle_error(dci, &priv.stat);
    return IRQ_HANDLED;
    }
//
// get_eccstate - Return the ECC status
// @base:	Pointer to the OCM base address
//
// Get the ECC enable/disable status
//
// Return: ECC status 0/1.
//
#[no_mangle]
unsafe extern "C" fn get_eccstate(base: *mut void __iomem) -> bool {
    static bool get_eccstate(void __iomem *base)
    {
    return readl(base + ECC_CTRL_OFST) & OCM_ECC_ENABLE_MASK;
    }

//
// write_fault_count - write fault injection count
// @priv:	Pointer to the EDAC private struct
//
// Update the fault injection count register, once the counter reaches
// zero, it injects errors
//
#[no_mangle]
unsafe extern "C" fn write_fault_count(priv: *mut edac_priv) {
    static void write_fault_count(struct edac_priv *priv)
    {
    let mut ficount: u32 = priv.fault_injection_cnt;
    if (ficount & ~OCM_FICOUNT_MASK) {
    ficount &= OCM_FICOUNT_MASK;
    edac_printk(KERN_INFO, EDAC_DEVICE,
    "Fault injection count value truncated to %d\n", ficount);
    }
    writel(ficount, priv.baseaddr + OCM_FIC_OFST);
    }
//
// To get the Correctable Error injected, the following steps are needed:
// - Setup the optional Fault Injection Count:
// echo <fault_count val> > /sys/kernel/debug/edac/ocm/inject_fault_count
// - Write the Correctable Error bit position value:
// echo <bit_pos val> > /sys/kernel/debug/edac/ocm/inject_ce_bitpos
//
    static ssize_t inject_ce_write(struct file *file, const char __user *data,
    size_t count, loff_t *ppos)
    {
    struct edac_device_ctl_info *edac_dev = file.private_data;
    struct edac_priv *priv = edac_dev.pvt_info;
    int ret;
    if (!data)
    return -EFAULT;
    ret = kstrtou8_from_user(data, count, 0, &priv.ce_bitpos);
    if (ret)
    return ret;
    if (priv.ce_bitpos > UE_MAX_BITPOS_UPPER)
    return -EINVAL;
    if (priv.ce_bitpos <= UE_MAX_BITPOS_LOWER) {
    writel(BIT(priv.ce_bitpos), priv.baseaddr + OCM_FID0_OFST);
    writel(0, priv.baseaddr + OCM_FID1_OFST);
    } else {
    writel(BIT(priv.ce_bitpos - UE_MIN_BITPOS_UPPER),
    priv.baseaddr + OCM_FID1_OFST);
    writel(0, priv.baseaddr + OCM_FID0_OFST);
    }
    write_fault_count(priv);
    return count;
    }
    static const struct file_operations inject_ce_fops = {
    .open = simple_open,
    .write = inject_ce_write,
    .llseek = generic_file_llseek,
    };
//
// To get the Uncorrectable Error injected, the following steps are needed:
// - Setup the optional Fault Injection Count:
// echo <fault_count val> > /sys/kernel/debug/edac/ocm/inject_fault_count
// - Write the Uncorrectable Error bit position values:
// echo <bit_pos0 val>,<bit_pos1 val> > /sys/kernel/debug/edac/ocm/inject_ue_bitpos
//
    static ssize_t inject_ue_write(struct file *file, const char __user *data,
    size_t count, loff_t *ppos)
    {
    struct edac_device_ctl_info *edac_dev = file.private_data;
    struct edac_priv *priv = edac_dev.pvt_info;
    char buf[6], *pbuf, *token[2];
    u64 ue_bitpos;
    int i, ret;
    u8 len;
    if (!data)
    return -EFAULT;
    len = min_t(size_t, count, sizeof(buf));
    if (copy_from_user(buf, data, len))
    return -EFAULT;
    buf[len] = '\0';
    pbuf = &buf[0];
    for (i = 0; i < OCM_NUM_UE_BITPOS; i++)
    token[i] = strsep(&pbuf, ",");
    ret = kstrtou8(token[0], 0, &priv.ue_bitpos[0]);
    if (ret)
    return ret;
    ret = kstrtou8(token[1], 0, &priv.ue_bitpos[1]);
    if (ret)
    return ret;
    if (priv.ue_bitpos[0] > UE_MAX_BITPOS_UPPER ||
    priv.ue_bitpos[1] > UE_MAX_BITPOS_UPPER)
    return -EINVAL;
    if (priv.ue_bitpos[0] == priv.ue_bitpos[1]) {
    edac_printk(KERN_ERR, EDAC_DEVICE, "Bit positions should not be equal\n");
    return -EINVAL;
    }
    ue_bitpos = BIT(priv.ue_bitpos[0]) | BIT(priv.ue_bitpos[1]);
    writel((u32)ue_bitpos, priv.baseaddr + OCM_FID0_OFST);
    writel((u32)(ue_bitpos >> 32), priv.baseaddr + OCM_FID1_OFST);
    write_fault_count(priv);
    return count;
    }
    static const struct file_operations inject_ue_fops = {
    .open = simple_open,
    .write = inject_ue_write,
    .llseek = generic_file_llseek,
    };
#[no_mangle]
unsafe extern "C" fn setup_debugfs(edac_dev: *mut edac_device_ctl_info) {
    static void setup_debugfs(struct edac_device_ctl_info *edac_dev)
    {
    struct edac_priv *priv = edac_dev.pvt_info;
    priv.debugfs_dir = edac_debugfs_create_dir("ocm");
    if (!priv.debugfs_dir)
    return;
    edac_debugfs_create_x32("inject_fault_count", 0644, priv.debugfs_dir,
    &priv.fault_injection_cnt);
    edac_debugfs_create_file("inject_ue_bitpos", 0644, priv.debugfs_dir,
    edac_dev, &inject_ue_fops);
    edac_debugfs_create_file("inject_ce_bitpos", 0644, priv.debugfs_dir,
    edac_dev, &inject_ce_fops);
    }

#[no_mangle]
unsafe extern "C" fn edac_probe(pdev: *mut platform_device) -> c_int {
    static int edac_probe(struct platform_device *pdev)
    {
    struct edac_device_ctl_info *dci;
    struct edac_priv *priv;
    void __iomem *baseaddr;
    struct resource *res;
    int irq, ret;
    baseaddr = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(baseaddr))
    return PTR_ERR(baseaddr);
    if (!get_eccstate(baseaddr)) {
    edac_printk(KERN_INFO, EDAC_DEVICE, "ECC not enabled\n");
    return -ENXIO;
    }
    dci = edac_device_alloc_ctl_info(sizeof(*priv), ZYNQMP_OCM_EDAC_STRING,
    1, ZYNQMP_OCM_EDAC_STRING, 1, 0,
    edac_device_alloc_index());
    if (!dci)
    return -ENOMEM;
    priv = dci.pvt_info;
    platform_set_drvdata(pdev, dci);
    dci.dev = &pdev.dev;
    priv.baseaddr = baseaddr;
    dci.mod_name = pdev.dev.driver.name;
    dci.ctl_name = ZYNQMP_OCM_EDAC_STRING;
    dci.dev_name = dev_name(&pdev.dev);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    ret = irq;
    goto free_dev_ctl;
    }
    ret = devm_request_irq(&pdev.dev, irq, intr_handler, 0,
    dev_name(&pdev.dev), dci);
    if (ret) {
    edac_printk(KERN_ERR, EDAC_DEVICE, "Failed to request Irq\n");
    goto free_dev_ctl;
    }
// Enable UE, CE interrupts
    writel((OCM_CEINTR_MASK | OCM_UEINTR_MASK), priv.baseaddr + OCM_IEN_OFST);

    setup_debugfs(dci);

    ret = edac_device_add_device(dci);
    if (ret)
    goto free_dev_ctl;
    return 0;
    free_dev_ctl:
    edac_device_free_ctl_info(dci);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn edac_remove(pdev: *mut platform_device) {
    static void edac_remove(struct platform_device *pdev)
    {
    struct edac_device_ctl_info *dci = platform_get_drvdata(pdev);
    struct edac_priv *priv = dci.pvt_info;
// Disable UE, CE interrupts
    writel((OCM_CEINTR_MASK | OCM_UEINTR_MASK), priv.baseaddr + OCM_IDS_OFST);

    debugfs_remove_recursive(priv.debugfs_dir);

    edac_device_del_device(&pdev.dev);
    edac_device_free_ctl_info(dci);
    }
    static const struct of_device_id zynqmp_ocm_edac_match[] = {
    { .compatible = "xlnx,zynqmp-ocmc-1.0"},
    { /* end of table */ }
    };
    MODULE_DEVICE_TABLE(of, zynqmp_ocm_edac_match);
    static struct platform_driver zynqmp_ocm_edac_driver = {
    .driver = {
    .name = "zynqmp-ocm-edac",
    .of_match_table = zynqmp_ocm_edac_match,
    },
    .probe = edac_probe,
    .remove = edac_remove,
    };
    module_platform_driver(zynqmp_ocm_edac_driver);
    MODULE_AUTHOR("Advanced Micro Devices, Inc");
    MODULE_DESCRIPTION("Xilinx ZynqMP OCM ECC driver");
    MODULE_LICENSE("GPL");
