//! Automatically rewritten from C to Rust
//! Source: sound/soc/pxa/pxa2xx-ac97-lib.c
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
// Based on sound/arm/pxa2xx-ac97.c and sound/soc/pxa/pxa2xx-ac97.c
// which contain:
//
// Author:	Nicolas Pitre
// Created:	Dec 02, 2004
// Copyright:	MontaVista Software Inc.
//

    static DEFINE_MUTEX(car_mutex);
    static DECLARE_WAIT_QUEUE_HEAD(gsr_wq);
    static volatile long gsr_bits;
    static struct clk *ac97_clk;
    static struct clk *ac97conf_clk;
    struct gpio_desc *rst_gpio;
    static void __iomem *ac97_reg_base;
//
// Beware PXA27x bugs:
//
// o Slot 12 read from modem space will hang controller.
// o CDONE, SDONE interrupt fails after any slot 12 IO.
//
// We therefore have an hybrid approach for waiting on SDONE (interrupt or
// 1 jiffy timeout if interrupt never comes).
//
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_read(slot: c_int, reg: c_ushort) -> c_int {
    int pxa2xx_ac97_read(int slot, unsigned short reg)
    {
    let mut val: c_int = -ENODEV;
    u32 __iomem *reg_addr;
    if (slot > 0)
    return -ENODEV;
    guard(mutex)(&car_mutex);
// set up primary or secondary codec space
    if (cpu_is_pxa25x() && reg == AC97_GPIO_STATUS)
    reg_addr = ac97_reg_base +
    (slot ? SMC_REG_BASE : PMC_REG_BASE);
    else
    reg_addr = ac97_reg_base +
    (slot ? SAC_REG_BASE : PAC_REG_BASE);
    reg_addr += (reg >> 1);
// start read access across the ac97 link
    writel(GSR_CDONE | GSR_SDONE, ac97_reg_base + GSR);
    gsr_bits = 0;
    val = (readl(reg_addr) & 0xffff);
    if (reg == AC97_GPIO_STATUS)
    return val;
    if (wait_event_timeout(gsr_wq, (readl(ac97_reg_base + GSR) | gsr_bits) & GSR_SDONE, 1) <= 0 &&
    !((readl(ac97_reg_base + GSR) | gsr_bits) & GSR_SDONE)) {
    printk(KERN_ERR "%s: read error (ac97_reg=%d GSR=%#lx)\n",
    __func__, reg, readl(ac97_reg_base + GSR) | gsr_bits);
    return -ETIMEDOUT;
    }
// valid data now
    writel(GSR_CDONE | GSR_SDONE, ac97_reg_base + GSR);
    gsr_bits = 0;
    val = (readl(reg_addr) & 0xffff);
// but we've just started another cycle...
    wait_event_timeout(gsr_wq, (readl(ac97_reg_base + GSR) | gsr_bits) & GSR_SDONE, 1);
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_write(slot: c_int, reg: c_ushort, val: c_ushort) -> c_int {
    int pxa2xx_ac97_write(int slot, unsigned short reg, unsigned short val)
    {
    u32 __iomem *reg_addr;
    let mut ret: c_int = 0;
    guard(mutex)(&car_mutex);
// set up primary or secondary codec space
    if (cpu_is_pxa25x() && reg == AC97_GPIO_STATUS)
    reg_addr = ac97_reg_base +
    (slot ? SMC_REG_BASE : PMC_REG_BASE);
    else
    reg_addr = ac97_reg_base +
    (slot ? SAC_REG_BASE : PAC_REG_BASE);
    reg_addr += (reg >> 1);
    writel(GSR_CDONE | GSR_SDONE, ac97_reg_base + GSR);
    gsr_bits = 0;
    writel(val, reg_addr);
    if (wait_event_timeout(gsr_wq, (readl(ac97_reg_base + GSR) | gsr_bits) & GSR_CDONE, 1) <= 0 &&
    !((readl(ac97_reg_base + GSR) | gsr_bits) & GSR_CDONE)) {
    printk(KERN_ERR "%s: write error (ac97_reg=%d GSR=%#lx)\n",
    __func__, reg, readl(ac97_reg_base + GSR) | gsr_bits);
    ret = -EIO;
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn pxa_ac97_warm_pxa25x() {
    static inline void pxa_ac97_warm_pxa25x(void)
    {
    gsr_bits = 0;
    writel(readl(ac97_reg_base + GCR) | (GCR_WARM_RST), ac97_reg_base + GCR);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa_ac97_cold_pxa25x() {
    static inline void pxa_ac97_cold_pxa25x(void)
    {
    writel(readl(ac97_reg_base + GCR) & ( GCR_COLD_RST), ac97_reg_base + GCR);  /* clear everything but nCRST */
    writel(readl(ac97_reg_base + GCR) & (~GCR_COLD_RST), ac97_reg_base + GCR);  /* then assert nCRST */
    gsr_bits = 0;
    writel(GCR_COLD_RST, ac97_reg_base + GCR);
    }

#[no_mangle]
pub unsafe extern "C" fn pxa_ac97_warm_pxa27x() {
    static inline void pxa_ac97_warm_pxa27x(void)
    {
    gsr_bits = 0;
// warm reset broken on Bulverde, so manually keep AC97 reset high
    pxa27x_configure_ac97reset(rst_gpio, true);
    udelay(10);
    writel(readl(ac97_reg_base + GCR) | (GCR_WARM_RST), ac97_reg_base + GCR);
    pxa27x_configure_ac97reset(rst_gpio, false);
    udelay(500);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa_ac97_cold_pxa27x() {
    static inline void pxa_ac97_cold_pxa27x(void)
    {
    writel(readl(ac97_reg_base + GCR) & ( GCR_COLD_RST), ac97_reg_base + GCR);  /* clear everything but nCRST */
    writel(readl(ac97_reg_base + GCR) & (~GCR_COLD_RST), ac97_reg_base + GCR);  /* then assert nCRST */
    gsr_bits = 0;
// PXA27x Developers Manual section 13.5.2.2.1
    clk_prepare_enable(ac97conf_clk);
    udelay(5);
    clk_disable_unprepare(ac97conf_clk);
    writel(GCR_COLD_RST | GCR_WARM_RST, ac97_reg_base + GCR);
    }

#[no_mangle]
pub unsafe extern "C" fn pxa_ac97_warm_pxa3xx() {
    static inline void pxa_ac97_warm_pxa3xx(void)
    {
    gsr_bits = 0;
// Can't use interrupts
    writel(readl(ac97_reg_base + GCR) | (GCR_WARM_RST), ac97_reg_base + GCR);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa_ac97_cold_pxa3xx() {
    static inline void pxa_ac97_cold_pxa3xx(void)
    {
// Hold CLKBPB for 100us
    writel(0, ac97_reg_base + GCR);
    writel(GCR_CLKBPB, ac97_reg_base + GCR);
    udelay(100);
    writel(0, ac97_reg_base + GCR);
    writel(readl(ac97_reg_base + GCR) & ( GCR_COLD_RST), ac97_reg_base + GCR);  /* clear everything but nCRST */
    writel(readl(ac97_reg_base + GCR) & (~GCR_COLD_RST), ac97_reg_base + GCR);  /* then assert nCRST */
    gsr_bits = 0;
// Can't use interrupts on PXA3xx
    writel(readl(ac97_reg_base + GCR) & (~(GCR_PRIRDY_IEN|GCR_SECRDY_IEN)), ac97_reg_base + GCR);
    writel(GCR_WARM_RST | GCR_COLD_RST, ac97_reg_base + GCR);
    }

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_try_warm_reset() -> bool {
    bool pxa2xx_ac97_try_warm_reset(void)
    {
    unsigned long gsr;
    let mut timeout: c_uint = 100;

    if (cpu_is_pxa25x())
    pxa_ac97_warm_pxa25x();
    else

    if (cpu_is_pxa27x())
    pxa_ac97_warm_pxa27x();
    else

    if (cpu_is_pxa3xx())
    pxa_ac97_warm_pxa3xx();
    else

    snd_BUG();
    while (!((readl(ac97_reg_base + GSR) | gsr_bits) & (GSR_PCR | GSR_SCR)) && timeout--)
    mdelay(1);
    gsr = readl(ac97_reg_base + GSR) | gsr_bits;
    if (!(gsr & (GSR_PCR | GSR_SCR))) {
    printk(KERN_INFO "%s: warm reset timeout (GSR=%#lx)\n",
    __func__, gsr);
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_try_cold_reset() -> bool {
    bool pxa2xx_ac97_try_cold_reset(void)
    {
    unsigned long gsr;
    let mut timeout: c_uint = 1000;

    if (cpu_is_pxa25x())
    pxa_ac97_cold_pxa25x();
    else

    if (cpu_is_pxa27x())
    pxa_ac97_cold_pxa27x();
    else

    if (cpu_is_pxa3xx())
    pxa_ac97_cold_pxa3xx();
    else

    snd_BUG();
    while (!((readl(ac97_reg_base + GSR) | gsr_bits) & (GSR_PCR | GSR_SCR)) && timeout--)
    mdelay(1);
    gsr = readl(ac97_reg_base + GSR) | gsr_bits;
    if (!(gsr & (GSR_PCR | GSR_SCR))) {
    printk(KERN_INFO "%s: cold reset timeout (GSR=%#lx)\n",
    __func__, gsr);
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_finish_reset() {
    void pxa2xx_ac97_finish_reset(void)
    {
    let mut gcr: u32 = readl(ac97_reg_base + GCR);
    gcr &= ~(GCR_PRIRDY_IEN|GCR_SECRDY_IEN);
    gcr |= GCR_SDONE_IE|GCR_CDONE_IE;
    writel(gcr, ac97_reg_base + GCR);
    }
#[no_mangle]
unsafe extern "C" fn pxa2xx_ac97_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pxa2xx_ac97_irq(int irq, void *dev_id)
    {
    long status;
    status = readl(ac97_reg_base + GSR);
    if (status) {
    writel(status, ac97_reg_base + GSR);
    gsr_bits |= status;
    wake_up(&gsr_wq);
// Although we don't use those we still need to clear them
    since they tend to spuriously trigger when MMC is used
    (hardware bug? go figure)... */
    if (cpu_is_pxa27x()) {
    writel(MISR_EOC, ac97_reg_base + MISR);
    writel(PISR_EOC, ac97_reg_base + PISR);
    writel(MCSR_EOC, ac97_reg_base + MCSR);
    }
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_hw_suspend() -> c_int {
    int pxa2xx_ac97_hw_suspend(void)
    {
    writel(readl(ac97_reg_base + GCR) | (GCR_ACLINK_OFF), ac97_reg_base + GCR);
    clk_disable_unprepare(ac97_clk);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_hw_resume() -> c_int {
    int pxa2xx_ac97_hw_resume(void)
    {
    clk_prepare_enable(ac97_clk);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_hw_probe(dev: *mut platform_device) -> c_int {
    int pxa2xx_ac97_hw_probe(struct platform_device *dev)
    {
    int ret;
    int irq;
    ac97_reg_base = devm_platform_ioremap_resource(dev, 0);
    if (IS_ERR(ac97_reg_base))
    return PTR_ERR(ac97_reg_base);
    if (cpu_is_pxa27x()) {
// Assert reset using GPIOD_OUT_HIGH, because reset is GPIO_ACTIVE_LOW
    rst_gpio = devm_gpiod_get_optional(&dev.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(rst_gpio))
    return dev_err_probe(&dev.dev, PTR_ERR(rst_gpio),
    "reset gpio failed\n");
//
// This gpio is needed for a work-around to a bug in the ac97
// controller during warm reset.  The direction and level is set
// here so that it is an output driven high when switching from
// AC97_nRESET alt function to generic gpio.
//
    gpiod_set_consumer_name(rst_gpio, "pxa27x ac97 reset");
    pxa27x_configure_ac97reset(rst_gpio, false);
    ac97conf_clk = clk_get(&dev.dev, "AC97CONFCLK");
    if (IS_ERR(ac97conf_clk)) {
    ret = PTR_ERR(ac97conf_clk);
    ac97conf_clk = core::ptr::null_mut();
    goto err_conf;
    }
    }
    ac97_clk = clk_get(&dev.dev, "AC97CLK");
    if (IS_ERR(ac97_clk)) {
    ret = PTR_ERR(ac97_clk);
    ac97_clk = core::ptr::null_mut();
    goto err_clk;
    }
    ret = clk_prepare_enable(ac97_clk);
    if (ret)
    goto err_clk2;
    irq = platform_get_irq(dev, 0);
    if (irq < 0) {
    ret = irq;
    goto err_irq;
    }
    ret = request_irq(irq, pxa2xx_ac97_irq, 0, "AC97", core::ptr::null_mut());
    if (ret < 0)
    goto err_irq;
    return 0;
    err_irq:
    writel(readl(ac97_reg_base + GCR) | (GCR_ACLINK_OFF), ac97_reg_base + GCR);
    err_clk2:
    clk_put(ac97_clk);
    ac97_clk = core::ptr::null_mut();
    err_clk:
    if (ac97conf_clk) {
    clk_put(ac97conf_clk);
    ac97conf_clk = core::ptr::null_mut();
    }
    err_conf:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_hw_remove(dev: *mut platform_device) {
    void pxa2xx_ac97_hw_remove(struct platform_device *dev)
    {
    writel(readl(ac97_reg_base + GCR) | (GCR_ACLINK_OFF), ac97_reg_base + GCR);
    free_irq(platform_get_irq(dev, 0), core::ptr::null_mut());
    if (ac97conf_clk) {
    clk_put(ac97conf_clk);
    ac97conf_clk = core::ptr::null_mut();
    }
    clk_disable_unprepare(ac97_clk);
    clk_put(ac97_clk);
    ac97_clk = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_read_modr() -> u32 {
    u32 pxa2xx_ac97_read_modr(void)
    {
    if (!ac97_reg_base)
    return 0;
    return readl(ac97_reg_base + MODR);
    }
    EXPORT_SYMBOL_GPL(pxa2xx_ac97_read_modr);
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_read_misr() -> u32 {
    u32 pxa2xx_ac97_read_misr(void)
    {
    if (!ac97_reg_base)
    return 0;
    return readl(ac97_reg_base + MISR);
    }
    EXPORT_SYMBOL_GPL(pxa2xx_ac97_read_misr);
    MODULE_AUTHOR("Nicolas Pitre");
    MODULE_DESCRIPTION("Intel/Marvell PXA sound library");
    MODULE_LICENSE("GPL");
