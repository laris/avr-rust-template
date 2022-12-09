
enum GPIOMode {
  InputADC,
  InputTriState,
  InputPullUp,       // same with OutputPullUp
  OutputTriState,
  OutputPullUp,
}

// map to DDR/PORT/PIN/PUD

trait PullUp {
      fn default();
      fn set();
      fn get();
      fn toggle();
}

trait DataDirection {
      fn default();
      fn set();
      fn get();
      fn toggle();
}

trait DataInput {
      fn default();
      fn read();
      fn write();
}

trait DataOutput {
      fn default();
      fn read();
      fn write();
}

trait GPIOPort {
      fn read();
      fn write();
      fn default();
      fn set_mode();
      fn get_mode();
      fn set_onoff();
      fn get_onoff();
      fn toggle_onoff();
      fn set_high();
      fn is_high();
      fn set_low();
      fn is_low();
      fn toggle();
}

trait GPIOPin {
      fn default();
      fn set_mode();
      fn get_mode();
      fn set_onoff();
      fn get_onoff();
      fn toggle_onoff();
      fn set_high();
      fn is_high();
      fn set_low();
      fn is_low();
      fn toggle();
}

