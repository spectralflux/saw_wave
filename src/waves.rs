use note;

pub fn saw_wave(t: f64, current_note: u8, level: f64) -> f64 {
    let note_hz =  note::midi_note_to_hz(current_note);
    let full_period_time = 1.0 / note_hz;
    let local_time = t % full_period_time;

    let signal = (level as f64) * (local_time / full_period_time) * 2.0 - 1.0;
    signal
}