use toml;
use rb;
use pulse_simple;

/// Record audio into a ring buffer
pub fn recorder(config: ::std::sync::Arc<toml::Value>, audio_producer: rb::Producer<[f32; 2]>) {
    let read_buffer_size = config
        .get("RECORD_READ_BUFFER_SIZE")
        .map(|v| {
            v.as_integer().expect(
                "RECORD_READ_BUFFER_SIZE must be an integer",
            )
        })
        .unwrap_or(2048) as usize;
    info!("RECORD_READ_BUFFER_SIZE = {}", read_buffer_size);

    let rate = config
        .get("RECORD_RATE")
        .map(|v| v.as_integer().expect("RECORD_RATE must be an integer"))
        .unwrap_or(48000) as u32;
    info!("RATE = {}", rate);

    let recorder: pulse_simple::Record<[f32; 2]> =
        pulse_simple::Record::new("PAVisualizer", "PulseAudio vizializer", None, rate);
    let mut buffer = Vec::with_capacity(read_buffer_size);

    for _ in 0..read_buffer_size {
        buffer.push([0.0, 0.0]);
    }

    loop {
        use rb::RbProducer;

        recorder.read(&mut buffer[..]);
        audio_producer.write_blocking(&buffer).expect(
            "Failed to write to RingBuffer",
        );
    }
}
