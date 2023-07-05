use tokio::sync::watch;

use crate::ScopeState;

#[derive(Clone)]
pub struct DebugProvider {
    pub rx: watch::Receiver<DebugInfo>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DebugInfo {
    pub looped: usize,
}

pub fn use_debug_info(cx: &ScopeState) -> DebugInfo {
    let mut provider = cx.consume_context::<DebugProvider>().unwrap();
    let mut rx = provider.rx.clone();
    let data = provider.rx.borrow_and_update();

    // updater
    let update = cx.schedule_update();
    cx.push_future(async move {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        _ = rx.changed().await;
        update();
    });

    data.clone()
}
