use dfhack_remote::CoreRunCommandRequest;

fn main() {
    env_logger::init();
    let mut client = dfhack_remote::connect().unwrap();
    let mut command = CoreRunCommandRequest::new();
    command.set_command("lua".to_string());
    command.arguments.push("print(dfhack.df2utf(dfhack.TranslateName(df.global.world.world_data.active_site[0].name)))".to_string());
    let reply = client.core().run_command(command).unwrap();

    let mut command = CoreRunCommandRequest::new();
    command.set_command("lua".to_string());
    command.arguments.push("print(dfhack.df2utf(dfhack.TranslateName(df.global.world.world_data.active_site[0].name, true)))".to_string());
    let reply_en = client.core().run_command(command).unwrap();

    println!(
        "The fortress is named {} ({})",
        reply.fragments[0].text().trim(),
        reply_en.fragments[0].text().trim()
    );
}
