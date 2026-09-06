//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/db8500-prcmu.c
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
// DB8500 PRCM Unit driver
//
// Copyright (C) STMicroelectronics 2009
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Kumar Sanghvi <kumar.sanghvi@stericsson.com>
// Author: Sundar Iyer <sundar.iyer@stericsson.com>
// Author: Mattias Nilsson <mattias.i.nilsson@stericsson.com>
//
// U8500 PRCM Unit interface driver
//

// Index of different voltages to be used when accessing AVSData
pub const PRCM_AVS_BASE: c_uint = 0x2FC;

pub const PRCM_AVS_VOLTAGE: c_int = 0;
pub const PRCM_AVS_VOLTAGE_MASK: c_uint = 0x3f;
pub const PRCM_AVS_ISSLOWSTARTUP: c_int = 6;

pub const PRCM_AVS_ISMODEENABLE: c_int = 7;

pub const PRCM_BOOT_STATUS: c_uint = 0xFFF;
pub const PRCM_ROMCODE_A2P: c_uint = 0xFFE;
pub const PRCM_ROMCODE_P2A: c_uint = 0xFFD;
pub const PRCM_XP70_CUR_PWR_STATE: c_uint = 0xFFC      /* 4 BYTES */;
pub const PRCM_SW_RST_REASON: c_uint = 0xFF8 /* 2 bytes */;
pub const _PRCM_MBOX_HEADER: c_uint = 0xFE8 /* 16 bytes */;

// Req Mailboxes
pub const PRCM_REQ_MB0: c_uint = 0xFDC /* 12 bytes  */;
pub const PRCM_REQ_MB1: c_uint = 0xFD0 /* 12 bytes  */;
pub const PRCM_REQ_MB2: c_uint = 0xFC0 /* 16 bytes  */;
pub const PRCM_REQ_MB3: c_uint = 0xE4C /* 372 bytes  */;
pub const PRCM_REQ_MB4: c_uint = 0xE48 /* 4 bytes  */;
pub const PRCM_REQ_MB5: c_uint = 0xE44 /* 4 bytes  */;
// Ack Mailboxes
pub const PRCM_ACK_MB0: c_uint = 0xE08 /* 52 bytes  */;
pub const PRCM_ACK_MB1: c_uint = 0xE04 /* 4 bytes */;
pub const PRCM_ACK_MB2: c_uint = 0xE00 /* 4 bytes */;
pub const PRCM_ACK_MB3: c_uint = 0xDFC /* 4 bytes */;
pub const PRCM_ACK_MB4: c_uint = 0xDF8 /* 4 bytes */;
pub const PRCM_ACK_MB5: c_uint = 0xDF4 /* 4 bytes */;
// Mailbox 0 headers
pub const MB0H_POWER_STATE_TRANS: c_int = 0;
pub const MB0H_CONFIG_WAKEUPS_EXE: c_int = 1;
pub const MB0H_READ_WAKEUP_ACK: c_int = 3;
pub const MB0H_CONFIG_WAKEUPS_SLEEP: c_int = 4;
pub const MB0H_WAKEUP_EXE: c_int = 2;
pub const MB0H_WAKEUP_SLEEP: c_int = 5;
// Mailbox 0 REQs

// Mailbox 0 ACKs

pub const PRCM_ACK_MB0_EVENT_4500_NUMBERS: c_int = 20;
// Mailbox 1 headers
pub const MB1H_ARM_APE_OPP: c_uint = 0x0;
pub const MB1H_RESET_MODEM: c_uint = 0x2;
pub const MB1H_REQUEST_APE_OPP_100_VOLT: c_uint = 0x3;
pub const MB1H_RELEASE_APE_OPP_100_VOLT: c_uint = 0x4;
pub const MB1H_RELEASE_USB_WAKEUP: c_uint = 0x5;
pub const MB1H_PLL_ON_OFF: c_uint = 0x6;
// Mailbox 1 Requests

pub const PLL_SOC0_OFF: c_uint = 0x1;
pub const PLL_SOC0_ON: c_uint = 0x2;
pub const PLL_SOC1_OFF: c_uint = 0x4;
pub const PLL_SOC1_ON: c_uint = 0x8;
// Mailbox 1 ACKs

// Mailbox 2 headers
pub const MB2H_DPS: c_uint = 0x0;
pub const MB2H_AUTO_PWR: c_uint = 0x1;
// Mailbox 2 REQs

// Mailbox 2 ACKs

pub const HWACC_PWR_ST_OK: c_uint = 0xFE;
// Mailbox 3 headers
pub const MB3H_ANC: c_uint = 0x0;
pub const MB3H_SIDETONE: c_uint = 0x1;
pub const MB3H_SYSCLK: c_uint = 0xE;
// Mailbox 3 Requests

// Mailbox 4 headers
pub const MB4H_DDR_INIT: c_uint = 0x0;
pub const MB4H_MEM_ST: c_uint = 0x1;
pub const MB4H_HOTDOG: c_uint = 0x12;
pub const MB4H_HOTMON: c_uint = 0x13;
pub const MB4H_HOT_PERIOD: c_uint = 0x14;
pub const MB4H_A9WDOG_CONF: c_uint = 0x16;
pub const MB4H_A9WDOG_EN: c_uint = 0x17;
pub const MB4H_A9WDOG_DIS: c_uint = 0x18;
pub const MB4H_A9WDOG_LOAD: c_uint = 0x19;
pub const MB4H_A9WDOG_KICK: c_uint = 0x20;
// Mailbox 4 Requests

pub const A9WDOG_AUTO_OFF_DIS: c_int = 0;
pub const A9WDOG_ID_MASK: c_uint = 0xf;
// Mailbox 5 Requests

// Mailbox 5 ACKs

pub const I2C_WR_OK: c_uint = 0x1;
pub const I2C_RD_OK: c_uint = 0x2;
pub const NUM_MB: c_int = 8;

//
// Wakeups/IRQs
//

    static struct {
    bool valid;
    struct prcmu_fw_version version;
    } fw_info;
    static struct irq_domain *db8500_irq_domain;
//
// This vector maps irq numbers to the bits in the bit field used in
// communication with the PRCMU firmware.
//
// The reason for having this is to keep the irq numbers contiguous even though
// the bits in the bit field are not. (The bits also have a tendency to move
// around, to further complicate matters.)
//

pub const IRQ_PRCMU_RTC: c_int = 0;
pub const IRQ_PRCMU_RTT0: c_int = 1;
pub const IRQ_PRCMU_RTT1: c_int = 2;
pub const IRQ_PRCMU_HSI0: c_int = 3;
pub const IRQ_PRCMU_HSI1: c_int = 4;
pub const IRQ_PRCMU_CA_WAKE: c_int = 5;
pub const IRQ_PRCMU_USB: c_int = 6;
pub const IRQ_PRCMU_ABB: c_int = 7;
pub const IRQ_PRCMU_ABB_FIFO: c_int = 8;
pub const IRQ_PRCMU_ARM: c_int = 9;
pub const IRQ_PRCMU_MODEM_SW_RESET_REQ: c_int = 10;
pub const IRQ_PRCMU_GPIO0: c_int = 11;
pub const IRQ_PRCMU_GPIO1: c_int = 12;
pub const IRQ_PRCMU_GPIO2: c_int = 13;
pub const IRQ_PRCMU_GPIO3: c_int = 14;
pub const IRQ_PRCMU_GPIO4: c_int = 15;
pub const IRQ_PRCMU_GPIO5: c_int = 16;
pub const IRQ_PRCMU_GPIO6: c_int = 17;
pub const IRQ_PRCMU_GPIO7: c_int = 18;
pub const IRQ_PRCMU_GPIO8: c_int = 19;
pub const IRQ_PRCMU_CA_SLEEP: c_int = 20;
pub const IRQ_PRCMU_HOTMON_LOW: c_int = 21;
pub const IRQ_PRCMU_HOTMON_HIGH: c_int = 22;
pub const NUM_PRCMU_WAKEUPS: c_int = 23;
    static u32 prcmu_irq_bit[NUM_PRCMU_WAKEUPS] = {
    IRQ_ENTRY(RTC),
    IRQ_ENTRY(RTT0),
    IRQ_ENTRY(RTT1),
    IRQ_ENTRY(HSI0),
    IRQ_ENTRY(HSI1),
    IRQ_ENTRY(CA_WAKE),
    IRQ_ENTRY(USB),
    IRQ_ENTRY(ABB),
    IRQ_ENTRY(ABB_FIFO),
    IRQ_ENTRY(CA_SLEEP),
    IRQ_ENTRY(ARM),
    IRQ_ENTRY(HOTMON_LOW),
    IRQ_ENTRY(HOTMON_HIGH),
    IRQ_ENTRY(MODEM_SW_RESET_REQ),
    IRQ_ENTRY(GPIO0),
    IRQ_ENTRY(GPIO1),
    IRQ_ENTRY(GPIO2),
    IRQ_ENTRY(GPIO3),
    IRQ_ENTRY(GPIO4),
    IRQ_ENTRY(GPIO5),
    IRQ_ENTRY(GPIO6),
    IRQ_ENTRY(GPIO7),
    IRQ_ENTRY(GPIO8)
    };

    static u32 prcmu_wakeup_bit[NUM_PRCMU_WAKEUP_INDICES] = {
    WAKEUP_ENTRY(RTC),
    WAKEUP_ENTRY(RTT0),
    WAKEUP_ENTRY(RTT1),
    WAKEUP_ENTRY(HSI0),
    WAKEUP_ENTRY(HSI1),
    WAKEUP_ENTRY(USB),
    WAKEUP_ENTRY(ABB),
    WAKEUP_ENTRY(ABB_FIFO),
    WAKEUP_ENTRY(ARM)
    };
//
// mb0_transfer - state needed for mailbox 0 communication.
// @lock:		The transaction lock.
// @dbb_events_lock:	A lock used to handle concurrent access to (parts of)
// the request data.
// @mask_work:		Work structure used for (un)masking wakeup interrupts.
// @req:		Request data that need to persist between requests.
//
    static struct {
    spinlock_t lock;
    spinlock_t dbb_irqs_lock;
    struct work_struct mask_work;
    struct mutex ac_wake_lock;
    struct completion ac_wake_work;
    struct {
    u32 dbb_irqs;
    u32 dbb_wakeups;
    u32 abb_events;
    } req;
    } mb0_transfer;
//
// mb1_transfer - state needed for mailbox 1 communication.
// @lock:	The transaction lock.
// @work:	The transaction completion structure.
// @ape_opp:	The current APE OPP.
// @ack:	Reply ("acknowledge") data.
//
    static struct {
    struct mutex lock;
    struct completion work;
    u8 ape_opp;
    struct {
    u8 header;
    u8 arm_opp;
    u8 ape_opp;
    u8 ape_voltage_status;
    } ack;
    } mb1_transfer;
//
// mb2_transfer - state needed for mailbox 2 communication.
// @lock:            The transaction lock.
// @work:            The transaction completion structure.
// @auto_pm_lock:    The autonomous power management configuration lock.
// @auto_pm_enabled: A flag indicating whether autonomous PM is enabled.
// @req:             Request data that need to persist between requests.
// @ack:             Reply ("acknowledge") data.
//
    static struct {
    struct mutex lock;
    struct completion work;
    spinlock_t auto_pm_lock;
    bool auto_pm_enabled;
    struct {
    u8 status;
    } ack;
    } mb2_transfer;
//
// mb3_transfer - state needed for mailbox 3 communication.
// @lock:		The request lock.
// @sysclk_lock:	A lock used to handle concurrent sysclk requests.
// @sysclk_work:	Work structure used for sysclk requests.
//
    static struct {
    spinlock_t lock;
    struct mutex sysclk_lock;
    struct completion sysclk_work;
    } mb3_transfer;
//
// mb4_transfer - state needed for mailbox 4 communication.
// @lock:	The transaction lock.
// @work:	The transaction completion structure.
//
    static struct {
    struct mutex lock;
    struct completion work;
    } mb4_transfer;
//
// mb5_transfer - state needed for mailbox 5 communication.
// @lock:	The transaction lock.
// @work:	The transaction completion structure.
// @ack:	Reply ("acknowledge") data.
//
    static struct {
    struct mutex lock;
    struct completion work;
    struct {
    u8 status;
    u8 value;
    } ack;
    } mb5_transfer;
    let mut ac_wake_req_state: static atomic_t = ATOMIC_INIT(0);
// Spinlocks
    static DEFINE_SPINLOCK(prcmu_lock);
    static DEFINE_SPINLOCK(clkout_lock);
// Global var to runtime determine TCDM base for v2 or v1
    static __iomem void *tcdm_base;
    static __iomem void *prcmu_base;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgt {
    pub offset: u32,
    pub pllsw: u32,
    pub branch: c_int,
    pub clk38div: bool,
}

    enum {
    PLL_RAW,
    PLL_FIX,
    PLL_DIV
    };
    static DEFINE_SPINLOCK(clk_mgt_lock);

    { (PRCM_##_name##_MGT), 0 , _branch, _clk38div}
    static struct clk_mgt clk_mgt[PRCMU_NUM_REG_CLOCKS] = {
    CLK_MGT_ENTRY(SGACLK, PLL_DIV, false),
    CLK_MGT_ENTRY(UARTCLK, PLL_FIX, true),
    CLK_MGT_ENTRY(MSP02CLK, PLL_FIX, true),
    CLK_MGT_ENTRY(MSP1CLK, PLL_FIX, true),
    CLK_MGT_ENTRY(I2CCLK, PLL_FIX, true),
    CLK_MGT_ENTRY(SDMMCCLK, PLL_DIV, true),
    CLK_MGT_ENTRY(SLIMCLK, PLL_FIX, true),
    CLK_MGT_ENTRY(PER1CLK, PLL_DIV, true),
    CLK_MGT_ENTRY(PER2CLK, PLL_DIV, true),
    CLK_MGT_ENTRY(PER3CLK, PLL_DIV, true),
    CLK_MGT_ENTRY(PER5CLK, PLL_DIV, true),
    CLK_MGT_ENTRY(PER6CLK, PLL_DIV, true),
    CLK_MGT_ENTRY(PER7CLK, PLL_DIV, true),
    CLK_MGT_ENTRY(LCDCLK, PLL_FIX, true),
    CLK_MGT_ENTRY(BMLCLK, PLL_DIV, true),
    CLK_MGT_ENTRY(HSITXCLK, PLL_DIV, true),
    CLK_MGT_ENTRY(HSIRXCLK, PLL_DIV, true),
    CLK_MGT_ENTRY(HDMICLK, PLL_FIX, false),
    CLK_MGT_ENTRY(APEATCLK, PLL_DIV, true),
    CLK_MGT_ENTRY(APETRACECLK, PLL_DIV, true),
    CLK_MGT_ENTRY(MCDECLK, PLL_DIV, true),
    CLK_MGT_ENTRY(IPI2CCLK, PLL_FIX, true),
    CLK_MGT_ENTRY(DSIALTCLK, PLL_FIX, false),
    CLK_MGT_ENTRY(DMACLK, PLL_DIV, true),
    CLK_MGT_ENTRY(B2R2CLK, PLL_DIV, true),
    CLK_MGT_ENTRY(TVCLK, PLL_FIX, true),
    CLK_MGT_ENTRY(SSPCLK, PLL_FIX, true),
    CLK_MGT_ENTRY(RNGCLK, PLL_FIX, true),
    CLK_MGT_ENTRY(UICCCLK, PLL_FIX, false),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsiclk {
    pub divsel_mask: u32,
    pub divsel_shift: u32,
    pub divsel: u32,
}

    static struct dsiclk dsiclk[2] = {
    {
    .divsel_mask = PRCM_DSI_PLLOUT_SEL_DSI0_PLLOUT_DIVSEL_MASK,
    .divsel_shift = PRCM_DSI_PLLOUT_SEL_DSI0_PLLOUT_DIVSEL_SHIFT,
    .divsel = PRCM_DSI_PLLOUT_SEL_PHI,
    },
    {
    .divsel_mask = PRCM_DSI_PLLOUT_SEL_DSI1_PLLOUT_DIVSEL_MASK,
    .divsel_shift = PRCM_DSI_PLLOUT_SEL_DSI1_PLLOUT_DIVSEL_SHIFT,
    .divsel = PRCM_DSI_PLLOUT_SEL_PHI,
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsiescclk {
    pub en: u32,
    pub div_mask: u32,
    pub div_shift: u32,
}

    static struct dsiescclk dsiescclk[3] = {
    {
    .en = PRCM_DSITVCLK_DIV_DSI0_ESC_CLK_EN,
    .div_mask = PRCM_DSITVCLK_DIV_DSI0_ESC_CLK_DIV_MASK,
    .div_shift = PRCM_DSITVCLK_DIV_DSI0_ESC_CLK_DIV_SHIFT,
    },
    {
    .en = PRCM_DSITVCLK_DIV_DSI1_ESC_CLK_EN,
    .div_mask = PRCM_DSITVCLK_DIV_DSI1_ESC_CLK_DIV_MASK,
    .div_shift = PRCM_DSITVCLK_DIV_DSI1_ESC_CLK_DIV_SHIFT,
    },
    {
    .en = PRCM_DSITVCLK_DIV_DSI2_ESC_CLK_EN,
    .div_mask = PRCM_DSITVCLK_DIV_DSI2_ESC_CLK_DIV_MASK,
    .div_shift = PRCM_DSITVCLK_DIV_DSI2_ESC_CLK_DIV_SHIFT,
    }
    };
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_read(reg: c_uint) -> u32 {
    u32 db8500_prcmu_read(unsigned int reg)
    {
    return readl(prcmu_base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_write(reg: c_uint, value: u32) {
    void db8500_prcmu_write(unsigned int reg, u32 value)
    {
    unsigned long flags;
    spin_lock_irqsave(&prcmu_lock, flags);
    writel(value, (prcmu_base + reg));
    spin_unlock_irqrestore(&prcmu_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_write_masked(reg: c_uint, mask: u32, value: u32) {
    void db8500_prcmu_write_masked(unsigned int reg, u32 mask, u32 value)
    {
    u32 val;
    unsigned long flags;
    spin_lock_irqsave(&prcmu_lock, flags);
    val = readl(prcmu_base + reg);
    val = ((val & ~mask) | (value & mask));
    writel(val, (prcmu_base + reg));
    spin_unlock_irqrestore(&prcmu_lock, flags);
    }
    struct prcmu_fw_version *prcmu_get_fw_version(void)
    {
    return fw_info.valid ? &fw_info.version : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn prcmu_is_ulppll_disabled() -> bool {
    static bool prcmu_is_ulppll_disabled(void)
    {
    struct prcmu_fw_version *ver;
    ver = prcmu_get_fw_version();
    return ver && ver.project == PRCMU_FW_PROJECT_U8420_SYSCLK;
    }
#[no_mangle]
pub unsafe extern "C" fn prcmu_has_arm_maxopp() -> bool {
    bool prcmu_has_arm_maxopp(void)
    {
    return (readb(tcdm_base + PRCM_AVS_VARM_MAX_OPP) &
    PRCM_AVS_ISMODEENABLE_MASK) == PRCM_AVS_ISMODEENABLE_MASK;
    }
//
// prcmu_set_rc_a2p - This function is used to run few power state sequences
// @val: Value to be set, i.e. transition requested
// Returns: 0 on success, -EINVAL on invalid argument
//
// This function is used to run the following power state sequences -
// any state to ApReset,  ApDeepSleep to ApExecute, ApExecute to ApDeepSleep
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_set_rc_a2p(val: enum romcode_write) -> c_int {
    int prcmu_set_rc_a2p(enum romcode_write val)
    {
    if (val < RDY_2_DS || val > RDY_2_XP70_RST)
    return -EINVAL;
    writeb(val, (tcdm_base + PRCM_ROMCODE_A2P));
    return 0;
    }
//
// prcmu_get_rc_p2a - This function is used to get power state sequences
// Returns: the power transition that has last happened
//
// This function can return the following transitions-
// any state to ApReset,  ApDeepSleep to ApExecute, ApExecute to ApDeepSleep
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_get_rc_p2a() -> enum romcode_read {
    enum romcode_read prcmu_get_rc_p2a(void)
    {
    return readb(tcdm_base + PRCM_ROMCODE_P2A);
    }
//
// prcmu_get_xp70_current_state - Return the current XP70 power mode
// Returns: Returns the current AP(ARM) power mode: init,
// apBoot, apExecute, apDeepSleep, apSleep, apIdle, apReset
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_get_xp70_current_state() -> enum ap_pwrst {
    enum ap_pwrst prcmu_get_xp70_current_state(void)
    {
    return readb(tcdm_base + PRCM_XP70_CUR_PWR_STATE);
    }
//
// prcmu_config_clkout - Configure one of the programmable clock outputs.
// @clkout:	The CLKOUT number (0 or 1).
// @source:	The clock to be used (one of the PRCMU_CLKSRC_*).
// @div:	The divider to be applied.
//
// Configures one of the programmable clock outputs (CLKOUTs).
// @div should be in the range [1,63] to request a configuration, or 0 to
// inform that the configuration is no longer requested.
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_config_clkout(clkout: u8, source: u8, div: u8) -> c_int {
    int prcmu_config_clkout(u8 clkout, u8 source, u8 div)
    {
    static int requests[2];
    let mut r: c_int = 0;
    unsigned long flags;
    u32 val;
    u32 bits;
    u32 mask;
    u32 div_mask;
    BUG_ON(clkout > 1);
    BUG_ON(div > 63);
    BUG_ON((clkout == 0) && (source > PRCMU_CLKSRC_CLK009));
    if (!div && !requests[clkout])
    return -EINVAL;
    if (clkout == 0) {
    div_mask = PRCM_CLKOCR_CLKODIV0_MASK;
    mask = (PRCM_CLKOCR_CLKODIV0_MASK | PRCM_CLKOCR_CLKOSEL0_MASK);
    bits = ((source << PRCM_CLKOCR_CLKOSEL0_SHIFT) |
    (div << PRCM_CLKOCR_CLKODIV0_SHIFT));
    } else {
    div_mask = PRCM_CLKOCR_CLKODIV1_MASK;
    mask = (PRCM_CLKOCR_CLKODIV1_MASK | PRCM_CLKOCR_CLKOSEL1_MASK |
    PRCM_CLKOCR_CLK1TYPE);
    bits = ((source << PRCM_CLKOCR_CLKOSEL1_SHIFT) |
    (div << PRCM_CLKOCR_CLKODIV1_SHIFT));
    }
    bits &= mask;
    spin_lock_irqsave(&clkout_lock, flags);
    val = readl(PRCM_CLKOCR);
    if (val & div_mask) {
    if (div) {
    if ((val & mask) != bits) {
    r = -EBUSY;
    goto unlock_and_return;
    }
    } else {
    if ((val & mask & ~div_mask) != bits) {
    r = -EINVAL;
    goto unlock_and_return;
    }
    }
    }
    writel((bits | (val & ~mask)), PRCM_CLKOCR);
    requests[clkout] += (div ? 1 : -1);
    unlock_and_return:
    spin_unlock_irqrestore(&clkout_lock, flags);
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_set_power_state(state: u8, keep_ulp_clk: bool, keep_ap_pll: bool) -> c_int {
    int db8500_prcmu_set_power_state(u8 state, bool keep_ulp_clk, bool keep_ap_pll)
    {
    unsigned long flags;
    BUG_ON((state < PRCMU_AP_SLEEP) || (PRCMU_AP_DEEP_IDLE < state));
    spin_lock_irqsave(&mb0_transfer.lock, flags);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(0))
    cpu_relax();
    writeb(MB0H_POWER_STATE_TRANS, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB0));
    writeb(state, (tcdm_base + PRCM_REQ_MB0_AP_POWER_STATE));
    writeb((keep_ap_pll ? 1 : 0), (tcdm_base + PRCM_REQ_MB0_AP_PLL_STATE));
    writeb((keep_ulp_clk ? 1 : 0),
    (tcdm_base + PRCM_REQ_MB0_ULP_CLOCK_STATE));
    writeb(0, (tcdm_base + PRCM_REQ_MB0_DO_NOT_WFI));
    writel(MBOX_BIT(0), PRCM_MBOX_CPU_SET);
    spin_unlock_irqrestore(&mb0_transfer.lock, flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_get_power_state_result() -> u8 {
    u8 db8500_prcmu_get_power_state_result(void)
    {
    return readb(tcdm_base + PRCM_ACK_MB0_AP_PWRSTTR_STATUS);
    }
// This function should only be called while mb0_transfer.lock is held.
#[no_mangle]
unsafe extern "C" fn config_wakeups() {
    static void config_wakeups(void)
    {
    const u8 header[2] = {
    MB0H_CONFIG_WAKEUPS_EXE,
    MB0H_CONFIG_WAKEUPS_SLEEP
    };
    static u32 last_dbb_events;
    static u32 last_abb_events;
    u32 dbb_events;
    u32 abb_events;
    unsigned int i;
    dbb_events = mb0_transfer.req.dbb_irqs | mb0_transfer.req.dbb_wakeups;
    dbb_events |= (WAKEUP_BIT_AC_WAKE_ACK | WAKEUP_BIT_AC_SLEEP_ACK);
    abb_events = mb0_transfer.req.abb_events;
    if ((dbb_events == last_dbb_events) && (abb_events == last_abb_events))
    return;
    for (i = 0; i < 2; i++) {
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(0))
    cpu_relax();
    writel(dbb_events, (tcdm_base + PRCM_REQ_MB0_WAKEUP_8500));
    writel(abb_events, (tcdm_base + PRCM_REQ_MB0_WAKEUP_4500));
    writeb(header[i], (tcdm_base + PRCM_MBOX_HEADER_REQ_MB0));
    writel(MBOX_BIT(0), PRCM_MBOX_CPU_SET);
    }
    last_dbb_events = dbb_events;
    last_abb_events = abb_events;
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_enable_wakeups(wakeups: u32) {
    void db8500_prcmu_enable_wakeups(u32 wakeups)
    {
    unsigned long flags;
    u32 bits;
    int i;
    BUG_ON(wakeups != (wakeups & VALID_WAKEUPS));
    for (i = 0, bits = 0; i < NUM_PRCMU_WAKEUP_INDICES; i++) {
    if (wakeups & BIT(i))
    bits |= prcmu_wakeup_bit[i];
    }
    spin_lock_irqsave(&mb0_transfer.lock, flags);
    mb0_transfer.req.dbb_wakeups = bits;
    config_wakeups();
    spin_unlock_irqrestore(&mb0_transfer.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_config_abb_event_readout(abb_events: u32) {
    void db8500_prcmu_config_abb_event_readout(u32 abb_events)
    {
    unsigned long flags;
    spin_lock_irqsave(&mb0_transfer.lock, flags);
    mb0_transfer.req.abb_events = abb_events;
    config_wakeups();
    spin_unlock_irqrestore(&mb0_transfer.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_get_abb_event_buffer(buf: *mut void __iomem) {
    void db8500_prcmu_get_abb_event_buffer(void __iomem **buf)
    {
    if (readb(tcdm_base + PRCM_ACK_MB0_READ_POINTER) & 1)
// buf = (tcdm_base + PRCM_ACK_MB0_WAKEUP_1_4500);
    else
// buf = (tcdm_base + PRCM_ACK_MB0_WAKEUP_0_4500);
    }
//
// db8500_prcmu_set_arm_opp - set the appropriate ARM OPP
// @opp: The new ARM operating point to which transition is to be made
// Returns: 0 on success, non-zero on failure
//
// This function sets the operating point of the ARM.
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_set_arm_opp(opp: u8) -> c_int {
    int db8500_prcmu_set_arm_opp(u8 opp)
    {
    int r;
    if (opp < ARM_NO_CHANGE || opp > ARM_EXTCLK)
    return -EINVAL;
    r = 0;
    mutex_lock(&mb1_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(1))
    cpu_relax();
    writeb(MB1H_ARM_APE_OPP, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB1));
    writeb(opp, (tcdm_base + PRCM_REQ_MB1_ARM_OPP));
    writeb(APE_NO_CHANGE, (tcdm_base + PRCM_REQ_MB1_APE_OPP));
    writel(MBOX_BIT(1), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb1_transfer.work);
    if ((mb1_transfer.ack.header != MB1H_ARM_APE_OPP) ||
    (mb1_transfer.ack.arm_opp != opp))
    r = -EIO;
    mutex_unlock(&mb1_transfer.lock);
    return r;
    }
//
// db8500_prcmu_get_arm_opp - get the current ARM OPP
//
// Returns: the current ARM OPP
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_get_arm_opp() -> c_int {
    int db8500_prcmu_get_arm_opp(void)
    {
    return readb(tcdm_base + PRCM_ACK_MB1_CURRENT_ARM_OPP);
    }
//
// db8500_prcmu_get_ddr_opp - get the current DDR OPP
//
// Returns: the current DDR OPP
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_get_ddr_opp() -> c_int {
    int db8500_prcmu_get_ddr_opp(void)
    {
    return readb(PRCM_DDR_SUBSYS_APE_MINBW);
    }
// Divide the frequency of certain clocks by 2 for APE_50_PARTLY_25_OPP.
#[no_mangle]
unsafe extern "C" fn request_even_slower_clocks(enable: bool) {
    static void request_even_slower_clocks(bool enable)
    {
    u32 clock_reg[] = {
    PRCM_ACLK_MGT,
    PRCM_DMACLK_MGT
    };
    unsigned long flags;
    unsigned int i;
    spin_lock_irqsave(&clk_mgt_lock, flags);
// Grab the HW semaphore.
    while ((readl(PRCM_SEM) & PRCM_SEM_PRCM_SEM) != 0)
    cpu_relax();
    for (i = 0; i < ARRAY_SIZE(clock_reg); i++) {
    u32 val;
    u32 div;
    val = readl(prcmu_base + clock_reg[i]);
    div = (val & PRCM_CLK_MGT_CLKPLLDIV_MASK);
    if (enable) {
    if ((div <= 1) || (div > 15)) {
    pr_err("prcmu: Bad clock divider %d in %s\n",
    div, __func__);
    goto unlock_and_return;
    }
    div <<= 1;
    } else {
    if (div <= 2)
    goto unlock_and_return;
    div >>= 1;
    }
    val = ((val & ~PRCM_CLK_MGT_CLKPLLDIV_MASK) |
    (div & PRCM_CLK_MGT_CLKPLLDIV_MASK));
    writel(val, prcmu_base + clock_reg[i]);
    }
    unlock_and_return:
// Release the HW semaphore.
    writel(0, PRCM_SEM);
    spin_unlock_irqrestore(&clk_mgt_lock, flags);
    }
//
// db8500_prcmu_set_ape_opp - set the appropriate APE OPP
// @opp: The new APE operating point to which transition is to be made
// Returns: 0 on success, non-zero on failure
//
// This function sets the operating point of the APE.
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_set_ape_opp(opp: u8) -> c_int {
    int db8500_prcmu_set_ape_opp(u8 opp)
    {
    let mut r: c_int = 0;
    if (opp == mb1_transfer.ape_opp)
    return 0;
    mutex_lock(&mb1_transfer.lock);
    if (mb1_transfer.ape_opp == APE_50_PARTLY_25_OPP)
    request_even_slower_clocks(false);
    if ((opp != APE_100_OPP) && (mb1_transfer.ape_opp != APE_100_OPP))
    goto skip_message;
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(1))
    cpu_relax();
    writeb(MB1H_ARM_APE_OPP, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB1));
    writeb(ARM_NO_CHANGE, (tcdm_base + PRCM_REQ_MB1_ARM_OPP));
    writeb(((opp == APE_50_PARTLY_25_OPP) ? APE_50_OPP : opp),
    (tcdm_base + PRCM_REQ_MB1_APE_OPP));
    writel(MBOX_BIT(1), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb1_transfer.work);
    if ((mb1_transfer.ack.header != MB1H_ARM_APE_OPP) ||
    (mb1_transfer.ack.ape_opp != opp))
    r = -EIO;
    skip_message:
    if ((!r && (opp == APE_50_PARTLY_25_OPP)) ||
    (r && (mb1_transfer.ape_opp == APE_50_PARTLY_25_OPP)))
    request_even_slower_clocks(true);
    if (!r)
    mb1_transfer.ape_opp = opp;
    mutex_unlock(&mb1_transfer.lock);
    return r;
    }
//
// db8500_prcmu_get_ape_opp - get the current APE OPP
//
// Returns: the current APE OPP
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_get_ape_opp() -> c_int {
    int db8500_prcmu_get_ape_opp(void)
    {
    return readb(tcdm_base + PRCM_ACK_MB1_CURRENT_APE_OPP);
    }
//
// db8500_prcmu_request_ape_opp_100_voltage - Request APE OPP 100% voltage
// @enable: true to request the higher voltage, false to drop a request.
//
// Calls to this function to enable and disable requests must be balanced.
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_request_ape_opp_100_voltage(enable: bool) -> c_int {
    int db8500_prcmu_request_ape_opp_100_voltage(bool enable)
    {
    let mut r: c_int = 0;
    u8 header;
    static unsigned int requests;
    mutex_lock(&mb1_transfer.lock);
    if (enable) {
    if (0 != requests++)
    goto unlock_and_return;
    header = MB1H_REQUEST_APE_OPP_100_VOLT;
    } else {
    if (requests == 0) {
    r = -EIO;
    goto unlock_and_return;
    } else if (1 != requests--) {
    goto unlock_and_return;
    }
    header = MB1H_RELEASE_APE_OPP_100_VOLT;
    }
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(1))
    cpu_relax();
    writeb(header, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB1));
    writel(MBOX_BIT(1), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb1_transfer.work);
    if ((mb1_transfer.ack.header != header) ||
    ((mb1_transfer.ack.ape_voltage_status & BIT(0)) != 0))
    r = -EIO;
    unlock_and_return:
    mutex_unlock(&mb1_transfer.lock);
    return r;
    }
//
// prcmu_release_usb_wakeup_state - release the state required by a USB wakeup
//
// This function releases the power state requirements of a USB wakeup.
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_release_usb_wakeup_state() -> c_int {
    int prcmu_release_usb_wakeup_state(void)
    {
    let mut r: c_int = 0;
    mutex_lock(&mb1_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(1))
    cpu_relax();
    writeb(MB1H_RELEASE_USB_WAKEUP,
    (tcdm_base + PRCM_MBOX_HEADER_REQ_MB1));
    writel(MBOX_BIT(1), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb1_transfer.work);
    if ((mb1_transfer.ack.header != MB1H_RELEASE_USB_WAKEUP) ||
    ((mb1_transfer.ack.ape_voltage_status & BIT(0)) != 0))
    r = -EIO;
    mutex_unlock(&mb1_transfer.lock);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn request_pll(clock: u8, enable: bool) -> c_int {
    static int request_pll(u8 clock, bool enable)
    {
    let mut r: c_int = 0;
    if (clock == PRCMU_PLLSOC0)
    clock = (enable ? PLL_SOC0_ON : PLL_SOC0_OFF);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_PLLSOC1: clock ==) -> else {
    else if (clock == PRCMU_PLLSOC1)
    clock = (enable ? PLL_SOC1_ON : PLL_SOC1_OFF);
    else
    return -EINVAL;
    mutex_lock(&mb1_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(1))
    cpu_relax();
    writeb(MB1H_PLL_ON_OFF, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB1));
    writeb(clock, (tcdm_base + PRCM_REQ_MB1_PLL_ON_OFF));
    writel(MBOX_BIT(1), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb1_transfer.work);
    if (mb1_transfer.ack.header != MB1H_PLL_ON_OFF)
    r = -EIO;
    mutex_unlock(&mb1_transfer.lock);
    return r;
    }
//
// db8500_prcmu_set_epod - set the state of a EPOD (power domain)
// @epod_id: The EPOD to set
// @epod_state: The new EPOD state
//
// This function sets the state of a EPOD (power domain). It may not be called
// from interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_set_epod(epod_id: u16, epod_state: u8) -> c_int {
    int db8500_prcmu_set_epod(u16 epod_id, u8 epod_state)
    {
    let mut r: c_int = 0;
    let mut ram_retention: bool = false;
    int i;
// check argument
    BUG_ON(epod_id >= NUM_EPOD_ID);
// set flag if retention is possible
    switch (epod_id) {
    case EPOD_ID_SVAMMDSP:
    case EPOD_ID_SIAMMDSP:
    case EPOD_ID_ESRAM12:
    case EPOD_ID_ESRAM34:
    ram_retention = true;
    break;
    }
// check argument
    BUG_ON(epod_state > EPOD_STATE_ON);
    BUG_ON(epod_state == EPOD_STATE_RAMRET && !ram_retention);
// get lock
    mutex_lock(&mb2_transfer.lock);
// wait for mailbox
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(2))
    cpu_relax();
// fill in mailbox
    for (i = 0; i < NUM_EPOD_ID; i++)
    writeb(EPOD_STATE_NO_CHANGE, (tcdm_base + PRCM_REQ_MB2 + i));
    writeb(epod_state, (tcdm_base + PRCM_REQ_MB2 + epod_id));
    writeb(MB2H_DPS, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB2));
    writel(MBOX_BIT(2), PRCM_MBOX_CPU_SET);
//
// The current firmware version does not handle errors correctly,
// and we cannot recover if there is an error.
// This is expected to change when the firmware is updated.
//
    if (!wait_for_completion_timeout(&mb2_transfer.work,
    msecs_to_jiffies(20000))) {
    pr_err("prcmu: %s timed out (20 s) waiting for a reply.\n",
    __func__);
    r = -EIO;
    goto unlock_and_return;
    }
    if (mb2_transfer.ack.status != HWACC_PWR_ST_OK)
    r = -EIO;
    unlock_and_return:
    mutex_unlock(&mb2_transfer.lock);
    return r;
    }
//
// prcmu_configure_auto_pm - Configure autonomous power management.
// @sleep: Configuration for ApSleep.
// @idle:  Configuration for ApIdle.
//
    void prcmu_configure_auto_pm(struct prcmu_auto_pm_config *sleep,
    struct prcmu_auto_pm_config *idle)
    {
    u32 sleep_cfg;
    u32 idle_cfg;
    unsigned long flags;
    BUG_ON((sleep == core::ptr::null_mut()) || (idle == core::ptr::null_mut()));
    sleep_cfg = (sleep.sva_auto_pm_enable & 0xF);
    sleep_cfg = ((sleep_cfg << 4) | (sleep.sia_auto_pm_enable & 0xF));
    sleep_cfg = ((sleep_cfg << 8) | (sleep.sva_power_on & 0xFF));
    sleep_cfg = ((sleep_cfg << 8) | (sleep.sia_power_on & 0xFF));
    sleep_cfg = ((sleep_cfg << 4) | (sleep.sva_policy & 0xF));
    sleep_cfg = ((sleep_cfg << 4) | (sleep.sia_policy & 0xF));
    idle_cfg = (idle.sva_auto_pm_enable & 0xF);
    idle_cfg = ((idle_cfg << 4) | (idle.sia_auto_pm_enable & 0xF));
    idle_cfg = ((idle_cfg << 8) | (idle.sva_power_on & 0xFF));
    idle_cfg = ((idle_cfg << 8) | (idle.sia_power_on & 0xFF));
    idle_cfg = ((idle_cfg << 4) | (idle.sva_policy & 0xF));
    idle_cfg = ((idle_cfg << 4) | (idle.sia_policy & 0xF));
    spin_lock_irqsave(&mb2_transfer.auto_pm_lock, flags);
//
// The autonomous power management configuration is done through
// fields in mailbox 2, but these fields are only used as shared
// variables - i.e. there is no need to send a message.
//
    writel(sleep_cfg, (tcdm_base + PRCM_REQ_MB2_AUTO_PM_SLEEP));
    writel(idle_cfg, (tcdm_base + PRCM_REQ_MB2_AUTO_PM_IDLE));
    mb2_transfer.auto_pm_enabled =
    ((sleep.sva_auto_pm_enable == PRCMU_AUTO_PM_ON) ||
    (sleep.sia_auto_pm_enable == PRCMU_AUTO_PM_ON) ||
    (idle.sva_auto_pm_enable == PRCMU_AUTO_PM_ON) ||
    (idle.sia_auto_pm_enable == PRCMU_AUTO_PM_ON));
    spin_unlock_irqrestore(&mb2_transfer.auto_pm_lock, flags);
    }
    EXPORT_SYMBOL(prcmu_configure_auto_pm);
#[no_mangle]
pub unsafe extern "C" fn prcmu_is_auto_pm_enabled() -> bool {
    bool prcmu_is_auto_pm_enabled(void)
    {
    return mb2_transfer.auto_pm_enabled;
    }
#[no_mangle]
unsafe extern "C" fn request_sysclk(enable: bool) -> c_int {
    static int request_sysclk(bool enable)
    {
    int r;
    unsigned long flags;
    r = 0;
    mutex_lock(&mb3_transfer.sysclk_lock);
    spin_lock_irqsave(&mb3_transfer.lock, flags);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(3))
    cpu_relax();
    writeb((enable ? ON : OFF), (tcdm_base + PRCM_REQ_MB3_SYSCLK_MGT));
    writeb(MB3H_SYSCLK, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB3));
    writel(MBOX_BIT(3), PRCM_MBOX_CPU_SET);
    spin_unlock_irqrestore(&mb3_transfer.lock, flags);
//
// The firmware only sends an ACK if we want to enable the
// SysClk, and it succeeds.
//
    if (enable && !wait_for_completion_timeout(&mb3_transfer.sysclk_work,
    msecs_to_jiffies(20000))) {
    pr_err("prcmu: %s timed out (20 s) waiting for a reply.\n",
    __func__);
    r = -EIO;
    }
    mutex_unlock(&mb3_transfer.sysclk_lock);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn request_timclk(enable: bool) -> c_int {
    static int request_timclk(bool enable)
    {
    u32 val;
//
// On the U8420_CLKSEL firmware, the ULP (Ultra Low Power)
// PLL is disabled so we cannot use doze mode, this will
// stop the clock on this firmware.
//
    if (prcmu_is_ulppll_disabled())
    val = 0;
    else
    val = (PRCM_TCR_DOZE_MODE | PRCM_TCR_TENSEL_MASK);
    if (!enable)
    val |= PRCM_TCR_STOP_TIMERS |
    PRCM_TCR_DOZE_MODE |
    PRCM_TCR_TENSEL_MASK;
    writel(val, PRCM_TCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn request_clock(clock: u8, enable: bool) -> c_int {
    static int request_clock(u8 clock, bool enable)
    {
    u32 val;
    unsigned long flags;
    spin_lock_irqsave(&clk_mgt_lock, flags);
// Grab the HW semaphore.
    while ((readl(PRCM_SEM) & PRCM_SEM_PRCM_SEM) != 0)
    cpu_relax();
    val = readl(prcmu_base + clk_mgt[clock].offset);
    if (enable) {
    val |= (PRCM_CLK_MGT_CLKEN | clk_mgt[clock].pllsw);
    } else {
    clk_mgt[clock].pllsw = (val & PRCM_CLK_MGT_CLKPLLSW_MASK);
    val &= ~(PRCM_CLK_MGT_CLKEN | PRCM_CLK_MGT_CLKPLLSW_MASK);
    }
    writel(val, prcmu_base + clk_mgt[clock].offset);
// Release the HW semaphore.
    writel(0, PRCM_SEM);
    spin_unlock_irqrestore(&clk_mgt_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn request_sga_clock(clock: u8, enable: bool) -> c_int {
    static int request_sga_clock(u8 clock, bool enable)
    {
    u32 val;
    int ret;
    if (enable) {
    val = readl(PRCM_CGATING_BYPASS);
    writel(val | PRCM_CGATING_BYPASS_ICN2, PRCM_CGATING_BYPASS);
    }
    ret = request_clock(clock, enable);
    if (!ret && !enable) {
    val = readl(PRCM_CGATING_BYPASS);
    writel(val & ~PRCM_CGATING_BYPASS_ICN2, PRCM_CGATING_BYPASS);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn plldsi_locked() -> bool {
    static inline bool plldsi_locked(void)
    {
    return (readl(PRCM_PLLDSI_LOCKP) &
    (PRCM_PLLDSI_LOCKP_PRCM_PLLDSI_LOCKP10 |
    PRCM_PLLDSI_LOCKP_PRCM_PLLDSI_LOCKP3)) ==
    (PRCM_PLLDSI_LOCKP_PRCM_PLLDSI_LOCKP10 |
    PRCM_PLLDSI_LOCKP_PRCM_PLLDSI_LOCKP3);
    }
#[no_mangle]
unsafe extern "C" fn request_plldsi(enable: bool) -> c_int {
    static int request_plldsi(bool enable)
    {
    let mut r: c_int = 0;
    u32 val;
    writel((PRCM_MMIP_LS_CLAMP_DSIPLL_CLAMP |
    PRCM_MMIP_LS_CLAMP_DSIPLL_CLAMPI), (enable ?
    PRCM_MMIP_LS_CLAMP_CLR : PRCM_MMIP_LS_CLAMP_SET));
    val = readl(PRCM_PLLDSI_ENABLE);
    if (enable)
    val |= PRCM_PLLDSI_ENABLE_PRCM_PLLDSI_ENABLE;
    else
    val &= ~PRCM_PLLDSI_ENABLE_PRCM_PLLDSI_ENABLE;
    writel(val, PRCM_PLLDSI_ENABLE);
    if (enable) {
    unsigned int i;
    let mut locked: bool = plldsi_locked();
    for (i = 10; !locked && (i > 0); --i) {
    udelay(100);
    locked = plldsi_locked();
    }
    if (locked) {
    writel(PRCM_APE_RESETN_DSIPLL_RESETN,
    PRCM_APE_RESETN_SET);
    } else {
    writel((PRCM_MMIP_LS_CLAMP_DSIPLL_CLAMP |
    PRCM_MMIP_LS_CLAMP_DSIPLL_CLAMPI),
    PRCM_MMIP_LS_CLAMP_SET);
    val &= ~PRCM_PLLDSI_ENABLE_PRCM_PLLDSI_ENABLE;
    writel(val, PRCM_PLLDSI_ENABLE);
    r = -EAGAIN;
    }
    } else {
    writel(PRCM_APE_RESETN_DSIPLL_RESETN, PRCM_APE_RESETN_CLR);
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn request_dsiclk(n: u8, enable: bool) -> c_int {
    static int request_dsiclk(u8 n, bool enable)
    {
    u32 val;
    val = readl(PRCM_DSI_PLLOUT_SEL);
    val &= ~dsiclk[n].divsel_mask;
    val |= ((enable ? dsiclk[n].divsel : PRCM_DSI_PLLOUT_SEL_OFF) <<
    dsiclk[n].divsel_shift);
    writel(val, PRCM_DSI_PLLOUT_SEL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn request_dsiescclk(n: u8, enable: bool) -> c_int {
    static int request_dsiescclk(u8 n, bool enable)
    {
    u32 val;
    val = readl(PRCM_DSITVCLK_DIV);
    enable ? (val |= dsiescclk[n].en) : (val &= ~dsiescclk[n].en);
    writel(val, PRCM_DSITVCLK_DIV);
    return 0;
    }
//
// db8500_prcmu_request_clock() - Request for a clock to be enabled or disabled.
// @clock:      The clock for which the request is made.
// @enable:     Whether the clock should be enabled (true) or disabled (false).
//
// This function should only be used by the clock implementation.
// Do not use it from any other place!
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_request_clock(clock: u8, enable: bool) -> c_int {
    int db8500_prcmu_request_clock(u8 clock, bool enable)
    {
    if (clock == PRCMU_SGACLK)
    return request_sga_clock(clock, enable);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_NUM_REG_CLOCKS: clock <) -> else {
    else if (clock < PRCMU_NUM_REG_CLOCKS)
    return request_clock(clock, enable);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_TIMCLK: clock ==) -> else {
    else if (clock == PRCMU_TIMCLK)
    return request_timclk(enable);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_DSI1CLK): (clock == PRCMU_DSI0CLK) || (clock ==) -> else {
    else if ((clock == PRCMU_DSI0CLK) || (clock == PRCMU_DSI1CLK))
    return request_dsiclk((clock - PRCMU_DSI0CLK), enable);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_DSI2ESCCLK): (PRCMU_DSI0ESCCLK <= clock) && (clock <=) -> else {
    else if ((PRCMU_DSI0ESCCLK <= clock) && (clock <= PRCMU_DSI2ESCCLK))
    return request_dsiescclk((clock - PRCMU_DSI0ESCCLK), enable);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_PLLDSI: clock ==) -> else {
    else if (clock == PRCMU_PLLDSI)
    return request_plldsi(enable);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_SYSCLK: clock ==) -> else {
    else if (clock == PRCMU_SYSCLK)
    return request_sysclk(enable);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_PLLSOC1): (clock == PRCMU_PLLSOC0) || (clock ==) -> else {
    else if ((clock == PRCMU_PLLSOC0) || (clock == PRCMU_PLLSOC1))
    return request_pll(clock, enable);
    else
    return -EINVAL;
    }
    static unsigned long pll_rate(void __iomem *reg, unsigned long src_rate,
    int branch)
    {
    u64 rate;
    u32 val;
    u32 d;
    let mut div: u32 = 1;
    val = readl(reg);
    rate = src_rate;
    rate *= ((val & PRCM_PLL_FREQ_D_MASK) >> PRCM_PLL_FREQ_D_SHIFT);
    d = ((val & PRCM_PLL_FREQ_N_MASK) >> PRCM_PLL_FREQ_N_SHIFT);
    if (d > 1)
    div *= d;
    d = ((val & PRCM_PLL_FREQ_R_MASK) >> PRCM_PLL_FREQ_R_SHIFT);
    if (d > 1)
    div *= d;
    if (val & PRCM_PLL_FREQ_SELDIV2)
    div *= 2;
    if ((branch == PLL_FIX) || ((branch == PLL_DIV) &&
    (val & PRCM_PLL_FREQ_DIV2EN) &&
    ((reg == PRCM_PLLSOC0_FREQ) ||
    (reg == PRCM_PLLARM_FREQ) ||
    (reg == PRCM_PLLDDR_FREQ))))
    div *= 2;
    (void)do_div(rate, div);
    return (unsigned long)rate;
    }
pub const ROOT_CLOCK_RATE: c_int = 38400000;
#[no_mangle]
unsafe extern "C" fn clock_rate(clock: u8) -> c_ulong {
    static unsigned long clock_rate(u8 clock)
    {
    u32 val;
    u32 pllsw;
    let mut rate: c_ulong = ROOT_CLOCK_RATE;
    val = readl(prcmu_base + clk_mgt[clock].offset);
    if (val & PRCM_CLK_MGT_CLK38) {
    if (clk_mgt[clock].clk38div && (val & PRCM_CLK_MGT_CLK38DIV))
    rate /= 2;
    return rate;
    }
    val |= clk_mgt[clock].pllsw;
    pllsw = (val & PRCM_CLK_MGT_CLKPLLSW_MASK);
    if (pllsw == PRCM_CLK_MGT_CLKPLLSW_SOC0)
    rate = pll_rate(PRCM_PLLSOC0_FREQ, rate, clk_mgt[clock].branch);
#[no_mangle]
pub unsafe extern "C" fn if(PRCM_CLK_MGT_CLKPLLSW_SOC1: pllsw ==) -> else {
    else if (pllsw == PRCM_CLK_MGT_CLKPLLSW_SOC1)
    rate = pll_rate(PRCM_PLLSOC1_FREQ, rate, clk_mgt[clock].branch);
#[no_mangle]
pub unsafe extern "C" fn if(PRCM_CLK_MGT_CLKPLLSW_DDR: pllsw ==) -> else {
    else if (pllsw == PRCM_CLK_MGT_CLKPLLSW_DDR)
    rate = pll_rate(PRCM_PLLDDR_FREQ, rate, clk_mgt[clock].branch);
    else
    return 0;
    if ((clock == PRCMU_SGACLK) &&
    (val & PRCM_SGACLK_MGT_SGACLKDIV_BY_2_5_EN)) {
    let mut r: u64 = (rate * 10);
    (void)do_div(r, 25);
    return (unsigned long)r;
    }
    val &= PRCM_CLK_MGT_CLKPLLDIV_MASK;
    if (val)
    return rate / val;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armss_rate() -> c_ulong {
    static unsigned long armss_rate(void)
    {
    u32 r;
    unsigned long rate;
    r = readl(PRCM_ARM_CHGCLKREQ);
    if (r & PRCM_ARM_CHGCLKREQ_PRCM_ARM_CHGCLKREQ) {
// External ARMCLKFIX clock
    rate = pll_rate(PRCM_PLLDDR_FREQ, ROOT_CLOCK_RATE, PLL_FIX);
// Check PRCM_ARM_CHGCLKREQ divider
    if (!(r & PRCM_ARM_CHGCLKREQ_PRCM_ARM_DIVSEL))
    rate /= 2;
// Check PRCM_ARMCLKFIX_MGT divider
    r = readl(PRCM_ARMCLKFIX_MGT);
    r &= PRCM_CLK_MGT_CLKPLLDIV_MASK;
    rate /= r;
    } else {/* ARM PLL */
    rate = pll_rate(PRCM_PLLARM_FREQ, ROOT_CLOCK_RATE, PLL_DIV);
    }
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn dsiclk_rate(n: u8) -> c_ulong {
    static unsigned long dsiclk_rate(u8 n)
    {
    u32 divsel;
    let mut div: u32 = 1;
    divsel = readl(PRCM_DSI_PLLOUT_SEL);
    divsel = ((divsel & dsiclk[n].divsel_mask) >> dsiclk[n].divsel_shift);
    if (divsel == PRCM_DSI_PLLOUT_SEL_OFF)
    divsel = dsiclk[n].divsel;
    else
    dsiclk[n].divsel = divsel;
    switch (divsel) {
    case PRCM_DSI_PLLOUT_SEL_PHI_4:
    div *= 2;
    fallthrough;
    case PRCM_DSI_PLLOUT_SEL_PHI_2:
    div *= 2;
    fallthrough;
    case PRCM_DSI_PLLOUT_SEL_PHI:
    return pll_rate(PRCM_PLLDSI_FREQ, clock_rate(PRCMU_HDMICLK),
    PLL_RAW) / div;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn dsiescclk_rate(n: u8) -> c_ulong {
    static unsigned long dsiescclk_rate(u8 n)
    {
    u32 div;
    div = readl(PRCM_DSITVCLK_DIV);
    div = ((div & dsiescclk[n].div_mask) >> (dsiescclk[n].div_shift));
    return clock_rate(PRCMU_TVCLK) / max((u32)1, div);
    }
#[no_mangle]
pub unsafe extern "C" fn prcmu_clock_rate(clock: u8) -> c_ulong {
    unsigned long prcmu_clock_rate(u8 clock)
    {
    if (clock < PRCMU_NUM_REG_CLOCKS)
    return clock_rate(clock);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_TIMCLK: clock ==) -> else {
    else if (clock == PRCMU_TIMCLK)
    return prcmu_is_ulppll_disabled() ?
    32768 : ROOT_CLOCK_RATE / 16;
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_SYSCLK: clock ==) -> else {
    else if (clock == PRCMU_SYSCLK)
    return ROOT_CLOCK_RATE;
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_PLLSOC0: clock ==) -> else {
    else if (clock == PRCMU_PLLSOC0)
    return pll_rate(PRCM_PLLSOC0_FREQ, ROOT_CLOCK_RATE, PLL_RAW);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_PLLSOC1: clock ==) -> else {
    else if (clock == PRCMU_PLLSOC1)
    return pll_rate(PRCM_PLLSOC1_FREQ, ROOT_CLOCK_RATE, PLL_RAW);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_ARMSS: clock ==) -> else {
    else if (clock == PRCMU_ARMSS)
    return armss_rate();
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_PLLDDR: clock ==) -> else {
    else if (clock == PRCMU_PLLDDR)
    return pll_rate(PRCM_PLLDDR_FREQ, ROOT_CLOCK_RATE, PLL_RAW);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_PLLDSI: clock ==) -> else {
    else if (clock == PRCMU_PLLDSI)
    return pll_rate(PRCM_PLLDSI_FREQ, clock_rate(PRCMU_HDMICLK),
    PLL_RAW);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_DSI1CLK): (clock == PRCMU_DSI0CLK) || (clock ==) -> else {
    else if ((clock == PRCMU_DSI0CLK) || (clock == PRCMU_DSI1CLK))
    return dsiclk_rate(clock - PRCMU_DSI0CLK);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_DSI2ESCCLK): (PRCMU_DSI0ESCCLK <= clock) && (clock <=) -> else {
    else if ((PRCMU_DSI0ESCCLK <= clock) && (clock <= PRCMU_DSI2ESCCLK))
    return dsiescclk_rate(clock - PRCMU_DSI0ESCCLK);
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clock_source_rate(clk_mgt_val: u32, branch: c_int) -> c_ulong {
    static unsigned long clock_source_rate(u32 clk_mgt_val, int branch)
    {
    if (clk_mgt_val & PRCM_CLK_MGT_CLK38)
    return ROOT_CLOCK_RATE;
    clk_mgt_val &= PRCM_CLK_MGT_CLKPLLSW_MASK;
    if (clk_mgt_val == PRCM_CLK_MGT_CLKPLLSW_SOC0)
    return pll_rate(PRCM_PLLSOC0_FREQ, ROOT_CLOCK_RATE, branch);
#[no_mangle]
pub unsafe extern "C" fn if(PRCM_CLK_MGT_CLKPLLSW_SOC1: clk_mgt_val ==) -> else {
    else if (clk_mgt_val == PRCM_CLK_MGT_CLKPLLSW_SOC1)
    return pll_rate(PRCM_PLLSOC1_FREQ, ROOT_CLOCK_RATE, branch);
#[no_mangle]
pub unsafe extern "C" fn if(PRCM_CLK_MGT_CLKPLLSW_DDR: clk_mgt_val ==) -> else {
    else if (clk_mgt_val == PRCM_CLK_MGT_CLKPLLSW_DDR)
    return pll_rate(PRCM_PLLDDR_FREQ, ROOT_CLOCK_RATE, branch);
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clock_divider(src_rate: c_ulong, rate: c_ulong) -> u32 {
    static u32 clock_divider(unsigned long src_rate, unsigned long rate)
    {
    u32 div;
    div = (src_rate / rate);
    if (div == 0)
    return 1;
    if (rate < (src_rate / div))
    div++;
    return div;
    }
#[no_mangle]
unsafe extern "C" fn round_clock_rate(clock: u8, rate: c_ulong) -> c_long {
    static long round_clock_rate(u8 clock, unsigned long rate)
    {
    u32 val;
    u32 div;
    unsigned long src_rate;
    long rounded_rate;
    val = readl(prcmu_base + clk_mgt[clock].offset);
    src_rate = clock_source_rate((val | clk_mgt[clock].pllsw),
    clk_mgt[clock].branch);
    div = clock_divider(src_rate, rate);
    if (val & PRCM_CLK_MGT_CLK38) {
    if (clk_mgt[clock].clk38div) {
    if (div > 2)
    div = 2;
    } else {
    div = 1;
    }
    } else if ((clock == PRCMU_SGACLK) && (div == 3)) {
    let mut r: u64 = (src_rate * 10);
    (void)do_div(r, 25);
    if (r <= rate)
    return (unsigned long)r;
    }
    rounded_rate = (src_rate / min(div, (u32)31));
    return rounded_rate;
    }
    static const unsigned long db8500_armss_freqs[] = {
    199680000,
    399360000,
    798720000,
    998400000
    };
// The DB8520 has slightly higher ARMSS max frequency
    static const unsigned long db8520_armss_freqs[] = {
    199680000,
    399360000,
    798720000,
    1152000000
    };
#[no_mangle]
unsafe extern "C" fn round_armss_rate(rate: c_ulong) -> c_long {
    static long round_armss_rate(unsigned long rate)
    {
    let mut freq: c_ulong = 0;
    const unsigned long *freqs;
    int nfreqs;
    int i;
    if (fw_info.version.project == PRCMU_FW_PROJECT_U8520) {
    freqs = db8520_armss_freqs;
    nfreqs = ARRAY_SIZE(db8520_armss_freqs);
    } else {
    freqs = db8500_armss_freqs;
    nfreqs = ARRAY_SIZE(db8500_armss_freqs);
    }
// Find the corresponding arm opp from the cpufreq table.
    for (i = 0; i < nfreqs; i++) {
    freq = freqs[i];
    if (rate <= freq)
    break;
    }
// Return the last valid value, even if a match was not found.
    return freq;
    }

#[no_mangle]
unsafe extern "C" fn round_plldsi_rate(rate: c_ulong) -> c_long {
    static long round_plldsi_rate(unsigned long rate)
    {
    let mut rounded_rate: c_long = 0;
    unsigned long src_rate;
    unsigned long rem;
    u32 r;
    src_rate = clock_rate(PRCMU_HDMICLK);
    rem = rate;
    for (r = 7; (rem > 0) && (r > 0); r--) {
    u64 d;
    d = (r * rate);
    (void)do_div(d, src_rate);
    if (d < 6)
    d = 6;
#[no_mangle]
pub unsafe extern "C" fn if(255: d >) -> else {
    else if (d > 255)
    d = 255;
    d *= src_rate;
    if (((2 * d) < (r * MIN_PLL_VCO_RATE)) ||
    ((r * MAX_PLL_VCO_RATE) < (2 * d)))
    continue;
    (void)do_div(d, r);
    if (rate < d) {
    if (rounded_rate == 0)
    rounded_rate = (long)d;
    break;
    }
    if ((rate - d) < rem) {
    rem = (rate - d);
    rounded_rate = (long)d;
    }
    }
    return rounded_rate;
    }
#[no_mangle]
unsafe extern "C" fn round_dsiclk_rate(rate: c_ulong) -> c_long {
    static long round_dsiclk_rate(unsigned long rate)
    {
    u32 div;
    unsigned long src_rate;
    long rounded_rate;
    src_rate = pll_rate(PRCM_PLLDSI_FREQ, clock_rate(PRCMU_HDMICLK),
    PLL_RAW);
    div = clock_divider(src_rate, rate);
    rounded_rate = (src_rate / ((div > 2) ? 4 : div));
    return rounded_rate;
    }
#[no_mangle]
unsafe extern "C" fn round_dsiescclk_rate(rate: c_ulong) -> c_long {
    static long round_dsiescclk_rate(unsigned long rate)
    {
    u32 div;
    unsigned long src_rate;
    long rounded_rate;
    src_rate = clock_rate(PRCMU_TVCLK);
    div = clock_divider(src_rate, rate);
    rounded_rate = (src_rate / min(div, (u32)255));
    return rounded_rate;
    }
#[no_mangle]
pub unsafe extern "C" fn prcmu_round_clock_rate(clock: u8, rate: c_ulong) -> c_long {
    long prcmu_round_clock_rate(u8 clock, unsigned long rate)
    {
    if (clock < PRCMU_NUM_REG_CLOCKS)
    return round_clock_rate(clock, rate);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_ARMSS: clock ==) -> else {
    else if (clock == PRCMU_ARMSS)
    return round_armss_rate(rate);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_PLLDSI: clock ==) -> else {
    else if (clock == PRCMU_PLLDSI)
    return round_plldsi_rate(rate);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_DSI1CLK): (clock == PRCMU_DSI0CLK) || (clock ==) -> else {
    else if ((clock == PRCMU_DSI0CLK) || (clock == PRCMU_DSI1CLK))
    return round_dsiclk_rate(rate);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_DSI2ESCCLK): (PRCMU_DSI0ESCCLK <= clock) && (clock <=) -> else {
    else if ((PRCMU_DSI0ESCCLK <= clock) && (clock <= PRCMU_DSI2ESCCLK))
    return round_dsiescclk_rate(rate);
    else
    return (long)prcmu_clock_rate(clock);
    }
#[no_mangle]
unsafe extern "C" fn set_clock_rate(clock: u8, rate: c_ulong) {
    static void set_clock_rate(u8 clock, unsigned long rate)
    {
    u32 val;
    u32 div;
    unsigned long src_rate;
    unsigned long flags;
    spin_lock_irqsave(&clk_mgt_lock, flags);
// Grab the HW semaphore.
    while ((readl(PRCM_SEM) & PRCM_SEM_PRCM_SEM) != 0)
    cpu_relax();
    val = readl(prcmu_base + clk_mgt[clock].offset);
    src_rate = clock_source_rate((val | clk_mgt[clock].pllsw),
    clk_mgt[clock].branch);
    div = clock_divider(src_rate, rate);
    if (val & PRCM_CLK_MGT_CLK38) {
    if (clk_mgt[clock].clk38div) {
    if (div > 1)
    val |= PRCM_CLK_MGT_CLK38DIV;
    else
    val &= ~PRCM_CLK_MGT_CLK38DIV;
    }
    } else if (clock == PRCMU_SGACLK) {
    val &= ~(PRCM_CLK_MGT_CLKPLLDIV_MASK |
    PRCM_SGACLK_MGT_SGACLKDIV_BY_2_5_EN);
    if (div == 3) {
    let mut r: u64 = (src_rate * 10);
    (void)do_div(r, 25);
    if (r <= rate) {
    val |= PRCM_SGACLK_MGT_SGACLKDIV_BY_2_5_EN;
    div = 0;
    }
    }
    val |= min(div, (u32)31);
    } else {
    val &= ~PRCM_CLK_MGT_CLKPLLDIV_MASK;
    val |= min(div, (u32)31);
    }
    writel(val, prcmu_base + clk_mgt[clock].offset);
// Release the HW semaphore.
    writel(0, PRCM_SEM);
    spin_unlock_irqrestore(&clk_mgt_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn set_armss_rate(rate: c_ulong) -> c_int {
    static int set_armss_rate(unsigned long rate)
    {
    unsigned long freq;
    u8 opps[] = { ARM_EXTCLK, ARM_50_OPP, ARM_100_OPP, ARM_MAX_OPP };
    const unsigned long *freqs;
    int nfreqs;
    int i;
    if (fw_info.version.project == PRCMU_FW_PROJECT_U8520) {
    freqs = db8520_armss_freqs;
    nfreqs = ARRAY_SIZE(db8520_armss_freqs);
    } else {
    freqs = db8500_armss_freqs;
    nfreqs = ARRAY_SIZE(db8500_armss_freqs);
    }
// Find the corresponding arm opp from the cpufreq table.
    for (i = 0; i < nfreqs; i++) {
    freq = freqs[i];
    if (rate == freq)
    break;
    }
    if (rate != freq)
    return -EINVAL;
// Set the new arm opp.
    pr_debug("SET ARM OPP 0x%02x\n", opps[i]);
    return db8500_prcmu_set_arm_opp(opps[i]);
    }
#[no_mangle]
unsafe extern "C" fn set_plldsi_rate(rate: c_ulong) -> c_int {
    static int set_plldsi_rate(unsigned long rate)
    {
    unsigned long src_rate;
    unsigned long rem;
    let mut pll_freq: u32 = 0;
    u32 r;
    src_rate = clock_rate(PRCMU_HDMICLK);
    rem = rate;
    for (r = 7; (rem > 0) && (r > 0); r--) {
    u64 d;
    u64 hwrate;
    d = (r * rate);
    (void)do_div(d, src_rate);
    if (d < 6)
    d = 6;
#[no_mangle]
pub unsafe extern "C" fn if(255: d >) -> else {
    else if (d > 255)
    d = 255;
    hwrate = (d * src_rate);
    if (((2 * hwrate) < (r * MIN_PLL_VCO_RATE)) ||
    ((r * MAX_PLL_VCO_RATE) < (2 * hwrate)))
    continue;
    (void)do_div(hwrate, r);
    if (rate < hwrate) {
    if (pll_freq == 0)
    pll_freq = (((u32)d << PRCM_PLL_FREQ_D_SHIFT) |
    (r << PRCM_PLL_FREQ_R_SHIFT));
    break;
    }
    if ((rate - hwrate) < rem) {
    rem = (rate - hwrate);
    pll_freq = (((u32)d << PRCM_PLL_FREQ_D_SHIFT) |
    (r << PRCM_PLL_FREQ_R_SHIFT));
    }
    }
    if (pll_freq == 0)
    return -EINVAL;
    pll_freq |= (1 << PRCM_PLL_FREQ_N_SHIFT);
    writel(pll_freq, PRCM_PLLDSI_FREQ);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_dsiclk_rate(n: u8, rate: c_ulong) {
    static void set_dsiclk_rate(u8 n, unsigned long rate)
    {
    u32 val;
    u32 div;
    div = clock_divider(pll_rate(PRCM_PLLDSI_FREQ,
    clock_rate(PRCMU_HDMICLK), PLL_RAW), rate);
    dsiclk[n].divsel = (div == 1) ? PRCM_DSI_PLLOUT_SEL_PHI :
    (div == 2) ? PRCM_DSI_PLLOUT_SEL_PHI_2 :
// else */	PRCM_DSI_PLLOUT_SEL_PHI_4;
    val = readl(PRCM_DSI_PLLOUT_SEL);
    val &= ~dsiclk[n].divsel_mask;
    val |= (dsiclk[n].divsel << dsiclk[n].divsel_shift);
    writel(val, PRCM_DSI_PLLOUT_SEL);
    }
#[no_mangle]
unsafe extern "C" fn set_dsiescclk_rate(n: u8, rate: c_ulong) {
    static void set_dsiescclk_rate(u8 n, unsigned long rate)
    {
    u32 val;
    u32 div;
    div = clock_divider(clock_rate(PRCMU_TVCLK), rate);
    val = readl(PRCM_DSITVCLK_DIV);
    val &= ~dsiescclk[n].div_mask;
    val |= (min(div, (u32)255) << dsiescclk[n].div_shift);
    writel(val, PRCM_DSITVCLK_DIV);
    }
#[no_mangle]
pub unsafe extern "C" fn prcmu_set_clock_rate(clock: u8, rate: c_ulong) -> c_int {
    int prcmu_set_clock_rate(u8 clock, unsigned long rate)
    {
    if (clock < PRCMU_NUM_REG_CLOCKS)
    set_clock_rate(clock, rate);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_ARMSS: clock ==) -> else {
    else if (clock == PRCMU_ARMSS)
    return set_armss_rate(rate);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_PLLDSI: clock ==) -> else {
    else if (clock == PRCMU_PLLDSI)
    return set_plldsi_rate(rate);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_DSI1CLK): (clock == PRCMU_DSI0CLK) || (clock ==) -> else {
    else if ((clock == PRCMU_DSI0CLK) || (clock == PRCMU_DSI1CLK))
    set_dsiclk_rate((clock - PRCMU_DSI0CLK), rate);
#[no_mangle]
pub unsafe extern "C" fn if(PRCMU_DSI2ESCCLK): (PRCMU_DSI0ESCCLK <= clock) && (clock <=) -> else {
    else if ((PRCMU_DSI0ESCCLK <= clock) && (clock <= PRCMU_DSI2ESCCLK))
    set_dsiescclk_rate((clock - PRCMU_DSI0ESCCLK), rate);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_config_esram0_deep_sleep(state: u8) -> c_int {
    int db8500_prcmu_config_esram0_deep_sleep(u8 state)
    {
    if ((state > ESRAM0_DEEP_SLEEP_STATE_RET) ||
    (state < ESRAM0_DEEP_SLEEP_STATE_OFF))
    return -EINVAL;
    mutex_lock(&mb4_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(4))
    cpu_relax();
    writeb(MB4H_MEM_ST, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB4));
    writeb(((DDR_PWR_STATE_OFFHIGHLAT << 4) | DDR_PWR_STATE_ON),
    (tcdm_base + PRCM_REQ_MB4_DDR_ST_AP_SLEEP_IDLE));
    writeb(DDR_PWR_STATE_ON,
    (tcdm_base + PRCM_REQ_MB4_DDR_ST_AP_DEEP_IDLE));
    writeb(state, (tcdm_base + PRCM_REQ_MB4_ESRAM0_ST));
    writel(MBOX_BIT(4), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb4_transfer.work);
    mutex_unlock(&mb4_transfer.lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_config_hotdog(threshold: u8) -> c_int {
    int db8500_prcmu_config_hotdog(u8 threshold)
    {
    mutex_lock(&mb4_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(4))
    cpu_relax();
    writeb(threshold, (tcdm_base + PRCM_REQ_MB4_HOTDOG_THRESHOLD));
    writeb(MB4H_HOTDOG, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB4));
    writel(MBOX_BIT(4), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb4_transfer.work);
    mutex_unlock(&mb4_transfer.lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_config_hotmon(low: u8, high: u8) -> c_int {
    int db8500_prcmu_config_hotmon(u8 low, u8 high)
    {
    mutex_lock(&mb4_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(4))
    cpu_relax();
    writeb(low, (tcdm_base + PRCM_REQ_MB4_HOTMON_LOW));
    writeb(high, (tcdm_base + PRCM_REQ_MB4_HOTMON_HIGH));
    writeb((HOTMON_CONFIG_LOW | HOTMON_CONFIG_HIGH),
    (tcdm_base + PRCM_REQ_MB4_HOTMON_CONFIG));
    writeb(MB4H_HOTMON, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB4));
    writel(MBOX_BIT(4), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb4_transfer.work);
    mutex_unlock(&mb4_transfer.lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(db8500_prcmu_config_hotmon);
#[no_mangle]
unsafe extern "C" fn config_hot_period(val: u16) -> c_int {
    static int config_hot_period(u16 val)
    {
    mutex_lock(&mb4_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(4))
    cpu_relax();
    writew(val, (tcdm_base + PRCM_REQ_MB4_HOT_PERIOD));
    writeb(MB4H_HOT_PERIOD, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB4));
    writel(MBOX_BIT(4), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb4_transfer.work);
    mutex_unlock(&mb4_transfer.lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_start_temp_sense(cycles32k: u16) -> c_int {
    int db8500_prcmu_start_temp_sense(u16 cycles32k)
    {
    if (cycles32k == 0xFFFF)
    return -EINVAL;
    return config_hot_period(cycles32k);
    }
    EXPORT_SYMBOL_GPL(db8500_prcmu_start_temp_sense);
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_stop_temp_sense() -> c_int {
    int db8500_prcmu_stop_temp_sense(void)
    {
    return config_hot_period(0xFFFF);
    }
    EXPORT_SYMBOL_GPL(db8500_prcmu_stop_temp_sense);
#[no_mangle]
unsafe extern "C" fn prcmu_a9wdog(cmd: u8, d0: u8, d1: u8, d2: u8, d3: u8) -> c_int {
    static int prcmu_a9wdog(u8 cmd, u8 d0, u8 d1, u8 d2, u8 d3)
    {
    mutex_lock(&mb4_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(4))
    cpu_relax();
    writeb(d0, (tcdm_base + PRCM_REQ_MB4_A9WDOG_0));
    writeb(d1, (tcdm_base + PRCM_REQ_MB4_A9WDOG_1));
    writeb(d2, (tcdm_base + PRCM_REQ_MB4_A9WDOG_2));
    writeb(d3, (tcdm_base + PRCM_REQ_MB4_A9WDOG_3));
    writeb(cmd, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB4));
    writel(MBOX_BIT(4), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb4_transfer.work);
    mutex_unlock(&mb4_transfer.lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_config_a9wdog(num: u8, sleep_auto_off: bool) -> c_int {
    int db8500_prcmu_config_a9wdog(u8 num, bool sleep_auto_off)
    {
    BUG_ON(num == 0 || num > 0xf);
    return prcmu_a9wdog(MB4H_A9WDOG_CONF, num, 0, 0,
    sleep_auto_off ? A9WDOG_AUTO_OFF_EN :
    A9WDOG_AUTO_OFF_DIS);
    }
    EXPORT_SYMBOL(db8500_prcmu_config_a9wdog);
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_enable_a9wdog(id: u8) -> c_int {
    int db8500_prcmu_enable_a9wdog(u8 id)
    {
    return prcmu_a9wdog(MB4H_A9WDOG_EN, id, 0, 0, 0);
    }
    EXPORT_SYMBOL(db8500_prcmu_enable_a9wdog);
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_disable_a9wdog(id: u8) -> c_int {
    int db8500_prcmu_disable_a9wdog(u8 id)
    {
    return prcmu_a9wdog(MB4H_A9WDOG_DIS, id, 0, 0, 0);
    }
    EXPORT_SYMBOL(db8500_prcmu_disable_a9wdog);
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_kick_a9wdog(id: u8) -> c_int {
    int db8500_prcmu_kick_a9wdog(u8 id)
    {
    return prcmu_a9wdog(MB4H_A9WDOG_KICK, id, 0, 0, 0);
    }
    EXPORT_SYMBOL(db8500_prcmu_kick_a9wdog);
//
// timeout is 28 bit, in ms.
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_load_a9wdog(id: u8, timeout: u32) -> c_int {
    int db8500_prcmu_load_a9wdog(u8 id, u32 timeout)
    {
    return prcmu_a9wdog(MB4H_A9WDOG_LOAD,
    (id & A9WDOG_ID_MASK) |
//
// Put the lowest 28 bits of timeout at
// offset 4. Four first bits are used for id.
//
    (u8)((timeout << 4) & 0xf0),
    (u8)((timeout >> 4) & 0xff),
    (u8)((timeout >> 12) & 0xff),
    (u8)((timeout >> 20) & 0xff));
    }
    EXPORT_SYMBOL(db8500_prcmu_load_a9wdog);
//
// prcmu_abb_read() - Read register value(s) from the ABB.
// @slave:	The I2C slave address.
// @reg:	The (start) register address.
// @value:	The read out value(s).
// @size:	The number of registers to read.
//
// Reads register value(s) from the ABB.
// @size has to be 1 for the current firmware version.
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_abb_read(slave: u8, reg: u8, value: *mut u8, size: u8) -> c_int {
    int prcmu_abb_read(u8 slave, u8 reg, u8 *value, u8 size)
    {
    int r;
    if (size != 1)
    return -EINVAL;
    mutex_lock(&mb5_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(5))
    cpu_relax();
    writeb(0, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB5));
    writeb(PRCMU_I2C_READ(slave), (tcdm_base + PRCM_REQ_MB5_I2C_SLAVE_OP));
    writeb(PRCMU_I2C_STOP_EN, (tcdm_base + PRCM_REQ_MB5_I2C_HW_BITS));
    writeb(reg, (tcdm_base + PRCM_REQ_MB5_I2C_REG));
    writeb(0, (tcdm_base + PRCM_REQ_MB5_I2C_VAL));
    writel(MBOX_BIT(5), PRCM_MBOX_CPU_SET);
    if (!wait_for_completion_timeout(&mb5_transfer.work,
    msecs_to_jiffies(20000))) {
    pr_err("prcmu: %s timed out (20 s) waiting for a reply.\n",
    __func__);
    r = -EIO;
    } else {
    r = ((mb5_transfer.ack.status == I2C_RD_OK) ? 0 : -EIO);
    }
    if (!r)
// value = mb5_transfer.ack.value;
    mutex_unlock(&mb5_transfer.lock);
    return r;
    }
//
// prcmu_abb_write_masked() - Write masked register value(s) to the ABB.
// @slave:	The I2C slave address.
// @reg:	The (start) register address.
// @value:	The value(s) to write.
// @mask:	The mask(s) to use.
// @size:	The number of registers to write.
//
// Writes masked register value(s) to the ABB.
// For each @value, only the bits set to 1 in the corresponding @mask
// will be written. The other bits are not changed.
// @size has to be 1 for the current firmware version.
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_abb_write_masked(slave: u8, reg: u8, value: *mut u8, mask: *mut u8, size: u8) -> c_int {
    int prcmu_abb_write_masked(u8 slave, u8 reg, u8 *value, u8 *mask, u8 size)
    {
    int r;
    if (size != 1)
    return -EINVAL;
    mutex_lock(&mb5_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(5))
    cpu_relax();
    writeb(~*mask, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB5));
    writeb(PRCMU_I2C_WRITE(slave), (tcdm_base + PRCM_REQ_MB5_I2C_SLAVE_OP));
    writeb(PRCMU_I2C_STOP_EN, (tcdm_base + PRCM_REQ_MB5_I2C_HW_BITS));
    writeb(reg, (tcdm_base + PRCM_REQ_MB5_I2C_REG));
    writeb(*value, (tcdm_base + PRCM_REQ_MB5_I2C_VAL));
    writel(MBOX_BIT(5), PRCM_MBOX_CPU_SET);
    if (!wait_for_completion_timeout(&mb5_transfer.work,
    msecs_to_jiffies(20000))) {
    pr_err("prcmu: %s timed out (20 s) waiting for a reply.\n",
    __func__);
    r = -EIO;
    } else {
    r = ((mb5_transfer.ack.status == I2C_WR_OK) ? 0 : -EIO);
    }
    mutex_unlock(&mb5_transfer.lock);
    return r;
    }
//
// prcmu_abb_write() - Write register value(s) to the ABB.
// @slave:	The I2C slave address.
// @reg:	The (start) register address.
// @value:	The value(s) to write.
// @size:	The number of registers to write.
//
// Writes register value(s) to the ABB.
// @size has to be 1 for the current firmware version.
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_abb_write(slave: u8, reg: u8, value: *mut u8, size: u8) -> c_int {
    int prcmu_abb_write(u8 slave, u8 reg, u8 *value, u8 size)
    {
    let mut mask: u8 = ~0;
    return prcmu_abb_write_masked(slave, reg, value, &mask, size);
    }
//
// prcmu_ac_wake_req - should be called whenever ARM wants to wakeup Modem
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_ac_wake_req() -> c_int {
    int prcmu_ac_wake_req(void)
    {
    u32 val;
    let mut ret: c_int = 0;
    mutex_lock(&mb0_transfer.ac_wake_lock);
    val = readl(PRCM_HOSTACCESS_REQ);
    if (val & PRCM_HOSTACCESS_REQ_HOSTACCESS_REQ)
    goto unlock_and_return;
    atomic_set(&ac_wake_req_state, 1);
//
// Force Modem Wake-up before hostaccess_req ping-pong.
// It prevents Modem to enter in Sleep while acking the hostaccess
// request. The 31us delay has been calculated by HWI.
//
    val |= PRCM_HOSTACCESS_REQ_WAKE_REQ;
    writel(val, PRCM_HOSTACCESS_REQ);
    udelay(31);
    val |= PRCM_HOSTACCESS_REQ_HOSTACCESS_REQ;
    writel(val, PRCM_HOSTACCESS_REQ);
    if (!wait_for_completion_timeout(&mb0_transfer.ac_wake_work,
    msecs_to_jiffies(5000))) {
    pr_crit("prcmu: %s timed out (5 s) waiting for a reply.\n",
    __func__);
    ret = -EFAULT;
    }
    unlock_and_return:
    mutex_unlock(&mb0_transfer.ac_wake_lock);
    return ret;
    }
//
// prcmu_ac_sleep_req - called when ARM no longer needs to talk to modem
//
#[no_mangle]
pub unsafe extern "C" fn prcmu_ac_sleep_req() {
    void prcmu_ac_sleep_req(void)
    {
    u32 val;
    mutex_lock(&mb0_transfer.ac_wake_lock);
    val = readl(PRCM_HOSTACCESS_REQ);
    if (!(val & PRCM_HOSTACCESS_REQ_HOSTACCESS_REQ))
    goto unlock_and_return;
    writel((val & ~PRCM_HOSTACCESS_REQ_HOSTACCESS_REQ),
    PRCM_HOSTACCESS_REQ);
    if (!wait_for_completion_timeout(&mb0_transfer.ac_wake_work,
    msecs_to_jiffies(5000))) {
    pr_crit("prcmu: %s timed out (5 s) waiting for a reply.\n",
    __func__);
    }
    atomic_set(&ac_wake_req_state, 0);
    unlock_and_return:
    mutex_unlock(&mb0_transfer.ac_wake_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_is_ac_wake_requested() -> bool {
    bool db8500_prcmu_is_ac_wake_requested(void)
    {
    return (atomic_read(&ac_wake_req_state) != 0);
    }
//
// db8500_prcmu_system_reset - System reset
//
// Saves the reset reason code and then sets the APE_SOFTRST register which
// fires interrupt to fw
//
// @reset_code: The reason for system reset
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_system_reset(reset_code: u16) {
    void db8500_prcmu_system_reset(u16 reset_code)
    {
    writew(reset_code, (tcdm_base + PRCM_SW_RST_REASON));
    writel(1, PRCM_APE_SOFTRST);
    }
//
// db8500_prcmu_get_reset_code - Retrieve SW reset reason code
//
// Retrieves the reset reason code stored by db8500_prcmu_system_reset() before
// last restart.
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_get_reset_code() -> u16 {
    u16 db8500_prcmu_get_reset_code(void)
    {
    return readw(tcdm_base + PRCM_SW_RST_REASON);
    }
//
// db8500_prcmu_modem_reset - ask the PRCMU to reset modem
//
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_modem_reset() {
    void db8500_prcmu_modem_reset(void)
    {
    mutex_lock(&mb1_transfer.lock);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(1))
    cpu_relax();
    writeb(MB1H_RESET_MODEM, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB1));
    writel(MBOX_BIT(1), PRCM_MBOX_CPU_SET);
    wait_for_completion(&mb1_transfer.work);
//
// No need to check return from PRCMU as modem should go in reset state
// This state is already managed by upper layer
//
    mutex_unlock(&mb1_transfer.lock);
    }
#[no_mangle]
unsafe extern "C" fn ack_dbb_wakeup() {
    static void ack_dbb_wakeup(void)
    {
    unsigned long flags;
    spin_lock_irqsave(&mb0_transfer.lock, flags);
    while (readl(PRCM_MBOX_CPU_VAL) & MBOX_BIT(0))
    cpu_relax();
    writeb(MB0H_READ_WAKEUP_ACK, (tcdm_base + PRCM_MBOX_HEADER_REQ_MB0));
    writel(MBOX_BIT(0), PRCM_MBOX_CPU_SET);
    spin_unlock_irqrestore(&mb0_transfer.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn print_unknown_header_warning(n: u8, header: u8) {
    static inline void print_unknown_header_warning(u8 n, u8 header)
    {
    pr_warn("prcmu: Unknown message header (%d) in mailbox %d\n",
    header, n);
    }
#[no_mangle]
unsafe extern "C" fn read_mailbox_0() -> bool {
    static bool read_mailbox_0(void)
    {
    bool r;
    u32 ev;
    unsigned int n;
    u8 header;
    header = readb(tcdm_base + PRCM_MBOX_HEADER_ACK_MB0);
    switch (header) {
    case MB0H_WAKEUP_EXE:
    case MB0H_WAKEUP_SLEEP:
    if (readb(tcdm_base + PRCM_ACK_MB0_READ_POINTER) & 1)
    ev = readl(tcdm_base + PRCM_ACK_MB0_WAKEUP_1_8500);
    else
    ev = readl(tcdm_base + PRCM_ACK_MB0_WAKEUP_0_8500);
    if (ev & (WAKEUP_BIT_AC_WAKE_ACK | WAKEUP_BIT_AC_SLEEP_ACK))
    complete(&mb0_transfer.ac_wake_work);
    if (ev & WAKEUP_BIT_SYSCLK_OK)
    complete(&mb3_transfer.sysclk_work);
    ev &= mb0_transfer.req.dbb_irqs;
    for (n = 0; n < NUM_PRCMU_WAKEUPS; n++) {
    if (ev & prcmu_irq_bit[n])
    generic_handle_domain_irq(db8500_irq_domain, n);
    }
    r = true;
    break;
    default:
    print_unknown_header_warning(0, header);
    r = false;
    break;
    }
    writel(MBOX_BIT(0), PRCM_ARM_IT1_CLR);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn read_mailbox_1() -> bool {
    static bool read_mailbox_1(void)
    {
    mb1_transfer.ack.header = readb(tcdm_base + PRCM_MBOX_HEADER_REQ_MB1);
    mb1_transfer.ack.arm_opp = readb(tcdm_base +
    PRCM_ACK_MB1_CURRENT_ARM_OPP);
    mb1_transfer.ack.ape_opp = readb(tcdm_base +
    PRCM_ACK_MB1_CURRENT_APE_OPP);
    mb1_transfer.ack.ape_voltage_status = readb(tcdm_base +
    PRCM_ACK_MB1_APE_VOLTAGE_STATUS);
    writel(MBOX_BIT(1), PRCM_ARM_IT1_CLR);
    complete(&mb1_transfer.work);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn read_mailbox_2() -> bool {
    static bool read_mailbox_2(void)
    {
    mb2_transfer.ack.status = readb(tcdm_base + PRCM_ACK_MB2_DPS_STATUS);
    writel(MBOX_BIT(2), PRCM_ARM_IT1_CLR);
    complete(&mb2_transfer.work);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn read_mailbox_3() -> bool {
    static bool read_mailbox_3(void)
    {
    writel(MBOX_BIT(3), PRCM_ARM_IT1_CLR);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn read_mailbox_4() -> bool {
    static bool read_mailbox_4(void)
    {
    u8 header;
    let mut do_complete: bool = true;
    header = readb(tcdm_base + PRCM_MBOX_HEADER_REQ_MB4);
    switch (header) {
    case MB4H_MEM_ST:
    case MB4H_HOTDOG:
    case MB4H_HOTMON:
    case MB4H_HOT_PERIOD:
    case MB4H_A9WDOG_CONF:
    case MB4H_A9WDOG_EN:
    case MB4H_A9WDOG_DIS:
    case MB4H_A9WDOG_LOAD:
    case MB4H_A9WDOG_KICK:
    break;
    default:
    print_unknown_header_warning(4, header);
    do_complete = false;
    break;
    }
    writel(MBOX_BIT(4), PRCM_ARM_IT1_CLR);
    if (do_complete)
    complete(&mb4_transfer.work);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn read_mailbox_5() -> bool {
    static bool read_mailbox_5(void)
    {
    mb5_transfer.ack.status = readb(tcdm_base + PRCM_ACK_MB5_I2C_STATUS);
    mb5_transfer.ack.value = readb(tcdm_base + PRCM_ACK_MB5_I2C_VAL);
    writel(MBOX_BIT(5), PRCM_ARM_IT1_CLR);
    complete(&mb5_transfer.work);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn read_mailbox_6() -> bool {
    static bool read_mailbox_6(void)
    {
    writel(MBOX_BIT(6), PRCM_ARM_IT1_CLR);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn read_mailbox_7() -> bool {
    static bool read_mailbox_7(void)
    {
    writel(MBOX_BIT(7), PRCM_ARM_IT1_CLR);
    return false;
    }
    static bool (* const read_mailbox[NUM_MB])(void) = {
    read_mailbox_0,
    read_mailbox_1,
    read_mailbox_2,
    read_mailbox_3,
    read_mailbox_4,
    read_mailbox_5,
    read_mailbox_6,
    read_mailbox_7
    };
#[no_mangle]
unsafe extern "C" fn prcmu_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t prcmu_irq_handler(int irq, void *data)
    {
    u32 bits;
    u8 n;
    irqreturn_t r;
    bits = (readl(PRCM_ARM_IT1_VAL) & ALL_MBOX_BITS);
    if (unlikely(!bits))
    return IRQ_NONE;
    r = IRQ_HANDLED;
    for (n = 0; bits; n++) {
    if (bits & MBOX_BIT(n)) {
    bits -= MBOX_BIT(n);
    if (read_mailbox[n]())
    r = IRQ_WAKE_THREAD;
    }
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn prcmu_irq_thread_fn(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t prcmu_irq_thread_fn(int irq, void *data)
    {
    ack_dbb_wakeup();
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn prcmu_mask_work(work: *mut work_struct) {
    static void prcmu_mask_work(struct work_struct *work)
    {
    unsigned long flags;
    spin_lock_irqsave(&mb0_transfer.lock, flags);
    config_wakeups();
    spin_unlock_irqrestore(&mb0_transfer.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn prcmu_irq_mask(d: *mut irq_data) {
    static void prcmu_irq_mask(struct irq_data *d)
    {
    unsigned long flags;
    spin_lock_irqsave(&mb0_transfer.dbb_irqs_lock, flags);
    mb0_transfer.req.dbb_irqs &= ~prcmu_irq_bit[d.hwirq];
    spin_unlock_irqrestore(&mb0_transfer.dbb_irqs_lock, flags);
    if (d.irq != IRQ_PRCMU_CA_SLEEP)
    schedule_work(&mb0_transfer.mask_work);
    }
#[no_mangle]
unsafe extern "C" fn prcmu_irq_unmask(d: *mut irq_data) {
    static void prcmu_irq_unmask(struct irq_data *d)
    {
    unsigned long flags;
    spin_lock_irqsave(&mb0_transfer.dbb_irqs_lock, flags);
    mb0_transfer.req.dbb_irqs |= prcmu_irq_bit[d.hwirq];
    spin_unlock_irqrestore(&mb0_transfer.dbb_irqs_lock, flags);
    if (d.irq != IRQ_PRCMU_CA_SLEEP)
    schedule_work(&mb0_transfer.mask_work);
    }
#[no_mangle]
unsafe extern "C" fn noop(d: *mut irq_data) {
    static void noop(struct irq_data *d)
    {
    }
    static struct irq_chip prcmu_irq_chip = {
    .name		= "prcmu",
    .irq_disable	= prcmu_irq_mask,
    .irq_ack	= noop,
    .irq_mask	= prcmu_irq_mask,
    .irq_unmask	= prcmu_irq_unmask,
    };
    static char *fw_project_name(u32 project)
    {
    switch (project) {
    case PRCMU_FW_PROJECT_U8500:
    return "U8500";
    case PRCMU_FW_PROJECT_U8400:
    return "U8400";
    case PRCMU_FW_PROJECT_U9500:
    return "U9500";
    case PRCMU_FW_PROJECT_U8500_MBB:
    return "U8500 MBB";
    case PRCMU_FW_PROJECT_U8500_C1:
    return "U8500 C1";
    case PRCMU_FW_PROJECT_U8500_C2:
    return "U8500 C2";
    case PRCMU_FW_PROJECT_U8500_C3:
    return "U8500 C3";
    case PRCMU_FW_PROJECT_U8500_C4:
    return "U8500 C4";
    case PRCMU_FW_PROJECT_U9500_MBL:
    return "U9500 MBL";
    case PRCMU_FW_PROJECT_U8500_SSG1:
    return "U8500 Samsung 1";
    case PRCMU_FW_PROJECT_U8500_MBL2:
    return "U8500 MBL2";
    case PRCMU_FW_PROJECT_U8520:
    return "U8520 MBL";
    case PRCMU_FW_PROJECT_U8420:
    return "U8420";
    case PRCMU_FW_PROJECT_U8500_SSG2:
    return "U8500 Samsung 2";
    case PRCMU_FW_PROJECT_U8420_SYSCLK:
    return "U8420-sysclk";
    case PRCMU_FW_PROJECT_U9540:
    return "U9540";
    case PRCMU_FW_PROJECT_A9420:
    return "A9420";
    case PRCMU_FW_PROJECT_L8540:
    return "L8540";
    case PRCMU_FW_PROJECT_L8580:
    return "L8580";
    default:
    return "Unknown";
    }
    }
    static int db8500_irq_map(struct irq_domain *d, unsigned int virq,
    irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(virq, &prcmu_irq_chip,
    handle_simple_irq);
    return 0;
    }
    static const struct irq_domain_ops db8500_irq_ops = {
    .map    = db8500_irq_map,
    .xlate  = irq_domain_xlate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn db8500_irq_init(np: *mut device_node) -> c_int {
    static int db8500_irq_init(struct device_node *np)
    {
    int i;
    db8500_irq_domain = irq_domain_create_simple(of_fwnode_handle(np),
    NUM_PRCMU_WAKEUPS, 0,
    &db8500_irq_ops, core::ptr::null_mut());
    if (!db8500_irq_domain) {
    pr_err("Failed to create irqdomain\n");
    return -ENOSYS;
    }
// All wakeups will be used, so create mappings for all
    for (i = 0; i < NUM_PRCMU_WAKEUPS; i++)
    irq_create_mapping(db8500_irq_domain, i);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dbx500_fw_version_init(np: *mut device_node) {
    static void dbx500_fw_version_init(struct device_node *np)
    {
    void __iomem *tcpm_base;
    u32 version;
    tcpm_base = of_iomap(np, 1);
    if (!tcpm_base) {
    pr_err("no prcmu tcpm mem region provided\n");
    return;
    }
    version = readl(tcpm_base + DB8500_PRCMU_FW_VERSION_OFFSET);
    fw_info.version.project = (version & 0xFF);
    fw_info.version.api_version = (version >> 8) & 0xFF;
    fw_info.version.func_version = (version >> 16) & 0xFF;
    fw_info.version.errata = (version >> 24) & 0xFF;
    strscpy(fw_info.version.project_name,
    fw_project_name(fw_info.version.project),
    sizeof(fw_info.version.project_name));
    fw_info.valid = true;
    pr_info("PRCMU firmware: %s(%d), version %d.%d.%d\n",
    fw_info.version.project_name,
    fw_info.version.project,
    fw_info.version.api_version,
    fw_info.version.func_version,
    fw_info.version.errata);
    iounmap(tcpm_base);
    }
#[no_mangle]
pub unsafe extern "C" fn db8500_prcmu_early_init() -> void __init {
    void __init db8500_prcmu_early_init(void)
    {
//
// This is a temporary remap to bring up the clocks. It is
// subsequently replaces with a real remap. After the merge of
// the mailbox subsystem all of this early code goes away, and the
// clock driver can probe independently. An early initcall will
// still be needed, but it can be diverted into drivers/clk/ux500.
//
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "stericsson,db8500-prcmu");
    prcmu_base = of_iomap(np, 0);
    if (!prcmu_base) {
    of_node_put(np);
    pr_err("%s: ioremap() of prcmu registers failed!\n", __func__);
    return;
    }
    dbx500_fw_version_init(np);
    of_node_put(np);
    spin_lock_init(&mb0_transfer.lock);
    spin_lock_init(&mb0_transfer.dbb_irqs_lock);
    mutex_init(&mb0_transfer.ac_wake_lock);
    init_completion(&mb0_transfer.ac_wake_work);
    mutex_init(&mb1_transfer.lock);
    init_completion(&mb1_transfer.work);
    mb1_transfer.ape_opp = APE_NO_CHANGE;
    mutex_init(&mb2_transfer.lock);
    init_completion(&mb2_transfer.work);
    spin_lock_init(&mb2_transfer.auto_pm_lock);
    spin_lock_init(&mb3_transfer.lock);
    mutex_init(&mb3_transfer.sysclk_lock);
    init_completion(&mb3_transfer.sysclk_work);
    mutex_init(&mb4_transfer.lock);
    init_completion(&mb4_transfer.work);
    mutex_init(&mb5_transfer.lock);
    init_completion(&mb5_transfer.work);
    INIT_WORK(&mb0_transfer.mask_work, prcmu_mask_work);
    }
#[no_mangle]
unsafe extern "C" fn init_prcm_registers() {
    static void init_prcm_registers(void)
    {
    u32 val;
    val = readl(PRCM_A9PL_FORCE_CLKEN);
    val &= ~(PRCM_A9PL_FORCE_CLKEN_PRCM_A9PL_FORCE_CLKEN |
    PRCM_A9PL_FORCE_CLKEN_PRCM_A9AXI_FORCE_CLKEN);
    writel(val, (PRCM_A9PL_FORCE_CLKEN));
    }
//
// Power domain switches (ePODs) modeled as regulators for the DB8500 SoC
//
    static struct regulator_consumer_supply db8500_vape_consumers[] = {
    REGULATOR_SUPPLY("v-ape", core::ptr::null_mut()),
    REGULATOR_SUPPLY("v-i2c", "nmk-i2c.0"),
    REGULATOR_SUPPLY("v-i2c", "nmk-i2c.1"),
    REGULATOR_SUPPLY("v-i2c", "nmk-i2c.2"),
    REGULATOR_SUPPLY("v-i2c", "nmk-i2c.3"),
    REGULATOR_SUPPLY("v-i2c", "nmk-i2c.4"),
// "v-mmc" changed to "vcore" in the mainline kernel
    REGULATOR_SUPPLY("vcore", "sdi0"),
    REGULATOR_SUPPLY("vcore", "sdi1"),
    REGULATOR_SUPPLY("vcore", "sdi2"),
    REGULATOR_SUPPLY("vcore", "sdi3"),
    REGULATOR_SUPPLY("vcore", "sdi4"),
    REGULATOR_SUPPLY("v-dma", "dma40.0"),
    REGULATOR_SUPPLY("v-ape", "ab8500-usb.0"),
// "v-uart" changed to "vcore" in the mainline kernel
    REGULATOR_SUPPLY("vcore", "uart0"),
    REGULATOR_SUPPLY("vcore", "uart1"),
    REGULATOR_SUPPLY("vcore", "uart2"),
    REGULATOR_SUPPLY("v-ape", "nmk-ske-keypad.0"),
    REGULATOR_SUPPLY("v-hsi", "ste_hsi.0"),
    REGULATOR_SUPPLY("vddvario", "smsc911x.0"),
    };
    static struct regulator_consumer_supply db8500_vsmps2_consumers[] = {
    REGULATOR_SUPPLY("musb_1v8", "ab8500-usb.0"),
// AV8100 regulator
    REGULATOR_SUPPLY("hdmi_1v8", "0-0070"),
    };
    static struct regulator_consumer_supply db8500_b2r2_mcde_consumers[] = {
    REGULATOR_SUPPLY("vsupply", "b2r2_bus"),
    REGULATOR_SUPPLY("vsupply", "mcde"),
    };
// SVA MMDSP regulator switch
    static struct regulator_consumer_supply db8500_svammdsp_consumers[] = {
    REGULATOR_SUPPLY("sva-mmdsp", "cm_control"),
    };
// SVA pipe regulator switch
    static struct regulator_consumer_supply db8500_svapipe_consumers[] = {
    REGULATOR_SUPPLY("sva-pipe", "cm_control"),
    };
// SIA MMDSP regulator switch
    static struct regulator_consumer_supply db8500_siammdsp_consumers[] = {
    REGULATOR_SUPPLY("sia-mmdsp", "cm_control"),
    };
// SIA pipe regulator switch
    static struct regulator_consumer_supply db8500_siapipe_consumers[] = {
    REGULATOR_SUPPLY("sia-pipe", "cm_control"),
    };
    static struct regulator_consumer_supply db8500_sga_consumers[] = {
    REGULATOR_SUPPLY("v-mali", core::ptr::null_mut()),
    };
// ESRAM1 and 2 regulator switch
    static struct regulator_consumer_supply db8500_esram12_consumers[] = {
    REGULATOR_SUPPLY("esram12", "cm_control"),
    };
// ESRAM3 and 4 regulator switch
    static struct regulator_consumer_supply db8500_esram34_consumers[] = {
    REGULATOR_SUPPLY("v-esram34", "mcde"),
    REGULATOR_SUPPLY("esram34", "cm_control"),
    REGULATOR_SUPPLY("lcla_esram", "dma40.0"),
    };
    static struct regulator_init_data db8500_regulators[DB8500_NUM_REGULATORS] = {
    [DB8500_REGULATOR_VAPE] = {
    .constraints = {
    .name = "db8500-vape",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    .always_on = true,
    },
    .consumer_supplies = db8500_vape_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_vape_consumers),
    },
    [DB8500_REGULATOR_VARM] = {
    .constraints = {
    .name = "db8500-varm",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    [DB8500_REGULATOR_VMODEM] = {
    .constraints = {
    .name = "db8500-vmodem",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    [DB8500_REGULATOR_VPLL] = {
    .constraints = {
    .name = "db8500-vpll",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    [DB8500_REGULATOR_VSMPS1] = {
    .constraints = {
    .name = "db8500-vsmps1",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    [DB8500_REGULATOR_VSMPS2] = {
    .constraints = {
    .name = "db8500-vsmps2",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    .consumer_supplies = db8500_vsmps2_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_vsmps2_consumers),
    },
    [DB8500_REGULATOR_VSMPS3] = {
    .constraints = {
    .name = "db8500-vsmps3",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    [DB8500_REGULATOR_VRF1] = {
    .constraints = {
    .name = "db8500-vrf1",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    [DB8500_REGULATOR_SWITCH_SVAMMDSP] = {
// dependency to u8500-vape is handled outside regulator framework
    .constraints = {
    .name = "db8500-sva-mmdsp",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    .consumer_supplies = db8500_svammdsp_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_svammdsp_consumers),
    },
    [DB8500_REGULATOR_SWITCH_SVAMMDSPRET] = {
    .constraints = {
// "ret" means "retention"
    .name = "db8500-sva-mmdsp-ret",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    [DB8500_REGULATOR_SWITCH_SVAPIPE] = {
// dependency to u8500-vape is handled outside regulator framework
    .constraints = {
    .name = "db8500-sva-pipe",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    .consumer_supplies = db8500_svapipe_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_svapipe_consumers),
    },
    [DB8500_REGULATOR_SWITCH_SIAMMDSP] = {
// dependency to u8500-vape is handled outside regulator framework
    .constraints = {
    .name = "db8500-sia-mmdsp",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    .consumer_supplies = db8500_siammdsp_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_siammdsp_consumers),
    },
    [DB8500_REGULATOR_SWITCH_SIAMMDSPRET] = {
    .constraints = {
    .name = "db8500-sia-mmdsp-ret",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    [DB8500_REGULATOR_SWITCH_SIAPIPE] = {
// dependency to u8500-vape is handled outside regulator framework
    .constraints = {
    .name = "db8500-sia-pipe",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    .consumer_supplies = db8500_siapipe_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_siapipe_consumers),
    },
    [DB8500_REGULATOR_SWITCH_SGA] = {
    .supply_regulator = "db8500-vape",
    .constraints = {
    .name = "db8500-sga",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    .consumer_supplies = db8500_sga_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_sga_consumers),
    },
    [DB8500_REGULATOR_SWITCH_B2R2_MCDE] = {
    .supply_regulator = "db8500-vape",
    .constraints = {
    .name = "db8500-b2r2-mcde",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    .consumer_supplies = db8500_b2r2_mcde_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_b2r2_mcde_consumers),
    },
    [DB8500_REGULATOR_SWITCH_ESRAM12] = {
//
// esram12 is set in retention and supplied by Vsafe when Vape is off,
// no need to hold Vape
//
    .constraints = {
    .name = "db8500-esram12",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    .consumer_supplies = db8500_esram12_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_esram12_consumers),
    },
    [DB8500_REGULATOR_SWITCH_ESRAM12RET] = {
    .constraints = {
    .name = "db8500-esram12-ret",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    [DB8500_REGULATOR_SWITCH_ESRAM34] = {
//
// esram34 is set in retention and supplied by Vsafe when Vape is off,
// no need to hold Vape
//
    .constraints = {
    .name = "db8500-esram34",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    .consumer_supplies = db8500_esram34_consumers,
    .num_consumer_supplies = ARRAY_SIZE(db8500_esram34_consumers),
    },
    [DB8500_REGULATOR_SWITCH_ESRAM34RET] = {
    .constraints = {
    .name = "db8500-esram34-ret",
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    },
    };
    static const struct mfd_cell common_prcmu_devs[] = {
    MFD_CELL_NAME("db8500_wdt"),
    MFD_CELL_NAME("db8500-cpuidle"),
    };
    static const struct mfd_cell db8500_prcmu_devs[] = {
    MFD_CELL_OF("db8500-prcmu-regulators", core::ptr::null_mut(),
    &db8500_regulators, sizeof(db8500_regulators), 0,
    "stericsson,db8500-prcmu-regulator"),
    MFD_CELL_OF("db8500-thermal",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,db8500-thermal"),
    };
#[no_mangle]
unsafe extern "C" fn db8500_prcmu_register_ab8500(parent: *mut device) -> c_int {
    static int db8500_prcmu_register_ab8500(struct device *parent)
    {
    struct device_node *np;
    struct resource ab850x_resource;
    const struct mfd_cell ab8500_cell = {
    .name = "ab8500-core",
    .of_compatible = "stericsson,ab8500",
    .id = AB8500_VERSION_AB8500,
    .resources = &ab850x_resource,
    .num_resources = 1,
    };
    const struct mfd_cell ab8505_cell = {
    .name = "ab8505-core",
    .of_compatible = "stericsson,ab8505",
    .id = AB8500_VERSION_AB8505,
    .resources = &ab850x_resource,
    .num_resources = 1,
    };
    const struct mfd_cell *ab850x_cell;
    if (!parent.of_node)
    return -ENODEV;
// Look up the device node, sneak the IRQ out of it
    for_each_child_of_node(parent.of_node, np) {
    if (of_device_is_compatible(np, ab8500_cell.of_compatible)) {
    ab850x_cell = &ab8500_cell;
    break;
    }
    if (of_device_is_compatible(np, ab8505_cell.of_compatible)) {
    ab850x_cell = &ab8505_cell;
    break;
    }
    }
    if (!np) {
    dev_info(parent, "could not find AB850X node in the device tree\n");
    return -ENODEV;
    }
    of_irq_to_resource_table(np, &ab850x_resource, 1);
    return mfd_add_devices(parent, 0, ab850x_cell, 1, core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn db8500_prcmu_probe(pdev: *mut platform_device) -> c_int {
    static int db8500_prcmu_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    let mut irq: c_int = 0, err = 0;
    struct resource *res;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "prcmu");
    if (!res) {
    dev_err(&pdev.dev, "no prcmu memory region provided\n");
    return -EINVAL;
    }
    prcmu_base = devm_ioremap(&pdev.dev, res.start, resource_size(res));
    if (!prcmu_base) {
    dev_err(&pdev.dev,
    "failed to ioremap prcmu register memory\n");
    return -ENOMEM;
    }
    init_prcm_registers();
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "prcmu-tcdm");
    if (!res) {
    dev_err(&pdev.dev, "no prcmu tcdm region provided\n");
    return -EINVAL;
    }
    tcdm_base = devm_ioremap(&pdev.dev, res.start,
    resource_size(res));
    if (!tcdm_base) {
    dev_err(&pdev.dev,
    "failed to ioremap prcmu-tcdm register memory\n");
    return -ENOMEM;
    }
// Clean up the mailbox interrupts after pre-kernel code.
    writel(ALL_MBOX_BITS, PRCM_ARM_IT1_CLR);
    irq = platform_get_irq(pdev, 0);
    if (irq <= 0)
    return irq;
    err = request_threaded_irq(irq, prcmu_irq_handler,
    prcmu_irq_thread_fn, IRQF_NO_SUSPEND, "prcmu", core::ptr::null_mut());
    if (err < 0) {
    pr_err("prcmu: Failed to allocate IRQ_DB8500_PRCMU1.\n");
    return err;
    }
    db8500_irq_init(np);
    db8500_prcmu_config_esram0_deep_sleep(ESRAM0_DEEP_SLEEP_STATE_RET);
    err = mfd_add_devices(&pdev.dev, 0, common_prcmu_devs,
    ARRAY_SIZE(common_prcmu_devs), core::ptr::null_mut(), 0, db8500_irq_domain);
    if (err) {
    pr_err("prcmu: Failed to add subdevices\n");
    return err;
    }
// TODO: Remove restriction when clk definitions are available.
    if (!of_machine_is_compatible("st-ericsson,u8540")) {
    err = mfd_add_devices(&pdev.dev, 0, db8500_prcmu_devs,
    ARRAY_SIZE(db8500_prcmu_devs), core::ptr::null_mut(), 0,
    db8500_irq_domain);
    if (err) {
    mfd_remove_devices(&pdev.dev);
    pr_err("prcmu: Failed to add subdevices\n");
    return err;
    }
    }
    err = db8500_prcmu_register_ab8500(&pdev.dev);
    if (err) {
    mfd_remove_devices(&pdev.dev);
    pr_err("prcmu: Failed to add ab8500 subdevice\n");
    return err;
    }
    pr_info("DB8500 PRCMU initialized\n");
    return err;
    }
    static const struct of_device_id db8500_prcmu_match[] = {
    { .compatible = "stericsson,db8500-prcmu"},
    { },
    };
    static struct platform_driver db8500_prcmu_driver = {
    .driver = {
    .name = "db8500-prcmu",
    .of_match_table = db8500_prcmu_match,
    },
    .probe = db8500_prcmu_probe,
    };
#[no_mangle]
unsafe extern "C" fn db8500_prcmu_init() -> int __init {
    static int __init db8500_prcmu_init(void)
    {
    return platform_driver_register(&db8500_prcmu_driver);
    }
    core_initcall(db8500_prcmu_init);
