//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/xilinx-trng.c
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
// AMD Versal True Random Number Generator driver
// Copyright (c) 2024 - 2025 Advanced Micro Devices, Inc.
//

// TRNG Registers Offsets
pub const TRNG_STATUS_OFFSET: c_uint = 0x4U;
pub const TRNG_CTRL_OFFSET: c_uint = 0x8U;
pub const TRNG_EXT_SEED_OFFSET: c_uint = 0x40U;
pub const TRNG_PER_STRNG_OFFSET: c_uint = 0x80U;
pub const TRNG_CORE_OUTPUT_OFFSET: c_uint = 0xC0U;
pub const TRNG_RESET_OFFSET: c_uint = 0xD0U;
pub const TRNG_OSC_EN_OFFSET: c_uint = 0xD4U;
// Mask values

pub const TRNG_STATUS_QCNT_16_BYTES: c_uint = 0x800;
// Sizes in bytes

pub const TRNG_RESET_DELAY: c_int = 10;

pub const TRNG_READ_4_WORD: c_int = 4;
pub const TRNG_DATA_READ_DELAY: c_int = 8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_rng {
    pub rng_base: *mut void __iomem,
    pub dev: *mut device,
    pub trng: hwrng,
}

#[no_mangle]
unsafe extern "C" fn xtrng_readwrite32(addr: *mut void __iomem, mask: u32, value: u8) {
    static void xtrng_readwrite32(void __iomem *addr, u32 mask, u8 value)
    {
    u32 val;
    val = ioread32(addr);
    val = (val & (~mask)) | (mask & value);
    iowrite32(val, addr);
    }
#[no_mangle]
unsafe extern "C" fn xtrng_trng_reset(addr: *mut void __iomem) {
    static void xtrng_trng_reset(void __iomem *addr)
    {
    xtrng_readwrite32(addr + TRNG_RESET_OFFSET, TRNG_RESET_VAL_MASK, TRNG_RESET_VAL_MASK);
    udelay(TRNG_RESET_DELAY);
    xtrng_readwrite32(addr + TRNG_RESET_OFFSET, TRNG_RESET_VAL_MASK, 0);
    }
#[no_mangle]
unsafe extern "C" fn xtrng_hold_reset(addr: *mut void __iomem) {
    static void xtrng_hold_reset(void __iomem *addr)
    {
    xtrng_readwrite32(addr + TRNG_CTRL_OFFSET, TRNG_CTRL_PRNGSRST_MASK,
    TRNG_CTRL_PRNGSRST_MASK);
    iowrite32(TRNG_RESET_VAL_MASK, addr + TRNG_RESET_OFFSET);
    udelay(TRNG_RESET_DELAY);
    }
#[no_mangle]
unsafe extern "C" fn xtrng_softreset(rng: *mut xilinx_rng) {
    static void xtrng_softreset(struct xilinx_rng *rng)
    {
    xtrng_readwrite32(rng.rng_base + TRNG_CTRL_OFFSET, TRNG_CTRL_PRNGSRST_MASK,
    TRNG_CTRL_PRNGSRST_MASK);
    udelay(TRNG_RESET_DELAY);
    xtrng_readwrite32(rng.rng_base + TRNG_CTRL_OFFSET, TRNG_CTRL_PRNGSRST_MASK, 0);
    }
// Return no. of bytes read or a negative error before any data is read.
#[no_mangle]
unsafe extern "C" fn xtrng_readblock32(rng_base: *mut void __iomem, buf: *mut __be32, blocks32: c_int, wait: bool) -> c_int {
    static int xtrng_readblock32(void __iomem *rng_base, __be32 *buf, int blocks32, bool wait)
    {
    let mut read: c_int = 0, ret;
    let mut timeout: c_int = 1;
    int i, idx;
    u32 val;
    if (wait)
    timeout = TRNG_DATA_READ_DELAY;
    for (i = 0; i < (blocks32 * 2); i++) {
// TRNG core generate data in 16 bytes. Read twice to complete 32 bytes read
    ret = readl_poll_timeout(rng_base + TRNG_STATUS_OFFSET, val,
    (val & TRNG_STATUS_QCNT_MASK) ==
    TRNG_STATUS_QCNT_16_BYTES, !!wait, timeout);
    if (ret) {
    if (!read)
    return ret;
    break;
    }
    for (idx = 0; idx < TRNG_READ_4_WORD; idx++) {
// (buf + read) = cpu_to_be32(ioread32(rng_base + TRNG_CORE_OUTPUT_OFFSET));
    read += 1;
    }
    }
    return read * 4;
    }
    static int xtrng_collect_random_data(struct xilinx_rng *rng, u8 *rand_gen_buf,
    int no_of_random_bytes, bool wait)
    {
    u8 randbuf[TRNG_SEC_STRENGTH_BYTES];
    int byteleft, blocks, count = 0;
    int full_blocks_bytes;
    int ret;
    byteleft = no_of_random_bytes & (TRNG_SEC_STRENGTH_BYTES - 1);
    blocks = no_of_random_bytes >> TRNG_SEC_STRENGTH_SHIFT;
    full_blocks_bytes = blocks * TRNG_SEC_STRENGTH_BYTES;
    xtrng_readwrite32(rng.rng_base + TRNG_CTRL_OFFSET, TRNG_CTRL_PRNGSTART_MASK,
    TRNG_CTRL_PRNGSTART_MASK);
    if (blocks) {
    ret = xtrng_readblock32(rng.rng_base, (__be32 *)rand_gen_buf, blocks, wait);
    if (ret <= 0) {
    count = ret;
    goto out_stop;
    }
    count += ret;
    if (ret < full_blocks_bytes)
    goto out_stop;
    }
    if (byteleft) {
    ret = xtrng_readblock32(rng.rng_base, (__be32 *)randbuf, 1, wait);
    if (ret < 0) {
    if (!count)
    count = ret;
    goto out_stop;
    }
    if (!ret)
    goto out_stop;
    ret = min(ret, no_of_random_bytes - count);
    memcpy(rand_gen_buf + count, randbuf, ret);
    count += ret;
    }
    out_stop:
    xtrng_readwrite32(rng.rng_base + TRNG_CTRL_OFFSET,
    TRNG_CTRL_PRNGMODE_MASK | TRNG_CTRL_PRNGSTART_MASK, 0U);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn xtrng_write_multiple_registers(base_addr: *mut void __iomem, values: *mut u32, n: usize) {
    static void xtrng_write_multiple_registers(void __iomem *base_addr, u32 *values, size_t n)
    {
    void __iomem *reg_addr;
    size_t i;
// Write seed value into EXTERNAL_SEED Registers in big endian format
    for (i = 0; i < n; i++) {
    reg_addr = (base_addr + ((n - 1 - i) * TRNG_BYTES_PER_REG));
    iowrite32((u32 )(cpu_to_be32(values[i])), reg_addr);
    }
    }
#[no_mangle]
unsafe extern "C" fn xtrng_enable_entropy(rng: *mut xilinx_rng) {
    static void xtrng_enable_entropy(struct xilinx_rng *rng)
    {
    iowrite32(TRNG_OSC_EN_VAL_MASK, rng.rng_base + TRNG_OSC_EN_OFFSET);
    xtrng_softreset(rng);
    iowrite32(TRNG_CTRL_EUMODE_MASK | TRNG_CTRL_TRSSEN_MASK, rng.rng_base + TRNG_CTRL_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn xtrng_reseed_internal(rng: *mut xilinx_rng) -> c_int {
    static int xtrng_reseed_internal(struct xilinx_rng *rng)
    {
    static const u8 default_salt[SHA512_DIGEST_SIZE];
    u8 entropy[SHA512_DIGEST_SIZE] __aligned(4);
    u32 val;
    int ret;
    xtrng_enable_entropy(rng);
// Collect some output from the TRNG.
    static_assert(sizeof(entropy) >= TRNG_SEED_LEN_BYTES);
    ret = xtrng_collect_random_data(rng, entropy, TRNG_SEED_LEN_BYTES, true);
    if (ret != TRNG_SEED_LEN_BYTES)
    return -EINVAL;
// Extract entropy from the TRNG output using HKDF-SHA512-Extract.
    hmac_sha512_usingrawkey(default_salt, sizeof(default_salt), entropy,
    TRNG_SEED_LEN_BYTES, entropy);
// Write the extracted entropy to the hardware.
    xtrng_write_multiple_registers(rng.rng_base + TRNG_EXT_SEED_OFFSET,
    (u32 *)entropy, TRNG_NUM_INIT_REGS);
// Clear the entropy from the stack.
    memzero_explicit(entropy, sizeof(entropy));
// select reseed operation
    iowrite32(TRNG_CTRL_PRNGXS_MASK, rng.rng_base + TRNG_CTRL_OFFSET);
// Start the reseed operation with above configuration and wait for STATUS.Done bit to be
// set. Monitor STATUS.CERTF bit, if set indicates SP800-90B entropy health test has failed.
//
    xtrng_readwrite32(rng.rng_base + TRNG_CTRL_OFFSET, TRNG_CTRL_PRNGSTART_MASK,
    TRNG_CTRL_PRNGSTART_MASK);
    ret = readl_poll_timeout(rng.rng_base + TRNG_STATUS_OFFSET, val,
    (val & TRNG_STATUS_DONE_MASK) == TRNG_STATUS_DONE_MASK,
    1U, 15000U);
    if (ret)
    return ret;
    xtrng_readwrite32(rng.rng_base + TRNG_CTRL_OFFSET, TRNG_CTRL_PRNGSTART_MASK, 0U);
    return 0;
    }
    static int xtrng_random_bytes_generate(struct xilinx_rng *rng, u8 *rand_buf_ptr,
    u32 rand_buf_size, int wait)
    {
    int nbytes;
    int ret;
    xtrng_readwrite32(rng.rng_base + TRNG_CTRL_OFFSET,
    TRNG_CTRL_PRNGMODE_MASK | TRNG_CTRL_PRNGXS_MASK,
    TRNG_CTRL_PRNGMODE_MASK | TRNG_CTRL_PRNGXS_MASK);
    nbytes = xtrng_collect_random_data(rng, rand_buf_ptr, rand_buf_size, wait);
    ret = xtrng_reseed_internal(rng);
    if (ret) {
    dev_err(rng.dev, "Re-seed fail\n");
    return ret;
    }
    return nbytes;
    }
#[no_mangle]
unsafe extern "C" fn xtrng_hwrng_trng_read(hwrng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int xtrng_hwrng_trng_read(struct hwrng *hwrng, void *data, size_t max, bool wait)
    {
    u8 buf[TRNG_SEC_STRENGTH_BYTES];
    struct xilinx_rng *rng;
    let mut ret: c_int = 0, i = 0;
    rng = container_of(hwrng, struct xilinx_rng, trng);
    while (i < max) {
    ret = xtrng_random_bytes_generate(rng, buf, TRNG_SEC_STRENGTH_BYTES, wait);
    if (ret < 0) {
    if (i == 0)
    return ret;
    break;
    }
    memcpy(data + i, buf, min_t(int, ret, (max - i)));
    i += min_t(int, ret, (max - i));
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn xtrng_hwrng_register(trng: *mut hwrng) -> c_int {
    static int xtrng_hwrng_register(struct hwrng *trng)
    {
    int ret;
    trng.name = "Xilinx Versal Crypto Engine TRNG";
    trng.read = xtrng_hwrng_trng_read;
    ret = hwrng_register(trng);
    if (ret)
    pr_err("Fail to register the TRNG\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xtrng_hwrng_unregister(trng: *mut hwrng) {
    static void xtrng_hwrng_unregister(struct hwrng *trng)
    {
    hwrng_unregister(trng);
    }
#[no_mangle]
unsafe extern "C" fn xtrng_probe(pdev: *mut platform_device) -> c_int {
    static int xtrng_probe(struct platform_device *pdev)
    {
    struct xilinx_rng *rng;
    int ret;
    rng = devm_kzalloc(&pdev.dev, sizeof(*rng), GFP_KERNEL);
    if (!rng)
    return -ENOMEM;
    rng.dev = &pdev.dev;
    rng.rng_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rng.rng_base)) {
    dev_err(&pdev.dev, "Failed to map resource %pe\n", rng.rng_base);
    return PTR_ERR(rng.rng_base);
    }
    xtrng_trng_reset(rng.rng_base);
    ret = xtrng_reseed_internal(rng);
    if (ret) {
    dev_err(&pdev.dev, "TRNG Seed fail\n");
    return ret;
    }
    ret = xtrng_hwrng_register(&rng.trng);
    if (ret) {
    dev_err(&pdev.dev, "HWRNG device registration failed: %d\n", ret);
    return ret;
    }
    platform_set_drvdata(pdev, rng);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xtrng_remove(pdev: *mut platform_device) {
    static void xtrng_remove(struct platform_device *pdev)
    {
    struct xilinx_rng *rng;
    u32 zero[TRNG_NUM_INIT_REGS] = { };
    rng = platform_get_drvdata(pdev);
    xtrng_hwrng_unregister(&rng.trng);
    xtrng_write_multiple_registers(rng.rng_base + TRNG_EXT_SEED_OFFSET, zero,
    TRNG_NUM_INIT_REGS);
    xtrng_write_multiple_registers(rng.rng_base + TRNG_PER_STRNG_OFFSET, zero,
    TRNG_NUM_INIT_REGS);
    xtrng_hold_reset(rng.rng_base);
    }
    static const struct of_device_id xtrng_of_match[] = {
    { .compatible = "xlnx,versal-trng", },
    {},
    };
    MODULE_DEVICE_TABLE(of, xtrng_of_match);
    static struct platform_driver xtrng_driver = {
    .driver = {
    .name = "xlnx,versal-trng",
    .of_match_table	= xtrng_of_match,
    },
    .probe = xtrng_probe,
    .remove = xtrng_remove,
    };
    module_platform_driver(xtrng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Harsh Jain <h.jain@amd.com>");
    MODULE_AUTHOR("Mounika Botcha <mounika.botcha@amd.com>");
    MODULE_DESCRIPTION("True Random Number Generator Driver");
