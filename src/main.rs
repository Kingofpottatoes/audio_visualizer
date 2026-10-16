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

fn main() {
    let freq:f32=440.0;
    let taux:f32=44100.0;
    let n:usize=2048;
    
    let mut signal=get_sin(freq, taux, n);
    let signal2=get_sin(freq*2.0,taux,n);
    for i in 0..signal.len(){
        signal[i]=signal[i]+signal2[i];
    }
    applies_hann(&mut signal);
    let test=get_amplitudes(&signal);
    println!("{}",i_max(&test));
    
    for i in 15..45{
        println!("bin {} : {:.1}", i, test[i]);
    }
}
