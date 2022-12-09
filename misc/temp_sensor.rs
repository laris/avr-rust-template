use cortex_m::asm::nop;
use hal::{
    gpio::{gpiob::PB8, OpenDrain, Output},
    prelude::*,
    stm32,
};
use stm32f0xx_hal as hal;

pub struct TempSensor {
    io: PB8<Output<OpenDrain>>,
}

impl TempSensor {
    pub fn new() -> Self {
        let mut dp = unsafe { stm32::Peripherals::steal() };
        let mut rcc = dp
            .RCC
            .configure()
            .hclk(48.mhz())
            .sysclk(48.mhz())
            .freeze(&mut dp.FLASH);
        let gpiob = dp.GPIOB.split(&mut rcc);

        Self {
            io: cortex_m::interrupt::free(|cs| gpiob.pb8.into_open_drain_output(cs)),
        }
    }

    fn my_delay(&mut self, mut num: u32) {
        while num > 0 {
            nop();
            num -= 1;
        }
    }

    fn read_io(&mut self) -> u8 {
        let mut io_statue = 0;

        match self.io.is_high() {
            Ok(true) => io_statue = 1,
            Ok(false) => io_statue = 0,
            _ => unreachable!(),
        };
        io_statue
    }

    fn rst(&mut self) {
        self.io.set_low().ok();
        //add delay 480us
        self.my_delay(480);
        self.io.set_high().ok();
        //add delay 15us
        self.my_delay(15);
    }

    fn answer_check(&mut self) -> u8 {
        let mut delay_num: u8 = 0;

        while self.read_io() == 1 && delay_num < 100 {
            delay_num += 1;
            //add delay 1us
            self.my_delay(1);
        }
        if delay_num >= 100 {
            return 1;
        } else {
            delay_num = 0;
        }
        while self.read_io() == 0 && delay_num < 240 {
            delay_num += 1;
            //add delay 1us

            self.my_delay(1);
        }
        if delay_num >= 240 {
            return 1;
        } else {
            return 0;
        }
    }

    pub fn read_bit(&mut self) -> u8 {
        let mut data: u8 = 0;

        self.io.set_low().ok();
        //add delay 2us
        self.my_delay(2);
        self.io.set_high().ok();
        //add delay 12us , cancel
        data = self.read_io();
        //add delay 50us
        self.my_delay(50);
        data
    }

    fn read_2bit(&mut self) -> u8 {
        let mut data: u8 = 0;

        for _ in 0..2 {
            data = data << 1;
            self.io.set_low().ok();
            //add delay 2us
            self.my_delay(2);
            self.io.set_high().ok();
            //add delay 12 us,cancel
            if self.read_io() == 1 {
                data |= 0x01;
            }
            //add delay 50 us
            self.my_delay(50);
        }
        data
    }

    fn read_byte(&mut self) -> u8 {
        let mut data: u8 = 0;
        let mut temp: u8 = 0;

        for i in 0..8 {
            temp = self.read_bit();
            data = data | (temp << i);
        }
        data
    }

    fn write_bit(&mut self, data: u8) {
        if data > 0 {
            self.io.set_low().ok();
            //add delay 2us
            self.my_delay(2);
            self.io.set_high().ok();
            //add delay 60us
            self.my_delay(60);
        } else {
            self.io.set_low().ok();
            //add delay 60us
            self.my_delay(60);
            self.io.set_high().ok();
            //add delay 2us
            self.my_delay(2);
        }
    }

    pub fn write_byte(&mut self, mut data: u8) {
        let mut data_b: u8 = 0;

        for _ in 0..8 {
            data_b = data & 0x01;
            data = data >> 1;
            self.write_bit(data_b);
        }
    }

    pub fn get_temp(&mut self) -> f32 {
        let mut tl: u8 = 0;
        let mut th: u8 = 0;
        let mut tem: u16 = 0;

        self.rst();
        self.answer_check();
        self.write_byte(0xcc);
        self.write_byte(0x44);
        self.rst();
        self.answer_check();
        self.write_byte(0xcc);
        self.write_byte(0xbe);
        tl = self.read_byte();
        th = self.read_byte();

        // if th > 7 {
        //     th = !th;
        //     tl = !tl;
        //     temp = 0;
        // } else {
        //     temp = 1;
        // }
        tem = th as u16;
        tem <<= 8;
        tem += tl as u16;
        let temperature = tem as f32 * 0.0625;
        temperature
    }
}
