#[allow(non_camel_case)]
#[allow(non_snake_case)]
#[allow(non_upper_camel_case)]
use std::marker::PhantomData;
#[no_mangle]
static mut SESSION: *const ni_fpga::Session = std::ptr::null();
mod types {
    use super::types;
    use ni_fpga_macros::{Cluster, Enum};
    #[derive(Cluster)]
    pub struct AI_Config {
        ScanSize: ni_fpga::fxp::FXP<3, 3, false>,
        ConvertRate: ni_fpga::fxp::FXP<26, 26, false>,
    }
    #[derive(Cluster)]
    pub struct AI_ReadSelect {
        Channel: ni_fpga::fxp::FXP<3, 3, false>,
        Averaged: bool,
    }
    #[derive(Cluster)]
    pub struct ASource {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct Accumulator0_Output {
        Value: i64,
        Count: u32,
    }
    #[derive(Cluster)]
    pub struct Accumulator1_Output {
        Value: i64,
        Count: u32,
    }
    #[derive(Cluster)]
    pub struct AnalogTrigger0_SourceSelect {
        Channel: ni_fpga::fxp::FXP<3, 3, false>,
        Averaged: bool,
        DutyCycle: bool,
        Filter: bool,
        FloatingRollover: bool,
        RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster)]
    pub struct AnalogTrigger1_SourceSelect {
        Channel: ni_fpga::fxp::FXP<3, 3, false>,
        Averaged: bool,
        DutyCycle: bool,
        Filter: bool,
        FloatingRollover: bool,
        RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster)]
    pub struct AnalogTrigger2_SourceSelect {
        Channel: ni_fpga::fxp::FXP<3, 3, false>,
        Averaged: bool,
        DutyCycle: bool,
        Filter: bool,
        FloatingRollover: bool,
        RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster)]
    pub struct AnalogTrigger3_SourceSelect {
        Channel: ni_fpga::fxp::FXP<3, 3, false>,
        Averaged: bool,
        DutyCycle: bool,
        Filter: bool,
        FloatingRollover: bool,
        RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster)]
    pub struct AnalogTrigger4_SourceSelect {
        Channel: ni_fpga::fxp::FXP<3, 3, false>,
        Averaged: bool,
        DutyCycle: bool,
        Filter: bool,
        FloatingRollover: bool,
        RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster)]
    pub struct AnalogTrigger5_SourceSelect {
        Channel: ni_fpga::fxp::FXP<3, 3, false>,
        Averaged: bool,
        DutyCycle: bool,
        Filter: bool,
        FloatingRollover: bool,
        RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster)]
    pub struct AnalogTrigger6_SourceSelect {
        Channel: ni_fpga::fxp::FXP<3, 3, false>,
        Averaged: bool,
        DutyCycle: bool,
        Filter: bool,
        FloatingRollover: bool,
        RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster)]
    pub struct AnalogTrigger7_SourceSelect {
        Channel: ni_fpga::fxp::FXP<3, 3, false>,
        Averaged: bool,
        DutyCycle: bool,
        Filter: bool,
        FloatingRollover: bool,
        RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster)]
    pub struct AnalogTrigger_Output {
        InHysteresis: bool,
        OverLimit: bool,
        Rising: bool,
        Falling: bool,
    }
    #[derive(Cluster)]
    pub struct BSource {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct Counter0_Config {
        UpSource: types::UpSource,
        DownSource: types::DownSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        UpRisingEdge: bool,
        UpFallingEdge: bool,
        DownRisingEdge: bool,
        DownFallingEdge: bool,
        Mode: ni_fpga::fxp::FXP<2, 2, false>,
        PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster)]
    pub struct Counter0_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Counter0_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Counter0_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Counter1_Config {
        UpSource: types::UpSource,
        DownSource: types::DownSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        UpRisingEdge: bool,
        UpFallingEdge: bool,
        DownRisingEdge: bool,
        DownFallingEdge: bool,
        Mode: ni_fpga::fxp::FXP<2, 2, false>,
        PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster)]
    pub struct Counter1_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Counter1_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Counter1_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Counter2_Config {
        UpSource: types::UpSource,
        DownSource: types::DownSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        UpRisingEdge: bool,
        UpFallingEdge: bool,
        DownRisingEdge: bool,
        DownFallingEdge: bool,
        Mode: ni_fpga::fxp::FXP<2, 2, false>,
        PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster)]
    pub struct Counter2_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Counter2_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Counter2_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Counter3_Config {
        UpSource: types::UpSource,
        DownSource: types::DownSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        UpRisingEdge: bool,
        UpFallingEdge: bool,
        DownRisingEdge: bool,
        DownFallingEdge: bool,
        Mode: ni_fpga::fxp::FXP<2, 2, false>,
        PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster)]
    pub struct Counter3_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Counter3_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Counter3_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Counter4_Config {
        UpSource: types::UpSource,
        DownSource: types::DownSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        UpRisingEdge: bool,
        UpFallingEdge: bool,
        DownRisingEdge: bool,
        DownFallingEdge: bool,
        Mode: ni_fpga::fxp::FXP<2, 2, false>,
        PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster)]
    pub struct Counter4_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Counter4_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Counter4_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Counter5_Config {
        UpSource: types::UpSource,
        DownSource: types::DownSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        UpRisingEdge: bool,
        UpFallingEdge: bool,
        DownRisingEdge: bool,
        DownFallingEdge: bool,
        Mode: ni_fpga::fxp::FXP<2, 2, false>,
        PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster)]
    pub struct Counter5_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Counter5_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Counter5_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Counter6_Config {
        UpSource: types::UpSource,
        DownSource: types::DownSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        UpRisingEdge: bool,
        UpFallingEdge: bool,
        DownRisingEdge: bool,
        DownFallingEdge: bool,
        Mode: ni_fpga::fxp::FXP<2, 2, false>,
        PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster)]
    pub struct Counter6_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Counter6_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Counter6_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Counter7_Config {
        UpSource: types::UpSource,
        DownSource: types::DownSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        UpRisingEdge: bool,
        UpFallingEdge: bool,
        DownRisingEdge: bool,
        DownFallingEdge: bool,
        Mode: ni_fpga::fxp::FXP<2, 2, false>,
        PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster)]
    pub struct Counter7_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Counter7_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Counter7_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct DIO_DI {
        Headers: ni_fpga::fxp::FXP<10, 10, false>,
        SPIPort: ni_fpga::fxp::FXP<5, 5, false>,
        Reserved: ni_fpga::fxp::FXP<1, 1, false>,
        MXP: u16,
    }
    #[derive(Cluster)]
    pub struct DIO_DO {
        Headers: ni_fpga::fxp::FXP<10, 10, false>,
        SPIPort: ni_fpga::fxp::FXP<5, 5, false>,
        Reserved: ni_fpga::fxp::FXP<1, 1, false>,
        MXP: u16,
    }
    #[derive(Cluster)]
    pub struct DIO_OutputEnable {
        Headers: ni_fpga::fxp::FXP<10, 10, false>,
        SPIPort: ni_fpga::fxp::FXP<5, 5, false>,
        Reserved: ni_fpga::fxp::FXP<1, 1, false>,
        MXP: u16,
    }
    #[derive(Cluster)]
    pub struct DIO_Pulse {
        Headers: ni_fpga::fxp::FXP<10, 10, false>,
        SPIPort: ni_fpga::fxp::FXP<5, 5, false>,
        Reserved: ni_fpga::fxp::FXP<1, 1, false>,
        MXP: u16,
    }
    #[derive(Cluster)]
    pub struct DMA_Config {
        Pause: bool,
        Enable: types::Enable,
        ExternalClock: bool,
    }
    #[derive(Cluster)]
    pub struct DownSource {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct DutyCycle0_Source {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct DutyCycle1_Source {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct DutyCycle2_Source {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct DutyCycle3_Source {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct DutyCycle4_Source {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct DutyCycle5_Source {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct DutyCycle6_Source {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct DutyCycle7_Source {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct Enable {
        AI0_Low: bool,
        AI0_High: bool,
        AIAveraged0_Low: bool,
        AIAveraged0_High: bool,
        AI1_Low: bool,
        AI1_High: bool,
        AIAveraged1_Low: bool,
        AIAveraged1_High: bool,
        Accumulator0: bool,
        Accumulator1: bool,
        DI: bool,
        AnalogTriggers: bool,
        Counters_Low: bool,
        Counters_High: bool,
        CounterTimers_Low: bool,
        CounterTimers_High: bool,
        Encoders_Low: bool,
        Encoders_High: bool,
        EncoderTimers_Low: bool,
        EncoderTimers_High: bool,
        DutyCycle_Low: bool,
        DutyCycle_High: bool,
    }
    #[derive(Cluster)]
    pub struct Enables {
        AI0_Low: bool,
        AI0_High: bool,
        AIAveraged0_Low: bool,
        AIAveraged0_High: bool,
        AI1_Low: bool,
        AI1_High: bool,
        AIAveraged1_Low: bool,
        AIAveraged1_High: bool,
        Accumulator0: bool,
        Accumulator1: bool,
        DI: bool,
        AnalogTriggers: bool,
        Counters_Low: bool,
        Counters_High: bool,
        CounterTimers_Low: bool,
        CounterTimers_High: bool,
        Encoders_Low: bool,
        Encoders_High: bool,
        EncoderTimers_Low: bool,
        EncoderTimers_High: bool,
        DutyCycle_Low: bool,
        DutyCycle_High: bool,
        Interrupts: bool,
        PWM: bool,
        PWM_MXP: bool,
        Relay_DO_AO: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder0_Config {
        ASource: types::ASource,
        BSource: types::BSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        Reverse: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder0_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Encoder0_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder0_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder1_Config {
        ASource: types::ASource,
        BSource: types::BSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        Reverse: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder1_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Encoder1_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder1_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder2_Config {
        ASource: types::ASource,
        BSource: types::BSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        Reverse: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder2_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Encoder2_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder2_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder3_Config {
        ASource: types::ASource,
        BSource: types::BSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        Reverse: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder3_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Encoder3_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder3_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder4_Config {
        ASource: types::ASource,
        BSource: types::BSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        Reverse: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder4_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Encoder4_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder4_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder5_Config {
        ASource: types::ASource,
        BSource: types::BSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        Reverse: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder5_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Encoder5_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder5_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder6_Config {
        ASource: types::ASource,
        BSource: types::BSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        Reverse: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder6_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Encoder6_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder6_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder7_Config {
        ASource: types::ASource,
        BSource: types::BSource,
        IndexSource: types::IndexSource,
        IndexActiveHigh: bool,
        IndexEdgeSensitive: bool,
        Reverse: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder7_Output {
        Direction: bool,
        Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster)]
    pub struct Encoder7_TimerConfig {
        StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        UpdateWhenEmpty: bool,
    }
    #[derive(Cluster)]
    pub struct Encoder7_TimerOutput {
        Period: ni_fpga::fxp::FXP<23, 24, false>,
        Count: ni_fpga::fxp::FXP<8, 8, true>,
        Stalled: bool,
    }
    #[derive(Cluster)]
    pub struct ExternalClockSource {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct HMB_Config {
        Enables: types::Enables,
    }
    #[derive(Cluster)]
    pub struct IndexSource {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct Interrupt0_Config {
        Source: types::Source,
        RisingEdge: bool,
        FallingEdge: bool,
        WaitForAck: bool,
    }
    #[derive(Cluster)]
    pub struct Interrupt1_Config {
        Source: types::Source,
        RisingEdge: bool,
        FallingEdge: bool,
        WaitForAck: bool,
    }
    #[derive(Cluster)]
    pub struct Interrupt2_Config {
        Source: types::Source,
        RisingEdge: bool,
        FallingEdge: bool,
        WaitForAck: bool,
    }
    #[derive(Cluster)]
    pub struct Interrupt3_Config {
        Source: types::Source,
        RisingEdge: bool,
        FallingEdge: bool,
        WaitForAck: bool,
    }
    #[derive(Cluster)]
    pub struct Interrupt4_Config {
        Source: types::Source,
        RisingEdge: bool,
        FallingEdge: bool,
        WaitForAck: bool,
    }
    #[derive(Cluster)]
    pub struct Interrupt5_Config {
        Source: types::Source,
        RisingEdge: bool,
        FallingEdge: bool,
        WaitForAck: bool,
    }
    #[derive(Cluster)]
    pub struct Interrupt6_Config {
        Source: types::Source,
        RisingEdge: bool,
        FallingEdge: bool,
        WaitForAck: bool,
    }
    #[derive(Cluster)]
    pub struct Interrupt7_Config {
        Source: types::Source,
        RisingEdge: bool,
        FallingEdge: bool,
        WaitForAck: bool,
    }
    #[derive(Cluster)]
    pub struct LEDs {
        Comm: u8,
        Mode: u8,
        RSL: bool,
    }
    #[derive(Cluster)]
    pub struct PWM_Config {
        Period: u16,
        MinHigh: u16,
    }
    #[derive(Cluster)]
    pub struct Power_Disable {
        User3V3: bool,
        User5V: bool,
        User6V: bool,
    }
    #[derive(Cluster)]
    pub struct Power_FaultCounts {
        OverCurrentFaultCount3V3: u8,
        OverCurrentFaultCount5V: u8,
        OverCurrentFaultCount6V: u8,
        UnderVoltageFaultCount5V: u8,
    }
    #[derive(Cluster)]
    pub struct Power_Status {
        User3V3: u8,
        User5V: u8,
        User6V: u8,
    }
    #[derive(Cluster)]
    pub struct Relay_Value {
        Forward: ni_fpga::fxp::FXP<4, 4, false>,
        Reverse: ni_fpga::fxp::FXP<4, 4, false>,
    }
    #[derive(Cluster)]
    pub struct SPI_AutoByteCount {
        TxByteCount: ni_fpga::fxp::FXP<5, 5, false>,
        ZeroByteCount: ni_fpga::fxp::FXP<7, 7, false>,
    }
    #[derive(Cluster)]
    pub struct SPI_AutoTriggerConfig {
        ExternalClockSource: types::ExternalClockSource,
        RisingEdge: bool,
        FallingEdge: bool,
        ExternalClock: bool,
    }
    #[derive(Cluster)]
    pub struct SPI_ChipSelectActiveHigh {
        Hdr: ni_fpga::fxp::FXP<4, 4, false>,
        MXP: ni_fpga::fxp::FXP<1, 1, false>,
    }
    #[derive(Cluster)]
    pub struct SPI_StallConfig {
        Pow2BytesPerRead: u8,
        StallTicks: u16,
        CsToSclkTicks: u8,
    }
    #[derive(Cluster)]
    pub struct Source {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Cluster)]
    pub struct SysWatchdog_Status {
        SystemActive: bool,
        PowerAlive: bool,
        SysDisableCount: ni_fpga::fxp::FXP<15, 15, false>,
        PowerDisableCount: ni_fpga::fxp::FXP<15, 15, false>,
    }
    #[derive(Cluster)]
    pub struct Trigger {
        ExternalClockSource: types::ExternalClockSource,
        RisingEdge: bool,
        FallingEdge: bool,
    }
    #[derive(Cluster)]
    pub struct UpSource {
        Channel: ni_fpga::fxp::FXP<4, 4, false>,
        Module: ni_fpga::fxp::FXP<1, 1, false>,
        AnalogTrigger: bool,
    }
    #[derive(Enum)]
    pub enum SPI_DebugState {
        Idle,
        Check_Window,
        Check_Available,
        Set_Fifo_Mark,
        Enable_SPI,
        Stuff_Fifo,
        Check_Mark,
        Shuffle_Data,
        Disable,
    }
}
pub struct LocalTime {
    _marker: PhantomData<*const ()>,
}
impl LocalTime {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98304)
    }
}
pub struct Revision {
    _marker: PhantomData<*const ()>,
}
impl Revision {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98308)
    }
}
pub struct Version {
    _marker: PhantomData<*const ()>,
}
impl Version {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98314)
    }
}
pub struct LocalTimeUpper {
    _marker: PhantomData<*const ()>,
}
impl LocalTimeUpper {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98316)
    }
}
pub struct LEDs {
    _marker: PhantomData<*const ()>,
}
impl LEDs {
    pub fn read(&self) -> Result<types::LEDs, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98320)
    }
    pub fn write(&self, value: &types::LEDs) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98320, value)
    }
}
pub struct UserButton {
    _marker: PhantomData<*const ()>,
}
impl UserButton {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98326)
    }
}
pub struct InterruptForceOnce {
    _marker: PhantomData<*const ()>,
}
impl InterruptForceOnce {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98330)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98330, value)
    }
}
pub struct InterruptForceNumber {
    _marker: PhantomData<*const ()>,
}
impl InterruptForceNumber {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98334)
    }
    pub fn write(&self, value: &u8) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98334, value)
    }
}
pub struct SysWatchdog_Status {
    _marker: PhantomData<*const ()>,
}
impl SysWatchdog_Status {
    pub fn read(&self) -> Result<types::SysWatchdog_Status, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98336)
    }
}
pub struct SysWatchdog_Command {
    _marker: PhantomData<*const ()>,
}
impl SysWatchdog_Command {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98340)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98340, value)
    }
}
pub struct SysWatchdog_Challenge {
    _marker: PhantomData<*const ()>,
}
impl SysWatchdog_Challenge {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98344)
    }
}
pub struct SysWatchdog_Timer {
    _marker: PhantomData<*const ()>,
}
impl SysWatchdog_Timer {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98348)
    }
}
pub struct SysWatchdog_Active {
    _marker: PhantomData<*const ()>,
}
impl SysWatchdog_Active {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98354)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98354, value)
    }
}
pub struct SysWatchdog_ForcedKills {
    _marker: PhantomData<*const ()>,
}
impl SysWatchdog_ForcedKills {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<15, 15, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98358)
    }
}
pub struct AI_ReadSelect {
    _marker: PhantomData<*const ()>,
}
impl AI_ReadSelect {
    pub fn read(&self) -> Result<types::AI_ReadSelect, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98362)
    }
    pub fn write(&self, value: &types::AI_ReadSelect) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98362, value)
    }
}
pub struct AI_LatchOutput {
    _marker: PhantomData<*const ()>,
}
impl AI_LatchOutput {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98366)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98366, value)
    }
}
pub struct AI_Output {
    _marker: PhantomData<*const ()>,
}
impl AI_Output {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98368)
    }
}
pub struct AI_Config {
    _marker: PhantomData<*const ()>,
}
impl AI_Config {
    pub fn read(&self) -> Result<types::AI_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98372)
    }
    pub fn write(&self, value: &types::AI_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98372, value)
    }
}
pub struct AI_ScanList {
    _marker: PhantomData<*const ()>,
}
impl AI_ScanList {
    pub fn read(&self) -> Result<[ni_fpga::fxp::FXP<3, 3, false>; 8], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98376)
    }
    pub fn write(&self, value: &[ni_fpga::fxp::FXP<3, 3, false>; 8]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98376, value)
    }
}
pub struct AI_OversampleBits {
    _marker: PhantomData<*const ()>,
}
impl AI_OversampleBits {
    pub fn read(&self) -> Result<[ni_fpga::fxp::FXP<4, 4, false>; 8], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98380)
    }
    pub fn write(&self, value: &[ni_fpga::fxp::FXP<4, 4, false>; 8]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98380, value)
    }
}
pub struct AI_AverageBits {
    _marker: PhantomData<*const ()>,
}
impl AI_AverageBits {
    pub fn read(&self) -> Result<[ni_fpga::fxp::FXP<4, 4, false>; 8], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98384)
    }
    pub fn write(&self, value: &[ni_fpga::fxp::FXP<4, 4, false>; 8]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98384, value)
    }
}
pub struct AI_LoopTiming {
    _marker: PhantomData<*const ()>,
}
impl AI_LoopTiming {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98388)
    }
}
pub struct Accumulator0_Center {
    _marker: PhantomData<*const ()>,
}
impl Accumulator0_Center {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98392)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98392, value)
    }
}
pub struct Accumulator0_Reset {
    _marker: PhantomData<*const ()>,
}
impl Accumulator0_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98398)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98398, value)
    }
}
pub struct Accumulator0_Output {
    _marker: PhantomData<*const ()>,
}
impl Accumulator0_Output {
    pub fn read(&self) -> Result<types::Accumulator0_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98400)
    }
}
pub struct Accumulator0_Deadband {
    _marker: PhantomData<*const ()>,
}
impl Accumulator0_Deadband {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98404)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98404, value)
    }
}
pub struct Accumulator1_Center {
    _marker: PhantomData<*const ()>,
}
impl Accumulator1_Center {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98408)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98408, value)
    }
}
pub struct Accumulator1_Reset {
    _marker: PhantomData<*const ()>,
}
impl Accumulator1_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98414)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98414, value)
    }
}
pub struct Accumulator1_Output {
    _marker: PhantomData<*const ()>,
}
impl Accumulator1_Output {
    pub fn read(&self) -> Result<types::Accumulator1_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98416)
    }
}
pub struct Accumulator1_Deadband {
    _marker: PhantomData<*const ()>,
}
impl Accumulator1_Deadband {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98420)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98420, value)
    }
}
pub struct AnalogTrigger_Output {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger_Output {
    pub fn read(&self) -> Result<[types::AnalogTrigger_Output; 8], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98424)
    }
}
pub struct AnalogTrigger0_SourceSelect {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger0_SourceSelect {
    pub fn read(&self) -> Result<types::AnalogTrigger0_SourceSelect, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98430)
    }
    pub fn write(&self, value: &types::AnalogTrigger0_SourceSelect) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98430, value)
    }
}
pub struct AnalogTrigger0_UpperLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger0_UpperLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98432)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98432, value)
    }
}
pub struct AnalogTrigger0_LowerLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger0_LowerLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98436)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98436, value)
    }
}
pub struct AnalogTrigger1_SourceSelect {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger1_SourceSelect {
    pub fn read(&self) -> Result<types::AnalogTrigger1_SourceSelect, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98442)
    }
    pub fn write(&self, value: &types::AnalogTrigger1_SourceSelect) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98442, value)
    }
}
pub struct AnalogTrigger1_UpperLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger1_UpperLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98444)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98444, value)
    }
}
pub struct AnalogTrigger1_LowerLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger1_LowerLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98448)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98448, value)
    }
}
pub struct AnalogTrigger2_SourceSelect {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger2_SourceSelect {
    pub fn read(&self) -> Result<types::AnalogTrigger2_SourceSelect, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98454)
    }
    pub fn write(&self, value: &types::AnalogTrigger2_SourceSelect) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98454, value)
    }
}
pub struct AnalogTrigger2_UpperLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger2_UpperLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98456)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98456, value)
    }
}
pub struct AnalogTrigger2_LowerLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger2_LowerLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98460)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98460, value)
    }
}
pub struct AnalogTrigger3_SourceSelect {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger3_SourceSelect {
    pub fn read(&self) -> Result<types::AnalogTrigger3_SourceSelect, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98466)
    }
    pub fn write(&self, value: &types::AnalogTrigger3_SourceSelect) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98466, value)
    }
}
pub struct AnalogTrigger3_UpperLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger3_UpperLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98468)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98468, value)
    }
}
pub struct AnalogTrigger3_LowerLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger3_LowerLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98472)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98472, value)
    }
}
pub struct AnalogTrigger4_SourceSelect {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger4_SourceSelect {
    pub fn read(&self) -> Result<types::AnalogTrigger4_SourceSelect, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98478)
    }
    pub fn write(&self, value: &types::AnalogTrigger4_SourceSelect) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98478, value)
    }
}
pub struct AnalogTrigger4_UpperLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger4_UpperLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98480)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98480, value)
    }
}
pub struct AnalogTrigger4_LowerLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger4_LowerLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98484)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98484, value)
    }
}
pub struct AnalogTrigger5_SourceSelect {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger5_SourceSelect {
    pub fn read(&self) -> Result<types::AnalogTrigger5_SourceSelect, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98490)
    }
    pub fn write(&self, value: &types::AnalogTrigger5_SourceSelect) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98490, value)
    }
}
pub struct AnalogTrigger6_UpperLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger6_UpperLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98492)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98492, value)
    }
}
pub struct AnalogTrigger6_LowerLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger6_LowerLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98496)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98496, value)
    }
}
pub struct AnalogTrigger6_SourceSelect {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger6_SourceSelect {
    pub fn read(&self) -> Result<types::AnalogTrigger6_SourceSelect, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98502)
    }
    pub fn write(&self, value: &types::AnalogTrigger6_SourceSelect) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98502, value)
    }
}
pub struct AnalogTrigger5_UpperLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger5_UpperLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98504)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98504, value)
    }
}
pub struct AnalogTrigger5_LowerLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger5_LowerLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98508)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98508, value)
    }
}
pub struct AnalogTrigger7_SourceSelect {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger7_SourceSelect {
    pub fn read(&self) -> Result<types::AnalogTrigger7_SourceSelect, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98514)
    }
    pub fn write(&self, value: &types::AnalogTrigger7_SourceSelect) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98514, value)
    }
}
pub struct AnalogTrigger7_UpperLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger7_UpperLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98516)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98516, value)
    }
}
pub struct AnalogTrigger7_LowerLimit {
    _marker: PhantomData<*const ()>,
}
impl AnalogTrigger7_LowerLimit {
    pub fn read(&self) -> Result<i32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98520)
    }
    pub fn write(&self, value: &i32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98520, value)
    }
}
pub struct PWM_LoopTiming {
    _marker: PhantomData<*const ()>,
}
impl PWM_LoopTiming {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98526)
    }
}
pub struct PWM_CycleStartTimeUpper {
    _marker: PhantomData<*const ()>,
}
impl PWM_CycleStartTimeUpper {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98528)
    }
}
pub struct PWM_CycleStartTime {
    _marker: PhantomData<*const ()>,
}
impl PWM_CycleStartTime {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98532)
    }
}
pub struct PWM_Config {
    _marker: PhantomData<*const ()>,
}
impl PWM_Config {
    pub fn read(&self) -> Result<types::PWM_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98536)
    }
    pub fn write(&self, value: &types::PWM_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98536, value)
    }
}
pub struct PWM_PeriodScaleHdr {
    _marker: PhantomData<*const ()>,
}
impl PWM_PeriodScaleHdr {
    pub fn read(&self) -> Result<[ni_fpga::fxp::FXP<2, 2, false>; 10], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98540)
    }
    pub fn write(
        &self,
        value: &[ni_fpga::fxp::FXP<2, 2, false>; 10],
    ) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98540, value)
    }
}
pub struct PWM_PeriodScaleMXP {
    _marker: PhantomData<*const ()>,
}
impl PWM_PeriodScaleMXP {
    pub fn read(&self) -> Result<[ni_fpga::fxp::FXP<2, 2, false>; 10], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98544)
    }
    pub fn write(
        &self,
        value: &[ni_fpga::fxp::FXP<2, 2, false>; 10],
    ) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98544, value)
    }
}
pub struct PWM_ZeroLatch {
    _marker: PhantomData<*const ()>,
}
impl PWM_ZeroLatch {
    pub fn read(&self) -> Result<[bool; 20], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98548)
    }
    pub fn write(&self, value: &[bool; 20]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98548, value)
    }
}
pub struct PWM_Hdr0 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr0 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98554)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98554, value)
    }
}
pub struct PWM_Hdr1 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr1 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98558)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98558, value)
    }
}
pub struct PWM_Hdr2 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr2 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98562)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98562, value)
    }
}
pub struct PWM_Hdr3 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr3 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98566)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98566, value)
    }
}
pub struct PWM_Hdr4 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr4 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98570)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98570, value)
    }
}
pub struct PWM_Hdr5 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr5 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98574)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98574, value)
    }
}
pub struct PWM_Hdr6 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr6 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98578)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98578, value)
    }
}
pub struct PWM_Hdr7 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr7 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98582)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98582, value)
    }
}
pub struct PWM_Hdr8 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr8 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98586)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98586, value)
    }
}
pub struct PWM_Hdr9 {
    _marker: PhantomData<*const ()>,
}
impl PWM_Hdr9 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98590)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98590, value)
    }
}
pub struct PWM_MXP0 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP0 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98594)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98594, value)
    }
}
pub struct PWM_MXP1 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP1 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98598)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98598, value)
    }
}
pub struct PWM_MXP2 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP2 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98602)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98602, value)
    }
}
pub struct PWM_MXP3 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP3 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98606)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98606, value)
    }
}
pub struct PWM_MXP4 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP4 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98610)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98610, value)
    }
}
pub struct PWM_MXP5 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP5 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98614)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98614, value)
    }
}
pub struct PWM_MXP6 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP6 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98618)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98618, value)
    }
}
pub struct PWM_MXP7 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP7 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98622)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98622, value)
    }
}
pub struct PWM_MXP8 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP8 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98626)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98626, value)
    }
}
pub struct PWM_MXP9 {
    _marker: PhantomData<*const ()>,
}
impl PWM_MXP9 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<12, 12, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98630)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<12, 12, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98630, value)
    }
}
pub struct DIO_OutputEnable {
    _marker: PhantomData<*const ()>,
}
impl DIO_OutputEnable {
    pub fn read(&self) -> Result<types::DIO_OutputEnable, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98632)
    }
    pub fn write(&self, value: &types::DIO_OutputEnable) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98632, value)
    }
}
pub struct DIO_DO {
    _marker: PhantomData<*const ()>,
}
impl DIO_DO {
    pub fn read(&self) -> Result<types::DIO_DO, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98636)
    }
    pub fn write(&self, value: &types::DIO_DO) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98636, value)
    }
}
pub struct DIO_DI {
    _marker: PhantomData<*const ()>,
}
impl DIO_DI {
    pub fn read(&self) -> Result<types::DIO_DI, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98640)
    }
}
pub struct DIO_FilterSelectHdr {
    _marker: PhantomData<*const ()>,
}
impl DIO_FilterSelectHdr {
    pub fn read(&self) -> Result<[ni_fpga::fxp::FXP<2, 2, false>; 16], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98644)
    }
    pub fn write(
        &self,
        value: &[ni_fpga::fxp::FXP<2, 2, false>; 16],
    ) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98644, value)
    }
}
pub struct DIO_FilterPeriodHdr0 {
    _marker: PhantomData<*const ()>,
}
impl DIO_FilterPeriodHdr0 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<24, 24, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98648)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<24, 24, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98648, value)
    }
}
pub struct DIO_FilterPeriodHdr1 {
    _marker: PhantomData<*const ()>,
}
impl DIO_FilterPeriodHdr1 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<24, 24, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98652)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<24, 24, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98652, value)
    }
}
pub struct DIO_FilterPeriodHdr2 {
    _marker: PhantomData<*const ()>,
}
impl DIO_FilterPeriodHdr2 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<24, 24, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98656)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<24, 24, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98656, value)
    }
}
pub struct DIO_FilterSelectMXP {
    _marker: PhantomData<*const ()>,
}
impl DIO_FilterSelectMXP {
    pub fn read(&self) -> Result<[ni_fpga::fxp::FXP<2, 2, false>; 16], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98660)
    }
    pub fn write(
        &self,
        value: &[ni_fpga::fxp::FXP<2, 2, false>; 16],
    ) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98660, value)
    }
}
pub struct DIO_FilterPeriodMXP0 {
    _marker: PhantomData<*const ()>,
}
impl DIO_FilterPeriodMXP0 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<24, 24, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98664)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<24, 24, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98664, value)
    }
}
pub struct DIO_FilterPeriodMXP1 {
    _marker: PhantomData<*const ()>,
}
impl DIO_FilterPeriodMXP1 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<24, 24, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98668)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<24, 24, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98668, value)
    }
}
pub struct DIO_FilterPeriodMXP2 {
    _marker: PhantomData<*const ()>,
}
impl DIO_FilterPeriodMXP2 {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<24, 24, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98672)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<24, 24, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98672, value)
    }
}
pub struct DIO_EnableMXPSpecialFunction {
    _marker: PhantomData<*const ()>,
}
impl DIO_EnableMXPSpecialFunction {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98678)
    }
    pub fn write(&self, value: &u16) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98678, value)
    }
}
pub struct DIO_PulseLength {
    _marker: PhantomData<*const ()>,
}
impl DIO_PulseLength {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98682)
    }
    pub fn write(&self, value: &u16) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98682, value)
    }
}
pub struct DIO_Pulse {
    _marker: PhantomData<*const ()>,
}
impl DIO_Pulse {
    pub fn read(&self) -> Result<types::DIO_Pulse, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98684)
    }
    pub fn write(&self, value: &types::DIO_Pulse) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98684, value)
    }
}
pub struct DIO_PWMDutyCycleA {
    _marker: PhantomData<*const ()>,
}
impl DIO_PWMDutyCycleA {
    pub fn read(&self) -> Result<[u8; 4], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98688)
    }
    pub fn write(&self, value: &[u8; 4]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98688, value)
    }
}
pub struct DIO_PWMDutyCycleB {
    _marker: PhantomData<*const ()>,
}
impl DIO_PWMDutyCycleB {
    pub fn read(&self) -> Result<[u8; 2], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98694)
    }
    pub fn write(&self, value: &[u8; 2]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98694, value)
    }
}
pub struct DIO_PWMOutputSelect {
    _marker: PhantomData<*const ()>,
}
impl DIO_PWMOutputSelect {
    pub fn read(&self) -> Result<[ni_fpga::fxp::FXP<5, 5, false>; 6], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98696)
    }
    pub fn write(&self, value: &[ni_fpga::fxp::FXP<5, 5, false>; 6]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98696, value)
    }
}
pub struct DIO_PWMPeriodPower {
    _marker: PhantomData<*const ()>,
}
impl DIO_PWMPeriodPower {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98702)
    }
    pub fn write(&self, value: &u16) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98702, value)
    }
}
pub struct Counter0_Config {
    _marker: PhantomData<*const ()>,
}
impl Counter0_Config {
    pub fn read(&self) -> Result<types::Counter0_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98704)
    }
    pub fn write(&self, value: &types::Counter0_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98704, value)
    }
}
pub struct Counter0_Reset {
    _marker: PhantomData<*const ()>,
}
impl Counter0_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98710)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98710, value)
    }
}
pub struct Counter0_Output {
    _marker: PhantomData<*const ()>,
}
impl Counter0_Output {
    pub fn read(&self) -> Result<types::Counter0_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98712)
    }
}
pub struct Counter0_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Counter0_TimerConfig {
    pub fn read(&self) -> Result<types::Counter0_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98716)
    }
    pub fn write(&self, value: &types::Counter0_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98716, value)
    }
}
pub struct Counter0_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Counter0_TimerOutput {
    pub fn read(&self) -> Result<types::Counter0_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98720)
    }
}
pub struct Counter1_Config {
    _marker: PhantomData<*const ()>,
}
impl Counter1_Config {
    pub fn read(&self) -> Result<types::Counter1_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98724)
    }
    pub fn write(&self, value: &types::Counter1_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98724, value)
    }
}
pub struct Counter1_Reset {
    _marker: PhantomData<*const ()>,
}
impl Counter1_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98730)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98730, value)
    }
}
pub struct Counter1_Output {
    _marker: PhantomData<*const ()>,
}
impl Counter1_Output {
    pub fn read(&self) -> Result<types::Counter1_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98732)
    }
}
pub struct Counter1_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Counter1_TimerConfig {
    pub fn read(&self) -> Result<types::Counter1_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98736)
    }
    pub fn write(&self, value: &types::Counter1_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98736, value)
    }
}
pub struct Counter1_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Counter1_TimerOutput {
    pub fn read(&self) -> Result<types::Counter1_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98740)
    }
}
pub struct Counter2_Config {
    _marker: PhantomData<*const ()>,
}
impl Counter2_Config {
    pub fn read(&self) -> Result<types::Counter2_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98744)
    }
    pub fn write(&self, value: &types::Counter2_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98744, value)
    }
}
pub struct Counter2_Reset {
    _marker: PhantomData<*const ()>,
}
impl Counter2_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98750)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98750, value)
    }
}
pub struct Counter2_Output {
    _marker: PhantomData<*const ()>,
}
impl Counter2_Output {
    pub fn read(&self) -> Result<types::Counter2_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98752)
    }
}
pub struct Counter2_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Counter2_TimerConfig {
    pub fn read(&self) -> Result<types::Counter2_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98756)
    }
    pub fn write(&self, value: &types::Counter2_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98756, value)
    }
}
pub struct Counter2_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Counter2_TimerOutput {
    pub fn read(&self) -> Result<types::Counter2_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98760)
    }
}
pub struct Counter3_Config {
    _marker: PhantomData<*const ()>,
}
impl Counter3_Config {
    pub fn read(&self) -> Result<types::Counter3_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98764)
    }
    pub fn write(&self, value: &types::Counter3_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98764, value)
    }
}
pub struct Counter3_Reset {
    _marker: PhantomData<*const ()>,
}
impl Counter3_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98770)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98770, value)
    }
}
pub struct Counter3_Output {
    _marker: PhantomData<*const ()>,
}
impl Counter3_Output {
    pub fn read(&self) -> Result<types::Counter3_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98772)
    }
}
pub struct Counter3_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Counter3_TimerConfig {
    pub fn read(&self) -> Result<types::Counter3_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98776)
    }
    pub fn write(&self, value: &types::Counter3_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98776, value)
    }
}
pub struct Counter3_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Counter3_TimerOutput {
    pub fn read(&self) -> Result<types::Counter3_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98780)
    }
}
pub struct Counter4_Config {
    _marker: PhantomData<*const ()>,
}
impl Counter4_Config {
    pub fn read(&self) -> Result<types::Counter4_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98784)
    }
    pub fn write(&self, value: &types::Counter4_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98784, value)
    }
}
pub struct Counter4_Reset {
    _marker: PhantomData<*const ()>,
}
impl Counter4_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98790)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98790, value)
    }
}
pub struct Counter4_Output {
    _marker: PhantomData<*const ()>,
}
impl Counter4_Output {
    pub fn read(&self) -> Result<types::Counter4_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98792)
    }
}
pub struct Counter4_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Counter4_TimerConfig {
    pub fn read(&self) -> Result<types::Counter4_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98796)
    }
    pub fn write(&self, value: &types::Counter4_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98796, value)
    }
}
pub struct Counter4_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Counter4_TimerOutput {
    pub fn read(&self) -> Result<types::Counter4_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98800)
    }
}
pub struct Counter5_Config {
    _marker: PhantomData<*const ()>,
}
impl Counter5_Config {
    pub fn read(&self) -> Result<types::Counter5_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98804)
    }
    pub fn write(&self, value: &types::Counter5_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98804, value)
    }
}
pub struct Counter5_Reset {
    _marker: PhantomData<*const ()>,
}
impl Counter5_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98810)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98810, value)
    }
}
pub struct Counter5_Output {
    _marker: PhantomData<*const ()>,
}
impl Counter5_Output {
    pub fn read(&self) -> Result<types::Counter5_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98812)
    }
}
pub struct Counter5_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Counter5_TimerConfig {
    pub fn read(&self) -> Result<types::Counter5_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98816)
    }
    pub fn write(&self, value: &types::Counter5_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98816, value)
    }
}
pub struct Counter5_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Counter5_TimerOutput {
    pub fn read(&self) -> Result<types::Counter5_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98820)
    }
}
pub struct Counter6_Config {
    _marker: PhantomData<*const ()>,
}
impl Counter6_Config {
    pub fn read(&self) -> Result<types::Counter6_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98824)
    }
    pub fn write(&self, value: &types::Counter6_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98824, value)
    }
}
pub struct Counter6_Reset {
    _marker: PhantomData<*const ()>,
}
impl Counter6_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98830)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98830, value)
    }
}
pub struct Counter6_Output {
    _marker: PhantomData<*const ()>,
}
impl Counter6_Output {
    pub fn read(&self) -> Result<types::Counter6_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98832)
    }
}
pub struct Counter6_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Counter6_TimerConfig {
    pub fn read(&self) -> Result<types::Counter6_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98836)
    }
    pub fn write(&self, value: &types::Counter6_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98836, value)
    }
}
pub struct Counter6_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Counter6_TimerOutput {
    pub fn read(&self) -> Result<types::Counter6_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98840)
    }
}
pub struct Counter7_Config {
    _marker: PhantomData<*const ()>,
}
impl Counter7_Config {
    pub fn read(&self) -> Result<types::Counter7_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98844)
    }
    pub fn write(&self, value: &types::Counter7_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98844, value)
    }
}
pub struct Counter7_Reset {
    _marker: PhantomData<*const ()>,
}
impl Counter7_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98850)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98850, value)
    }
}
pub struct Counter7_Output {
    _marker: PhantomData<*const ()>,
}
impl Counter7_Output {
    pub fn read(&self) -> Result<types::Counter7_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98852)
    }
}
pub struct Counter7_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Counter7_TimerConfig {
    pub fn read(&self) -> Result<types::Counter7_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98856)
    }
    pub fn write(&self, value: &types::Counter7_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98856, value)
    }
}
pub struct Counter7_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Counter7_TimerOutput {
    pub fn read(&self) -> Result<types::Counter7_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98860)
    }
}
pub struct Encoder0_Config {
    _marker: PhantomData<*const ()>,
}
impl Encoder0_Config {
    pub fn read(&self) -> Result<types::Encoder0_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98864)
    }
    pub fn write(&self, value: &types::Encoder0_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98864, value)
    }
}
pub struct Encoder0_Reset {
    _marker: PhantomData<*const ()>,
}
impl Encoder0_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98870)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98870, value)
    }
}
pub struct Encoder0_Output {
    _marker: PhantomData<*const ()>,
}
impl Encoder0_Output {
    pub fn read(&self) -> Result<types::Encoder0_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98872)
    }
}
pub struct Encoder0_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Encoder0_TimerConfig {
    pub fn read(&self) -> Result<types::Encoder0_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98876)
    }
    pub fn write(&self, value: &types::Encoder0_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98876, value)
    }
}
pub struct Encoder0_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Encoder0_TimerOutput {
    pub fn read(&self) -> Result<types::Encoder0_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98880)
    }
}
pub struct Encoder1_Config {
    _marker: PhantomData<*const ()>,
}
impl Encoder1_Config {
    pub fn read(&self) -> Result<types::Encoder1_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98884)
    }
    pub fn write(&self, value: &types::Encoder1_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98884, value)
    }
}
pub struct Encoder1_Reset {
    _marker: PhantomData<*const ()>,
}
impl Encoder1_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98890)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98890, value)
    }
}
pub struct Encoder1_Output {
    _marker: PhantomData<*const ()>,
}
impl Encoder1_Output {
    pub fn read(&self) -> Result<types::Encoder1_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98892)
    }
}
pub struct Encoder1_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Encoder1_TimerConfig {
    pub fn read(&self) -> Result<types::Encoder1_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98896)
    }
    pub fn write(&self, value: &types::Encoder1_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98896, value)
    }
}
pub struct Encoder1_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Encoder1_TimerOutput {
    pub fn read(&self) -> Result<types::Encoder1_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98900)
    }
}
pub struct Encoder2_Config {
    _marker: PhantomData<*const ()>,
}
impl Encoder2_Config {
    pub fn read(&self) -> Result<types::Encoder2_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98904)
    }
    pub fn write(&self, value: &types::Encoder2_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98904, value)
    }
}
pub struct Encoder2_Reset {
    _marker: PhantomData<*const ()>,
}
impl Encoder2_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98910)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98910, value)
    }
}
pub struct Encoder2_Output {
    _marker: PhantomData<*const ()>,
}
impl Encoder2_Output {
    pub fn read(&self) -> Result<types::Encoder2_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98912)
    }
}
pub struct Encoder2_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Encoder2_TimerConfig {
    pub fn read(&self) -> Result<types::Encoder2_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98916)
    }
    pub fn write(&self, value: &types::Encoder2_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98916, value)
    }
}
pub struct Encoder2_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Encoder2_TimerOutput {
    pub fn read(&self) -> Result<types::Encoder2_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98920)
    }
}
pub struct Encoder3_Config {
    _marker: PhantomData<*const ()>,
}
impl Encoder3_Config {
    pub fn read(&self) -> Result<types::Encoder3_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98924)
    }
    pub fn write(&self, value: &types::Encoder3_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98924, value)
    }
}
pub struct Encoder3_Reset {
    _marker: PhantomData<*const ()>,
}
impl Encoder3_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98930)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98930, value)
    }
}
pub struct Encoder3_Output {
    _marker: PhantomData<*const ()>,
}
impl Encoder3_Output {
    pub fn read(&self) -> Result<types::Encoder3_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98932)
    }
}
pub struct Encoder3_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Encoder3_TimerConfig {
    pub fn read(&self) -> Result<types::Encoder3_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98936)
    }
    pub fn write(&self, value: &types::Encoder3_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98936, value)
    }
}
pub struct Encoder3_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Encoder3_TimerOutput {
    pub fn read(&self) -> Result<types::Encoder3_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98940)
    }
}
pub struct Encoder4_Config {
    _marker: PhantomData<*const ()>,
}
impl Encoder4_Config {
    pub fn read(&self) -> Result<types::Encoder4_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98944)
    }
    pub fn write(&self, value: &types::Encoder4_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98944, value)
    }
}
pub struct Encoder4_Reset {
    _marker: PhantomData<*const ()>,
}
impl Encoder4_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98950)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98950, value)
    }
}
pub struct Encoder4_Output {
    _marker: PhantomData<*const ()>,
}
impl Encoder4_Output {
    pub fn read(&self) -> Result<types::Encoder4_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98952)
    }
}
pub struct Encoder4_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Encoder4_TimerConfig {
    pub fn read(&self) -> Result<types::Encoder4_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98956)
    }
    pub fn write(&self, value: &types::Encoder4_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98956, value)
    }
}
pub struct Encoder4_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Encoder4_TimerOutput {
    pub fn read(&self) -> Result<types::Encoder4_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98960)
    }
}
pub struct Encoder5_Config {
    _marker: PhantomData<*const ()>,
}
impl Encoder5_Config {
    pub fn read(&self) -> Result<types::Encoder5_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98964)
    }
    pub fn write(&self, value: &types::Encoder5_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98964, value)
    }
}
pub struct Encoder5_Reset {
    _marker: PhantomData<*const ()>,
}
impl Encoder5_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98970)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98970, value)
    }
}
pub struct Encoder5_Output {
    _marker: PhantomData<*const ()>,
}
impl Encoder5_Output {
    pub fn read(&self) -> Result<types::Encoder5_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98972)
    }
}
pub struct Encoder5_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Encoder5_TimerConfig {
    pub fn read(&self) -> Result<types::Encoder5_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98976)
    }
    pub fn write(&self, value: &types::Encoder5_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98976, value)
    }
}
pub struct Encoder5_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Encoder5_TimerOutput {
    pub fn read(&self) -> Result<types::Encoder5_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98980)
    }
}
pub struct Encoder6_Config {
    _marker: PhantomData<*const ()>,
}
impl Encoder6_Config {
    pub fn read(&self) -> Result<types::Encoder6_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98984)
    }
    pub fn write(&self, value: &types::Encoder6_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98984, value)
    }
}
pub struct Encoder6_Reset {
    _marker: PhantomData<*const ()>,
}
impl Encoder6_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98990)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98990, value)
    }
}
pub struct Encoder6_Output {
    _marker: PhantomData<*const ()>,
}
impl Encoder6_Output {
    pub fn read(&self) -> Result<types::Encoder6_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98992)
    }
}
pub struct Encoder6_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Encoder6_TimerConfig {
    pub fn read(&self) -> Result<types::Encoder6_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(98996)
    }
    pub fn write(&self, value: &types::Encoder6_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(98996, value)
    }
}
pub struct Encoder6_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Encoder6_TimerOutput {
    pub fn read(&self) -> Result<types::Encoder6_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99000)
    }
}
pub struct Encoder7_Config {
    _marker: PhantomData<*const ()>,
}
impl Encoder7_Config {
    pub fn read(&self) -> Result<types::Encoder7_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99004)
    }
    pub fn write(&self, value: &types::Encoder7_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99004, value)
    }
}
pub struct Encoder7_Reset {
    _marker: PhantomData<*const ()>,
}
impl Encoder7_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99010)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99010, value)
    }
}
pub struct Encoder7_Output {
    _marker: PhantomData<*const ()>,
}
impl Encoder7_Output {
    pub fn read(&self) -> Result<types::Encoder7_Output, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99012)
    }
}
pub struct Encoder7_TimerConfig {
    _marker: PhantomData<*const ()>,
}
impl Encoder7_TimerConfig {
    pub fn read(&self) -> Result<types::Encoder7_TimerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99016)
    }
    pub fn write(&self, value: &types::Encoder7_TimerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99016, value)
    }
}
pub struct Encoder7_TimerOutput {
    _marker: PhantomData<*const ()>,
}
impl Encoder7_TimerOutput {
    pub fn read(&self) -> Result<types::Encoder7_TimerOutput, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99020)
    }
}
pub struct Interrupt0_Config {
    _marker: PhantomData<*const ()>,
}
impl Interrupt0_Config {
    pub fn read(&self) -> Result<types::Interrupt0_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99026)
    }
    pub fn write(&self, value: &types::Interrupt0_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99026, value)
    }
}
pub struct Interrupt0_RisingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt0_RisingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99028)
    }
}
pub struct Interrupt0_FallingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt0_FallingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99032)
    }
}
pub struct Interrupt1_Config {
    _marker: PhantomData<*const ()>,
}
impl Interrupt1_Config {
    pub fn read(&self) -> Result<types::Interrupt1_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99038)
    }
    pub fn write(&self, value: &types::Interrupt1_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99038, value)
    }
}
pub struct Interrupt1_RisingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt1_RisingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99040)
    }
}
pub struct Interrupt1_FallingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt1_FallingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99044)
    }
}
pub struct Interrupt2_Config {
    _marker: PhantomData<*const ()>,
}
impl Interrupt2_Config {
    pub fn read(&self) -> Result<types::Interrupt2_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99050)
    }
    pub fn write(&self, value: &types::Interrupt2_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99050, value)
    }
}
pub struct Interrupt2_RisingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt2_RisingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99052)
    }
}
pub struct Interrupt2_FallingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt2_FallingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99056)
    }
}
pub struct Interrupt3_Config {
    _marker: PhantomData<*const ()>,
}
impl Interrupt3_Config {
    pub fn read(&self) -> Result<types::Interrupt3_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99062)
    }
    pub fn write(&self, value: &types::Interrupt3_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99062, value)
    }
}
pub struct Interrupt3_RisingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt3_RisingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99064)
    }
}
pub struct Interrupt3_FallingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt3_FallingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99068)
    }
}
pub struct Interrupt4_Config {
    _marker: PhantomData<*const ()>,
}
impl Interrupt4_Config {
    pub fn read(&self) -> Result<types::Interrupt4_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99074)
    }
    pub fn write(&self, value: &types::Interrupt4_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99074, value)
    }
}
pub struct Interrupt4_RisingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt4_RisingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99076)
    }
}
pub struct Interrupt4_FallingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt4_FallingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99080)
    }
}
pub struct Interrupt5_Config {
    _marker: PhantomData<*const ()>,
}
impl Interrupt5_Config {
    pub fn read(&self) -> Result<types::Interrupt5_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99086)
    }
    pub fn write(&self, value: &types::Interrupt5_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99086, value)
    }
}
pub struct Interrupt5_RisingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt5_RisingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99088)
    }
}
pub struct Interrupt5_FallingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt5_FallingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99092)
    }
}
pub struct Interrupt6_Config {
    _marker: PhantomData<*const ()>,
}
impl Interrupt6_Config {
    pub fn read(&self) -> Result<types::Interrupt6_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99098)
    }
    pub fn write(&self, value: &types::Interrupt6_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99098, value)
    }
}
pub struct Interrupt6_RisingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt6_RisingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99100)
    }
}
pub struct Interrupt6_FallingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt6_FallingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99104)
    }
}
pub struct Interrupt7_Config {
    _marker: PhantomData<*const ()>,
}
impl Interrupt7_Config {
    pub fn read(&self) -> Result<types::Interrupt7_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99110)
    }
    pub fn write(&self, value: &types::Interrupt7_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99110, value)
    }
}
pub struct Interrupt7_RisingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt7_RisingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99112)
    }
}
pub struct Interrupt7_FallingTimeStamp {
    _marker: PhantomData<*const ()>,
}
impl Interrupt7_FallingTimeStamp {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99116)
    }
}
pub struct DMA_Rate {
    _marker: PhantomData<*const ()>,
}
impl DMA_Rate {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99120)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99120, value)
    }
}
pub struct DMA_Config {
    _marker: PhantomData<*const ()>,
}
impl DMA_Config {
    pub fn read(&self) -> Result<types::DMA_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99124)
    }
    pub fn write(&self, value: &types::DMA_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99124, value)
    }
}
pub struct DMA_ExternalTriggers0 {
    _marker: PhantomData<*const ()>,
}
impl DMA_ExternalTriggers0 {
    pub fn read(&self) -> Result<[types::Trigger; 4], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99128)
    }
    pub fn write(&self, value: &[types::Trigger; 4]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99128, value)
    }
}
pub struct DMA_ExternalTriggers1 {
    _marker: PhantomData<*const ()>,
}
impl DMA_ExternalTriggers1 {
    pub fn read(&self) -> Result<[types::Trigger; 4], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99132)
    }
    pub fn write(&self, value: &[types::Trigger; 4]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99132, value)
    }
}
pub struct Alarm_TriggerTime {
    _marker: PhantomData<*const ()>,
}
impl Alarm_TriggerTime {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99136)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99136, value)
    }
}
pub struct Alarm_Enable {
    _marker: PhantomData<*const ()>,
}
impl Alarm_Enable {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99142)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99142, value)
    }
}
pub struct Relay_Value {
    _marker: PhantomData<*const ()>,
}
impl Relay_Value {
    pub fn read(&self) -> Result<types::Relay_Value, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99146)
    }
    pub fn write(&self, value: &types::Relay_Value) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99146, value)
    }
}
pub struct Power_Status {
    _marker: PhantomData<*const ()>,
}
impl Power_Status {
    pub fn read(&self) -> Result<types::Power_Status, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99148)
    }
}
pub struct Power_Disable {
    _marker: PhantomData<*const ()>,
}
impl Power_Disable {
    pub fn read(&self) -> Result<types::Power_Disable, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99154)
    }
    pub fn write(&self, value: &types::Power_Disable) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99154, value)
    }
}
pub struct Power_UserVoltage6V {
    _marker: PhantomData<*const ()>,
}
impl Power_UserVoltage6V {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99158)
    }
}
pub struct Power_UserCurrent6V {
    _marker: PhantomData<*const ()>,
}
impl Power_UserCurrent6V {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99162)
    }
}
pub struct Power_UserVoltage5V {
    _marker: PhantomData<*const ()>,
}
impl Power_UserVoltage5V {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99166)
    }
}
pub struct Power_UserCurrent5V {
    _marker: PhantomData<*const ()>,
}
impl Power_UserCurrent5V {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99170)
    }
}
pub struct Power_UserVoltage3V3 {
    _marker: PhantomData<*const ()>,
}
impl Power_UserVoltage3V3 {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99174)
    }
}
pub struct Power_UserCurrent3V3 {
    _marker: PhantomData<*const ()>,
}
impl Power_UserCurrent3V3 {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99178)
    }
}
pub struct Power_VinVoltage {
    _marker: PhantomData<*const ()>,
}
impl Power_VinVoltage {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99182)
    }
}
pub struct Power_VinCurrent {
    _marker: PhantomData<*const ()>,
}
impl Power_VinCurrent {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99186)
    }
}
pub struct Power_OnChipTemperature {
    _marker: PhantomData<*const ()>,
}
impl Power_OnChipTemperature {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99190)
    }
}
pub struct Power_MXP_DIOVoltage {
    _marker: PhantomData<*const ()>,
}
impl Power_MXP_DIOVoltage {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99194)
    }
}
pub struct Power_IntegratedIO {
    _marker: PhantomData<*const ()>,
}
impl Power_IntegratedIO {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99198)
    }
}
pub struct Power_AOVoltage {
    _marker: PhantomData<*const ()>,
}
impl Power_AOVoltage {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99202)
    }
}
pub struct Power_FaultCounts {
    _marker: PhantomData<*const ()>,
}
impl Power_FaultCounts {
    pub fn read(&self) -> Result<types::Power_FaultCounts, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99204)
    }
}
pub struct Power_ResetFaultCounts {
    _marker: PhantomData<*const ()>,
}
impl Power_ResetFaultCounts {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99210)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99210, value)
    }
}
pub struct BIST_Enable {
    _marker: PhantomData<*const ()>,
}
impl BIST_Enable {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99214)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99214, value)
    }
}
pub struct BIST_DO0SquareEnable {
    _marker: PhantomData<*const ()>,
}
impl BIST_DO0SquareEnable {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99218)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99218, value)
    }
}
pub struct BIST_DO0SquareTicks {
    _marker: PhantomData<*const ()>,
}
impl BIST_DO0SquareTicks {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99220)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99220, value)
    }
}
pub struct BIST_DO0 {
    _marker: PhantomData<*const ()>,
}
impl BIST_DO0 {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99226)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99226, value)
    }
}
pub struct BIST_DO1SquareEnable {
    _marker: PhantomData<*const ()>,
}
impl BIST_DO1SquareEnable {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99230)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99230, value)
    }
}
pub struct BIST_DO1SquareTicks {
    _marker: PhantomData<*const ()>,
}
impl BIST_DO1SquareTicks {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99232)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99232, value)
    }
}
pub struct BIST_DO1 {
    _marker: PhantomData<*const ()>,
}
impl BIST_DO1 {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99238)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99238, value)
    }
}
pub struct AO_MXP0 {
    _marker: PhantomData<*const ()>,
}
impl AO_MXP0 {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99242)
    }
    pub fn write(&self, value: &u16) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99242, value)
    }
}
pub struct AO_MXP1 {
    _marker: PhantomData<*const ()>,
}
impl AO_MXP1 {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99246)
    }
    pub fn write(&self, value: &u16) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99246, value)
    }
}
pub struct SPI_ChipSelectActiveHigh {
    _marker: PhantomData<*const ()>,
}
impl SPI_ChipSelectActiveHigh {
    pub fn read(&self) -> Result<types::SPI_ChipSelectActiveHigh, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99250)
    }
    pub fn write(&self, value: &types::SPI_ChipSelectActiveHigh) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99250, value)
    }
}
pub struct SPI_EnableDIO {
    _marker: PhantomData<*const ()>,
}
impl SPI_EnableDIO {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<5, 5, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99254)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<5, 5, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99254, value)
    }
}
pub struct SPI_AutoSPI1Select {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoSPI1Select {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99258)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99258, value)
    }
}
pub struct SPI_AutoByteCount {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoByteCount {
    pub fn read(&self) -> Result<types::SPI_AutoByteCount, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99262)
    }
    pub fn write(&self, value: &types::SPI_AutoByteCount) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99262, value)
    }
}
pub struct SPI_AutoForceOne {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoForceOne {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99266)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99266, value)
    }
}
pub struct SPI_AutoRate {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoRate {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99268)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99268, value)
    }
}
pub struct SPI_AutoTriggerConfig {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoTriggerConfig {
    pub fn read(&self) -> Result<types::SPI_AutoTriggerConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99274)
    }
    pub fn write(&self, value: &types::SPI_AutoTriggerConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99274, value)
    }
}
pub struct SPI_AutoChipSelect {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoChipSelect {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99278)
    }
    pub fn write(&self, value: &u8) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99278, value)
    }
}
pub struct SPI_AutoTx0 {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoTx0 {
    pub fn read(&self) -> Result<[u8; 4], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99280)
    }
    pub fn write(&self, value: &[u8; 4]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99280, value)
    }
}
pub struct SPI_AutoTx1 {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoTx1 {
    pub fn read(&self) -> Result<[u8; 4], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99284)
    }
    pub fn write(&self, value: &[u8; 4]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99284, value)
    }
}
pub struct SPI_AutoTx2 {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoTx2 {
    pub fn read(&self) -> Result<[u8; 4], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99288)
    }
    pub fn write(&self, value: &[u8; 4]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99288, value)
    }
}
pub struct SPI_AutoTx3 {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoTx3 {
    pub fn read(&self) -> Result<[u8; 4], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99292)
    }
    pub fn write(&self, value: &[u8; 4]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99292, value)
    }
}
pub struct SPI_AutoTx4 {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoTx4 {
    pub fn read(&self) -> Result<[u8; 4], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99296)
    }
    pub fn write(&self, value: &[u8; 4]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99296, value)
    }
}
pub struct SPI_AutoTx5 {
    _marker: PhantomData<*const ()>,
}
impl SPI_AutoTx5 {
    pub fn read(&self) -> Result<[u8; 4], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99300)
    }
    pub fn write(&self, value: &[u8; 4]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99300, value)
    }
}
pub struct SPI_TransferSkippedFullCount {
    _marker: PhantomData<*const ()>,
}
impl SPI_TransferSkippedFullCount {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99304)
    }
}
pub struct SPI_StallConfig {
    _marker: PhantomData<*const ()>,
}
impl SPI_StallConfig {
    pub fn read(&self) -> Result<types::SPI_StallConfig, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99308)
    }
    pub fn write(&self, value: &types::SPI_StallConfig) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99308, value)
    }
}
pub struct SPI_DebugState {
    _marker: PhantomData<*const ()>,
}
impl SPI_DebugState {
    pub fn read(&self) -> Result<types::SPI_DebugState, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99314)
    }
}
pub struct SPI_DebugSubstate {
    _marker: PhantomData<*const ()>,
}
impl SPI_DebugSubstate {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99318)
    }
}
pub struct SPI_DebugRevision {
    _marker: PhantomData<*const ()>,
}
impl SPI_DebugRevision {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99320)
    }
}
pub struct SPI_DebugEnabled {
    _marker: PhantomData<*const ()>,
}
impl SPI_DebugEnabled {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99324)
    }
}
pub struct SPI_DebugIntStat {
    _marker: PhantomData<*const ()>,
}
impl SPI_DebugIntStat {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99328)
    }
}
pub struct SPI_DebugIntStatReadCount {
    _marker: PhantomData<*const ()>,
}
impl SPI_DebugIntStatReadCount {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99332)
    }
}
pub struct Accel_ADDR {
    _marker: PhantomData<*const ()>,
}
impl Accel_ADDR {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99338)
    }
    pub fn write(&self, value: &u8) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99338, value)
    }
}
pub struct Accel_CNTR {
    _marker: PhantomData<*const ()>,
}
impl Accel_CNTR {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99342)
    }
    pub fn write(&self, value: &u8) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99342, value)
    }
}
pub struct Accel_DATO {
    _marker: PhantomData<*const ()>,
}
impl Accel_DATO {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99346)
    }
    pub fn write(&self, value: &u8) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99346, value)
    }
}
pub struct Accel_DATI {
    _marker: PhantomData<*const ()>,
}
impl Accel_DATI {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99350)
    }
}
pub struct Accel_CNTL {
    _marker: PhantomData<*const ()>,
}
impl Accel_CNTL {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99354)
    }
    pub fn write(&self, value: &u8) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99354, value)
    }
}
pub struct Accel_STAT {
    _marker: PhantomData<*const ()>,
}
impl Accel_STAT {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99358)
    }
}
pub struct Accel_CNFG {
    _marker: PhantomData<*const ()>,
}
impl Accel_CNFG {
    pub fn read(&self) -> Result<u8, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99362)
    }
    pub fn write(&self, value: &u8) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99362, value)
    }
}
pub struct Accel_GO {
    _marker: PhantomData<*const ()>,
}
impl Accel_GO {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99366)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99366, value)
    }
}
pub struct HMB_Config {
    _marker: PhantomData<*const ()>,
}
impl HMB_Config {
    pub fn read(&self) -> Result<types::HMB_Config, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99368)
    }
    pub fn write(&self, value: &types::HMB_Config) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99368, value)
    }
}
pub struct HMB_ForceOnce {
    _marker: PhantomData<*const ()>,
}
impl HMB_ForceOnce {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99374)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99374, value)
    }
}
pub struct DutyCycle0_Source {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle0_Source {
    pub fn read(&self) -> Result<types::DutyCycle0_Source, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99378)
    }
    pub fn write(&self, value: &types::DutyCycle0_Source) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99378, value)
    }
}
pub struct DutyCycle0_Frequency {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle0_Frequency {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<11, 11, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99382)
    }
}
pub struct DutyCycle0_Output {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle0_Output {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<31, 31, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99384)
    }
}
pub struct DutyCycle1_Source {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle1_Source {
    pub fn read(&self) -> Result<types::DutyCycle1_Source, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99390)
    }
    pub fn write(&self, value: &types::DutyCycle1_Source) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99390, value)
    }
}
pub struct DutyCycle1_Frequency {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle1_Frequency {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<11, 11, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99394)
    }
}
pub struct DutyCycle1_Output {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle1_Output {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<31, 31, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99396)
    }
}
pub struct DutyCycle2_Source {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle2_Source {
    pub fn read(&self) -> Result<types::DutyCycle2_Source, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99402)
    }
    pub fn write(&self, value: &types::DutyCycle2_Source) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99402, value)
    }
}
pub struct DutyCycle2_Frequency {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle2_Frequency {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<11, 11, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99406)
    }
}
pub struct DutyCycle2_Output {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle2_Output {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<31, 31, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99408)
    }
}
pub struct DutyCycle3_Source {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle3_Source {
    pub fn read(&self) -> Result<types::DutyCycle3_Source, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99414)
    }
    pub fn write(&self, value: &types::DutyCycle3_Source) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99414, value)
    }
}
pub struct DutyCycle3_Frequency {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle3_Frequency {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<11, 11, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99418)
    }
}
pub struct DutyCycle3_Output {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle3_Output {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<31, 31, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99420)
    }
}
pub struct DutyCycle4_Source {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle4_Source {
    pub fn read(&self) -> Result<types::DutyCycle4_Source, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99426)
    }
    pub fn write(&self, value: &types::DutyCycle4_Source) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99426, value)
    }
}
pub struct DutyCycle4_Frequency {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle4_Frequency {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<11, 11, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99430)
    }
}
pub struct DutyCycle4_Output {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle4_Output {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<31, 31, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99432)
    }
}
pub struct DutyCycle5_Source {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle5_Source {
    pub fn read(&self) -> Result<types::DutyCycle5_Source, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99438)
    }
    pub fn write(&self, value: &types::DutyCycle5_Source) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99438, value)
    }
}
pub struct DutyCycle5_Frequency {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle5_Frequency {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<11, 11, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99442)
    }
}
pub struct DutyCycle5_Output {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle5_Output {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<31, 31, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99444)
    }
}
pub struct DutyCycle6_Source {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle6_Source {
    pub fn read(&self) -> Result<types::DutyCycle6_Source, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99450)
    }
    pub fn write(&self, value: &types::DutyCycle6_Source) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99450, value)
    }
}
pub struct DutyCycle6_Frequency {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle6_Frequency {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<11, 11, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99454)
    }
}
pub struct DutyCycle6_Output {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle6_Output {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<31, 31, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99456)
    }
}
pub struct DutyCycle7_Source {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle7_Source {
    pub fn read(&self) -> Result<types::DutyCycle7_Source, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99462)
    }
    pub fn write(&self, value: &types::DutyCycle7_Source) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99462, value)
    }
}
pub struct DutyCycle7_Frequency {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle7_Frequency {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<11, 11, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99466)
    }
}
pub struct DutyCycle7_Output {
    _marker: PhantomData<*const ()>,
}
impl DutyCycle7_Output {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<31, 31, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99468)
    }
}
pub struct LED_OutputSelect {
    _marker: PhantomData<*const ()>,
}
impl LED_OutputSelect {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<4, 4, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99474)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<4, 4, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99474, value)
    }
}
pub struct LED_StringLength {
    _marker: PhantomData<*const ()>,
}
impl LED_StringLength {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99478)
    }
    pub fn write(&self, value: &u16) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99478, value)
    }
}
pub struct LED_Load {
    _marker: PhantomData<*const ()>,
}
impl LED_Load {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99482)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99482, value)
    }
}
pub struct LED_Reset {
    _marker: PhantomData<*const ()>,
}
impl LED_Reset {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99486)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99486, value)
    }
}
pub struct LED_Start {
    _marker: PhantomData<*const ()>,
}
impl LED_Start {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99490)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99490, value)
    }
}
pub struct LED_Abort {
    _marker: PhantomData<*const ()>,
}
impl LED_Abort {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99494)
    }
    pub fn write(&self, value: &bool) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99494, value)
    }
}
pub struct LED_SyncTiming {
    _marker: PhantomData<*const ()>,
}
impl LED_SyncTiming {
    pub fn read(&self) -> Result<ni_fpga::fxp::FXP<16, 16, false>, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99498)
    }
    pub fn write(&self, value: &ni_fpga::fxp::FXP<16, 16, false>) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99498, value)
    }
}
pub struct LED_HighBitTickTiming {
    _marker: PhantomData<*const ()>,
}
impl LED_HighBitTickTiming {
    pub fn read(&self) -> Result<[u8; 2], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99502)
    }
    pub fn write(&self, value: &[u8; 2]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99502, value)
    }
}
pub struct LED_LowBitTickTiming {
    _marker: PhantomData<*const ()>,
}
impl LED_LowBitTickTiming {
    pub fn read(&self) -> Result<[u8; 2], ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99506)
    }
    pub fn write(&self, value: &[u8; 2]) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(99506, value)
    }
}
pub struct LED_Active {
    _marker: PhantomData<*const ()>,
}
impl LED_Active {
    pub fn read(&self) -> Result<bool, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99510)
    }
}
pub struct LED_PixelWriteIndex {
    _marker: PhantomData<*const ()>,
}
impl LED_PixelWriteIndex {
    pub fn read(&self) -> Result<i16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99514)
    }
}
pub struct LED_PixelOutputIndex {
    _marker: PhantomData<*const ()>,
}
impl LED_PixelOutputIndex {
    pub fn read(&self) -> Result<u16, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(99518)
    }
}
pub struct ViControl {
    _marker: PhantomData<*const ()>,
}
impl ViControl {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(94208)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(94208, value)
    }
}
pub struct DiagramReset {
    _marker: PhantomData<*const ()>,
}
impl DiagramReset {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(94216)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(94216, value)
    }
}
pub struct InterruptEnable {
    _marker: PhantomData<*const ()>,
}
impl InterruptEnable {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(90112)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(90112, value)
    }
}
pub struct InterruptMask {
    _marker: PhantomData<*const ()>,
}
impl InterruptMask {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(90120)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(90120, value)
    }
}
pub struct InterruptStatus {
    _marker: PhantomData<*const ()>,
}
impl InterruptStatus {
    pub fn read(&self) -> Result<u32, ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().read(90124)
    }
    pub fn write(&self, value: &u32) -> Result<(), ni_fpga::Error> {
        unsafe { SESSION.as_ref() }.unwrap().write(90124, value)
    }
}
pub struct Registers {
    _marker: PhantomData<*const ()>,
    pub LocalTime: LocalTime,
    pub Revision: Revision,
    pub Version: Version,
    pub LocalTimeUpper: LocalTimeUpper,
    pub LEDs: LEDs,
    pub UserButton: UserButton,
    pub InterruptForceOnce: InterruptForceOnce,
    pub InterruptForceNumber: InterruptForceNumber,
    pub SysWatchdog_Status: SysWatchdog_Status,
    pub SysWatchdog_Command: SysWatchdog_Command,
    pub SysWatchdog_Challenge: SysWatchdog_Challenge,
    pub SysWatchdog_Timer: SysWatchdog_Timer,
    pub SysWatchdog_Active: SysWatchdog_Active,
    pub SysWatchdog_ForcedKills: SysWatchdog_ForcedKills,
    pub AI_ReadSelect: AI_ReadSelect,
    pub AI_LatchOutput: AI_LatchOutput,
    pub AI_Output: AI_Output,
    pub AI_Config: AI_Config,
    pub AI_ScanList: AI_ScanList,
    pub AI_OversampleBits: AI_OversampleBits,
    pub AI_AverageBits: AI_AverageBits,
    pub AI_LoopTiming: AI_LoopTiming,
    pub Accumulator0_Center: Accumulator0_Center,
    pub Accumulator0_Reset: Accumulator0_Reset,
    pub Accumulator0_Output: Accumulator0_Output,
    pub Accumulator0_Deadband: Accumulator0_Deadband,
    pub Accumulator1_Center: Accumulator1_Center,
    pub Accumulator1_Reset: Accumulator1_Reset,
    pub Accumulator1_Output: Accumulator1_Output,
    pub Accumulator1_Deadband: Accumulator1_Deadband,
    pub AnalogTrigger_Output: AnalogTrigger_Output,
    pub AnalogTrigger0_SourceSelect: AnalogTrigger0_SourceSelect,
    pub AnalogTrigger0_UpperLimit: AnalogTrigger0_UpperLimit,
    pub AnalogTrigger0_LowerLimit: AnalogTrigger0_LowerLimit,
    pub AnalogTrigger1_SourceSelect: AnalogTrigger1_SourceSelect,
    pub AnalogTrigger1_UpperLimit: AnalogTrigger1_UpperLimit,
    pub AnalogTrigger1_LowerLimit: AnalogTrigger1_LowerLimit,
    pub AnalogTrigger2_SourceSelect: AnalogTrigger2_SourceSelect,
    pub AnalogTrigger2_UpperLimit: AnalogTrigger2_UpperLimit,
    pub AnalogTrigger2_LowerLimit: AnalogTrigger2_LowerLimit,
    pub AnalogTrigger3_SourceSelect: AnalogTrigger3_SourceSelect,
    pub AnalogTrigger3_UpperLimit: AnalogTrigger3_UpperLimit,
    pub AnalogTrigger3_LowerLimit: AnalogTrigger3_LowerLimit,
    pub AnalogTrigger4_SourceSelect: AnalogTrigger4_SourceSelect,
    pub AnalogTrigger4_UpperLimit: AnalogTrigger4_UpperLimit,
    pub AnalogTrigger4_LowerLimit: AnalogTrigger4_LowerLimit,
    pub AnalogTrigger5_SourceSelect: AnalogTrigger5_SourceSelect,
    pub AnalogTrigger6_UpperLimit: AnalogTrigger6_UpperLimit,
    pub AnalogTrigger6_LowerLimit: AnalogTrigger6_LowerLimit,
    pub AnalogTrigger6_SourceSelect: AnalogTrigger6_SourceSelect,
    pub AnalogTrigger5_UpperLimit: AnalogTrigger5_UpperLimit,
    pub AnalogTrigger5_LowerLimit: AnalogTrigger5_LowerLimit,
    pub AnalogTrigger7_SourceSelect: AnalogTrigger7_SourceSelect,
    pub AnalogTrigger7_UpperLimit: AnalogTrigger7_UpperLimit,
    pub AnalogTrigger7_LowerLimit: AnalogTrigger7_LowerLimit,
    pub PWM_LoopTiming: PWM_LoopTiming,
    pub PWM_CycleStartTimeUpper: PWM_CycleStartTimeUpper,
    pub PWM_CycleStartTime: PWM_CycleStartTime,
    pub PWM_Config: PWM_Config,
    pub PWM_PeriodScaleHdr: PWM_PeriodScaleHdr,
    pub PWM_PeriodScaleMXP: PWM_PeriodScaleMXP,
    pub PWM_ZeroLatch: PWM_ZeroLatch,
    pub PWM_Hdr0: PWM_Hdr0,
    pub PWM_Hdr1: PWM_Hdr1,
    pub PWM_Hdr2: PWM_Hdr2,
    pub PWM_Hdr3: PWM_Hdr3,
    pub PWM_Hdr4: PWM_Hdr4,
    pub PWM_Hdr5: PWM_Hdr5,
    pub PWM_Hdr6: PWM_Hdr6,
    pub PWM_Hdr7: PWM_Hdr7,
    pub PWM_Hdr8: PWM_Hdr8,
    pub PWM_Hdr9: PWM_Hdr9,
    pub PWM_MXP0: PWM_MXP0,
    pub PWM_MXP1: PWM_MXP1,
    pub PWM_MXP2: PWM_MXP2,
    pub PWM_MXP3: PWM_MXP3,
    pub PWM_MXP4: PWM_MXP4,
    pub PWM_MXP5: PWM_MXP5,
    pub PWM_MXP6: PWM_MXP6,
    pub PWM_MXP7: PWM_MXP7,
    pub PWM_MXP8: PWM_MXP8,
    pub PWM_MXP9: PWM_MXP9,
    pub DIO_OutputEnable: DIO_OutputEnable,
    pub DIO_DO: DIO_DO,
    pub DIO_DI: DIO_DI,
    pub DIO_FilterSelectHdr: DIO_FilterSelectHdr,
    pub DIO_FilterPeriodHdr0: DIO_FilterPeriodHdr0,
    pub DIO_FilterPeriodHdr1: DIO_FilterPeriodHdr1,
    pub DIO_FilterPeriodHdr2: DIO_FilterPeriodHdr2,
    pub DIO_FilterSelectMXP: DIO_FilterSelectMXP,
    pub DIO_FilterPeriodMXP0: DIO_FilterPeriodMXP0,
    pub DIO_FilterPeriodMXP1: DIO_FilterPeriodMXP1,
    pub DIO_FilterPeriodMXP2: DIO_FilterPeriodMXP2,
    pub DIO_EnableMXPSpecialFunction: DIO_EnableMXPSpecialFunction,
    pub DIO_PulseLength: DIO_PulseLength,
    pub DIO_Pulse: DIO_Pulse,
    pub DIO_PWMDutyCycleA: DIO_PWMDutyCycleA,
    pub DIO_PWMDutyCycleB: DIO_PWMDutyCycleB,
    pub DIO_PWMOutputSelect: DIO_PWMOutputSelect,
    pub DIO_PWMPeriodPower: DIO_PWMPeriodPower,
    pub Counter0_Config: Counter0_Config,
    pub Counter0_Reset: Counter0_Reset,
    pub Counter0_Output: Counter0_Output,
    pub Counter0_TimerConfig: Counter0_TimerConfig,
    pub Counter0_TimerOutput: Counter0_TimerOutput,
    pub Counter1_Config: Counter1_Config,
    pub Counter1_Reset: Counter1_Reset,
    pub Counter1_Output: Counter1_Output,
    pub Counter1_TimerConfig: Counter1_TimerConfig,
    pub Counter1_TimerOutput: Counter1_TimerOutput,
    pub Counter2_Config: Counter2_Config,
    pub Counter2_Reset: Counter2_Reset,
    pub Counter2_Output: Counter2_Output,
    pub Counter2_TimerConfig: Counter2_TimerConfig,
    pub Counter2_TimerOutput: Counter2_TimerOutput,
    pub Counter3_Config: Counter3_Config,
    pub Counter3_Reset: Counter3_Reset,
    pub Counter3_Output: Counter3_Output,
    pub Counter3_TimerConfig: Counter3_TimerConfig,
    pub Counter3_TimerOutput: Counter3_TimerOutput,
    pub Counter4_Config: Counter4_Config,
    pub Counter4_Reset: Counter4_Reset,
    pub Counter4_Output: Counter4_Output,
    pub Counter4_TimerConfig: Counter4_TimerConfig,
    pub Counter4_TimerOutput: Counter4_TimerOutput,
    pub Counter5_Config: Counter5_Config,
    pub Counter5_Reset: Counter5_Reset,
    pub Counter5_Output: Counter5_Output,
    pub Counter5_TimerConfig: Counter5_TimerConfig,
    pub Counter5_TimerOutput: Counter5_TimerOutput,
    pub Counter6_Config: Counter6_Config,
    pub Counter6_Reset: Counter6_Reset,
    pub Counter6_Output: Counter6_Output,
    pub Counter6_TimerConfig: Counter6_TimerConfig,
    pub Counter6_TimerOutput: Counter6_TimerOutput,
    pub Counter7_Config: Counter7_Config,
    pub Counter7_Reset: Counter7_Reset,
    pub Counter7_Output: Counter7_Output,
    pub Counter7_TimerConfig: Counter7_TimerConfig,
    pub Counter7_TimerOutput: Counter7_TimerOutput,
    pub Encoder0_Config: Encoder0_Config,
    pub Encoder0_Reset: Encoder0_Reset,
    pub Encoder0_Output: Encoder0_Output,
    pub Encoder0_TimerConfig: Encoder0_TimerConfig,
    pub Encoder0_TimerOutput: Encoder0_TimerOutput,
    pub Encoder1_Config: Encoder1_Config,
    pub Encoder1_Reset: Encoder1_Reset,
    pub Encoder1_Output: Encoder1_Output,
    pub Encoder1_TimerConfig: Encoder1_TimerConfig,
    pub Encoder1_TimerOutput: Encoder1_TimerOutput,
    pub Encoder2_Config: Encoder2_Config,
    pub Encoder2_Reset: Encoder2_Reset,
    pub Encoder2_Output: Encoder2_Output,
    pub Encoder2_TimerConfig: Encoder2_TimerConfig,
    pub Encoder2_TimerOutput: Encoder2_TimerOutput,
    pub Encoder3_Config: Encoder3_Config,
    pub Encoder3_Reset: Encoder3_Reset,
    pub Encoder3_Output: Encoder3_Output,
    pub Encoder3_TimerConfig: Encoder3_TimerConfig,
    pub Encoder3_TimerOutput: Encoder3_TimerOutput,
    pub Encoder4_Config: Encoder4_Config,
    pub Encoder4_Reset: Encoder4_Reset,
    pub Encoder4_Output: Encoder4_Output,
    pub Encoder4_TimerConfig: Encoder4_TimerConfig,
    pub Encoder4_TimerOutput: Encoder4_TimerOutput,
    pub Encoder5_Config: Encoder5_Config,
    pub Encoder5_Reset: Encoder5_Reset,
    pub Encoder5_Output: Encoder5_Output,
    pub Encoder5_TimerConfig: Encoder5_TimerConfig,
    pub Encoder5_TimerOutput: Encoder5_TimerOutput,
    pub Encoder6_Config: Encoder6_Config,
    pub Encoder6_Reset: Encoder6_Reset,
    pub Encoder6_Output: Encoder6_Output,
    pub Encoder6_TimerConfig: Encoder6_TimerConfig,
    pub Encoder6_TimerOutput: Encoder6_TimerOutput,
    pub Encoder7_Config: Encoder7_Config,
    pub Encoder7_Reset: Encoder7_Reset,
    pub Encoder7_Output: Encoder7_Output,
    pub Encoder7_TimerConfig: Encoder7_TimerConfig,
    pub Encoder7_TimerOutput: Encoder7_TimerOutput,
    pub Interrupt0_Config: Interrupt0_Config,
    pub Interrupt0_RisingTimeStamp: Interrupt0_RisingTimeStamp,
    pub Interrupt0_FallingTimeStamp: Interrupt0_FallingTimeStamp,
    pub Interrupt1_Config: Interrupt1_Config,
    pub Interrupt1_RisingTimeStamp: Interrupt1_RisingTimeStamp,
    pub Interrupt1_FallingTimeStamp: Interrupt1_FallingTimeStamp,
    pub Interrupt2_Config: Interrupt2_Config,
    pub Interrupt2_RisingTimeStamp: Interrupt2_RisingTimeStamp,
    pub Interrupt2_FallingTimeStamp: Interrupt2_FallingTimeStamp,
    pub Interrupt3_Config: Interrupt3_Config,
    pub Interrupt3_RisingTimeStamp: Interrupt3_RisingTimeStamp,
    pub Interrupt3_FallingTimeStamp: Interrupt3_FallingTimeStamp,
    pub Interrupt4_Config: Interrupt4_Config,
    pub Interrupt4_RisingTimeStamp: Interrupt4_RisingTimeStamp,
    pub Interrupt4_FallingTimeStamp: Interrupt4_FallingTimeStamp,
    pub Interrupt5_Config: Interrupt5_Config,
    pub Interrupt5_RisingTimeStamp: Interrupt5_RisingTimeStamp,
    pub Interrupt5_FallingTimeStamp: Interrupt5_FallingTimeStamp,
    pub Interrupt6_Config: Interrupt6_Config,
    pub Interrupt6_RisingTimeStamp: Interrupt6_RisingTimeStamp,
    pub Interrupt6_FallingTimeStamp: Interrupt6_FallingTimeStamp,
    pub Interrupt7_Config: Interrupt7_Config,
    pub Interrupt7_RisingTimeStamp: Interrupt7_RisingTimeStamp,
    pub Interrupt7_FallingTimeStamp: Interrupt7_FallingTimeStamp,
    pub DMA_Rate: DMA_Rate,
    pub DMA_Config: DMA_Config,
    pub DMA_ExternalTriggers0: DMA_ExternalTriggers0,
    pub DMA_ExternalTriggers1: DMA_ExternalTriggers1,
    pub Alarm_TriggerTime: Alarm_TriggerTime,
    pub Alarm_Enable: Alarm_Enable,
    pub Relay_Value: Relay_Value,
    pub Power_Status: Power_Status,
    pub Power_Disable: Power_Disable,
    pub Power_UserVoltage6V: Power_UserVoltage6V,
    pub Power_UserCurrent6V: Power_UserCurrent6V,
    pub Power_UserVoltage5V: Power_UserVoltage5V,
    pub Power_UserCurrent5V: Power_UserCurrent5V,
    pub Power_UserVoltage3V3: Power_UserVoltage3V3,
    pub Power_UserCurrent3V3: Power_UserCurrent3V3,
    pub Power_VinVoltage: Power_VinVoltage,
    pub Power_VinCurrent: Power_VinCurrent,
    pub Power_OnChipTemperature: Power_OnChipTemperature,
    pub Power_MXP_DIOVoltage: Power_MXP_DIOVoltage,
    pub Power_IntegratedIO: Power_IntegratedIO,
    pub Power_AOVoltage: Power_AOVoltage,
    pub Power_FaultCounts: Power_FaultCounts,
    pub Power_ResetFaultCounts: Power_ResetFaultCounts,
    pub BIST_Enable: BIST_Enable,
    pub BIST_DO0SquareEnable: BIST_DO0SquareEnable,
    pub BIST_DO0SquareTicks: BIST_DO0SquareTicks,
    pub BIST_DO0: BIST_DO0,
    pub BIST_DO1SquareEnable: BIST_DO1SquareEnable,
    pub BIST_DO1SquareTicks: BIST_DO1SquareTicks,
    pub BIST_DO1: BIST_DO1,
    pub AO_MXP0: AO_MXP0,
    pub AO_MXP1: AO_MXP1,
    pub SPI_ChipSelectActiveHigh: SPI_ChipSelectActiveHigh,
    pub SPI_EnableDIO: SPI_EnableDIO,
    pub SPI_AutoSPI1Select: SPI_AutoSPI1Select,
    pub SPI_AutoByteCount: SPI_AutoByteCount,
    pub SPI_AutoForceOne: SPI_AutoForceOne,
    pub SPI_AutoRate: SPI_AutoRate,
    pub SPI_AutoTriggerConfig: SPI_AutoTriggerConfig,
    pub SPI_AutoChipSelect: SPI_AutoChipSelect,
    pub SPI_AutoTx0: SPI_AutoTx0,
    pub SPI_AutoTx1: SPI_AutoTx1,
    pub SPI_AutoTx2: SPI_AutoTx2,
    pub SPI_AutoTx3: SPI_AutoTx3,
    pub SPI_AutoTx4: SPI_AutoTx4,
    pub SPI_AutoTx5: SPI_AutoTx5,
    pub SPI_TransferSkippedFullCount: SPI_TransferSkippedFullCount,
    pub SPI_StallConfig: SPI_StallConfig,
    pub SPI_DebugState: SPI_DebugState,
    pub SPI_DebugSubstate: SPI_DebugSubstate,
    pub SPI_DebugRevision: SPI_DebugRevision,
    pub SPI_DebugEnabled: SPI_DebugEnabled,
    pub SPI_DebugIntStat: SPI_DebugIntStat,
    pub SPI_DebugIntStatReadCount: SPI_DebugIntStatReadCount,
    pub Accel_ADDR: Accel_ADDR,
    pub Accel_CNTR: Accel_CNTR,
    pub Accel_DATO: Accel_DATO,
    pub Accel_DATI: Accel_DATI,
    pub Accel_CNTL: Accel_CNTL,
    pub Accel_STAT: Accel_STAT,
    pub Accel_CNFG: Accel_CNFG,
    pub Accel_GO: Accel_GO,
    pub HMB_Config: HMB_Config,
    pub HMB_ForceOnce: HMB_ForceOnce,
    pub DutyCycle0_Source: DutyCycle0_Source,
    pub DutyCycle0_Frequency: DutyCycle0_Frequency,
    pub DutyCycle0_Output: DutyCycle0_Output,
    pub DutyCycle1_Source: DutyCycle1_Source,
    pub DutyCycle1_Frequency: DutyCycle1_Frequency,
    pub DutyCycle1_Output: DutyCycle1_Output,
    pub DutyCycle2_Source: DutyCycle2_Source,
    pub DutyCycle2_Frequency: DutyCycle2_Frequency,
    pub DutyCycle2_Output: DutyCycle2_Output,
    pub DutyCycle3_Source: DutyCycle3_Source,
    pub DutyCycle3_Frequency: DutyCycle3_Frequency,
    pub DutyCycle3_Output: DutyCycle3_Output,
    pub DutyCycle4_Source: DutyCycle4_Source,
    pub DutyCycle4_Frequency: DutyCycle4_Frequency,
    pub DutyCycle4_Output: DutyCycle4_Output,
    pub DutyCycle5_Source: DutyCycle5_Source,
    pub DutyCycle5_Frequency: DutyCycle5_Frequency,
    pub DutyCycle5_Output: DutyCycle5_Output,
    pub DutyCycle6_Source: DutyCycle6_Source,
    pub DutyCycle6_Frequency: DutyCycle6_Frequency,
    pub DutyCycle6_Output: DutyCycle6_Output,
    pub DutyCycle7_Source: DutyCycle7_Source,
    pub DutyCycle7_Frequency: DutyCycle7_Frequency,
    pub DutyCycle7_Output: DutyCycle7_Output,
    pub LED_OutputSelect: LED_OutputSelect,
    pub LED_StringLength: LED_StringLength,
    pub LED_Load: LED_Load,
    pub LED_Reset: LED_Reset,
    pub LED_Start: LED_Start,
    pub LED_Abort: LED_Abort,
    pub LED_SyncTiming: LED_SyncTiming,
    pub LED_HighBitTickTiming: LED_HighBitTickTiming,
    pub LED_LowBitTickTiming: LED_LowBitTickTiming,
    pub LED_Active: LED_Active,
    pub LED_PixelWriteIndex: LED_PixelWriteIndex,
    pub LED_PixelOutputIndex: LED_PixelOutputIndex,
    pub ViControl: ViControl,
    pub DiagramReset: DiagramReset,
    pub InterruptEnable: InterruptEnable,
    pub InterruptMask: InterruptMask,
    pub InterruptStatus: InterruptStatus,
}
impl Registers {
    pub fn take(bitfile_path: &str, resource: &str) -> Result<Option<Self>, ni_fpga::Error> {
        if unsafe { !SESSION.is_null() } {
            Ok(None)
        } else {
            let session =
                ni_fpga::Session::open(bitfile_path, "264D0BA312FF00B741D4742415E1D470", resource)?;
            unsafe {
                SESSION = &session as *const ni_fpga::Session;
            }
            std::mem::forget(session);
            Ok(Some(Self {
                _marker: PhantomData,
                LocalTime: LocalTime {
                    _marker: PhantomData,
                },
                Revision: Revision {
                    _marker: PhantomData,
                },
                Version: Version {
                    _marker: PhantomData,
                },
                LocalTimeUpper: LocalTimeUpper {
                    _marker: PhantomData,
                },
                LEDs: LEDs {
                    _marker: PhantomData,
                },
                UserButton: UserButton {
                    _marker: PhantomData,
                },
                InterruptForceOnce: InterruptForceOnce {
                    _marker: PhantomData,
                },
                InterruptForceNumber: InterruptForceNumber {
                    _marker: PhantomData,
                },
                SysWatchdog_Status: SysWatchdog_Status {
                    _marker: PhantomData,
                },
                SysWatchdog_Command: SysWatchdog_Command {
                    _marker: PhantomData,
                },
                SysWatchdog_Challenge: SysWatchdog_Challenge {
                    _marker: PhantomData,
                },
                SysWatchdog_Timer: SysWatchdog_Timer {
                    _marker: PhantomData,
                },
                SysWatchdog_Active: SysWatchdog_Active {
                    _marker: PhantomData,
                },
                SysWatchdog_ForcedKills: SysWatchdog_ForcedKills {
                    _marker: PhantomData,
                },
                AI_ReadSelect: AI_ReadSelect {
                    _marker: PhantomData,
                },
                AI_LatchOutput: AI_LatchOutput {
                    _marker: PhantomData,
                },
                AI_Output: AI_Output {
                    _marker: PhantomData,
                },
                AI_Config: AI_Config {
                    _marker: PhantomData,
                },
                AI_ScanList: AI_ScanList {
                    _marker: PhantomData,
                },
                AI_OversampleBits: AI_OversampleBits {
                    _marker: PhantomData,
                },
                AI_AverageBits: AI_AverageBits {
                    _marker: PhantomData,
                },
                AI_LoopTiming: AI_LoopTiming {
                    _marker: PhantomData,
                },
                Accumulator0_Center: Accumulator0_Center {
                    _marker: PhantomData,
                },
                Accumulator0_Reset: Accumulator0_Reset {
                    _marker: PhantomData,
                },
                Accumulator0_Output: Accumulator0_Output {
                    _marker: PhantomData,
                },
                Accumulator0_Deadband: Accumulator0_Deadband {
                    _marker: PhantomData,
                },
                Accumulator1_Center: Accumulator1_Center {
                    _marker: PhantomData,
                },
                Accumulator1_Reset: Accumulator1_Reset {
                    _marker: PhantomData,
                },
                Accumulator1_Output: Accumulator1_Output {
                    _marker: PhantomData,
                },
                Accumulator1_Deadband: Accumulator1_Deadband {
                    _marker: PhantomData,
                },
                AnalogTrigger_Output: AnalogTrigger_Output {
                    _marker: PhantomData,
                },
                AnalogTrigger0_SourceSelect: AnalogTrigger0_SourceSelect {
                    _marker: PhantomData,
                },
                AnalogTrigger0_UpperLimit: AnalogTrigger0_UpperLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger0_LowerLimit: AnalogTrigger0_LowerLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger1_SourceSelect: AnalogTrigger1_SourceSelect {
                    _marker: PhantomData,
                },
                AnalogTrigger1_UpperLimit: AnalogTrigger1_UpperLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger1_LowerLimit: AnalogTrigger1_LowerLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger2_SourceSelect: AnalogTrigger2_SourceSelect {
                    _marker: PhantomData,
                },
                AnalogTrigger2_UpperLimit: AnalogTrigger2_UpperLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger2_LowerLimit: AnalogTrigger2_LowerLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger3_SourceSelect: AnalogTrigger3_SourceSelect {
                    _marker: PhantomData,
                },
                AnalogTrigger3_UpperLimit: AnalogTrigger3_UpperLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger3_LowerLimit: AnalogTrigger3_LowerLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger4_SourceSelect: AnalogTrigger4_SourceSelect {
                    _marker: PhantomData,
                },
                AnalogTrigger4_UpperLimit: AnalogTrigger4_UpperLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger4_LowerLimit: AnalogTrigger4_LowerLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger5_SourceSelect: AnalogTrigger5_SourceSelect {
                    _marker: PhantomData,
                },
                AnalogTrigger6_UpperLimit: AnalogTrigger6_UpperLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger6_LowerLimit: AnalogTrigger6_LowerLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger6_SourceSelect: AnalogTrigger6_SourceSelect {
                    _marker: PhantomData,
                },
                AnalogTrigger5_UpperLimit: AnalogTrigger5_UpperLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger5_LowerLimit: AnalogTrigger5_LowerLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger7_SourceSelect: AnalogTrigger7_SourceSelect {
                    _marker: PhantomData,
                },
                AnalogTrigger7_UpperLimit: AnalogTrigger7_UpperLimit {
                    _marker: PhantomData,
                },
                AnalogTrigger7_LowerLimit: AnalogTrigger7_LowerLimit {
                    _marker: PhantomData,
                },
                PWM_LoopTiming: PWM_LoopTiming {
                    _marker: PhantomData,
                },
                PWM_CycleStartTimeUpper: PWM_CycleStartTimeUpper {
                    _marker: PhantomData,
                },
                PWM_CycleStartTime: PWM_CycleStartTime {
                    _marker: PhantomData,
                },
                PWM_Config: PWM_Config {
                    _marker: PhantomData,
                },
                PWM_PeriodScaleHdr: PWM_PeriodScaleHdr {
                    _marker: PhantomData,
                },
                PWM_PeriodScaleMXP: PWM_PeriodScaleMXP {
                    _marker: PhantomData,
                },
                PWM_ZeroLatch: PWM_ZeroLatch {
                    _marker: PhantomData,
                },
                PWM_Hdr0: PWM_Hdr0 {
                    _marker: PhantomData,
                },
                PWM_Hdr1: PWM_Hdr1 {
                    _marker: PhantomData,
                },
                PWM_Hdr2: PWM_Hdr2 {
                    _marker: PhantomData,
                },
                PWM_Hdr3: PWM_Hdr3 {
                    _marker: PhantomData,
                },
                PWM_Hdr4: PWM_Hdr4 {
                    _marker: PhantomData,
                },
                PWM_Hdr5: PWM_Hdr5 {
                    _marker: PhantomData,
                },
                PWM_Hdr6: PWM_Hdr6 {
                    _marker: PhantomData,
                },
                PWM_Hdr7: PWM_Hdr7 {
                    _marker: PhantomData,
                },
                PWM_Hdr8: PWM_Hdr8 {
                    _marker: PhantomData,
                },
                PWM_Hdr9: PWM_Hdr9 {
                    _marker: PhantomData,
                },
                PWM_MXP0: PWM_MXP0 {
                    _marker: PhantomData,
                },
                PWM_MXP1: PWM_MXP1 {
                    _marker: PhantomData,
                },
                PWM_MXP2: PWM_MXP2 {
                    _marker: PhantomData,
                },
                PWM_MXP3: PWM_MXP3 {
                    _marker: PhantomData,
                },
                PWM_MXP4: PWM_MXP4 {
                    _marker: PhantomData,
                },
                PWM_MXP5: PWM_MXP5 {
                    _marker: PhantomData,
                },
                PWM_MXP6: PWM_MXP6 {
                    _marker: PhantomData,
                },
                PWM_MXP7: PWM_MXP7 {
                    _marker: PhantomData,
                },
                PWM_MXP8: PWM_MXP8 {
                    _marker: PhantomData,
                },
                PWM_MXP9: PWM_MXP9 {
                    _marker: PhantomData,
                },
                DIO_OutputEnable: DIO_OutputEnable {
                    _marker: PhantomData,
                },
                DIO_DO: DIO_DO {
                    _marker: PhantomData,
                },
                DIO_DI: DIO_DI {
                    _marker: PhantomData,
                },
                DIO_FilterSelectHdr: DIO_FilterSelectHdr {
                    _marker: PhantomData,
                },
                DIO_FilterPeriodHdr0: DIO_FilterPeriodHdr0 {
                    _marker: PhantomData,
                },
                DIO_FilterPeriodHdr1: DIO_FilterPeriodHdr1 {
                    _marker: PhantomData,
                },
                DIO_FilterPeriodHdr2: DIO_FilterPeriodHdr2 {
                    _marker: PhantomData,
                },
                DIO_FilterSelectMXP: DIO_FilterSelectMXP {
                    _marker: PhantomData,
                },
                DIO_FilterPeriodMXP0: DIO_FilterPeriodMXP0 {
                    _marker: PhantomData,
                },
                DIO_FilterPeriodMXP1: DIO_FilterPeriodMXP1 {
                    _marker: PhantomData,
                },
                DIO_FilterPeriodMXP2: DIO_FilterPeriodMXP2 {
                    _marker: PhantomData,
                },
                DIO_EnableMXPSpecialFunction: DIO_EnableMXPSpecialFunction {
                    _marker: PhantomData,
                },
                DIO_PulseLength: DIO_PulseLength {
                    _marker: PhantomData,
                },
                DIO_Pulse: DIO_Pulse {
                    _marker: PhantomData,
                },
                DIO_PWMDutyCycleA: DIO_PWMDutyCycleA {
                    _marker: PhantomData,
                },
                DIO_PWMDutyCycleB: DIO_PWMDutyCycleB {
                    _marker: PhantomData,
                },
                DIO_PWMOutputSelect: DIO_PWMOutputSelect {
                    _marker: PhantomData,
                },
                DIO_PWMPeriodPower: DIO_PWMPeriodPower {
                    _marker: PhantomData,
                },
                Counter0_Config: Counter0_Config {
                    _marker: PhantomData,
                },
                Counter0_Reset: Counter0_Reset {
                    _marker: PhantomData,
                },
                Counter0_Output: Counter0_Output {
                    _marker: PhantomData,
                },
                Counter0_TimerConfig: Counter0_TimerConfig {
                    _marker: PhantomData,
                },
                Counter0_TimerOutput: Counter0_TimerOutput {
                    _marker: PhantomData,
                },
                Counter1_Config: Counter1_Config {
                    _marker: PhantomData,
                },
                Counter1_Reset: Counter1_Reset {
                    _marker: PhantomData,
                },
                Counter1_Output: Counter1_Output {
                    _marker: PhantomData,
                },
                Counter1_TimerConfig: Counter1_TimerConfig {
                    _marker: PhantomData,
                },
                Counter1_TimerOutput: Counter1_TimerOutput {
                    _marker: PhantomData,
                },
                Counter2_Config: Counter2_Config {
                    _marker: PhantomData,
                },
                Counter2_Reset: Counter2_Reset {
                    _marker: PhantomData,
                },
                Counter2_Output: Counter2_Output {
                    _marker: PhantomData,
                },
                Counter2_TimerConfig: Counter2_TimerConfig {
                    _marker: PhantomData,
                },
                Counter2_TimerOutput: Counter2_TimerOutput {
                    _marker: PhantomData,
                },
                Counter3_Config: Counter3_Config {
                    _marker: PhantomData,
                },
                Counter3_Reset: Counter3_Reset {
                    _marker: PhantomData,
                },
                Counter3_Output: Counter3_Output {
                    _marker: PhantomData,
                },
                Counter3_TimerConfig: Counter3_TimerConfig {
                    _marker: PhantomData,
                },
                Counter3_TimerOutput: Counter3_TimerOutput {
                    _marker: PhantomData,
                },
                Counter4_Config: Counter4_Config {
                    _marker: PhantomData,
                },
                Counter4_Reset: Counter4_Reset {
                    _marker: PhantomData,
                },
                Counter4_Output: Counter4_Output {
                    _marker: PhantomData,
                },
                Counter4_TimerConfig: Counter4_TimerConfig {
                    _marker: PhantomData,
                },
                Counter4_TimerOutput: Counter4_TimerOutput {
                    _marker: PhantomData,
                },
                Counter5_Config: Counter5_Config {
                    _marker: PhantomData,
                },
                Counter5_Reset: Counter5_Reset {
                    _marker: PhantomData,
                },
                Counter5_Output: Counter5_Output {
                    _marker: PhantomData,
                },
                Counter5_TimerConfig: Counter5_TimerConfig {
                    _marker: PhantomData,
                },
                Counter5_TimerOutput: Counter5_TimerOutput {
                    _marker: PhantomData,
                },
                Counter6_Config: Counter6_Config {
                    _marker: PhantomData,
                },
                Counter6_Reset: Counter6_Reset {
                    _marker: PhantomData,
                },
                Counter6_Output: Counter6_Output {
                    _marker: PhantomData,
                },
                Counter6_TimerConfig: Counter6_TimerConfig {
                    _marker: PhantomData,
                },
                Counter6_TimerOutput: Counter6_TimerOutput {
                    _marker: PhantomData,
                },
                Counter7_Config: Counter7_Config {
                    _marker: PhantomData,
                },
                Counter7_Reset: Counter7_Reset {
                    _marker: PhantomData,
                },
                Counter7_Output: Counter7_Output {
                    _marker: PhantomData,
                },
                Counter7_TimerConfig: Counter7_TimerConfig {
                    _marker: PhantomData,
                },
                Counter7_TimerOutput: Counter7_TimerOutput {
                    _marker: PhantomData,
                },
                Encoder0_Config: Encoder0_Config {
                    _marker: PhantomData,
                },
                Encoder0_Reset: Encoder0_Reset {
                    _marker: PhantomData,
                },
                Encoder0_Output: Encoder0_Output {
                    _marker: PhantomData,
                },
                Encoder0_TimerConfig: Encoder0_TimerConfig {
                    _marker: PhantomData,
                },
                Encoder0_TimerOutput: Encoder0_TimerOutput {
                    _marker: PhantomData,
                },
                Encoder1_Config: Encoder1_Config {
                    _marker: PhantomData,
                },
                Encoder1_Reset: Encoder1_Reset {
                    _marker: PhantomData,
                },
                Encoder1_Output: Encoder1_Output {
                    _marker: PhantomData,
                },
                Encoder1_TimerConfig: Encoder1_TimerConfig {
                    _marker: PhantomData,
                },
                Encoder1_TimerOutput: Encoder1_TimerOutput {
                    _marker: PhantomData,
                },
                Encoder2_Config: Encoder2_Config {
                    _marker: PhantomData,
                },
                Encoder2_Reset: Encoder2_Reset {
                    _marker: PhantomData,
                },
                Encoder2_Output: Encoder2_Output {
                    _marker: PhantomData,
                },
                Encoder2_TimerConfig: Encoder2_TimerConfig {
                    _marker: PhantomData,
                },
                Encoder2_TimerOutput: Encoder2_TimerOutput {
                    _marker: PhantomData,
                },
                Encoder3_Config: Encoder3_Config {
                    _marker: PhantomData,
                },
                Encoder3_Reset: Encoder3_Reset {
                    _marker: PhantomData,
                },
                Encoder3_Output: Encoder3_Output {
                    _marker: PhantomData,
                },
                Encoder3_TimerConfig: Encoder3_TimerConfig {
                    _marker: PhantomData,
                },
                Encoder3_TimerOutput: Encoder3_TimerOutput {
                    _marker: PhantomData,
                },
                Encoder4_Config: Encoder4_Config {
                    _marker: PhantomData,
                },
                Encoder4_Reset: Encoder4_Reset {
                    _marker: PhantomData,
                },
                Encoder4_Output: Encoder4_Output {
                    _marker: PhantomData,
                },
                Encoder4_TimerConfig: Encoder4_TimerConfig {
                    _marker: PhantomData,
                },
                Encoder4_TimerOutput: Encoder4_TimerOutput {
                    _marker: PhantomData,
                },
                Encoder5_Config: Encoder5_Config {
                    _marker: PhantomData,
                },
                Encoder5_Reset: Encoder5_Reset {
                    _marker: PhantomData,
                },
                Encoder5_Output: Encoder5_Output {
                    _marker: PhantomData,
                },
                Encoder5_TimerConfig: Encoder5_TimerConfig {
                    _marker: PhantomData,
                },
                Encoder5_TimerOutput: Encoder5_TimerOutput {
                    _marker: PhantomData,
                },
                Encoder6_Config: Encoder6_Config {
                    _marker: PhantomData,
                },
                Encoder6_Reset: Encoder6_Reset {
                    _marker: PhantomData,
                },
                Encoder6_Output: Encoder6_Output {
                    _marker: PhantomData,
                },
                Encoder6_TimerConfig: Encoder6_TimerConfig {
                    _marker: PhantomData,
                },
                Encoder6_TimerOutput: Encoder6_TimerOutput {
                    _marker: PhantomData,
                },
                Encoder7_Config: Encoder7_Config {
                    _marker: PhantomData,
                },
                Encoder7_Reset: Encoder7_Reset {
                    _marker: PhantomData,
                },
                Encoder7_Output: Encoder7_Output {
                    _marker: PhantomData,
                },
                Encoder7_TimerConfig: Encoder7_TimerConfig {
                    _marker: PhantomData,
                },
                Encoder7_TimerOutput: Encoder7_TimerOutput {
                    _marker: PhantomData,
                },
                Interrupt0_Config: Interrupt0_Config {
                    _marker: PhantomData,
                },
                Interrupt0_RisingTimeStamp: Interrupt0_RisingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt0_FallingTimeStamp: Interrupt0_FallingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt1_Config: Interrupt1_Config {
                    _marker: PhantomData,
                },
                Interrupt1_RisingTimeStamp: Interrupt1_RisingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt1_FallingTimeStamp: Interrupt1_FallingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt2_Config: Interrupt2_Config {
                    _marker: PhantomData,
                },
                Interrupt2_RisingTimeStamp: Interrupt2_RisingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt2_FallingTimeStamp: Interrupt2_FallingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt3_Config: Interrupt3_Config {
                    _marker: PhantomData,
                },
                Interrupt3_RisingTimeStamp: Interrupt3_RisingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt3_FallingTimeStamp: Interrupt3_FallingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt4_Config: Interrupt4_Config {
                    _marker: PhantomData,
                },
                Interrupt4_RisingTimeStamp: Interrupt4_RisingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt4_FallingTimeStamp: Interrupt4_FallingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt5_Config: Interrupt5_Config {
                    _marker: PhantomData,
                },
                Interrupt5_RisingTimeStamp: Interrupt5_RisingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt5_FallingTimeStamp: Interrupt5_FallingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt6_Config: Interrupt6_Config {
                    _marker: PhantomData,
                },
                Interrupt6_RisingTimeStamp: Interrupt6_RisingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt6_FallingTimeStamp: Interrupt6_FallingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt7_Config: Interrupt7_Config {
                    _marker: PhantomData,
                },
                Interrupt7_RisingTimeStamp: Interrupt7_RisingTimeStamp {
                    _marker: PhantomData,
                },
                Interrupt7_FallingTimeStamp: Interrupt7_FallingTimeStamp {
                    _marker: PhantomData,
                },
                DMA_Rate: DMA_Rate {
                    _marker: PhantomData,
                },
                DMA_Config: DMA_Config {
                    _marker: PhantomData,
                },
                DMA_ExternalTriggers0: DMA_ExternalTriggers0 {
                    _marker: PhantomData,
                },
                DMA_ExternalTriggers1: DMA_ExternalTriggers1 {
                    _marker: PhantomData,
                },
                Alarm_TriggerTime: Alarm_TriggerTime {
                    _marker: PhantomData,
                },
                Alarm_Enable: Alarm_Enable {
                    _marker: PhantomData,
                },
                Relay_Value: Relay_Value {
                    _marker: PhantomData,
                },
                Power_Status: Power_Status {
                    _marker: PhantomData,
                },
                Power_Disable: Power_Disable {
                    _marker: PhantomData,
                },
                Power_UserVoltage6V: Power_UserVoltage6V {
                    _marker: PhantomData,
                },
                Power_UserCurrent6V: Power_UserCurrent6V {
                    _marker: PhantomData,
                },
                Power_UserVoltage5V: Power_UserVoltage5V {
                    _marker: PhantomData,
                },
                Power_UserCurrent5V: Power_UserCurrent5V {
                    _marker: PhantomData,
                },
                Power_UserVoltage3V3: Power_UserVoltage3V3 {
                    _marker: PhantomData,
                },
                Power_UserCurrent3V3: Power_UserCurrent3V3 {
                    _marker: PhantomData,
                },
                Power_VinVoltage: Power_VinVoltage {
                    _marker: PhantomData,
                },
                Power_VinCurrent: Power_VinCurrent {
                    _marker: PhantomData,
                },
                Power_OnChipTemperature: Power_OnChipTemperature {
                    _marker: PhantomData,
                },
                Power_MXP_DIOVoltage: Power_MXP_DIOVoltage {
                    _marker: PhantomData,
                },
                Power_IntegratedIO: Power_IntegratedIO {
                    _marker: PhantomData,
                },
                Power_AOVoltage: Power_AOVoltage {
                    _marker: PhantomData,
                },
                Power_FaultCounts: Power_FaultCounts {
                    _marker: PhantomData,
                },
                Power_ResetFaultCounts: Power_ResetFaultCounts {
                    _marker: PhantomData,
                },
                BIST_Enable: BIST_Enable {
                    _marker: PhantomData,
                },
                BIST_DO0SquareEnable: BIST_DO0SquareEnable {
                    _marker: PhantomData,
                },
                BIST_DO0SquareTicks: BIST_DO0SquareTicks {
                    _marker: PhantomData,
                },
                BIST_DO0: BIST_DO0 {
                    _marker: PhantomData,
                },
                BIST_DO1SquareEnable: BIST_DO1SquareEnable {
                    _marker: PhantomData,
                },
                BIST_DO1SquareTicks: BIST_DO1SquareTicks {
                    _marker: PhantomData,
                },
                BIST_DO1: BIST_DO1 {
                    _marker: PhantomData,
                },
                AO_MXP0: AO_MXP0 {
                    _marker: PhantomData,
                },
                AO_MXP1: AO_MXP1 {
                    _marker: PhantomData,
                },
                SPI_ChipSelectActiveHigh: SPI_ChipSelectActiveHigh {
                    _marker: PhantomData,
                },
                SPI_EnableDIO: SPI_EnableDIO {
                    _marker: PhantomData,
                },
                SPI_AutoSPI1Select: SPI_AutoSPI1Select {
                    _marker: PhantomData,
                },
                SPI_AutoByteCount: SPI_AutoByteCount {
                    _marker: PhantomData,
                },
                SPI_AutoForceOne: SPI_AutoForceOne {
                    _marker: PhantomData,
                },
                SPI_AutoRate: SPI_AutoRate {
                    _marker: PhantomData,
                },
                SPI_AutoTriggerConfig: SPI_AutoTriggerConfig {
                    _marker: PhantomData,
                },
                SPI_AutoChipSelect: SPI_AutoChipSelect {
                    _marker: PhantomData,
                },
                SPI_AutoTx0: SPI_AutoTx0 {
                    _marker: PhantomData,
                },
                SPI_AutoTx1: SPI_AutoTx1 {
                    _marker: PhantomData,
                },
                SPI_AutoTx2: SPI_AutoTx2 {
                    _marker: PhantomData,
                },
                SPI_AutoTx3: SPI_AutoTx3 {
                    _marker: PhantomData,
                },
                SPI_AutoTx4: SPI_AutoTx4 {
                    _marker: PhantomData,
                },
                SPI_AutoTx5: SPI_AutoTx5 {
                    _marker: PhantomData,
                },
                SPI_TransferSkippedFullCount: SPI_TransferSkippedFullCount {
                    _marker: PhantomData,
                },
                SPI_StallConfig: SPI_StallConfig {
                    _marker: PhantomData,
                },
                SPI_DebugState: SPI_DebugState {
                    _marker: PhantomData,
                },
                SPI_DebugSubstate: SPI_DebugSubstate {
                    _marker: PhantomData,
                },
                SPI_DebugRevision: SPI_DebugRevision {
                    _marker: PhantomData,
                },
                SPI_DebugEnabled: SPI_DebugEnabled {
                    _marker: PhantomData,
                },
                SPI_DebugIntStat: SPI_DebugIntStat {
                    _marker: PhantomData,
                },
                SPI_DebugIntStatReadCount: SPI_DebugIntStatReadCount {
                    _marker: PhantomData,
                },
                Accel_ADDR: Accel_ADDR {
                    _marker: PhantomData,
                },
                Accel_CNTR: Accel_CNTR {
                    _marker: PhantomData,
                },
                Accel_DATO: Accel_DATO {
                    _marker: PhantomData,
                },
                Accel_DATI: Accel_DATI {
                    _marker: PhantomData,
                },
                Accel_CNTL: Accel_CNTL {
                    _marker: PhantomData,
                },
                Accel_STAT: Accel_STAT {
                    _marker: PhantomData,
                },
                Accel_CNFG: Accel_CNFG {
                    _marker: PhantomData,
                },
                Accel_GO: Accel_GO {
                    _marker: PhantomData,
                },
                HMB_Config: HMB_Config {
                    _marker: PhantomData,
                },
                HMB_ForceOnce: HMB_ForceOnce {
                    _marker: PhantomData,
                },
                DutyCycle0_Source: DutyCycle0_Source {
                    _marker: PhantomData,
                },
                DutyCycle0_Frequency: DutyCycle0_Frequency {
                    _marker: PhantomData,
                },
                DutyCycle0_Output: DutyCycle0_Output {
                    _marker: PhantomData,
                },
                DutyCycle1_Source: DutyCycle1_Source {
                    _marker: PhantomData,
                },
                DutyCycle1_Frequency: DutyCycle1_Frequency {
                    _marker: PhantomData,
                },
                DutyCycle1_Output: DutyCycle1_Output {
                    _marker: PhantomData,
                },
                DutyCycle2_Source: DutyCycle2_Source {
                    _marker: PhantomData,
                },
                DutyCycle2_Frequency: DutyCycle2_Frequency {
                    _marker: PhantomData,
                },
                DutyCycle2_Output: DutyCycle2_Output {
                    _marker: PhantomData,
                },
                DutyCycle3_Source: DutyCycle3_Source {
                    _marker: PhantomData,
                },
                DutyCycle3_Frequency: DutyCycle3_Frequency {
                    _marker: PhantomData,
                },
                DutyCycle3_Output: DutyCycle3_Output {
                    _marker: PhantomData,
                },
                DutyCycle4_Source: DutyCycle4_Source {
                    _marker: PhantomData,
                },
                DutyCycle4_Frequency: DutyCycle4_Frequency {
                    _marker: PhantomData,
                },
                DutyCycle4_Output: DutyCycle4_Output {
                    _marker: PhantomData,
                },
                DutyCycle5_Source: DutyCycle5_Source {
                    _marker: PhantomData,
                },
                DutyCycle5_Frequency: DutyCycle5_Frequency {
                    _marker: PhantomData,
                },
                DutyCycle5_Output: DutyCycle5_Output {
                    _marker: PhantomData,
                },
                DutyCycle6_Source: DutyCycle6_Source {
                    _marker: PhantomData,
                },
                DutyCycle6_Frequency: DutyCycle6_Frequency {
                    _marker: PhantomData,
                },
                DutyCycle6_Output: DutyCycle6_Output {
                    _marker: PhantomData,
                },
                DutyCycle7_Source: DutyCycle7_Source {
                    _marker: PhantomData,
                },
                DutyCycle7_Frequency: DutyCycle7_Frequency {
                    _marker: PhantomData,
                },
                DutyCycle7_Output: DutyCycle7_Output {
                    _marker: PhantomData,
                },
                LED_OutputSelect: LED_OutputSelect {
                    _marker: PhantomData,
                },
                LED_StringLength: LED_StringLength {
                    _marker: PhantomData,
                },
                LED_Load: LED_Load {
                    _marker: PhantomData,
                },
                LED_Reset: LED_Reset {
                    _marker: PhantomData,
                },
                LED_Start: LED_Start {
                    _marker: PhantomData,
                },
                LED_Abort: LED_Abort {
                    _marker: PhantomData,
                },
                LED_SyncTiming: LED_SyncTiming {
                    _marker: PhantomData,
                },
                LED_HighBitTickTiming: LED_HighBitTickTiming {
                    _marker: PhantomData,
                },
                LED_LowBitTickTiming: LED_LowBitTickTiming {
                    _marker: PhantomData,
                },
                LED_Active: LED_Active {
                    _marker: PhantomData,
                },
                LED_PixelWriteIndex: LED_PixelWriteIndex {
                    _marker: PhantomData,
                },
                LED_PixelOutputIndex: LED_PixelOutputIndex {
                    _marker: PhantomData,
                },
                ViControl: ViControl {
                    _marker: PhantomData,
                },
                DiagramReset: DiagramReset {
                    _marker: PhantomData,
                },
                InterruptEnable: InterruptEnable {
                    _marker: PhantomData,
                },
                InterruptMask: InterruptMask {
                    _marker: PhantomData,
                },
                InterruptStatus: InterruptStatus {
                    _marker: PhantomData,
                },
            }))
        }
    }
}
impl Drop for Registers {
    fn drop(&mut self) {
        std::mem::drop(unsafe { SESSION.as_ref() }.unwrap())
    }
}
