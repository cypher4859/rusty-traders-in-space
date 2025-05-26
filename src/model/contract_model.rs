use std::{fmt::DebugStruct, str::FromStr};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter};

// use crate::{dto::contract_dto::ContractTermsDTO, services::{contract, faction}, ContractDTO};
use crate::ContractTermsDTO;
use crate::ContractDTO;
use crate::PaymentDTO;
use crate::DeliverDTO;
use crate::model::faction_model::FactionSymbol;

use super::Faction; 


#[derive(Debug, Clone)]
pub struct Contract {
    id:            String,
    faction_symbol: FactionSymbol,
    contract_type:  String,           
    terms:         ContractTerms,
    accepted:      bool,
    fulfilled:     bool,
    expiration:    String,
    deadline_to_accept: String,
}

impl Contract {
    pub fn new<S1, S3, S4, S5>(
        id: S1,
        faction_symbol: FactionSymbol,
        contract_type: S3,
        terms: ContractTerms,
        accepted: bool,
        fulfilled: bool,
        expiration: S4,
        deadline_to_accept: S5
    ) -> anyhow::Result<Self>
    where
        S1: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
        S5: Into<String>
    {
        let id: String = id.into();
        let contract_type: String = contract_type.into();
        let expiration: String = expiration.into();
        let deadline_to_accept: String = deadline_to_accept.into();

        Ok(Self {
            id,
            faction_symbol,
            contract_type,
            terms,
            accepted,
            fulfilled,
            expiration,
            deadline_to_accept
        })
    }
}

impl TryFrom<ContractDTO> for Contract {
    type Error = anyhow::Error;


    fn try_from(dto: ContractDTO) -> anyhow::Result<Self> {
        Contract::new(
            dto.id,
            FactionSymbol::from_str(&dto.faction_symbol)?,
            dto.contract_type,
            dto.terms.try_into()?,
            dto.accepted,
            dto.fulfilled,
            dto.expiration,
            dto.deadline_to_accept
        )
    }
}


#[derive(Debug, Clone)]
pub struct ContractTerms {
    pub deadline: String,
    pub payment:  Payment,
    pub deliver:  Vec<Deliver>,
}

impl ContractTerms {
    pub fn new<S1>(
        deadline: S1,
        payment: Payment,
        deliver: Vec<Deliver>
    ) -> anyhow::Result<Self>
    where
        S1: Into<String>
    {
        let deadline: String = deadline.into();

        Ok(Self {
            deadline,
            payment,
            deliver
        })
    }
}

impl TryFrom<ContractTermsDTO> for ContractTerms {
    type Error = anyhow::Error;

    fn try_from(dto: ContractTermsDTO) -> anyhow::Result<Self> {
        let payment = dto.payment.try_into()?;
        let deliver = dto.
                                        deliver
                                        .into_iter()
                                        .map(Deliver::try_from)
                                        .collect::<anyhow::Result<Vec<_>>>()?;

        ContractTerms::new(
            dto.deadline,
            payment,
            deliver
        )
    }
}

#[derive(Debug, Clone)]
pub struct Payment {
    on_accepted: i64,
    on_fulfilled: i64
}

impl Payment {
    pub fn new(
        on_accepted: i64,
        on_fulfilled: i64
    ) -> anyhow::Result<Self>
    {
        Ok(Self {
            on_accepted,
            on_fulfilled
        })
    }
}

impl TryFrom<PaymentDTO> for Payment {
    type Error = anyhow::Error;

    fn try_from(dto: PaymentDTO) -> anyhow::Result<Self> {
        Payment::new(
            dto.on_accepted,
            dto.on_fulfilled
        )
    }
}

#[derive(Debug, Clone)]
pub struct Deliver {
    trade_symbol: String,
    destination_symbol: String,
    units_required: i64,
    units_fulfilled: i64,
}

impl Deliver {
    pub fn new<S1, S2>(
        trade_symbol: S1,
        destination_symbol: S2,
        units_required: i64,
        units_fulfilled: i64
    ) -> anyhow::Result<Self>
    where 
        S1: Into<String>,
        S2: Into<String>
    {
        let trade_symbol = trade_symbol.into();
        let destination_symbol = destination_symbol.into();

        Ok(Self {
            trade_symbol,
            destination_symbol,
            units_required,
            units_fulfilled
        })
    }
}

impl TryFrom<DeliverDTO> for Deliver {
    type Error = anyhow::Error;

    fn try_from(dto: DeliverDTO) -> anyhow::Result<Self> {
        Deliver::new(
            dto.trade_symbol,
            dto.destination_symbol,
            dto.units_required,
            dto.units_fulfilled
        )
    }
}

#[derive(Debug, Clone)]
pub struct DeliverGoods {
    trade_symbol: String,
    destination_symbol: String,
    units_required: i64,
    units_fulfilled: i64,
}

impl DeliverGoods {
    pub fn new<S1, S2>(
        trade_symbol: S1,
        destination_symbol: S2,
        units_required: i64,
        units_fulfilled: i64
    ) -> anyhow::Result<Self>
    where 
        S1: Into<String>,
        S2: Into<String>
    {
        let trade_symbol = trade_symbol.into();
        let destination_symbol = destination_symbol.into();

        Ok(Self {
            trade_symbol,
            destination_symbol,
            units_required,
            units_fulfilled
        })
    }
}

// impl TryFrom<DeliverDTO> for Deliver {
//     type Error = anyhow::Error;

//     fn try_from(dto: DeliverDTO) -> anyhow::Result<Self> {
//         Deliver::new(
//             dto.trade_symbol,
//             dto.destination_symbol,
//             dto.units_required,
//             dto.units_fulfilled
//         )
//     }
// }