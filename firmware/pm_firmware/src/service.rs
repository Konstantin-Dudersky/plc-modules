//! Название сервиса для работы исполнителя

use rsiot::message::ServiceBound;

#[derive(Debug, Clone, PartialEq)]
pub enum Service {}

impl ServiceBound for Service {}
