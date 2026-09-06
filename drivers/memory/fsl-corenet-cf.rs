//! Automatically rewritten from C to Rust
//! Source: drivers/memory/fsl-corenet-cf.c
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
// CoreNet Coherency Fabric error reporting
//
// Copyright 2014 Freescale Semiconductor Inc.
//

    enum ccf_version {
    CCF1,
    CCF2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccf_info {
    pub version: enum ccf_version,
    pub err_reg_offs: c_int,
    pub has_brr: bool,
}

    static const struct ccf_info ccf1_info = {
    .version = CCF1,
    .err_reg_offs = 0xa00,
    .has_brr = false,
    };
    static const struct ccf_info ccf2_info = {
    .version = CCF2,
    .err_reg_offs = 0xe40,
    .has_brr = true,
    };
//
// This register is present but not documented, with different values for
// IP_ID, on other chips with fsl,corenet2-cf such as t4240 and b4860.
//
pub const CCF_BRR: c_uint = 0xbf8;
pub const CCF_BRR_IPID: c_uint = 0xffff0000;
pub const CCF_BRR_IPID_T1040: c_uint = 0x09310000;
    static const struct of_device_id ccf_matches[] = {
    {
    .compatible = "fsl,corenet1-cf",
    .data = &ccf1_info,
    },
    {
    .compatible = "fsl,corenet2-cf",
    .data = &ccf2_info,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, ccf_matches);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccf_err_regs {
    pub /: *mut *mut u32 errdet; / 0x00 Error Detect Register,
// 0x04 Error Enable (ccf1)/Disable (ccf2) Register
    pub errdis: u32,
// 0x08 Error Interrupt Enable Register (ccf2 only)
    pub errinten: u32,
    pub /: *mut *mut u32 cecar; / 0x0c Error Capture Attribute Register,
    pub /: *mut *mut u32 cecaddrh; / 0x10 Error Capture Address High,
    pub /: *mut *mut u32 cecaddrl; / 0x14 Error Capture Address Low,
    pub /: *mut *mut u32 cecar2; / 0x18 Error Capture Attribute Register 2,
}

// LAE/CV also valid for errdis and errinten

pub const CECAR_SRCID_SHIFT_CCF1: c_int = 24;

pub const CECAR_SRCID_SHIFT_CCF2: c_int = 18;

pub const CECADDRH_ADDRH: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccf_private {
    pub info: *const ccf_info,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub err_regs: *mut ccf_err_regs __iomem,
    pub t1040: bool,
}

#[no_mangle]
unsafe extern "C" fn ccf_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ccf_irq(int irq, void *dev_id)
    {
    struct ccf_private *ccf = dev_id;
    static DEFINE_RATELIMIT_STATE(ratelimit, DEFAULT_RATELIMIT_INTERVAL,
    DEFAULT_RATELIMIT_BURST);
    u32 errdet, cecar, cecar2;
    u64 addr;
    u32 src_id;
    let mut uvt: bool = false;
    let mut cap_valid: bool = false;
    errdet = ioread32be(&ccf.err_regs.errdet);
    cecar = ioread32be(&ccf.err_regs.cecar);
    cecar2 = ioread32be(&ccf.err_regs.cecar2);
    addr = ioread32be(&ccf.err_regs.cecaddrl);
    addr |= ((u64)(ioread32be(&ccf.err_regs.cecaddrh) &
    CECADDRH_ADDRH)) << 32;
    if (!__ratelimit(&ratelimit))
    goto out;
    switch (ccf.info.version) {
    case CCF1:
    if (cecar & CECAR_VAL) {
    if (cecar & CECAR_UVT)
    uvt = true;
    src_id = (cecar & CECAR_SRCID_MASK_CCF1) >>
    CECAR_SRCID_SHIFT_CCF1;
    cap_valid = true;
    }
    break;
    case CCF2:
    if (errdet & ERRDET_CAP) {
    src_id = (cecar & CECAR_SRCID_MASK_CCF2) >>
    CECAR_SRCID_SHIFT_CCF2;
    cap_valid = true;
    }
    break;
    }
    dev_crit(ccf.dev, "errdet 0x%08x cecar 0x%08x cecar2 0x%08x\n",
    errdet, cecar, cecar2);
    if (errdet & ERRDET_LAE) {
    if (uvt)
    dev_crit(ccf.dev, "LAW Unavailable Target ID\n");
    else
    dev_crit(ccf.dev, "Local Access Window Error\n");
    }
    if (errdet & ERRDET_CV)
    dev_crit(ccf.dev, "Coherency Violation\n");
    if (errdet & ERRDET_UTID)
    dev_crit(ccf.dev, "Unavailable Target ID\n");
    if (errdet & ERRDET_MCST)
    dev_crit(ccf.dev, "Multicast Stash\n");
    if (cap_valid) {
    dev_crit(ccf.dev, "address 0x%09llx, src id 0x%x\n",
    addr, src_id);
    }
    out:
    iowrite32be(errdet, &ccf.err_regs.errdet);
    return errdet ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn ccf_probe(pdev: *mut platform_device) -> c_int {
    static int ccf_probe(struct platform_device *pdev)
    {
    struct ccf_private *ccf;
    u32 errinten;
    int ret, irq;
    ccf = devm_kzalloc(&pdev.dev, sizeof(*ccf), GFP_KERNEL);
    if (!ccf)
    return -ENOMEM;
    ccf.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ccf.regs))
    return PTR_ERR(ccf.regs);
    ccf.dev = &pdev.dev;
    ccf.info = device_get_match_data(&pdev.dev);
    ccf.err_regs = ccf.regs + ccf.info.err_reg_offs;
    if (ccf.info.has_brr) {
    let mut brr: u32 = ioread32be(ccf.regs + CCF_BRR);
    if ((brr & CCF_BRR_IPID) == CCF_BRR_IPID_T1040)
    ccf.t1040 = true;
    }
    dev_set_drvdata(&pdev.dev, ccf);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, ccf_irq, 0, pdev.name, ccf);
    if (ret) {
    dev_err(&pdev.dev, "%s: can't request irq\n", __func__);
    return ret;
    }
    errinten = ERRDET_LAE | ERRDET_CV;
    if (ccf.t1040)
    errinten |= ERRDET_UTID | ERRDET_MCST;
    switch (ccf.info.version) {
    case CCF1:
// On CCF1 this register enables rather than disables.
    iowrite32be(errinten, &ccf.err_regs.errdis);
    break;
    case CCF2:
    iowrite32be(0, &ccf.err_regs.errdis);
    iowrite32be(errinten, &ccf.err_regs.errinten);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ccf_remove(pdev: *mut platform_device) {
    static void ccf_remove(struct platform_device *pdev)
    {
    struct ccf_private *ccf = dev_get_drvdata(&pdev.dev);
    switch (ccf.info.version) {
    case CCF1:
    iowrite32be(0, &ccf.err_regs.errdis);
    break;
    case CCF2:
//
// We clear errdis on ccf1 because that's the only way to
// disable interrupts, but on ccf2 there's no need to disable
// detection.
//
    iowrite32be(0, &ccf.err_regs.errinten);
    break;
    }
    }
    static struct platform_driver ccf_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = ccf_matches,
    },
    .probe = ccf_probe,
    .remove = ccf_remove,
    };
    module_platform_driver(ccf_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Freescale Semiconductor");
    MODULE_DESCRIPTION("Freescale CoreNet Coherency Fabric error reporting");
