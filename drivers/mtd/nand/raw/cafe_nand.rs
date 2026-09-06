//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/cafe_nand.c
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
// Driver for One Laptop Per Child ‘CAFÉ’ controller, aka Marvell 88ALP01
//
// The data sheet for this device can be found at:
// http://wiki.laptop.org/go/Datasheets
//
// Copyright © 2006 Red Hat, Inc.
// Copyright © 2006 David Woodhouse <dwmw2@infradead.org>
//
// Macro flag: #define DEBUG

pub const CAFE_NAND_CTRL1: c_uint = 0x00;
pub const CAFE_NAND_CTRL2: c_uint = 0x04;
pub const CAFE_NAND_CTRL3: c_uint = 0x08;
pub const CAFE_NAND_STATUS: c_uint = 0x0c;
pub const CAFE_NAND_IRQ: c_uint = 0x10;
pub const CAFE_NAND_IRQ_MASK: c_uint = 0x14;
pub const CAFE_NAND_DATA_LEN: c_uint = 0x18;
pub const CAFE_NAND_ADDR1: c_uint = 0x1c;
pub const CAFE_NAND_ADDR2: c_uint = 0x20;
pub const CAFE_NAND_TIMING1: c_uint = 0x24;
pub const CAFE_NAND_TIMING2: c_uint = 0x28;
pub const CAFE_NAND_TIMING3: c_uint = 0x2c;
pub const CAFE_NAND_NONMEM: c_uint = 0x30;
pub const CAFE_NAND_ECC_RESULT: c_uint = 0x3C;
pub const CAFE_NAND_DMA_CTRL: c_uint = 0x40;
pub const CAFE_NAND_DMA_ADDR0: c_uint = 0x44;
pub const CAFE_NAND_DMA_ADDR1: c_uint = 0x48;
pub const CAFE_NAND_ECC_SYN01: c_uint = 0x50;
pub const CAFE_NAND_ECC_SYN23: c_uint = 0x54;
pub const CAFE_NAND_ECC_SYN45: c_uint = 0x58;
pub const CAFE_NAND_ECC_SYN67: c_uint = 0x5c;
pub const CAFE_NAND_READ_DATA: c_uint = 0x1000;
pub const CAFE_NAND_WRITE_DATA: c_uint = 0x2000;
pub const CAFE_GLOBAL_CTRL: c_uint = 0x3004;
pub const CAFE_GLOBAL_IRQ: c_uint = 0x3008;
pub const CAFE_GLOBAL_IRQ_MASK: c_uint = 0x300c;
pub const CAFE_NAND_RESET: c_uint = 0x3034;
// Missing from the datasheet: bit 19 of CTRL1 sets CE0 vs. CE1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cafe_priv {
    pub nand: nand_chip,
    pub pdev: *mut pci_dev,
    pub mmio: *mut void __iomem,
    pub rs: *mut rs_control,
    pub ctl1: u32,
    pub ctl2: u32,
    pub datalen: c_int,
    pub nr_data: c_int,
    pub data_pos: c_int,
    pub page_addr: c_int,
    pub usedma: bool,
    pub dmaaddr: dma_addr_t,
    pub dmabuf: *mut c_uchar,
}

    let mut usedma: static int = 1;
    module_param(usedma, int, 0644);
    let mut skipbbt: static int = 0;
    module_param(skipbbt, int, 0644);
    let mut debug: static int = 0;
    module_param(debug, int, 0644);
    let mut regdebug: static int = 0;
    module_param(regdebug, int, 0644);
    let mut checkecc: static int = 1;
    module_param(checkecc, int, 0644);
    static unsigned int numtimings;
    static int timing[3];
    module_param_array(timing, int, &numtimings, 0644);
    static const char *part_probes[] = { "cmdlinepart", "RedBoot", core::ptr::null_mut() };
// Hrm. Why isn't this already conditional on something in the struct device?

// Make it easier to switch to PIO if we need to

#[no_mangle]
unsafe extern "C" fn cafe_device_ready(chip: *mut nand_chip) -> c_int {
    static int cafe_device_ready(struct nand_chip *chip)
    {
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    let mut result: c_int = !!(cafe_readl(cafe, NAND_STATUS) & 0x40000000);
    let mut irqs: u32 = cafe_readl(cafe, NAND_IRQ);
    cafe_writel(cafe, irqs, NAND_IRQ);
    cafe_dev_dbg(&cafe.pdev.dev, "NAND device is%s ready, IRQ %x (%x) (%x,%x)\n",
    result?"":" not", irqs, cafe_readl(cafe, NAND_IRQ),
    cafe_readl(cafe, GLOBAL_IRQ), cafe_readl(cafe, GLOBAL_IRQ_MASK));
    return result;
    }
#[no_mangle]
unsafe extern "C" fn cafe_write_buf(chip: *mut nand_chip, buf: *const u8, len: c_int) {
    static void cafe_write_buf(struct nand_chip *chip, const uint8_t *buf, int len)
    {
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    if (cafe.usedma)
    memcpy(cafe.dmabuf + cafe.datalen, buf, len);
    else
    memcpy_toio(cafe.mmio + CAFE_NAND_WRITE_DATA + cafe.datalen, buf, len);
    cafe.datalen += len;
    cafe_dev_dbg(&cafe.pdev.dev, "Copy 0x%x bytes to write buffer. datalen 0x%x\n",
    len, cafe.datalen);
    }
#[no_mangle]
unsafe extern "C" fn cafe_read_buf(chip: *mut nand_chip, buf: *mut u8, len: c_int) {
    static void cafe_read_buf(struct nand_chip *chip, uint8_t *buf, int len)
    {
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    if (cafe.usedma)
    memcpy(buf, cafe.dmabuf + cafe.datalen, len);
    else
    memcpy_fromio(buf, cafe.mmio + CAFE_NAND_READ_DATA + cafe.datalen, len);
    cafe_dev_dbg(&cafe.pdev.dev, "Copy 0x%x bytes from position 0x%x in read buffer.\n",
    len, cafe.datalen);
    cafe.datalen += len;
    }
#[no_mangle]
unsafe extern "C" fn cafe_read_byte(chip: *mut nand_chip) -> u8 {
    static uint8_t cafe_read_byte(struct nand_chip *chip)
    {
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    uint8_t d;
    cafe_read_buf(chip, &d, 1);
    cafe_dev_dbg(&cafe.pdev.dev, "Read %02x\n", d);
    return d;
    }
    static void cafe_nand_cmdfunc(struct nand_chip *chip, unsigned command,
    int column, int page_addr)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    let mut adrbytes: c_int = 0;
    uint32_t ctl1;
    let mut doneint: u32 = 0x80000000;
    cafe_dev_dbg(&cafe.pdev.dev, "cmdfunc %02x, 0x%x, 0x%x\n",
    command, column, page_addr);
    if (command == NAND_CMD_ERASE2 || command == NAND_CMD_PAGEPROG) {
// Second half of a command we already calculated
    cafe_writel(cafe, cafe.ctl2 | 0x100 | command, NAND_CTRL2);
    ctl1 = cafe.ctl1;
    cafe.ctl2 &= ~(1<<30);
    cafe_dev_dbg(&cafe.pdev.dev, "Continue command, ctl1 %08x, #data %d\n",
    cafe.ctl1, cafe.nr_data);
    goto do_command;
    }
// Reset ECC engine
    cafe_writel(cafe, 0, NAND_CTRL2);
// Emulate NAND_CMD_READOOB on large-page chips
    if (mtd.writesize > 512 &&
    command == NAND_CMD_READOOB) {
    column += mtd.writesize;
    command = NAND_CMD_READ0;
    }
// FIXME: Do we need to send read command before sending data
    for small-page chips, to position the buffer correctly? */
    if (column != -1) {
    cafe_writel(cafe, column, NAND_ADDR1);
    adrbytes = 2;
    if (page_addr != -1)
    goto write_adr2;
    } else if (page_addr != -1) {
    cafe_writel(cafe, page_addr & 0xffff, NAND_ADDR1);
    page_addr >>= 16;
    write_adr2:
    cafe_writel(cafe, page_addr, NAND_ADDR2);
    adrbytes += 2;
    if (mtd.size > mtd.writesize << 16)
    adrbytes++;
    }
    cafe.data_pos = cafe.datalen = 0;
// Set command valid bit, mask in the chip select bit
    ctl1 = 0x80000000 | command | (cafe.ctl1 & CTRL1_CHIPSELECT);
// Set RD or WR bits as appropriate
    if (command == NAND_CMD_READID || command == NAND_CMD_STATUS) {
    ctl1 |= (1<<26); /* rd */
// Always 5 bytes, for now
    cafe.datalen = 4;
// And one address cycle -- even for STATUS, since the controller doesn't work without
    adrbytes = 1;
    } else if (command == NAND_CMD_READ0 || command == NAND_CMD_READ1 ||
    command == NAND_CMD_READOOB || command == NAND_CMD_RNDOUT) {
    ctl1 |= 1<<26; /* rd */
// For now, assume just read to end of page
    cafe.datalen = mtd.writesize + mtd.oobsize - column;
    } else if (command == NAND_CMD_SEQIN)
    ctl1 |= 1<<25; /* wr */
// Set number of address bytes
    if (adrbytes)
    ctl1 |= ((adrbytes-1)|8) << 27;
    if (command == NAND_CMD_SEQIN || command == NAND_CMD_ERASE1) {
// Ignore the first command of a pair; the hardware
    deals with them both at once, later */
    cafe.ctl1 = ctl1;
    cafe_dev_dbg(&cafe.pdev.dev, "Setup for delayed command, ctl1 %08x, dlen %x\n",
    cafe.ctl1, cafe.datalen);
    return;
    }
// RNDOUT and READ0 commands need a following byte
    if (command == NAND_CMD_RNDOUT)
    cafe_writel(cafe, cafe.ctl2 | 0x100 | NAND_CMD_RNDOUTSTART, NAND_CTRL2);
#[no_mangle]
pub unsafe extern "C" fn if(512: command == NAND_CMD_READ0 && mtd->writesize >) -> else {
    else if (command == NAND_CMD_READ0 && mtd.writesize > 512)
    cafe_writel(cafe, cafe.ctl2 | 0x100 | NAND_CMD_READSTART, NAND_CTRL2);
    do_command:
    cafe_dev_dbg(&cafe.pdev.dev, "dlen %x, ctl1 %x, ctl2 %x\n",
    cafe.datalen, ctl1, cafe_readl(cafe, NAND_CTRL2));
// NB: The datasheet lies -- we really should be subtracting 1 here
    cafe_writel(cafe, cafe.datalen, NAND_DATA_LEN);
    cafe_writel(cafe, 0x90000000, NAND_IRQ);
    if (cafe.usedma && (ctl1 & (3<<25))) {
    let mut dmactl: u32 = 0xc0000000 + cafe.datalen;
// If WR or RD bits set, set up DMA
    if (ctl1 & (1<<26)) {
// It's a read
    dmactl |= (1<<29);
// ... so it's done when the DMA is done, not just
    the command. */
    doneint = 0x10000000;
    }
    cafe_writel(cafe, dmactl, NAND_DMA_CTRL);
    }
    cafe.datalen = 0;
    if (unlikely(regdebug)) {
    int i;
    printk("About to write command %08x to register 0\n", ctl1);
    for (i=4; i< 0x5c; i+=4)
    printk("Register %x: %08x\n", i, readl(cafe.mmio + i));
    }
    cafe_writel(cafe, ctl1, NAND_CTRL1);
// Apply this short delay always to ensure that we do wait tWB in
// any case on any machine.
    ndelay(100);
    if (1) {
    int c;
    uint32_t irqs;
    for (c = 500000; c != 0; c--) {
    irqs = cafe_readl(cafe, NAND_IRQ);
    if (irqs & doneint)
    break;
    udelay(1);
    if (!(c % 100000))
    cafe_dev_dbg(&cafe.pdev.dev, "Wait for ready, IRQ %x\n", irqs);
    cpu_relax();
    }
    cafe_writel(cafe, doneint, NAND_IRQ);
    cafe_dev_dbg(&cafe.pdev.dev, "Command %x completed after %d usec, irqs %x (%x)\n",
    command, 500000-c, irqs, cafe_readl(cafe, NAND_IRQ));
    }
    WARN_ON(cafe.ctl2 & (1<<30));
    switch (command) {
    case NAND_CMD_CACHEDPROG:
    case NAND_CMD_PAGEPROG:
    case NAND_CMD_ERASE1:
    case NAND_CMD_ERASE2:
    case NAND_CMD_SEQIN:
    case NAND_CMD_RNDIN:
    case NAND_CMD_STATUS:
    case NAND_CMD_RNDOUT:
    cafe_writel(cafe, cafe.ctl2, NAND_CTRL2);
    return;
    }
    nand_wait_ready(chip);
    cafe_writel(cafe, cafe.ctl2, NAND_CTRL2);
    }
#[no_mangle]
unsafe extern "C" fn cafe_select_chip(chip: *mut nand_chip, chipnr: c_int) {
    static void cafe_select_chip(struct nand_chip *chip, int chipnr)
    {
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    cafe_dev_dbg(&cafe.pdev.dev, "select_chip %d\n", chipnr);
// Mask the appropriate bit into the stored value of ctl1
    which will be used by cafe_nand_cmdfunc() */
    if (chipnr)
    cafe.ctl1 |= CTRL1_CHIPSELECT;
    else
    cafe.ctl1 &= ~CTRL1_CHIPSELECT;
    }
#[no_mangle]
unsafe extern "C" fn cafe_nand_interrupt(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cafe_nand_interrupt(int irq, void *id)
    {
    struct mtd_info *mtd = id;
    struct nand_chip *chip = mtd_to_nand(mtd);
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    let mut irqs: u32 = cafe_readl(cafe, NAND_IRQ);
    cafe_writel(cafe, irqs & ~0x90000000, NAND_IRQ);
    if (!irqs)
    return IRQ_NONE;
    cafe_dev_dbg(&cafe.pdev.dev, "irq, bits %x (%x)\n", irqs, cafe_readl(cafe, NAND_IRQ));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cafe_nand_write_oob(chip: *mut nand_chip, page: c_int) -> c_int {
    static int cafe_nand_write_oob(struct nand_chip *chip, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    return nand_prog_page_op(chip, page, mtd.writesize, chip.oob_poi,
    mtd.oobsize);
    }
// Don't use -- use nand_read_oob_std for now
#[no_mangle]
unsafe extern "C" fn cafe_nand_read_oob(chip: *mut nand_chip, page: c_int) -> c_int {
    static int cafe_nand_read_oob(struct nand_chip *chip, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    return nand_read_oob_op(chip, page, 0, chip.oob_poi, mtd.oobsize);
    }
//
// cafe_nand_read_page - [REPLACEABLE] hardware ecc syndrome based page read
// @chip:	nand chip info structure
// @buf:	buffer to store read data
// @oob_required:	caller expects OOB data read to chip->oob_poi
// @page:	page number to read
//
// The hw generator calculates the error syndrome automatically. Therefore
// we need a special oob layout and handling.
//
    static int cafe_nand_read_page(struct nand_chip *chip, uint8_t *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    let mut max_bitflips: c_uint = 0;
    cafe_dev_dbg(&cafe.pdev.dev, "ECC result %08x SYN1,2 %08x\n",
    cafe_readl(cafe, NAND_ECC_RESULT),
    cafe_readl(cafe, NAND_ECC_SYN01));
    nand_read_page_op(chip, page, 0, buf, mtd.writesize);
    chip.legacy.read_buf(chip, chip.oob_poi, mtd.oobsize);
    if (checkecc && cafe_readl(cafe, NAND_ECC_RESULT) & (1<<18)) {
    unsigned short syn[8], pat[4];
    int pos[4];
    u8 *oob = chip.oob_poi;
    int i, n;
    for (i=0; i<8; i+=2) {
    let mut tmp: u32 = cafe_readl(cafe, NAND_ECC_SYN01 + (i*2));
    syn[i] = cafe.rs.codec.index_of[tmp & 0xfff];
    syn[i+1] = cafe.rs.codec.index_of[(tmp >> 16) & 0xfff];
    }
    n = decode_rs16(cafe.rs, core::ptr::null_mut(), core::ptr::null_mut(), 1367, syn, 0, pos, 0,
    pat);
    for (i = 0; i < n; i++) {
    let mut p: c_int = pos[i];
// The 12-bit symbols are mapped to bytes here
    if (p > 1374) {
// out of range
    n = -1374;
    } else if (p == 0) {
// high four bits do not correspond to data
    if (pat[i] > 0xff)
    n = -2048;
    else
    buf[0] ^= pat[i];
    } else if (p == 1365) {
    buf[2047] ^= pat[i] >> 4;
    oob[0] ^= pat[i] << 4;
    } else if (p > 1365) {
    if ((p & 1) == 1) {
    oob[3*p/2 - 2048] ^= pat[i] >> 4;
    oob[3*p/2 - 2047] ^= pat[i] << 4;
    } else {
    oob[3*p/2 - 2049] ^= pat[i] >> 8;
    oob[3*p/2 - 2048] ^= pat[i];
    }
    } else if ((p & 1) == 1) {
    buf[3*p/2] ^= pat[i] >> 4;
    buf[3*p/2 + 1] ^= pat[i] << 4;
    } else {
    buf[3*p/2 - 1] ^= pat[i] >> 8;
    buf[3*p/2] ^= pat[i];
    }
    }
    if (n < 0) {
    dev_dbg(&cafe.pdev.dev, "Failed to correct ECC at %08x\n",
    cafe_readl(cafe, NAND_ADDR2) * 2048);
    for (i = 0; i < 0x5c; i += 4)
    printk("Register %x: %08x\n", i, readl(cafe.mmio + i));
    mtd.ecc_stats.failed++;
    } else {
    dev_dbg(&cafe.pdev.dev, "Corrected %d symbol errors\n", n);
    mtd.ecc_stats.corrected += n;
    max_bitflips = max_t(unsigned int, max_bitflips, n);
    }
    }
    return max_bitflips;
    }
    static int cafe_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    if (section)
    return -ERANGE;
    oobregion.offset = 0;
    oobregion.length = chip.ecc.total;
    return 0;
    }
    static int cafe_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    if (section)
    return -ERANGE;
    oobregion.offset = chip.ecc.total;
    oobregion.length = mtd.oobsize - chip.ecc.total;
    return 0;
    }
    static const struct mtd_ooblayout_ops cafe_ooblayout_ops = {
    .ecc = cafe_ooblayout_ecc,
    .free = cafe_ooblayout_free,
    };
// Ick. The BBT code really ought to be able to work this bit out
    for itself from the above, at least for the 2KiB case */
    static uint8_t cafe_bbt_pattern_2048[] = { 'B', 'b', 't', '0' };
    static uint8_t cafe_mirror_pattern_2048[] = { '1', 't', 'b', 'B' };
    static uint8_t cafe_bbt_pattern_512[] = { 0xBB };
    static uint8_t cafe_mirror_pattern_512[] = { 0xBC };
    static struct nand_bbt_descr cafe_bbt_main_descr_2048 = {
    .options = NAND_BBT_LASTBLOCK | NAND_BBT_CREATE | NAND_BBT_WRITE
    | NAND_BBT_2BIT | NAND_BBT_VERSION,
    .offs =	14,
    .len = 4,
    .veroffs = 18,
    .maxblocks = 4,
    .pattern = cafe_bbt_pattern_2048
    };
    static struct nand_bbt_descr cafe_bbt_mirror_descr_2048 = {
    .options = NAND_BBT_LASTBLOCK | NAND_BBT_CREATE | NAND_BBT_WRITE
    | NAND_BBT_2BIT | NAND_BBT_VERSION,
    .offs =	14,
    .len = 4,
    .veroffs = 18,
    .maxblocks = 4,
    .pattern = cafe_mirror_pattern_2048
    };
    static struct nand_bbt_descr cafe_bbt_main_descr_512 = {
    .options = NAND_BBT_LASTBLOCK | NAND_BBT_CREATE | NAND_BBT_WRITE
    | NAND_BBT_2BIT | NAND_BBT_VERSION,
    .offs =	14,
    .len = 1,
    .veroffs = 15,
    .maxblocks = 4,
    .pattern = cafe_bbt_pattern_512
    };
    static struct nand_bbt_descr cafe_bbt_mirror_descr_512 = {
    .options = NAND_BBT_LASTBLOCK | NAND_BBT_CREATE | NAND_BBT_WRITE
    | NAND_BBT_2BIT | NAND_BBT_VERSION,
    .offs =	14,
    .len = 1,
    .veroffs = 15,
    .maxblocks = 4,
    .pattern = cafe_mirror_pattern_512
    };
    static int cafe_nand_write_page_lowlevel(struct nand_chip *chip,
    const uint8_t *buf, int oob_required,
    int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    nand_prog_page_begin_op(chip, page, 0, buf, mtd.writesize);
    chip.legacy.write_buf(chip, chip.oob_poi, mtd.oobsize);
// Set up ECC autogeneration
    cafe.ctl2 |= (1<<30);
    return nand_prog_page_end_op(chip);
    }
// F_2[X]/(X**6+X+1)
#[no_mangle]
unsafe extern "C" fn gf64_mul(a: u8, b: u8) -> c_ushort {
    static unsigned short gf64_mul(u8 a, u8 b)
    {
    u8 c;
    unsigned int i;
    c = 0;
    for (i = 0; i < 6; i++) {
    if (a & 1)
    c ^= b;
    a >>= 1;
    b <<= 1;
    if ((b & 0x40) != 0)
    b ^= 0x43;
    }
    return c;
    }
// F_64[X]/(X**2+X+A**-1) with A the generator of F_64[X]
#[no_mangle]
unsafe extern "C" fn gf4096_mul(a: u16, b: u16) -> u16 {
    static u16 gf4096_mul(u16 a, u16 b)
    {
    u8 ah, al, bh, bl, ch, cl;
    ah = a >> 6;
    al = a & 0x3f;
    bh = b >> 6;
    bl = b & 0x3f;
    ch = gf64_mul(ah ^ al, bh ^ bl) ^ gf64_mul(al, bl);
    cl = gf64_mul(gf64_mul(ah, bh), 0x21) ^ gf64_mul(al, bl);
    return (ch << 6) ^ cl;
    }
#[no_mangle]
unsafe extern "C" fn cafe_mul(x: c_int) -> c_int {
    static int cafe_mul(int x)
    {
    if (x == 0)
    return 1;
    return gf4096_mul(x, 0xe01);
    }
#[no_mangle]
unsafe extern "C" fn cafe_nand_attach_chip(chip: *mut nand_chip) -> c_int {
    static int cafe_nand_attach_chip(struct nand_chip *chip)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    let mut err: c_int = 0;
    cafe.dmabuf = dma_alloc_coherent(&cafe.pdev.dev, 2112,
    &cafe.dmaaddr, GFP_KERNEL);
    if (!cafe.dmabuf)
    return -ENOMEM;
// Set up DMA address
    cafe_writel(cafe, lower_32_bits(cafe.dmaaddr), NAND_DMA_ADDR0);
    cafe_writel(cafe, upper_32_bits(cafe.dmaaddr), NAND_DMA_ADDR1);
    cafe_dev_dbg(&cafe.pdev.dev, "Set DMA address to %x (virt %p)\n",
    cafe_readl(cafe, NAND_DMA_ADDR0), cafe.dmabuf);
// Restore the DMA flag
    cafe.usedma = usedma;
    cafe.ctl2 = BIT(27); /* Reed-Solomon ECC */
    if (mtd.writesize == 2048)
    cafe.ctl2 |= BIT(29); /* 2KiB page size */
// Set up ECC according to the type of chip we found
    mtd_set_ooblayout(mtd, &cafe_ooblayout_ops);
    if (mtd.writesize == 2048) {
    cafe.nand.bbt_td = &cafe_bbt_main_descr_2048;
    cafe.nand.bbt_md = &cafe_bbt_mirror_descr_2048;
    } else if (mtd.writesize == 512) {
    cafe.nand.bbt_td = &cafe_bbt_main_descr_512;
    cafe.nand.bbt_md = &cafe_bbt_mirror_descr_512;
    } else {
    dev_warn(&cafe.pdev.dev,
    "Unexpected NAND flash writesize %d. Aborting\n",
    mtd.writesize);
    err = -ENOTSUPP;
    goto out_free_dma;
    }
    cafe.nand.ecc.engine_type = NAND_ECC_ENGINE_TYPE_ON_HOST;
    cafe.nand.ecc.placement = NAND_ECC_PLACEMENT_INTERLEAVED;
    cafe.nand.ecc.size = mtd.writesize;
    cafe.nand.ecc.bytes = 14;
    cafe.nand.ecc.strength = 4;
    cafe.nand.ecc.write_page = cafe_nand_write_page_lowlevel;
    cafe.nand.ecc.write_oob = cafe_nand_write_oob;
    cafe.nand.ecc.read_page = cafe_nand_read_page;
    cafe.nand.ecc.read_oob = cafe_nand_read_oob;
    return 0;
    out_free_dma:
    dma_free_coherent(&cafe.pdev.dev, 2112, cafe.dmabuf, cafe.dmaaddr);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cafe_nand_detach_chip(chip: *mut nand_chip) {
    static void cafe_nand_detach_chip(struct nand_chip *chip)
    {
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    dma_free_coherent(&cafe.pdev.dev, 2112, cafe.dmabuf, cafe.dmaaddr);
    }
    static const struct nand_controller_ops cafe_nand_controller_ops = {
    .attach_chip = cafe_nand_attach_chip,
    .detach_chip = cafe_nand_detach_chip,
    };
    static int cafe_nand_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct mtd_info *mtd;
    struct cafe_priv *cafe;
    uint32_t ctrl;
    let mut err: c_int = 0;
// Very old versions shared the same PCI ident for all three
    functions on the chip. Verify the class too... */
    if ((pdev.class >> 8) != PCI_CLASS_MEMORY_FLASH)
    return -ENODEV;
    err = pci_enable_device(pdev);
    if (err)
    return err;
    pci_set_master(pdev);
    cafe = kzalloc_obj(*cafe);
    if (!cafe) {
    err = -ENOMEM;
    goto out_disable_device;
    }
    mtd = nand_to_mtd(&cafe.nand);
    mtd.dev.parent = &pdev.dev;
    nand_set_controller_data(&cafe.nand, cafe);
    cafe.pdev = pdev;
    cafe.mmio = pci_iomap(pdev, 0, 0);
    if (!cafe.mmio) {
    dev_warn(&pdev.dev, "failed to iomap\n");
    err = -ENOMEM;
    goto out_free_mtd;
    }
    cafe.rs = init_rs_non_canonical(12, &cafe_mul, 0, 1, 8);
    if (!cafe.rs) {
    err = -ENOMEM;
    goto out_ior;
    }
    cafe.nand.legacy.cmdfunc = cafe_nand_cmdfunc;
    cafe.nand.legacy.dev_ready = cafe_device_ready;
    cafe.nand.legacy.read_byte = cafe_read_byte;
    cafe.nand.legacy.read_buf = cafe_read_buf;
    cafe.nand.legacy.write_buf = cafe_write_buf;
    cafe.nand.legacy.select_chip = cafe_select_chip;
    cafe.nand.legacy.set_features = nand_get_set_features_notsupp;
    cafe.nand.legacy.get_features = nand_get_set_features_notsupp;
    cafe.nand.legacy.chip_delay = 0;
// Enable the following for a flash based bad block table
    cafe.nand.bbt_options = NAND_BBT_USE_FLASH;
    if (skipbbt)
    cafe.nand.options |= NAND_SKIP_BBTSCAN | NAND_NO_BBM_QUIRK;
    if (numtimings && numtimings != 3) {
    dev_warn(&cafe.pdev.dev, "%d timing register values ignored; precisely three are required\n", numtimings);
    }
    if (numtimings == 3) {
    cafe_dev_dbg(&cafe.pdev.dev, "Using provided timings (%08x %08x %08x)\n",
    timing[0], timing[1], timing[2]);
    } else {
    timing[0] = cafe_readl(cafe, NAND_TIMING1);
    timing[1] = cafe_readl(cafe, NAND_TIMING2);
    timing[2] = cafe_readl(cafe, NAND_TIMING3);
    if (timing[0] | timing[1] | timing[2]) {
    cafe_dev_dbg(&cafe.pdev.dev, "Timing registers already set (%08x %08x %08x)\n",
    timing[0], timing[1], timing[2]);
    } else {
    dev_warn(&cafe.pdev.dev, "Timing registers unset; using most conservative defaults\n");
    timing[0] = timing[1] = timing[2] = 0xffffffff;
    }
    }
// Start off by resetting the NAND controller completely
    cafe_writel(cafe, 1, NAND_RESET);
    cafe_writel(cafe, 0, NAND_RESET);
    cafe_writel(cafe, timing[0], NAND_TIMING1);
    cafe_writel(cafe, timing[1], NAND_TIMING2);
    cafe_writel(cafe, timing[2], NAND_TIMING3);
    cafe_writel(cafe, 0xffffffff, NAND_IRQ_MASK);
    err = request_irq(pdev.irq, &cafe_nand_interrupt, IRQF_SHARED,
    "CAFE NAND", mtd);
    if (err) {
    dev_warn(&pdev.dev, "Could not register IRQ %d\n", pdev.irq);
    goto out_free_rs;
    }
// Disable master reset, enable NAND clock
    ctrl = cafe_readl(cafe, GLOBAL_CTRL);
    ctrl &= 0xffffeff0;
    ctrl |= 0x00007000;
    cafe_writel(cafe, ctrl | 0x05, GLOBAL_CTRL);
    cafe_writel(cafe, ctrl | 0x0a, GLOBAL_CTRL);
    cafe_writel(cafe, 0, NAND_DMA_CTRL);
    cafe_writel(cafe, 0x7006, GLOBAL_CTRL);
    cafe_writel(cafe, 0x700a, GLOBAL_CTRL);
// Enable NAND IRQ in global IRQ mask register
    cafe_writel(cafe, 0x80000007, GLOBAL_IRQ_MASK);
    cafe_dev_dbg(&cafe.pdev.dev, "Control %x, IRQ mask %x\n",
    cafe_readl(cafe, GLOBAL_CTRL),
    cafe_readl(cafe, GLOBAL_IRQ_MASK));
// Do not use the DMA during the NAND identification
    cafe.usedma = 0;
// Scan to find existence of the device
    cafe.nand.legacy.dummy_controller.ops = &cafe_nand_controller_ops;
    err = nand_scan(&cafe.nand, 2);
    if (err)
    goto out_irq;
    pci_set_drvdata(pdev, mtd);
    mtd.name = "cafe_nand";
    err = mtd_device_parse_register(mtd, part_probes, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    if (err)
    goto out_cleanup_nand;
    goto out;
    out_cleanup_nand:
    nand_cleanup(&cafe.nand);
    out_irq:
// Disable NAND IRQ in global IRQ mask register
    cafe_writel(cafe, ~1 & cafe_readl(cafe, GLOBAL_IRQ_MASK), GLOBAL_IRQ_MASK);
    free_irq(pdev.irq, mtd);
    out_free_rs:
    free_rs(cafe.rs);
    out_ior:
    pci_iounmap(pdev, cafe.mmio);
    out_free_mtd:
    kfree(cafe);
    out_disable_device:
    pci_disable_device(pdev);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cafe_nand_remove(pdev: *mut pci_dev) {
    static void cafe_nand_remove(struct pci_dev *pdev)
    {
    struct mtd_info *mtd = pci_get_drvdata(pdev);
    struct nand_chip *chip = mtd_to_nand(mtd);
    struct cafe_priv *cafe = nand_get_controller_data(chip);
    int ret;
// Disable NAND IRQ in global IRQ mask register
    cafe_writel(cafe, ~1 & cafe_readl(cafe, GLOBAL_IRQ_MASK), GLOBAL_IRQ_MASK);
    free_irq(pdev.irq, mtd);
    ret = mtd_device_unregister(mtd);
    WARN_ON(ret);
    nand_cleanup(chip);
    free_rs(cafe.rs);
    pci_iounmap(pdev, cafe.mmio);
    dma_free_coherent(&cafe.pdev.dev, 2112, cafe.dmabuf, cafe.dmaaddr);
    kfree(cafe);
    pci_disable_device(pdev);
    }
    static const struct pci_device_id cafe_nand_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL, PCI_DEVICE_ID_MARVELL_88ALP01_NAND) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, cafe_nand_tbl);
#[no_mangle]
unsafe extern "C" fn cafe_nand_resume(dev: *mut device) -> c_int {
    static int cafe_nand_resume(struct device *dev)
    {
    uint32_t ctrl;
    struct pci_dev *pdev = to_pci_dev(dev);
    struct mtd_info *mtd = pci_get_drvdata(pdev);
    struct nand_chip *chip = mtd_to_nand(mtd);
    struct cafe_priv *cafe = nand_get_controller_data(chip);
// Start off by resetting the NAND controller completely
    cafe_writel(cafe, 1, NAND_RESET);
    cafe_writel(cafe, 0, NAND_RESET);
    cafe_writel(cafe, 0xffffffff, NAND_IRQ_MASK);
// Restore timing configuration
    cafe_writel(cafe, timing[0], NAND_TIMING1);
    cafe_writel(cafe, timing[1], NAND_TIMING2);
    cafe_writel(cafe, timing[2], NAND_TIMING3);
// Disable master reset, enable NAND clock
    ctrl = cafe_readl(cafe, GLOBAL_CTRL);
    ctrl &= 0xffffeff0;
    ctrl |= 0x00007000;
    cafe_writel(cafe, ctrl | 0x05, GLOBAL_CTRL);
    cafe_writel(cafe, ctrl | 0x0a, GLOBAL_CTRL);
    cafe_writel(cafe, 0, NAND_DMA_CTRL);
    cafe_writel(cafe, 0x7006, GLOBAL_CTRL);
    cafe_writel(cafe, 0x700a, GLOBAL_CTRL);
// Set up DMA address
    cafe_writel(cafe, cafe.dmaaddr & 0xffffffff, NAND_DMA_ADDR0);
    if (sizeof(cafe.dmaaddr) > 4)
// Shift in two parts to shut the compiler up
    cafe_writel(cafe, (cafe.dmaaddr >> 16) >> 16, NAND_DMA_ADDR1);
    else
    cafe_writel(cafe, 0, NAND_DMA_ADDR1);
// Enable NAND IRQ in global IRQ mask register
    cafe_writel(cafe, 0x80000007, GLOBAL_IRQ_MASK);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(cafe_nand_ops, core::ptr::null_mut(), cafe_nand_resume);
    static struct pci_driver cafe_nand_pci_driver = {
    .name = "CAFÉ NAND",
    .id_table = cafe_nand_tbl,
    .probe = cafe_nand_probe,
    .remove = cafe_nand_remove,
    .driver.pm = &cafe_nand_ops,
    };
    module_pci_driver(cafe_nand_pci_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_DESCRIPTION("NAND flash driver for OLPC CAFÉ chip");
