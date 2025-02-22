use std::time::Duration;

use rsiot::{components::cmp_livecounter, message::Message};

use messages::Custom;

pub fn config() -> cmp_livecounter::Config<Custom> {
    cmp_livecounter::Config {
        fn_generate_self_counter: |counter| Message::new_custom(Custom::MasterLiveCounter(counter)),
        generate_self_period: Duration::from_millis(500),
        fn_find_partner_counter: |_| None,
        fn_check_partner_counter: |_| Message::new_custom(Custom::ConnectionState(false)),
        check_partner_period: Duration::from_millis(2000),
    }
}
