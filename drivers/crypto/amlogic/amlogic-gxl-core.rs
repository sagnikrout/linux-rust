//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/amlogic/amlogic-gxl-core.c
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
// amlgoic-core.c - hardware cryptographic offloader for Amlogic GXL SoC
//
// Copyright (C) 2018-2019 Corentin Labbe <clabbe@baylibre.com>
//
// Core file which registers crypto algorithms supported by the hardware.
//

#[no_mangle]
unsafe extern "C" fn meson_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t meson_irq_handler(int irq, void *data)
    {
    struct meson_dev *mc = (struct meson_dev *)data;
    int flow;
    u32 p;
    for (flow = 0; flow < MAXFLOW; flow++) {
    if (mc.irqs[flow] == irq) {
    p = readl(mc.base + ((0x04 + flow) << 2));
    if (p) {
    writel_relaxed(0xF, mc.base + ((0x4 + flow) << 2));
    mc.chanlist[flow].status = 1;
    complete(&mc.chanlist[flow].complete);
    return IRQ_HANDLED;
    }
    dev_err(mc.dev, "%s %d Got irq for flow %d but ctrl is empty\n", __func__, irq, flow);
    }
    }
    dev_err(mc.dev, "%s %d from unknown irq\n", __func__, irq);
    return IRQ_HANDLED;
    }
    static struct meson_alg_template mc_algs[] = {
    {
    .type = CRYPTO_ALG_TYPE_SKCIPHER,
    .blockmode = MESON_OPMODE_CBC,
    .alg.skcipher.base = {
    .base = {
    .cra_name = "cbc(aes)",
    .cra_driver_name = "cbc-aes-gxl",
    .cra_priority = 400,
    .cra_blocksize = AES_BLOCK_SIZE,
    .cra_flags = CRYPTO_ALG_TYPE_SKCIPHER |
    CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_ctxsize = sizeof(struct meson_cipher_tfm_ctx),
    .cra_module = THIS_MODULE,
    .cra_alignmask = 0xf,
    .cra_init = meson_cipher_init,
    .cra_exit = meson_cipher_exit,
    },
    .min_keysize	= AES_MIN_KEY_SIZE,
    .max_keysize	= AES_MAX_KEY_SIZE,
    .ivsize		= AES_BLOCK_SIZE,
    .setkey		= meson_aes_setkey,
    .encrypt	= meson_skencrypt,
    .decrypt	= meson_skdecrypt,
    },
    .alg.skcipher.op = {
    .do_one_request = meson_handle_cipher_request,
    },
    },
    {
    .type = CRYPTO_ALG_TYPE_SKCIPHER,
    .blockmode = MESON_OPMODE_ECB,
    .alg.skcipher.base = {
    .base = {
    .cra_name = "ecb(aes)",
    .cra_driver_name = "ecb-aes-gxl",
    .cra_priority = 400,
    .cra_blocksize = AES_BLOCK_SIZE,
    .cra_flags = CRYPTO_ALG_TYPE_SKCIPHER |
    CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_ctxsize = sizeof(struct meson_cipher_tfm_ctx),
    .cra_module = THIS_MODULE,
    .cra_alignmask = 0xf,
    .cra_init = meson_cipher_init,
    .cra_exit = meson_cipher_exit,
    },
    .min_keysize	= AES_MIN_KEY_SIZE,
    .max_keysize	= AES_MAX_KEY_SIZE,
    .setkey		= meson_aes_setkey,
    .encrypt	= meson_skencrypt,
    .decrypt	= meson_skdecrypt,
    },
    .alg.skcipher.op = {
    .do_one_request = meson_handle_cipher_request,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn meson_debugfs_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int meson_debugfs_show(struct seq_file *seq, void *v)
    {
    let mut __maybe_unused: *mut meson_dev mc = seq.private;
    int i;
    for (i = 0; i < MAXFLOW; i++)
    seq_printf(seq, "Channel %d: nreq %lu\n", i,

    mc.chanlist[i].stat_req);

    0ul);

    for (i = 0; i < ARRAY_SIZE(mc_algs); i++) {
    switch (mc_algs[i].type) {
    case CRYPTO_ALG_TYPE_SKCIPHER:
    seq_printf(seq, "%s %s %lu %lu\n",
    mc_algs[i].alg.skcipher.base.base.cra_driver_name,
    mc_algs[i].alg.skcipher.base.base.cra_name,

    mc_algs[i].stat_req, mc_algs[i].stat_fb);

    0ul, 0ul);

    break;
    }
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(meson_debugfs);
#[no_mangle]
unsafe extern "C" fn meson_free_chanlist(mc: *mut meson_dev, i: c_int) {
    static void meson_free_chanlist(struct meson_dev *mc, int i)
    {
    while (i >= 0) {
    crypto_engine_exit(mc.chanlist[i].engine);
    if (mc.chanlist[i].tl)
    dma_free_coherent(mc.dev, sizeof(struct meson_desc) * MAXDESC,
    mc.chanlist[i].tl,
    mc.chanlist[i].t_phy);
    i--;
    }
    }
//
// Allocate the channel list structure
//
#[no_mangle]
unsafe extern "C" fn meson_allocate_chanlist(mc: *mut meson_dev) -> c_int {
    static int meson_allocate_chanlist(struct meson_dev *mc)
    {
    int i, err;
    mc.chanlist = devm_kcalloc(mc.dev, MAXFLOW,
    sizeof(struct meson_flow), GFP_KERNEL);
    if (!mc.chanlist)
    return -ENOMEM;
    for (i = 0; i < MAXFLOW; i++) {
    init_completion(&mc.chanlist[i].complete);
    mc.chanlist[i].engine = crypto_engine_alloc_init(mc.dev, true);
    if (!mc.chanlist[i].engine) {
    dev_err(mc.dev, "Cannot allocate engine\n");
    i--;
    err = -ENOMEM;
    goto error_engine;
    }
    err = crypto_engine_start(mc.chanlist[i].engine);
    if (err) {
    dev_err(mc.dev, "Cannot start engine\n");
    goto error_engine;
    }
    mc.chanlist[i].tl = dma_alloc_coherent(mc.dev,
    sizeof(struct meson_desc) * MAXDESC,
    &mc.chanlist[i].t_phy,
    GFP_KERNEL);
    if (!mc.chanlist[i].tl) {
    err = -ENOMEM;
    goto error_engine;
    }
    }
    return 0;
    error_engine:
    meson_free_chanlist(mc, i);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn meson_register_algs(mc: *mut meson_dev) -> c_int {
    static int meson_register_algs(struct meson_dev *mc)
    {
    int err, i;
    for (i = 0; i < ARRAY_SIZE(mc_algs); i++) {
    mc_algs[i].mc = mc;
    switch (mc_algs[i].type) {
    case CRYPTO_ALG_TYPE_SKCIPHER:
    err = crypto_engine_register_skcipher(&mc_algs[i].alg.skcipher);
    if (err) {
    dev_err(mc.dev, "Fail to register %s\n",
    mc_algs[i].alg.skcipher.base.base.cra_name);
    mc_algs[i].mc = core::ptr::null_mut();
    return err;
    }
    break;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_unregister_algs(mc: *mut meson_dev) {
    static void meson_unregister_algs(struct meson_dev *mc)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(mc_algs); i++) {
    if (!mc_algs[i].mc)
    continue;
    switch (mc_algs[i].type) {
    case CRYPTO_ALG_TYPE_SKCIPHER:
    crypto_engine_unregister_skcipher(&mc_algs[i].alg.skcipher);
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn meson_crypto_probe(pdev: *mut platform_device) -> c_int {
    static int meson_crypto_probe(struct platform_device *pdev)
    {
    struct meson_dev *mc;
    int err, i;
    mc = devm_kzalloc(&pdev.dev, sizeof(*mc), GFP_KERNEL);
    if (!mc)
    return -ENOMEM;
    mc.dev = &pdev.dev;
    platform_set_drvdata(pdev, mc);
    mc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mc.base))
    return PTR_ERR(mc.base);
    mc.busclk = devm_clk_get(&pdev.dev, "blkmv");
    if (IS_ERR(mc.busclk)) {
    err = PTR_ERR(mc.busclk);
    dev_err(&pdev.dev, "Cannot get core clock err=%d\n", err);
    return err;
    }
    for (i = 0; i < MAXFLOW; i++) {
    mc.irqs[i] = platform_get_irq(pdev, i);
    if (mc.irqs[i] < 0)
    return mc.irqs[i];
    err = devm_request_irq(&pdev.dev, mc.irqs[i], meson_irq_handler, 0,
    "gxl-crypto", mc);
    if (err < 0)
    return err;
    }
    err = clk_prepare_enable(mc.busclk);
    if (err != 0) {
    dev_err(&pdev.dev, "Cannot prepare_enable busclk\n");
    return err;
    }
    err = meson_allocate_chanlist(mc);
    if (err)
    goto error_flow;
    err = meson_register_algs(mc);
    if (err)
    goto error_alg;
    if (IS_ENABLED(CONFIG_CRYPTO_DEV_AMLOGIC_GXL_DEBUG)) {
    struct dentry *dbgfs_dir;
    dbgfs_dir = debugfs_create_dir("gxl-crypto", core::ptr::null_mut());
    debugfs_create_file("stats", 0444, dbgfs_dir, mc, &meson_debugfs_fops);

    mc.dbgfs_dir = dbgfs_dir;

    }
    return 0;
    error_alg:
    meson_unregister_algs(mc);
    meson_free_chanlist(mc, MAXFLOW - 1);
    error_flow:
    clk_disable_unprepare(mc.busclk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn meson_crypto_remove(pdev: *mut platform_device) {
    static void meson_crypto_remove(struct platform_device *pdev)
    {
    struct meson_dev *mc = platform_get_drvdata(pdev);

    debugfs_remove_recursive(mc.dbgfs_dir);

    meson_unregister_algs(mc);
    meson_free_chanlist(mc, MAXFLOW - 1);
    clk_disable_unprepare(mc.busclk);
    }
    static const struct of_device_id meson_crypto_of_match_table[] = {
    { .compatible = "amlogic,gxl-crypto", },
    {}
    };
    MODULE_DEVICE_TABLE(of, meson_crypto_of_match_table);
    static struct platform_driver meson_crypto_driver = {
    .probe		 = meson_crypto_probe,
    .remove		 = meson_crypto_remove,
    .driver		 = {
    .name		   = "gxl-crypto",
    .of_match_table	= meson_crypto_of_match_table,
    },
    };
    module_platform_driver(meson_crypto_driver);
    MODULE_DESCRIPTION("Amlogic GXL cryptographic offloader");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Corentin Labbe <clabbe@baylibre.com>");
