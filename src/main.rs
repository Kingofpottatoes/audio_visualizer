use realfft::{RealFftPlanner, RealToComplex};
use std::io::{Read, Write};
use std::process::{ChildStdout, Command, Stdio};

fn get_amplitudes(fft: &dyn RealToComplex<f32>, signal: &mut Vec<f32>) -> Vec<f32> {
    let mut entree = fft.make_input_vec();
    let mut sortie = fft.make_output_vec();

    entree.copy_from_slice(&signal);

    fft.process(&mut entree, &mut sortie).unwrap();

    let mut output: Vec<f32> = vec![0.0; sortie.len()];

    for i in 0..sortie.len() {
        output[i] = sortie[i].norm();
    }
    output
}

fn applies_hann(signal: &mut Vec<f32>) {
    let n = signal.len();
    let pi = std::f32::consts::PI;
    for i in 0..n {
        let w = 0.5 * (1.0 - (2.0 * pi * (i as f32) / ((n as f32) - 1.0)).cos());
        signal[i] = signal[i] * w;
    }
}
fn calcul_fk(f_max: f32, f_min: f32, n_barres: usize, k: usize) -> f32 {
    f_min * (f_max / f_min).powf((k as f32) / (n_barres as f32))
}
fn get_bandes(amplitudes: &Vec<f32>, taux: f32, n_barres: usize) -> Vec<f32> {
    let n_fft = (amplitudes.len() - 1) * 2;
    let f_min: f32 = 50.0;
    let f_max: f32 = 12000.0;
    let mut sliced_amplitudes = Vec::new();
    if n_barres == 0 {
        return Vec::new();
    }
    for i in 0..n_barres {
        let fk_min = calcul_fk(f_max, f_min, n_barres, i);
        let fk_max = calcul_fk(f_max, f_min, n_barres, i + 1);
        let mut debut = get_bin(fk_min, n_fft, taux);
        let mut fin = get_bin(fk_max, n_fft, taux);
        debut = debut.min(amplitudes.len());
        fin = fin.max(debut + 1);
        fin = fin.min(amplitudes.len());
        sliced_amplitudes.push(get_max(&amplitudes[debut..fin]));
    }
    sliced_amplitudes
}
fn get_bin(freq: f32, n_nfft: usize, taux: f32) -> usize {
    (freq * ((n_nfft as f32) / taux)).round() as usize
}
fn get_max(vec: &[f32]) -> f32 {
    let mut i_max = 0;
    if vec.len() == 0 {
        return 0.0;
    }
    for i in 0..vec.len() {
        if vec[i_max] < vec[i] {
            i_max = i;
        }
    }
    vec[i_max]
}
fn amplitude_to_decibels(amplitude: f32, nfft: usize) -> f32 {
    20.0 * ((amplitude / ((nfft as f32) / 4.0)).max(1e-6).log10())
}
fn normalize(db: f32, plafond: f32, plancher: f32) -> f32 {
    ((db - plancher) / (plafond - plancher)).clamp(0.0, 1.0)
}
struct Lisseur {
    son: Vec<f32>,
    k: f32,
}
impl Lisseur {
    fn new(n_barres: usize, k: f32) -> Lisseur {
        Lisseur {
            son: vec![0.0; n_barres],
            k: k,
        }
    }
    fn update(&mut self, target: &Vec<f32>) {
        if self.son.len() != target.len() {
            return;
        }
        for i in 0..self.son.len() {
            if target[i] >= self.son[i] {
                self.son[i] = target[i];
            } else {
                self.son[i] = self.son[i] + (target[i] - self.son[i]) * self.k;
            }
        }
    }
}
fn frac_to_percent(son: &[f32]) -> Vec<u32> {
    let mut percent = vec![0; son.len()];
    for i in 0..son.len() {
        percent[i] = ((son[i] * 100.0).round()) as u32;
    }
    percent
}
fn to_line(son: &[u32]) -> String {
    let mut txt = String::new();
    for i in 0..son.len() {
        if i > 0 {
            txt.push(';');
        }
        txt.push_str(&son[i].to_string())
    }
    txt
}
fn analyse(
    signal: &mut Vec<f32>,
    fft: &dyn RealToComplex<f32>,
    taux: f32,
    n_barres: usize,
    plafond: f32,
    plancher: f32,
) -> Vec<f32> {
    applies_hann(signal);
    let amplitude = get_amplitudes(fft, signal);
    let mut bandes = get_bandes(&amplitude, taux, n_barres);
    for i in 0..bandes.len() {
        bandes[i] = amplitude_to_decibels(bandes[i], signal.len());
        bandes[i] = normalize(bandes[i], plafond, plancher);
    }
    bandes
}
fn get_echantillons(tuyau: &mut ChildStdout, n: usize) -> Vec<f32> {
    let mut tampon = vec![0; n * 4];
    if let Err(e) = tuyau.read_exact(&mut tampon) {
        eprintln!("flux audio interrompu : {}", e);
        std::process::exit(1);
    }
    let mut echantillon = vec![0.0; n];
    let mut i: usize = 0;
    for bytes in tampon.chunks_exact(4) {
        echantillon[i] = f32::from_le_bytes(bytes.try_into().unwrap());
        i += 1;
    }
    echantillon
}
struct Attaque {
    barres: Vec<f32>,
    seuil: f32,
    gain: f32,
    retombee: f32,
}
impl Attaque {
    fn new(n_barres: usize, seuil: f32, gain: f32, retombee: f32) -> Attaque {
        Attaque {
            barres: vec![0.0; n_barres],
            seuil: seuil,
            gain: gain,
            retombee: retombee,
        }
    }
    fn update(&mut self, avant: &[f32], apres: &[f32]) {
        if (self.barres.len() != avant.len()) || (avant.len() != apres.len()) {
            return;
        }
        for i in 0..self.barres.len() {
            let mut hausse_i = (apres[i] - avant[i]).max(0.0);
            if hausse_i < self.seuil {
                hausse_i = 0.0;
            } else {
                hausse_i = hausse_i * self.gain;
            }
            self.barres[i] = hausse_i.max(self.barres[i] * self.retombee);
        }
    }
}
fn main() {
    //data
    let taux: f32 = 48000.0;
    let format = "float32le";
    let n: usize = 2048;
    let canaux = 1;
    let latence = 20;
    //normalisation
    let plafond = -20.0;
    let plancher = -80.0;
    //appearence
    let n_barres = 32;
    let k: f32 = 0.1;
    let seuil = 0.1;
    let gain = 4.0;
    let retombee = 0.75;
    let pas: usize = 800;

    let device = match Command::new("pactl").arg("get-default-sink").output() {
        Err(_) => {
            eprintln!("pls install libpulse");
            std::process::exit(1);
        }
        Ok(sortie) if !sortie.status.success() => {
            eprintln!("pactl didnt respond, Pipewire or PulseAudio running ?");
            std::process::exit(1);
        }
        Ok(sortie) => {
            let device = String::from_utf8_lossy(&sortie.stdout).trim().to_string();
            if device.is_empty() {
                eprintln!("no device");
                std::process::exit(1);
            }
            format!("{}.monitor", device)
        }
    };

    let mut child = match Command::new("parec")
        .arg(format!("--device={}", device))
        .arg(format!("--format={}", format))
        .arg(format!("--rate={}", taux as u32))
        .arg(format!("--channels={}", canaux))
        .arg(format!("--latency-msec={}", latence))
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error parec: {}", e);
            std::process::exit(1);
        }
    };

    let mut tuyau = child.stdout.take().unwrap();

    let mut lisseur = Lisseur::new(n_barres, k);
    let mut attaque = Attaque::new(n_barres, seuil, gain, retombee);

    let mut planner = RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(n);

    let mut window = vec![0.0f32; n];

    loop {
        let new_echantillons = get_echantillons(&mut tuyau, pas);
        window.drain(..pas);
        window.extend(new_echantillons);
        let mut echantillons = window.clone();
        let barres = analyse(
            &mut echantillons,
            fft.as_ref(),
            taux,
            n_barres,
            plafond,
            plancher,
        );
        let avant = lisseur.son.clone();
        lisseur.update(&barres);
        attaque.update(&avant, &lisseur.son);
        let barres_sizes = frac_to_percent(&lisseur.son);
        let mut line = barres_sizes;
        line.extend(frac_to_percent(&attaque.barres));
        println!("{}", to_line(&line));
        std::io::stdout().flush().unwrap();
    }
}
