use milter::{
	on_abort, on_body, on_close, on_connect, on_data, on_eoh, on_eom, on_header, on_helo, on_mail,
	on_negotiate, on_rcpt, Milter, Context, Actions, MacroValue, ProtocolOpts, Stage, Status,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	Milter::new("inet:3015@localhost")
		.name("PostfixMilter")
		.on_header(header_callback)
		.on_negotiate(negotiate_callback)
		.on_close(close_callback)
		.run()
		.expect("milter execution failed");


	loop {}
}

#[on_negotiate(negotiate_callback)]
fn handle_negotiate(
	ctx: Context<()>,
	actions: Actions,
	_: ProtocolOpts,
) -> milter::Result<(Status, Actions, ProtocolOpts)> {
	let req_actions = Actions::ADD_HEADER | Actions::REPLACE_HEADER;

	Ok((Status::Continue, req_actions, Default::default()))
}

#[on_header(header_callback)]
fn handle_header(ctx: Context<()>, name: &str, value: &str) -> milter::Result<Status> {
	println!("HEADER");

	let actions = Actions::REPLACE_HEADER;
	ctx.api.replace_header("X-Processed-Filter-Id", 1, None);

	println!("header {}: {}", name, value);

	if name.eq("X-Processed-Filter-Id") {
		println!("HEADER OF INTEREST");

		return Ok(Status::Accept);
	}

	// ctx.api.add_header("X-Archived", "asdsadadad")?;

	// print_macros(&ctx.api)?;

	Ok(Status::Continue)
}

#[on_close(close_callback)]
fn handle_close(ctx: Context<()>) -> milter::Result<Status> {
	println!("CLOSE");

	// print_macros(&ctx.api)?;

	Ok(Status::Accept)
}


