//! Название сервиса для работы исполнителя

use rsiot::message::ServiceBound;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Service {
    PM_DI16,
    PM_RQ8,
}

impl ServiceBound for Service {}
