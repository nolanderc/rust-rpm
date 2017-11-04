
use clap::{App, Arg, ArgMatches};





pub fn get_matches<'a>() -> ArgMatches<'a> {
	App::new("Project Manager")
		.version("0.0.1")
		.author("Christofer N. <christofer.nolander@gmail.com>")
		.about("Manages various workspaces and projects")
		
		.subcommand(new_item_command())
		.subcommand(remove_item_command())
		
		.subcommand(switch_workspace_command())
		.subcommand(display_current_workspace_command())
		
		.subcommand(open_project())
		
		.get_matches()
}


fn new_item_command<'a>() -> App<'a, 'a> {
	App::new("new")
		.about("Creates a new item ([workspace] or [project])")
		.arg(Arg::with_name("type")
			.help("Type of item to create ([workspace] or [project])")
			.required(true)
			)
		.arg(Arg::with_name("name")
			.help("The name of the new item")
			.required(true))
		.arg(Arg::with_name("path")
			.help("[Optional] The path of the new item")
			.short("d")
			.long("directory")
			.takes_value(true)
			.value_name("path")
			.required(false))
}


fn switch_workspace_command<'a>() -> App<'a, 'a> {
	App::new("switch")
		.about("Changes the active workspace")
		.arg(Arg::with_name("name")
			.help("The name of the workspace")
			.required(true)
			)
}

fn display_current_workspace_command<'a>() -> App<'a, 'a> {
	App::new("current")
		.about("Displays the name of the active workspace")
}


fn remove_item_command<'a>() -> App<'a, 'a> {
	App::new("remove")
		.about("Removes an item ([workspace] or [project])")
		.arg(Arg::with_name("type")
			.help("Type of item to remove ([workspace] or [project])")
			.required(true)
			)
		.arg(Arg::with_name("name")
			.help("The name of the item")
			.required(true)
			)
		.arg(Arg::with_name("purge")
			.short("p")
			.long("purge")
			.help("Erases the item from disk completely")
			)
}


fn open_project<'a>() -> App<'a, 'a> {
	App::new("open")
		.about("Opens a project in the file explorer")
		.arg(Arg::with_name("name")
			.help("The name of the project")
			.required(true)
			)
}