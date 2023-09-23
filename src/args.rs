use clap::{Args, Parser, Subcommand};

/// Simple porgram to change some moodle stuff.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
  /// list or hide/show courses
  #[clap(subcommand)]
  pub entity: EntityType,

  /// Username to log in with
  #[arg(short, long)]
  pub username: String,

  /// Your password
  #[arg(short, long)]
  pub password: String,

  ///Moodle url
  #[arg(long)]
  pub url: String,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
  /// Do actions on categories
  Category(EntityArgs),
  /// Do actions on courses
  Course(EntityArgs),
}

#[derive(Debug, Args)]
pub struct EntityArgs {
  #[clap(subcommand)]
  pub function: FunType,
}

#[derive(Debug, Subcommand)]
pub enum FunType {
  /// List all courses
  List,
  /// Show the entity
  Show(VisibilityCommand),
  /// Hide the entity
  Hide(VisibilityCommand),
}

#[derive(Debug, Args)]
pub struct VisibilityCommand {
  /// The id of the entity found with the list command
  pub id: String,
}
