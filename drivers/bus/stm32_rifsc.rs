//! Automatically rewritten from C to Rust
//! Source: drivers/bus/stm32_rifsc.c
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
// Copyright (C) 2023, STMicroelectronics - All Rights Reserved
//

//
// RIFSC offset register
//
pub const RIFSC_RISC_SECCFGR0: c_uint = 0x10;
pub const RIFSC_RISC_PRIVCFGR0: c_uint = 0x30;
pub const RIFSC_RISC_PER0_CIDCFGR: c_uint = 0x100;
pub const RIFSC_RISC_PER0_SEMCR: c_uint = 0x104;
pub const RIFSC_RISC_REG0_ACFGR: c_uint = 0x900;
pub const RIFSC_RISC_REG3_AADDR: c_uint = 0x924;
pub const RIFSC_RISC_HWCFGR2: c_uint = 0xFEC;
//
// SEMCR register
//

//
// HWCFGR2 register
//

//
// RIFSC miscellaneous
//

pub const RIFSC_RISC_SEML_SHIFT: c_int = 16;

    RIFSC_RISC_SEM_EN_MASK | \
    RIFSC_RISC_SCID_MASK | \
    RIFSC_RISC_SEMWL_MASK)
pub const IDS_PER_RISC_SEC_PRIV_REGS: c_int = 32;
// RIF miscellaneous
//
// CIDCFGR register fields
//

pub const SEMWL_SHIFT: c_int = 16;
// Compartiment IDs
pub const RIF_CID0: c_uint = 0x0;
pub const RIF_CID1: c_uint = 0x1;

pub const RIFSC_RISUP_ENTRIES: c_int = 128;
pub const RIFSC_RIMU_ENTRIES: c_int = 16;
pub const RIFSC_RISAL_SUBREGIONS: c_int = 2;
pub const RIFSC_RISAL_GRANULARITY: c_int = 8;
pub const RIFSC_RIMC_ATTR0: c_uint = 0xC10;

    static const char *stm32mp21_rifsc_rimu_names[RIFSC_RIMU_ENTRIES] = {
    "ETR",
    "SDMMC1",
    "SDMMC2",
    "SDMMC3",
    "OTG_HS",
    "USBH",
    "ETH1",
    "ETH2",
    "RESERVED",
    "RESERVED",
    "DCMIPP",
    "LTDC_L1/L2",
    "LTDC_L3",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    };
    static const char *stm32mp25_rifsc_rimu_names[RIFSC_RIMU_ENTRIES] = {
    "ETR",
    "SDMMC1",
    "SDMMC2",
    "SDMMC3",
    "USB3DR",
    "USBH",
    "ETH1",
    "ETH2",
    "PCIE",
    "GPU",
    "DMCIPP",
    "LTDC_L0/L1",
    "LTDC_L2",
    "LTDC_ROT",
    "VDEC",
    "VENC"
    };
    static const char *stm32mp21_rifsc_risup_names[RIFSC_RISUP_ENTRIES] = {
    "TIM1",
    "TIM2",
    "TIM3",
    "TIM4",
    "TIM5",
    "TIM6",
    "TIM7",
    "TIM8",
    "TIM10",
    "TIM11",
    "TIM12",
    "TIM13",
    "TIM14",
    "TIM15",
    "TIM16",
    "TIM17",
    "RESERVED",
    "LPTIM1",
    "LPTIM2",
    "LPTIM3",
    "LPTIM4",
    "LPTIM5",
    "SPI1",
    "SPI2",
    "SPI3",
    "SPI4",
    "SPI5",
    "SPI6",
    "RESERVED",
    "RESERVED",
    "SPDIFRX",
    "USART1",
    "USART2",
    "USART3",
    "UART4",
    "UART5",
    "USART6",
    "UART7",
    "RESERVED",
    "RESERVED",
    "LPUART1",
    "I2C1",
    "I2C2",
    "I2C3",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "SAI1",
    "SAI2",
    "SAI3",
    "SAI4",
    "RESERVED",
    "MDF1",
    "RESERVED",
    "FDCAN",
    "HDP",
    "ADC1",
    "ADC2",
    "ETH1",
    "ETH2",
    "RESERVED",
    "USBH",
    "RESERVED",
    "RESERVED",
    "OTG_HS",
    "DDRPERFM",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "STGEN",
    "OCTOSPI1",
    "RESERVED",
    "SDMMC1",
    "SDMMC2",
    "SDMMC3",
    "RESERVED",
    "LTDC_CMN",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "CSI",
    "DCMIPP",
    "DCMI_PSSI",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "RNG1",
    "RNG2",
    "PKA",
    "SAES",
    "HASH1",
    "HASH2",
    "CRYP1",
    "CRYP2",
    "IWDG1",
    "IWDG2",
    "IWDG3",
    "IWDG4",
    "WWDG1",
    "RESERVED",
    "VREFBUF",
    "DTS",
    "RAMCFG",
    "CRC",
    "SERC",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "I3C1",
    "I3C2",
    "I3C3",
    "RESERVED",
    "ICACHE_DCACHE",
    "LTDC_L1L2",
    "LTDC_L3",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "RESERVED",
    "OTFDEC1",
    "RESERVED",
    "IAC",
    };
    static const char *stm32mp25_rifsc_risup_names[RIFSC_RISUP_ENTRIES] = {
    "TIM1",
    "TIM2",
    "TIM3",
    "TIM4",
    "TIM5",
    "TIM6",
    "TIM7",
    "TIM8",
    "TIM10",
    "TIM11",
    "TIM12",
    "TIM13",
    "TIM14",
    "TIM15",
    "TIM16",
    "TIM17",
    "TIM20",
    "LPTIM1",
    "LPTIM2",
    "LPTIM3",
    "LPTIM4",
    "LPTIM5",
    "SPI1",
    "SPI2",
    "SPI3",
    "SPI4",
    "SPI5",
    "SPI6",
    "SPI7",
    "SPI8",
    "SPDIFRX",
    "USART1",
    "USART2",
    "USART3",
    "UART4",
    "UART5",
    "USART6",
    "UART7",
    "UART8",
    "UART9",
    "LPUART1",
    "I2C1",
    "I2C2",
    "I2C3",
    "I2C4",
    "I2C5",
    "I2C6",
    "I2C7",
    "I2C8",
    "SAI1",
    "SAI2",
    "SAI3",
    "SAI4",
    "RESERVED",
    "MDF1",
    "ADF1",
    "FDCAN",
    "HDP",
    "ADC12",
    "ADC3",
    "ETH1",
    "ETH2",
    "RESERVED",
    "USBH",
    "RESERVED",
    "RESERVED",
    "USB3DR",
    "COMBOPHY",
    "PCIE",
    "UCPD1",
    "ETHSW_DEIP",
    "ETHSW_ACM_CF",
    "ETHSW_ACM_MSGBU",
    "STGEN",
    "OCTOSPI1",
    "OCTOSPI2",
    "SDMMC1",
    "SDMMC2",
    "SDMMC3",
    "GPU",
    "LTDC_CMN",
    "DSI_CMN",
    "RESERVED",
    "RESERVED",
    "LVDS",
    "RESERVED",
    "CSI",
    "DCMIPP",
    "DCMI_PSSI",
    "VDEC",
    "VENC",
    "RESERVED",
    "RNG",
    "PKA",
    "SAES",
    "HASH",
    "CRYP1",
    "CRYP2",
    "IWDG1",
    "IWDG2",
    "IWDG3",
    "IWDG4",
    "IWDG5",
    "WWDG1",
    "WWDG2",
    "RESERVED",
    "VREFBUF",
    "DTS",
    "RAMCFG",
    "CRC",
    "SERC",
    "OCTOSPIM",
    "GICV2M",
    "RESERVED",
    "I3C1",
    "I3C2",
    "I3C3",
    "I3C4",
    "ICACHE_DCACHE",
    "LTDC_L0L1",
    "LTDC_L2",
    "LTDC_ROT",
    "DSI_TRIG",
    "DSI_RDFIFO",
    "RESERVED",
    "OTFDEC1",
    "OTFDEC2",
    "IAC",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rifsc_risup_debug_data {
    pub dev_name: [c_char; 15],
    pub dev_cid: u8,
    pub dev_sem_cids: u8,
    pub dev_id: u8,
    pub dev_cid_filt_en: bool,
    pub dev_sem_en: bool,
    pub dev_priv: bool,
    pub dev_sec: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rifsc_rimu_debug_data {
    pub m_name: [c_char; 11],
    pub m_cid: u8,
    pub cidsel: bool,
    pub m_sec: bool,
    pub m_priv: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rifsc_subreg_debug_data {
    pub sr_sec: bool,
    pub sr_priv: bool,
    pub sr_cid: u8,
    pub sr_rlock: bool,
    pub sr_enable: bool,
    pub sr_start: u16,
    pub sr_length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_rifsc_resources_names {
    pub device_names: *const c_char,
    pub initiator_names: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rifsc_dbg_private {
    pub res_names: *const stm32_rifsc_resources_names,
    pub mmio: *mut void __iomem,
    pub nb_risup: c_uint,
    pub nb_rimu: c_uint,
    pub nb_risal: c_uint,
}

    static const struct stm32_rifsc_resources_names rifsc_mp21_res_names = {
    .device_names = stm32mp21_rifsc_risup_names,
    .initiator_names = stm32mp21_rifsc_rimu_names,
    };
    static const struct stm32_rifsc_resources_names rifsc_mp25_res_names = {
    .device_names = stm32mp25_rifsc_risup_names,
    .initiator_names = stm32mp25_rifsc_rimu_names,
    };
    static void stm32_rifsc_fill_rimu_dbg_entry(struct rifsc_dbg_private *rifsc,
    struct rifsc_rimu_debug_data *dbg_entry, int i)
    {
    const struct stm32_rifsc_resources_names *dbg_names = rifsc.res_names;
    let mut rimc_attr: u32 = readl_relaxed(rifsc.mmio + RIFSC_RIMC_ATTR0 + 0x4 * i);
    strscpy(dbg_entry.m_name, dbg_names.initiator_names[i]);
    dbg_entry.m_cid = FIELD_GET(RIFSC_RIMC_MCID_MASK, rimc_attr);
    dbg_entry.cidsel = rimc_attr & RIFSC_RIMC_CIDSEL;
    dbg_entry.m_sec = rimc_attr & RIFSC_RIMC_MSEC;
    dbg_entry.m_priv = rimc_attr & RIFSC_RIMC_MPRIV;
    }
    static void stm32_rifsc_fill_dev_dbg_entry(struct rifsc_dbg_private *rifsc,
    struct rifsc_risup_debug_data *dbg_entry, int i)
    {
    const struct stm32_rifsc_resources_names *dbg_names = rifsc.res_names;
    u32 cid_cfgr, sec_cfgr, priv_cfgr;
    let mut reg_id: u8 = i / IDS_PER_RISC_SEC_PRIV_REGS;
    let mut reg_offset: u8 = i % IDS_PER_RISC_SEC_PRIV_REGS;
    cid_cfgr = readl_relaxed(rifsc.mmio + RIFSC_RISC_PER0_CIDCFGR + 0x8 * i);
    sec_cfgr = readl_relaxed(rifsc.mmio + RIFSC_RISC_SECCFGR0 + 0x4 * reg_id);
    priv_cfgr = readl_relaxed(rifsc.mmio + RIFSC_RISC_PRIVCFGR0 + 0x4 * reg_id);
    strscpy(dbg_entry.dev_name, dbg_names.device_names[i]);
    dbg_entry.dev_id = i;
    dbg_entry.dev_cid_filt_en = cid_cfgr & CIDCFGR_CFEN;
    dbg_entry.dev_sem_en = cid_cfgr & CIDCFGR_SEMEN;
    dbg_entry.dev_cid = FIELD_GET(RIFSC_RISC_SCID_MASK, cid_cfgr);
    dbg_entry.dev_sem_cids = FIELD_GET(RIFSC_RISC_SEMWL_MASK, cid_cfgr);
    dbg_entry.dev_sec = sec_cfgr & BIT(reg_offset) ?  true : false;
    dbg_entry.dev_priv = priv_cfgr & BIT(reg_offset) ?  true : false;
    }
    static void stm32_rifsc_fill_subreg_dbg_entry(struct rifsc_dbg_private *rifsc,
    struct rifsc_subreg_debug_data *dbg_entry, int i,
    int j)
    {
    let mut risc_xcfgr: u32 = readl_relaxed(rifsc.mmio + RIFSC_RISC_REG0_ACFGR + 0x10 * i + 0x8 * j);
    u32 risc_xaddr;
    dbg_entry.sr_sec = risc_xcfgr & RIFSC_RISC_SRSEC;
    dbg_entry.sr_priv = risc_xcfgr & RIFSC_RISC_SRPRIV;
    dbg_entry.sr_cid = FIELD_GET(RIFSC_RISC_SRCID_MASK, risc_xcfgr);
    dbg_entry.sr_rlock = risc_xcfgr & RIFSC_RISC_SRRLOCK;
    dbg_entry.sr_enable = risc_xcfgr & RIFSC_RISC_SREN;
    if (i == 2) {
    risc_xaddr = readl_relaxed(rifsc.mmio + RIFSC_RISC_REG3_AADDR + 0x8 * j);
    dbg_entry.sr_length = FIELD_GET(RIFSC_RISC_SRLENGTH_MASK, risc_xaddr);
    dbg_entry.sr_start = FIELD_GET(RIFSC_RISC_SRSTART_MASK, risc_xaddr);
    } else {
    dbg_entry.sr_start = 0;
    dbg_entry.sr_length = U16_MAX;
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_rifsc_conf_dump_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int stm32_rifsc_conf_dump_show(struct seq_file *s, void *data)
    {
    struct rifsc_dbg_private *rifsc = (struct rifsc_dbg_private *)s.private;
    int i, j;
    seq_puts(s, "\n=============================================\n");
    seq_puts(s, "                 RIFSC dump\n");
    seq_puts(s, "=============================================\n\n");
    seq_puts(s, "\n=============================================\n");
    seq_puts(s, "                 RISUP dump\n");
    seq_puts(s, "=============================================\n");
    seq_printf(s, "\n| %-15s |", "Peripheral name");
    seq_puts(s, "| Firewall ID |");
    seq_puts(s, "| N/SECURE |");
    seq_puts(s, "| N/PRIVILEGED |");
    seq_puts(s, "| CID filtering |");
    seq_puts(s, "| Semaphore mode |");
    seq_puts(s, "| SCID |");
    seq_printf(s, "| %7s |\n", "SEMWL");
    for (i = 0; i < RIFSC_RISUP_ENTRIES && i < rifsc.nb_risup; i++) {
    struct rifsc_risup_debug_data d_dbg_entry;
    stm32_rifsc_fill_dev_dbg_entry(rifsc, &d_dbg_entry, i);
    seq_printf(s, "| %-15s |", d_dbg_entry.dev_name);
    seq_printf(s, "| %-11d |", d_dbg_entry.dev_id);
    seq_printf(s, "| %-8s |", d_dbg_entry.dev_sec ? "SEC" : "NSEC");
    seq_printf(s, "| %-12s |", d_dbg_entry.dev_priv ? "PRIV" : "NPRIV");
    seq_printf(s, "| %-13s |", str_enabled_disabled(d_dbg_entry.dev_cid_filt_en));
    seq_printf(s, "| %-14s |", str_enabled_disabled(d_dbg_entry.dev_sem_en));
    seq_printf(s, "| %-4d |", d_dbg_entry.dev_cid);
    seq_printf(s, "| %#-7x |\n", d_dbg_entry.dev_sem_cids);
    }
    seq_puts(s, "\n=============================================\n");
    seq_puts(s, "                  RIMU dump\n");
    seq_puts(s, "=============================================\n");
    seq_puts(s, "| RIMU's name |");
    seq_puts(s, "| CIDSEL |");
    seq_puts(s, "| MCID |");
    seq_puts(s, "| N/SECURE |");
    seq_puts(s, "| N/PRIVILEGED |\n");
    for (i = 0; i < RIFSC_RIMU_ENTRIES && rifsc.nb_rimu; i++) {
    struct rifsc_rimu_debug_data m_dbg_entry;
    stm32_rifsc_fill_rimu_dbg_entry(rifsc, &m_dbg_entry, i);
    seq_printf(s, "| %-11s |", m_dbg_entry.m_name);
    seq_printf(s, "| %-6s |", m_dbg_entry.cidsel ? "CIDSEL" : "");
    seq_printf(s, "| %-4d |", m_dbg_entry.m_cid);
    seq_printf(s, "| %-8s |", m_dbg_entry.m_sec ? "SEC" : "NSEC");
    seq_printf(s, "| %-12s |\n", m_dbg_entry.m_priv ? "PRIV" : "NPRIV");
    }
    if (rifsc.nb_risal > 0) {
    seq_puts(s, "\n=============================================\n");
    seq_puts(s, "                  RISAL dump\n");
    seq_puts(s, "=============================================\n");
    seq_puts(s, "| Memory  |");
    seq_puts(s, "| Subreg. |");
    seq_puts(s, "| N/SECURE |");
    seq_puts(s, "| N/PRIVILEGED |");
    seq_puts(s, "| Subreg. CID |");
    seq_puts(s, "| Resource lock |");
    seq_puts(s, "| Subreg. enable |");
    seq_puts(s, "| Subreg. start |");
    seq_puts(s, "|  Subreg. end  |\n");
    for (i = 0; i < rifsc.nb_risal; i++) {
    for (j = 0; j < RIFSC_RISAL_SUBREGIONS; j++) {
    struct rifsc_subreg_debug_data sr_dbg_entry;
    stm32_rifsc_fill_subreg_dbg_entry(rifsc, &sr_dbg_entry, i, j);
    seq_printf(s, "| LPSRAM%1d |", i + 1);
    seq_printf(s, "|    %1s    |", (j == 0) ? "A" : "B");
    seq_printf(s, "| %-8s |", sr_dbg_entry.sr_sec ? "SEC" : "NSEC");
    seq_printf(s, "| %-12s |", sr_dbg_entry.sr_priv ? "PRIV" : "NPRIV");
    seq_printf(s, "| 0x%-9x |", sr_dbg_entry.sr_cid);
    seq_printf(s, "| %-13s |",
    sr_dbg_entry.sr_rlock ? "locked (1)" : "unlocked (0)");
    seq_printf(s, "| %-14s |",
    str_enabled_disabled(sr_dbg_entry.sr_enable));
    seq_printf(s, "| 0x%-11x |", sr_dbg_entry.sr_start);
    seq_printf(s, "| 0x%-11x |\n", sr_dbg_entry.sr_start +
    sr_dbg_entry.sr_length - 1);
    }
    }
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(stm32_rifsc_conf_dump);
    static int stm32_rifsc_register_debugfs(struct stm32_firewall_controller *rifsc_controller,
    u32 nb_risup, u32 nb_rimu, u32 nb_risal)
    {
    struct rifsc_dbg_private *rifsc_priv;
    struct dentry *root = core::ptr::null_mut();
    rifsc_priv = devm_kzalloc(rifsc_controller.dev, sizeof(*rifsc_priv), GFP_KERNEL);
    if (!rifsc_priv)
    return -ENOMEM;
    rifsc_priv.mmio = rifsc_controller.mmio;
    rifsc_priv.nb_risup = nb_risup;
    rifsc_priv.nb_rimu = nb_rimu;
    rifsc_priv.nb_risal = nb_risal;
    rifsc_priv.res_names = of_device_get_match_data(rifsc_controller.dev);
    root = debugfs_lookup("stm32_firewall", core::ptr::null_mut());
    if (!root)
    root = debugfs_create_dir("stm32_firewall", core::ptr::null_mut());
    if (IS_ERR(root))
    return PTR_ERR(root);
    debugfs_create_file("rifsc", 0444, root, rifsc_priv, &stm32_rifsc_conf_dump_fops);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn stm32_rifsc_is_semaphore_available(addr: *mut void __iomem) -> bool {
    static bool stm32_rifsc_is_semaphore_available(void __iomem *addr)
    {
    return !(readl(addr) & SEMCR_MUTEX);
    }
    static int stm32_rif_acquire_semaphore(struct stm32_firewall_controller *stm32_firewall_controller,
    int id)
    {
    void __iomem *addr = stm32_firewall_controller.mmio + RIFSC_RISC_PER0_SEMCR + 0x8 * id;
    writel(SEMCR_MUTEX, addr);
// Check that CID1 has the semaphore
    if (stm32_rifsc_is_semaphore_available(addr) ||
    FIELD_GET(RIFSC_RISC_SCID_MASK, readl(addr)) != RIF_CID1)
    return -EACCES;
    return 0;
    }
    static void stm32_rif_release_semaphore(struct stm32_firewall_controller *stm32_firewall_controller,
    int id)
    {
    void __iomem *addr = stm32_firewall_controller.mmio + RIFSC_RISC_PER0_SEMCR + 0x8 * id;
    if (stm32_rifsc_is_semaphore_available(addr))
    return;
    writel(SEMCR_MUTEX, addr);
// Ok if another compartment takes the semaphore before the check
    WARN_ON(!stm32_rifsc_is_semaphore_available(addr) &&
    FIELD_GET(RIFSC_RISC_SCID_MASK, readl(addr)) == RIF_CID1);
    }
#[no_mangle]
unsafe extern "C" fn stm32_rifsc_grant_access(ctrl: *mut stm32_firewall_controller, firewall_id: u32) -> c_int {
    static int stm32_rifsc_grant_access(struct stm32_firewall_controller *ctrl, u32 firewall_id)
    {
    struct stm32_firewall_controller *rifsc_controller = ctrl;
    u32 reg_offset, reg_id, sec_reg_value, cid_reg_value;
    int rc;
    if (firewall_id >= rifsc_controller.max_entries) {
    dev_err(rifsc_controller.dev, "Invalid sys bus ID %u", firewall_id);
    return -EINVAL;
    }
//
// RIFSC_RISC_PRIVCFGRx and RIFSC_RISC_SECCFGRx both handle configuration access for
// 32 peripherals. On the other hand, there is one _RIFSC_RISC_PERx_CIDCFGR register
// per peripheral
//
    reg_id = firewall_id / IDS_PER_RISC_SEC_PRIV_REGS;
    reg_offset = firewall_id % IDS_PER_RISC_SEC_PRIV_REGS;
    sec_reg_value = readl(rifsc_controller.mmio + RIFSC_RISC_SECCFGR0 + 0x4 * reg_id);
    cid_reg_value = readl(rifsc_controller.mmio + RIFSC_RISC_PER0_CIDCFGR + 0x8 * firewall_id);
// Check security configuration
    if (sec_reg_value & BIT(reg_offset)) {
    dev_dbg(rifsc_controller.dev,
    "Invalid security configuration for peripheral: %d\n", firewall_id);
    return -EACCES;
    }
// Skip CID check if CID filtering isn't enabled
    if (!(cid_reg_value & CIDCFGR_CFEN))
    goto skip_cid_check;
// First check conditions for semaphore mode, which doesn't take into account static CID.
    if (cid_reg_value & CIDCFGR_SEMEN) {
    if (!(cid_reg_value & BIT(RIF_CID1 + SEMWL_SHIFT))) {
    dev_dbg(rifsc_controller.dev,
    "Invalid bus semaphore configuration: index %d\n", firewall_id);
    return -EACCES;
    }
    rc = stm32_rif_acquire_semaphore(rifsc_controller, firewall_id);
    if (rc) {
    dev_dbg(rifsc_controller.dev,
    "Couldn't acquire semaphore for peripheral: %d\n", firewall_id);
    return rc;
    }
    } else if (FIELD_GET(RIFSC_RISC_SCID_MASK, cid_reg_value) != RIF_CID1) {
    dev_dbg(rifsc_controller.dev, "Invalid CID configuration for peripheral: %d\n",
    firewall_id);
    return -EACCES;
    }
    skip_cid_check:
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_rifsc_release_access(ctrl: *mut stm32_firewall_controller, firewall_id: u32) {
    static void stm32_rifsc_release_access(struct stm32_firewall_controller *ctrl, u32 firewall_id)
    {
    stm32_rif_release_semaphore(ctrl, firewall_id);
    }
#[no_mangle]
unsafe extern "C" fn stm32_rifsc_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_rifsc_probe(struct platform_device *pdev)
    {
    struct stm32_firewall_controller *rifsc_controller;
    struct device_node *np = pdev.dev.of_node;
    u32 nb_risup, nb_rimu, nb_risal;
    struct resource *res;
    void __iomem *mmio;
    int rc;
    rifsc_controller = devm_kzalloc(&pdev.dev, sizeof(*rifsc_controller), GFP_KERNEL);
    if (!rifsc_controller)
    return -ENOMEM;
    mmio = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(mmio))
    return PTR_ERR(mmio);
    rifsc_controller.dev = &pdev.dev;
    rifsc_controller.mmio = mmio;
    rifsc_controller.name = dev_driver_string(rifsc_controller.dev);
    rifsc_controller.type = STM32_PERIPHERAL_FIREWALL | STM32_MEMORY_FIREWALL;
    rifsc_controller.grant_access = stm32_rifsc_grant_access;
    rifsc_controller.release_access = stm32_rifsc_release_access;
// Get number of RIFSC entries
    nb_risup = FIELD_GET(HWCFGR2_CONF1_MASK,
    readl(rifsc_controller.mmio + RIFSC_RISC_HWCFGR2));
    nb_rimu = FIELD_GET(HWCFGR2_CONF2_MASK,
    readl(rifsc_controller.mmio + RIFSC_RISC_HWCFGR2));
    nb_risal = FIELD_GET(HWCFGR2_CONF3_MASK,
    readl(rifsc_controller.mmio + RIFSC_RISC_HWCFGR2));
//
// On STM32MP21, RIFSC_RISC_HWCFGR2 shows an incorrect number of RISAL (NUM_RISAL is 3
// instead of 0). A software workaround is implemented using the st,mem-map property in the
// device tree. This property is absent or left empty if there is no RISAL.
//
    if (of_device_is_compatible(np, "st,stm32mp21-rifsc"))
    nb_risal = 0;
    rifsc_controller.max_entries = nb_risup + nb_rimu + nb_risal;
    platform_set_drvdata(pdev, rifsc_controller);
    rc = stm32_firewall_controller_register(rifsc_controller);
    if (rc) {
    dev_err(rifsc_controller.dev, "Couldn't register as a firewall controller: %d",
    rc);
    return rc;
    }
    rc = stm32_firewall_populate_bus(rifsc_controller);
    if (rc) {
    dev_err(rifsc_controller.dev, "Couldn't populate RIFSC bus: %d",
    rc);
    return rc;
    }

    rc = stm32_rifsc_register_debugfs(rifsc_controller, nb_risup, nb_rimu, nb_risal);
    if (rc)
    return dev_err_probe(rifsc_controller.dev, rc, "Failed creating debugfs entry\n");

// Populate all allowed nodes
    return of_platform_populate(np, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    }
    static const struct of_device_id stm32_rifsc_of_match[] = {
    {
    .compatible = "st,stm32mp25-rifsc",

    .data = &rifsc_mp25_res_names,

    },
    {
    .compatible = "st,stm32mp21-rifsc",

    .data = &rifsc_mp21_res_names,

    },
    {}
    };
    MODULE_DEVICE_TABLE(of, stm32_rifsc_of_match);
    static struct platform_driver stm32_rifsc_driver = {
    .probe  = stm32_rifsc_probe,
    .driver = {
    .name = "stm32-rifsc",
    .of_match_table = stm32_rifsc_of_match,
    },
    };
    module_platform_driver(stm32_rifsc_driver);
    MODULE_AUTHOR("Gatien Chevallier <gatien.chevallier@foss.st.com>");
    MODULE_DESCRIPTION("STMicroelectronics RIFSC driver");
    MODULE_LICENSE("GPL");
