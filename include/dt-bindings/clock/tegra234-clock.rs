//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/tegra234-clock.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018-2022, NVIDIA CORPORATION. All rights reserved.
//
// @file
// @defgroup bpmp_clock_ids Clock ID's
// @{
//
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_ACTMON

// @brief output of gate CLK_ENB_ADSP

// @brief output of gate CLK_ENB_ADSPNEON

// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AHUB

// @brief output of gate CLK_ENB_APB2APE

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_APE

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AUD_MCLK

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AXI_CBB

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_CAN1

// @brief output of gate CLK_ENB_CAN1_HOST

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_CAN2

// @brief output of gate CLK_ENB_CAN2_HOST

// @brief output of divider CLK_RST_CONTROLLER_CLK_M_DIVIDE

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC1

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC2

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC3

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC4

// @brief output of gate CLK_ENB_DPAUX

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVJPG1

//
// @brief output of mux controlled by CLK_RST_CONTROLLER_ACLK_BURST_POLICY
// divided by the divider controlled by ACLK_CLK_DIVISOR in
// CLK_RST_CONTROLLER_SUPER_ACLK_DIVIDER
//

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_MSS_ENCRYPT switch divider output

// @brief clock recovered from EAVB input

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_AON_APB switch divider output

// @brief CLK_RST_CONTROLLER_AON_NIC_RATE divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_AON_CPU_NIC switch divider output

// @brief PLL controlled by CLK_RST_CONTROLLER_PLLA1_BASE for use by audio clocks

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSPK1

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSPK2

//
// @brief controls the EMC clock frequency.
// @details Doing a clk_set_rate on this clock will select the
// appropriate clock source, program the source rate and execute a
// specific sequence to switch to the new clock source for both memory
// controllers. This can be used to control the balance between memory
// throughput and memory controller power.
//

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_AXI_CLK_0 divider gated output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_PTP_REF_CLK_0 divider gated output

// @brief output of gate CLK_ENB_EQOS_RX

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_TX_CLK divider gated output

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH1

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH2

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH3

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH4

// @brief output of gate CLK_ENB_FUSE

// @brief output of GPU GPC0 clkGen (in 1x mode same rate as GPC0 MUX2 out)

// @brief TODO

// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HDA2CODEC_2X
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X

// @brief xusb_hs_hsicp_clk

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C1

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C2

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C3

// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C4

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C6

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C7

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C8

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C9

// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S1

// @brief clock recovered from I2S1 input

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S2

// @brief clock recovered from I2S2 input

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S3

// @brief clock recovered from I2S3 input

// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S4

// @brief clock recovered from I2S4 input

// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S5

// @brief clock recovered from I2S5 input

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S6

// @brief clock recovered from I2S6 input

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_ISP

// @brief Monitored branch of EQOS_RX clock

// @brief CLK_RST_CONTROLLER_MAUDCLK_OUT_SWITCH_DIVIDER switch divider output (maudclk)

// @brief output of gate CLK_ENB_MIPI_CAL

// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_CORE_PLL_FIXED

// @brief output of gate CLK_ENB_MPHY_L0_RX_ANA

// @brief output of gate CLK_ENB_MPHY_L0_RX_LS_BIT

// @brief output of gate CLK_ENB_MPHY_L0_RX_SYMB

// @brief output of gate CLK_ENB_MPHY_L0_TX_LS_3XBIT

// @brief output of gate CLK_ENB_MPHY_L0_TX_SYMB

// @brief output of gate CLK_ENB_MPHY_L1_RX_ANA

// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_TX_1MHZ_REF

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVCSI

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVCSILP

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDEC

// @brief CLK_RST_CONTROLLER_HUBCLK_OUT_SWITCH_DIVIDER switch divider output (hubclk)

// @brief CLK_RST_CONTROLLER_DISPCLK_SWITCH_DIVIDER switch divider output (dispclk)

// @brief RG_CLK_CTRL__0_DIV divider output (nvdisplay_p0_clk)

// @brief RG_CLK_CTRL__1_DIV divider output (nvdisplay_p1_clk)

// @brief DSC_CLK (DISPCLK ÷ 3)

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVENC

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVJPG

// @brief input from Tegra's XTAL_IN

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_AON_TOUCH switch divider output

// PLL controlled by CLK_RST_CONTROLLER_PLLA_BASE for use by audio clocks

// @brief PLL controlled by CLK_RST_CONTROLLER_PLLAON_BASE for use by IP blocks in the AON domain

// Fixed 100MHz PLL for PCIe, SATA and superspeed USB

// @brief PLLP vco output

// @brief PLLP clk output

// Fixed frequency 960MHz PLL for USB and EAVB

// @brief output of the divider CLK_RST_CONTROLLER_PLLA_OUT

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM1

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM2

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM3

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM4

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM5

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM6

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM7

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM8

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_RCE_CPU_NIC output

// @brief CLK_RST_CONTROLLER_RCE_NIC_RATE divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_AON_I2C_SLOW switch divider output

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SCE_CPU_NIC

// @brief output of divider CLK_RST_CONTROLLER_SCE_NIC_RATE

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC1

// @brief Logical clk for setting the UPHY PLL3 rate

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC4

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_SE switch divider gated output

// @brief VPLL select for sor0_ref clk driven by disp_2clk_sor0_head_sel signal

// @brief Output of mux controlled by disp_2clk_sor0_pll_ref_clk_safe signal (sor0_ref_clk)

// @brief VPLL select for sor1_ref clk driven by disp_2clk_sor0_head_sel signal

// @brief SOR_PLL_REF_CLK_CTRL__0_DIV divider output

// @brief Output of mux controlled by disp_2clk_sor1_pll_ref_clk_safe signal (sor1_ref_clk)

// @brief SOR_PLL_REF_CLK_CTRL__1_DIV divider output

// @brief output of gate CLK_ENB_SOR_SAFE

// @brief SOR_CLK_CTRL__0_DIV divider output

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC5

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI1

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI2

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI3

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C_SLOW

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC1

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC2

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC3

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC4

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DSPK1

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DSPK2

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S1

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S2

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S3

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S4

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S5

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S6

// @brief controls MPHY_FORCE_LS_MODE upon enable & disable

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TACH0

// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TSEC

// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PKA

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTA

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTB

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTC

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTD

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTE

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTF

// @brief output of gate CLK_ENB_PEX1_CORE_6

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UART_FST_MIPI_CAL

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UFSDEV_REF

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UFSHC_CG_SYS

// @brief output of gate CLK_ENB_USB2_TRK

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VI

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VIC

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_CSITE switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_IST switch divider output

// @brief output of mux controlled by CLK_RST_CONTROLLER_IST_JTAG_REG_CLK_SEL

// @brief output of gate CLK_ENB_PEX2_CORE_7

// @brief output of gate CLK_ENB_PEX2_CORE_8

// @brief output of gate CLK_ENB_PEX2_CORE_9

// @brief dla0_falcon_clk

// @brief dla0_core_clk

// @brief dla1_falcon_clk

// @brief dla1_core_clk

// @brief Output of mux controlled by disp_2clk_sor0_clk_safe signal (sor0_clk)

// @brief Output of mux controlled by disp_2clk_sor1_clk_safe signal (sor1_clk)

// @brief DP macro feedback clock (same as LINKA_SYM CLKOUT)

// @brief Output of mux controlled by disp_2clk_h0_dsi_sel signal in sf0_clk path

// @brief Output of mux controlled by disp_2clk_sf0_clk_safe signal (sf0_clk)

// @brief Output of mux controlled by disp_2clk_sf1_clk_safe signal (sf1_clk)

// @brief CLKOUT_AB output from DSI BRICK A (dsi_clkout_ab)

// @brief output of gate CLK_ENB_PEX2_CORE_10

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_UARTI switch divider output (uarti_r_clk)

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_UARTJ switch divider output (uartj_r_clk)

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_UARTH switch divider output

// @brief ungated version of fuse clk

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_QSPI0 switch divider output (qspi0_2x_pm_clk)

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_QSPI1 switch divider output (qspi1_2x_pm_clk)

// @brief output of the divider QSPI_CLK_DIV2_SEL in CLK_RST_CONTROLLER_CLK_SOURCE_QSPI0 (qspi0_pm_clk)

// @brief output of the divider QSPI_CLK_DIV2_SEL in CLK_RST_CONTROLLER_CLK_SOURCE_QSPI1 (qspi1_pm_clk)

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_VI_CONST switch divider output

// @brief NAFLL clock source for BPMP

// @brief NAFLL clock source for SCE

// @brief NAFLL clock source for NVDEC

// @brief NAFLL clock source for NVJPG

// @brief NAFLL clock source for TSEC

// @brief NAFLL clock source for VI

// @brief NAFLL clock source for SE

// @brief NAFLL clock source for NVENC

// @brief NAFLL clock source for ISP

// @brief NAFLL clock source for VIC

// @brief NAFLL clock source for AXICBB

// @brief NAFLL clock source for NVJPG1

// @brief NAFLL clock source for PVA core

// @brief NAFLL clock source for PVA VPS

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_DBGAPB_0 switch divider output (dbgapb_clk)

// @brief NAFLL clock source for RCE

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_LA switch divider output (la_r_clk)

// @brief output of the divider CLK_RST_CONTROLLER_PLLP_OUTD

// @brief AXI_CBB branch sharing gate control with SDMMC4

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC_LEGACY_TM switch divider output

// @brief output of gate CLK_ENB_PEX0_CORE_0

// @brief output of gate CLK_ENB_PEX0_CORE_1

// @brief output of gate CLK_ENB_PEX0_CORE_2

// @brief output of gate CLK_ENB_PEX0_CORE_3

// @brief output of gate CLK_ENB_PEX0_CORE_4

// @brief output of gate CLK_ENB_PEX1_CORE_5

// @brief Monitored branch of PEX0_C0_CORE clock

// @brief Monitored branch of PEX0_C1_CORE clock

// @brief Monitored branch of PEX0_C2_CORE clock

// @brief Monitored branch of PEX0_C3_CORE clock

// @brief Monitored branch of PEX0_C4_CORE clock

// @brief Monitored branch of PEX1_C5_CORE clock

// @brief Monitored branch of PEX1_C6_CORE clock

// @brief output of GPU GPC1 clkGen (in 1x mode same rate as GPC1 MUX2 out)

// @brief PLL controlled by CLK_RST_CONTROLLER_PLLC4_BASE

// @brief PLLC4 VCO followed by DIV3 path

// @brief PLLC4 VCO followed by DIV5 path

// @brief output of the mux controlled by PLLC4_CLK_SEL

// @brief PLLC4 VCO followed by DIV2 path

// @brief PLL controlled by CLK_RST_CONTROLLER_PLLNVHS_BASE

// @brief Monitored branch of PEX2_C7_CORE clock

// @brief Monitored branch of PEX2_C8_CORE clock

// @brief Monitored branch of PEX2_C9_CORE clock

// @brief Monitored branch of PEX2_C10_CORE clock

// @brief RX clock recovered from MGBE0 lane input

// @brief RX clock recovered from MGBE1 lane input

// @brief RX clock recovered from MGBE2 lane input

// @brief RX clock recovered from MGBE3 lane input

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_SATA_USB_RX_BYP switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL0_MGMT switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL1_MGMT switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL2_MGMT switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL3_MGMT switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_NVHS_RX_BYP switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_NVHS_PLL0_MGMT switch divider output

// @brief xusb_core_dev_clk

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_XUSB_CORE_HOST switch divider output

// @brief xusb_core_host_clk

// @brief xusb_core_superspeed_clk

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_XUSB_FALCON switch divider output

// @brief xusb_falcon_host_clk

// @brief xusb_falcon_superspeed_clk

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_XUSB_FS switch divider output

// @brief xusb_fs_host_clk

// @brief xusb_fs_dev_clk

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_XUSB_SS switch divider output

// @brief xusb_ss_dev_clk

// @brief xusb_ss_superspeed_clk

// @brief NAFLL clock source for CPU cluster 0

// @brief NAFLL clock source for CPU cluster 1

// @brief NAFLL clock source for CPU cluster 2

// @brief CLK_RST_CONTROLLER_CAN1_CORE_RATE divider output

// @brief CLK_RST_CONTROLLER_CAN2_CORE_RATE divider outputt

// @brief CLK_RST_CONTROLLER_PLLA1_OUT1 switch divider output

// @brief NVHS PLL hardware power sequencer (overrides 'manual' programming of PLL)

// @brief PLL controlled by CLK_RST_CONTROLLER_PLLREFE_BASE

// @brief 32K input clock provided by PMIC

// @brief Fixed 48MHz clock divided down from utmipll

// @brief Fixed 480MHz clock divided down from utmipll

// @brief PLL controlled by CLK_RST_CONTROLLER_PLLNVCSI_BASE

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_PVA0_CPU_AXI switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_PVA0_VPS switch divider output

// @brief DLA0_CORE_NAFLL

// @brief DLA0_FALCON_NAFLL

// @brief DLA1_CORE_NAFLL

// @brief DLA1_FALCON_NAFLL

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_UART_FST_MIPI_CAL

// @brief GPU system clock

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C5

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_SE switch divider free running clk

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_BPMP_CPU_NIC switch divider output

// @brief output of gate CLK_ENB_BPMP_CPU

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_TSC switch divider output

// @brief output of mem pll A sync mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EMC

// @brief output of mem pll B sync mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EMCSB

// @brief output of mem pll C sync mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EMCSC

// @brief output of mem pll D sync mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EMCSD

// @brief PLL controlled by CLK_RST_CONTROLLER_PLLC_BASE

// @brief PLL controlled by CLK_RST_CONTROLLER_PLLC2_BASE

// @brief CLK_RST_CONTROLLER_TSC_HS_SUPER_CLK_DIVIDER skip divider output

// @brief Dummy clock to ensure minimum SoC voltage for fuse burning

// @brief GBE PLL

// @brief GBE PLL hardware power sequencer

// @brief output of EMC CDB side A fixed (DIV4)  divider

// @brief output of EMC CDB side B fixed (DIV4)  divider

// @brief output of EMC CDB side C fixed (DIV4)  divider

// @brief output of EMC CDB side D fixed (DIV4)  divider

// @brief PLLE hardware power sequencer (overrides 'manual' programming of PLL)

// @brief CLK_ENB_PLLREFE_OUT gate output

// @brief TEGRA234_CLK_SOR_SAFE clk source (PLLP_OUT0 divided by 17)

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_SOC_THERM switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_TSENSE switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_SEU1 switch divider free running clk

// @brief NAFLL clock source for OFA

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_OFA switch divider output

// @brief NAFLL clock source for SEU1

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_SEU1 switch divider gated output

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI4

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI5

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DCE_CPU_NIC

// @brief output of divider CLK_RST_CONTROLLER_DCE_NIC_RATE

// @brief NAFLL clock source for DCE

// @brief Monitored branch of MPHY_L0_RX_ANA clock

// @brief Monitored branch of MPHY_L1_RX_ANA clock

// @brief ungated version of TX symbol clock after fixed 1/2 divider

// @brief output of divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_L0_TX_LS_SYMB

// @brief output of gate CLK_ENB_MPHY_L0_TX_2X_SYMB

// @brief output of SW_MPHY_L0_TX_HS_SYMB divider in CLK_RST_CONTROLLER_MPHY_L0_TX_CLK_CTRL_0

// @brief output of SW_MPHY_L0_TX_LS_3XBIT divider in CLK_RST_CONTROLLER_MPHY_L0_TX_CLK_CTRL_0

// @brief LS/HS divider mux SW_MPHY_L0_TX_LS_HS_SEL in CLK_RST_CONTROLLER_MPHY_L0_TX_CLK_CTRL_0

// @brief Monitored branch of MPHY_L0_TX_SYMB clock

// @brief output of divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_L0_RX_LS_SYMB

// @brief output of SW_MPHY_L0_RX_HS_SYMB divider in CLK_RST_CONTROLLER_MPHY_L0_RX_CLK_CTRL_0

// @brief output of SW_MPHY_L0_RX_LS_BIT divider in  CLK_RST_CONTROLLER_MPHY_L0_RX_CLK_CTRL_0

// @brief LS/HS divider mux SW_MPHY_L0_RX_LS_HS_SEL in CLK_RST_CONTROLLER_MPHY_L0_RX_CLK_CTRL_0

// @brief Monitored branch of MPHY_L0_RX_SYMB clock

// @brief Monitored branch of MBGE0 RX input clock

// @brief Monitored branch of MBGE1 RX input clock

// @brief Monitored branch of MBGE2 RX input clock

// @brief Monitored branch of MBGE3 RX input clock

// @brief Monitored branch of MGBE0 RX PCS mux output

// @brief Monitored branch of MGBE1 RX PCS mux output

// @brief Monitored branch of MGBE2 RX PCS mux output

// @brief Monitored branch of MGBE3 RX PCS mux output

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TACH1

// @brief GBE_UPHY_MGBES_APP_CLK switch divider gated output

// @brief Logical clk for setting GBE UPHY PLL2 TX_REF rate

// @brief Logical clk for setting GBE UPHY PLL2 XDIG rate

// @brief RX PCS clock recovered from MGBE0 lane input

// @brief RX PCS clock recovered from MGBE1 lane input

// @brief RX PCS clock recovered from MGBE2 lane input

// @brief RX PCS clock recovered from MGBE3 lane input

// @brief output of mux controlled by GBE_UPHY_MGBE0_RX_PCS_CLK_SRC_SEL

// @brief GBE_UPHY_MGBE0_TX_CLK divider gated output

// @brief GBE_UPHY_MGBE0_TX_PCS_CLK divider gated output

// @brief GBE_UPHY_MGBE0_MAC_CLK divider output

// @brief GBE_UPHY_MGBE0_MAC_CLK gate output

// @brief GBE_UPHY_MGBE0_MACSEC_CLK gate output

// @brief GBE_UPHY_MGBE0_EEE_PCS_CLK gate output

// @brief GBE_UPHY_MGBE0_APP_CLK gate output

// @brief GBE_UPHY_MGBE0_PTP_REF_CLK divider gated output

// @brief output of mux controlled by GBE_UPHY_MGBE1_RX_PCS_CLK_SRC_SEL

// @brief GBE_UPHY_MGBE1_TX_CLK divider gated output

// @brief GBE_UPHY_MGBE1_TX_PCS_CLK divider gated output

// @brief GBE_UPHY_MGBE1_MAC_CLK divider output

// @brief GBE_UPHY_MGBE1_MAC_CLK gate output

// @brief GBE_UPHY_MGBE1_MACSEC_CLK gate output

// @brief GBE_UPHY_MGBE1_EEE_PCS_CLK gate output

// @brief GBE_UPHY_MGBE1_APP_CLK gate output

// @brief GBE_UPHY_MGBE1_PTP_REF_CLK divider gated output

// @brief output of mux controlled by GBE_UPHY_MGBE2_RX_PCS_CLK_SRC_SEL

// @brief GBE_UPHY_MGBE2_TX_CLK divider gated output

// @brief GBE_UPHY_MGBE2_TX_PCS_CLK divider gated output

// @brief GBE_UPHY_MGBE2_MAC_CLK divider output

// @brief GBE_UPHY_MGBE2_MAC_CLK gate output

// @brief GBE_UPHY_MGBE2_MACSEC_CLK gate output

// @brief GBE_UPHY_MGBE2_EEE_PCS_CLK gate output

// @brief GBE_UPHY_MGBE2_APP_CLK gate output

// @brief GBE_UPHY_MGBE2_PTP_REF_CLK divider gated output

// @brief output of mux controlled by GBE_UPHY_MGBE3_RX_PCS_CLK_SRC_SEL

// @brief GBE_UPHY_MGBE3_TX_CLK divider gated output

// @brief GBE_UPHY_MGBE3_TX_PCS_CLK divider gated output

// @brief GBE_UPHY_MGBE3_MAC_CLK divider output

// @brief GBE_UPHY_MGBE3_MAC_CLK gate output

// @brief GBE_UPHY_MGBE3_MACSEC_CLK gate output

// @brief GBE_UPHY_MGBE3_EEE_PCS_CLK gate output

// @brief GBE_UPHY_MGBE3_APP_CLK gate output

// @brief GBE_UPHY_MGBE3_PTP_REF_CLK divider gated output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_GBE_RX_BYP switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_GBE_PLL0_MGMT switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_GBE_PLL1_MGMT switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_GBE_PLL2_MGMT switch divider output

// @brief output of gate CLK_ENB_EQOS_MACSEC_RX

// @brief output of gate CLK_ENB_EQOS_MACSEC_TX

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_TX_CLK divider ungated output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_NVHS_PLL1_MGMT switch divider output

// @brief CLK_RST_CONTROLLER_CLK_SOURCE_EMCHUB mux output

// @brief clock recovered from I2S7 input

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S7

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S7

// @brief Monitored output of I2S7 pad macro mux

// @brief clock recovered from I2S8 input

// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S8

// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S8

// @brief Monitored output of I2S8 pad macro mux

// @brief NAFLL clock source for GPU GPC0

// @brief NAFLL clock source for GPU GPC1

// @brief NAFLL clock source for GPU SYSCLK

// @brief NAFLL clock source for CPU cluster 0 DSUCLK

// @brief NAFLL clock source for CPU cluster 1 DSUCLK

// @brief NAFLL clock source for CPU cluster 2 DSUCLK

// @brief output of gate CLK_ENB_SCE_CPU

// @brief output of gate CLK_ENB_RCE_CPU

// @brief output of gate CLK_ENB_DCE_CPU

// @brief DSIPLL VCO output

// @brief DSIPLL SYNC_CLKOUTP/N differential output

// @brief DSIPLL SYNC_CLKOUTA output

// @brief SPPLL0 VCO output

// @brief SPPLL0 SYNC_CLKOUTP/N differential output

// @brief SPPLL0 SYNC_CLKOUTA output

// @brief SPPLL0 SYNC_CLKOUTB output

// @brief SPPLL0 CLKOUT_DIVBY10 output

// @brief SPPLL0 CLKOUT_DIVBY25 output

// @brief SPPLL0 CLKOUT_DIVBY27P/N differential output

// @brief SPPLL1 VCO output

// @brief SPPLL1 SYNC_CLKOUTP/N differential output

// @brief SPPLL1 CLKOUT_DIVBY27P/N differential output

// @brief VPLL0 reference clock

// @brief VPLL0

// @brief VPLL1

// @brief NVDISPLAY_P0_CLK reference select

// @brief RG0_PCLK

// @brief RG1_PCLK

// @brief DISPPLL output

// @brief DISPHUBPLL output

// @brief CLK_RST_CONTROLLER_DSI_LP_SWITCH_DIVIDER switch divider output (dsi_lp_clk)

// @brief CLK_RST_CONTROLLER_AZA2XBITCLK_OUT_SWITCH_DIVIDER switch divider output (aza_2xbitclk)

// @brief aza_2xbitclk / 2 (aza_bitclk)

// @brief SWITCH_DSI_CORE_PIXEL_MISC_DSI_CORE_CLK_SRC switch output (dsi_core_clk)

// @brief Output of mux controlled by pkt_wr_fifo_signal from dsi (dsi_pixel_clk)

// @brief Output of mux controlled by disp_2clk_sor0_dp_sel (pre_sor0_clk)

// @brief Output of mux controlled by disp_2clk_sor1_dp_sel (pre_sor1_clk)

// @brief CLK_RST_CONTROLLER_LINK_REFCLK_CFG__0 output

// @brief Link clock input from DP macro brick PLL

// @brief SOR AFIFO clock outut

// @brief Monitored branch of linka_afifo_clk

// @brief Monitored branch of rg0_pclk

// @brief Monitored branch of rg1_pclk

// @brief Monitored branch of sor0_clk

// @brief Monitored branch of sor1_clk

// @brief EMC PLLHUB output

// @brief output of fixed (DIV2) MC HUB divider

// @brief output of divider controlled by EMC side A MC_EMC_SAFE_SAME_FREQ

// @brief output of divider controlled by EMC side B MC_EMC_SAFE_SAME_FREQ

// @brief output of divider controlled by EMC side C MC_EMC_SAFE_SAME_FREQ

// @brief output of divider controlled by EMC side D MC_EMC_SAFE_SAME_FREQ

// @}
