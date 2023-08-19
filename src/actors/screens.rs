use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::models::screen::{Screen, ScreenData, ScreenDataWithAddress};
use crate::schema::screens::dsl::screens;
use crate::schema::screens::{
    business_id as screen_business_id_column,
    address_id as screen_address_id,
    characteristics as screen_characteristics_column,
    price_per_time as screen_price_per_time_column,
    screen_id as screen_screen_id_column,
    screen_name as screen_name_column,
    traffic as screen_traffic_column
};
use actix::{Handler, Message};
use diesel::expression_methods::ExpressionMethods;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use slog::{o, Logger};
use uuid::Uuid;
use crate::schema::addresses::{address_id, address_name as address_name_column};
use crate::schema::addresses::dsl::addresses;


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

impl Handler<CreateScreen> for DbActor {
    type Result = Result<Screen, AppError>;

    fn handle(&mut self, msg: CreateScreen, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "create_screen"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let new_address = Screen {
            screen_id: Uuid::new_v4(),
            screen_name: msg.name,
            price_per_time: msg.price_per_time,
            characteristics: msg.characteristics,
            traffic: msg.traffic,
            address_id: msg.address_id,
            business_id: msg.business_id,
        };

        let result = diesel::insert_into(screens)
            .values(new_address)
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
        let sub_log = msg.logger.new(o!("handle" => "get_all_screens_by_business_id"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;
        let result = screens.filter(screen_business_id_column.eq(msg.business_id)).get_results::<Screen>(&mut conn)?;
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