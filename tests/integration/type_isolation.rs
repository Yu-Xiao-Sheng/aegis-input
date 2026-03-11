mod fake_platform;

use aegis_input::core::policy::PolicyAction;
use aegis_input::core::state::{DeviceEvent, DeviceOrigin, DeviceState, DeviceType};
use aegis_input::service::runner::Runner;
use fake_platform::FakePlatform;

#[test]
fn keyboard_and_mouse_are_independent() {
    let kb_connect = DeviceEvent {
        id: "usb-kb".to_string(),
        name: "External Keyboard".to_string(),
        device_type: DeviceType::Keyboard,
        origin: DeviceOrigin::External,
        state: DeviceState::Connected,
    };
    let mouse_connect = DeviceEvent {
        id: "usb-mouse".to_string(),
        name: "External Mouse".to_string(),
        device_type: DeviceType::Pointing,
        origin: DeviceOrigin::External,
        state: DeviceState::Connected,
    };

    let backend = FakePlatform::new(Vec::new(), vec![kb_connect, mouse_connect]);
    let mut runner = Runner::new(backend, true).expect("runner init");

    runner.process_next_event().expect("kb connect");
    runner.process_next_event().expect("mouse connect");

    assert!(
        runner
            .backend()
            .actions
            .contains(&PolicyAction::Disable(DeviceType::Keyboard))
    );
    assert!(
        runner
            .backend()
            .actions
            .contains(&PolicyAction::Disable(DeviceType::Pointing))
    );
}
