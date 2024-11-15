use std::io;

const OUT_PORTS: u32 = 2;

const DEST_L: &str = "";
const DEST_R: &str = "";

fn main() {
    // 1. Create client
    let (client, _status) =
    jack::Client::new("sink_multiplexer", jack::ClientOptions::default()).unwrap();
    let in_l = client
    .register_port("Input L", jack::AudioIn::default())
    .unwrap();

    let in_r = client
    .register_port("Input R", jack::AudioIn::default())
    .unwrap();
    println!("{:?}", in_r.name());


    let mut out_ports: Vec::<(jack::Port<jack::AudioOut>,jack::Port<jack::AudioOut>)> = Vec::new();

    for i in 0..OUT_PORTS {
        let out_l = client
        .register_port(format!("Output L {i}").as_str(), jack::AudioOut::default())
        .unwrap();

        let out_r = client
        .register_port(format!("Output R {i}").as_str(), jack::AudioOut::default())
        .unwrap();

        out_ports.push((out_l, out_r));
    }

    let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let in_buf_l = in_l.as_slice(ps);
        let in_buf_r = in_r.as_slice(ps);
        
        for i in 0..OUT_PORTS {
            out_ports[i as usize].0.as_mut_slice(ps).clone_from_slice(in_buf_l);
            out_ports[i as usize].1.as_mut_slice(ps).clone_from_slice(in_buf_r)
        }
        
        jack::Control::Continue
    };
    let process = jack::contrib::ClosureProcessHandler::new(process_callback);
    
    
    
    // 3. Activate the client, which starts the processing.
    let active_client = client.activate_async((), process).unwrap();

    // 4. Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();
    
}
