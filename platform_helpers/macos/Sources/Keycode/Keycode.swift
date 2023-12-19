//! Logic taken from https://stackoverflow.com/a/71046488

import Foundation
import Carbon
import Quartz
import AppKit

extension NSEvent.ModifierFlags: Hashable { }

extension OptionSet {
    public func first(_ options: Self.Element ...) -> Self.Element? {
        for option in options {
            if contains(option) {
                return option
            }
        }
        return nil
    }
}

struct Keys {
    static var special = [NSEvent.SpecialKey:CGKeyCode]();
    static var specialNames = [String:NSEvent.SpecialKey]();
    static var character = [String:(CGKeyCode, UInt16)]();
    static var modifierFlag = [NSEvent.ModifierFlags:CGKeyCode]();
}

@_cdecl("initialize")
func initialize() {
    for keyCode in (0..<128).map({ CGKeyCode($0) }) {
        for modifiers in (0..<4) {
            guard let cgevent = CGEvent(keyboardEventSource: nil, virtualKey: CGKeyCode(keyCode), keyDown: true) else { continue };

            if modifiers & (1 << 0) != 0 {
                cgevent.flags.insert(.maskShift);
            }
            if modifiers & (1 << 1) != 0 {
                cgevent.flags.insert(.maskAlternate);
            }

            guard let nsevent = NSEvent(cgEvent: cgevent) else { continue };

            if nsevent.type == .keyDown {
                if let specialKey = nsevent.specialKey {
                    Keys.special[specialKey] = keyCode;
                } else if let characters = nsevent.charactersIgnoringModifiers, !characters.isEmpty && characters != "\u{0010}" {
                    if Keys.character[characters] == nil {
                        Keys.character[characters] = (keyCode, UInt16(modifiers));
                    }
                }
            } else if nsevent.type == .flagsChanged {
                guard let modifierFlag = nsevent.modifierFlags.first(.capsLock, .shift, .control, .option, .command, .help, .function) else {continue};
                Keys.modifierFlag[modifierFlag] = keyCode;
            }
        }
    }

    // https://developer.apple.com/documentation/appkit/nsevent/specialkey
    Keys.specialNames["carriageReturn"] = NSEvent.SpecialKey.carriageReturn;
    Keys.specialNames["backspace"] = NSEvent.SpecialKey.backspace;
    Keys.specialNames["newline"] = NSEvent.SpecialKey.newline;
    Keys.specialNames["enter"] = NSEvent.SpecialKey.enter;
    Keys.specialNames["delete"] = NSEvent.SpecialKey.delete;
    Keys.specialNames["deleteForward"] = NSEvent.SpecialKey.deleteForward;
    Keys.specialNames["backTab"] = NSEvent.SpecialKey.backTab;
    Keys.specialNames["tab"] = NSEvent.SpecialKey.tab;
    Keys.specialNames["upArrow"] = NSEvent.SpecialKey.upArrow;
    Keys.specialNames["downArrow"] = NSEvent.SpecialKey.downArrow;
    Keys.specialNames["leftArrow"] = NSEvent.SpecialKey.leftArrow;
    Keys.specialNames["rightArrow"] = NSEvent.SpecialKey.rightArrow;
    Keys.specialNames["pageUp"] = NSEvent.SpecialKey.pageUp;
    Keys.specialNames["pageDown"] = NSEvent.SpecialKey.pageDown;
    Keys.specialNames["home"] = NSEvent.SpecialKey.home;
    Keys.specialNames["end"] = NSEvent.SpecialKey.end;
    Keys.specialNames["prev"] = NSEvent.SpecialKey.prev;
    Keys.specialNames["next"] = NSEvent.SpecialKey.next;
    Keys.specialNames["begin"] = NSEvent.SpecialKey.begin;
    Keys.specialNames["break"] = NSEvent.SpecialKey.break;
    Keys.specialNames["clearDisplay"] = NSEvent.SpecialKey.clearDisplay;
    Keys.specialNames["clearLine"] = NSEvent.SpecialKey.clearLine;
    Keys.specialNames["deleteCharacter"] = NSEvent.SpecialKey.deleteCharacter;
    Keys.specialNames["deleteLine"] = NSEvent.SpecialKey.deleteLine;
    Keys.specialNames["execute"] = NSEvent.SpecialKey.execute;
    Keys.specialNames["find"] = NSEvent.SpecialKey.find;
    Keys.specialNames["formFeed"] = NSEvent.SpecialKey.formFeed;
    Keys.specialNames["help"] = NSEvent.SpecialKey.help;
    Keys.specialNames["insert"] = NSEvent.SpecialKey.insert;
    Keys.specialNames["insertCharacter"] = NSEvent.SpecialKey.insertCharacter;
    Keys.specialNames["insertLine"] = NSEvent.SpecialKey.insertLine;
    Keys.specialNames["lineSeparator"] = NSEvent.SpecialKey.lineSeparator;
    Keys.specialNames["menu"] = NSEvent.SpecialKey.menu;
    Keys.specialNames["modeSwitch"] = NSEvent.SpecialKey.modeSwitch;
    Keys.specialNames["paragraphSeparator"] = NSEvent.SpecialKey.paragraphSeparator;
    Keys.specialNames["pause"] = NSEvent.SpecialKey.pause;
    Keys.specialNames["print"] = NSEvent.SpecialKey.print;
    Keys.specialNames["printScreen"] = NSEvent.SpecialKey.printScreen;
    Keys.specialNames["redo"] = NSEvent.SpecialKey.redo;
    Keys.specialNames["reset"] = NSEvent.SpecialKey.reset;
    Keys.specialNames["scrollLock"] = NSEvent.SpecialKey.scrollLock;
    Keys.specialNames["select"] = NSEvent.SpecialKey.select;
    Keys.specialNames["stop"] = NSEvent.SpecialKey.stop;
    Keys.specialNames["sysReq"] = NSEvent.SpecialKey.sysReq;
    Keys.specialNames["system"] = NSEvent.SpecialKey.system;
    Keys.specialNames["undo"] = NSEvent.SpecialKey.undo;
    Keys.specialNames["user"] = NSEvent.SpecialKey.user;
    Keys.specialNames["f1"] = NSEvent.SpecialKey.f1;
    Keys.specialNames["f2"] = NSEvent.SpecialKey.f2;
    Keys.specialNames["f3"] = NSEvent.SpecialKey.f3;
    Keys.specialNames["f4"] = NSEvent.SpecialKey.f4;
    Keys.specialNames["f5"] = NSEvent.SpecialKey.f5;
    Keys.specialNames["f6"] = NSEvent.SpecialKey.f6;
    Keys.specialNames["f7"] = NSEvent.SpecialKey.f7;
    Keys.specialNames["f8"] = NSEvent.SpecialKey.f8;
    Keys.specialNames["f9"] = NSEvent.SpecialKey.f9;
    Keys.specialNames["f10"] = NSEvent.SpecialKey.f10;
    Keys.specialNames["f11"] = NSEvent.SpecialKey.f11;
    Keys.specialNames["f12"] = NSEvent.SpecialKey.f12;
    Keys.specialNames["f13"] = NSEvent.SpecialKey.f13;
    Keys.specialNames["f14"] = NSEvent.SpecialKey.f14;
    Keys.specialNames["f15"] = NSEvent.SpecialKey.f15;
    Keys.specialNames["f16"] = NSEvent.SpecialKey.f16;
    Keys.specialNames["f17"] = NSEvent.SpecialKey.f17;
    Keys.specialNames["f18"] = NSEvent.SpecialKey.f18;
    Keys.specialNames["f19"] = NSEvent.SpecialKey.f19;
    Keys.specialNames["f20"] = NSEvent.SpecialKey.f20;
    Keys.specialNames["f21"] = NSEvent.SpecialKey.f21;
    Keys.specialNames["f22"] = NSEvent.SpecialKey.f22;
    Keys.specialNames["f23"] = NSEvent.SpecialKey.f23;
    Keys.specialNames["f24"] = NSEvent.SpecialKey.f24;
    Keys.specialNames["f25"] = NSEvent.SpecialKey.f25;
    Keys.specialNames["f26"] = NSEvent.SpecialKey.f26;
    Keys.specialNames["f27"] = NSEvent.SpecialKey.f27;
    Keys.specialNames["f28"] = NSEvent.SpecialKey.f28;
    Keys.specialNames["f29"] = NSEvent.SpecialKey.f29;
    Keys.specialNames["f30"] = NSEvent.SpecialKey.f30;
    Keys.specialNames["f31"] = NSEvent.SpecialKey.f31;
    Keys.specialNames["f32"] = NSEvent.SpecialKey.f32;
    Keys.specialNames["f33"] = NSEvent.SpecialKey.f33;
    Keys.specialNames["f34"] = NSEvent.SpecialKey.f34;
    Keys.specialNames["f35"] = NSEvent.SpecialKey.f35;
}

@_cdecl("get_special")
public func get_special(name: UnsafePointer<CChar>) -> UInt16 {
    let name = String(cString: name);
    guard let specialKey = Keys.specialNames[name] else { return UInt16.max };
    guard let keyCode = Keys.special[specialKey] else { return 1 };
    return keyCode;
}

@_cdecl("get_character")
public func get_character(character: UInt16) -> UInt32 {
    guard let (keyCode, modifiers) = Keys.character[String(UnicodeScalar(character)!)] else { return UInt32.max };
    return (UInt32(modifiers) << 16) | UInt32(keyCode);
}
