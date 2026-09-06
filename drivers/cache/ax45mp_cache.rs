//! Automatically rewritten from C to Rust
//! Source: drivers/cache/ax45mp_cache.c
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
// non-coherent cache functions for Andes AX45MP
//
// Copyright (C) 2023 Renesas Electronics Corp.
//

// L2 cache registers
pub const AX45MP_L2C_REG_CTL_OFFSET: c_uint = 0x8;
pub const AX45MP_L2C_REG_C0_CMD_OFFSET: c_uint = 0x40;
pub const AX45MP_L2C_REG_C0_ACC_OFFSET: c_uint = 0x48;
pub const AX45MP_L2C_REG_STATUS_OFFSET: c_uint = 0x80;
// D-cache operation

// L2 CCTL status
pub const AX45MP_CCTL_L2_STATUS_IDLE: c_int = 0;
// L2 CCTL status cores mask
pub const AX45MP_CCTL_L2_STATUS_C0_MASK: c_uint = 0xf;
// L2 cache operation
pub const AX45MP_CCTL_L2_PA_INVAL: c_uint = 0x8 /* Invalidate an L2 cache entry */;
pub const AX45MP_CCTL_L2_PA_WB: c_uint = 0x9 /* Write-back an L2 cache entry */;
pub const AX45MP_L2C_REG_PER_CORE_OFFSET: c_uint = 0x10;
pub const AX45MP_CCTL_L2_STATUS_PER_CORE_OFFSET: c_int = 4;

    (AX45MP_L2C_REG_C0_CMD_OFFSET + ((n) * AX45MP_L2C_REG_PER_CORE_OFFSET))

    (AX45MP_L2C_REG_C0_ACC_OFFSET + ((n) * AX45MP_L2C_REG_PER_CORE_OFFSET))

    (AX45MP_CCTL_L2_STATUS_C0_MASK << ((n) * AX45MP_CCTL_L2_STATUS_PER_CORE_OFFSET))
pub const AX45MP_CCTL_REG_UCCTLBEGINADDR_NUM: c_uint = 0x80b;
pub const AX45MP_CCTL_REG_UCCTLCOMMAND_NUM: c_uint = 0x80c;
pub const AX45MP_CACHE_LINE_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax45mp_priv {
    pub l2c_base: *mut void __iomem,
    pub ax45mp_cache_line_size: u32,
}

    static struct ax45mp_priv ax45mp_priv;
// L2 Cache operations
#[no_mangle]
pub unsafe extern "C" fn ax45mp_cpu_l2c_get_cctl_status() -> u32 {
    static inline uint32_t ax45mp_cpu_l2c_get_cctl_status(void)
    {
    return readl(ax45mp_priv.l2c_base + AX45MP_L2C_REG_STATUS_OFFSET);
    }
    static void ax45mp_cpu_cache_operation(unsigned long start, unsigned long end,
    unsigned int l1_op, unsigned int l2_op)
    {
    let mut line_size: c_ulong = ax45mp_priv.ax45mp_cache_line_size;
    void __iomem *base = ax45mp_priv.l2c_base;
    let mut mhartid: c_int = smp_processor_id();
    unsigned long pa;
    while (end > start) {
    csr_write(AX45MP_CCTL_REG_UCCTLBEGINADDR_NUM, start);
    csr_write(AX45MP_CCTL_REG_UCCTLCOMMAND_NUM, l1_op);
    pa = virt_to_phys((void *)start);
    writel(pa, base + AX45MP_L2C_REG_CN_ACC_OFFSET(mhartid));
    writel(l2_op, base + AX45MP_L2C_REG_CN_CMD_OFFSET(mhartid));
    while ((ax45mp_cpu_l2c_get_cctl_status() &
    AX45MP_CCTL_L2_STATUS_CN_MASK(mhartid)) !=
    AX45MP_CCTL_L2_STATUS_IDLE)
    ;
    start += line_size;
    }
    }
// Write-back L1 and L2 cache entry
#[no_mangle]
pub unsafe extern "C" fn ax45mp_cpu_dcache_wb_range(start: c_ulong, end: c_ulong) {
    static inline void ax45mp_cpu_dcache_wb_range(unsigned long start, unsigned long end)
    {
    ax45mp_cpu_cache_operation(start, end, AX45MP_CCTL_L1D_VA_WB,
    AX45MP_CCTL_L2_PA_WB);
    }
// Invalidate the L1 and L2 cache entry
#[no_mangle]
pub unsafe extern "C" fn ax45mp_cpu_dcache_inval_range(start: c_ulong, end: c_ulong) {
    static inline void ax45mp_cpu_dcache_inval_range(unsigned long start, unsigned long end)
    {
    ax45mp_cpu_cache_operation(start, end, AX45MP_CCTL_L1D_VA_INVAL,
    AX45MP_CCTL_L2_PA_INVAL);
    }
#[no_mangle]
unsafe extern "C" fn ax45mp_dma_cache_inv(paddr: phys_addr_t, size: usize) {
    static void ax45mp_dma_cache_inv(phys_addr_t paddr, size_t size)
    {
    let mut start: c_ulong = (unsigned long)phys_to_virt(paddr);
    let mut end: c_ulong = start + size;
    unsigned long line_size;
    unsigned long flags;
    if (unlikely(start == end))
    return;
    line_size = ax45mp_priv.ax45mp_cache_line_size;
    start = start & (~(line_size - 1));
    end = ((end + line_size - 1) & (~(line_size - 1)));
    local_irq_save(flags);
    ax45mp_cpu_dcache_inval_range(start, end);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn ax45mp_dma_cache_wback(paddr: phys_addr_t, size: usize) {
    static void ax45mp_dma_cache_wback(phys_addr_t paddr, size_t size)
    {
    let mut start: c_ulong = (unsigned long)phys_to_virt(paddr);
    let mut end: c_ulong = start + size;
    unsigned long line_size;
    unsigned long flags;
    if (unlikely(start == end))
    return;
    line_size = ax45mp_priv.ax45mp_cache_line_size;
    start = start & (~(line_size - 1));
    end = ((end + line_size - 1) & (~(line_size - 1)));
    local_irq_save(flags);
    ax45mp_cpu_dcache_wb_range(start, end);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn ax45mp_dma_cache_wback_inv(paddr: phys_addr_t, size: usize) {
    static void ax45mp_dma_cache_wback_inv(phys_addr_t paddr, size_t size)
    {
    ax45mp_dma_cache_wback(paddr, size);
    ax45mp_dma_cache_inv(paddr, size);
    }
#[no_mangle]
unsafe extern "C" fn ax45mp_get_l2_line_size(np: *mut device_node) -> c_int {
    static int ax45mp_get_l2_line_size(struct device_node *np)
    {
    int ret;
    ret = of_property_read_u32(np, "cache-line-size", &ax45mp_priv.ax45mp_cache_line_size);
    if (ret) {
    pr_err("Failed to get cache-line-size, defaulting to 64 bytes\n");
    return ret;
    }
    if (ax45mp_priv.ax45mp_cache_line_size != AX45MP_CACHE_LINE_SIZE) {
    pr_err("Expected cache-line-size to be 64 bytes (found:%u)\n",
    ax45mp_priv.ax45mp_cache_line_size);
    return -EINVAL;
    }
    return 0;
    }
    static const struct riscv_nonstd_cache_ops ax45mp_cmo_ops __initdata = {
    .wback = &ax45mp_dma_cache_wback,
    .inv = &ax45mp_dma_cache_inv,
    .wback_inv = &ax45mp_dma_cache_wback_inv,
    };
    static const struct of_device_id ax45mp_cache_ids[] = {
    { .compatible = "andestech,ax45mp-cache" },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn ax45mp_cache_init() -> int __init {
    static int __init ax45mp_cache_init(void)
    {
    struct resource res;
    int ret;
    struct device_node *np __free(device_node) =
    of_find_matching_node(core::ptr::null_mut(), ax45mp_cache_ids);
    if (!of_device_is_available(np))
    return -ENODEV;
    ret = of_address_to_resource(np, 0, &res);
    if (ret)
    return ret;
//
// If IOCP is present on the Andes AX45MP core riscv_cbom_block_size
// will be 0 for sure, so we can definitely rely on it. If
// riscv_cbom_block_size = 0 we don't need to handle CMO using SW any
// more so we just return success here and only if its being set we
// continue further in the probe path.
//
    if (!riscv_cbom_block_size)
    return 0;
    ax45mp_priv.l2c_base = ioremap(res.start, resource_size(&res));
    if (!ax45mp_priv.l2c_base)
    return -ENOMEM;
    ret = ax45mp_get_l2_line_size(np);
    if (ret) {
    iounmap(ax45mp_priv.l2c_base);
    return ret;
    }
    riscv_noncoherent_register_cache_ops(&ax45mp_cmo_ops);
    return 0;
    }
    early_initcall(ax45mp_cache_init);
