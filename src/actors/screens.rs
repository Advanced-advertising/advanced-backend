use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::models::screen::{Screen, ScreenData, ScreenDataWithAddress};
use crate::schema::addresses::dsl::addresses;
use crate::schema::addresses::{address_id, address_name as address_name_column};
use crate::schema::business_categories::business_id as business_categories_business_id_column;
use crate::schema::business_categories::category_id as business_categories_cat_id_column;
use crate::schema::business_categories::dsl::business_categories;
use crate::schema::businesses::business_id as business_id_column;
use crate::schema::businesses::dsl::businesses;
use crate::schema::screens::dsl::screens;
use crate::schema::screens::{
    address_id as screen_address_id, business_id as screen_business_id_column,
    characteristics as screen_characteristics_column,
    price_per_time as screen_price_per_time_column, screen_id as screen_screen_id_column,
    screen_name as screen_name_column, traffic as screen_traffic_column,
};
use actix::{Handler, Message};
use diesel::expression_methods::ExpressionMethods;
use diesel::{JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use slog::{o, Logger};
use std::cmp::Reverse;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Screen, AppError>")]
pub struct CreateScreen {
    pub name: String,
    pub price_per_time: f64,
    pub characteristics: String,
    pub traffic: i32,
    pub business_id: Uuid,
    pub address_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Screen, AppError>")]
pub struct UpdateScreen {
    pub id: Uuid,
    pub name: String,
    pub price_per_time: f64,
    pub characteristics: String,
    pub traffic: i32,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Screen>, AppError>")]
pub struct GetAllScreens {
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Option<ScreenDataWithAddress>, AppError>")]
pub struct GetScreenDataById {
    pub screen_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Screen>, AppError>")]
pub struct GetAllScreensByBusinessId {
    pub business_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Screen>, AppError>")]
pub struct GetOptimalScreens {
    pub user_budget: f64,
    pub ad_category_ids: Vec<Uuid>,
    pub logger: Logger,
}

impl Handler<CreateScreen> for DbActor {
    type Result = Result<Screen, AppError>;

    fn handle(&mut self, msg: CreateScreen, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "create_screen"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let new_screen = Screen {
            screen_id: Uuid::new_v4(),
            screen_name: msg.name,
            price_per_time: msg.price_per_time,
            characteristics: msg.characteristics,
            traffic: msg.traffic,
            address_id: msg.address_id,
            business_id: msg.business_id,
        };

        let result = diesel::insert_into(screens)
            .values(new_screen)
            .get_result::<Screen>(&mut conn)?;

        Ok(result)
    }
}

impl Handler<UpdateScreen> for DbActor {
    type Result = Result<Screen, AppError>;

    fn handle(&mut self, msg: UpdateScreen, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "update_address"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let updated_screen = diesel::update(screens)
            .filter(screen_screen_id_column.eq(msg.id))
            .set((
                screen_name_column.eq(msg.name),
                screen_price_per_time_column.eq(msg.price_per_time),
                screen_characteristics_column.eq(msg.characteristics),
                screen_traffic_column.eq(msg.traffic),
            ))
            .get_result::<Screen>(&mut conn)?;

        Ok(updated_screen)
    }
}

impl Handler<GetAllScreens> for DbActor {
    type Result = Result<Vec<Screen>, AppError>;

    fn handle(&mut self, msg: GetAllScreens, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "get_all_screens"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;
        let result = screens.get_results::<Screen>(&mut conn)?;
        Ok(result)
    }
}

impl Handler<GetAllScreensByBusinessId> for DbActor {
    type Result = Result<Vec<Screen>, AppError>;

    fn handle(&mut self, msg: GetAllScreensByBusinessId, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg
            .logger
            .new(o!("handle" => "get_all_screens_by_business_id"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;
        let result = screens
            .filter(screen_business_id_column.eq(msg.business_id))
            .get_results::<Screen>(&mut conn)?;
        Ok(result)
    }
}

impl Handler<GetScreenDataById> for DbActor {
    type Result = Result<Option<ScreenDataWithAddress>, AppError>;

    fn handle(&mut self, msg: GetScreenDataById, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "get_screen_data_by_id"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let screen_data = screens
            .filter(screen_screen_id_column.eq(msg.screen_id))
            .select((
                screen_name_column,
                screen_price_per_time_column,
                screen_characteristics_column,
                screen_traffic_column,
                screen_business_id_column,
                screen_address_id,
            ))
            .first::<ScreenData>(&mut conn)
            .optional()?;

        if let Some(screen_data) = screen_data {
            let address_name_query = addresses
                .select(address_name_column)
                .filter(address_id.eq(screen_data.address_id))
                .first::<String>(&mut conn);

            match address_name_query {
                Ok(address_name) => {
                    let screen_data_with_address = ScreenDataWithAddress {
                        screen_id: msg.screen_id,
                        screen_name: screen_data.screen_name,
                        price_per_time: screen_data.price_per_time,
                        characteristics: screen_data.characteristics,
                        traffic: screen_data.traffic,
                        address_name,
                        business_id: screen_data.business_id,
                    };
                    Ok(Some(screen_data_with_address))
                }
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl Handler<GetOptimalScreens> for DbActor {
    type Result = Result<Vec<Screen>, AppError>;

    fn handle(&mut self, msg: GetOptimalScreens, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "get_optimal_screens"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let mut screens_data: Vec<Screen> = screens
            .inner_join(businesses.on(business_id_column.eq(business_id_column)))
            .inner_join(
                business_categories
                    .on(business_id_column.eq(business_categories_business_id_column)),
            )
            .filter(business_categories_cat_id_column.eq_any(msg.ad_category_ids))
            .select(Screen::as_select())
            .load::<Screen>(&mut conn)?;

        let result = find_optimal_screens(msg.user_budget, &mut screens_data);

        fn find_optimal_screens(user_budget: f64, screens_data: &mut Vec<Screen>) -> Vec<Screen> {
            let mut optimal_screens: Vec<Screen> = Vec::new();
            let mut remaining_budget = user_budget;

            screens_data.sort_by_key(|screen| {
                Reverse((screen.traffic as f64 / screen.price_per_time * 1000.0) as i64)
            });

            for screen in screens_data.iter_mut() {
                if screen.price_per_time <= remaining_budget {
                    optimal_screens.push(screen.clone());
                    remaining_budget -= screen.price_per_time;
                }
            }

            optimal_screens
        }

        Ok(result)
    }
}
