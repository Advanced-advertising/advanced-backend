use crate::actors::db::{get_pooled_connection, DbActor};
use crate::diesel::ExpressionMethods;
use crate::errors::AppError;
use crate::models::ad::Ad;
use crate::models::income::{Income, IncomeAllData};
use crate::models::user::User;
use crate::schema::ad_orders::dsl::ad_orders;
use crate::schema::ad_orders::{ad_id as order_ad_id_column, ad_order_id as order_id_column};
use crate::schema::ads::dsl::ads;
use crate::schema::ads::{ad_id as ad_id_column, user_id as ad_user_id_column};
use crate::schema::incomes::dsl::incomes;
use crate::schema::incomes::{
    ad_order_id as income_order_id_column, business_id as income_business_id_column,
};
use crate::schema::users::dsl::users;
use crate::schema::users::user_id as user_id_column;
use actix::{Handler, Message};
use diesel::prelude::*;
use diesel::{JoinOnDsl, QueryDsl, SelectableHelper};
use slog::{o, Logger};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Vec<IncomeAllData>, AppError>")]
pub struct GetAllIncomes {
    pub business_id: Uuid,
    pub logger: Logger,
}

impl Handler<GetAllIncomes> for DbActor {
    type Result = Result<Vec<IncomeAllData>, AppError>;

    fn handle(&mut self, msg: GetAllIncomes, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "get_all_icomes"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let incomes_data = incomes
            .inner_join(ad_orders.on(order_id_column.eq(income_order_id_column)))
            .inner_join(ads.on(ad_id_column.eq(order_ad_id_column)))
            .inner_join(users.on(user_id_column.eq(ad_user_id_column)))
            .select((Income::as_select(), User::as_select(), Ad::as_select()))
            .filter(income_business_id_column.eq(msg.business_id))
            .load::<(Income, User, Ad)>(&mut conn)?;

        let incomes_all_data = incomes_data
            .into_iter()
            .map(|(income, user, ad)| IncomeAllData {
                price: income.income,
                client: user,
                ad,
            })
            .collect();

        Ok(incomes_all_data)
    }
}
