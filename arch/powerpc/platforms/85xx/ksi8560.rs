//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/ksi8560.c
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


//
// Board setup routines for the Emerson KSI8560
//
// Author: Alexandr Smirnov <asmirnov@ru.mvista.com>
//
// Based on mpc85xx_ads.c maintained by Kumar Gala
//
// 2008 (c) MontaVista, Software, Inc.  This file is licensed under
// the terms of the GNU General Public License version 2.  This program
// is licensed "as is" without any warranty of any kind, whether express
// or implied.
//

pub const KSI8560_CPLD_HVR: c_uint = 0x04 /* Hardware Version Register */;
pub const KSI8560_CPLD_PVR: c_uint = 0x08 /* PLD Version Register */;
pub const KSI8560_CPLD_RCR1: c_uint = 0x30 /* Reset Command Register 1 */;
pub const KSI8560_CPLD_RCR1_CPUHR: c_uint = 0x80 /* CPU Hard Reset */;
    static void __iomem *cpld_base = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn machine_restart(cmd: *mut c_char) -> void __noreturn {
    static void __noreturn machine_restart(char *cmd)
    {
    if (cpld_base)
    out_8(cpld_base + KSI8560_CPLD_RCR1, KSI8560_CPLD_RCR1_CPUHR);
    else
    printk(KERN_ERR "Can't find CPLD base, hang forever\n");
    for (;;);
    }
#[no_mangle]
unsafe extern "C" fn ksi8560_pic_init() -> void __init {
    static void __init ksi8560_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    mpc85xx_cpm2_pic_init();
    }

//
// Setup I/O ports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_pin {
    pub flags: int port, pin,,
}

    static struct cpm_pin __initdata ksi8560_pins[] = {
// SCC1
    {3, 29, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {3, 30, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {3, 31, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
// SCC2
    {3, 26, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {3, 27, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {3, 28, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
// FCC1
    {0, 14, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {0, 15, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {0, 16, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {0, 17, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {0, 18, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {0, 19, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {0, 20, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {0, 21, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {0, 26, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
    {0, 27, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
    {0, 28, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {0, 29, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {0, 30, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
    {0, 31, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
    {2, 23, CPM_PIN_INPUT | CPM_PIN_PRIMARY}, /* CLK9 */
    {2, 22, CPM_PIN_INPUT | CPM_PIN_PRIMARY}, /* CLK10 */
    };
#[no_mangle]
unsafe extern "C" fn init_ioports() -> void __init {
    static void __init init_ioports(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(ksi8560_pins); i++) {
    struct cpm_pin *pin = &ksi8560_pins[i];
    cpm2_set_pin(pin.port, pin.pin, pin.flags);
    }
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_BRG1, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_BRG1, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_SCC2, CPM_BRG2, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_SCC2, CPM_BRG2, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK9, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK10, CPM_CLK_TX);
    }

//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn ksi8560_setup_arch() -> void __init {
    static void __init ksi8560_setup_arch(void)
    {
    struct device_node *cpld;
    cpld = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "emerson,KSI8560-cpld");
    if (cpld)
    cpld_base = of_iomap(cpld, 0);
    else
    printk(KERN_ERR "Can't find CPLD in device tree\n");
    of_node_put(cpld);
    if (ppc_md.progress)
    ppc_md.progress("ksi8560_setup_arch()", 0);

    cpm2_reset();
    init_ioports();

    }
#[no_mangle]
unsafe extern "C" fn ksi8560_show_cpuinfo(m: *mut seq_file) {
    static void ksi8560_show_cpuinfo(struct seq_file *m)
    {
    uint pvid, svid, phid1;
    pvid = mfspr(SPRN_PVR);
    svid = mfspr(SPRN_SVR);
    seq_printf(m, "Vendor\t\t: Emerson Network Power\n");
    seq_printf(m, "Board\t\t: KSI8560\n");
    if (cpld_base) {
    seq_printf(m, "Hardware rev\t: %d\n",
    in_8(cpld_base + KSI8560_CPLD_HVR));
    seq_printf(m, "CPLD rev\t: %d\n",
    in_8(cpld_base + KSI8560_CPLD_PVR));
    } else
    seq_printf(m, "Unknown Hardware and CPLD revs\n");
    seq_printf(m, "PVR\t\t: 0x%x\n", pvid);
    seq_printf(m, "SVR\t\t: 0x%x\n", svid);
// Display cpu Pll setting
    phid1 = mfspr(SPRN_HID1);
    seq_printf(m, "PLL setting\t: 0x%x\n", ((phid1 >> 24) & 0x3f));
    }
    machine_device_initcall(ksi8560, mpc85xx_common_publish_devices);
    define_machine(ksi8560) {
    .name			= "KSI8560",
    .compatible		= "emerson,KSI8560",
    .setup_arch		= ksi8560_setup_arch,
    .init_IRQ		= ksi8560_pic_init,
    .show_cpuinfo		= ksi8560_show_cpuinfo,
    .get_irq		= mpic_get_irq,
    .restart		= machine_restart,
    };
