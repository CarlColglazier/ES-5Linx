extern crate jack;

use std::thread;
use util::bits_to_float;

fn main() {
		let (client, _) = jack::Client::new(
				"es-5",
				jack::ClientOptions::NO_START_SERVER,
		).unwrap();

		let ins: [jack::Port<jack::AudioIn>; 8] = [
				client.register_port("in_1", jack::AudioIn::default()).unwrap(),
				client.register_port("in_2", jack::AudioIn::default()).unwrap(),
				client.register_port("in_3", jack::AudioIn::default()).unwrap(),
				client.register_port("in_4", jack::AudioIn::default()).unwrap(),
				client.register_port("in_5", jack::AudioIn::default()).unwrap(),
				client.register_port("in_6", jack::AudioIn::default()).unwrap(),
				client.register_port("in_7", jack::AudioIn::default()).unwrap(),
				client.register_port("in_8", jack::AudioIn::default()).unwrap(),
		];

		let mut out_port = client
				.register_port("out", jack::AudioOut::default())
				.unwrap();

		let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
				let out = out_port.as_mut_slice(ps);
				for i in 0..out.len() {
						let mut value: i32 = 0;
						for in_index in 0..ins.len() {
								let insounds = ins[in_index].as_slice(ps)[i];
								if insounds > 0.0 {
										value += 1 << in_index;
								}
						}
						out[i] = bits_to_float(value << 16);
				}
				// Continue
				return jack::Control::Continue;
		};
		let process = jack::ClosureProcessHandler::new(process_callback);

		let _active_client = client.activate_async(Notifications, process).unwrap();

		loop {
				thread::yield_now();
		}

		//active_client.deactivate().unwrap();
}


struct Notifications;

impl jack::NotificationHandler for Notifications {
    fn thread_init(&self, _: &jack::Client) {
        println!("JACK: thread init");
    }

    fn shutdown(&mut self, status: jack::ClientStatus, reason: &str) {
        println!(
            "JACK: shutdown with status {:?} because \"{}\"",
            status, reason
        );
    }

    fn freewheel(&mut self, _: &jack::Client, is_enabled: bool) {
        println!(
            "JACK: freewheel mode is {}",
            if is_enabled { "on" } else { "off" }
        );
    }

    fn buffer_size(&mut self, _: &jack::Client, sz: jack::Frames) -> jack::Control {
        println!("JACK: buffer size changed to {}", sz);
        jack::Control::Continue
    }

    fn sample_rate(&mut self, _: &jack::Client, srate: jack::Frames) -> jack::Control {
        println!("JACK: sample rate changed to {}", srate);
        jack::Control::Continue
    }

    fn client_registration(&mut self, _: &jack::Client, name: &str, is_reg: bool) {
        println!(
            "JACK: {} client with name \"{}\"",
            if is_reg { "registered" } else { "unregistered" },
            name
        );
    }

    fn port_registration(&mut self, _: &jack::Client, port_id: jack::PortId, is_reg: bool) {
        println!(
            "JACK: {} port with id {}",
            if is_reg { "registered" } else { "unregistered" },
            port_id
        );
    }

    fn port_rename(
        &mut self,
        _: &jack::Client,
        port_id: jack::PortId,
        old_name: &str,
        new_name: &str,
    ) -> jack::Control {
        println!(
            "JACK: port with id {} renamed from {} to {}",
            port_id, old_name, new_name
        );
        jack::Control::Continue
    }

    fn ports_connected(
        &mut self,
        _: &jack::Client,
        port_id_a: jack::PortId,
        port_id_b: jack::PortId,
        are_connected: bool,
    ) {
        println!(
            "JACK: ports with id {} and {} are {}",
            port_id_a,
            port_id_b,
            if are_connected {
                "connected"
            } else {
                "disconnected"
            }
        );
    }

    fn graph_reorder(&mut self, _: &jack::Client) -> jack::Control {
        println!("JACK: graph reordered");
        jack::Control::Continue
    }

    fn xrun(&mut self, _: &jack::Client) -> jack::Control {
        println!("JACK: xrun occurred");
        jack::Control::Continue
    }

    fn latency(&mut self, _: &jack::Client, mode: jack::LatencyType) {
        println!(
            "JACK: {} latency has changed",
            match mode {
                jack::LatencyType::Capture => "capture",
                jack::LatencyType::Playback => "playback",
            }
        );
    }
}
