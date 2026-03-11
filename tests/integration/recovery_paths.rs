mod fake_platform;

use aegis_input::core::policy::PolicyAction;
use aegis_input::core::state::{DeviceEvent, DeviceOrigin, DeviceState, DeviceType};
use aegis_input::service::runner::Runner;
use fake_platform::FakePlatform;

#[test]
fn disable_feature_restores_and_blocks_actions() {
    let connect = DeviceEvent {
        id: "usb-kb-1".to_string(),
        name: "External Keyboard".to_string(),
        device_type: DeviceType::Keyboard,
        origin: DeviceOrigin::External,
        state: DeviceState::Connected,
    };

    let backend = FakePlatform::new(Vec::new(), vec![connect]);
    let mut runner = Runner::new(backend, false).expect("runner init");

    runner.process_next_event().expect("process connect");
    assert!(
        !runner
            .backend()
            .actions
            .contains(&PolicyAction::Disable(DeviceType::Keyboard))
    );

    runner.set_enabled(true).expect("enable feature");
    assert!(
        runner
            .backend()
            .actions
            .contains(&PolicyAction::Disable(DeviceType::Keyboard))
    );

    runner.set_enabled(false).expect("disable feature");
    assert!(
        runner
            .backend()
            .actions
            .contains(&PolicyAction::Enable(DeviceType::Keyboard))
    );
}
