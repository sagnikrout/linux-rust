//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-pca953x.c
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
// PCA953x 4/8/16/24/40 bit I/O ports
//
// Copyright (C) 2005 Ben Gardner <bgardner@wabtec.com>
// Copyright (C) 2007 Marvell International Ltd.
//
// Derived from drivers/i2c/chips/pca9539.c
//

pub const PCA953X_INPUT: c_uint = 0x00;
pub const PCA953X_OUTPUT: c_uint = 0x01;
pub const PCA953X_INVERT: c_uint = 0x02;
pub const PCA953X_DIRECTION: c_uint = 0x03;
pub const TCA6418_INPUT: c_uint = 0x14;
pub const TCA6418_OUTPUT: c_uint = 0x17;
pub const TCA6418_DIRECTION: c_uint = 0x23;

pub const PCA957X_IN: c_uint = 0x00;
pub const PCA957X_INVRT: c_uint = 0x01;
pub const PCA957X_BKEN: c_uint = 0x02;
pub const PCA957X_PUPD: c_uint = 0x03;
pub const PCA957X_CFG: c_uint = 0x04;
pub const PCA957X_OUT: c_uint = 0x05;
pub const PCA957X_MSK: c_uint = 0x06;
pub const PCA957X_INTS: c_uint = 0x07;
pub const PCAL953X_OUT_STRENGTH: c_uint = 0x20;
pub const PCAL953X_IN_LATCH: c_uint = 0x22;
pub const PCAL953X_PULL_EN: c_uint = 0x23;
pub const PCAL953X_PULL_SEL: c_uint = 0x24;
pub const PCAL953X_INT_MASK: c_uint = 0x25;
pub const PCAL953X_INT_STAT: c_uint = 0x26;
pub const PCAL953X_OUT_CONF: c_uint = 0x27;
pub const PCAL6524_INT_EDGE: c_uint = 0x28;
pub const PCAL6524_INT_CLR: c_uint = 0x2a;
pub const PCAL6524_IN_STATUS: c_uint = 0x2b;
pub const PCAL6524_OUT_INDCONF: c_uint = 0x2c;
pub const PCAL6524_DEBOUNCE: c_uint = 0x2d;

    static const struct i2c_device_id pca953x_id[] = {
    { .name = "pca6408", .driver_data = 8  | PCA953X_TYPE | PCA_INT },
    { .name = "pca6416", .driver_data = 16 | PCA953X_TYPE | PCA_INT },
    { .name = "pca9505", .driver_data = 40 | PCA953X_TYPE | PCA_INT },
    { .name = "pca9506", .driver_data = 40 | PCA953X_TYPE | PCA_INT },
    { .name = "pca9534", .driver_data = 8  | PCA953X_TYPE | PCA_INT },
    { .name = "pca9535", .driver_data = 16 | PCA953X_TYPE | PCA_INT },
    { .name = "pca9536", .driver_data = 4  | PCA953X_TYPE },
    { .name = "pca9537", .driver_data = 4  | PCA953X_TYPE | PCA_INT },
    { .name = "pca9538", .driver_data = 8  | PCA953X_TYPE | PCA_INT },
    { .name = "pca9539", .driver_data = 16 | PCA953X_TYPE | PCA_INT },
    { .name = "pca9554", .driver_data = 8  | PCA953X_TYPE | PCA_INT },
    { .name = "pca9555", .driver_data = 16 | PCA953X_TYPE | PCA_INT },
    { .name = "pca9556", .driver_data = 8  | PCA953X_TYPE },
    { .name = "pca9557", .driver_data = 8  | PCA953X_TYPE },
    { .name = "pca9574", .driver_data = 8  | PCA957X_TYPE | PCA_INT },
    { .name = "pca9575", .driver_data = 16 | PCA957X_TYPE | PCA_INT },
    { .name = "pca9698", .driver_data = 40 | PCA953X_TYPE },
    { .name = "pcal6408",  .driver_data = 8  | PCA953X_TYPE | PCA_LATCH_INT },
    { .name = "pcal6416",  .driver_data = 16 | PCA953X_TYPE | PCA_LATCH_INT },
    { .name = "pcal6524",  .driver_data = 24 | PCA953X_TYPE | PCA_LATCH_INT },
    { .name = "pcal6534",  .driver_data = 34 | PCAL653X_TYPE | PCA_LATCH_INT },
    { .name = "pcal9535",  .driver_data = 16 | PCA953X_TYPE | PCA_LATCH_INT },
    { .name = "pcal9554b", .driver_data = 8  | PCA953X_TYPE | PCA_LATCH_INT },
    { .name = "pcal9555a", .driver_data = 16 | PCA953X_TYPE | PCA_LATCH_INT },
    { .name = "max7310", .driver_data = 8  | PCA953X_TYPE },
    { .name = "max7312", .driver_data = 16 | PCA953X_TYPE | PCA_INT },
    { .name = "max7313", .driver_data = 16 | PCA953X_TYPE | PCA_INT },
    { .name = "max7315", .driver_data = 8  | PCA953X_TYPE | PCA_INT },
    { .name = "max7318", .driver_data = 16 | PCA953X_TYPE | PCA_INT },
    { .name = "pca6107", .driver_data = 8  | PCA953X_TYPE | PCA_INT },
    { .name = "tca6408", .driver_data = 8  | PCA953X_TYPE | PCA_INT },
    { .name = "tca6416", .driver_data = 16 | PCA953X_TYPE | PCA_INT },
    { .name = "tca6418", .driver_data = 18 | TCA6418_TYPE | PCA_INT },
    { .name = "tca6424", .driver_data = 24 | PCA953X_TYPE | PCA_INT },
    { .name = "tca9538", .driver_data = 8  | PCA953X_TYPE | PCA_INT },
    { .name = "tca9539", .driver_data = 16 | PCA953X_TYPE | PCA_INT },
    { .name = "tca9554", .driver_data = 8  | PCA953X_TYPE | PCA_INT },
    { .name = "xra1202", .driver_data = 8  | PCA953X_TYPE },
    { .name = "tcal6408", .driver_data = 8  | PCA953X_TYPE | PCA_LATCH_INT },
    { .name = "tcal6416", .driver_data = 16 | PCA953X_TYPE | PCA_LATCH_INT },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pca953x_id);

    let mut pca953x_irq_gpios: static struct acpi_gpio_params = { 0, 0, true };
    static const struct acpi_gpio_mapping pca953x_acpi_irq_gpios[] = {
    { "irq-gpios", &pca953x_irq_gpios, 1, ACPI_GPIO_QUIRK_ABSOLUTE_NUMBER },
    { }
    };
#[no_mangle]
unsafe extern "C" fn pca953x_acpi_get_irq(dev: *mut device) -> c_int {
    static int pca953x_acpi_get_irq(struct device *dev)
    {
    int ret;
    ret = devm_acpi_dev_add_driver_gpios(dev, pca953x_acpi_irq_gpios);
    if (ret)
    dev_warn(dev, "can't add GPIO ACPI mapping\n");
    ret = acpi_dev_gpio_irq_get_by(ACPI_COMPANION(dev), "irq", 0);
    if (ret < 0)
    return ret;
    dev_info(dev, "ACPI interrupt quirk (IRQ %d)\n", ret);
    return ret;
    }
    static const struct dmi_system_id pca953x_dmi_acpi_irq_info[] = {
    {
//
// On Intel Galileo Gen 2 board the IRQ pin of one of
// the I²C GPIO expanders, which has GpioInt() resource,
// is provided as an absolute number instead of being
// relative. Since first controller (gpio-sch.c) and
// second (gpio-dwapb.c) are at the fixed bases, we may
// safely refer to the number in the global space to get
// an IRQ out of it.
//
    .matches = {
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "GalileoGen2"),
    },
    },
    {}
    };

    static const struct acpi_device_id pca953x_acpi_ids[] = {
    { "INT3491", 16 | PCA953X_TYPE | PCA_LATCH_INT, },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, pca953x_acpi_ids);
pub const MAX_BANK: c_int = 5;
pub const BANK_SZ: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca953x_reg_config {
    pub direction: c_int,
    pub output: c_int,
    pub input: c_int,
    pub invert: c_int,
}

    static const struct pca953x_reg_config pca953x_regs = {
    .direction = PCA953X_DIRECTION,
    .output = PCA953X_OUTPUT,
    .input = PCA953X_INPUT,
    .invert = PCA953X_INVERT,
    };
    static const struct pca953x_reg_config pca957x_regs = {
    .direction = PCA957X_CFG,
    .output = PCA957X_OUT,
    .input = PCA957X_IN,
    .invert = PCA957X_INVRT,
    };
    static const struct pca953x_reg_config tca6418_regs = {
    .direction = TCA6418_DIRECTION,
    .output = TCA6418_OUTPUT,
    .input = TCA6418_INPUT,
    .invert = 0xFF, /* Does not apply */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca953x_chip {
    pub gpio_start: unsigned,
    pub i2c_lock: mutex,
    pub regmap: *mut regmap,

    pub irq_lock: mutex,
    pub MAX_LINE): DECLARE_BITMAP(irq_mask,,
    pub MAX_LINE): DECLARE_BITMAP(irq_stat,,
    pub MAX_LINE): DECLARE_BITMAP(irq_trig_raise,,
    pub MAX_LINE): DECLARE_BITMAP(irq_trig_fall,,
    pub MAX_LINE): DECLARE_BITMAP(irq_trig_level_high,,
    pub MAX_LINE): DECLARE_BITMAP(irq_trig_level_low,,

    pub wakeup_path: core::sync::atomic::AtomicI32,
    pub client: *mut i2c_client,
    pub gpio_chip: gpio_chip,
    pub driver_data: c_ulong,
    pub regulator: *mut regulator,
    pub regs: *const pca953x_reg_config,
    pub off): *mut *mut *mut u8 (recalc_addr)(struct pca953x_chip chip, int reg, int,
    bool (*check_reg)(struct pca953x_chip *chip, unsigned int reg,
    pub checkbank): u32,
}

#[no_mangle]
unsafe extern "C" fn pca953x_bank_shift(chip: *mut pca953x_chip) -> c_int {
    static int pca953x_bank_shift(struct pca953x_chip *chip)
    {
    return fls((chip.gpio_chip.ngpio - 1) / BANK_SZ);
    }
//
// Helper function to get the correct bit mask for a given offset and chip type.
// The TCA6418's input, output, and direction banks have a peculiar bit order:
// the first byte uses reversed bit order, while the second byte uses standard order.
//
#[no_mangle]
pub unsafe extern "C" fn pca953x_get_bit_mask(chip: *mut pca953x_chip, offset: c_uint) -> u8 {
    static inline u8 pca953x_get_bit_mask(struct pca953x_chip *chip, unsigned int offset)
    {
    let mut bit_pos_in_bank: c_uint = offset % BANK_SZ;
    let mut msb: c_int = BANK_SZ - 1;
    if (PCA_CHIP_TYPE(chip.driver_data) == TCA6418_TYPE && offset <= msb)
    return BIT(msb - bit_pos_in_bank);
    return BIT(bit_pos_in_bank);
    }

//
// We care about the following registers:
// - Standard set, below 0x40, each port can be replicated up to 8 times
// - PCA953x standard
// Input port			0x00 + 0 * bank_size	R
// Output port			0x00 + 1 * bank_size	RW
// Polarity Inversion port		0x00 + 2 * bank_size	RW
// Configuration port		0x00 + 3 * bank_size	RW
// - PCA957x with mixed up registers
// Input port			0x00 + 0 * bank_size	R
// Polarity Inversion port		0x00 + 1 * bank_size	RW
// Bus hold port			0x00 + 2 * bank_size	RW
// Configuration port		0x00 + 4 * bank_size	RW
// Output port			0x00 + 5 * bank_size	RW
//
// - Extended set, above 0x40, often chip specific.
// - PCAL6524/PCAL9555A with custom PCAL IRQ handling:
// Input latch register		0x40 + 2 * bank_size	RW
// Pull-up/pull-down enable reg	0x40 + 3 * bank_size    RW
// Pull-up/pull-down select reg	0x40 + 4 * bank_size    RW
// Interrupt mask register		0x40 + 5 * bank_size	RW
// Interrupt status register	0x40 + 6 * bank_size	R
//
// - Registers with bit 0x80 set, the AI bit (auto increment)
// The bit is cleared and the registers fall into one of the
// categories above.
//
    static bool pca953x_check_register(struct pca953x_chip *chip, unsigned int reg,
    u32 checkbank)
    {
    let mut bank_shift: c_int = pca953x_bank_shift(chip);
    let mut bank: c_int = (reg & REG_ADDR_MASK) >> bank_shift;
    let mut offset: c_int = reg & (BIT(bank_shift) - 1);
// Special PCAL extended register check.
    if (reg & REG_ADDR_EXT) {
    if (!(chip.driver_data & PCA_PCAL))
    return false;
    bank += 8;
    }
// Register is not in the matching bank.
    if (!(BIT(bank) & checkbank))
    return false;
// Register is not within allowed range of bank.
    if (offset >= NBANK(chip))
    return false;
    return true;
    }
//
// Unfortunately, whilst the PCAL6534 chip (and compatibles) broadly follow the
// same register layout as the PCAL6524, the spacing of the registers has been
// fundamentally altered by compacting them and thus does not obey the same
// rules, including being able to use bit shifting to determine bank. These
// chips hence need special handling here.
//
    static bool pcal6534_check_register(struct pca953x_chip *chip, unsigned int reg,
    u32 checkbank)
    {
    int bank_shift;
    int bank;
    int offset;
    if (reg >= 0x54) {
//
// Handle lack of reserved registers after output port
// configuration register to form a bank.
//
    reg -= 0x54;
    bank_shift = 16;
    } else if (reg >= 0x30) {
//
// Reserved block between 14h and 2Fh does not align on
// expected bank boundaries like other devices.
//
    reg -= 0x30;
    bank_shift = 8;
    } else {
    bank_shift = 0;
    }
    bank = bank_shift + reg / NBANK(chip);
    offset = reg % NBANK(chip);
// Register is not in the matching bank.
    if (!(BIT(bank) & checkbank))
    return false;
// Register is not within allowed range of bank.
    if (offset >= NBANK(chip))
    return false;
    return true;
    }
// TCA6418 breaks the PCA953x register order rule
    static bool tca6418_check_register(struct pca953x_chip *chip, unsigned int reg,
    u32 access_type_mask)
    {
// Valid Input Registers - BIT(0) for readable access
    if (reg >= TCA6418_INPUT && reg < (TCA6418_INPUT + NBANK(chip)))
    return (access_type_mask & BIT(0));
// Valid Output Registers - BIT(1) for writeable access
    if (reg >= TCA6418_OUTPUT && reg < (TCA6418_OUTPUT + NBANK(chip)))
    return (access_type_mask & (BIT(0) | BIT(1)));
// Valid Direction Registers - BIT(2) for volatile access
    if (reg >= TCA6418_DIRECTION && reg < (TCA6418_DIRECTION + NBANK(chip)))
    return (access_type_mask & (BIT(0) | BIT(1)));
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pca953x_readable_register(dev: *mut device, reg: c_uint) -> bool {
    static bool pca953x_readable_register(struct device *dev, unsigned int reg)
    {
    struct pca953x_chip *chip = dev_get_drvdata(dev);
    u32 bank;
    switch (PCA_CHIP_TYPE(chip.driver_data)) {
    case PCA957X_TYPE:
    bank = PCA957x_BANK_INPUT | PCA957x_BANK_OUTPUT |
    PCA957x_BANK_POLARITY | PCA957x_BANK_CONFIG |
    PCA957x_BANK_BUSHOLD;
    break;
    case TCA6418_TYPE:
// BIT(0) to indicate read access
    return tca6418_check_register(chip, reg, BIT(0));
    default:
    bank = PCA953x_BANK_INPUT | PCA953x_BANK_OUTPUT |
    PCA953x_BANK_POLARITY | PCA953x_BANK_CONFIG;
    break;
    }
    if (chip.driver_data & PCA_PCAL) {
    bank |= PCAL9xxx_BANK_IN_LATCH | PCAL9xxx_BANK_PULL_EN |
    PCAL9xxx_BANK_PULL_SEL | PCAL9xxx_BANK_IRQ_MASK |
    PCAL9xxx_BANK_IRQ_STAT;
    }
    return chip.check_reg(chip, reg, bank);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_writeable_register(dev: *mut device, reg: c_uint) -> bool {
    static bool pca953x_writeable_register(struct device *dev, unsigned int reg)
    {
    struct pca953x_chip *chip = dev_get_drvdata(dev);
    u32 bank;
    switch (PCA_CHIP_TYPE(chip.driver_data)) {
    case PCA957X_TYPE:
    bank = PCA957x_BANK_OUTPUT | PCA957x_BANK_POLARITY |
    PCA957x_BANK_CONFIG | PCA957x_BANK_BUSHOLD;
    break;
    case TCA6418_TYPE:
// BIT(1) for write access
    return tca6418_check_register(chip, reg, BIT(1));
    default:
    bank = PCA953x_BANK_OUTPUT | PCA953x_BANK_POLARITY |
    PCA953x_BANK_CONFIG;
    break;
    }
    if (chip.driver_data & PCA_PCAL)
    bank |= PCAL9xxx_BANK_IN_LATCH | PCAL9xxx_BANK_PULL_EN |
    PCAL9xxx_BANK_PULL_SEL | PCAL9xxx_BANK_IRQ_MASK;
    return chip.check_reg(chip, reg, bank);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_volatile_register(dev: *mut device, reg: c_uint) -> bool {
    static bool pca953x_volatile_register(struct device *dev, unsigned int reg)
    {
    struct pca953x_chip *chip = dev_get_drvdata(dev);
    u32 bank;
    switch (PCA_CHIP_TYPE(chip.driver_data)) {
    case PCA957X_TYPE:
    bank = PCA957x_BANK_INPUT;
    break;
    case TCA6418_TYPE:
// BIT(2) for volatile access
    return tca6418_check_register(chip, reg, BIT(2));
    default:
    bank = PCA953x_BANK_INPUT;
    break;
    }
    if (chip.driver_data & PCA_PCAL)
    bank |= PCAL9xxx_BANK_IRQ_STAT;
    return chip.check_reg(chip, reg, bank);
    }
    static const struct regmap_config pca953x_i2c_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .use_single_read = true,
    .use_single_write = true,
    .readable_reg = pca953x_readable_register,
    .writeable_reg = pca953x_writeable_register,
    .volatile_reg = pca953x_volatile_register,
    .disable_locking = true,
    .cache_type = REGCACHE_MAPLE,
    .max_register = 0x7f,
    };
    static const struct regmap_config pca953x_ai_i2c_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .read_flag_mask = REG_ADDR_AI,
    .write_flag_mask = REG_ADDR_AI,
    .readable_reg = pca953x_readable_register,
    .writeable_reg = pca953x_writeable_register,
    .volatile_reg = pca953x_volatile_register,
    .disable_locking = true,
    .cache_type = REGCACHE_MAPLE,
    .max_register = 0x7f,
    };
#[no_mangle]
unsafe extern "C" fn pca953x_recalc_addr(chip: *mut pca953x_chip, reg: c_int, off: c_int) -> u8 {
    static u8 pca953x_recalc_addr(struct pca953x_chip *chip, int reg, int off)
    {
    let mut bank_shift: c_int = pca953x_bank_shift(chip);
    let mut addr: c_int = (reg & PCAL_GPIO_MASK) << bank_shift;
    let mut pinctrl: c_int = (reg & PCAL_PINCTRL_MASK) << 1;
    let mut regaddr: u8 = pinctrl | addr | (off / BANK_SZ);
    return regaddr;
    }
//
// The PCAL6534 and compatible chips have altered bank alignment that doesn't
// fit within the bit shifting scheme used for other devices.
//
#[no_mangle]
unsafe extern "C" fn pcal6534_recalc_addr(chip: *mut pca953x_chip, reg: c_int, off: c_int) -> u8 {
    static u8 pcal6534_recalc_addr(struct pca953x_chip *chip, int reg, int off)
    {
    int addr;
    int pinctrl;
    addr = (reg & PCAL_GPIO_MASK) * NBANK(chip);
    switch (reg) {
    case PCAL953X_OUT_STRENGTH:
    case PCAL953X_IN_LATCH:
    case PCAL953X_PULL_EN:
    case PCAL953X_PULL_SEL:
    case PCAL953X_INT_MASK:
    case PCAL953X_INT_STAT:
    pinctrl = ((reg & PCAL_PINCTRL_MASK) >> 1) + 0x20;
    break;
    case PCAL6524_INT_EDGE:
    case PCAL6524_INT_CLR:
    case PCAL6524_IN_STATUS:
    case PCAL6524_OUT_INDCONF:
    case PCAL6524_DEBOUNCE:
    pinctrl = ((reg & PCAL_PINCTRL_MASK) >> 1) + 0x1c;
    break;
    default:
    pinctrl = 0;
    break;
    }
    return pinctrl + addr + (off / BANK_SZ);
    }
#[no_mangle]
unsafe extern "C" fn tca6418_recalc_addr(chip: *mut pca953x_chip, reg_base: c_int, offset: c_int) -> u8 {
    static u8 tca6418_recalc_addr(struct pca953x_chip *chip, int reg_base, int offset)
    {
//
// reg_base will be TCA6418_INPUT, TCA6418_OUTPUT, or TCA6418_DIRECTION
// offset is the global GPIO line offset (0-17)
// BANK_SZ is 8 for TCA6418 (8 bits per register bank)
//
    return reg_base + (offset / BANK_SZ);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_write_regs(chip: *mut pca953x_chip, reg: c_int, val: *mut c_ulong) -> c_int {
    static int pca953x_write_regs(struct pca953x_chip *chip, int reg, unsigned long *val)
    {
    let mut regaddr: u8 = chip.recalc_addr(chip, reg, 0);
    u8 value[MAX_BANK];
    int i, ret;
    for (i = 0; i < NBANK(chip); i++)
    value[i] = bitmap_get_value8(val, i * BANK_SZ);
    ret = regmap_bulk_write(chip.regmap, regaddr, value, NBANK(chip));
    if (ret < 0) {
    dev_err(&chip.client.dev, "failed writing register: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca953x_read_regs(chip: *mut pca953x_chip, reg: c_int, val: *mut c_ulong) -> c_int {
    static int pca953x_read_regs(struct pca953x_chip *chip, int reg, unsigned long *val)
    {
    let mut regaddr: u8 = chip.recalc_addr(chip, reg, 0);
    u8 value[MAX_BANK];
    int i, ret;
    ret = regmap_bulk_read(chip.regmap, regaddr, value, NBANK(chip));
    if (ret < 0) {
    dev_err(&chip.client.dev, "failed reading register: %d\n", ret);
    return ret;
    }
    for (i = 0; i < NBANK(chip); i++)
    bitmap_set_value8(val, value[i], i * BANK_SZ);
    return 0;
    }
    static int pca953x_gpio_direction_input_unlocked(struct gpio_chip *gc,
    unsigned int off)
    {
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    let mut dirreg: u8 = chip.recalc_addr(chip, chip.regs.direction, off);
    let mut bit: u8 = pca953x_get_bit_mask(chip, off);
    if (PCA_CHIP_TYPE(chip.driver_data) == TCA6418_TYPE)
    return regmap_update_bits(chip.regmap, dirreg, bit, 0);
    return regmap_update_bits(chip.regmap, dirreg, bit, bit);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_gpio_direction_input(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int pca953x_gpio_direction_input(struct gpio_chip *gc, unsigned int off)
    {
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    guard(mutex)(&chip.i2c_lock);
    return pca953x_gpio_direction_input_unlocked(gc, off);
    }
    static int pca953x_gpio_direction_output(struct gpio_chip *gc,
    unsigned off, int val)
    {
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    let mut dirreg: u8 = chip.recalc_addr(chip, chip.regs.direction, off);
    let mut outreg: u8 = chip.recalc_addr(chip, chip.regs.output, off);
    let mut bit: u8 = pca953x_get_bit_mask(chip, off);
    int ret;
    guard(mutex)(&chip.i2c_lock);
// set output level
    ret = regmap_update_bits(chip.regmap, outreg, bit, val ? bit : 0);
    if (ret)
    return ret;
//
// then direction
// (in/out logic is inverted on TCA6418)
//
    if (PCA_CHIP_TYPE(chip.driver_data) == TCA6418_TYPE)
    return regmap_update_bits(chip.regmap, dirreg, bit, bit);
    return regmap_update_bits(chip.regmap, dirreg, bit, 0);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_gpio_get_value(gc: *mut gpio_chip, off: unsigned) -> c_int {
    static int pca953x_gpio_get_value(struct gpio_chip *gc, unsigned off)
    {
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    let mut inreg: u8 = chip.recalc_addr(chip, chip.regs.input, off);
    let mut bit: u8 = pca953x_get_bit_mask(chip, off);
    u32 reg_val;
    int ret;
    scoped_guard(mutex, &chip.i2c_lock)
    ret = regmap_read(chip.regmap, inreg, &reg_val);
    if (ret < 0)
    return ret;
    return !!(reg_val & bit);
    }
    static int pca953x_gpio_set_value(struct gpio_chip *gc, unsigned int off,
    int val)
    {
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    let mut outreg: u8 = chip.recalc_addr(chip, chip.regs.output, off);
    let mut bit: u8 = pca953x_get_bit_mask(chip, off);
    guard(mutex)(&chip.i2c_lock);
    return regmap_update_bits(chip.regmap, outreg, bit, val ? bit : 0);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_gpio_get_direction(gc: *mut gpio_chip, off: unsigned) -> c_int {
    static int pca953x_gpio_get_direction(struct gpio_chip *gc, unsigned off)
    {
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    let mut dirreg: u8 = chip.recalc_addr(chip, chip.regs.direction, off);
    let mut bit: u8 = pca953x_get_bit_mask(chip, off);
    u32 reg_val;
    int ret;
    scoped_guard(mutex, &chip.i2c_lock)
    ret = regmap_read(chip.regmap, dirreg, &reg_val);
    if (ret < 0)
    return ret;
// (in/out logic is inverted on TCA6418)
    if (reg_val & bit) {
    if (PCA_CHIP_TYPE(chip.driver_data) == TCA6418_TYPE)
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
    if (PCA_CHIP_TYPE(chip.driver_data) == TCA6418_TYPE)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int pca953x_gpio_get_multiple(struct gpio_chip *gc,
    unsigned long *mask, unsigned long *bits)
    {
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    DECLARE_BITMAP(reg_val, MAX_LINE);
    int ret;
    scoped_guard(mutex, &chip.i2c_lock)
    ret = pca953x_read_regs(chip, chip.regs.input, reg_val);
    if (ret)
    return ret;
    bitmap_replace(bits, bits, reg_val, mask, gc.ngpio);
    return 0;
    }
    static int pca953x_gpio_set_multiple(struct gpio_chip *gc,
    unsigned long *mask, unsigned long *bits)
    {
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    DECLARE_BITMAP(reg_val, MAX_LINE);
    int ret;
    guard(mutex)(&chip.i2c_lock);
    ret = pca953x_read_regs(chip, chip.regs.output, reg_val);
    if (ret)
    return ret;
    bitmap_replace(reg_val, reg_val, bits, mask, gc.ngpio);
    return pca953x_write_regs(chip, chip.regs.output, reg_val);
    }
    static int pca953x_gpio_set_pull_up_down(struct pca953x_chip *chip,
    unsigned int offset,
    unsigned long config)
    {
    let mut param: enum pin_config_param = pinconf_to_config_param(config);
    let mut pull_en_reg: u8 = chip.recalc_addr(chip, PCAL953X_PULL_EN, offset);
    let mut pull_sel_reg: u8 = chip.recalc_addr(chip, PCAL953X_PULL_SEL, offset);
    let mut bit: u8 = BIT(offset % BANK_SZ);
    int ret;
//
// pull-up/pull-down configuration requires PCAL extended
// registers
//
    if (!(chip.driver_data & PCA_PCAL))
    return -ENOTSUPP;
    guard(mutex)(&chip.i2c_lock);
// Configure pull-up/pull-down
    if (param == PIN_CONFIG_BIAS_PULL_UP)
    ret = regmap_update_bits(chip.regmap, pull_sel_reg, bit, bit);
#[no_mangle]
pub unsafe extern "C" fn if(PIN_CONFIG_BIAS_PULL_DOWN: param ==) -> else {
    else if (param == PIN_CONFIG_BIAS_PULL_DOWN)
    ret = regmap_update_bits(chip.regmap, pull_sel_reg, bit, 0);
    else
    ret = 0;
    if (ret)
    return ret;
// Disable/Enable pull-up/pull-down
    if (param == PIN_CONFIG_BIAS_DISABLE)
    return regmap_update_bits(chip.regmap, pull_en_reg, bit, 0);
    else
    return regmap_update_bits(chip.regmap, pull_en_reg, bit, bit);
    }
    static int pca953x_gpio_set_config(struct gpio_chip *gc, unsigned int offset,
    unsigned long config)
    {
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_BIAS_PULL_UP:
    case PIN_CONFIG_BIAS_PULL_PIN_DEFAULT:
    case PIN_CONFIG_BIAS_PULL_DOWN:
    case PIN_CONFIG_BIAS_DISABLE:
    return pca953x_gpio_set_pull_up_down(chip, offset, config);
    default:
    return -ENOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn pca953x_setup_gpio(chip: *mut pca953x_chip, gpios: c_int) {
    static void pca953x_setup_gpio(struct pca953x_chip *chip, int gpios)
    {
    struct gpio_chip *gc = &chip.gpio_chip;
    gc.direction_input  = pca953x_gpio_direction_input;
    gc.direction_output = pca953x_gpio_direction_output;
    gc.get = pca953x_gpio_get_value;
    gc.set = pca953x_gpio_set_value;
    gc.get_direction = pca953x_gpio_get_direction;
    gc.get_multiple = pca953x_gpio_get_multiple;
    gc.set_multiple = pca953x_gpio_set_multiple;
    gc.set_config = pca953x_gpio_set_config;
    gc.can_sleep = true;
    gc.base = chip.gpio_start;
    gc.ngpio = gpios;
    gc.label = dev_name(&chip.client.dev);
    gc.parent = &chip.client.dev;
    gc.owner = THIS_MODULE;
    }

#[no_mangle]
unsafe extern "C" fn pca953x_irq_mask(d: *mut irq_data) {
    static void pca953x_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    clear_bit(hwirq, chip.irq_mask);
    gpiochip_disable_irq(gc, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_irq_unmask(d: *mut irq_data) {
    static void pca953x_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    gpiochip_enable_irq(gc, hwirq);
    set_bit(hwirq, chip.irq_mask);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_irq_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    static int pca953x_irq_set_wake(struct irq_data *d, unsigned int on)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    if (on)
    atomic_inc(&chip.wakeup_path);
    else
    atomic_dec(&chip.wakeup_path);
    return irq_set_irq_wake(chip.client.irq, on);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_irq_bus_lock(d: *mut irq_data) {
    static void pca953x_irq_bus_lock(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    mutex_lock(&chip.irq_lock);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_irq_bus_sync_unlock(d: *mut irq_data) {
    static void pca953x_irq_bus_sync_unlock(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    DECLARE_BITMAP(irq_mask, MAX_LINE);
    DECLARE_BITMAP(reg_direction, MAX_LINE);
    int level;
    guard(mutex)(&chip.i2c_lock);
    if (chip.driver_data & PCA_PCAL) {
    DECLARE_BITMAP(latched_inputs, MAX_LINE);
// Enable latch on edge-triggered interrupt-enabled inputs
    bitmap_or(latched_inputs, chip.irq_trig_fall, chip.irq_trig_raise, gc.ngpio);
    bitmap_and(latched_inputs, latched_inputs, chip.irq_mask, gc.ngpio);
    pca953x_write_regs(chip, PCAL953X_IN_LATCH, latched_inputs);
    bitmap_complement(irq_mask, chip.irq_mask, gc.ngpio);
// Unmask enabled interrupts
    pca953x_write_regs(chip, PCAL953X_INT_MASK, irq_mask);
    }
// Switch direction to input if needed
    pca953x_read_regs(chip, chip.regs.direction, reg_direction);
    bitmap_or(irq_mask, chip.irq_trig_fall, chip.irq_trig_raise, gc.ngpio);
    bitmap_or(irq_mask, irq_mask, chip.irq_trig_level_high, gc.ngpio);
    bitmap_or(irq_mask, irq_mask, chip.irq_trig_level_low, gc.ngpio);
// Look for any newly setup interrupt
    for_each_andnot_bit(level, irq_mask, reg_direction, gc.ngpio)
    pca953x_gpio_direction_input_unlocked(&chip.gpio_chip, level);
    mutex_unlock(&chip.irq_lock);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int pca953x_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    struct device *dev = &chip.client.dev;
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    if (!(type & IRQ_TYPE_SENSE_MASK)) {
    dev_err(dev, "irq %d: unsupported type %d\n", d.irq, type);
    return -EINVAL;
    }
    assign_bit(hwirq, chip.irq_trig_fall, type & IRQ_TYPE_EDGE_FALLING);
    assign_bit(hwirq, chip.irq_trig_raise, type & IRQ_TYPE_EDGE_RISING);
    assign_bit(hwirq, chip.irq_trig_level_low, type & IRQ_TYPE_LEVEL_LOW);
    assign_bit(hwirq, chip.irq_trig_level_high, type & IRQ_TYPE_LEVEL_HIGH);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca953x_irq_shutdown(d: *mut irq_data) {
    static void pca953x_irq_shutdown(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct pca953x_chip *chip = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    clear_bit(hwirq, chip.irq_trig_raise);
    clear_bit(hwirq, chip.irq_trig_fall);
    clear_bit(hwirq, chip.irq_trig_level_low);
    clear_bit(hwirq, chip.irq_trig_level_high);
    pca953x_irq_mask(d);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_irq_print_chip(data: *mut irq_data, p: *mut seq_file) {
    static void pca953x_irq_print_chip(struct irq_data *data, struct seq_file *p)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(data);
    seq_puts(p, dev_name(gc.parent));
    }
    static const struct irq_chip pca953x_irq_chip = {
    .irq_mask		= pca953x_irq_mask,
    .irq_unmask		= pca953x_irq_unmask,
    .irq_set_wake		= pca953x_irq_set_wake,
    .irq_bus_lock		= pca953x_irq_bus_lock,
    .irq_bus_sync_unlock	= pca953x_irq_bus_sync_unlock,
    .irq_set_type		= pca953x_irq_set_type,
    .irq_shutdown		= pca953x_irq_shutdown,
    .irq_print_chip		= pca953x_irq_print_chip,
    .flags			= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn pca953x_irq_pending(chip: *mut pca953x_chip, pending: *mut c_ulong) -> bool {
    static bool pca953x_irq_pending(struct pca953x_chip *chip, unsigned long *pending)
    {
    struct gpio_chip *gc = &chip.gpio_chip;
    DECLARE_BITMAP(reg_direction, MAX_LINE);
    DECLARE_BITMAP(old_stat, MAX_LINE);
    DECLARE_BITMAP(cur_stat, MAX_LINE);
    DECLARE_BITMAP(new_stat, MAX_LINE);
    DECLARE_BITMAP(int_stat, MAX_LINE);
    DECLARE_BITMAP(trigger, MAX_LINE);
    DECLARE_BITMAP(edges, MAX_LINE);
    int ret;
    if (chip.driver_data & PCA_PCAL) {
// Read INT_STAT before it is cleared by the input-port read.
    ret = pca953x_read_regs(chip, PCAL953X_INT_STAT, int_stat);
    if (ret)
    return false;
    }
    ret = pca953x_read_regs(chip, chip.regs.input, cur_stat);
    if (ret)
    return false;
    if (chip.driver_data & PCA_PCAL) {
// Detect short pulses via INT_STAT.
    bitmap_and(trigger, int_stat, chip.irq_mask, gc.ngpio);
// Apply filter for rising/falling edge selection.
    bitmap_replace(new_stat, chip.irq_trig_fall, chip.irq_trig_raise,
    cur_stat, gc.ngpio);
    bitmap_and(int_stat, new_stat, trigger, gc.ngpio);
    } else {
    bitmap_zero(int_stat, gc.ngpio);
    }
// Remove output pins from the equation
    pca953x_read_regs(chip, chip.regs.direction, reg_direction);
    bitmap_copy(old_stat, chip.irq_stat, gc.ngpio);
    bitmap_and(new_stat, cur_stat, reg_direction, gc.ngpio);
    bitmap_xor(cur_stat, new_stat, old_stat, gc.ngpio);
    bitmap_and(trigger, cur_stat, chip.irq_mask, gc.ngpio);
    bitmap_copy(chip.irq_stat, new_stat, gc.ngpio);
    if (bitmap_empty(chip.irq_trig_level_high, gc.ngpio) &&
    bitmap_empty(chip.irq_trig_level_low, gc.ngpio)) {
    if (bitmap_empty(trigger, gc.ngpio) &&
    bitmap_empty(int_stat, gc.ngpio))
    return false;
    }
    bitmap_and(cur_stat, chip.irq_trig_fall, old_stat, gc.ngpio);
    bitmap_and(old_stat, chip.irq_trig_raise, new_stat, gc.ngpio);
    bitmap_or(edges, old_stat, cur_stat, gc.ngpio);
    bitmap_and(pending, edges, trigger, gc.ngpio);
    bitmap_or(pending, pending, int_stat, gc.ngpio);
    bitmap_and(cur_stat, new_stat, chip.irq_trig_level_high, gc.ngpio);
    bitmap_and(cur_stat, cur_stat, chip.irq_mask, gc.ngpio);
    bitmap_or(pending, pending, cur_stat, gc.ngpio);
    bitmap_andnot(cur_stat, reg_direction, new_stat, gc.ngpio);
    bitmap_and(old_stat, cur_stat, chip.irq_trig_level_low, gc.ngpio);
    bitmap_and(old_stat, old_stat, chip.irq_mask, gc.ngpio);
    bitmap_or(pending, pending, old_stat, gc.ngpio);
    return !bitmap_empty(pending, gc.ngpio);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_irq_handler(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t pca953x_irq_handler(int irq, void *devid)
    {
    struct pca953x_chip *chip = devid;
    struct gpio_chip *gc = &chip.gpio_chip;
    DECLARE_BITMAP(pending, MAX_LINE);
    int level;
    bool ret;
    bitmap_zero(pending, MAX_LINE);
    scoped_guard(mutex, &chip.i2c_lock)
    ret = pca953x_irq_pending(chip, pending);
    if (ret) {
    ret = 0;
    for_each_set_bit(level, pending, gc.ngpio) {
    let mut nested_irq: c_int = irq_find_mapping(gc.irq.domain, level);
    if (unlikely(nested_irq <= 0)) {
    dev_warn_ratelimited(gc.parent, "unmapped interrupt %d\n", level);
    continue;
    }
    handle_nested_irq(nested_irq);
    ret = 1;
    }
    }
    return IRQ_RETVAL(ret);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_irq_setup(chip: *mut pca953x_chip, irq_base: c_int) -> c_int {
    static int pca953x_irq_setup(struct pca953x_chip *chip, int irq_base)
    {
    struct i2c_client *client = chip.client;
    struct device *dev = &client.dev;
    DECLARE_BITMAP(reg_direction, MAX_LINE);
    DECLARE_BITMAP(irq_stat, MAX_LINE);
    struct gpio_chip *gc = &chip.gpio_chip;
    struct gpio_irq_chip *girq;
    int ret;
    if (dmi_first_match(pca953x_dmi_acpi_irq_info)) {
    ret = pca953x_acpi_get_irq(dev);
    if (ret > 0)
    client.irq = ret;
    }
    if (!client.irq)
    return 0;
    if (irq_base == -1)
    return 0;
    if (!(chip.driver_data & PCA_INT))
    return 0;
    ret = pca953x_read_regs(chip, chip.regs.input, irq_stat);
    if (ret)
    return ret;
//
// There is no way to know which GPIO line generated the
// interrupt.  We have to rely on the previous read for
// this purpose.
//
    pca953x_read_regs(chip, chip.regs.direction, reg_direction);
    bitmap_and(chip.irq_stat, irq_stat, reg_direction, gc.ngpio);
    mutex_init(&chip.irq_lock);
    girq = &chip.gpio_chip.irq;
    gpio_irq_chip_set_chip(girq, &pca953x_irq_chip);
// This will let us handle the parent IRQ in the driver
    girq.parent_handler = core::ptr::null_mut();
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_simple_irq;
    girq.threaded = true;
    girq.first = irq_base; /* FIXME: get rid of this */
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(), pca953x_irq_handler,
    IRQF_ONESHOT | IRQF_SHARED, dev_name(dev),
    chip);
    if (ret)
    return dev_err_probe(dev, ret, "failed to request irq\n");
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pca953x_irq_setup(chip: *mut pca953x_chip, irq_base: c_int) -> c_int {
    static int pca953x_irq_setup(struct pca953x_chip *chip, int irq_base)
    {
    struct i2c_client *client = chip.client;
    struct device *dev = &client.dev;
    if (client.irq && irq_base != -1 && (chip.driver_data & PCA_INT))
    dev_warn(dev, "interrupt support not compiled in\n");
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn device_pca95xx_init(chip: *mut pca953x_chip) -> c_int {
    static int device_pca95xx_init(struct pca953x_chip *chip)
    {
    DECLARE_BITMAP(val, MAX_LINE);
    u8 regaddr;
    int ret;
    regaddr = chip.recalc_addr(chip, chip.regs.output, 0);
    ret = regcache_sync_region(chip.regmap, regaddr,
    regaddr + NBANK(chip) - 1);
    if (ret)
    return ret;
    regaddr = chip.recalc_addr(chip, chip.regs.direction, 0);
    ret = regcache_sync_region(chip.regmap, regaddr,
    regaddr + NBANK(chip) - 1);
    if (ret)
    return ret;
// clear polarity inversion
    bitmap_zero(val, MAX_LINE);
    return pca953x_write_regs(chip, chip.regs.invert, val);
    }
#[no_mangle]
unsafe extern "C" fn device_pca957x_init(chip: *mut pca953x_chip) -> c_int {
    static int device_pca957x_init(struct pca953x_chip *chip)
    {
    DECLARE_BITMAP(val, MAX_LINE);
    unsigned int i;
    int ret;
    ret = device_pca95xx_init(chip);
    if (ret)
    return ret;
// To enable register 6, 7 to control pull up and pull down
    for (i = 0; i < NBANK(chip); i++)
    bitmap_set_value8(val, 0x02, i * BANK_SZ);
    return pca953x_write_regs(chip, PCA957X_BKEN, val);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_disable_regulator(reg: *mut c_void) {
    static void pca953x_disable_regulator(void *reg)
    {
    regulator_disable(reg);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_get_and_enable_regulator(chip: *mut pca953x_chip) -> c_int {
    static int pca953x_get_and_enable_regulator(struct pca953x_chip *chip)
    {
    struct device *dev = &chip.client.dev;
    struct regulator *reg = chip.regulator;
    int ret;
    reg = devm_regulator_get(dev, "vcc");
    if (IS_ERR(reg))
    return dev_err_probe(dev, PTR_ERR(reg), "reg get err\n");
    ret = regulator_enable(reg);
    if (ret)
    return dev_err_probe(dev, ret, "reg en err\n");
    ret = devm_add_action_or_reset(dev, pca953x_disable_regulator, reg);
    if (ret)
    return ret;
    chip.regulator = reg;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca953x_probe(client: *mut i2c_client) -> c_int {
    static int pca953x_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct pca953x_platform_data *pdata;
    struct pca953x_chip *chip;
    int irq_base;
    int ret;
    const struct regmap_config *regmap_config;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (chip == core::ptr::null_mut())
    return -ENOMEM;
    pdata = dev_get_platdata(dev);
    if (pdata) {
    irq_base = pdata.irq_base;
    chip.gpio_start = pdata.gpio_base;
    } else {
    struct gpio_desc *reset_gpio;
    chip.gpio_start = -1;
    irq_base = 0;
//
// See if we need to de-assert a reset pin.
//
// There is no known ACPI-enabled platforms that are
// using "reset" GPIO. Otherwise any of those platform
// must use _DSD method with corresponding property.
//
    reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(reset_gpio))
    return dev_err_probe(dev, PTR_ERR(reset_gpio),
    "Failed to get reset gpio\n");
    }
    chip.client = client;
    chip.driver_data = (uintptr_t)i2c_get_match_data(client);
    if (!chip.driver_data)
    return -ENODEV;
    ret = pca953x_get_and_enable_regulator(chip);
    if (ret)
    return ret;
    i2c_set_clientdata(client, chip);
    pca953x_setup_gpio(chip, chip.driver_data & PCA_GPIO_MASK);
    if (NBANK(chip) > 2 || PCA_CHIP_TYPE(chip.driver_data) == PCA957X_TYPE) {
    dev_info(dev, "using auto increment\n");
    regmap_config = &pca953x_ai_i2c_regmap;
    } else {
    dev_info(dev, "using no auto increment\n");
    regmap_config = &pca953x_i2c_regmap;
    }
    switch (PCA_CHIP_TYPE(chip.driver_data)) {
    case PCAL653X_TYPE:
    chip.recalc_addr = pcal6534_recalc_addr;
    chip.check_reg = pcal6534_check_register;
    break;
    case TCA6418_TYPE:
    chip.recalc_addr = tca6418_recalc_addr;
//
// We don't assign chip->check_reg = tca6418_check_register directly here.
// Instead, the wrappers handle the dispatch based on PCA_CHIP_TYPE.
//
    break;
    default:
    chip.recalc_addr = pca953x_recalc_addr;
    chip.check_reg = pca953x_check_register;
    break;
    }
    chip.regmap = devm_regmap_init_i2c(client, regmap_config);
    if (IS_ERR(chip.regmap))
    return PTR_ERR(chip.regmap);
    regcache_mark_dirty(chip.regmap);
    mutex_init(&chip.i2c_lock);
//
// In case we have an i2c-mux controlled by a GPIO provided by an
// expander using the same driver higher on the device tree, read the
// i2c adapter nesting depth and use the retrieved value as lockdep
// subclass for chip->i2c_lock.
//
// REVISIT: This solution is not complete. It protects us from lockdep
// false positives when the expander controlling the i2c-mux is on
// a different level on the device tree, but not when it's on the same
// level on a different branch (in which case the subclass number
// would be the same).
//
// TODO: Once a correct solution is developed, a similar fix should be
// applied to all other i2c-controlled GPIO expanders (and potentially
// regmap-i2c).
//
    lockdep_set_subclass(&chip.i2c_lock,
    i2c_adapter_depth(client.adapter));
//
// initialize cached registers from their original values.
// we can't share this chip with another i2c master.
//
    switch (PCA_CHIP_TYPE(chip.driver_data)) {
    case PCA957X_TYPE:
    chip.regs = &pca957x_regs;
    ret = device_pca957x_init(chip);
    break;
    case TCA6418_TYPE:
    chip.regs = &tca6418_regs;
    break;
    default:
    chip.regs = &pca953x_regs;
    ret = device_pca95xx_init(chip);
    break;
    }
    if (ret)
    return ret;
    ret = pca953x_irq_setup(chip, irq_base);
    if (ret)
    return ret;
    return devm_gpiochip_add_data(dev, &chip.gpio_chip, chip);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_regcache_sync(chip: *mut pca953x_chip) -> c_int {
    static int pca953x_regcache_sync(struct pca953x_chip *chip)
    {
    struct device *dev = &chip.client.dev;
    int ret;
    u8 regaddr;
//
// The ordering between direction and output is important,
// sync these registers first and only then sync the rest.
//
    regaddr = chip.recalc_addr(chip, chip.regs.direction, 0);
    ret = regcache_sync_region(chip.regmap, regaddr, regaddr + NBANK(chip) - 1);
    if (ret) {
    dev_err(dev, "Failed to sync GPIO dir registers: %d\n", ret);
    return ret;
    }
    regaddr = chip.recalc_addr(chip, chip.regs.output, 0);
    ret = regcache_sync_region(chip.regmap, regaddr, regaddr + NBANK(chip) - 1);
    if (ret) {
    dev_err(dev, "Failed to sync GPIO out registers: %d\n", ret);
    return ret;
    }

    if (chip.driver_data & PCA_PCAL) {
    regaddr = chip.recalc_addr(chip, PCAL953X_IN_LATCH, 0);
    ret = regcache_sync_region(chip.regmap, regaddr,
    regaddr + NBANK(chip) - 1);
    if (ret) {
    dev_err(dev, "Failed to sync INT latch registers: %d\n",
    ret);
    return ret;
    }
    regaddr = chip.recalc_addr(chip, PCAL953X_INT_MASK, 0);
    ret = regcache_sync_region(chip.regmap, regaddr,
    regaddr + NBANK(chip) - 1);
    if (ret) {
    dev_err(dev, "Failed to sync INT mask registers: %d\n",
    ret);
    return ret;
    }
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca953x_restore_context(chip: *mut pca953x_chip) -> c_int {
    static int pca953x_restore_context(struct pca953x_chip *chip)
    {
    int ret;
    guard(mutex)(&chip.i2c_lock);
    if (chip.client.irq > 0)
    enable_irq(chip.client.irq);
    regcache_cache_only(chip.regmap, false);
    regcache_mark_dirty(chip.regmap);
    ret = pca953x_regcache_sync(chip);
    if (ret)
    goto err;
    ret = regcache_sync(chip.regmap);
    if (ret)
    goto err;
    return 0;
    err:
    if (chip.client.irq > 0)
    disable_irq(chip.client.irq);
    regcache_cache_only(chip.regmap, true);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pca953x_save_context(chip: *mut pca953x_chip) {
    static void pca953x_save_context(struct pca953x_chip *chip)
    {
    guard(mutex)(&chip.i2c_lock);
// Disable IRQ to prevent early triggering while regmap "cache only" is on
    if (chip.client.irq > 0)
    disable_irq(chip.client.irq);
    regcache_cache_only(chip.regmap, true);
    }
#[no_mangle]
unsafe extern "C" fn pca953x_suspend(dev: *mut device) -> c_int {
    static int pca953x_suspend(struct device *dev)
    {
    struct pca953x_chip *chip = dev_get_drvdata(dev);
    pca953x_save_context(chip);
    if (atomic_read(&chip.wakeup_path))
    device_set_wakeup_path(dev);
    else
    regulator_disable(chip.regulator);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca953x_resume(dev: *mut device) -> c_int {
    static int pca953x_resume(struct device *dev)
    {
    struct pca953x_chip *chip = dev_get_drvdata(dev);
    int ret;
    if (!atomic_read(&chip.wakeup_path)) {
    ret = regulator_enable(chip.regulator);
    if (ret) {
    dev_err(dev, "Failed to enable regulator: %d\n", ret);
    return ret;
    }
    }
    ret = pca953x_restore_context(chip);
    if (ret)
    dev_err(dev, "Failed to restore register map: %d\n", ret);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pca953x_pm_ops, pca953x_suspend, pca953x_resume);
// convenience to stop overlong match-table lines

    static const struct of_device_id pca953x_dt_ids[] = {
    { .compatible = "nxp,pca6408", .data = OF_953X(8, PCA_INT), },
    { .compatible = "nxp,pca6416", .data = OF_953X(16, PCA_INT), },
    { .compatible = "nxp,pca9505", .data = OF_953X(40, PCA_INT), },
    { .compatible = "nxp,pca9506", .data = OF_953X(40, PCA_INT), },
    { .compatible = "nxp,pca9534", .data = OF_953X( 8, PCA_INT), },
    { .compatible = "nxp,pca9535", .data = OF_953X(16, PCA_INT), },
    { .compatible = "nxp,pca9536", .data = OF_953X( 4, 0), },
    { .compatible = "nxp,pca9537", .data = OF_953X( 4, PCA_INT), },
    { .compatible = "nxp,pca9538", .data = OF_953X( 8, PCA_INT), },
    { .compatible = "nxp,pca9539", .data = OF_953X(16, PCA_INT), },
    { .compatible = "nxp,pca9554", .data = OF_953X( 8, PCA_INT), },
    { .compatible = "nxp,pca9555", .data = OF_953X(16, PCA_INT), },
    { .compatible = "nxp,pca9556", .data = OF_953X( 8, 0), },
    { .compatible = "nxp,pca9557", .data = OF_953X( 8, 0), },
    { .compatible = "nxp,pca9574", .data = OF_957X( 8, PCA_INT), },
    { .compatible = "nxp,pca9575", .data = OF_957X(16, PCA_INT), },
    { .compatible = "nxp,pca9698", .data = OF_953X(40, 0), },
    { .compatible = "nxp,pcal6408", .data = OF_953X(8, PCA_LATCH_INT), },
    { .compatible = "nxp,pcal6416", .data = OF_953X(16, PCA_LATCH_INT), },
    { .compatible = "nxp,pcal6524", .data = OF_953X(24, PCA_LATCH_INT), },
    { .compatible = "nxp,pcal6534", .data = OF_653X(34, PCA_LATCH_INT), },
    { .compatible = "nxp,pcal9535", .data = OF_953X(16, PCA_LATCH_INT), },
    { .compatible = "nxp,pcal9554b", .data = OF_953X( 8, PCA_LATCH_INT), },
    { .compatible = "nxp,pcal9555a", .data = OF_953X(16, PCA_LATCH_INT), },
    { .compatible = "maxim,max7310", .data = OF_953X( 8, 0), },
    { .compatible = "maxim,max7312", .data = OF_953X(16, PCA_INT), },
    { .compatible = "maxim,max7313", .data = OF_953X(16, PCA_INT), },
    { .compatible = "maxim,max7315", .data = OF_953X( 8, PCA_INT), },
    { .compatible = "maxim,max7318", .data = OF_953X(16, PCA_INT), },
    { .compatible = "ti,pca6107", .data = OF_953X( 8, PCA_INT), },
    { .compatible = "ti,pca9536", .data = OF_953X( 4, 0), },
    { .compatible = "ti,tca6408", .data = OF_953X( 8, PCA_INT), },
    { .compatible = "ti,tca6416", .data = OF_953X(16, PCA_INT), },
    { .compatible = "ti,tca6418", .data = (void *)(18 | TCA6418_TYPE | PCA_INT), },
    { .compatible = "ti,tca6424", .data = OF_953X(24, PCA_INT), },
    { .compatible = "ti,tca9535", .data = OF_953X(16, PCA_INT), },
    { .compatible = "ti,tca9538", .data = OF_953X( 8, PCA_INT), },
    { .compatible = "ti,tca9539", .data = OF_953X(16, PCA_INT), },
    { .compatible = "ti,tcal6408", .data = OF_953X( 8, PCA_LATCH_INT), },
    { .compatible = "ti,tcal6416", .data = OF_953X(16, PCA_LATCH_INT), },
    { .compatible = "onnn,cat9554", .data = OF_953X( 8, PCA_INT), },
    { .compatible = "onnn,pca9654", .data = OF_953X( 8, PCA_INT), },
    { .compatible = "onnn,pca9655", .data = OF_953X(16, PCA_INT), },
    { .compatible = "exar,xra1202", .data = OF_953X( 8, 0), },
    { }
    };
    MODULE_DEVICE_TABLE(of, pca953x_dt_ids);
    static struct i2c_driver pca953x_driver = {
    .driver = {
    .name	= "pca953x",
    .pm	= pm_sleep_ptr(&pca953x_pm_ops),
    .of_match_table = pca953x_dt_ids,
    .acpi_match_table = pca953x_acpi_ids,
    },
    .probe		= pca953x_probe,
    .id_table	= pca953x_id,
    };
#[no_mangle]
unsafe extern "C" fn pca953x_init() -> int __init {
    static int __init pca953x_init(void)
    {
    return i2c_add_driver(&pca953x_driver);
    }
//
// register after i2c postcore initcall and before
// subsys initcalls that may rely on these GPIOs
//
    subsys_initcall(pca953x_init);
#[no_mangle]
unsafe extern "C" fn pca953x_exit() -> void __exit {
    static void __exit pca953x_exit(void)
    {
    i2c_del_driver(&pca953x_driver);
    }
    module_exit(pca953x_exit);
    MODULE_AUTHOR("eric miao <eric.miao@marvell.com>");
    MODULE_DESCRIPTION("GPIO expander driver for PCA953x");
    MODULE_LICENSE("GPL");
