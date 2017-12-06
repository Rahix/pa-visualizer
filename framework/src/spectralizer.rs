use toml;
use rb;
use dft;

/// Information to be used in visualization
pub struct AudioInfo {
    /// Left channel spectrum
    pub spectrum_left: Vec<f32>,

    /// Right channel spectrum
    pub spectrum_right: Vec<f32>,
}

/// Convert raw audio signal into a spectrum
pub fn spectralizer(config: ::std::sync::Arc<toml::Value>,
                    audio_buffer: rb::SpscRb<[f32; 2]>,
                    audio_info: ::std::sync::Arc<::std::sync::RwLock<AudioInfo>>) {
    use rb::RbInspector;

    let window_size = config.get("WINDOW_SIZE")
        .map(|v| v.as_integer().expect("WINDOW_SIZE must be an integer"))
        .unwrap_or(2048) as usize;
    info!("WINDOW_SIZE = {}", window_size);

    let columns = config.get("COLUMNS")
        .map(|v| v.as_integer().expect("COLUMNS must be an integer"))
        .unwrap_or(50) as usize;
    info!("COLUMNS = {}", columns);

    let max_dampen = config.get("MAX_DAMPEN")
        .map(|v| v.as_float().expect("MAX_DAMPEN must be a float"))
        .unwrap_or(0.95) as f32;
    info!("MAX_DAMPEN = {}", max_dampen);

    let dampen = config.get("DAMPEN")
        .map(|v| v.as_float().expect("DAMPEN must be a float"))
        .unwrap_or(0.8) as f32;
    info!("DAMPEN = {}", dampen);

    let mut buffer = Vec::with_capacity(window_size);

    for _ in 0..window_size {
        buffer.push([0.0, 0.0]);
    }

    {
        let mut ai = audio_info.write().expect("Couldn't write audio info");
        ai.spectrum_left = Vec::with_capacity(columns);
        ai.spectrum_right = Vec::with_capacity(columns);
        for _ in 0..columns {
            ai.spectrum_left.push(0.0);
            ai.spectrum_right.push(0.0);
        }
    }

    let mut transform_buffer = Vec::with_capacity(window_size);
    let audio_consumer = {
        use rb::RB;

        audio_buffer.consumer()
    };

    let plan = dft::Plan::new(dft::Operation::Forward, window_size);

    let mut spectralize = |buf: &Vec<[f32; 2]>, ch| -> Vec<f32> {
        transform_buffer.clear();
        for s in buf {
            transform_buffer.push(s[ch]);
        }

        dft::transform(&mut transform_buffer, &plan);
        let output = dft::unpack(&transform_buffer);

        let mut out_buffer = Vec::with_capacity(window_size);
        for ref c in output {
            out_buffer.push(c.norm());
        }

        out_buffer
    };

    let mut max: f32 = 0.0;

    //let _buff_cap = audio_buffer.capacity() as i64;

    loop {
        use rb::RbConsumer;

        // Move ring buffer one window_size behind its end
        let count = audio_buffer.count();
        if count < window_size {
            continue;
        } else if count > window_size {
            audio_consumer.skip(count - window_size).expect("Can't move ring buffer forward");
        }

        audio_consumer.get(&mut buffer).expect("Can't read audio buffer");
        // audio_consumer.read_blocking(&mut buffer).unwrap();

        let freqs_l = spectralize(&buffer, 0);
        let freqs_r = spectralize(&buffer, 1);

        let mut ai = audio_info.write().expect("Couldn't write to audio info");
        // ai.spectrum_left.clear();
        // ai.spectrum_right.clear();
        max *= max_dampen;  // Dampen
        for column in 1..columns {
            let c1 = column * (window_size / columns / 2);
            let c2 = (column + 1) * (window_size / columns / 2);
            let mut sum_l: f32 = 0.0;
            let mut sum_r: f32 = 0.0;
            for x in c1..c2 {
                sum_l += freqs_l[x];
                sum_r += freqs_r[x];
            }

            max = max.max(sum_l);
            max = max.max(sum_r);
            let left = sum_l.max(0.0) / max;
            let right = sum_r.max(0.0) / max;
            ai.spectrum_left[column] = (ai.spectrum_left[column] * dampen).max(left);
            ai.spectrum_right[column] = (ai.spectrum_right[column] * dampen).max(right);
        }
        ::std::thread::sleep(::std::time::Duration::from_millis(15));
    }
}
