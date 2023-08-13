use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::models::address::Address;
use crate::schema::addresses::dsl::{address_id, address_name, addresses};
use actix::{Handler, Message};
use diesel::expression_methods::ExpressionMethods;
use diesel::RunQueryDsl;
use slog::{o, Logger};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Address, AppError>")]
pub struct CreateAddress {
    pub name: String,
    pub business_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Address, AppError>")]
pub struct UpdateAddress {
    pub id: Uuid,
    pub name: String,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Address>, AppError>")]
pub struct GetAllAddresses;

impl Handler<CreateAddress> for DbActor {
    type Result = Result<Address, AppError>;

    fn handle(&mut self, msg: CreateAddress, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "create_address"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let new_address = Address {
            address_id: Uuid::new_v4(),
            address_name: msg.name,
            business_id: Some(msg.business_id),
        };

        let result = diesel::insert_into(addresses)
            .values(new_address)
            .get_result::<Address>(&mut conn)?;

        Ok(result)
    }
}

impl Handler<UpdateAddress> for DbActor {
    type Result = Result<Address, AppError>;

    fn handle(&mut self, msg: UpdateAddress, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "update_address"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let updated_address = diesel::update(addresses)
            .filter(address_id.eq(msg.id))
            .set(address_name.eq(msg.name))
            .get_result::<Address>(&mut conn)?;

        Ok(updated_address)
    }
}

impl Handler<GetAllAddresses> for DbActor {
    type Result = Result<Vec<Address>, AppError>;

    fn handle(&mut self, _: GetAllAddresses, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;
        let result = addresses.get_results::<Address>(&mut conn)?;
        Ok(result)
    }
}
