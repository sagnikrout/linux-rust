//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/cs553x_nand.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// (C) 2005, 2006 Red Hat Inc.
//
// Author: David Woodhouse <dwmw2@infradead.org>
// Tom Sylla <tom.sylla@amd.com>
//
// Overview:
// This is a device driver for the NAND flash controller found on
// the AMD CS5535/CS5536 companion chipsets for the Geode processor.
// mtd-id for command line partitioning is cs553x_nand_cs[0-3]
// where 0-3 reflects the chip select for NAND.
//

pub const NR_CS553X_CONTROLLERS: c_int = 4;
pub const MSR_DIVIL_GLD_CAP: c_uint = 0x51400000	/* DIVIL capabilities */;
pub const CAP_CS5535: c_uint = 0x2df000ULL;
pub const CAP_CS5536: c_uint = 0x5df500ULL;
// NAND Timing MSRs
pub const MSR_NANDF_DATA: c_uint = 0x5140001b	/* NAND Flash Data Timing MSR */;
pub const MSR_NANDF_CTL: c_uint = 0x5140001c	/* NAND Flash Control Timing */;
pub const MSR_NANDF_RSVD: c_uint = 0x5140001d	/* Reserved */;
// NAND BAR MSRs
pub const MSR_DIVIL_LBAR_FLSH0: c_uint = 0x51400010	/* Flash Chip Select 0 */;
pub const MSR_DIVIL_LBAR_FLSH1: c_uint = 0x51400011	/* Flash Chip Select 1 */;
pub const MSR_DIVIL_LBAR_FLSH2: c_uint = 0x51400012	/* Flash Chip Select 2 */;
pub const MSR_DIVIL_LBAR_FLSH3: c_uint = 0x51400013	/* Flash Chip Select 3 */;
// Each made up of...

// I/O BARs have BASE_ADDR in bits 15:4, IO_MASK in 47:36
// MMIO BARs have BASE_ADDR in bits 31:12, MEM_MASK in 63:44
// Pin function selection MSR (IDE vs. flash on the IDE pins)
pub const MSR_DIVIL_BALL_OPTS: c_uint = 0x51400015;

// Registers within the NAND flash controller BAR -- memory mapped
pub const MM_NAND_DATA: c_uint = 0x00	/* 0 to 0x7ff, in fact */;
pub const MM_NAND_CTL: c_uint = 0x800	/* Any even address 0x800-0x80e */;
pub const MM_NAND_IO: c_uint = 0x801	/* Any odd address 0x801-0x80f */;
pub const MM_NAND_STS: c_uint = 0x810;
pub const MM_NAND_ECC_LSB: c_uint = 0x811;
pub const MM_NAND_ECC_MSB: c_uint = 0x812;
pub const MM_NAND_ECC_COL: c_uint = 0x813;
pub const MM_NAND_LAC: c_uint = 0x814;
pub const MM_NAND_ECC_CTL: c_uint = 0x815;
// Registers within the NAND flash controller BAR -- I/O mapped
pub const IO_NAND_DATA: c_uint = 0x00	/* 0 to 3, in fact */;
pub const IO_NAND_CTL: c_uint = 0x04;
pub const IO_NAND_IO: c_uint = 0x05;
pub const IO_NAND_STS: c_uint = 0x06;
pub const IO_NAND_ECC_CTL: c_uint = 0x08;
pub const IO_NAND_ECC_LSB: c_uint = 0x09;
pub const IO_NAND_ECC_MSB: c_uint = 0x0a;
pub const IO_NAND_ECC_COL: c_uint = 0x0b;
pub const IO_NAND_LAC: c_uint = 0x0c;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs553x_nand_controller {
    pub base: nand_controller,
    pub chip: nand_chip,
    pub mmio: *mut void __iomem,
}

    static struct cs553x_nand_controller *
    to_cs553x(struct nand_controller *controller)
    {
    return container_of(controller, struct cs553x_nand_controller, base);
    }
    static int cs553x_write_ctrl_byte(struct cs553x_nand_controller *cs553x,
    u32 ctl, u8 data)
    {
    u8 status;
    writeb(ctl, cs553x.mmio + MM_NAND_CTL);
    writeb(data, cs553x.mmio + MM_NAND_IO);
    return readb_poll_timeout_atomic(cs553x.mmio + MM_NAND_STS, status,
    !(status & CS_NAND_CTLR_BUSY), 1,
    100000);
    }
    static void cs553x_data_in(struct cs553x_nand_controller *cs553x, void *buf,
    unsigned int len)
    {
    writeb(0, cs553x.mmio + MM_NAND_CTL);
    while (unlikely(len > 0x800)) {
    memcpy_fromio(buf, cs553x.mmio, 0x800);
    buf += 0x800;
    len -= 0x800;
    }
    memcpy_fromio(buf, cs553x.mmio, len);
    }
    static void cs553x_data_out(struct cs553x_nand_controller *cs553x,
    const void *buf, unsigned int len)
    {
    writeb(0, cs553x.mmio + MM_NAND_CTL);
    while (unlikely(len > 0x800)) {
    memcpy_toio(cs553x.mmio, buf, 0x800);
    buf += 0x800;
    len -= 0x800;
    }
    memcpy_toio(cs553x.mmio, buf, len);
    }
    static int cs553x_wait_ready(struct cs553x_nand_controller *cs553x,
    unsigned int timeout_ms)
    {
    let mut mask: u8 = CS_NAND_CTLR_BUSY | CS_NAND_STS_FLASH_RDY;
    u8 status;
    return readb_poll_timeout(cs553x.mmio + MM_NAND_STS, status,
    (status & mask) == CS_NAND_STS_FLASH_RDY, 100,
    timeout_ms * 1000);
    }
    static int cs553x_exec_instr(struct cs553x_nand_controller *cs553x,
    const struct nand_op_instr *instr)
    {
    unsigned int i;
    let mut ret: c_int = 0;
    switch (instr.type) {
    case NAND_OP_CMD_INSTR:
    ret = cs553x_write_ctrl_byte(cs553x, CS_NAND_CTL_CLE,
    instr.ctx.cmd.opcode);
    break;
    case NAND_OP_ADDR_INSTR:
    for (i = 0; i < instr.ctx.addr.naddrs; i++) {
    ret = cs553x_write_ctrl_byte(cs553x, CS_NAND_CTL_ALE,
    instr.ctx.addr.addrs[i]);
    if (ret)
    break;
    }
    break;
    case NAND_OP_DATA_IN_INSTR:
    cs553x_data_in(cs553x, instr.ctx.data.buf.in,
    instr.ctx.data.len);
    break;
    case NAND_OP_DATA_OUT_INSTR:
    cs553x_data_out(cs553x, instr.ctx.data.buf.out,
    instr.ctx.data.len);
    break;
    case NAND_OP_WAITRDY_INSTR:
    ret = cs553x_wait_ready(cs553x, instr.ctx.waitrdy.timeout_ms);
    break;
    }
    if (instr.delay_ns)
    ndelay(instr.delay_ns);
    return ret;
    }
    static int cs553x_exec_op(struct nand_chip *this,
    const struct nand_operation *op,
    bool check_only)
    {
    struct cs553x_nand_controller *cs553x = to_cs553x(this.controller);
    unsigned int i;
    int ret;
    if (check_only)
    return true;
// De-assert the CE pin
    writeb(0, cs553x.mmio + MM_NAND_CTL);
    for (i = 0; i < op.ninstrs; i++) {
    ret = cs553x_exec_instr(cs553x, &op.instrs[i]);
    if (ret)
    break;
    }
// Re-assert the CE pin.
    writeb(CS_NAND_CTL_CE, cs553x.mmio + MM_NAND_CTL);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cs_enable_hwecc(this: *mut nand_chip, mode: c_int) {
    static void cs_enable_hwecc(struct nand_chip *this, int mode)
    {
    struct cs553x_nand_controller *cs553x = to_cs553x(this.controller);
    writeb(0x07, cs553x.mmio + MM_NAND_ECC_CTL);
    }
    static int cs_calculate_ecc(struct nand_chip *this, const u_char *dat,
    u_char *ecc_code)
    {
    struct cs553x_nand_controller *cs553x = to_cs553x(this.controller);
    uint32_t ecc;
    ecc = readl(cs553x.mmio + MM_NAND_STS);
    ecc_code[1] = ecc >> 8;
    ecc_code[0] = ecc >> 16;
    ecc_code[2] = ecc >> 24;
    return 0;
    }
    static struct cs553x_nand_controller *controllers[4];
#[no_mangle]
unsafe extern "C" fn cs553x_attach_chip(chip: *mut nand_chip) -> c_int {
    static int cs553x_attach_chip(struct nand_chip *chip)
    {
    if (chip.ecc.engine_type != NAND_ECC_ENGINE_TYPE_ON_HOST)
    return 0;
    chip.ecc.size = 256;
    chip.ecc.bytes = 3;
    chip.ecc.hwctl  = cs_enable_hwecc;
    chip.ecc.calculate = cs_calculate_ecc;
    chip.ecc.correct  = rawnand_sw_hamming_correct;
    chip.ecc.strength = 1;
    return 0;
    }
    static const struct nand_controller_ops cs553x_nand_controller_ops = {
    .exec_op = cs553x_exec_op,
    .attach_chip = cs553x_attach_chip,
    };
#[no_mangle]
unsafe extern "C" fn cs553x_init_one(cs: c_int, mmio: c_int, adr: c_ulong) -> int __init {
    static int __init cs553x_init_one(int cs, int mmio, unsigned long adr)
    {
    struct cs553x_nand_controller *controller;
    let mut err: c_int = 0;
    struct nand_chip *this;
    struct mtd_info *new_mtd;
    pr_notice("Probing CS553x NAND controller CS#%d at %sIO 0x%08lx\n",
    cs, mmio ? "MM" : "P", adr);
    if (!mmio) {
    pr_notice("PIO mode not yet implemented for CS553X NAND controller\n");
    return -ENXIO;
    }
// Allocate memory for MTD device structure and private data
    controller = kzalloc_obj(*controller);
    if (!controller) {
    err = -ENOMEM;
    goto out;
    }
    this = &controller.chip;
    nand_controller_init(&controller.base);
    controller.base.ops = &cs553x_nand_controller_ops;
    this.controller = &controller.base;
    new_mtd = nand_to_mtd(this);
// Link the private data with the MTD structure
    new_mtd.owner = THIS_MODULE;
// map physical address
    controller.mmio = ioremap(adr, 4096);
    if (!controller.mmio) {
    pr_warn("ioremap cs553x NAND @0x%08lx failed\n", adr);
    err = -EIO;
    goto out_mtd;
    }
// Enable the following for a flash based bad block table
    this.bbt_options = NAND_BBT_USE_FLASH;
    new_mtd.name = kasprintf(GFP_KERNEL, "cs553x_nand_cs%d", cs);
    if (!new_mtd.name) {
    err = -ENOMEM;
    goto out_ior;
    }
// Scan to find existence of the device
    err = nand_scan(this, 1);
    if (err)
    goto out_free;
    controllers[cs] = controller;
    goto out;
    out_free:
    kfree(new_mtd.name);
    out_ior:
    iounmap(controller.mmio);
    out_mtd:
    kfree(controller);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn is_geode() -> c_int {
    static int is_geode(void)
    {
// These are the CPUs which will have a CS553[56] companion chip
    if (boot_cpu_data.x86_vendor == X86_VENDOR_AMD &&
    boot_cpu_data.x86 == 5 &&
    boot_cpu_data.x86_model == 10)
    return 1; /* Geode LX */
    if ((boot_cpu_data.x86_vendor == X86_VENDOR_NSC ||
    boot_cpu_data.x86_vendor == X86_VENDOR_CYRIX) &&
    boot_cpu_data.x86 == 5 &&
    boot_cpu_data.x86_model == 5)
    return 1; /* Geode GX (née GX2) */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cs553x_init() -> int __init {
    static int __init cs553x_init(void)
    {
    let mut err: c_int = -ENXIO;
    int i;
    uint64_t val;
// If the CPU isn't a Geode GX or LX, abort
    if (!is_geode())
    return -ENXIO;
// If it doesn't have the CS553[56], abort
    rdmsrq(MSR_DIVIL_GLD_CAP, val);
    val &= ~0xFFULL;
    if (val != CAP_CS5535 && val != CAP_CS5536)
    return -ENXIO;
// If it doesn't have the NAND controller enabled, abort
    rdmsrq(MSR_DIVIL_BALL_OPTS, val);
    if (val & PIN_OPT_IDE) {
    pr_info("CS553x NAND controller: Flash I/O not enabled in MSR_DIVIL_BALL_OPTS.\n");
    return -ENXIO;
    }
    for (i = 0; i < NR_CS553X_CONTROLLERS; i++) {
    rdmsrq(MSR_DIVIL_LBAR_FLSH0 + i, val);
    if ((val & (FLSH_LBAR_EN|FLSH_NOR_NAND)) == (FLSH_LBAR_EN|FLSH_NOR_NAND))
    err = cs553x_init_one(i, !!(val & FLSH_MEM_IO), val & 0xFFFFFFFF);
    }
// Register all devices together here. This means we can easily hack it to
    do mtdconcat etc. if we want to. */
    for (i = 0; i < NR_CS553X_CONTROLLERS; i++) {
    if (controllers[i]) {
// If any devices registered, return success. Else the last error.
    mtd_device_register(nand_to_mtd(&controllers[i].chip),
    core::ptr::null_mut(), 0);
    err = 0;
    }
    }
    return err;
    }
    module_init(cs553x_init);
#[no_mangle]
unsafe extern "C" fn cs553x_cleanup() -> void __exit {
    static void __exit cs553x_cleanup(void)
    {
    int i;
    for (i = 0; i < NR_CS553X_CONTROLLERS; i++) {
    struct cs553x_nand_controller *controller = controllers[i];
    struct nand_chip *this = &controller.chip;
    struct mtd_info *mtd = nand_to_mtd(this);
    int ret;
    if (!mtd)
    continue;
// Release resources, unregister device
    ret = mtd_device_unregister(mtd);
    WARN_ON(ret);
    nand_cleanup(this);
    kfree(mtd.name);
    controllers[i] = core::ptr::null_mut();
// unmap physical address
    iounmap(controller.mmio);
// Free the MTD device structure
    kfree(controller);
    }
    }
    module_exit(cs553x_cleanup);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_DESCRIPTION("NAND controller driver for AMD CS5535/CS5536 companion chip");
