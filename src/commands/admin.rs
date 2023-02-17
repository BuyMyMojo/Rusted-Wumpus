use rusted_wumpus_lib::checks::is_admin;

use crate::{Context, Error};

/// Register application commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(
    prefix_command,
    slash_command,
    hide_in_help,
    owners_only,
    category = "Admin",
    check = "is_admin"
)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::samples::register_application_commands_buttons(ctx).await?;

    Ok(())
}
