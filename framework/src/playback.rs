pub fn playback_frame(render_info: &mut ::RenderingInfo) -> bool {
    use rb::RbProducer;
    let n = (render_info.sample_rate as f32 * render_info.frame_time) as usize;
    let mut buf = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(s_left) = render_info.samples.next() {
            if let Some(s_right) = render_info.samples.next() {
                let s_left = s_left.expect("Sample reading error");
                let s_right = s_right.expect("Sample reading error");
                buf.push([s_left, s_right]);
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    render_info.audio_producer.write(&buf).expect(
        "Could not write to RingBuffer",
    );
    true
}
