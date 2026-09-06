//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/db8500-prcmu.h
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
// Copyright (C) STMicroelectronics 2009
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Kumar Sanghvi <kumar.sanghvi@stericsson.com>
//
// PRCMU f/w APIs
//

//
// Registers
//
pub const DB8500_PRCM_LINE_VALUE: c_uint = 0x170;

pub const DB8500_PRCM_DSI_SW_RESET: c_uint = 0x324;

// Offset for the firmware version within the TCPM
pub const DB8500_PRCMU_FW_VERSION_OFFSET: c_uint = 0xA4;
pub const DB8500_PRCMU_LEGACY_OFFSET: c_uint = 0xDD4;
//
// CLKOUT sources
//
pub const PRCMU_CLKSRC_CLK38M: c_uint = 0x00;
pub const PRCMU_CLKSRC_ACLK: c_uint = 0x01;
pub const PRCMU_CLKSRC_SYSCLK: c_uint = 0x02;
pub const PRCMU_CLKSRC_LCDCLK: c_uint = 0x03;
pub const PRCMU_CLKSRC_SDMMCCLK: c_uint = 0x04;
pub const PRCMU_CLKSRC_TVCLK: c_uint = 0x05;
pub const PRCMU_CLKSRC_TIMCLK: c_uint = 0x06;
pub const PRCMU_CLKSRC_CLK009: c_uint = 0x07;
// These are only valid for CLKOUT1:
pub const PRCMU_CLKSRC_SIAMMDSPCLK: c_uint = 0x40;
pub const PRCMU_CLKSRC_I2CCLK: c_uint = 0x41;
pub const PRCMU_CLKSRC_MSP02CLK: c_uint = 0x42;
pub const PRCMU_CLKSRC_ARMPLL_OBSCLK: c_uint = 0x43;
pub const PRCMU_CLKSRC_HSIRXCLK: c_uint = 0x44;
pub const PRCMU_CLKSRC_HSITXCLK: c_uint = 0x45;
pub const PRCMU_CLKSRC_ARMCLKFIX: c_uint = 0x46;
pub const PRCMU_CLKSRC_HDMICLK: c_uint = 0x47;
//
// Definitions for controlling ESRAM0 in deep sleep.
//
pub const ESRAM0_DEEP_SLEEP_STATE_OFF: c_int = 1;
pub const ESRAM0_DEEP_SLEEP_STATE_RET: c_int = 2;
// This portion previously known as <mach/prcmu-fw-defs_v1.h>
//
// enum state - ON/OFF state definition
// @OFF: State is ON
// @ON: State is OFF
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum state {
    OFF = 0x0,
    ON  = 0x1,
}

//
// enum ret_state - general purpose On/Off/Retention states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ret_state {
    OFFST = 0,
    ONST  = 1,
    RETST = 2
}

//
// enum clk_arm - ARM Cortex A9 clock schemes
// @A9_OFF:
// @A9_BOOT:
// @A9_OPPT1:
// @A9_OPPT2:
// @A9_EXTCLK:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_arm {
    A9_OFF,
    A9_BOOT,
    A9_OPPT1,
    A9_OPPT2,
    A9_EXTCLK
}

//
// enum clk_gen - GEN#0/GEN#1 clock schemes
// @GEN_OFF:
// @GEN_BOOT:
// @GEN_OPPT1:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_gen {
    GEN_OFF,
    GEN_BOOT,
    GEN_OPPT1,
}

// some information between arm and xp70
//
// enum romcode_write - Romcode message written by A9 AND read by XP70
// @RDY_2_DS: Value set when ApDeepSleep state can be executed by XP70
// @RDY_2_XP70_RST: Value set when 0x0F has been successfully polled by the
// romcode. The xp70 will go into self-reset
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum romcode_write {
    RDY_2_DS = 0x09,
    RDY_2_XP70_RST = 0x10
}

//
// enum romcode_read - Romcode message written by XP70 and read by A9
// @INIT: Init value when romcode field is not used
// @FS_2_DS: Value set when power state is going from ApExecute to
// ApDeepSleep
// @END_DS: Value set when ApDeepSleep power state is reached coming from
// ApExecute state
// @DS_TO_FS: Value set when power state is going from ApDeepSleep to
// ApExecute
// @END_FS: Value set when ApExecute power state is reached coming from
// ApDeepSleep state
// @SWR: Value set when power state is going to ApReset
// @END_SWR: Value set when the xp70 finished executing ApReset actions and
// waits for romcode acknowledgment to go to self-reset
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum romcode_read {
    INIT = 0x00,
    FS_2_DS = 0x0A,
    END_DS = 0x0B,
    DS_TO_FS = 0x0C,
    END_FS = 0x0D,
    SWR = 0x0E,
    END_SWR = 0x0F
}

//
// enum ap_pwrst - current power states defined in PRCMU firmware
// @NO_PWRST: Current power state init
// @AP_BOOT: Current power state is apBoot
// @AP_EXECUTE: Current power state is apExecute
// @AP_DEEP_SLEEP: Current power state is apDeepSleep
// @AP_SLEEP: Current power state is apSleep
// @AP_IDLE: Current power state is apIdle
// @AP_RESET: Current power state is apReset
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ap_pwrst {
    NO_PWRST = 0x00,
    AP_BOOT = 0x01,
    AP_EXECUTE = 0x02,
    AP_DEEP_SLEEP = 0x03,
    AP_SLEEP = 0x04,
    AP_IDLE = 0x05,
    AP_RESET = 0x06
}

//
// enum ap_pwrst_trans - Transition states defined in PRCMU firmware
// @NO_TRANSITION: No power state transition
// @APEXECUTE_TO_APSLEEP: Power state transition from ApExecute to ApSleep
// @APIDLE_TO_APSLEEP: Power state transition from ApIdle to ApSleep
// @APBOOT_TO_APEXECUTE: Power state transition from ApBoot to ApExecute
// @APEXECUTE_TO_APDEEPSLEEP: Power state transition from ApExecute to
// ApDeepSleep
// @APEXECUTE_TO_APIDLE: Power state transition from ApExecute to ApIdle
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ap_pwrst_trans {
    PRCMU_AP_NO_CHANGE		= 0x00,
    APEXECUTE_TO_APSLEEP		= 0x01,
    APIDLE_TO_APSLEEP		= 0x02, /* To be removed */
    PRCMU_AP_SLEEP			= 0x01,
    APBOOT_TO_APEXECUTE		= 0x03,
    APEXECUTE_TO_APDEEPSLEEP	= 0x04, /* To be removed */
    PRCMU_AP_DEEP_SLEEP		= 0x04,
    APEXECUTE_TO_APIDLE		= 0x05, /* To be removed */
    PRCMU_AP_IDLE			= 0x05,
    PRCMU_AP_DEEP_IDLE		= 0x07,
}

//
// enum hw_acc_state - State definition for hardware accelerator
// @HW_NO_CHANGE: The hardware accelerator state must remain unchanged
// @HW_OFF: The hardware accelerator must be switched off
// @HW_OFF_RAMRET: The hardware accelerator must be switched off with its
// internal RAM in retention
// @HW_ON: The hwa hardware accelerator hwa must be switched on
//
// NOTE! Deprecated, to be removed when all users switched over to use the
// regulator API.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_acc_state {
    HW_NO_CHANGE = 0x00,
    HW_OFF = 0x01,
    HW_OFF_RAMRET = 0x02,
    HW_ON = 0x04
}

//
// enum  mbox_2_arm_stat - Status messages definition for mbox_arm
// @BOOT_TO_EXECUTEOK: The apBoot to apExecute state transition has been
// completed
// @DEEPSLEEPOK: The apExecute to apDeepSleep state transition has been
// completed
// @SLEEPOK: The apExecute to apSleep state transition has been completed
// @IDLEOK: The apExecute to apIdle state transition has been completed
// @SOFTRESETOK: The A9 watchdog/ SoftReset state has been completed
// @SOFTRESETGO : The A9 watchdog/SoftReset state is on going
// @BOOT_TO_EXECUTE: The apBoot to apExecute state transition is on going
// @EXECUTE_TO_DEEPSLEEP: The apExecute to apDeepSleep state transition is on
// going
// @DEEPSLEEP_TO_EXECUTE: The apDeepSleep to apExecute state transition is on
// going
// @DEEPSLEEP_TO_EXECUTEOK: The apDeepSleep to apExecute state transition has
// been completed
// @EXECUTE_TO_SLEEP: The apExecute to apSleep state transition is on going
// @SLEEP_TO_EXECUTE: The apSleep to apExecute state transition is on going
// @SLEEP_TO_EXECUTEOK: The apSleep to apExecute state transition has been
// completed
// @EXECUTE_TO_IDLE: The apExecute to apIdle state transition is on going
// @IDLE_TO_EXECUTE: The apIdle to apExecute state transition is on going
// @IDLE_TO_EXECUTEOK: The apIdle to apExecute state transition has been
// completed
// @INIT_STATUS: Status init
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ap_pwrsttr_status {
    BOOT_TO_EXECUTEOK = 0xFF,
    DEEPSLEEPOK = 0xFE,
    SLEEPOK = 0xFD,
    IDLEOK = 0xFC,
    SOFTRESETOK = 0xFB,
    SOFTRESETGO = 0xFA,
    BOOT_TO_EXECUTE = 0xF9,
    EXECUTE_TO_DEEPSLEEP = 0xF8,
    DEEPSLEEP_TO_EXECUTE = 0xF7,
    DEEPSLEEP_TO_EXECUTEOK = 0xF6,
    EXECUTE_TO_SLEEP = 0xF5,
    SLEEP_TO_EXECUTE = 0xF4,
    SLEEP_TO_EXECUTEOK = 0xF3,
    EXECUTE_TO_IDLE = 0xF2,
    IDLE_TO_EXECUTE = 0xF1,
    IDLE_TO_EXECUTEOK = 0xF0,
    RDYTODS_RETURNTOEXE    = 0xEF,
    NORDYTODS_RETURNTOEXE  = 0xEE,
    EXETOSLEEP_RETURNTOEXE = 0xED,
    EXETOIDLE_RETURNTOEXE  = 0xEC,
    INIT_STATUS = 0xEB,

// error messages
    INITERROR                     = 0x00,
    PLLARMLOCKP_ER                = 0x01,
    PLLDDRLOCKP_ER                = 0x02,
    PLLSOCLOCKP_ER                = 0x03,
    PLLSOCK1LOCKP_ER              = 0x04,
    ARMWFI_ER                     = 0x05,
    SYSCLKOK_ER                   = 0x06,
    I2C_NACK_DATA_ER              = 0x07,
    BOOT_ER                       = 0x08,
    I2C_STATUS_ALWAYS_1           = 0x0A,
    I2C_NACK_REG_ADDR_ER          = 0x0B,
    I2C_NACK_DATA0123_ER          = 0x1B,
    I2C_NACK_ADDR_ER              = 0x1F,
    CURAPPWRSTISNOT_BOOT          = 0x20,
    CURAPPWRSTISNOT_EXECUTE       = 0x21,
    CURAPPWRSTISNOT_SLEEPMODE     = 0x22,
    CURAPPWRSTISNOT_CORRECTFORIT10 = 0x23,
    FIFO4500WUISNOT_WUPEVENT      = 0x24,
    PLL32KLOCKP_ER                = 0x29,
    DDRDEEPSLEEPOK_ER             = 0x2A,
    ROMCODEREADY_ER               = 0x50,
    WUPBEFOREDS                   = 0x51,
    DDRCONFIG_ER                  = 0x52,
    WUPBEFORESLEEP                = 0x53,
    WUPBEFOREIDLE                 = 0x54
}

//
// enum dvfs_stat - DVFS status messages definition
// @DVFS_GO: A state transition DVFS is on going
// @DVFS_ARM100OPPOK: The state transition DVFS has been completed for 100OPP
// @DVFS_ARM50OPPOK: The state transition DVFS has been completed for 50OPP
// @DVFS_ARMEXTCLKOK: The state transition DVFS has been completed for EXTCLK
// @DVFS_NOCHGTCLKOK: The state transition DVFS has been completed for
// NOCHGCLK
// @DVFS_INITSTATUS: Value init
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dvfs_stat {
    DVFS_GO = 0xFF,
    DVFS_ARM100OPPOK = 0xFE,
    DVFS_ARM50OPPOK = 0xFD,
    DVFS_ARMEXTCLKOK = 0xFC,
    DVFS_NOCHGTCLKOK = 0xFB,
    DVFS_INITSTATUS = 0x00
}

//
// enum sva_mmdsp_stat - SVA MMDSP status messages
// @SVA_MMDSP_GO: SVAMMDSP interrupt has happened
// @SVA_MMDSP_INIT: Status init
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sva_mmdsp_stat {
    SVA_MMDSP_GO = 0xFF,
    SVA_MMDSP_INIT = 0x00
}

//
// enum sia_mmdsp_stat - SIA MMDSP status messages
// @SIA_MMDSP_GO: SIAMMDSP interrupt has happened
// @SIA_MMDSP_INIT: Status init
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sia_mmdsp_stat {
    SIA_MMDSP_GO = 0xFF,
    SIA_MMDSP_INIT = 0x00
}

//
// enum  mbox_to_arm_err - Error messages definition
// @INIT_ERR: Init value
// @PLLARMLOCKP_ERR: PLLARM has not been correctly locked in given time
// @PLLDDRLOCKP_ERR: PLLDDR has not been correctly locked in the given time
// @PLLSOC0LOCKP_ERR: PLLSOC0 has not been correctly locked in the given time
// @PLLSOC1LOCKP_ERR: PLLSOC1 has not been correctly locked in the given time
// @ARMWFI_ERR: The ARM WFI has not been correctly executed in the given time
// @SYSCLKOK_ERR: The SYSCLK is not available in the given time
// @BOOT_ERR: Romcode has not validated the XP70 self reset in the given time
// @ROMCODESAVECONTEXT: The Romcode didn.t correctly save it secure context
// @VARMHIGHSPEEDVALTO_ERR: The ARM high speed supply value transfered
// through I2C has not been correctly executed in the given time
// @VARMHIGHSPEEDACCESS_ERR: The command value of VarmHighSpeedVal transfered
// through I2C has not been correctly executed in the given time
// @VARMLOWSPEEDVALTO_ERR:The ARM low speed supply value transfered through
// I2C has not been correctly executed in the given time
// @VARMLOWSPEEDACCESS_ERR: The command value of VarmLowSpeedVal transfered
// through I2C has not been correctly executed in the given time
// @VARMRETENTIONVALTO_ERR: The ARM retention supply value transfered through
// I2C has not been correctly executed in the given time
// @VARMRETENTIONACCESS_ERR: The command value of VarmRetentionVal transfered
// through I2C has not been correctly executed in the given time
// @VAPEHIGHSPEEDVALTO_ERR: The APE highspeed supply value transfered through
// I2C has not been correctly executed in the given time
// @VSAFEHPVALTO_ERR: The SAFE high power supply value transfered through I2C
// has not been correctly executed in the given time
// @VMODSEL1VALTO_ERR: The MODEM sel1 supply value transfered through I2C has
// not been correctly executed in the given time
// @VMODSEL2VALTO_ERR: The MODEM sel2 supply value transfered through I2C has
// not been correctly executed in the given time
// @VARMOFFACCESS_ERR: The command value of Varm ON/OFF transfered through
// I2C has not been correctly executed in the given time
// @VAPEOFFACCESS_ERR: The command value of Vape ON/OFF transfered through
// I2C has not been correctly executed in the given time
// @VARMRETACCES_ERR: The command value of Varm retention ON/OFF transfered
// through I2C has not been correctly executed in the given time
// @CURAPPWRSTISNOTBOOT:Generated when Arm want to do power state transition
// ApBoot to ApExecute but the power current state is not Apboot
// @CURAPPWRSTISNOTEXECUTE: Generated when Arm want to do power state
// transition from ApExecute to others power state but the
// power current state is not ApExecute
// @CURAPPWRSTISNOTSLEEPMODE: Generated when wake up events are transmitted
// but the power current state is not ApDeepSleep/ApSleep/ApIdle
// @CURAPPWRSTISNOTCORRECTDBG:  Generated when wake up events are transmitted
// but the power current state is not correct
// @ARMREGU1VALTO_ERR:The ArmRegu1 value transferred through I2C has not
// been correctly executed in the given time
// @ARMREGU2VALTO_ERR: The ArmRegu2 value transferred through I2C has not
// been correctly executed in the given time
// @VAPEREGUVALTO_ERR: The VApeRegu value transfered through I2C has not
// been correctly executed in the given time
// @VSMPS3REGUVALTO_ERR: The VSmps3Regu value transfered through I2C has not
// been correctly executed in the given time
// @VMODREGUVALTO_ERR: The VModemRegu value transfered through I2C has not
// been correctly executed in the given time
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox_to_arm_err {
    INIT_ERR = 0x00,
    PLLARMLOCKP_ERR = 0x01,
    PLLDDRLOCKP_ERR = 0x02,
    PLLSOC0LOCKP_ERR = 0x03,
    PLLSOC1LOCKP_ERR = 0x04,
    ARMWFI_ERR = 0x05,
    SYSCLKOK_ERR = 0x06,
    BOOT_ERR = 0x07,
    ROMCODESAVECONTEXT = 0x08,
    VARMHIGHSPEEDVALTO_ERR = 0x10,
    VARMHIGHSPEEDACCESS_ERR = 0x11,
    VARMLOWSPEEDVALTO_ERR = 0x12,
    VARMLOWSPEEDACCESS_ERR = 0x13,
    VARMRETENTIONVALTO_ERR = 0x14,
    VARMRETENTIONACCESS_ERR = 0x15,
    VAPEHIGHSPEEDVALTO_ERR = 0x16,
    VSAFEHPVALTO_ERR = 0x17,
    VMODSEL1VALTO_ERR = 0x18,
    VMODSEL2VALTO_ERR = 0x19,
    VARMOFFACCESS_ERR = 0x1A,
    VAPEOFFACCESS_ERR = 0x1B,
    VARMRETACCES_ERR = 0x1C,
    CURAPPWRSTISNOTBOOT = 0x20,
    CURAPPWRSTISNOTEXECUTE = 0x21,
    CURAPPWRSTISNOTSLEEPMODE = 0x22,
    CURAPPWRSTISNOTCORRECTDBG = 0x23,
    ARMREGU1VALTO_ERR = 0x24,
    ARMREGU2VALTO_ERR = 0x25,
    VAPEREGUVALTO_ERR = 0x26,
    VSMPS3REGUVALTO_ERR = 0x27,
    VMODREGUVALTO_ERR = 0x28
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_acc {
    SVAMMDSP = 0,
    SVAPIPE = 1,
    SIAMMDSP = 2,
    SIAPIPE = 3,
    SGA = 4,
    B2R2MCDE = 5,
    ESRAM12 = 6,
    ESRAM34 = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs_pwrmgt {
    PWRDNCS0  = 0,
    WKUPCS0   = 1,
    PWRDNCS1  = 2,
    WKUPCS1   = 3
}

// Defs related to autonomous power management
//
// enum sia_sva_pwr_policy - Power policy
// @NO_CHGT:	No change
// @DSPOFF_HWPOFF:
// @DSPOFFRAMRET_HWPOFF:
// @DSPCLKOFF_HWPOFF:
// @DSPCLKOFF_HWPCLKOFF:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sia_sva_pwr_policy {
    NO_CHGT			= 0x0,
    DSPOFF_HWPOFF		= 0x1,
    DSPOFFRAMRET_HWPOFF	= 0x2,
    DSPCLKOFF_HWPOFF	= 0x3,
    DSPCLKOFF_HWPCLKOFF	= 0x4,
}

//
// enum auto_enable - Auto Power enable
// @AUTO_OFF:
// @AUTO_ON:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum auto_enable {
    AUTO_OFF	= 0x0,
    AUTO_ON		= 0x1,
}

// End of file previously known as prcmu-fw-defs_v1.h
//
// enum prcmu_power_status - results from set_power_state
// @PRCMU_SLEEP_OK: Sleep went ok
// @PRCMU_DEEP_SLEEP_OK: DeepSleep went ok
// @PRCMU_IDLE_OK: Idle went ok
// @PRCMU_DEEPIDLE_OK: DeepIdle went ok
// @PRCMU_PRCMU2ARMPENDINGIT_ER: Pending interrupt detected
// @PRCMU_ARMPENDINGIT_ER: Pending interrupt detected
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prcmu_power_status {
    PRCMU_SLEEP_OK			= 0xf3,
    PRCMU_DEEP_SLEEP_OK		= 0xf6,
    PRCMU_IDLE_OK			= 0xf0,
    PRCMU_DEEPIDLE_OK		= 0xe3,
    PRCMU_PRCMU2ARMPENDINGIT_ER	= 0x91,
    PRCMU_ARMPENDINGIT_ER		= 0x93,
}

// PRCMU Wakeup defines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prcmu_wakeup_index {
    PRCMU_WAKEUP_INDEX_RTC,
    PRCMU_WAKEUP_INDEX_RTT0,
    PRCMU_WAKEUP_INDEX_RTT1,
    PRCMU_WAKEUP_INDEX_HSI0,
    PRCMU_WAKEUP_INDEX_HSI1,
    PRCMU_WAKEUP_INDEX_USB,
    PRCMU_WAKEUP_INDEX_ABB,
    PRCMU_WAKEUP_INDEX_ABB_FIFO,
    PRCMU_WAKEUP_INDEX_ARM,
    PRCMU_WAKEUP_INDEX_CD_IRQ,
    NUM_PRCMU_WAKEUP_INDICES
}

//
// enum prcmu_wdog_id - PRCMU watchdog IDs
// @PRCMU_WDOG_ALL: use all timers
// @PRCMU_WDOG_CPU1: use first CPU timer only
// @PRCMU_WDOG_CPU2: use second CPU timer conly
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prcmu_wdog_id {
    PRCMU_WDOG_ALL = 0x00,
    PRCMU_WDOG_CPU1 = 0x01,
    PRCMU_WDOG_CPU2 = 0x02,
}

//
// enum ape_opp - APE OPP states definition
// @APE_OPP_INIT:
// @APE_NO_CHANGE: The APE operating point is unchanged
// @APE_100_OPP: The new APE operating point is ape100opp
// @APE_50_OPP: 50%
// @APE_50_PARTLY_25_OPP: 50%, except some clocks at 25%.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ape_opp {
    APE_OPP_INIT = 0x00,
    APE_NO_CHANGE = 0x01,
    APE_100_OPP = 0x02,
    APE_50_OPP = 0x03,
    APE_50_PARTLY_25_OPP = 0xFF,
}

//
// enum arm_opp - ARM OPP states definition
// @ARM_OPP_INIT:
// @ARM_NO_CHANGE: The ARM operating point is unchanged
// @ARM_100_OPP: The new ARM operating point is arm100opp
// @ARM_50_OPP: The new ARM operating point is arm50opp
// @ARM_MAX_OPP: Operating point is "max" (more than 100)
// @ARM_MAX_FREQ100OPP: Set max opp if available, else 100
// @ARM_EXTCLK: The new ARM operating point is armExtClk
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_opp {
    ARM_OPP_INIT = 0x00,
    ARM_NO_CHANGE = 0x01,
    ARM_100_OPP = 0x02,
    ARM_50_OPP = 0x03,
    ARM_MAX_OPP = 0x04,
    ARM_MAX_FREQ100OPP = 0x05,
    ARM_EXTCLK = 0x07
}

//
// enum ddr_opp - DDR OPP states definition
// @DDR_100_OPP: The new DDR operating point is ddr100opp
// @DDR_50_OPP: The new DDR operating point is ddr50opp
// @DDR_25_OPP: The new DDR operating point is ddr25opp
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ddr_opp {
    DDR_100_OPP = 0x00,
    DDR_50_OPP = 0x01,
    DDR_25_OPP = 0x02,
}

//
// enum ddr_pwrst - DDR power states definition
// @DDR_PWR_STATE_UNCHANGED: SDRAM and DDR controller state is unchanged
// @DDR_PWR_STATE_ON:
// @DDR_PWR_STATE_OFFLOWLAT:
// @DDR_PWR_STATE_OFFHIGHLAT:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ddr_pwrst {
    DDR_PWR_STATE_UNCHANGED     = 0x00,
    DDR_PWR_STATE_ON            = 0x01,
    DDR_PWR_STATE_OFFLOWLAT     = 0x02,
    DDR_PWR_STATE_OFFHIGHLAT    = 0x03
}

//
// Definitions for autonomous power management configuration.
//
// EPOD (power domain) IDs
//
// DB8500 EPODs
// - EPOD_ID_SVAMMDSP: power domain for SVA MMDSP
// - EPOD_ID_SVAPIPE: power domain for SVA pipe
// - EPOD_ID_SIAMMDSP: power domain for SIA MMDSP
// - EPOD_ID_SIAPIPE: power domain for SIA pipe
// - EPOD_ID_SGA: power domain for SGA
// - EPOD_ID_B2R2_MCDE: power domain for B2R2 and MCDE
// - EPOD_ID_ESRAM12: power domain for ESRAM 1 and 2
// - EPOD_ID_ESRAM34: power domain for ESRAM 3 and 4
// - NUM_EPOD_ID: number of power domains
//
// TODO: These should be prefixed.
//
pub const EPOD_ID_SVAMMDSP: c_int = 0;
pub const EPOD_ID_SVAPIPE: c_int = 1;
pub const EPOD_ID_SIAMMDSP: c_int = 2;
pub const EPOD_ID_SIAPIPE: c_int = 3;
pub const EPOD_ID_SGA: c_int = 4;
pub const EPOD_ID_B2R2_MCDE: c_int = 5;
pub const EPOD_ID_ESRAM12: c_int = 6;
pub const EPOD_ID_ESRAM34: c_int = 7;
pub const NUM_EPOD_ID: c_int = 8;
//
// state definition for EPOD (power domain)
// - EPOD_STATE_NO_CHANGE: The EPOD should remain unchanged
// - EPOD_STATE_OFF: The EPOD is switched off
// - EPOD_STATE_RAMRET: The EPOD is switched off with its internal RAM in
// retention
// - EPOD_STATE_ON_CLK_OFF: The EPOD is switched on, clock is still off
// - EPOD_STATE_ON: Same as above, but with clock enabled
//
pub const EPOD_STATE_NO_CHANGE: c_uint = 0x00;
pub const EPOD_STATE_OFF: c_uint = 0x01;
pub const EPOD_STATE_RAMRET: c_uint = 0x02;
pub const EPOD_STATE_ON_CLK_OFF: c_uint = 0x03;
pub const EPOD_STATE_ON: c_uint = 0x04;
pub const PRCMU_FW_PROJECT_U8500: c_int = 2;
pub const PRCMU_FW_PROJECT_U8400: c_int = 3;

pub const PRCMU_FW_PROJECT_U8500_MBB: c_int = 5;
pub const PRCMU_FW_PROJECT_U8500_C1: c_int = 6;
pub const PRCMU_FW_PROJECT_U8500_C2: c_int = 7;
pub const PRCMU_FW_PROJECT_U8500_C3: c_int = 8;
pub const PRCMU_FW_PROJECT_U8500_C4: c_int = 9;
pub const PRCMU_FW_PROJECT_U9500_MBL: c_int = 10;

pub const PRCMU_FW_PROJECT_U8520: c_int = 13;
pub const PRCMU_FW_PROJECT_U8420: c_int = 14;

pub const PRCMU_FW_PROJECT_U8420_SYSCLK: c_int = 17;
pub const PRCMU_FW_PROJECT_A9420: c_int = 20;
// [32..63] 9540 and derivatives
pub const PRCMU_FW_PROJECT_U9540: c_int = 32;
// [64..95] 8540 and derivatives
pub const PRCMU_FW_PROJECT_L8540: c_int = 64;
// [96..126] 8580 and derivatives
pub const PRCMU_FW_PROJECT_L8580: c_int = 96;
pub const PRCMU_FW_PROJECT_NAME_LEN: c_int = 20;
// PRCMU QoS APE OPP class
pub const PRCMU_QOS_APE_OPP: c_int = 1;
pub const PRCMU_QOS_DDR_OPP: c_int = 2;
pub const PRCMU_QOS_ARM_OPP: c_int = 3;

pub const PRCMU_AUTO_PM_OFF: c_int = 0;
pub const PRCMU_AUTO_PM_ON: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prcmu_auto_pm_policy {
    PRCMU_AUTO_PM_POLICY_NO_CHANGE,
    PRCMU_AUTO_PM_POLICY_DSP_OFF_HWP_OFF,
    PRCMU_AUTO_PM_POLICY_DSP_OFF_RAMRET_HWP_OFF,
    PRCMU_AUTO_PM_POLICY_DSP_CLK_OFF_HWP_OFF,
    PRCMU_AUTO_PM_POLICY_DSP_CLK_OFF_HWP_CLK_OFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prcmu_fw_version {
    pub /: *mut *mut u32 project; / Notice, project shifted with 8 on ux540,
    pub api_version: u8,
    pub func_version: u8,
    pub errata: u8,
    pub project_name: [c_char; PRCMU_FW_PROJECT_NAME_LEN],
}

//
// struct prcmu_auto_pm_config - Autonomous power management configuration.
// @sia_auto_pm_enable: SIA autonomous pm enable. (PRCMU_AUTO_PM_{OFF,ON})
// @sia_power_on:       SIA power ON enable. (PRCMU_AUTO_PM_POWER_ON_* bitmask)
// @sia_policy:         SIA power policy. (enum prcmu_auto_pm_policy)
// @sva_auto_pm_enable: SVA autonomous pm enable. (PRCMU_AUTO_PM_{OFF,ON})
// @sva_power_on:       SVA power ON enable. (PRCMU_AUTO_PM_POWER_ON_* bitmask)
// @sva_policy:         SVA power policy. (enum prcmu_auto_pm_policy)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prcmu_auto_pm_config {
    pub sia_auto_pm_enable: u8,
    pub sia_power_on: u8,
    pub sia_policy: u8,
    pub sva_auto_pm_enable: u8,
    pub sva_power_on: u8,
    pub sva_policy: u8,
}

extern "C" {
    pub fn db8500_prcmu_early_init();
}
extern "C" {
    pub fn prcmu_set_rc_a2p(romcode_write: enum) -> c_int;
}
extern "C" {
    pub fn prcmu_get_rc_p2a() -> romcode_read;
}
extern "C" {
    pub fn prcmu_get_xp70_current_state() -> ap_pwrst;
}
extern "C" {
    pub fn prcmu_has_arm_maxopp() -> bool;
}
extern "C" {
    pub fn prcmu_release_usb_wakeup_state() -> c_int;
}
extern "C" {
    pub fn prcmu_is_auto_pm_enabled() -> bool;
}
extern "C" {
    pub fn prcmu_config_clkout(clkout: u8, source: u8, div: u8) -> c_int;
}
extern "C" {
    pub fn prcmu_clock_rate(clock: u8) -> c_ulong;
}
extern "C" {
    pub fn prcmu_round_clock_rate(clock: u8, rate: c_ulong) -> c_long;
}
extern "C" {
    pub fn prcmu_set_clock_rate(clock: u8, rate: c_ulong) -> c_int;
}
extern "C" {
    pub fn prcmu_set_clock_divider(clock: u8, divider: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_config_hotdog(threshold: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_config_hotmon(low: u8, high: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_start_temp_sense(cycles32k: u16) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_stop_temp_sense() -> c_int;
}
extern "C" {
    pub fn prcmu_abb_read(slave: u8, reg: u8, value: *mut u8, size: u8) -> c_int;
}
extern "C" {
    pub fn prcmu_abb_write(slave: u8, reg: u8, value: *mut u8, size: u8) -> c_int;
}
extern "C" {
    pub fn prcmu_ac_wake_req() -> c_int;
}
extern "C" {
    pub fn prcmu_ac_sleep_req();
}
extern "C" {
    pub fn db8500_prcmu_modem_reset();
}
extern "C" {
    pub fn db8500_prcmu_config_a9wdog(num: u8, sleep_auto_off: bool) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_enable_a9wdog(id: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_disable_a9wdog(id: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_kick_a9wdog(id: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_load_a9wdog(id: u8, val: u32) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_system_reset(reset_code: u16);
}
extern "C" {
    pub fn db8500_prcmu_set_power_state(state: u8, keep_ulp_clk: bool, keep_ap_pll: bool) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_get_power_state_result() -> u8;
}
extern "C" {
    pub fn db8500_prcmu_enable_wakeups(wakeups: u32);
}
extern "C" {
    pub fn db8500_prcmu_set_epod(epod_id: u16, epod_state: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_request_clock(clock: u8, enable: bool) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_config_abb_event_readout(abb_events: u32);
}
extern "C" {
    pub fn db8500_prcmu_get_abb_event_buffer(buf: *mut void __iomem);
}
extern "C" {
    pub fn db8500_prcmu_config_esram0_deep_sleep(state: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_get_reset_code() -> u16;
}
extern "C" {
    pub fn db8500_prcmu_is_ac_wake_requested() -> bool;
}
extern "C" {
    pub fn db8500_prcmu_set_arm_opp(opp: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_get_arm_opp() -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_set_ape_opp(opp: u8) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_get_ape_opp() -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_request_ape_opp_100_voltage(enable: bool) -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_get_ddr_opp() -> c_int;
}
extern "C" {
    pub fn db8500_prcmu_read(reg: c_uint) -> u32;
}
extern "C" {
    pub fn db8500_prcmu_write(reg: c_uint, value: u32);
}
extern "C" {
    pub fn db8500_prcmu_write_masked(reg: c_uint, mask: u32, value: u32);
}

