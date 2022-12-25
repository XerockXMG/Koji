use models::api::Response;

use super::*;

use crate::{
    models::{
        api::{Args, ArgsUnwrapped, BoundsArg},
        KojiDb,
    },
    queries::{area, gym, instance, pokestop, spawnpoint},
};

#[post("/all/{category}")]
async fn all(
    conn: web::Data<KojiDb>,
    scanner_type: web::Data<String>,
    category: actix_web::web::Path<String>,
    payload: web::Json<Args>,
) -> Result<HttpResponse, Error> {
    let ArgsUnwrapped { last_seen, .. } = payload.into_inner().init(Some("all_data"));
    let category = category.into_inner();
    let scanner_type = scanner_type.as_ref();

    println!(
        "\n[DATA_ALL] Scanner Type: {} | Category: {}",
        scanner_type, category
    );

    let all_data = match category.as_str() {
        "gym" => gym::all(&conn.data_db, last_seen).await,
        "pokestop" => pokestop::all(&conn.data_db, last_seen).await,
        "spawnpoint" => spawnpoint::all(&conn.data_db, last_seen).await,
        _ => Err(DbErr::Custom("invalid_category".to_string())),
    }
    .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("[DATA-ALL] Returning {} {}s\n", all_data.len(), category);
    Ok(HttpResponse::Ok().json(all_data))
}

#[post("/bound/{category}")]
async fn bound(
    conn: web::Data<KojiDb>,
    scanner_type: web::Data<String>,
    category: actix_web::web::Path<String>,
    payload: web::Json<BoundsArg>,
) -> Result<HttpResponse, Error> {
    let scanner_type = scanner_type.as_ref();
    let category = category.into_inner();

    println!(
        "\n[DATA_BOUND] Scanner Type: {} | Category: {}",
        scanner_type, category
    );

    let bound_data = match category.as_str() {
        "gym" => gym::bound(&conn.data_db, &payload, 0).await,
        "pokestop" => pokestop::bound(&conn.data_db, &payload, 0).await,
        "spawnpoint" => spawnpoint::bound(&conn.data_db, &payload, 0).await,
        _ => Err(DbErr::Custom("invalid_category".to_string())),
    }
    .map_err(actix_web::error::ErrorInternalServerError)?;

    println!(
        "[DATA-BOUND] Returning {} {}s\n",
        bound_data.len(),
        category
    );
    Ok(HttpResponse::Ok().json(bound_data))
}

#[post("/area/{category}")]
async fn by_area(
    conn: web::Data<KojiDb>,
    scanner_type: web::Data<String>,
    category: actix_web::web::Path<String>,
    payload: web::Json<Args>,
) -> Result<HttpResponse, Error> {
    let scanner_type = scanner_type.as_ref();
    let category = category.into_inner();

    let ArgsUnwrapped {
        area,
        instance,
        last_seen,
        ..
    } = payload.into_inner().init(None);

    println!(
        "\n[DATA_AREA] Scanner Type: {} | Category: {}",
        scanner_type, category
    );

    if area.features.is_empty() && instance.is_empty() {
        return Ok(
            HttpResponse::BadRequest().json(Response::send_error("no_area_and_empty_instance"))
        );
    }

    let area = if !area.features.is_empty() && !instance.is_empty() {
        if scanner_type == "rdm" {
            instance::route(&conn.data_db, &instance).await
        } else {
            area::route(&conn.unown_db.as_ref().unwrap(), &instance).await
        }
    } else {
        Ok(area)
    }
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let area_data = if category == "gym" {
        gym::area(&conn.data_db, &area, last_seen).await
    } else if category == "pokestop" {
        pokestop::area(&conn.data_db, &area, last_seen).await
    } else {
        spawnpoint::area(&conn.data_db, &area, last_seen).await
    }
    .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("[DATA-AREA] Returning {} {}s\n", area_data.len(), category);
    Ok(HttpResponse::Ok().json(area_data))
}

#[post("/area_stats/{category}")]
async fn area_stats(
    conn: web::Data<KojiDb>,
    scanner_type: web::Data<String>,
    category: actix_web::web::Path<String>,
    payload: web::Json<Args>,
) -> Result<HttpResponse, Error> {
    let scanner_type = scanner_type.as_ref();
    let category = category.into_inner();

    let ArgsUnwrapped {
        area,
        instance,
        last_seen,
        ..
    } = payload.into_inner().init(None);

    println!(
        "\n[DATA_AREA] Scanner Type: {} | Category: {}",
        scanner_type, category
    );

    if area.features.is_empty() && instance.is_empty() {
        return Ok(
            HttpResponse::BadRequest().json(Response::send_error("no_area_and_empty_instance"))
        );
    }

    let area = if !area.features.is_empty() && !instance.is_empty() {
        if scanner_type == "rdm" {
            instance::route(&conn.data_db, &instance).await
        } else {
            area::route(&conn.unown_db.as_ref().unwrap(), &instance).await
        }
    } else {
        Ok(area)
    }
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let area_data = if category == "gym" {
        gym::stats(&conn.data_db, &area, last_seen).await
    } else if category == "pokestop" {
        pokestop::stats(&conn.data_db, &area, last_seen).await
    } else {
        spawnpoint::stats(&conn.data_db, &area, last_seen).await
    }
    .map_err(actix_web::error::ErrorInternalServerError)?;

    println!(
        "[DATA-AREA] Returning {} Total: {}\n",
        category, area_data.total
    );
    Ok(HttpResponse::Ok().json(area_data))
}