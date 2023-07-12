use hound;

const SAMPLE_RATE: f32 = 44100.0;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create("edm.wav", spec).unwrap();

    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    wav.extend(sawtooth_wave(60, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(67, calc_len(bpm, 4), 0.5));
    for i in 0..3 {
        wav.extend(sawtooth_wave(69, calc_len(bpm, 4), 0.5));
        wav.extend(sawtooth_wave(69, calc_len(bpm, 4), 0.0));
    }
    for i in 0..3 {
        wav.extend(sawtooth_wave(69, calc_len(bpm, 8), 0.5));
        wav.extend(sawtooth_wave(69, calc_len(bpm, 8), 0.0)); 
    }
    for i in 0..3 {
        wav.extend(sawtooth_wave(69, calc_len(bpm, 16), 0.5));
        wav.extend(sawtooth_wave(69, calc_len(bpm, 16), 0.0));
    }
    for i in 0..4 {
        wav.extend(sawtooth_wave(69, calc_len(bpm, 32), 0.5));
        wav.extend(sawtooth_wave(69, calc_len(bpm, 32), 0.0));
    }
    for i in 0..11 {
        wav.extend(sawtooth_wave(69, calc_len(bpm, 48), 0.5));
        wav.extend(sawtooth_wave(69, calc_len(bpm, 48), 0.0));
    }
    // 
    wav.extend(sawtooth_wave(30, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(30, calc_len(bpm, 32), 0.0));
    wav.extend(sawtooth_wave(30, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(30, calc_len(bpm, 32), 0.0));
    wav.extend(sawtooth_wave(30, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(30, calc_len(bpm, 32), 0.0));
    wav.extend(sawtooth_wave(30, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(30, calc_len(bpm, 32), 0.0));
    wav.extend(sawtooth_wave(48, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(48, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(52, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(52, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(38, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(38, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(42, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(42, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(48, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(48, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(52, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(52, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(38, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(38, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(42, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(42, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(48, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(48, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(52, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(52, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(38, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(38, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(42, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(42, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.0));
    // 
    wav.extend(sawtooth_wave(48, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(48, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(54, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(52, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(52, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(38, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(38, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(42, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(42, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(66, calc_len(bpm, 24), 0.5));
    wav.extend(sawtooth_wave(66, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(72, calc_len(bpm, 12), 0.5));
    wav.extend(sawtooth_wave(72, calc_len(bpm, 24), 0.0));
    wav.extend(sawtooth_wave(70, calc_len(bpm, 12), 0.5));
    wav.extend(sawtooth_wave(70, calc_len(bpm, 24), 0.0));
    for i in 0..84 {
        wav.extend(sawtooth_wave(84-i, calc_len(bpm, 88), 0.5));
    }

    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        // println!("{}", v);
    }
}

fn noteno_to_hz(no: i32) -> f32 {
    440.0 * 2.0f32.powf((no-69) as f32 / 12.0)
}

fn calc_len(bpm: usize, n: usize) -> usize {
    let base_len = (60.0 / bpm as f32) * SAMPLE_RATE;
    ((4.0 / n as f32) * base_len) as usize
}

fn sawtooth_wave(noteno: i32, len: usize, gain: f32) -> Vec<f32> {
    let tone = noteno_to_hz(noteno);
    let form_samples = SAMPLE_RATE / tone;
    let mut wav:Vec<f32> = vec![0.0; len];
    for i in 0..len {
        let pif = (i as f32 / form_samples) % 1.0;
        wav[i] = pif * 2.0 - 1.0;
    }
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}