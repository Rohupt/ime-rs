use crate::engine::CompositionProcessorEngine;
use windows::Win32::UI::{Input::KeyboardAndMouse::{
    VK_BACK, VK_DOWN, VK_END, VK_ESCAPE, VK_HOME, VK_LEFT, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT,
    VK_TAB, VK_UP, VK_SPACE,
}, TextServices::{TF_MOD_CONTROL, TF_MOD_ALT, TF_MOD_SHIFT}};

#[repr(C)]
#[derive(PartialEq)]
pub enum KeystrokeCategory {
    None,
    Composing,
    Candidate,
    InvokeCompositionEditSession,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum KeystrokeFunction {
    None,
    Input,
    Cancel,
    FinalizeTextstore,
    FinalizeTextstoreAndInput,
    FinalizeTextstoreOriginal,
    FinalizeCandidatelist,
    FinalizeCandidatelistAndInput,
    Convert,
    SelectByNumber,
    Backspace,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MovePageUp,
    MovePageDown,
    MovePageTop,
    MovePageBottom,

    // Function Double/Single byte
    DoubleSingleByte,

    // Function Punctuation
    Punctuation,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub enum CandidateMode {
    None,
    Original,
    Incremental,
}

fn map_invariable_keystroke_function(keystroke: u16) -> Option<KeystrokeFunction> {
    match keystroke {
        k if k == VK_TAB.0 => Some(KeystrokeFunction::Convert),
        k if k == VK_RETURN.0 => Some(KeystrokeFunction::FinalizeCandidatelist),
        k if k == VK_SPACE.0 => Some(KeystrokeFunction::FinalizeTextstore),
        k if k == VK_UP.0 => Some(KeystrokeFunction::MoveUp),
        k if k == VK_DOWN.0 => Some(KeystrokeFunction::MoveDown),
        k if k == VK_PRIOR.0 => Some(KeystrokeFunction::MovePageUp),
        k if k == VK_NEXT.0 => Some(KeystrokeFunction::MovePageDown),
        k if k == VK_HOME.0 => Some(KeystrokeFunction::MovePageTop),
        k if k == VK_END.0 => Some(KeystrokeFunction::MovePageBottom),
        _ => None,
    }
}

fn character_affects_keystroke_composition(ch: char, modifiers: u32, candidate_mode: CandidateMode) -> bool {
    // Normally [a-z] are only relevant since composition does not happen with Shift,
    // but KEYEVENTF_UNICODE can dispatch A-Z without Shift and that's allowed here
    // to maintain the original behavior.
    (
        ((' '..='~').contains(&ch) && modifiers & (TF_MOD_CONTROL | TF_MOD_ALT) == 0) &&
        (if ch == ' ' { modifiers & TF_MOD_SHIFT == 0 } else { true })
    ) && !(ch.is_numeric() && candidate_mode == CandidateMode::Original)
}

fn is_keystroke_range(ch: char, candidate_mode: CandidateMode) -> bool {
    if !ch.is_numeric() {
        false
    } else {
        candidate_mode == CandidateMode::Original
    }
}

/// Test virtual key code need to the Composition Processor Engine.
/// If engine need this virtual key code, returns true. Otherwise returns false.
/// Returns function regarding virtual key.
pub fn test_virtual_key(
    engine: &CompositionProcessorEngine,
    code: u16,
    ch: char,
    mut composing: bool,
    candidate_mode: CandidateMode,
) -> (bool, KeystrokeCategory, KeystrokeFunction) {
    if candidate_mode == CandidateMode::Original {
        composing = false;
    }

    let modifiers = engine.modifiers().get();

    // Candidate list could not handle key. We can try to restart the composition.
    if character_affects_keystroke_composition(ch, modifiers, candidate_mode) {
        return if candidate_mode == CandidateMode::Original {
            (
                true,
                KeystrokeCategory::Candidate,
                KeystrokeFunction::FinalizeCandidatelistAndInput,
            )
        } else {
            (true, KeystrokeCategory::Composing, KeystrokeFunction::Input)
        };
    }

    let mapped_function = map_invariable_keystroke_function(code);
    // System pre-defined keystroke
    if composing {
        if let Some(mapped_function) = mapped_function {
            if mapped_function == KeystrokeFunction::FinalizeTextstore && modifiers & (TF_MOD_CONTROL | TF_MOD_ALT) == 0 {
                return if modifiers & TF_MOD_SHIFT == 0 {
                    (false, KeystrokeCategory::Composing, KeystrokeFunction::Input)
                } else {
                    (true, KeystrokeCategory::Composing, KeystrokeFunction::FinalizeTextstore)
                };
            }
            if mapped_function == KeystrokeFunction::FinalizeCandidatelist && modifiers & (TF_MOD_CONTROL | TF_MOD_ALT) == 0 && modifiers & (TF_MOD_SHIFT) != 0 {
                return (true, KeystrokeCategory::Composing, KeystrokeFunction::FinalizeTextstoreOriginal);
            }
            let category = if candidate_mode == CandidateMode::Incremental {
                KeystrokeCategory::Candidate
            } else {
                KeystrokeCategory::Composing
            };
            return (true, category, mapped_function);
        }
        if candidate_mode != CandidateMode::Incremental {
            match code {
                c if c == VK_LEFT.0 => {
                    return (
                        true,
                        KeystrokeCategory::Composing,
                        KeystrokeFunction::MoveLeft,
                    )
                }
                c if c == VK_RIGHT.0 => {
                    return (
                        true,
                        KeystrokeCategory::Composing,
                        KeystrokeFunction::MoveRight,
                    )
                }
                c if c == VK_ESCAPE.0 => {
                    return (
                        true,
                        KeystrokeCategory::Composing,
                        KeystrokeFunction::Cancel,
                    )
                }
                c if c == VK_BACK.0 => {
                    return (
                        true,
                        KeystrokeCategory::Composing,
                        KeystrokeFunction::Backspace,
                    )
                }
                _ => (),
            }
        } else {
            match code {
                // VK_LEFT, VK_RIGHT - set *pIsEaten = false for application could move caret left or right.
                // and for CUAS, invoke _HandleCompositionCancel() edit session due to ignore CUAS default key handler for send out terminate composition
                c if c == VK_LEFT.0 || c == VK_RIGHT.0 => {
                    return (
                        false,
                        KeystrokeCategory::InvokeCompositionEditSession,
                        KeystrokeFunction::Cancel,
                    )
                }
                c if c == VK_ESCAPE.0 => {
                    return (
                        true,
                        KeystrokeCategory::Candidate,
                        KeystrokeFunction::Cancel,
                    )
                }
                // VK_BACK - remove one char from reading string.
                c if c == VK_BACK.0 => {
                    return (
                        true,
                        KeystrokeCategory::Composing,
                        KeystrokeFunction::Backspace,
                    )
                }
                _ => (),
            }
        }
    }

    if candidate_mode == CandidateMode::Original {
        if let Some(mapped_function) = mapped_function {
            if mapped_function == KeystrokeFunction::FinalizeTextstoreAndInput {
                return (true, KeystrokeCategory::InvokeCompositionEditSession, KeystrokeFunction::FinalizeTextstoreAndInput);
            } else if mapped_function == KeystrokeFunction::Convert {
                if modifiers & (TF_MOD_CONTROL | TF_MOD_ALT) == 0 {
                    if modifiers & (TF_MOD_SHIFT) == 0 {
                        return (true, KeystrokeCategory::Candidate, KeystrokeFunction::MoveDown);
                    } else {
                        return (true, KeystrokeCategory::Candidate, KeystrokeFunction::MoveUp);
                    }
                }
            } else if mapped_function == KeystrokeFunction::FinalizeTextstore && modifiers & (TF_MOD_CONTROL | TF_MOD_ALT) == 0 {
                return if modifiers & TF_MOD_SHIFT == 0 {
                    (true, KeystrokeCategory::Composing, KeystrokeFunction::FinalizeTextstoreAndInput)
                } else {
                    (true, KeystrokeCategory::Composing, KeystrokeFunction::FinalizeTextstore)
                };
            }
            return (true, KeystrokeCategory::Candidate, mapped_function);
        }
        match code {
            c if c == VK_BACK.0 => {
                return (
                    true,
                    KeystrokeCategory::Candidate,
                    KeystrokeFunction::Cancel,
                )
            }
            c if c == VK_ESCAPE.0 => {
                return (
                        true,
                        KeystrokeCategory::Candidate,
                        KeystrokeFunction::Cancel,
                )
            }
            _ => (),
        }
    }

    if is_keystroke_range(ch, candidate_mode) {
        return (
            true,
            KeystrokeCategory::Candidate,
            KeystrokeFunction::SelectByNumber,
        );
    }

    if ch != '\0' {
        return if character_affects_keystroke_composition(ch, modifiers, candidate_mode) {
            (
                false,
                KeystrokeCategory::Composing,
                KeystrokeFunction::Input,
            )
        } else {
            (
                false,
                KeystrokeCategory::InvokeCompositionEditSession,
                KeystrokeFunction::FinalizeTextstore,
            )
        };
    }

    (false, KeystrokeCategory::None, KeystrokeFunction::None)
}
