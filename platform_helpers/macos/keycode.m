// clang test.m -fobjc-arc -fmodules -mmacosx-version-min=10.10 -o test

#import <Foundation/Foundation.h>
#include <CoreFoundation/CoreFoundation.h>
#include <Carbon/Carbon.h>

/* Returns string representation of key, if it is printable.
 * Ownership follows the Create Rule; that is, it is the caller's
 * responsibility to release the returned object. */
CFStringRef createStringForKey(CGKeyCode keyCode, Boolean shift, Boolean alt)
{
    TISInputSourceRef currentKeyboard = TISCopyCurrentKeyboardInputSource();
    CFDataRef layoutData =
        TISGetInputSourceProperty(currentKeyboard,
                                  kTISPropertyUnicodeKeyLayoutData);
    const UCKeyboardLayout *keyboardLayout =
        (const UCKeyboardLayout *)CFDataGetBytePtr(layoutData);

    UInt32 keysDown = 0;
    UniChar chars[4];
    UniCharCount realLength;

    UInt32 modifiers = 0;

    // https://github.com/2bbb/ofxMacKeyboard/blob/master/src/ofxMacKeyboardEventStealer.mm#L34

    if (shift) {
        modifiers |= NX_DEVICELSHIFTKEYMASK;
    }

    if (alt) {
        // https://github.com/abarnert/pykeycode/issues/3#issuecomment-47027019
        modifiers |= 0x08; // NX_DEVICELALTKEYMASK;
    }

    UCKeyTranslate(keyboardLayout,
                   keyCode,
                   kUCKeyActionDown,
                //    kUCKeyActionDisplay,
                   modifiers,
                   LMGetKbdType(),
                   kUCKeyTranslateNoDeadKeysBit,
                   &keysDown,
                   sizeof(chars) / sizeof(chars[0]),
                   &realLength,
                   chars);
    CFRelease(currentKeyboard);

    if (realLength == 0) {
        return NULL;
    }

    return CFStringCreateWithCharacters(kCFAllocatorDefault, chars, 1);
}


/* Returns key code for given character via the above function, or UINT16_MAX
 * on error. The key code is then orred with shited modifier state bits */
UInt32 keyCodeForChar(const char c)
{
    UniChar character = c;
    CFStringRef charStr = CFStringCreateWithCharacters(kCFAllocatorDefault, &character, 1);

    UInt16 i;
    UInt8 j;


    /* Loop through every keycode to find the current mapping. */
    for (i = 0; i < 0xffff; ++i) {
        for (j = 0; j < 4; j++) {
            Boolean shift = j % 2 == 1;
            Boolean alt = j >= 2;


            CFStringRef string = createStringForKey((CGKeyCode) i, shift, alt);

            if (string != NULL) {
                CFRelease(string);

                if (CFEqual(string, charStr)) {
                    return (((UInt32) j) << 16) | i;
                }
            }
        }
    }

    return UINT16_MAX;
}
