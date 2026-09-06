//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/fsl_upm.c
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
// Freescale UPM NAND driver.
//
// Copyright © 2007-2008  MontaVista Software, Inc.
//
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_upm_nand {
    pub base: nand_controller,
    pub dev: *mut device,
    pub chip: nand_chip,
    pub upm: fsl_upm,
    pub upm_addr_offset: u8,
    pub upm_cmd_offset: u8,
    pub io_base: *mut void __iomem,
    pub rnb_gpio: [*mut gpio_desc; NAND_MAX_CHIPS],
    pub mchip_offsets: [u32; NAND_MAX_CHIPS],
    pub mchip_count: u32,
    pub mchip_number: u32,
}

    static inline struct fsl_upm_nand *to_fsl_upm_nand(struct mtd_info *mtdinfo)
    {
    return container_of(mtd_to_nand(mtdinfo), struct fsl_upm_nand,
    chip);
    }
    static int fun_chip_init(struct fsl_upm_nand *fun,
    const struct device_node *upm_np,
    const struct resource *io_res)
    {
    struct mtd_info *mtd = nand_to_mtd(&fun.chip);
    int ret;
    struct device_node *flash_np;
    fun.chip.ecc.engine_type = NAND_ECC_ENGINE_TYPE_SOFT;
    fun.chip.ecc.algo = NAND_ECC_ALGO_HAMMING;
    fun.chip.controller = &fun.base;
    mtd.dev.parent = fun.dev;
    flash_np = of_get_next_child(upm_np, core::ptr::null_mut());
    if (!flash_np)
    return -ENODEV;
    nand_set_flash_node(&fun.chip, flash_np);
    mtd.name = devm_kasprintf(fun.dev, GFP_KERNEL, "0x%llx.%pOFn",
    (u64)io_res.start,
    flash_np);
    if (!mtd.name) {
    ret = -ENOMEM;
    goto err;
    }
    ret = nand_scan(&fun.chip, fun.mchip_count);
    if (ret)
    goto err;
    ret = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    err:
    of_node_put(flash_np);
    return ret;
    }
    static int func_exec_instr(struct nand_chip *chip,
    const struct nand_op_instr *instr)
    {
    struct fsl_upm_nand *fun = to_fsl_upm_nand(nand_to_mtd(chip));
    u32 mar, reg_offs = fun.mchip_offsets[fun.mchip_number];
    unsigned int i;
    const u8 *out;
    u8 *in;
    switch (instr.type) {
    case NAND_OP_CMD_INSTR:
    fsl_upm_start_pattern(&fun.upm, fun.upm_cmd_offset);
    mar = (instr.ctx.cmd.opcode << (32 - fun.upm.width)) |
    reg_offs;
    fsl_upm_run_pattern(&fun.upm, fun.io_base + reg_offs, mar);
    fsl_upm_end_pattern(&fun.upm);
    return 0;
    case NAND_OP_ADDR_INSTR:
    fsl_upm_start_pattern(&fun.upm, fun.upm_addr_offset);
    for (i = 0; i < instr.ctx.addr.naddrs; i++) {
    mar = (instr.ctx.addr.addrs[i] << (32 - fun.upm.width)) |
    reg_offs;
    fsl_upm_run_pattern(&fun.upm, fun.io_base + reg_offs, mar);
    }
    fsl_upm_end_pattern(&fun.upm);
    return 0;
    case NAND_OP_DATA_IN_INSTR:
    in = instr.ctx.data.buf.in;
    for (i = 0; i < instr.ctx.data.len; i++)
    in[i] = in_8(fun.io_base + reg_offs);
    return 0;
    case NAND_OP_DATA_OUT_INSTR:
    out = instr.ctx.data.buf.out;
    for (i = 0; i < instr.ctx.data.len; i++)
    out_8(fun.io_base + reg_offs, out[i]);
    return 0;
    case NAND_OP_WAITRDY_INSTR:
    if (!fun.rnb_gpio[fun.mchip_number])
    return nand_soft_waitrdy(chip, instr.ctx.waitrdy.timeout_ms);
    return nand_gpio_waitrdy(chip, fun.rnb_gpio[fun.mchip_number],
    instr.ctx.waitrdy.timeout_ms);
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int fun_exec_op(struct nand_chip *chip, const struct nand_operation *op,
    bool check_only)
    {
    struct fsl_upm_nand *fun = to_fsl_upm_nand(nand_to_mtd(chip));
    unsigned int i;
    int ret;
    if (op.cs >= NAND_MAX_CHIPS)
    return -EINVAL;
    if (check_only)
    return 0;
    fun.mchip_number = op.cs;
    for (i = 0; i < op.ninstrs; i++) {
    ret = func_exec_instr(chip, &op.instrs[i]);
    if (ret)
    return ret;
    if (op.instrs[i].delay_ns)
    ndelay(op.instrs[i].delay_ns);
    }
    return 0;
    }
    static const struct nand_controller_ops fun_ops = {
    .exec_op = fun_exec_op,
    };
#[no_mangle]
unsafe extern "C" fn fun_probe(ofdev: *mut platform_device) -> c_int {
    static int fun_probe(struct platform_device *ofdev)
    {
    struct fsl_upm_nand *fun;
    struct resource *io_res;
    const __be32 *prop;
    int ret;
    int size;
    int i;
    fun = devm_kzalloc(&ofdev.dev, sizeof(*fun), GFP_KERNEL);
    if (!fun)
    return -ENOMEM;
    fun.io_base = devm_platform_get_and_ioremap_resource(ofdev, 0, &io_res);
    if (IS_ERR(fun.io_base))
    return PTR_ERR(fun.io_base);
    ret = fsl_upm_find(io_res.start, &fun.upm);
    if (ret) {
    dev_err(&ofdev.dev, "can't find UPM\n");
    return ret;
    }
    prop = of_get_property(ofdev.dev.of_node, "fsl,upm-addr-offset",
    &size);
    if (!prop || size != sizeof(uint32_t)) {
    dev_err(&ofdev.dev, "can't get UPM address offset\n");
    return -EINVAL;
    }
    fun.upm_addr_offset = *prop;
    prop = of_get_property(ofdev.dev.of_node, "fsl,upm-cmd-offset", &size);
    if (!prop || size != sizeof(uint32_t)) {
    dev_err(&ofdev.dev, "can't get UPM command offset\n");
    return -EINVAL;
    }
    fun.upm_cmd_offset = *prop;
    prop = of_get_property(ofdev.dev.of_node,
    "fsl,upm-addr-line-cs-offsets", &size);
    if (prop && (size / sizeof(uint32_t)) > 0) {
    fun.mchip_count = size / sizeof(uint32_t);
    if (fun.mchip_count >= NAND_MAX_CHIPS) {
    dev_err(&ofdev.dev, "too much multiple chips\n");
    return -EINVAL;
    }
    for (i = 0; i < fun.mchip_count; i++)
    fun.mchip_offsets[i] = be32_to_cpu(prop[i]);
    } else {
    fun.mchip_count = 1;
    }
    for (i = 0; i < fun.mchip_count; i++) {
    fun.rnb_gpio[i] = devm_gpiod_get_index_optional(&ofdev.dev,
    core::ptr::null_mut(), i,
    GPIOD_IN);
    if (IS_ERR(fun.rnb_gpio[i])) {
    dev_err(&ofdev.dev, "RNB gpio #%d is invalid\n", i);
    return PTR_ERR(fun.rnb_gpio[i]);
    }
    }
    nand_controller_init(&fun.base);
    fun.base.ops = &fun_ops;
    fun.dev = &ofdev.dev;
    ret = fun_chip_init(fun, ofdev.dev.of_node, io_res);
    if (ret)
    return ret;
    dev_set_drvdata(&ofdev.dev, fun);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fun_remove(ofdev: *mut platform_device) {
    static void fun_remove(struct platform_device *ofdev)
    {
    struct fsl_upm_nand *fun = dev_get_drvdata(&ofdev.dev);
    struct nand_chip *chip = &fun.chip;
    struct mtd_info *mtd = nand_to_mtd(chip);
    int ret;
    ret = mtd_device_unregister(mtd);
    WARN_ON(ret);
    nand_cleanup(chip);
    }
    static const struct of_device_id of_fun_match[] = {
    { .compatible = "fsl,upm-nand" },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_fun_match);
    static struct platform_driver of_fun_driver = {
    .driver = {
    .name = "fsl,upm-nand",
    .of_match_table = of_fun_match,
    },
    .probe		= fun_probe,
    .remove		= fun_remove,
    };
    module_platform_driver(of_fun_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Anton Vorontsov <avorontsov@ru.mvista.com>");
    MODULE_DESCRIPTION("Driver for NAND chips working through Freescale "
    "LocalBus User-Programmable Machine");
