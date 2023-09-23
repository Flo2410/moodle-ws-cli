mod args;
mod moodle_helpers;

use std::collections::HashMap;

use moodle::client::{login, MoodleClient};

use clap::Parser;

use args::{CliArgs, EntityType, FunType};
use moodle_helpers::*;

#[tokio::main]
async fn main() {
  let args = CliArgs::parse();

  // Login to moodle
  let token = login(&args.url, &args.username, &args.password).await.unwrap();
  let mut client = MoodleClient::new(&args.url, &token);

  match args.entity {
    EntityType::Course(course) => match course.function {
      FunType::List => {
        let courses = get_enrolled_courses(&mut client).await.unwrap();

        // Print list
        for course in courses {
          println!(
            "{} // {} // {} // {}",
            course.id.unwrap(),
            course.fullnamedisplay.unwrap(),
            course.coursecategory.unwrap(),
            course.hidden.unwrap(),
          );
        }
      }

      FunType::Hide(visibility_command) => {
        println!("Hide course: {}", visibility_command.id);
        let res = set_course_visibility(&mut client, visibility_command.id, false).await;

        if res.is_err() {
          println!("Error!");
        }
      }

      FunType::Show(visibility_command) => {
        println!("Show course: {}", visibility_command.id);
        let res = set_course_visibility(&mut client, visibility_command.id, true).await;

        if res.is_err() {
          println!("Error!");
        }
      }
    },

    EntityType::Category(category) => match category.function {
      FunType::List => {
        // Get the course list
        let courses = get_enrolled_courses(&mut client).await.unwrap();

        // Get the categories list
        let categories = get_categories(&mut client).await.unwrap();

        let mut found_categories: HashMap<i64, String> = HashMap::new();

        // Find categories which contain an enroled course
        for course in courses {
          let category = categories.iter().find(|cat| cat.name == course.coursecategory).unwrap();
          if !found_categories.contains_key(category.id.as_ref().unwrap()) {
            found_categories.insert(category.id.unwrap(), category.name.clone().unwrap());
          }
        }

        // Print categories
        found_categories
          .iter()
          .for_each(|found_category| println!("{} // {}", found_category.0, found_category.1));
      }

      FunType::Hide(visibility_command) => {
        println!("Hide category: {}", visibility_command.id);

        let courses = get_enrolled_courses(&mut client).await.unwrap();
        let category = get_category_by_id(&mut client, visibility_command.id).await.unwrap();
        let category_name = category.first().unwrap().name.as_ref().unwrap();

        for course in courses {
          if course.coursecategory.unwrap().eq(category_name) {
            let res = set_course_visibility(&mut client, course.id.unwrap().to_string(), false).await;
            if res.is_err() {
              println!("Error!");
            }
          }
        }
      }

      FunType::Show(visibility_command) => {
        println!("Show cagtegory: {}", visibility_command.id);

        let courses = get_enrolled_courses(&mut client).await.unwrap();
        let category = get_category_by_id(&mut client, visibility_command.id).await.unwrap();
        let category_name = category.first().unwrap().name.as_ref().unwrap();

        for course in courses {
          if course.coursecategory.unwrap().eq(category_name) {
            let res = set_course_visibility(&mut client, course.id.unwrap().to_string(), true).await;
            if res.is_err() {
              println!("Error!");
            }
          }
        }
      }
    },
  }
}
