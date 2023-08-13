use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::models::screen::Screen;
use crate::schema::screens::dsl::screens;
use crate::schema::screens::{characteristics, price_per_time, screen_id, screen_name, traffic};
use actix::{Handler, Message};
use diesel::expression_methods::ExpressionMethods;
use diesel::RunQueryDsl;
use slog::{o, Logger};
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
            address_id: Uuid::new_v4(),
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
            .filter(screen_id.eq(msg.id))
            .set((
                screen_name.eq(msg.name),
                price_per_time.eq(msg.price_per_time),
                characteristics.eq(msg.characteristics),
                traffic.eq(msg.traffic),
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
