/// Convert the midi note into the equivalent frequency. Assumes A4=440Hz
pub fn midi_note_to_hz(note: u8) -> f64 {
    const A4: f64 = 440.0;

    (A4 / 32.0) * ((note as f64 - 9.0) / 12.0).exp2()
}
