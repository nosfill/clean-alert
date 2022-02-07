use chrono::{DateTime, Local};
use std::error::Error;
use std::fs::OpenOptions;


#[derive(Debug)]
pub struct SlackApp {
    token: String,
    channel: String,
}

impl SlackApp {
    pub fn new(token: String, channel: String) -> Self {
        SlackApp {
            token: token,
            channel: channel    
        }
    }

    // pub fn get_channel_member_id(&self) -> Result<Vec<String>, ureq::Error> {
    //     let url = "https://slack.com/api/conversations.members";

    //     let resp = ureq::get(&url)
    //         .set("Authorization", &format!("Bearer {}", &self.token))
    //         .set("Content-Type", "application/x-www-form-urlencoded")
    //         .query("channel", &self.channel)
    //         .call()?
    //         .into_json::<serde_json::Value>()?;

    //     Ok(resp["members"]
    //         .as_array()
    //         .unwrap()
    //         .iter()
    //         .map(|x| x.as_str().unwrap().to_string())
    //         .collect())
    // }

    pub fn post_message(&self, text: &String) -> Result<ureq::Response, ureq::Error> {
        let url = "https://slack.com/api/chat.postMessage";

        Ok(ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", &self.token))
            .set("Content-Type", "application/json")
            .send_json(
                ureq::json!({ "channel": &self.channel, "text": text, "link_names": "true"}),
            )?)
    }

    pub fn record_cleaner(&self, members: Vec<String>) -> Result<(), Box<dyn Error>> {
        let dt: DateTime<Local> = Local::now();
        let mut record: Vec<String> = vec![dt.format("%F").to_string()];
        record.append(&mut members.clone());

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("./member_log.csv")?;
        let mut wtr = csv::Writer::from_writer(file);
        wtr.write_record(record)?;
        Ok(())
    }
}