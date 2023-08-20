use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::{AppError, AppErrorType};
use crate::models::ad::{Ad, AdStatus};
use crate::models::ad_order::{AdOrder, AdOrderAllData};
use crate::models::address::Address;
use crate::models::income::Income;
use crate::models::screen::Screen;
use crate::models::user::User;
use crate::schema::ad_orders::dsl::ad_orders;
use crate::schema::ad_orders::{
    ad_id as ad_orders_ad_id_column, ad_order_id as order_id_column,
    is_rejected as is_rejected_column, screen_id as ad_orders_screen_id_column,
};
use crate::schema::addresses::address_id as address_id_column;
use crate::schema::addresses::dsl::addresses;
use crate::schema::ads::dsl::ads;
use crate::schema::ads::{ad_id as ad_id_column, user_id as ads_user_id_column};
use crate::schema::businesses::business_id as business_id_column;
use crate::schema::businesses::dsl::businesses;
use crate::schema::incomes::dsl::incomes;
use crate::schema::screens::dsl::screens;
use crate::schema::screens::{
    address_id as screen_address_id_column, business_id as screen_business_id_column,
    screen_id as screen_id_column,
};
use crate::schema::users::dsl::users;
use crate::schema::users::user_id as user_id_column;
use actix::{Handler, Message};
use diesel::data_types::PgTimestamp;
use diesel::expression_methods::ExpressionMethods;
use diesel::{Connection, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use slog::{o, Logger};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<(), AppError>")]
pub struct CreateAdOrder {
    pub start_time: i64,
    pub end_time: i64,
    pub price: f64,
    pub ad_id: Uuid,
    pub screen_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<AdOrderAllData>, AppError>")]
pub struct GetBusinessAdOrders {
    pub business_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<(), AppError>")]
pub struct RejectAdOrder {
    pub ad_order_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<(), AppError>")]
pub struct ApproveAdOrder {
    pub ad_order_id: Uuid,
    pub logger: Logger,
}

impl Handler<CreateAdOrder> for DbActor {
    type Result = Result<(), AppError>;

    fn handle(&mut self, msg: CreateAdOrder, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "create_ad_order"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let ad: Ad = ads.find(msg.ad_id).first::<Ad>(&mut conn)?;

        if ad.status == AdStatus::Unverified.to_string() {
            let message = Some("Ad is unverified".to_string());
            return Err(AppError::new(
                message,
                None,
                AppErrorType::UnverifiedAdError,
            ));
        } else if ad.status == AdStatus::Rejected.to_string() {
            let message = Some("Ad is rejected".to_string());
            return Err(AppError::new(message, None, AppErrorType::RejectedAdError));
        }

        let start_time = PgTimestamp(msg.start_time);
        let end_time = PgTimestamp(msg.end_time);

        let new_ad_order = AdOrder {
            ad_order_id: Uuid::new_v4(),
            start_time,
            end_time,
            price: msg.price,
            is_rejected: false,
            ad_id: msg.ad_id,
            screen_id: msg.screen_id,
        };

        diesel::insert_into(ad_orders)
            .values(new_ad_order)
            .get_result::<AdOrder>(&mut conn)?;

        Ok(())
    }
}

impl Handler<GetBusinessAdOrders> for DbActor {
    type Result = Result<Vec<AdOrderAllData>, AppError>;

    fn handle(&mut self, msg: GetBusinessAdOrders, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "get_all_ad_orders"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let ad_orders_data = ad_orders
            .inner_join(ads.on(ad_id_column.eq(ad_orders_ad_id_column)))
            .inner_join(users.on(user_id_column.eq(ads_user_id_column)))
            .inner_join(screens.on(screen_id_column.eq(ad_orders_screen_id_column)))
            .inner_join(addresses.on(address_id_column.eq(screen_address_id_column)))
            .select((
                Ad::as_select(),
                User::as_select(),
                Screen::as_select(),
                Address::as_select(),
                AdOrder::as_select(),
            ))
            .filter(screen_business_id_column.eq(msg.business_id))
            .load::<(Ad, User, Screen, Address, AdOrder)>(&mut conn)?;

        let ad_orders_all_data = ad_orders_data
            .into_iter()
            .map(|(ad, client, screen, address, ad_order)| AdOrderAllData {
                order_id: ad_order.ad_order_id,
                start_time: ad_order.start_time.0,
                end_time: ad_order.end_time.0,
                price: ad_order.price,
                is_rejected: ad_order.is_rejected,
                address_name: address.address_name,
                ad,
                client,
                screen,
            })
            .collect();

        Ok(ad_orders_all_data)
    }
}

impl Handler<ApproveAdOrder> for DbActor {
    type Result = Result<(), AppError>;

    fn handle(&mut self, msg: ApproveAdOrder, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "approve_ad_order"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let wrapped_ad_order: Option<AdOrder> = ad_orders
            .filter(order_id_column.eq(msg.ad_order_id))
            .first(&mut conn)
            .optional()?;

        let ad_order = match wrapped_ad_order {
            Some(ad_order) => {
                if !ad_order.is_rejected {
                    return Err(AppError::new(
                        Some("Add order already approved".to_string()),
                        None,
                        AppErrorType::RejectedAdError,
                    ));
                }
                ad_order
            }
            None => {
                return Err(AppError::new(
                    Some("Add order not found".to_string()),
                    None,
                    AppErrorType::NotFoundError,
                ));
            }
        };

        let wrapped_business_id: Option<Uuid> = screens
            .filter(screen_id_column.eq(ad_order.screen_id))
            .inner_join(businesses)
            .select(business_id_column)
            .first(&mut conn)
            .optional()?;

        let business_id = if wrapped_business_id.is_none() {
            return Err(AppError::new(
                Some("Business id not found".to_string()),
                None,
                AppErrorType::NotFoundError,
            ));
        } else {
            wrapped_business_id.unwrap()
        };

        let new_income = Income {
            income_id: Uuid::new_v4(),
            income: ad_order.price,
            business_id,
            ad_order_id: ad_order.ad_order_id,
        };

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::insert_into(incomes)
                .values(new_income)
                .get_result::<Income>(conn)?;

            diesel::update(ad_orders.filter(order_id_column.eq(msg.ad_order_id)))
                .set(is_rejected_column.eq(false))
                .execute(conn)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl Handler<RejectAdOrder> for DbActor {
    type Result = Result<(), AppError>;

    fn handle(&mut self, msg: RejectAdOrder, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "reject_ad_order"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let wrapped_ad_order: Option<AdOrder> = ad_orders
            .filter(order_id_column.eq(msg.ad_order_id))
            .first(&mut conn)
            .optional()?;

        match wrapped_ad_order {
            Some(ad_order) => {
                if ad_order.is_rejected {
                    return Err(AppError::new(
                        Some("Add order already rejected".to_string()),
                        None,
                        AppErrorType::RejectedAdError,
                    ));
                }
            }
            None => {
                return Err(AppError::new(
                    Some("Add order not found".to_string()),
                    None,
                    AppErrorType::NotFoundError,
                ));
            }
        };

        diesel::update(ad_orders.filter(order_id_column.eq(msg.ad_order_id)))
            .set(is_rejected_column.eq(true))
            .execute(&mut conn)?;

        Ok(())
    }
}
