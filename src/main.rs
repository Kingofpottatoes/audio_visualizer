use std::io::Read;
use std::process::{ChildStdout, Command, Stdio};
use realfft::{RealFftPlanner, RealToComplex};

fn _get_sin(freq:f32, taux:f32, n:usize)->Vec<f32>{
    let mut echantillon = vec![0.0f32;n];
    let pi=std::f32::consts::PI;
    for i in 0..n {
        echantillon[i]=(((2 as f32)*pi*freq*(i as f32))/taux).sin();
    }
    echantillon
}

fn get_amplitudes(fft: &dyn RealToComplex<f32>,signal:&mut Vec<f32>)->Vec<f32>{
   
    let mut entree = fft.make_input_vec();
    let mut sortie = fft.make_output_vec();

    entree.copy_from_slice(&signal);

    fft.process(&mut entree, &mut sortie).unwrap();

    let mut output:Vec<f32>=vec![0.0;sortie.len()];

    for i in 0..sortie.len(){
        output[i]=sortie[i].norm();
    }
    output
}

fn _i_max(vec: &Vec<f32>)->usize{
    let mut max=0;
    for i in 0..vec.len(){
        if vec[i]>vec[max]{
            max=i;
        }
    }
    max
}

fn applies_hann(signal: &mut Vec<f32>){
    let n=signal.len();
    let pi =std::f32::consts::PI;
    for i in 0..n{
        let w = 0.5*(1.0-(2.0*pi*(i as f32)/((n as f32)-1.0)).cos());
        signal[i]=signal[i]*w;
    }
}
fn calcul_fk(f_max:f32,f_min:f32,n_barres:usize,k:usize)->f32{
    f_min*(f_max/f_min).powf((k as f32)/(n_barres as f32))
}
fn get_bandes(amplitudes:&Vec<f32>,taux:f32,n_barres:usize)->Vec<f32>{
    let n_fft=(amplitudes.len()-1)*2;
    let f_min:f32=50.0;
    let f_max:f32=12000.0;
    let mut sliced_amplitudes=Vec::new();
    if n_barres==0{return Vec::new();}
    for i in 0..n_barres{
        let fk_min=calcul_fk(f_max, f_min, n_barres, i);
        let fk_max=calcul_fk(f_max, f_min, n_barres, i+1);
        let mut debut=get_bin(fk_min,n_fft,taux);
        let mut fin=get_bin(fk_max,n_fft,taux);
        debut=debut.min(amplitudes.len());
        fin=fin.max(debut+1);
        fin=fin.min(amplitudes.len());
        sliced_amplitudes.push(get_max(&amplitudes[debut..fin]));
    }
    sliced_amplitudes
}
fn get_bin(freq:f32,n_nfft:usize,taux:f32)->usize{
    (freq* ((n_nfft as f32)/taux)).round() as usize
}
fn get_max(vec:&[f32])->f32{
    let mut  i_max=0;
    if vec.len()==0{
        return 0.0;
    }
    for i in 0..vec.len(){
        if vec[i_max]<vec[i]{
            i_max=i;
        }
    }
    vec[i_max]
}
fn amplitude_to_decibels(amplitude:f32,nfft:usize)->f32{
    20.0*((amplitude/((nfft as f32)/4.0)).max(1e-6).log10())
}
fn normalize(db:f32,plafond:f32,plancher:f32)->f32{
    ((db-plancher)/(plafond-plancher)).clamp(0.0, 1.0)
}
struct Lisseur {
    son: Vec<f32>,
    k:f32
}
impl Lisseur {
    fn new(n_barres:usize)->Lisseur{
        Lisseur{son: vec![0.0;n_barres], k:0.2}
    }
    fn update(&mut self, target: &Vec<f32>){
        if self.son.len()!=target.len(){return}
        for i in 0..self.son.len(){
            if target[i]>=self.son[i]{
                self.son[i]=target[i];
            }
            else{
                self.son[i]=self.son[i]+(target[i]-self.son[i])*self.k;
            }
        }
    }
    fn frac_to_percent(&self)->Vec<u32>{
        let mut percent=vec![0;self.son.len()];
        for i in 0..self.son.len(){
            percent[i]=((self.son[i]*100.0).round()) as u32;
        }
        percent
}
}
fn to_line(son:&[u32])->String{
    let mut txt=String::new();
    for i in 0..son.len(){
        if i>0 {txt.push(';');}
        txt.push_str(&son[i].to_string())
    }
    txt
}
fn analyse(signal:&mut Vec<f32>,fft: &dyn RealToComplex<f32>,taux:f32,n_barres:usize,plafond:f32,plancher:f32)->Vec<f32>{
    applies_hann(signal);
    let amplitude=get_amplitudes(fft,signal);
    let mut bandes=get_bandes(&amplitude, taux, n_barres);
    for i in 0..bandes.len(){
        bandes[i]=amplitude_to_decibels(bandes[i], signal.len());
        bandes[i]=normalize(bandes[i], plafond, plancher);
    }
    bandes
}
fn get_echantillons(tuyau:&mut ChildStdout,n:usize)->Vec<f32>{
    let mut tampon=vec![0;n*4];
    tuyau.read_exact(&mut tampon).unwrap();
    let mut echantillon=vec![0.0;n];
    let mut i:usize=0;
    for bytes in tampon.chunks_exact(4){
        echantillon[i]=f32::from_le_bytes(bytes.try_into().unwrap());
        i+=1;
    }
    echantillon
}

fn main() {
    let _freq:f32=440.0;
    let taux:f32=48000.0;
    let _device="alsa_output.pci-0000_07_00.6.analog-stereo.monitor";
    let _format="float32le";
    let n:usize=1024;
    let plafond=0.0;
    let plancher=-80.0;
    let n_barres=32;
    let mut child = Command::new("parec")
        .args(["--device=alsa_output.pci-0000_07_00.6.analog-stereo.monitor", "--format=float32le", "--rate=48000", "--channels=1"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut tuyau = child.stdout.take().unwrap();
    let mut lisseur=Lisseur::new(n_barres);
    let mut planner = RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(n);
    loop{
        let mut echantillons=get_echantillons(&mut tuyau,n);
        let barres=analyse(&mut echantillons,fft.as_ref(),taux,n_barres,plafond,plancher);
        lisseur.update(&barres);
        let barres_sizes=lisseur.frac_to_percent();
        println!("{}",to_line(&barres_sizes));
    }
    
}
