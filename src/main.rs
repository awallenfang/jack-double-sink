use std::io;
use std::sync::mpsc::channel;
const OUT_PORTS: u32 = 2;

fn main() {
    let (tx, rx) = channel::<(f32,f32)>();
    // 1. Create client
    let (client_in, _status) =
    jack::Client::new("sink_multiplexer", jack::ClientOptions::default()).unwrap();

    let (client_out, _status) =
    jack::Client::new("faucet_multiplexer", jack::ClientOptions::default()).unwrap();

    let in_l = client_in
    .register_port("Input L", jack::AudioIn::default())
    .unwrap();

    let in_r  = client_in
    .register_port("Input R", jack::AudioIn::default())
    .unwrap();
    println!("{:?}", in_r.name());


    let mut out_ports: Vec::<(jack::Port<jack::AudioOut>,jack::Port<jack::AudioOut>)> = Vec::new();

    for i in 0..OUT_PORTS {
        let out_l = client_out
        .register_port(format!("Output {i} L").as_str(), jack::AudioOut::default())
        .unwrap();

        let out_r = client_out
        .register_port(format!("Output {i} R").as_str(), jack::AudioOut::default())
        .unwrap();

        out_ports.push((out_l, out_r));
    }

    let process_in_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let in_buf_l = in_l.as_slice(ps);
        let in_buf_r = in_r.as_slice(ps);

        for i in 0..in_buf_l.len() {
            let _ = tx.send((in_buf_l[i], in_buf_r[i]));
        }
    
        
        jack::Control::Continue
    };
    let process_in = jack::contrib::ClosureProcessHandler::new(process_in_callback);
    
    let process_out_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        // let in_buf_l = in_l.as_slice(ps);
        // let in_buf_r = in_r.as_slice(ps);

        // for i in 0..in_buf_l.len() {
        //     tx.send((in_buf_l[i], in_buf_r[i]));
        // }
        
        let (l,r): (f32,f32) = rx.recv().unwrap();
        let l_slice: &[f32] = &[l];
        let r_slice: &[f32] = &[r];
        for i in 0..OUT_PORTS {
            out_ports[i as usize].0.as_mut_slice(ps).clone_from_slice(l_slice);
            out_ports[i as usize].1.as_mut_slice(ps).clone_from_slice(r_slice)
        }
        
        jack::Control::Continue
    };
    let process_out = jack::contrib::ClosureProcessHandler::new(process_out_callback);
    
    
    // 3. Activate the client, which starts the processing.
    let _active_client_in = client_in.activate_async((), process_in).unwrap();
    let _active_client_out = client_out.activate_async((), process_out).unwrap();


    // 4. Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();
    
}
