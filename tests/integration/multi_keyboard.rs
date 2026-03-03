mod fake_platform;

use aegis_input::core::policy::PolicyAction;
use aegis_input::core::state::{DeviceEvent, DeviceOrigin, DeviceState, DeviceType};
use aegis_input::service::runner::Runner;
use fake_platform::FakePlatform;

#[test]
fn multiple_external_keyboards_require_all_removed() {
    let kb1_connect = DeviceEvent {
        id: "usb-kb-1".to_string(),
        name: "External Keyboard 1".to_string(),
        device_type: DeviceType::Keyboard,
        origin: DeviceOrigin::External,
        state: DeviceState::Connected,
    };
    let kb2_connect = DeviceEvent {
        id: "usb-kb-2".to_string(),
        name: "External Keyboard 2".to_string(),
        device_type: DeviceType::Keyboard,
        origin: DeviceOrigin::External,
        state: DeviceState::Connected,
    };
    let kb1_disconnect = DeviceEvent {
        id: "usb-kb-1".to_string(),
        name: "External Keyboard 1".to_string(),
        device_type: DeviceType::Keyboard,
        origin: DeviceOrigin::External,
        state: DeviceState::Disconnected,
    };
    let kb2_disconnect = DeviceEvent {
        id: "usb-kb-2".to_string(),
        name: "External Keyboard 2".to_string(),
        device_type: DeviceType::Keyboard,
        origin: DeviceOrigin::External,
        state: DeviceState::Disconnected,
    };

    let backend = FakePlatform::new(
        Vec::new(),
        vec![kb1_connect, kb2_connect, kb1_disconnect, kb2_disconnect],
    );
    let mut runner = Runner::new(backend, true).expect("runner init");

    runner.process_next_event().expect("kb1 connect");
    let disable_count = runner
        .backend()
        .actions
        .iter()
        .filter(|a| **a == PolicyAction::Disable(DeviceType::Keyboard))
        .count();
    assert_eq!(disable_count, 1);

    let actions_len_before = runner.backend().actions.len();
    runner.process_next_event().expect("kb2 connect");
    assert_eq!(runner.backend().actions.len(), actions_len_before);

    runner.process_next_event().expect("kb1 disconnect");
    assert!(!runner
        .backend()
        .actions
        .contains(&PolicyAction::Enable(DeviceType::Keyboard)));

    runner.process_next_event().expect("kb2 disconnect");
    assert!(runner
        .backend()
        .actions
        .contains(&PolicyAction::Enable(DeviceType::Keyboard)));
}
