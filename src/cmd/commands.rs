use crate::cmd::add::AddParams;
use crate::cmd::remove::RemoveParams;
use crate::cmd::select::SelectParams;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[clap(visible_aliases = ["a"], about = "Add a XRay server by URL")]
    Add(AddParams),

    #[clap(visible_aliases = ["d", "r", "remove"], about = "Delete a XRay server by URL")]
    Delete(RemoveParams),

    #[clap(visible_aliases = ["s", "choose"], about = "Select server")]
    Select(SelectParams),
}
