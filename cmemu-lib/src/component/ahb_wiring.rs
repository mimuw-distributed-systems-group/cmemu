use crate::component::{
    aon_bus, aon_bus::AonBusComponent, aon_event::AONEventComponent, bus_matrix, core,
    dwt::DWTComponent, event_fabric::EventFabricComponent, flash::FlashComponent,
    gpio::GPIOComponent, gpram::GPRAMComponent, mem_mock::MemoryMockComponent, nvic::NVICComponent,
    osc::OSCComponent, prcm::PRCMComponent, rfc::RFCComponent, rom::ROMComponent,
    rtc::RTCComponent, rtc_bypass::RTCBypass, sram::SRAMComponent, sysbus,
    uart_lite::UARTLiteComponent, vims, wuc::WUCComponent,
};
use crate::{bridge_ports, terminate_port};

bridge_ports!(@proxied @master vims::FlashMPort => @proxied @slave FlashComponent);
bridge_ports!(@proxied @master vims::GpramMPort => @proxied @slave GPRAMComponent);
bridge_ports!(@proxied @master vims::RomMPort => @proxied @slave ROMComponent);

bridge_ports!(@proxied @master bus_matrix::ICodeM => @proxied @slave vims::ICodeSPort);
bridge_ports!(@proxied @master bus_matrix::DCodeM => @proxied @slave vims::DCodeSPort);
bridge_ports!(@proxied @master bus_matrix::SysbusM => @proxied @slave sysbus::CoreSPort);
bridge_ports!(@proxied @master bus_matrix::NvicM => @proxied @slave NVICComponent);
bridge_ports!(@proxied @master bus_matrix::DwtM => @proxied @slave DWTComponent);

bridge_ports!(@proxied @master sysbus::VimsMPort => @proxied @slave vims::SysbusSPort);
bridge_ports!(@proxied @master sysbus::SramMPort => @proxied @slave SRAMComponent);
bridge_ports!(@proxied @master sysbus::MemMockMPort => @proxied @slave MemoryMockComponent);
bridge_ports!(@proxied @master sysbus::UartLiteMPort => @proxied @slave UARTLiteComponent);
bridge_ports!(@proxied @master sysbus::PrcmMPort => @proxied @slave PRCMComponent);
bridge_ports!(@proxied @master sysbus::AonBusMPort => @proxied @slave AonBusComponent);
bridge_ports!(@proxied @master sysbus::GpioMPort => @proxied @slave GPIOComponent);
bridge_ports!(@proxied @master sysbus::RfcMPort => @proxied @slave RFCComponent);
bridge_ports!(@proxied @master sysbus::EventFabricMPort => @proxied @slave EventFabricComponent);
bridge_ports!(@proxied @master sysbus::RTCBypassMPort => @proxied @slave RTCBypass);

bridge_ports!(@proxied @master core::IBusM => @proxied @slave bus_matrix::IBusS);
bridge_ports!(@proxied @master core::DBusM => @proxied @slave bus_matrix::DBusS);
terminate_port!(@configured_slave bus_matrix::DebugS);

// Slow busses follow next
bridge_ports!(@proxied @master aon_bus::OscMPort => @proxied @slave OSCComponent);
bridge_ports!(@proxied @master aon_bus::RtcMPort => @proxied @slave RTCComponent);
bridge_ports!(@proxied @master aon_bus::AonEventMPort => @proxied @slave AONEventComponent);
bridge_ports!(@proxied @master aon_bus::WucMPort => @proxied @slave WUCComponent);
