use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::models::category::Category;
use crate::schema::categories::dsl::{category_id, category_name, categories};
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use actix::{Handler, Message};
use diesel::RunQueryDsl;
use slog::{o, Logger};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Category, AppError>")]
pub struct CreateCategory {
    pub name: String,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Category, AppError>")]
pub struct UpdateCategory {
    pub id: Uuid,
    pub name: String,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Category>, AppError>")]
pub struct GetAllCategories;

impl Handler<CreateCategory> for DbActor {
    type Result = Result<Category, AppError>;

    fn handle(&mut self, msg: CreateCategory, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "create_category"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let new_category = Category {
            category_id: Uuid::new_v4(),
            category_name: msg.name,
        };

        let result = diesel::insert_into(categories)
            .values(new_category)
            .get_result::<Category>(&mut conn)?;

        Ok(result)
    }
}

impl Handler<UpdateCategory> for DbActor {
    type Result = Result<Category, AppError>;

    fn handle(&mut self, msg: UpdateCategory, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "create_category"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let updated_category = diesel::update(categories)
            .filter(category_id.eq(msg.id))
            .set(category_name.eq(msg.name))
            .get_result::<Category>(&mut conn)?;

        Ok(updated_category)
    }
}

impl Handler<GetAllCategories> for DbActor {
    type Result = Result<Vec<Category>, AppError>;

    fn handle(&mut self, _: GetAllCategories, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;
        let result = categories.get_results::<Category>(&mut conn)?;
        Ok(result)
    }
}
