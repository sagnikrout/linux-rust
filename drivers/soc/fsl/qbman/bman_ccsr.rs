//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/qbman/bman_ccsr.c
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


// Copyright (c) 2009 - 2016 Freescale Semiconductor, Inc.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

    u16 bman_ip_rev;
    EXPORT_SYMBOL(bman_ip_rev);
// Register offsets
pub const REG_FBPR_FPC: c_uint = 0x0800;
pub const REG_ECSR: c_uint = 0x0a00;
pub const REG_ECIR: c_uint = 0x0a04;
pub const REG_EADR: c_uint = 0x0a08;

pub const REG_IP_REV_1: c_uint = 0x0bf8;
pub const REG_IP_REV_2: c_uint = 0x0bfc;
pub const REG_FBPR_BARE: c_uint = 0x0c00;
pub const REG_FBPR_BAR: c_uint = 0x0c04;
pub const REG_FBPR_AR: c_uint = 0x0c10;
pub const REG_SRCIDR: c_uint = 0x0d04;
pub const REG_LIODNR: c_uint = 0x0d08;
pub const REG_ERR_ISR: c_uint = 0x0e00;
pub const REG_ERR_IER: c_uint = 0x0e04;
pub const REG_ERR_ISDR: c_uint = 0x0e08;
// Used by all error interrupt registers except 'inhibit'
pub const BM_EIRQ_IVCI: c_uint = 0x00000010	/* Invalid Command Verb */;
pub const BM_EIRQ_FLWI: c_uint = 0x00000008	/* FBPR Low Watermark */;
pub const BM_EIRQ_MBEI: c_uint = 0x00000004	/* Multi-bit ECC Error */;
pub const BM_EIRQ_SBEI: c_uint = 0x00000002	/* Single-bit ECC Error */;
pub const BM_EIRQ_BSCN: c_uint = 0x00000001	/* pool State Change Notification */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bman_hwerr_txt {
    pub mask: u32,
    pub txt: *const c_char,
}

    static const struct bman_hwerr_txt bman_hwerr_txts[] = {
    { BM_EIRQ_IVCI, "Invalid Command Verb" },
    { BM_EIRQ_FLWI, "FBPR Low Watermark" },
    { BM_EIRQ_MBEI, "Multi-bit ECC Error" },
    { BM_EIRQ_SBEI, "Single-bit ECC Error" },
    { BM_EIRQ_BSCN, "Pool State Change Notification" },
    };
// Only trigger low water mark interrupt once only

// Pointer to the start of the BMan's CCSR space
    static u32 __iomem *bm_ccsr_start;
#[no_mangle]
pub unsafe extern "C" fn bm_ccsr_in(offset: u32) -> u32 {
    static inline u32 bm_ccsr_in(u32 offset)
    {
    return ioread32be(bm_ccsr_start + offset/4);
    }
#[no_mangle]
pub unsafe extern "C" fn bm_ccsr_out(offset: u32, val: u32) {
    static inline void bm_ccsr_out(u32 offset, u32 val)
    {
    iowrite32be(val, bm_ccsr_start + offset/4);
    }
#[no_mangle]
unsafe extern "C" fn bm_get_version(id: *mut u16, major: *mut u8, minor: *mut u8) {
    static void bm_get_version(u16 *id, u8 *major, u8 *minor)
    {
    let mut v: u32 = bm_ccsr_in(REG_IP_REV_1);
// id = (v >> 16);
// major = (v >> 8) & 0xff;
// minor = v & 0xff;
    }
// signal transactions for FBPRs with higher priority

// Track if probe has occurred and if cleanup is required
    static int __bman_probed;
    static int __bman_requires_cleanup;
#[no_mangle]
unsafe extern "C" fn bm_set_memory(ba: u64, size: u32) -> c_int {
    static int bm_set_memory(u64 ba, u32 size)
    {
    u32 bar, bare;
    let mut exp: u32 = ilog2(size);
// choke if size isn't within range
    DPAA_ASSERT(size >= 4096 && size <= 1024*1024*1024 &&
    is_power_of_2(size));
// choke if '[e]ba' has lower-alignment than 'size'
    DPAA_ASSERT(!(ba & (size - 1)));
// Check to see if BMan has already been initialized
    bar = bm_ccsr_in(REG_FBPR_BAR);
    if (bar) {
// Maker sure ba == what was programmed)
    bare = bm_ccsr_in(REG_FBPR_BARE);
    if (bare != upper_32_bits(ba) || bar != lower_32_bits(ba)) {
    pr_err("Attempted to reinitialize BMan with different BAR, got 0x%llx read BARE=0x%x BAR=0x%x\n",
    ba, bare, bar);
    return -ENOMEM;
    }
    pr_info("BMan BAR already configured\n");
    __bman_requires_cleanup = 1;
    return 1;
    }
    bm_ccsr_out(REG_FBPR_BARE, upper_32_bits(ba));
    bm_ccsr_out(REG_FBPR_BAR, lower_32_bits(ba));
    bm_ccsr_out(REG_FBPR_AR, exp - 1);
    return 0;
    }
//
// Location and size of BMan private memory
//
// Ideally we would use the DMA API to turn rmem->base into a DMA address
// (especially if iommu translations ever get involved).  Unfortunately, the
// DMA API currently does not allow mapping anything that is not backed with
// a struct page.
//
    static dma_addr_t fbpr_a;
    static size_t fbpr_sz;
#[no_mangle]
unsafe extern "C" fn bman_isr(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t bman_isr(int irq, void *ptr)
    {
    u32 isr_val, ier_val, ecsr_val, isr_mask, i;
    struct device *dev = ptr;
    ier_val = bm_ccsr_in(REG_ERR_IER);
    isr_val = bm_ccsr_in(REG_ERR_ISR);
    ecsr_val = bm_ccsr_in(REG_ECSR);
    isr_mask = isr_val & ier_val;
    if (!isr_mask)
    return IRQ_NONE;
    for (i = 0; i < ARRAY_SIZE(bman_hwerr_txts); i++) {
    if (bman_hwerr_txts[i].mask & isr_mask) {
    dev_err_ratelimited(dev, "ErrInt: %s\n",
    bman_hwerr_txts[i].txt);
    if (bman_hwerr_txts[i].mask & ecsr_val) {
// Re-arm error capture registers
    bm_ccsr_out(REG_ECSR, ecsr_val);
    }
    if (bman_hwerr_txts[i].mask & BMAN_ERRS_TO_DISABLE) {
    dev_dbg(dev, "Disabling error 0x%x\n",
    bman_hwerr_txts[i].mask);
    ier_val &= ~bman_hwerr_txts[i].mask;
    bm_ccsr_out(REG_ERR_IER, ier_val);
    }
    }
    }
    bm_ccsr_out(REG_ERR_ISR, isr_val);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn bman_is_probed() -> c_int {
    int bman_is_probed(void)
    {
    return __bman_probed;
    }
    EXPORT_SYMBOL_GPL(bman_is_probed);
#[no_mangle]
pub unsafe extern "C" fn bman_requires_cleanup() -> c_int {
    int bman_requires_cleanup(void)
    {
    return __bman_requires_cleanup;
    }
#[no_mangle]
pub unsafe extern "C" fn bman_done_cleanup() {
    void bman_done_cleanup(void)
    {
    __bman_requires_cleanup = 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_bman_probe(pdev: *mut platform_device) -> c_int {
    static int fsl_bman_probe(struct platform_device *pdev)
    {
    int ret, err_irq;
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct resource *res;
    u16 id, bm_pool_cnt;
    u8 major, minor;
    __bman_probed = -1;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(dev, "Can't get %pOF property 'IORESOURCE_MEM'\n",
    node);
    return -ENXIO;
    }
    bm_ccsr_start = devm_ioremap(dev, res.start, resource_size(res));
    if (!bm_ccsr_start)
    return -ENXIO;
    bm_get_version(&id, &major, &minor);
    if (major == 1 && minor == 0) {
    bman_ip_rev = BMAN_REV10;
    bm_pool_cnt = BM_POOL_MAX;
    } else if (major == 2 && minor == 0) {
    bman_ip_rev = BMAN_REV20;
    bm_pool_cnt = 8;
    } else if (major == 2 && minor == 1) {
    bman_ip_rev = BMAN_REV21;
    bm_pool_cnt = BM_POOL_MAX;
    } else {
    dev_err(dev, "Unknown Bman version:%04x,%02x,%02x\n",
    id, major, minor);
    return -ENODEV;
    }
    ret = qbman_init_private_mem(dev, 0, "fsl,bman-fbpr", &fbpr_a, &fbpr_sz);
    if (ret) {
    dev_err(dev, "qbman_init_private_mem() failed 0x%x\n",
    ret);
    return -ENODEV;
    }
    dev_dbg(dev, "Allocated FBPR 0x%llx 0x%zx\n", fbpr_a, fbpr_sz);
    bm_set_memory(fbpr_a, fbpr_sz);
    err_irq = platform_get_irq(pdev, 0);
    if (err_irq <= 0) {
    dev_info(dev, "Can't get %pOF IRQ\n", node);
    return -ENODEV;
    }
    ret = devm_request_irq(dev, err_irq, bman_isr, IRQF_SHARED, "bman-err",
    dev);
    if (ret)  {
    dev_err(dev, "devm_request_irq() failed %d for '%pOF'\n",
    ret, node);
    return ret;
    }
// Disable Buffer Pool State Change
    bm_ccsr_out(REG_ERR_ISDR, BM_EIRQ_BSCN);
//
// Write-to-clear any stale bits, (eg. starvation being asserted prior
// to resource allocation during driver init).
//
    bm_ccsr_out(REG_ERR_ISR, 0xffffffff);
// Enable Error Interrupts
    bm_ccsr_out(REG_ERR_IER, 0xffffffff);
    bm_bpalloc = devm_gen_pool_create(dev, 0, -1, "bman-bpalloc");
    if (IS_ERR(bm_bpalloc)) {
    ret = PTR_ERR(bm_bpalloc);
    dev_err(dev, "bman-bpalloc pool init failed (%d)\n", ret);
    return ret;
    }
// seed BMan resource pool
    ret = gen_pool_add(bm_bpalloc, DPAA_GENALLOC_OFF, bm_pool_cnt, -1);
    if (ret) {
    dev_err(dev, "Failed to seed BPID range [%d..%d] (%d)\n",
    0, bm_pool_cnt - 1, ret);
    return ret;
    }
    __bman_probed = 1;
    return 0;
    };
    static const struct of_device_id fsl_bman_ids[] = {
    {
    .compatible = "fsl,bman",
    },
    {}
    };
    static struct platform_driver fsl_bman_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = fsl_bman_ids,
    .suppress_bind_attrs = true,
    },
    .probe = fsl_bman_probe,
    };
    builtin_platform_driver(fsl_bman_driver);
