use serde::{Deserialize, Serialize};
use std::fs;

mod handle_channel;
use crate::handle_channel::SlackApp;

#[derive(Serialize, Deserialize, Debug)]
struct Env {
    token: String,
    id: String,
    member: Vec<String>,
    ignore_member: Vec<String>,
}

impl Env {
    fn new() -> Self {
        let file = fs::read_to_string("channel_info.json").expect("Failed to load channel.json");
        let env: Env = serde_json::from_str(&file).unwrap();
        env
    }
}

fn read_latest_member_id() -> std::io::Result<Vec<String>> {
    let v = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("./member_log.csv")?
        .records()
        .last()
        .unwrap()
        .unwrap();
    Ok(vec![v.get(1).unwrap().to_string(), v.get(2).unwrap().to_string()])
}

fn select_user(members: &Vec<String>) -> Vec<String> {
    use itertools::Itertools;
    use rand::seq::SliceRandom;
    
    // Generate and Shuffle member permulation
    let mut pattern: Vec<Vec<&String>> = members.iter().permutations(2).collect();
    let mut rng = rand::thread_rng();
    pattern.shuffle(&mut rng);
    vec![pattern[0][0].to_owned(), pattern[0][1].to_owned()]
}

fn make_message(members: &Vec<String>) -> String {
    let jp = "今週のゴミ掃除担当です．\n作業が終了したら，この投稿に:done:をつけてください.";
    let en = "You are in charge of the garbage disposal of this week.\nWhen you're done, please reaction :done: to this post.";

    format!(
        "<@{}> <@{}>\n{}\n\n{}",
        members[0], members[1], jp, en
    )
}

fn main() {
    // Load environment variable
    let env = Env::new();

    // Make SlackApp instance
    let bot = SlackApp::new(env.token, env.id);
    
    // Select this week cleaner from Channel member (except "ignore_member" and "latest_member").
    let latest_member_id = read_latest_member_id().unwrap();
    let target: Vec<String> = env.member
        .into_iter()
        .filter(|x| !env.ignore_member.contains(x) && !latest_member_id.contains(x))
        .collect();
    let cleaner = select_user(&target);
    
    // Send message
    let message = make_message(&cleaner);
    bot.post_message(&message).unwrap();
    
    // Record this week cleaner to member_log.csv
    bot.record_cleaner(cleaner).unwrap();
    ()
}
