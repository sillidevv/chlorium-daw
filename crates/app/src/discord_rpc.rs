use discord_rpc_client::Client as DiscordRPC;
use discord_rpc_client::models::ActivityAssets;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const APP_ID: u64 = 1523106612762513480;

pub struct ChloriumDRPC {
	status: String,
	start_timestamp: u64,

	client: Arc<Mutex<DiscordRPC>>,
}

impl ChloriumDRPC {
	pub fn new() -> Self {
		Self {
			status: String::new(),
			start_timestamp: 0,
			client: Arc::new(Mutex::new(DiscordRPC::new(APP_ID))),
		}
	}

	pub fn initialize_drpc(&mut self) {
		self.start_timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("time went backwards")
			.as_secs();

		{
			let mut client = self.client.lock().unwrap();
			client.on_ready(|_ctx| println!("READY!"));
			client.on_error(|_ctx| eprintln!("An error occured"));
			client.start();
		}

		let client = Arc::clone(&self.client);
		let start_timestamp = self.start_timestamp;
		
		// thank you claude for this i fucking hate rust multithreading

		std::thread::spawn(move || {
			loop {
				{
					let mut client = client.lock().unwrap();

					if let Err(why) = client.set_activity(|mut a| {
						a.state("Running in debug mode (developer)")
							.timestamps(|mut ts| ts.start(start_timestamp))
							.assets(|ac| ac.large_image("chlorium-logo-1"))
					}) {
						println!("Failed to set presence: {}", why);
					} else {
						println!("Successfully set presence");
					}
				}
				std::thread::sleep(Duration::from_secs(15));
			}
		});
	}

	pub fn end_drpc(&mut self) {
		let mut client = self.client.lock().unwrap();
		client.clear_activity().expect("TODO: panic message");
	}
}
