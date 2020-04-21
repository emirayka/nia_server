use std::sync::mpsc;

use nia_events::{KeyChordPart, EventListener};
use nia_events::Event;
use nia_events::KeyCommand;
use nia_events::EventListenerSettingsBuilder;
use nia_events::KeyboardId;
use nia_events::Command;
use nia_events::KeyChord;

use crate::event_handler::Action;
use std::collections::HashMap;
use crate::error::Error;
use nia_state_machine::StateMachineResult;
use std::thread;
use std::sync::mpsc::TryRecvError;

pub struct NiaEventHandler {
    keyboards: Vec<(String, String)>,
    modifiers: Vec<KeyChordPart>,
    mappings: Vec<(Vec<KeyChord>, Action)>
}

impl NiaEventHandler {
    pub fn new() -> NiaEventHandler {
        NiaEventHandler {
            keyboards: Vec::new(),
            modifiers: Vec::new(),
            mappings: Vec::new(),
        }
    }

    pub fn add_keyboard(&mut self, path: &str, name: &str) {
        self.keyboards.push((String::from(path), String::from(name)))
    }

    pub fn add_modifier(&mut self, modifier: KeyChordPart) {
        self.modifiers.push(modifier)
    }

    pub fn add_mapping(&mut self, key_chords: Vec<KeyChord>, action: Action) {
        self.mappings.push((key_chords, action))
    }

    pub fn start_listening(
        &self
    ) -> Result<(mpsc::Sender<Command>, mpsc::Receiver<Action>, mpsc::Sender<()>), Error> {
        let mut settings_builder = EventListenerSettingsBuilder::new();
        let mut map = HashMap::new();
        let mut iterator = self.keyboards.iter().enumerate();

        for (index, (keyboard_path, keyboard_name)) in iterator {
            settings_builder = settings_builder.add_keyboard(keyboard_path.clone());
            map.insert(keyboard_name.clone(), KeyboardId::new(index as u16));
        }

        for modifier in self.modifiers.iter() {
            settings_builder = settings_builder.add_modifier(*modifier);
        }

        let settings = settings_builder.build();

        let event_listener = EventListener::new(settings);
        let (event_receiver, event_stopper) = event_listener.start_listening();

        let command_sender = nia_events::CommandSender::new();
        let (cmd_sender, cmd_stopper) = command_sender.start_sending();

        let mut state_machine = nia_state_machine::StateMachine::new();

        for (path, action) in self.mappings.iter() {
            state_machine.add(path.clone(), *action)
                .map_err(|_| Error::unknown())?;
        }

        println!("{:?}", self.mappings);
        let (action_sender, action_receiver) = mpsc::channel();
        let (tx, rx) = mpsc::channel();

        {
            let action_sender = action_sender.clone();
            let cmd_sender = cmd_sender.clone();

            thread::spawn(move || {
                loop {
                    let event = match event_receiver.recv() {
                        Ok(event) => {
                            event
                        },
                        Err(_) => break
                    };

                    println!("{:?}", event);

                    match event {
                        Event::KeyChordEvent(key_chord) => {
                            match state_machine.excite(key_chord) {
                                StateMachineResult::Excited(action) => {
                                    println!("Excited!");
                                    action_sender.send(action);
                                },
                                StateMachineResult::Fallback(previous) => {
                                    for key_chord in previous {
                                        let command = nia_events::Command::KeyCommand(
                                            KeyCommand::ForwardKeyChord(key_chord)
                                        );

                                        match cmd_sender.send(command) {
                                            Ok(_) => {},
                                            Err(_) => break
                                        }
                                    }
                                },
                                StateMachineResult::Transition() => {}
                            }
                        }
                    }

                    match rx.try_recv() {
                        Ok(()) | Err(TryRecvError::Disconnected) => {
                            break;
                        },
                        Err(TryRecvError::Empty) => {}
                    }
                }

                cmd_stopper.send(());
                event_stopper.send(());
            });
        }

        Ok((cmd_sender, action_receiver, tx))
    }
}
