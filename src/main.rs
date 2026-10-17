
use realfft::RealFftPlanner;

fn get_sin(freq:f32, taux:f32, n:usize)->Vec<f32>{
    let mut echantillon = vec![0.0f32;n];
    let pi=std::f32::consts::PI;
    for i in 0..n {
        echantillon[i]=(((2 as f32)*pi*freq*(i as f32))/taux).sin();
    }
    echantillon
}

fn get_amplitudes(signal:&Vec<f32>)->Vec<f32>{
    let mut planner = RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(signal.len());
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

fn i_max(vec: &Vec<f32>)->usize{
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
fn main() {
    let freq:f32=440.0;
    let taux:f32=44100.0;
    let n:usize=2048;
    let plafond=0.0;
    let plancher=-80.0;
    
    let mut signal=get_sin(freq, taux, n);
    let signal2=get_sin(freq*2.0,taux,n);
    for i in 0..signal.len(){
        signal[i]=signal[i]+signal2[i];
    }
    applies_hann(&mut signal);
    let test=get_amplitudes(&signal);
    println!("{}",i_max(&test));
    let mut bandes=get_bandes(&test, taux, 32);
    for i in 0..32{
        bandes[i]=normalize(amplitude_to_decibels(bandes[i], n), plafond, plancher);
        println!("{}",bandes[i]);
    }
}
