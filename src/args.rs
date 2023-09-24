use clap::{Args, Parser, Subcommand};

/// A simple CLI for moodle written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
  /// list or hide/show courses
  #[clap(subcommand)]
  pub entity: EntityType,

  /// Your moodle username
  #[arg(short, long)]
  pub username: String,

  /// Your moodle password
  #[arg(short, long)]
  pub password: String,

  ///Moodle url (e.g. https://moodle.example.com)
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
  /// List all entries of the entity
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
