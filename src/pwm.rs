//use cortex_m_semihosting::hprintln;

use core::marker::PhantomData;
use core::mem;

use cast::{u16, u32};
use core::any::{Any, TypeId};

//use crate::hal;
//use crate::stm32::{TIM1, TIM2, TIM3, TIM4};
use crate::stm32::{TIM1, TIM2, TIM3};
use embedded_hal as hal;

//use afio::MAPR;
//use bb;
use crate::gpio::gpioa::{PA8, PA9, PA10, PA11, PA15};
use crate::gpio::gpiob::{PB3, PB10};
use crate::gpio::{Alternate, PushPull, AF2};
use crate::rcc::{Clocks, Rcc};
use crate::time::Hertz;

pub trait Pins<TIM> {
    //const REMAP: u8;
    const C1: bool;
    const C2: bool;
    const C3: bool;
    const C4: bool;
    type Channels;
}

impl Pins<TIM1>
    for (
        PA8<Alternate<AF2>>,
        PA9<Alternate<AF2>>,
        PA10<Alternate<AF2>>,
        //PA11<Alternate<AF2>>,
    )
{
    //const REMAP: u8 = 0b00;
    const C1: bool = true;
    const C2: bool = true;
    const C3: bool = true;
    const C4: bool = false;
    type Channels = (Pwm<TIM1, C1>, Pwm<TIM1, C2>, Pwm<TIM1, C3>);//, Pwm<TIM1, C4>);
}

impl Pins<TIM2>
    for (
        PA15<Alternate<AF2>>,
        PB3<Alternate<AF2>>,
        PB10<Alternate<AF2>>,
        //PA11<Alternate<AF2>>,
    )
{
    //const REMAP: u8 = 0b00;
    const C1: bool = true;
    const C2: bool = true;
    const C3: bool = true;
    const C4: bool = false;
    type Channels = (Pwm<TIM2, C1>, Pwm<TIM2, C2>, Pwm<TIM2, C3>);//, Pwm<TIM2, C4>);
}
//
//impl Pins<TIM2> for PA0<Alternate<PushPull>> {
//    //const REMAP: u8 = 0b00;
//    const C1: bool = true;
//    const C2: bool = false;
//    const C3: bool = false;
//    const C4: bool = false;
//    type Channels = Pwm<TIM2, C1>;
//}
//
//impl Pins<TIM3>
//    for (
//        PA6<Alternate<PushPull>>,
//        PA7<Alternate<PushPull>>,
//        PB0<Alternate<PushPull>>,
//        PB1<Alternate<PushPull>>,
//    )
//{
//    //const REMAP: u8 = 0b00;
//    const C1: bool = true;
//    const C2: bool = true;
//    const C3: bool = true;
//    const C4: bool = true;
//    type Channels = (Pwm<TIM3, C1>, Pwm<TIM3, C2>, Pwm<TIM3, C3>, Pwm<TIM3, C4>);
//}
//
//impl Pins<TIM3> for (PB0<Alternate<PushPull>>, PB1<Alternate<PushPull>>) {
//    //const REMAP: u8 = 0b00;
//    const C1: bool = false;
//    const C2: bool = false;
//    const C3: bool = true;
//    const C4: bool = true;
//    type Channels = (Pwm<TIM3, C3>, Pwm<TIM3, C4>);
//}
//
//impl Pins<TIM4>
//    for (
//        PB6<Alternate<PushPull>>,
//        PB7<Alternate<PushPull>>,
//        PB8<Alternate<PushPull>>,
//        PB9<Alternate<PushPull>>,
//    )
//{
//    //const REMAP: u8 = 0b0;
//    const C1: bool = true;
//    const C2: bool = true;
//    const C3: bool = true;
//    const C4: bool = true;
//    type Channels = (Pwm<TIM4, C1>, Pwm<TIM4, C2>, Pwm<TIM4, C3>, Pwm<TIM4, C4>);
//}

pub trait PwmExt: Sized {
    fn pwm<PINS, T>(
        self,
        _pins: PINS,
        //mapr: &mut MAPR,
        frequency: T,
        clocks: Clocks,
        //rcc: &mut Rcc,
    ) -> PINS::Channels
    where
        PINS: Pins<Self>,
        T: Into<Hertz>;
}

impl PwmExt for TIM1 {
    fn pwm<PINS, T>(
        self,
        _pins: PINS,
        //mapr: &mut MAPR,
        freq: T,
        clocks: Clocks,
        //apb: &mut APB1,
    ) -> PINS::Channels
    where
        PINS: Pins<Self>,
        T: Into<Hertz>,
    {
        //mapr.mapr()
        //    .modify(|_, w| unsafe { w.tim2_remap().bits(PINS::REMAP) });

        tim1(self, _pins, freq.into(), clocks)
    }
}

impl PwmExt for TIM2 {
    fn pwm<PINS, T>(
        self,
        _pins: PINS,
        //mapr: &mut MAPR,
        freq: T,
        clocks: Clocks,
        //apb: &mut APB1,
    ) -> PINS::Channels
    where
        PINS: Pins<Self>,
        T: Into<Hertz>,
    {
        //mapr.mapr()
        //    .modify(|_, w| unsafe { w.tim2_remap().bits(PINS::REMAP) });

        tim2(self, _pins, freq.into(), clocks)
    }
}
//impl PwmExt for TIM2 {
//    fn pwm<PINS, T>(
//        self,
//        _pins: PINS,
//        mapr: &mut MAPR,
//        freq: T,
//        clocks: Clocks,
//        apb: &mut APB1,
//    ) -> PINS::Channels
//    where
//        PINS: Pins<Self>,
//        T: Into<Hertz>,
//    {
//        mapr.mapr()
//            .modify(|_, w| unsafe { w.tim2_remap().bits(PINS::REMAP) });
//
//        tim2(self, _pins, freq.into(), clocks, apb)
//    }
//}
//
//impl PwmExt for TIM3 {
//    fn pwm<PINS, T>(
//        self,
//        _pins: PINS,
//        mapr: &mut MAPR,
//        freq: T,
//        clocks: Clocks,
//        apb: &mut APB1,
//    ) -> PINS::Channels
//    where
//        PINS: Pins<Self>,
//        T: Into<Hertz>,
//    {
//        mapr.mapr()
//            .modify(|_, w| unsafe { w.tim3_remap().bits(PINS::REMAP) });
//
//        tim3(self, _pins, freq.into(), clocks, apb)
//    }
//}
//
//impl PwmExt for TIM4 {
//    fn pwm<PINS, T>(
//        self,
//        _pins: PINS,
//        mapr: &mut MAPR,
//        freq: T,
//        clocks: Clocks,
//        apb: &mut APB1,
//    ) -> PINS::Channels
//    where
//        PINS: Pins<Self>,
//        T: Into<Hertz>,
//    {
//        mapr.mapr()
//            .modify(|_, w| w.tim4_remap().bit(PINS::REMAP == 1));
//
//        tim4(self, _pins, freq.into(), clocks, apb)
//    }
//}

pub struct Pwm<TIM, CHANNEL> {
    _channel: PhantomData<CHANNEL>,
    _tim: PhantomData<TIM>,
}

pub struct C1;
pub struct C2;
pub struct C3;
pub struct C4;

macro_rules! advanced_timer {
    ($($TIMX:ident: ($timX:ident, $timXen:ident, $timXrst:ident),)+) => {
        $( 
            fn $timX<PINS>(
                tim: $TIMX,
                _pins: PINS,
                freq: Hertz,
                clocks: Clocks,
                //rcc: &mut Rcc,
            ) -> PINS::Channels
            where
                PINS: Pins<$TIMX>,
            {
                let rcc = unsafe { &(*crate::stm32::RCC::ptr()) };
                rcc.apb2enr.modify(|_, w| w.tim1en().set_bit());
                rcc.apb2rstr.modify(|_, w| w.tim1rst().set_bit());
                rcc.apb2rstr.modify(|_, w| w.tim1rst().clear_bit());

                if PINS::C1 {
                    tim.ccmr1_output
                        .modify(|_, w| unsafe { w.oc1pe().set_bit().oc1m().bits(6) });
                }

                if PINS::C2 {
                    tim.ccmr1_output
                        .modify(|_, w| unsafe { w.oc2pe().set_bit().oc2m().bits(6) });
                }

                if PINS::C3 {
                    tim.ccmr2_output
                        .modify(|_, w| unsafe { w.oc3pe().set_bit().oc3m().bits(6) });
                }

                if PINS::C4 {
                    tim.ccmr2_output
                        .modify(|_, w| unsafe { w.oc4pe().set_bit().oc4m().bits(6) });
                }

                let clk = clocks.pclk().0;
                let freq = freq.0;
                let ticks = clk / freq;
                let psc = u16(ticks / (1 << 16)).unwrap();
                tim.psc.write(|w| unsafe { w.psc().bits(psc) });
                
                let arr = u16(ticks / u32(psc + 1)).unwrap() - 1;
                tim.arr.write(|w| unsafe { w.arr().bits(arr) });

                tim.cr1.write(|w| unsafe {
                    w.cms()
                        .bits(0b00)
                        .dir()
                        .clear_bit()
                        .opm()
                        .clear_bit()
                        .cen()
                        .set_bit()
                });

                //$TIMX needs the master out enable bit set to work 
                tim.bdtr.write(|w| unsafe { w.moe().set_bit()});

                unsafe { mem::uninitialized() }
            }
            
            impl hal::PwmPin for Pwm<$TIMX, C1> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc1e().clear_bit()) } 
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc1e().set_bit()) }
                }

                fn get_duty(&self) -> u16 {
                    unsafe { (*$TIMX::ptr()).ccr4.read().ccr4().bits() as u16 } 
                }

                fn get_max_duty(&self) -> u16 {
                    unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 }
                }

                fn set_duty(&mut self, duty: u16) {
                    unsafe { (*$TIMX::ptr()).ccr1.write(|w| w.ccr1().bits(duty as u16))} 
                }
            }

           impl hal::PwmPin for Pwm<$TIMX, C2> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc2e().clear_bit()) }
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc2e().set_bit()) }
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).ccr2.read().ccr2().bits() as u16 }
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 }
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                        unsafe { (*$TIMX::ptr()).ccr2.write(|w| w.ccr2().bits(duty as u16)) }
                }
            } 

           impl hal::PwmPin for Pwm<$TIMX, C3> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc3e().clear_bit()) }
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc3e().set_bit()) }
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).ccr3.read().ccr3().bits() as u16 }
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 }
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                        unsafe { (*$TIMX::ptr()).ccr3.write(|w| w.ccr3().bits(duty as u16)) }
                }
            } 

           impl hal::PwmPin for Pwm<$TIMX, C4> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc4e().clear_bit()) }
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc4e().set_bit()) }
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).ccr4.read().ccr4().bits() as u16 }
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 }
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                        unsafe { (*$TIMX::ptr()).ccr4.write(|w| w.ccr4().bits(duty as u16)) }
                }
            }
        )+
    } 
}

macro_rules! hal {
    ($($TIMX:ident: ($timX:ident, $timXen:ident, $timXrst:ident),)+) => {
        $(    
            fn $timX<PINS>(
                tim: $TIMX,
                _pins: PINS,
                freq: Hertz,
                clocks: Clocks,
                //rcc: &mut Rcc,
            ) -> PINS::Channels
            where
                PINS: Pins<$TIMX>,
            {
                let rcc = unsafe { &(*crate::stm32::RCC::ptr()) };
                    rcc.apb1enr.modify(|_, w| w.$timXen().set_bit());
                    rcc.apb1rstr.modify(|_, w| w.$timXrst().set_bit());
                    rcc.apb1rstr.modify(|_, w| w.$timXrst().clear_bit());

                if PINS::C1 {
                    tim.ccmr1_output
                        .modify(|_, w| unsafe { w.oc1pe().set_bit().oc1m().bits(6) });
                }

                if PINS::C2 {
                    tim.ccmr1_output
                        .modify(|_, w| unsafe { w.oc2pe().set_bit().oc2m().bits(6) });
                }

                if PINS::C3 {
                    tim.ccmr2_output
                        .modify(|_, w| unsafe { w.oc3pe().set_bit().oc3m().bits(6) });
                }

                if PINS::C4 {
                    tim.ccmr2_output
                        .modify(|_, w| unsafe { w.oc4pe().set_bit().oc4m().bits(6) });
                }

                let clk = clocks.pclk().0;
                let freq = freq.0;
                let ticks = clk / freq;
                let psc = u16(ticks / (1 << 16)).unwrap();
                tim.psc.write(|w| unsafe { w.psc().bits(psc) });
                let arr = ticks / u32(psc + 1) - 1;
                tim.arr.write(|w| unsafe { w.arr().bits(arr) });

                tim.cr1.write(|w| unsafe {
                    w.cms()
                        .bits(0b00)
                        .dir()
                        .clear_bit()
                        .opm()
                        .clear_bit()
                        .cen()
                        .set_bit()
                });

                unsafe { mem::uninitialized() }
            }
            
            impl hal::PwmPin for Pwm<$TIMX, C1> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc1e().clear_bit()) } 
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc1e().set_bit()) }
                }

                fn get_duty(&self) -> u16 {
                    unsafe { (*$TIMX::ptr()).ccr4.read().ccr4().bits() as u16 } 
                }

                fn get_max_duty(&self) -> u16 {
                    unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 }
                }

                fn set_duty(&mut self, duty: u16) {
                        unsafe { (*$TIMX::ptr()).ccr1.write(|w| w.ccr1().bits(duty as u32))} 
                }
            }

           impl hal::PwmPin for Pwm<$TIMX, C2> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc2e().clear_bit()) }
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc2e().set_bit()) }
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).ccr2.read().ccr2().bits() as u16 }
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 }
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                        unsafe { (*$TIMX::ptr()).ccr2.write(|w| w.ccr2().bits(duty as u32)) }
                }
            } 

           impl hal::PwmPin for Pwm<$TIMX, C3> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc3e().clear_bit()) }
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc3e().set_bit()) }
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).ccr3.read().ccr3().bits() as u16 }
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 }
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                        unsafe { (*$TIMX::ptr()).ccr3.write(|w| w.ccr3().bits(duty as u32)) }
                }
            } 

           impl hal::PwmPin for Pwm<$TIMX, C4> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc4e().clear_bit()) }
                }

                fn enable(&mut self) {
                    unsafe { (*$TIMX::ptr()).ccer.modify(|_,w| w.cc4e().set_bit()) }
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).ccr4.read().ccr4().bits() as u16 }
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe { (*$TIMX::ptr()).arr.read().arr().bits() as u16 }
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                        unsafe { (*$TIMX::ptr()).ccr4.write(|w| w.ccr4().bits(duty as u32)) }
                }
            } 
        )+ 
    }
}

advanced_timer! {
    TIM1: (tim1, tim1en, tim1rst),
}

hal! {
    TIM2: (tim2, tim2en, tim2rst),
    //TIM3: (tim3, tim3en, tim3rst),
    //TIM4: (tim4, tim4en, tim4rst),
}
