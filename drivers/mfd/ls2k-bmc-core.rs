//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/ls2k-bmc-core.c
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
// Loongson-2K Board Management Controller (BMC) Core Driver.
//
// Copyright (C) 2024-2025 Loongson Technology Corporation Limited.
//
// Authors:
// Chong Qiao <qiaochong@loongson.cn>
// Binbin Zhou <zhoubinbin@loongson.cn>
//

// LS2K BMC resources

pub const LS2K_IPMI_RES_SIZE: c_uint = 0x1C;

pub const LS7A_PCI_CFG_SIZE: c_uint = 0x100;
// LS7A bridge registers
pub const LS7A_PCIE_PORT_CTL0: c_uint = 0x0;
pub const LS7A_PCIE_PORT_STS1: c_uint = 0xC;
pub const LS7A_GEN2_CTL: c_uint = 0x80C;
pub const LS7A_SYMBOL_TIMER: c_uint = 0x71C;
// Bits of LS7A_PCIE_PORT_CTL0

// Bits of LS7A_PCIE_PORT_STS1

pub const LS2K_BMC_PCIE_CONNECTED: c_uint = 0x11;
pub const LS2K_BMC_PCIE_DELAY_US: c_int = 1000;
pub const LS2K_BMC_PCIE_TIMEOUT_US: c_int = 1000000;
// Bits of LS7A_GEN2_CTL

// Bits of LS7A_SYMBOL_TIMER

// Interval between interruptions

// Maximum time to wait for U-Boot and DDR to be ready with ms.
pub const LS2K_BMC_RESET_WAIT_TIME: c_int = 10000;
// It's an experience value
pub const LS7A_BAR0_CHECK_MAX_TIMES: c_int = 2000;
pub const PCI_REG_STRIDE: c_uint = 0x4;
pub const LS2K_BMC_RESET_GPIO: c_int = 14;
pub const LOONGSON_GPIO_REG_BASE: c_uint = 0x1FE00500;
pub const LOONGSON_GPIO_REG_SIZE: c_uint = 0x18;
pub const LOONGSON_GPIO_OEN: c_uint = 0x0;
pub const LOONGSON_GPIO_FUNC: c_uint = 0x4;
pub const LOONGSON_GPIO_INTPOL: c_uint = 0x10;
pub const LOONGSON_GPIO_INTEN: c_uint = 0x14;
pub const LOONGSON_IO_INT_BASE: c_int = 16;

    enum {
    LS2K_BMC_DISPLAY,
    LS2K_BMC_IPMI0,
    LS2K_BMC_IPMI1,
    LS2K_BMC_IPMI2,
    LS2K_BMC_IPMI3,
    LS2K_BMC_IPMI4,
    };
    static struct resource ls2k_display_resources[] = {
    DEFINE_RES_MEM_NAMED(LS2K_DISPLAY_RES_START, SZ_4M, "simpledrm-res"),
    };
    static struct resource ls2k_ipmi0_resources[] = {
    DEFINE_RES_MEM_NAMED(LS2K_IPMI0_RES_START, LS2K_IPMI_RES_SIZE, "ipmi0-res"),
    };
    static struct resource ls2k_ipmi1_resources[] = {
    DEFINE_RES_MEM_NAMED(LS2K_IPMI1_RES_START, LS2K_IPMI_RES_SIZE, "ipmi1-res"),
    };
    static struct resource ls2k_ipmi2_resources[] = {
    DEFINE_RES_MEM_NAMED(LS2K_IPMI2_RES_START, LS2K_IPMI_RES_SIZE, "ipmi2-res"),
    };
    static struct resource ls2k_ipmi3_resources[] = {
    DEFINE_RES_MEM_NAMED(LS2K_IPMI3_RES_START, LS2K_IPMI_RES_SIZE, "ipmi3-res"),
    };
    static struct resource ls2k_ipmi4_resources[] = {
    DEFINE_RES_MEM_NAMED(LS2K_IPMI4_RES_START, LS2K_IPMI_RES_SIZE, "ipmi4-res"),
    };
    static struct mfd_cell ls2k_bmc_cells[] = {
    [LS2K_BMC_DISPLAY] = {
    .name = "simple-framebuffer",
    .num_resources = ARRAY_SIZE(ls2k_display_resources),
    .resources = ls2k_display_resources
    },
    [LS2K_BMC_IPMI0] = {
    .name = "ls2k-ipmi-si",
    .num_resources = ARRAY_SIZE(ls2k_ipmi0_resources),
    .resources = ls2k_ipmi0_resources
    },
    [LS2K_BMC_IPMI1] = {
    .name = "ls2k-ipmi-si",
    .num_resources = ARRAY_SIZE(ls2k_ipmi1_resources),
    .resources = ls2k_ipmi1_resources
    },
    [LS2K_BMC_IPMI2] = {
    .name = "ls2k-ipmi-si",
    .num_resources = ARRAY_SIZE(ls2k_ipmi2_resources),
    .resources = ls2k_ipmi2_resources
    },
    [LS2K_BMC_IPMI3] = {
    .name = "ls2k-ipmi-si",
    .num_resources = ARRAY_SIZE(ls2k_ipmi3_resources),
    .resources = ls2k_ipmi3_resources
    },
    [LS2K_BMC_IPMI4] = {
    .name = "ls2k-ipmi-si",
    .num_resources = ARRAY_SIZE(ls2k_ipmi4_resources),
    .resources = ls2k_ipmi4_resources
    },
    };
// Index of the BMC PCI configuration space to be restored at BMC reset.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls2k_bmc_pci_data {
    pub pci_command: u32,
    pub base_address0: u32,
    pub interrupt_line: u32,
}

// Index of the parent PCI configuration space to be restored at BMC reset.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls2k_bmc_bridge_pci_data {
    pub pci_command: u32,
    pub base_address: [u32; 6],
    pub rom_addreess: u32,
    pub interrupt_line: u32,
    pub msi_hi: u32,
    pub msi_lo: u32,
    pub devctl: u32,
    pub linkcap: u32,
    pub linkctl_sts: u32,
    pub symbol_timer: u32,
    pub gen2_ctrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls2k_bmc_ddata {
    pub dev: *mut device,
    pub bmc_reset_work: work_struct,
    pub bmc_pci_data: ls2k_bmc_pci_data,
    pub bridge_pci_data: ls2k_bmc_bridge_pci_data,
}

#[no_mangle]
unsafe extern "C" fn ls2k_bmc_bar0_addr_is_set(pdev: *mut pci_dev) -> bool {
    static bool ls2k_bmc_bar0_addr_is_set(struct pci_dev *pdev)
    {
    u32 addr;
    pci_read_config_dword(pdev, PCI_BASE_ADDRESS_0, &addr);
    return addr & PCI_BASE_ADDRESS_MEM_MASK ? true : false;
    }
#[no_mangle]
unsafe extern "C" fn ls2k_bmc_pcie_is_connected(parent: *mut pci_dev, ddata: *mut ls2k_bmc_ddata) -> bool {
    static bool ls2k_bmc_pcie_is_connected(struct pci_dev *parent, struct ls2k_bmc_ddata *ddata)
    {
    void __iomem *base;
    int val, ret;
    base = pci_iomap(parent, 0, LS7A_PCI_CFG_SIZE);
    if (!base)
    return false;
    val = readl(base + LS7A_PCIE_PORT_CTL0);
    writel(val | LS2K_BMC_PCIE_LTSSM_ENABLE, base + LS7A_PCIE_PORT_CTL0);
    ret = readl_poll_timeout_atomic(base + LS7A_PCIE_PORT_STS1, val,
    (val & LS2K_BMC_PCIE_LTSSM_STS) == LS2K_BMC_PCIE_CONNECTED,
    LS2K_BMC_PCIE_DELAY_US, LS2K_BMC_PCIE_TIMEOUT_US);
    if (ret) {
    pci_iounmap(parent, base);
    dev_err(ddata.dev, "PCI-E training failed status=0x%x\n", val);
    return false;
    }
    pci_iounmap(parent, base);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ls2k_bmc_restore_bridge_pci_data(parent: *mut pci_dev, ddata: *mut ls2k_bmc_ddata) {
    static void ls2k_bmc_restore_bridge_pci_data(struct pci_dev *parent, struct ls2k_bmc_ddata *ddata)
    {
    int base, i = 0;
    pci_write_config_dword(parent, PCI_COMMAND, ddata.bridge_pci_data.pci_command);
    for (base = PCI_BASE_ADDRESS_0; base <= PCI_BASE_ADDRESS_5; base += PCI_REG_STRIDE, i++)
    pci_write_config_dword(parent, base, ddata.bridge_pci_data.base_address[i]);
    pci_write_config_dword(parent, PCI_ROM_ADDRESS, ddata.bridge_pci_data.rom_addreess);
    pci_write_config_dword(parent, PCI_INTERRUPT_LINE, ddata.bridge_pci_data.interrupt_line);
    pci_write_config_dword(parent, parent.msi_cap + PCI_MSI_ADDRESS_LO,
    ddata.bridge_pci_data.msi_lo);
    pci_write_config_dword(parent, parent.msi_cap + PCI_MSI_ADDRESS_HI,
    ddata.bridge_pci_data.msi_hi);
    pci_write_config_dword(parent, parent.pcie_cap + PCI_EXP_DEVCTL,
    ddata.bridge_pci_data.devctl);
    pci_write_config_dword(parent, parent.pcie_cap + PCI_EXP_LNKCAP,
    ddata.bridge_pci_data.linkcap);
    pci_write_config_dword(parent, parent.pcie_cap + PCI_EXP_LNKCTL,
    ddata.bridge_pci_data.linkctl_sts);
    pci_write_config_dword(parent, LS7A_GEN2_CTL, ddata.bridge_pci_data.gen2_ctrl);
    pci_write_config_dword(parent, LS7A_SYMBOL_TIMER, ddata.bridge_pci_data.symbol_timer);
    }
#[no_mangle]
unsafe extern "C" fn ls2k_bmc_recover_pci_data(data: *mut c_void) -> c_int {
    static int ls2k_bmc_recover_pci_data(void *data)
    {
    struct ls2k_bmc_ddata *ddata = data;
    struct pci_dev *pdev = to_pci_dev(ddata.dev);
    struct pci_dev *parent = pdev.bus.self;
    u32 i;
//
// Clear the bus, io and mem resources of the PCI-E bridge to zero, so that
// the processor can not access the LS2K PCI-E port, to avoid crashing due to
// the lack of return signal from accessing the LS2K PCI-E port.
//
    pci_write_config_dword(parent, PCI_BASE_ADDRESS_2, 0);
    pci_write_config_dword(parent, PCI_BASE_ADDRESS_3, 0);
    pci_write_config_dword(parent, PCI_BASE_ADDRESS_4, 0);
//
// When the LS2K BMC is reset, the LS7A PCI-E port is also reset, and its PCI
// BAR0 register is cleared. Due to the time gap between the GPIO interrupt
// generation and the LS2K BMC reset, the LS7A PCI BAR0 register is read to
// determine whether the reset has begun.
//
    for (i = LS7A_BAR0_CHECK_MAX_TIMES; i > 0 ; i--) {
    if (!ls2k_bmc_bar0_addr_is_set(parent))
    break;
    mdelay(1);
    }
    if (i == 0)
    return false;
    ls2k_bmc_restore_bridge_pci_data(parent, ddata);
// Check if PCI-E is connected
    if (!ls2k_bmc_pcie_is_connected(parent, ddata))
    return false;
// Waiting for U-Boot and DDR ready
    mdelay(LS2K_BMC_RESET_WAIT_TIME);
    if (!ls2k_bmc_bar0_addr_is_set(parent))
    return false;
// Restore LS2K BMC PCI-E config data
    pci_write_config_dword(pdev, PCI_COMMAND, ddata.bmc_pci_data.pci_command);
    pci_write_config_dword(pdev, PCI_BASE_ADDRESS_0, ddata.bmc_pci_data.base_address0);
    pci_write_config_dword(pdev, PCI_INTERRUPT_LINE, ddata.bmc_pci_data.interrupt_line);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls2k_bmc_events_fn(work: *mut work_struct) {
    static void ls2k_bmc_events_fn(struct work_struct *work)
    {
    struct ls2k_bmc_ddata *ddata = container_of(work, struct ls2k_bmc_ddata, bmc_reset_work);
//
// The PCI-E is lost when the BMC resets, at which point access to the PCI-E
// from other CPUs is suspended to prevent a crash.
//
    stop_machine(ls2k_bmc_recover_pci_data, ddata, core::ptr::null_mut());
    if (IS_ENABLED(CONFIG_VT)) {
// Re-push the display due to previous PCI-E loss.
    set_console(vt_move_to_console(MAX_NR_CONSOLES - 1, 1));
    }
    }
#[no_mangle]
unsafe extern "C" fn ls2k_bmc_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t ls2k_bmc_interrupt(int irq, void *arg)
    {
    struct ls2k_bmc_ddata *ddata = arg;
    static unsigned long last_jiffies;
    if (system_state != SYSTEM_RUNNING)
    return IRQ_HANDLED;
// Skip interrupt in LS2K_BMC_INT_INTERVAL
    if (time_after(jiffies, last_jiffies + LS2K_BMC_INT_INTERVAL)) {
    schedule_work(&ddata.bmc_reset_work);
    last_jiffies = jiffies;
    }
    return IRQ_HANDLED;
    }
//
// Saves the BMC parent device (LS7A) and its own PCI configuration space registers
// that need to be restored after BMC reset.
//
#[no_mangle]
unsafe extern "C" fn ls2k_bmc_save_pci_data(pdev: *mut pci_dev, ddata: *mut ls2k_bmc_ddata) {
    static void ls2k_bmc_save_pci_data(struct pci_dev *pdev, struct ls2k_bmc_ddata *ddata)
    {
    struct pci_dev *parent = pdev.bus.self;
    int base, i = 0;
    pci_read_config_dword(parent, PCI_COMMAND, &ddata.bridge_pci_data.pci_command);
    for (base = PCI_BASE_ADDRESS_0; base <= PCI_BASE_ADDRESS_5; base += PCI_REG_STRIDE, i++)
    pci_read_config_dword(parent, base, &ddata.bridge_pci_data.base_address[i]);
    pci_read_config_dword(parent, PCI_ROM_ADDRESS, &ddata.bridge_pci_data.rom_addreess);
    pci_read_config_dword(parent, PCI_INTERRUPT_LINE, &ddata.bridge_pci_data.interrupt_line);
    pci_read_config_dword(parent, parent.msi_cap + PCI_MSI_ADDRESS_LO,
    &ddata.bridge_pci_data.msi_lo);
    pci_read_config_dword(parent, parent.msi_cap + PCI_MSI_ADDRESS_HI,
    &ddata.bridge_pci_data.msi_hi);
    pci_read_config_dword(parent, parent.pcie_cap + PCI_EXP_DEVCTL,
    &ddata.bridge_pci_data.devctl);
    pci_read_config_dword(parent, parent.pcie_cap + PCI_EXP_LNKCAP,
    &ddata.bridge_pci_data.linkcap);
    pci_read_config_dword(parent, parent.pcie_cap + PCI_EXP_LNKCTL,
    &ddata.bridge_pci_data.linkctl_sts);
    pci_read_config_dword(parent, LS7A_GEN2_CTL, &ddata.bridge_pci_data.gen2_ctrl);
    ddata.bridge_pci_data.gen2_ctrl |= FIELD_PREP(LS7A_GEN2_SPEED_CHANG, 0x1) |
    FIELD_PREP(LS7A_CONF_PHY_TX, 0x0);
    pci_read_config_dword(parent, LS7A_SYMBOL_TIMER, &ddata.bridge_pci_data.symbol_timer);
    ddata.bridge_pci_data.symbol_timer |= LS7A_MASK_LEN_MATCH;
    pci_read_config_dword(pdev, PCI_COMMAND, &ddata.bmc_pci_data.pci_command);
    pci_read_config_dword(pdev, PCI_BASE_ADDRESS_0, &ddata.bmc_pci_data.base_address0);
    pci_read_config_dword(pdev, PCI_INTERRUPT_LINE, &ddata.bmc_pci_data.interrupt_line);
    }
#[no_mangle]
unsafe extern "C" fn ls2k_bmc_init(ddata: *mut ls2k_bmc_ddata) -> c_int {
    static int ls2k_bmc_init(struct ls2k_bmc_ddata *ddata)
    {
    struct pci_dev *pdev = to_pci_dev(ddata.dev);
    void __iomem *gpio_base;
    int gpio_irq, ret, val;
    ls2k_bmc_save_pci_data(pdev, ddata);
    INIT_WORK(&ddata.bmc_reset_work, ls2k_bmc_events_fn);
    ret = devm_request_irq(&pdev.dev, pdev.irq, ls2k_bmc_interrupt,
    IRQF_SHARED | IRQF_TRIGGER_FALLING, "ls2kbmc pcie", ddata);
    if (ret) {
    dev_err(ddata.dev, "Failed to request LS2KBMC PCI-E IRQ %d.\n", pdev.irq);
    return ret;
    }
    gpio_base = ioremap(LOONGSON_GPIO_REG_BASE, LOONGSON_GPIO_REG_SIZE);
    if (!gpio_base)
    return -ENOMEM;
// Disable GPIO output
    val = readl(gpio_base + LOONGSON_GPIO_OEN);
    writel(val | BIT(LS2K_BMC_RESET_GPIO), gpio_base + LOONGSON_GPIO_OEN);
// Enable GPIO functionality
    val = readl(gpio_base + LOONGSON_GPIO_FUNC);
    writel(val & ~BIT(LS2K_BMC_RESET_GPIO), gpio_base + LOONGSON_GPIO_FUNC);
// Set GPIO interrupts to low-level active
    val = readl(gpio_base + LOONGSON_GPIO_INTPOL);
    writel(val & ~BIT(LS2K_BMC_RESET_GPIO), gpio_base + LOONGSON_GPIO_INTPOL);
// Enable GPIO interrupts
    val = readl(gpio_base + LOONGSON_GPIO_INTEN);
    writel(val | BIT(LS2K_BMC_RESET_GPIO), gpio_base + LOONGSON_GPIO_INTEN);
    iounmap(gpio_base);
//
// Since gpio_chip->to_irq is not implemented in the Loongson-3 GPIO driver,
// acpi_register_gsi() is used to obtain the GPIO IRQ. The GPIO interrupt is a
// watchdog interrupt that is triggered when the BMC resets.
//
    gpio_irq = acpi_register_gsi(core::ptr::null_mut(), LS2K_BMC_RESET_GPIO_GSI, ACPI_EDGE_SENSITIVE,
    ACPI_ACTIVE_LOW);
    if (gpio_irq < 0)
    return gpio_irq;
    ret = devm_request_irq(ddata.dev, gpio_irq, ls2k_bmc_interrupt,
    IRQF_SHARED | IRQF_TRIGGER_FALLING, "ls2kbmc gpio", ddata);
    if (ret)
    dev_err(ddata.dev, "Failed to request LS2KBMC GPIO IRQ %d.\n", gpio_irq);
    acpi_unregister_gsi(LS2K_BMC_RESET_GPIO_GSI);
    return ret;
    }
//
// Currently the Loongson-2K BMC hardware does not have an I2C interface to adapt to the
// resolution. We set the resolution by presetting "video=1280x1024-16@2M" to the BMC memory.
//
#[no_mangle]
unsafe extern "C" fn ls2k_bmc_parse_mode(pdev: *mut pci_dev, pd: *mut simplefb_platform_data) -> c_int {
    static int ls2k_bmc_parse_mode(struct pci_dev *pdev, struct simplefb_platform_data *pd)
    {
    char *mode;
    int depth, ret;
// The last 16M of PCI BAR0 is used to store the resolution string.
    mode = devm_ioremap(&pdev.dev, pci_resource_start(pdev, 0) + SZ_16M, SZ_16M);
    if (!mode)
    return -ENOMEM;
// The resolution field starts with the flag "video=".
    if (!strncmp(mode, "video=", 6))
    mode = mode + 6;
    ret = kstrtoint(strsep(&mode, "x"), 10, &pd.width);
    if (ret)
    return ret;
    ret = kstrtoint(strsep(&mode, "-"), 10, &pd.height);
    if (ret)
    return ret;
    ret = kstrtoint(strsep(&mode, "@"), 10, &depth);
    if (ret)
    return ret;
    pd.stride = pd.width * depth / 8;
    pd.format = depth == 32 ? "a8r8g8b8" : "r5g6b5";
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls2k_bmc_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int ls2k_bmc_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    struct simplefb_platform_data pd;
    struct ls2k_bmc_ddata *ddata;
    resource_size_t base;
    int ret;
    ret = pcim_enable_device(dev);
    if (ret)
    return ret;
    ddata = devm_kzalloc(&dev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    ddata.dev = &dev.dev;
    ret = ls2k_bmc_init(ddata);
    if (ret)
    return ret;
    ret = ls2k_bmc_parse_mode(dev, &pd);
    if (ret)
    return ret;
    ls2k_bmc_cells[LS2K_BMC_DISPLAY].platform_data = &pd;
    ls2k_bmc_cells[LS2K_BMC_DISPLAY].pdata_size = sizeof(pd);
    base = pci_resource_start(dev, 0) + LS2K_DISPLAY_RES_START;
// Remove conflicting efifb device
    ret = aperture_remove_conflicting_devices(base, SZ_4M, "simple-framebuffer");
    if (ret)
    return dev_err_probe(&dev.dev, ret, "Failed to removed firmware framebuffers\n");
    return devm_mfd_add_devices(&dev.dev, PLATFORM_DEVID_AUTO,
    ls2k_bmc_cells, ARRAY_SIZE(ls2k_bmc_cells),
    pci_resource_n(dev, 0), 0, core::ptr::null_mut());
    }
    static struct pci_device_id ls2k_bmc_devices[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_LOONGSON, 0x1a05) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, ls2k_bmc_devices);
    static struct pci_driver ls2k_bmc_driver = {
    .name = "ls2k-bmc",
    .id_table = ls2k_bmc_devices,
    .probe = ls2k_bmc_probe,
    };
    module_pci_driver(ls2k_bmc_driver);
    MODULE_DESCRIPTION("Loongson-2K Board Management Controller (BMC) Core driver");
    MODULE_AUTHOR("Loongson Technology Corporation Limited");
    MODULE_LICENSE("GPL");
