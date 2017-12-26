use toml;
use rb;
use dft;

/// Information to be used in visualization
pub struct AudioInfo {
    /// Left channel spectrum, packed into COLUMNS
    /// columns and normalized
    pub columns_left: Vec<f32>,

    /// Right channel spectrum, packed into COLUMNS
    /// columns and normalized
    pub columns_right: Vec<f32>,

    /// Raw left channel spectrum
    /// Warning: This data is not normalized, you have
    /// to take care of that yourself!
    pub raw_spectrum_left: Vec<f32>,

    /// Raw right channel spectrum
    /// Warning: This data is not normalized, you have
    /// to take care of that yourself!
    pub raw_spectrum_right: Vec<f32>,
}

pub struct Spectralizer {
    // Parameters from config
    window_size: usize,
    columns: usize,
    max_dampen: f32,
    dampen: f32,

    // Local vars
    buffer: Vec<[f32; 2]>,
    max: f32,

    // FFT closure
    spectralize_ch: Box<FnMut(&mut [f32], &Vec<[f32; 2]>, usize)>,

    // RingBuffer and AudioInfo
    audio_consumer: rb::Consumer<[f32; 2]>,
    audio_buffer: rb::SpscRb<[f32; 2]>,
    audio_info: ::std::sync::Arc<::std::sync::RwLock<AudioInfo>>,
}

impl Spectralizer {
    pub fn new(
        config: ::std::sync::Arc<toml::Value>,
        audio_buffer: rb::SpscRb<[f32; 2]>,
        audio_info: ::std::sync::Arc<::std::sync::RwLock<AudioInfo>>,
    ) -> Spectralizer {
        let window_size = config
            .get("WINDOW_SIZE")
            .map(|v| v.as_integer().expect("WINDOW_SIZE must be an integer"))
            .unwrap_or(2048) as usize;
        info!("WINDOW_SIZE = {}", window_size);

        let columns = config
            .get("COLUMNS")
            .map(|v| v.as_integer().expect("COLUMNS must be an integer"))
            .unwrap_or(50) as usize;
        info!("COLUMNS = {}", columns);

        let max_dampen = config
            .get("MAX_DAMPEN")
            .map(|v| v.as_float().expect("MAX_DAMPEN must be a float"))
            .unwrap_or(0.99) as f32;
        info!("MAX_DAMPEN = {}", max_dampen);

        let dampen = config
            .get("DAMPEN")
            .map(|v| v.as_float().expect("DAMPEN must be a float"))
            .unwrap_or(0.8) as f32;
        info!("DAMPEN = {}", dampen);

        let mut buffer = Vec::with_capacity(window_size);
        let mut transform_buffer = Vec::with_capacity(window_size);

        for _ in 0..window_size {
            buffer.push([0.0, 0.0]);
        }

        {
            let mut ai = audio_info.write().expect("Couldn't write audio info");
            ai.columns_left = Vec::with_capacity(columns);
            ai.columns_right = Vec::with_capacity(columns);
            ai.raw_spectrum_left = Vec::with_capacity(window_size);
            ai.raw_spectrum_right = Vec::with_capacity(window_size);
            for _ in 0..columns {
                ai.columns_left.push(0.0);
                ai.columns_right.push(0.0);
            }
            for _ in 0..window_size {
                ai.raw_spectrum_left.push(0.0);
                ai.raw_spectrum_right.push(0.0);
            }
        }

        let audio_consumer = {
            use rb::RB;

            audio_buffer.consumer()
        };

        let plan = dft::Plan::new(dft::Operation::Forward, window_size);

        let spectralize_ch = Box::new(move |out: &mut [f32], buf: &Vec<[f32; 2]>, ch| {
            transform_buffer.clear();
            for s in buf {
                transform_buffer.push(s[ch]);
            }

            dft::transform(&mut transform_buffer, &plan);
            let output = dft::unpack(&transform_buffer);

            // Old version returned newly allocated buffer
            // let mut out_buffer = Vec::with_capacity(window_size);
            // for ref c in output {
            //    out_buffer.push(c.norm());
            // }
            for (i, ref c) in output.iter().enumerate() {
                out[i] = c.norm();
            }

        });

        Spectralizer {
            window_size,
            columns,
            max_dampen,
            dampen,
            buffer,
            max: 0.0,

            spectralize_ch,

            audio_consumer,
            audio_buffer,
            audio_info,
        }
    }

    pub fn spectralize(&mut self) {
        use rb::RbConsumer;
        use rb::RbInspector;

        // Move ring buffer one window_size behind its end
        let count = self.audio_buffer.count();
        if count < self.window_size {
            // We can't spectralize if there are less samples
            // in the buffer than our window is wide
            return;
        } else if count > self.window_size {
            self.audio_consumer.skip(count - self.window_size).expect(
                "Can't move ring buffer forward",
            );
        }

        self.audio_consumer.get(&mut self.buffer).expect(
            "Can't read audio buffer",
        );
        // audio_consumer.read_blocking(&mut buffer).unwrap();


        {
            // Data access block
            let mut ai = self.audio_info.write().expect(
                "Couldn't write to audio info",
            );

            // Transform
            (*self.spectralize_ch)(&mut ai.raw_spectrum_left, &self.buffer, 0);
            (*self.spectralize_ch)(&mut ai.raw_spectrum_right, &self.buffer, 1);

            self.max *= self.max_dampen; // Dampen
            for column in 1..self.columns {
                let c1 = column * (self.window_size / self.columns / 2);
                let c2 = (column + 1) * (self.window_size / self.columns / 2);
                let mut sum_l: f32 = 0.0;
                let mut sum_r: f32 = 0.0;
                for x in c1..c2 {
                    sum_l += ai.raw_spectrum_left[x];
                    sum_r += ai.raw_spectrum_right[x];
                }

                self.max = self.max.max(sum_l);
                self.max = self.max.max(sum_r);
                let left = sum_l.max(0.0) / self.max;
                let right = sum_r.max(0.0) / self.max;
                ai.columns_left[column] = (ai.columns_left[column] * self.dampen).max(left);
                ai.columns_right[column] = (ai.columns_right[column] * self.dampen).max(right);
            }
        }
    }
}
