use pm_mod_firmware::settings::{LIVECOUNTER_CHECK, LIVECOUNTER_GENERATE};
use rsiot::{components::cmp_livecounter, message::Message};

use super::Custom;

pub fn config() -> cmp_livecounter::Config<Custom> {
    cmp_livecounter::Config {
        fn_generate_self_counter: |counter| Message::new_custom(Custom::SlaveLiveCounter(counter)),
        generate_self_period: LIVECOUNTER_GENERATE,

        fn_find_partner_counter: |msg| {
            let msg = msg.get_custom_data()?;
            if let Custom::MasterLiveCounter(counter) = msg {
                Some(counter)
            } else {
                None
            }
        },
        fn_check_partner_counter: |state| Message::new_custom(Custom::ConnectionState(state)),
        check_partner_period: LIVECOUNTER_CHECK,
    }
}
