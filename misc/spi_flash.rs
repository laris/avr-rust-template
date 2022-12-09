use hal::{
    gpio::{
        gpiob::{PB3, PB4, PB5, PB6},
        Alternate, Output, PushPull, AF0,
    },
    prelude::*,
    spi::{EightBit, Spi},
    spi::{Mode, Phase, Polarity},
    stm32::{self, SPI1},
};
use stm32f0xx_hal as hal;

const PAGE_SIZE: u16 = 256;

const WRITE_ENABLE: u8 = 0x06;
const READ_STATUS_REG: u8 = 0x05;
const DUMMY_BYTE: u8 = 0xff;
const DEVICE_ID: u8 = 0xab;
const JEDEC_DEVICE_ID: u8 = 0x9f;
const SECTOR_ERASE: u8 = 0x20;
const CHIP_ERASE: u8 = 0xc7;
const PAGE_PROGRAM: u8 = 0x02;
const READ_DATA: u8 = 0x03;

pub struct SpiFlash {
    spi_cs: PB6<Output<PushPull>>,
    spi: Spi<SPI1, PB3<Alternate<AF0>>, PB4<Alternate<AF0>>, PB5<Alternate<AF0>>, EightBit>,
}

impl SpiFlash {
    pub fn new() -> Self {
        let mut dp = unsafe { stm32::Peripherals::steal() };
        let mut rcc = dp.RCC.configure().sysclk(48.mhz()).freeze(&mut dp.FLASH);
        let gpioa = dp.GPIOA.split(&mut rcc);
        let gpiob = dp.GPIOB.split(&mut rcc);
        // Configure pins for SPI
        let (sck, miso, mosi) = cortex_m::interrupt::free(move |cs| {
            (
                gpiob.pb3.into_alternate_af0(cs),
                gpiob.pb4.into_alternate_af0(cs),
                gpiob.pb5.into_alternate_af0(cs),
            )
        });

        let mut spi_wp = cortex_m::interrupt::free(|cs| gpiob.pb7.into_push_pull_output(cs));
        let mut spi_hold = cortex_m::interrupt::free(|cs| gpioa.pa15.into_push_pull_output(cs));

        spi_wp.set_high().ok();
        spi_hold.set_high().ok();

        Self {
            spi_cs: cortex_m::interrupt::free(|cs| gpiob.pb6.into_push_pull_output(cs)),
            spi: Spi::spi1(
                dp.SPI1,
                (sck, miso, mosi),
                Mode {
                    polarity: Polarity::IdleHigh,
                    phase: Phase::CaptureOnSecondTransition,
                },
                100_000.hz(),
                &mut rcc,
            ),
        }
    }

    pub fn sector_erase(&mut self, sector_addr: u32) {
        self.write_enable();
        self.spi_cs.set_low().ok();
        self.spi
            .transfer(&mut (SECTOR_ERASE).to_be_bytes())
            .unwrap();
        self.spi
            .transfer(&mut ((sector_addr & 0xff0000) >> 16).to_be_bytes())
            .unwrap();
        self.spi
            .transfer(&mut ((sector_addr & 0xff00) >> 8).to_be_bytes())
            .unwrap();
        self.spi
            .transfer(&mut (sector_addr & 0xff).to_be_bytes())
            .unwrap();
        self.spi_cs.set_high().ok();
        self.wait_for_write_end();
    }

    pub fn bulk_erase(&mut self) {
        self.write_enable();
        self.spi_cs.set_low().ok();
        self.spi.transfer(&mut (CHIP_ERASE).to_be_bytes()).unwrap();
        self.spi_cs.set_high().ok();
        self.wait_for_write_end();
    }

    fn page_write(&mut self, buffer: &[u8], write_addr: &mut u32, num_byte: u16) {
        let mut num: usize = 0;

        self.write_enable();
        self.spi_cs.set_low().ok();
        self.spi
            .transfer(&mut (PAGE_PROGRAM).to_be_bytes())
            .unwrap();
        self.spi
            .transfer(&mut ((*write_addr & 0xff0000) >> 16).to_be_bytes())
            .unwrap();
        self.spi
            .transfer(&mut ((*write_addr & 0xff00) >> 8).to_be_bytes())
            .unwrap();
        self.spi
            .transfer(&mut (*write_addr & 0xff).to_be_bytes())
            .unwrap();

        while num < num_byte.into() {
            self.spi
                .transfer(&mut buffer.get(num).unwrap().to_be_bytes())
                .unwrap();
            num += 1;
        }
        self.spi_cs.set_high().ok();
        self.wait_for_write_end();
    }

    pub fn buffer_write(&mut self, mut buffer: &mut [u8], write_addr: &mut u32, num_byte: u16) {
        let mut addr = 0;
        let mut count: u16 = 0;
        let mut temp = 0;
        let mut num_of_page = 0;
        let mut num_of_single = 0;

        addr = *write_addr % PAGE_SIZE as u32;
        count = PAGE_SIZE - addr as u16;
        num_of_page = num_byte / PAGE_SIZE;
        num_of_single = num_byte % PAGE_SIZE;

        if addr == 0 {
            if num_of_page == 0 {
                self.page_write(buffer, write_addr, num_byte);
            } else {
                while num_of_page > 0 {
                    self.page_write(buffer, write_addr, PAGE_SIZE);
                    *write_addr += PAGE_SIZE as u32;
                    buffer = buffer.get_mut((PAGE_SIZE as usize)..).unwrap();
                    num_of_page -= 1;
                }
                self.page_write(buffer, write_addr, num_of_single);
            }
        } else {
            if num_of_page == 0 {
                if num_of_single > count {
                    temp = num_of_single - count;
                    self.page_write(buffer, write_addr, count);
                    *write_addr += count as u32;
                    buffer = buffer.get_mut((count as usize)..).unwrap();
                    self.page_write(buffer, write_addr, temp);
                } else {
                    self.page_write(buffer, write_addr, num_byte);
                }
            } else {
                let mut num_byte_to_write = num_byte;
                num_byte_to_write -= count;
                num_of_page = num_byte_to_write / PAGE_SIZE;
                num_of_single = num_byte_to_write % PAGE_SIZE;

                self.page_write(buffer, write_addr, count);
                *write_addr += count as u32;
                buffer = buffer.get_mut((count as usize)..).unwrap();

                while num_of_page > 0 {
                    self.page_write(buffer, write_addr, PAGE_SIZE);
                    *write_addr += PAGE_SIZE as u32;
                    buffer = buffer.get_mut((PAGE_SIZE as usize)..).unwrap();
                    num_of_page -= 1;
                }
                if num_of_single != 0 {
                    self.page_write(buffer, write_addr, num_of_single);
                }
            }
        }
    }

    pub fn buffer_read(&mut self, buffer: &mut [u8], read_addr: &mut u32, num_byte: u16) {
        let mut num = 0;
        self.spi_cs.set_low().ok();
        self.spi.transfer(&mut (READ_DATA).to_be_bytes()).unwrap();
        self.spi
            .transfer(&mut ((*read_addr & 0xff0000) >> 16).to_be_bytes())
            .unwrap();
        self.spi
            .transfer(&mut ((*read_addr & 0xff00) >> 8).to_be_bytes())
            .unwrap();
        self.spi
            .transfer(&mut (*read_addr & 0xff).to_be_bytes())
            .unwrap();
        while num < num_byte.into() {
            if let Some(data) = self
                .spi
                .transfer(&mut (DUMMY_BYTE).to_be_bytes())
                .unwrap()
                .first()
            {
                buffer[num] = *data;
            }
            num += 1;
        }
        self.spi_cs.set_high().ok();
    }

    pub fn read_id(&mut self) -> u32 {
        let mut temp: u32 = 0;
        let mut temp0: u32 = 0;
        let mut temp1: u32 = 0;
        let mut temp2: u32 = 0;

        self.spi_cs.set_low().ok();
        self.spi
            .transfer(&mut (JEDEC_DEVICE_ID).to_be_bytes())
            .unwrap();
        if let Some(data) = self
            .spi
            .transfer(&mut (DUMMY_BYTE).to_be_bytes())
            .unwrap()
            .first()
        {
            temp0 = *data as u32;
        }
        if let Some(data) = self
            .spi
            .transfer(&mut (DUMMY_BYTE).to_be_bytes())
            .unwrap()
            .first()
        {
            temp1 = *data as u32;
        }
        if let Some(data) = self
            .spi
            .transfer(&mut (DUMMY_BYTE).to_be_bytes())
            .unwrap()
            .first()
        {
            temp2 = *data as u32;
        }
        self.spi_cs.set_high().ok();

        temp = (temp0 << 16) | (temp1 << 8) | temp2;
        temp
    }

    pub fn read_device_id(&mut self) -> u8 {
        let mut rev_data: u8 = 0x00;

        self.spi_cs.set_low().ok();
        self.spi.transfer(&mut (DEVICE_ID).to_be_bytes()).unwrap();
        self.spi.transfer(&mut (DUMMY_BYTE).to_be_bytes()).unwrap();
        self.spi.transfer(&mut (DUMMY_BYTE).to_be_bytes()).unwrap();
        self.spi.transfer(&mut (DUMMY_BYTE).to_be_bytes()).unwrap();

        if let Some(data) = self
            .spi
            .transfer(&mut (DUMMY_BYTE).to_be_bytes())
            .unwrap()
            .first()
        {
            rev_data = *data;
        }
        self.spi_cs.set_high().ok();
        rev_data
    }

    fn write_enable(&mut self) {
        self.spi_cs.set_low().ok();
        self.spi
            .transfer(&mut (WRITE_ENABLE).to_be_bytes())
            .unwrap();
        self.spi_cs.set_high().ok();
    }

    fn wait_for_write_end(&mut self) {
        let mut flash_status: u8 = 0x01;

        self.spi_cs.set_low().ok();
        self.spi
            .transfer(&mut (READ_STATUS_REG).to_be_bytes())
            .unwrap();
        while flash_status & 0x01 == 0x01 {
            if let Some(data) = self
                .spi
                .transfer(&mut (DUMMY_BYTE).to_be_bytes())
                .unwrap()
                .first()
            {
                flash_status = *data;
            }
        }
        self.spi_cs.set_high().ok();
    }
}
