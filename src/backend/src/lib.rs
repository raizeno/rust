use std::cell::RefCell;

use ic_cdk_timers::TimerId;

thread_local! {
    static TIMER_ID: RefCell<Option<TimerId>> = RefCell::default();
}

fn timer_id() -> Option<TimerId> {
    TIMER_ID.with(|state| *state.borrow())
}
fn set_timer_id(value: Option<TimerId>) {
    TIMER_ID.with(|state| *state.borrow_mut() = value);
}
fn now() -> u64 {
    ic_cdk::api::time() / (1000 * 1000000)
}

#[ic_cdk::update]
#[candid::candid_method(update)]
pub fn start(interval_secs: u64) {
    let interval = std::time::Duration::from_secs(interval_secs as u64);
    let timer_id = ic_cdk_timers::set_timer_interval(interval, || {
        ic_cdk::println!("run task = {}", now());
    });
    set_timer_id(Some(timer_id));
    ic_cdk::println!("Started...: {}", now())
}

#[ic_cdk::query]
#[candid::candid_method(query)]
pub fn get_timer_id() -> String {
    let id = timer_id();
    format!("{:?}", id)
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stop() {
    let timer_id = timer_id();
    if let Some(timer_id) = timer_id {
        ic_cdk_timers::clear_timer(timer_id);
        set_timer_id(None);
        ic_cdk::println!("Stopped...: {}", now())
    }
}

candid::export_service!();
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}