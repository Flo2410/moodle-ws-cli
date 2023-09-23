use std::collections::HashMap;

use moodle::api::core::course::{get_categories, get_enrolled_courses_by_timeline_classification};
use moodle::client::MoodleClient;

pub async fn get_enrolled_courses<'a>(
  client: &'a mut MoodleClient,
) -> anyhow::Result<Vec<get_enrolled_courses_by_timeline_classification::ReturnsCoursesItem>> {
  // Get shown courses
  let res_all = get_enrolled_courses_by_timeline_classification::call(
    client,
    &mut get_enrolled_courses_by_timeline_classification::Params {
      classification: Some("all".to_string()),
      limit: None,
      offset: None,
      sort: Some(String::from("id")),
      customfieldname: None,
      customfieldvalue: None,
      searchvalue: None,
    },
  )
  .await
  .unwrap();

  // Get hidden courses
  let res_hidden = get_enrolled_courses_by_timeline_classification::call(
    client,
    &mut get_enrolled_courses_by_timeline_classification::Params {
      classification: Some("hidden".to_string()),
      limit: None,
      offset: None,
      sort: Some(String::from("id")),
      customfieldname: None,
      customfieldvalue: None,
      searchvalue: None,
    },
  )
  .await
  .unwrap();

  let mut courses = res_hidden.courses.unwrap();
  courses.append(&mut res_all.courses.unwrap());

  return Ok(courses);
}

pub async fn get_categories<'a>(client: &'a mut MoodleClient) -> anyhow::Result<get_categories::Returns> {
  // Get the group list
  get_categories::call(
    client,
    &mut get_categories::Params {
      addsubcategories: None,
      criteria: None,
    },
  )
  .await
}

pub async fn set_course_visibility<'a>(
  client: &'a mut MoodleClient,
  course_id: String,
  visibility: bool,
) -> Result<(), ()> {
  let mut params: HashMap<String, String> = HashMap::new();
  params.insert(
    "preferences[0][type]".to_string(),
    format!("block_myoverview_hidden_course_{}", course_id),
  );

  let is_hidden;
  match visibility {
    true => is_hidden = "0",
    false => is_hidden = "1",
  }

  params.insert("preferences[0][value]".to_string(), is_hidden.to_string());

  let res = client.post("core_user_update_user_preferences", &params).await;

  if res.is_ok() {
    return Ok(());
  } else {
    return Err(());
  }
}
