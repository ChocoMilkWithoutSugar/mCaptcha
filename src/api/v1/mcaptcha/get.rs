/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

use super::create::MCaptchaDetails;
use crate::errors::*;
use crate::AppData;

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.get",
    wrap = "crate::api::v1::get_middleware()"
)]
pub async fn get_captcha(
    payload: web::Json<MCaptchaDetails>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let levels = runner::get_captcha(&payload.key, &username, &data).await?;
    Ok(HttpResponse::Ok().json(levels))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Levels {
    levels: I32Levels,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct I32Levels {
    pub difficulty_factor: i32,
    pub visitor_threshold: i32,
}

pub mod runner {
    use super::*;

    // TODO get metadata from mcaptcha_config table
    pub async fn get_captcha(
        key: &str,
        username: &str,
        data: &AppData,
    ) -> ServiceResult<Vec<I32Levels>> {
        let levels = sqlx::query_as!(
            I32Levels,
            "SELECT difficulty_factor, visitor_threshold FROM mcaptcha_levels  WHERE
            config_id = (
                SELECT config_id FROM mcaptcha_config WHERE key = ($1)
                AND user_id = (SELECT ID from mcaptcha_users WHERE name = $2)
                )
            ORDER BY difficulty_factor ASC;",
            key,
            &username
        )
        .fetch_all(&data.db)
        .await?;

        Ok(levels)
    }
}
