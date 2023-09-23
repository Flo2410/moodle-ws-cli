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
