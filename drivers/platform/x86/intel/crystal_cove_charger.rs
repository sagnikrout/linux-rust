//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/crystal_cove_charger.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Driver for the external-charger IRQ pass-through function of the
// Intel Bay Trail Crystal Cove PMIC.
//
// Note this is NOT a power_supply class driver, it just deals with IRQ
// pass-through, this requires a separate driver because the PMIC's
// level 2 interrupt for this must be explicitly acked.
//

pub const CHGRIRQ_REG: c_uint = 0x0a;
pub const MCHGRIRQ_REG: c_uint = 0x17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crystal_cove_charger_data {
    pub /: *mut *mut mutex buslock; / irq_bus_lock,
    pub irqchip: irq_chip,
    pub regmap: *mut regmap,
    pub irq_domain: *mut irq_domain,
    pub irq: c_int,
    pub charger_irq: c_int,
    pub mask: u8,
    pub new_mask: u8,
}

#[no_mangle]
unsafe extern "C" fn crystal_cove_charger_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t crystal_cove_charger_irq(int irq, void *data)
    {
    struct crystal_cove_charger_data *charger = data;
// No need to read CHGRIRQ_REG as there is only 1 IRQ
    handle_nested_irq(charger.charger_irq);
// Ack CHGRIRQ 0
    regmap_write(charger.regmap, CHGRIRQ_REG, BIT(0));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_charger_irq_bus_lock(data: *mut irq_data) {
    static void crystal_cove_charger_irq_bus_lock(struct irq_data *data)
    {
    struct crystal_cove_charger_data *charger = irq_data_get_irq_chip_data(data);
    mutex_lock(&charger.buslock);
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_charger_irq_bus_sync_unlock(data: *mut irq_data) {
    static void crystal_cove_charger_irq_bus_sync_unlock(struct irq_data *data)
    {
    struct crystal_cove_charger_data *charger = irq_data_get_irq_chip_data(data);
    if (charger.mask != charger.new_mask) {
    regmap_write(charger.regmap, MCHGRIRQ_REG, charger.new_mask);
    charger.mask = charger.new_mask;
    }
    mutex_unlock(&charger.buslock);
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_charger_irq_unmask(data: *mut irq_data) {
    static void crystal_cove_charger_irq_unmask(struct irq_data *data)
    {
    struct crystal_cove_charger_data *charger = irq_data_get_irq_chip_data(data);
    charger.new_mask &= ~BIT(data.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_charger_irq_mask(data: *mut irq_data) {
    static void crystal_cove_charger_irq_mask(struct irq_data *data)
    {
    struct crystal_cove_charger_data *charger = irq_data_get_irq_chip_data(data);
    charger.new_mask |= BIT(data.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_charger_rm_irq_domain(data: *mut c_void) {
    static void crystal_cove_charger_rm_irq_domain(void *data)
    {
    struct crystal_cove_charger_data *charger = data;
    irq_domain_remove(charger.irq_domain);
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_charger_probe(pdev: *mut platform_device) -> c_int {
    static int crystal_cove_charger_probe(struct platform_device *pdev)
    {
    struct intel_soc_pmic *pmic = dev_get_drvdata(pdev.dev.parent);
    struct crystal_cove_charger_data *charger;
    int ret;
    charger = devm_kzalloc(&pdev.dev, sizeof(*charger), GFP_KERNEL);
    if (!charger)
    return -ENOMEM;
    charger.regmap = pmic.regmap;
    mutex_init(&charger.buslock);
    charger.irq = platform_get_irq(pdev, 0);
    if (charger.irq < 0)
    return charger.irq;
    charger.irq_domain = irq_domain_create_linear(dev_fwnode(pdev.dev.parent), 1,
    &irq_domain_simple_ops, core::ptr::null_mut());
    if (!charger.irq_domain)
    return -ENOMEM;
// Distuingish IRQ domain from others sharing (MFD) the same fwnode
    irq_domain_update_bus_token(charger.irq_domain, DOMAIN_BUS_WAKEUP);
    ret = devm_add_action_or_reset(&pdev.dev, crystal_cove_charger_rm_irq_domain, charger);
    if (ret)
    return ret;
    charger.charger_irq = irq_create_mapping(charger.irq_domain, 0);
    if (!charger.charger_irq)
    return -ENOMEM;
    charger.irqchip.name = KBUILD_MODNAME;
    charger.irqchip.irq_unmask = crystal_cove_charger_irq_unmask;
    charger.irqchip.irq_mask = crystal_cove_charger_irq_mask;
    charger.irqchip.irq_bus_lock = crystal_cove_charger_irq_bus_lock;
    charger.irqchip.irq_bus_sync_unlock = crystal_cove_charger_irq_bus_sync_unlock;
    irq_set_chip_data(charger.charger_irq, charger);
    irq_set_chip_and_handler(charger.charger_irq, &charger.irqchip, handle_simple_irq);
    irq_set_nested_thread(charger.charger_irq, true);
    irq_set_noprobe(charger.charger_irq);
// Mask the single 2nd level IRQ before enabling the 1st level IRQ
    charger.mask = charger.new_mask = BIT(0);
    regmap_write(charger.regmap, MCHGRIRQ_REG, charger.mask);
    ret = devm_request_threaded_irq(&pdev.dev, charger.irq, core::ptr::null_mut(),
    crystal_cove_charger_irq,
    IRQF_ONESHOT, KBUILD_MODNAME, charger);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "requesting irq\n");
    return 0;
    }
    static struct platform_driver crystal_cove_charger_driver = {
    .probe = crystal_cove_charger_probe,
    .driver = {
    .name = "crystal_cove_charger",
    },
    };
    module_platform_driver(crystal_cove_charger_driver);
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com");
    MODULE_DESCRIPTION("Intel Bay Trail Crystal Cove external charger IRQ pass-through");
    MODULE_LICENSE("GPL");
