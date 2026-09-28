use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait, QueryOrder,
};

use crate::wallet::model::{ActiveModel, Column, Entity, Model};

pub async fn list(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find()
        .order_by_desc(Column::CreatedAt)
        .all(db)
        .await
}

pub async fn create(db: &DatabaseConnection, name: String) -> Result<Model, DbErr> {
    let wallet = ActiveModel {
        name: Set(name),
        ..Default::default()
    };

    let res = wallet.insert(db).await;
    return res;
}
