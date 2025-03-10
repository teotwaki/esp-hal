use esp32h2 as pac;
// We need to export this for users to use
pub use pac::Interrupt;

// We need to export this in the hal for the drivers to use
pub(crate) use self::peripherals::*;

crate::peripherals! {
    AES => true,
    APB_SARADC => true,
    ASSIST_DEBUG => true,
    DMA => true,
    // DS => true,
    // ECC => true,
    EFUSE => true,
    GPIO => true,
    // HMAC => true,
    // HP_APM => true,
    // HP_SYS => true,
    I2C0 => true,
    I2C1 => true,
    // I2S0 => true,
    INTERRUPT_CORE0 => true,
    INTPRI => true,
    IO_MUX => true,
    LEDC => true,
    // LP_ANA => true,
    // LP_AON => true,
    // LP_APM => true,
    LP_CLKRST => true,
    // LP_PERI => true,
    // LP_TIMER => true,
    LP_WDT => true,
    MCPWM0 => true,
    MEM_MONITOR => true,
    MODEM_LPCON => true,
    MODEM_SYSCON => true,
    // OTP_DEBUG => true,
    // PARL_IO => true,
    // PAU => true,
    PCNT => true,
    PCR => true,
    PMU => true,
    RMT => true,
    // RNG => true,
    RSA => true,
    SHA => true,
    // SOC_ETM => true,
    SPI0 => true,
    SPI1 => true,
    SPI2 => true,
    SYSTIMER => true,
    // TEE => true,
    TIMG0 => true,
    TIMG1 => true,
    // TRACE => true,
    // TWAI0 => true,
    UART0 => true,
    UART1 => true,
    // UHCI0 => true,
    USB_DEVICE => true,
    RADIO => false,
}
