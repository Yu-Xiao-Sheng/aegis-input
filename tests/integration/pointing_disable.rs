mod fake_platform;

use aegis_input::core::policy::PolicyAction;
use aegis_input::core::state::{DeviceEvent, DeviceOrigin, DeviceState, DeviceType};
use aegis_input::service::runner::Runner;
use fake_platform::FakePlatform;

#[test]
fn external_mouse_disables_and_restores_internal_pointing() {
    let connect = DeviceEvent {
        id: "usb-mouse-1".to_string(),
        name: "External Mouse".to_string(),
        device_type: DeviceType::Pointing,
        origin: DeviceOrigin::External,
        state: DeviceState::Connected,
    };
    let disconnect = DeviceEvent {
        id: "usb-mouse-1".to_string(),
        name: "External Mouse".to_string(),
        device_type: DeviceType::Pointing,
        origin: DeviceOrigin::External,
        state: DeviceState::Disconnected,
    };

    let backend = FakePlatform::new(Vec::new(), vec![connect, disconnect]);
    let mut runner = Runner::new(backend, true).expect("runner init");

    runner.process_next_event().expect("process connect");
    assert!(runner
        .backend()
        .actions
        .contains(&PolicyAction::Disable(DeviceType::Pointing)));

    runner.process_next_event().expect("process disconnect");
    assert!(runner
        .backend()
        .actions
        .contains(&PolicyAction::Enable(DeviceType::Pointing)));
}
