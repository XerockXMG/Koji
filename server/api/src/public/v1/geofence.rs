use super::*;

use serde_json::json;

use model::{
    api::{
        args::{get_return_type, Args, ArgsUnwrapped, Response, ReturnTypeArg},
        ToCollection,
    },
    db::{area, geofence, instance},
    KojiDb,
};

#[get("/all")]
async fn all(conn: web::Data<KojiDb>) -> Result<HttpResponse, Error> {
    let geofences = geofence::Query::get_all(&conn.koji_db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let features: Vec<Feature> = geofences
        .into_iter()
        .map(|item| {
            let feature = Feature::from_json_value(item.area);
            let mut feature = if feature.is_ok() {
                feature.unwrap()
            } else {
                Feature::default()
            };
            feature.set_property("name", item.name);
            feature.set_property("id", item.id);
            feature
        })
        .collect();

    println!("[PUBLIC_API] Returning {} instances\n", features.len());
    Ok(HttpResponse::Ok().json(Response {
        data: Some(json!(features.to_collection(None, None))),
        message: "Success".to_string(),
        status: "ok".to_string(),
        stats: None,
        status_code: 200,
    }))
}

#[get("/area/{area_name}")]
async fn get_area(
    conn: web::Data<KojiDb>,
    area: actix_web::web::Path<String>,
) -> Result<HttpResponse, Error> {
    let area = area.into_inner();
    let feature = geofence::Query::route(&conn.koji_db, &area)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!(
        "[PUBLIC_API] Returning feature for {:?}\n",
        feature.property("name")
    );
    Ok(HttpResponse::Ok().json(Response {
        data: Some(json!(feature)),
        message: "Success".to_string(),
        status: "ok".to_string(),
        stats: None,
        status_code: 200,
    }))
}

#[post("/save-koji")]
async fn save_koji(
    conn: web::Data<KojiDb>,
    payload: web::Json<Args>,
) -> Result<HttpResponse, Error> {
    let ArgsUnwrapped { area, .. } = payload.into_inner().init(Some("geofence_save"));

    let (inserts, updates) = geofence::Query::save(&conn.koji_db, area)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Rows Updated: {}, Rows Inserted: {}", updates, inserts);

    Ok(HttpResponse::Ok().json(Response {
        data: Some(json!({ "updates": updates, "inserts": inserts })),
        message: "Success".to_string(),
        status: "ok".to_string(),
        stats: None,
        status_code: 200,
    }))
}

#[post("/save-scanner")]
async fn save_scanner(
    conn: web::Data<KojiDb>,
    scanner_type: web::Data<String>,
    payload: web::Json<Args>,
) -> Result<HttpResponse, Error> {
    let scanner_type = scanner_type.as_ref();
    let ArgsUnwrapped { area, .. } = payload.into_inner().init(Some("geofence_save"));

    let (inserts, updates) = if scanner_type == "rdm" {
        instance::Query::upsert_from_collection(&conn.data_db, area, false).await
    } else {
        area::Query::upsert_from_collection(&conn.unown_db.as_ref().unwrap(), area).await
    }
    .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Rows Updated: {}, Rows Inserted: {}", updates, inserts);

    Ok(HttpResponse::Ok().json(Response {
        data: Some(json!({ "updates": updates, "inserts": inserts })),
        message: "Success".to_string(),
        status: "ok".to_string(),
        stats: None,
        status_code: 200,
    }))
}

#[get("/{return_type}")]
async fn specific_return_type(
    conn: web::Data<KojiDb>,
    url: actix_web::web::Path<String>,
) -> Result<HttpResponse, Error> {
    let return_type = url.into_inner();
    let return_type = get_return_type(return_type, &ReturnTypeArg::FeatureCollection);

    let fc = geofence::Query::as_collection(&conn.koji_db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!(
        "[GEOFENCES_ALL] Returning {} instances\n",
        fc.features.len()
    );
    Ok(utils::response::send(fc, return_type, None, false, None))
}

#[get("/{return_type}/{project_name}")]
async fn specific_project(
    conn: web::Data<KojiDb>,
    url: actix_web::web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (return_type, project_name) = url.into_inner();
    let return_type = get_return_type(return_type, &ReturnTypeArg::FeatureCollection);
    let features = geofence::Query::by_project(&conn.koji_db, project_name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!(
        "[GEOFENCES_FC_ALL] Returning {} instances\n",
        features.len()
    );
    Ok(utils::response::send(
        features.to_collection(None, None),
        return_type,
        None,
        false,
        None,
    ))
}
