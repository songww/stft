#![feature(test)]

extern crate test;

use num_complex::Complex;
use num_traits::Zero;

use rustfft::FftPlanner;

use stft::{WindowType, STFT};

macro_rules! bench_fft_process {
    ($bencher:expr, $window_size:expr, $float:ty) => {{
        let window_size = $window_size;
        let fft = FftPlanner::<$float>::new().plan_fft_forward(window_size);
        let mut input = vec![Complex::<$float>::zero(); window_size];
        // let output = vec![Complex::<$float>::zero(); window_size];
        $bencher.iter(|| fft.process(&mut input));
    }};
}

#[bench]
fn bench_fft_process_1024_f32(bencher: &mut test::Bencher) {
    bench_fft_process!(bencher, 1024, f32);
}

#[bench]
fn bench_fft_process_1024_f64(bencher: &mut test::Bencher) {
    bench_fft_process!(bencher, 1024, f64);
}

macro_rules! bench_stft_compute {
    ($bencher:expr, $window_size:expr, $float:ty) => {{
        let mut stft = STFT::<$float>::new(WindowType::Hanning, $window_size, 0);
        let input: Vec<$float> = vec![0.; $window_size];
        let mut output: Vec<$float> = vec![0.; stft.output_size()];
        stft.append_samples(&input[..]);
        $bencher.iter(|| stft.compute_column(&mut output));
    }};
}

#[bench]
fn bench_stft_compute_1024_f32(bencher: &mut test::Bencher) {
    bench_stft_compute!(bencher, 1024, f32);
}

#[bench]
fn bench_stft_compute_1024_f64(bencher: &mut test::Bencher) {
    bench_stft_compute!(bencher, 1024, f64);
}

macro_rules! bench_stft_audio {
    ($bencher:expr, $seconds:expr, $float:ty) => {{
        // let's generate some fake audio
        let sample_rate: usize = 44100;
        let seconds: usize = $seconds;
        let sample_count = sample_rate * seconds;
        let all_samples = (0..sample_count)
            .map(|x| x as $float)
            .collect::<Vec<$float>>();
        $bencher.iter(|| {
            // let's initialize our short-time fourier transform
            let window_type: WindowType = WindowType::Hanning;
            let window_size: usize = 1024;
            let step_size: usize = 512;
            let mut stft = STFT::<$float>::new(window_type, window_size, step_size);
            // we need a buffer to hold a computed column of the spectrogram
            let mut spectrogram_column = vec![0. as $float; stft.output_size()];
            for some_samples in (&all_samples[..]).chunks(3000) {
                stft.append_samples(some_samples);
                while stft.contains_enough_to_compute() {
                    stft.compute_column(&mut spectrogram_column[..]);
                    stft.move_to_next_column();
                }
            }
        });
    }};
}

#[bench]
fn bench_stft_10_seconds_audio_f32(bencher: &mut test::Bencher) {
    bench_stft_audio!(bencher, 10, f32);
}

#[bench]
fn bench_stft_10_seconds_audio_f64(bencher: &mut test::Bencher) {
    bench_stft_audio!(bencher, 10, f64);
}
