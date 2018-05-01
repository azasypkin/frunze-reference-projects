use bare_metal::Nr;
#[cfg(feature = "rt")]
extern "C" {
    fn DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DH_TRAMPOLINE() {
    DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak WWDG\nWWDG = DH_TRAMPOLINE\n.weak PVD\nPVD = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak FLASH\nFLASH = DH_TRAMPOLINE\n.weak RCC_CRS\nRCC_CRS = DH_TRAMPOLINE\n.weak EXTI0_1\nEXTI0_1 = DH_TRAMPOLINE\n.weak EXTI2_3\nEXTI2_3 = DH_TRAMPOLINE\n.weak EXTI4_15\nEXTI4_15 = DH_TRAMPOLINE\n.weak TSC\nTSC = DH_TRAMPOLINE\n.weak DMA1_CH1\nDMA1_CH1 = DH_TRAMPOLINE\n.weak DMA1_CH2_3_DMA2_CH1_2\nDMA1_CH2_3_DMA2_CH1_2 = DH_TRAMPOLINE\n.weak DMA1_CH4_5_6_7_DMA2_CH3_4_5\nDMA1_CH4_5_6_7_DMA2_CH3_4_5 = DH_TRAMPOLINE\n.weak ADC_COMP\nADC_COMP = DH_TRAMPOLINE\n.weak TIM1_BRK_UP_TRG_COM\nTIM1_BRK_UP_TRG_COM = DH_TRAMPOLINE\n.weak TIM1_CC\nTIM1_CC = DH_TRAMPOLINE\n.weak TIM2\nTIM2 = DH_TRAMPOLINE\n.weak TIM3\nTIM3 = DH_TRAMPOLINE\n.weak TIM6_DAC\nTIM6_DAC = DH_TRAMPOLINE\n.weak TIM7\nTIM7 = DH_TRAMPOLINE\n.weak TIM14\nTIM14 = DH_TRAMPOLINE\n.weak TIM15\nTIM15 = DH_TRAMPOLINE\n.weak TIM16\nTIM16 = DH_TRAMPOLINE\n.weak TIM17\nTIM17 = DH_TRAMPOLINE\n.weak I2C1\nI2C1 = DH_TRAMPOLINE\n.weak I2C2\nI2C2 = DH_TRAMPOLINE\n.weak SPI1\nSPI1 = DH_TRAMPOLINE\n.weak SPI2\nSPI2 = DH_TRAMPOLINE\n.weak USART1\nUSART1 = DH_TRAMPOLINE\n.weak USART2\nUSART2 = DH_TRAMPOLINE\n.weak USART3_4_5_6_7_8\nUSART3_4_5_6_7_8 = DH_TRAMPOLINE\n.weak CEC_CAN\nCEC_CAN = DH_TRAMPOLINE\n.weak USB\nUSB = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn RTC();
    fn FLASH();
    fn RCC_CRS();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn TSC();
    fn DMA1_CH1();
    fn DMA1_CH2_3_DMA2_CH1_2();
    fn DMA1_CH4_5_6_7_DMA2_CH3_4_5();
    fn ADC_COMP();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM6_DAC();
    fn TIM7();
    fn TIM14();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn I2C1();
    fn I2C2();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3_4_5_6_7_8();
    fn CEC_CAN();
    fn USB();
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 32] = [
    Some(WWDG),
    Some(PVD),
    Some(RTC),
    Some(FLASH),
    Some(RCC_CRS),
    Some(EXTI0_1),
    Some(EXTI2_3),
    Some(EXTI4_15),
    Some(TSC),
    Some(DMA1_CH1),
    Some(DMA1_CH2_3_DMA2_CH1_2),
    Some(DMA1_CH4_5_6_7_DMA2_CH3_4_5),
    Some(ADC_COMP),
    Some(TIM1_BRK_UP_TRG_COM),
    Some(TIM1_CC),
    Some(TIM2),
    Some(TIM3),
    Some(TIM6_DAC),
    Some(TIM7),
    Some(TIM14),
    Some(TIM15),
    Some(TIM16),
    Some(TIM17),
    Some(I2C1),
    Some(I2C2),
    Some(SPI1),
    Some(SPI2),
    Some(USART1),
    Some(USART2),
    Some(USART3_4_5_6_7_8),
    Some(CEC_CAN),
    Some(USB),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG,
    #[doc = "1 - PVD and VDDIO2 supply comparator interrupt"]
    PVD,
    #[doc = "2 - RTC interrupts"]
    RTC,
    #[doc = "3 - Flash global interrupt"]
    FLASH,
    #[doc = "4 - RCC and CRS global interrupts"]
    RCC_CRS,
    #[doc = "5 - EXTI Line[1:0] interrupts"]
    EXTI0_1,
    #[doc = "6 - EXTI Line[3:2] interrupts"]
    EXTI2_3,
    #[doc = "7 - EXTI Line15 and EXTI4 interrupts"]
    EXTI4_15,
    #[doc = "8 - Touch sensing interrupt"]
    TSC,
    #[doc = "9 - DMA1 channel 1 interrupt"]
    DMA1_CH1,
    #[doc = "10 - DMA1 channel 2 and 3 and DMA2 channel 1 and 2 interrupt"]
    DMA1_CH2_3_DMA2_CH1_2,
    #[doc = "11 - DMA1 channel 4, 5, 6 and 7 and DMA2 channel 3, 4 and 5 interrupts"]
    DMA1_CH4_5_6_7_DMA2_CH3_4_5,
    #[doc = "12 - ADC and comparator interrupts"]
    ADC_COMP,
    #[doc = "13 - TIM1 break, update, trigger and commutation interrupt"]
    TIM1_BRK_UP_TRG_COM,
    #[doc = "14 - TIM1 Capture Compare interrupt"]
    TIM1_CC,
    #[doc = "15 - TIM2 global interrupt"]
    TIM2,
    #[doc = "16 - TIM3 global interrupt"]
    TIM3,
    #[doc = "17 - TIM6 global interrupt and DAC underrun interrupt"]
    TIM6_DAC,
    #[doc = "18 - TIM7 global interrupt"]
    TIM7,
    #[doc = "19 - TIM14 global interrupt"]
    TIM14,
    #[doc = "20 - TIM15 global interrupt"]
    TIM15,
    #[doc = "21 - TIM16 global interrupt"]
    TIM16,
    #[doc = "22 - TIM17 global interrupt"]
    TIM17,
    #[doc = "23 - I2C1 global interrupt"]
    I2C1,
    #[doc = "24 - I2C2 global interrupt"]
    I2C2,
    #[doc = "25 - SPI1_global_interrupt"]
    SPI1,
    #[doc = "26 - SPI2 global interrupt"]
    SPI2,
    #[doc = "27 - USART1 global interrupt"]
    USART1,
    #[doc = "28 - USART2 global interrupt"]
    USART2,
    #[doc = "29 - USART3, USART4, USART5, USART6, USART7, USART8 global interrupt"]
    USART3_4_5_6_7_8,
    #[doc = "30 - CEC and CAN global interrupt"]
    CEC_CAN,
    #[doc = "31 - USB global interrupt"]
    USB,
}
unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD => 1,
            Interrupt::RTC => 2,
            Interrupt::FLASH => 3,
            Interrupt::RCC_CRS => 4,
            Interrupt::EXTI0_1 => 5,
            Interrupt::EXTI2_3 => 6,
            Interrupt::EXTI4_15 => 7,
            Interrupt::TSC => 8,
            Interrupt::DMA1_CH1 => 9,
            Interrupt::DMA1_CH2_3_DMA2_CH1_2 => 10,
            Interrupt::DMA1_CH4_5_6_7_DMA2_CH3_4_5 => 11,
            Interrupt::ADC_COMP => 12,
            Interrupt::TIM1_BRK_UP_TRG_COM => 13,
            Interrupt::TIM1_CC => 14,
            Interrupt::TIM2 => 15,
            Interrupt::TIM3 => 16,
            Interrupt::TIM6_DAC => 17,
            Interrupt::TIM7 => 18,
            Interrupt::TIM14 => 19,
            Interrupt::TIM15 => 20,
            Interrupt::TIM16 => 21,
            Interrupt::TIM17 => 22,
            Interrupt::I2C1 => 23,
            Interrupt::I2C2 => 24,
            Interrupt::SPI1 => 25,
            Interrupt::SPI2 => 26,
            Interrupt::USART1 => 27,
            Interrupt::USART2 => 28,
            Interrupt::USART3_4_5_6_7_8 => 29,
            Interrupt::CEC_CAN => 30,
            Interrupt::USB => 31,
        }
    }
}
use core::convert::TryFrom;
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl TryFrom<u8> for Interrupt {
    type Error = TryFromInterruptError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Interrupt::WWDG),
            1 => Ok(Interrupt::PVD),
            2 => Ok(Interrupt::RTC),
            3 => Ok(Interrupt::FLASH),
            4 => Ok(Interrupt::RCC_CRS),
            5 => Ok(Interrupt::EXTI0_1),
            6 => Ok(Interrupt::EXTI2_3),
            7 => Ok(Interrupt::EXTI4_15),
            8 => Ok(Interrupt::TSC),
            9 => Ok(Interrupt::DMA1_CH1),
            10 => Ok(Interrupt::DMA1_CH2_3_DMA2_CH1_2),
            11 => Ok(Interrupt::DMA1_CH4_5_6_7_DMA2_CH3_4_5),
            12 => Ok(Interrupt::ADC_COMP),
            13 => Ok(Interrupt::TIM1_BRK_UP_TRG_COM),
            14 => Ok(Interrupt::TIM1_CC),
            15 => Ok(Interrupt::TIM2),
            16 => Ok(Interrupt::TIM3),
            17 => Ok(Interrupt::TIM6_DAC),
            18 => Ok(Interrupt::TIM7),
            19 => Ok(Interrupt::TIM14),
            20 => Ok(Interrupt::TIM15),
            21 => Ok(Interrupt::TIM16),
            22 => Ok(Interrupt::TIM17),
            23 => Ok(Interrupt::I2C1),
            24 => Ok(Interrupt::I2C2),
            25 => Ok(Interrupt::SPI1),
            26 => Ok(Interrupt::SPI2),
            27 => Ok(Interrupt::USART1),
            28 => Ok(Interrupt::USART2),
            29 => Ok(Interrupt::USART3_4_5_6_7_8),
            30 => Ok(Interrupt::CEC_CAN),
            31 => Ok(Interrupt::USB),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }