use moodle::api::core::course::{get_categories, get_enrolled_courses_by_timeline_classification};
use moodle::client::MoodleClient;

pub async fn get_enrolled_courses<'a>(
    client: &'a mut MoodleClient,
) -> Result<Vec<get_enrolled_courses_by_timeline_classification::ReturnsCoursesItem>, ()> {
    // Get course list
    let result = get_enrolled_courses_by_timeline_classification::call(
        client,
        &mut get_enrolled_courses_by_timeline_classification::Params {
            classification: Some("all".to_string()),
            limit: None,
            offset: None,
            sort: None,
            customfieldname: None,
            customfieldvalue: None,
            searchvalue: None,
        },
    )
    .await;

    let returns = result.unwrap();
    let mut courses = returns.courses.unwrap();
    courses.sort_by_key(|course| course.id);

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
