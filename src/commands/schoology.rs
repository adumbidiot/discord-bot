use serenity::{
    client::Context,
    framework::standard::{
        Args,
        Command,
        CommandOptions,
    },
    model::channel::Message,
};
use std::sync::Arc;

use serenity::framework::standard::CommandError;

use schoology::{
    client::Client,
    realms::{
        Group,
        GroupList,
        User,
        UserList,
    },
};

pub struct SchoologyGroup {
    client: Arc<Client>,
    opts: Arc<CommandOptions>,
}

impl SchoologyGroup {
    pub fn new(client: Arc<Client>) -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Schoology Group Commands"));
        SchoologyGroup {
            client,
            opts: Arc::from(opts),
        }
    }
}

impl Command for SchoologyGroup {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        let command: String = args.single()?;

        match command.as_str() {
            "list" => {
                let start: usize = args.single()?;
                let length: usize = args.single()?;

                let response = match self.client.get_realms(start, length) {
                    Ok(groups) => format_groups(&groups),
                    Err(e) => format!("Error: {:#?}", e),
                };

                msg.channel_id.say(response)?;
            }
            "info" => {
                let id: String = args.single()?;

                let response = match self.client.get_realm(&id) {
                    Ok(group) => format_group(&group),
                    Err(e) => format!("Error: {:#?}", e),
                };

                msg.channel_id.say(response)?;
            }
            _ => {
                msg.channel_id
                    .say(format!("Command {} is not recoginized", command))?;
            }
        }

        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

pub struct SchoologyUser {
    client: Arc<Client>,
    opts: Arc<CommandOptions>,
}

impl SchoologyUser {
    pub fn new(client: Arc<Client>) -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Schoology User Commands"));
        SchoologyUser {
            client,
            opts: Arc::from(opts),
        }
    }
}

impl Command for SchoologyUser {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        let command: String = args.single()?;

        match command.as_str() {
            "list" => {
                let start: usize = args.single()?;
                let length: usize = args.single()?;

                let response = match self.client.get_realms::<UserList>(start, length) {
                    Ok(users) => format_users(&users),
                    Err(e) => format!("Error: {:#?}", e),
                };

                msg.channel_id.say(response)?;
            }
            "info" => {
                let id: String = args.single()?;

                let response = match self.client.get_realm(&id) {
                    Ok(user) => format_user(&user),
                    Err(e) => format!("Error: {:#?}", e),
                };

                msg.channel_id.say(response)?;
            }
            _ => {
                msg.channel_id
                    .say(format!("Command {} is not recoginized", command))?;
            }
        }

        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}
// pub struct SchoologyUpdate {
// client: Arc<Client>,
// options: Arc<CommandOptions>,
// }
// impl SchoologyUpdate {
// pub fn new(client: Arc<Client>) -> Self {
// let mut options = CommandOptions::default();
// SchoologyUpdate {
// client,
// options: Arc::from(options),
// }
// }
// }
//
// impl Command for SchoologyUpdate {
// fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
// let command: String = args.single()?;
//
// match command.as_str() {
// "list" => {
// let start: usize = args.single()?;
// let length: usize = args.single()?;
//
// let response = match self.client.get_realm_objects(start, length) {
// Ok(users) => format_updates(&users),
// Err(e) => format!("Error: {:#?}", e),
// };
//
// msg.channel_id.say(response)?;
// }
// "info" => {
// let id: String = args.single()?;
//
// let response = match self.client.get_realm(&id) {
// Ok(user) => format_user(&user),
// Err(e) => format!("Error: {:#?}", e),
// };
//
// msg.channel_id.say(response)?;
// }
// _ => {
// msg.channel_id
// .say(format!("Command {} is not recoginized", command))?;
// }
// }

// Ok(())
// }
// }
fn format_groups(groups: &GroupList) -> String {
    let mut ret = String::from("__**Groups:**__\n\n");
    for group in &groups.group {
        ret += &format!("__***{}***__\n", &group.title);
        ret += &format!("\t__ID:__ {}\n", &group.id);
        if group.description.len() > 0 {
            ret += &format!("\t__Description:__ {}\n", &group.description);
        }
        ret += "\n";
    }
    ret
}

fn format_group(group: &Group) -> String {
    let mut ret = format!("__***{}***__\n", &group.title);
    ret += &format!("\t__ID:__ {}\n", &group.id);
    if group.description.len() > 0 {
        ret += &format!("\t__Description:__ {}\n", &group.description);
    }
    ret
}

fn format_users(users: &UserList) -> String {
    let mut ret = String::from("__**Users**__\n\n");
    for user in &users.user {
        ret += &format!(
            "__***{} {} {} {}***__\n",
            &user.name_title,
            &user.name_first,
            &user.name_middle.as_ref().unwrap_or(&String::new()),
            &user.name_last
        );

        ret += &format!("\t__ID:__ {}\n", &user.id);
        if let Some(ref gender) = user.gender {
            ret += &format!("\t__Gender:__ {}\n", &gender);
        }

        if let Some(ref position) = user.position {
            ret += &format!("\t__Position:__ {}\n", &position);
        }

        ret += "\n";
    }
    ret
}

fn format_user(user: &User) -> String {
    let mut ret = format!(
        "__***{} {} {} {}***__\n",
        &user.name_title,
        &user.name_first,
        &user.name_middle.as_ref().map(|x| x.as_str()).unwrap_or(""),
        &user.name_last
    );

    ret += &format!("\t__ID:__ {}\n", &user.id);
    if let Some(ref gender) = user.gender {
        ret += &format!("\t__Gender:__ {}\n", &gender);
    }

    if let Some(ref position) = user.position {
        ret += &format!("\t__Position:__ {}\n", &position);
    }

    if let Some(ref primary_email) = user.primary_email {
        ret += &format!("\t__Email:__ {}\n", &primary_email);
    }
    ret
}
// fn format_updates(updates: &UpdateList) -> String {
// let mut ret = String::from("__**Updates**__\n\n");
//
// for update in &updates.update {
// ret += &format!("__***Update***__\n");
// ret += &format!("\t__ID:__ {}\n", &update.id);
// ret += &format!("\t__Author:__ {}\n", &update.uid);
// ret += &format!(
// "\t__Date:__ {}\n",
// chrono::Local
// .timestamp(update.created as i64, 0)
// .to_rfc2822()
// );
// ret += &format!("\t__Likes:__ {}\n", &update.likes);
// ret += &format!("\t__Comments:__ {}\n", &update.num_comments);
// ret += &format!("\t__Body:__ {}\n", &update.body);
// ret += "\n";
// }
//
// return ret;
// }
