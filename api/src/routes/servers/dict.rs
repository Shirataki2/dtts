use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictItem {
    pub order: i32,
    pub word: String,
    pub pron: String,
}

#[get("/dict")]
pub async fn get_dict(
    req: HttpRequest,
    _user: User, // for auth
    query: Query<ServerDetailQuery>,
) -> Result<HttpResponse, Error> {
    let pool = get_data::<PgPool>(&req)?;
    let dict = Dictionary::optional_get(pool, query.id).await?;
    if let Some(dict) = dict {
        let item_str = dict.dict;
        let items = serde_json::from_str::<Vec<DictItem>>(&item_str)?;
        Ok(HttpResponse::Ok().json(items))
    } else {
        let new_dict = Dictionary {
            guild_id: query.id,
            dict: "[]".to_string(),
        };
        new_dict.create(pool).await?;
        Ok(HttpResponse::Ok().json(Vec::<DictItem>::new()))
    }
}

#[post("/dict")]
pub async fn post_dict(
    req: HttpRequest,
    _user: Member, // for auth
    query: Query<ServerDetailQuery>,
    body: Json<Vec<DictItem>>,
) -> Result<HttpResponse, Error> {
    let pool = get_data::<PgPool>(&req)?;
    let dict = Dictionary::optional_get(pool, query.id).await?;
    if let Some(mut dict) = dict {
        let item_str = serde_json::to_string(&body)?;
        dict.dict = item_str;
        dict.update(pool).await?;
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(Error::not_found("Dictionary"))
    }
}
