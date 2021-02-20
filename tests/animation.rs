use ::ezplatform::animation::*;

#[test]
fn test_state_machine_update_once() {
    let mut state_machine: StateMachine<bool> = StateMachine::new(3);

    state_machine.add_rule(0, 1, |monitor| monitor);
    state_machine.add_rule(1, 0, |monitor| !monitor);
    state_machine.add_rule(1, 2, |monitor| monitor);
    state_machine.add_rule(2, 0, |monitor| !monitor);

    assert_eq!(0, state_machine.state());
    state_machine.update_once(true);
    assert_eq!(1, state_machine.state());
    state_machine.update_once(true);
    assert_eq!(2, state_machine.state());
    state_machine.update_once(true);
    assert_eq!(2, state_machine.state());
    state_machine.update_once(false);
    assert_eq!(0, state_machine.state());
}

#[test]
fn test_state_machine_update_full() {
    let mut state_machine: StateMachine<bool> = StateMachine::new(3);

    state_machine.add_rule(0, 1, |monitor| monitor);
    state_machine.add_rule(1, 0, |monitor| !monitor);
    state_machine.add_rule(1, 2, |monitor| monitor);
    state_machine.add_rule(2, 1, |monitor| !monitor);

    assert_eq!(0, state_machine.state());
    state_machine.update_full(true);
    assert_eq!(2, state_machine.state());
    state_machine.update_full(false);
    assert_eq!(0, state_machine.state());
}

#[test]
fn test_state_machine_update_with_limit() {
    let mut state_machine: StateMachine<bool> = StateMachine::new(3);

    state_machine.add_rule(0, 1, |monitor| monitor);
    state_machine.add_rule(1, 2, |monitor| monitor);
    state_machine.add_rule(2, 1, |monitor| monitor);

    assert_eq!(0, state_machine.state());
    state_machine.update_with_limit(true, 5);
    assert_eq!(1, state_machine.state());
}
